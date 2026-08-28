use signal_ethos_zero::*;

fn file() -> FileLocation {
    FileLocation {
        source_name: SourceName("ethos-zero".into()),
        relative_path: RelativePath("ethos/signal.ethos".into()),
    }
}

fn frame(body: FrameBody) -> Frame {
    Frame {
        protocol_version: PROTOCOL_VERSION,
        body,
    }
}

fn raw_length_prefixed(value: &Frame) -> Vec<u8> {
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(value).expect("archive frame");
    let length = u32::try_from(archive.len()).expect("frame length fits prefix");
    let mut bytes = length.to_le_bytes().to_vec();
    bytes.extend_from_slice(&archive);
    bytes
}

#[test]
fn concrete_generate_request_round_trips_through_the_framed_signal() {
    let value = frame(FrameBody::Request(Request::Generate(GenerationRequest {
        file: file(),
    })));
    let encoded = value.encode_length_prefixed().expect("encode request");
    assert_eq!(
        Frame::decode_length_prefixed(&encoded).expect("decode request"),
        value
    );
}

#[test]
fn concrete_observation_and_every_refusal_round_trip() {
    let assembly = AssemblySummary {
        file: file(),
        artifact: ArtifactPath("src/generated/signal.rs".into()),
    };
    let replies = [
        Reply::Observed(Observation::Assemblies(AssemblySnapshot {
            assemblies: Assemblies(vec![assembly.clone()]),
        })),
        Reply::GenerationRejected(GenerationRefusal::UnknownSource(SourceName(
            "missing".into(),
        ))),
        Reply::GenerationRejected(GenerationRefusal::FileAbsent(file())),
        Reply::GenerationRejected(GenerationRefusal::ImportUnresolved(file())),
        Reply::GenerationRejected(GenerationRefusal::InvalidEthos(SyntaxFault {
            extent: SourceExtent {
                extent_start: ExtentStart(0),
                extent_end: ExtentEnd(17),
            },
            reason: SyntaxFaultReason("unexpected interface section".into()),
        })),
        Reply::GenerationRejected(GenerationRefusal::RustProjectionRejected(ProjectionFault {
            reason: ProjectionFaultReason("unrepresentable generated identifier".into()),
        })),
    ];
    for reply in replies {
        let value = frame(FrameBody::Reply(reply));
        let encoded = value.encode_length_prefixed().expect("encode reply");
        assert_eq!(
            Frame::decode_length_prefixed(&encoded).expect("decode reply"),
            value
        );
    }
}

#[test]
fn malformed_frames_are_rejected() {
    assert_eq!(
        Frame::decode_length_prefixed(&[]),
        Err(FrameCodecError::LengthPrefixMissing)
    );
    assert!(matches!(
        Frame::decode_length_prefixed(&[1, 0, 0, 0]),
        Err(FrameCodecError::LengthMismatch {
            expected: 1,
            found: 0
        })
    ));
    assert_eq!(
        Frame::decode_length_prefixed(&[1, 0, 0, 0, 0]),
        Err(FrameCodecError::ArchiveDecode)
    );
}

#[test]
fn protocol_versions_are_validated() {
    let value = Frame {
        protocol_version: ProtocolVersion::new(0, 1, 1),
        body: FrameBody::Request(Request::Observe(ObservationSelection::Assemblies)),
    };
    assert_eq!(
        value.encode_length_prefixed(),
        Err(FrameCodecError::UnsupportedProtocol {
            expected: PROTOCOL_VERSION,
            found: ProtocolVersion::new(0, 1, 1)
        })
    );
    assert_eq!(
        Frame::decode_length_prefixed(&raw_length_prefixed(&value)),
        Err(FrameCodecError::UnsupportedProtocol {
            expected: PROTOCOL_VERSION,
            found: ProtocolVersion::new(0, 1, 1)
        })
    );
}
