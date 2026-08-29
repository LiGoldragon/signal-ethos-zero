use datomic::{Datomic, TextEdge};
use protos::{PortionText, Text};
use signal_ethos_zero::*;

fn source_name(value: &str) -> SourceName {
    value.try_into().expect("representable source name")
}

fn relative_path(value: &str) -> RelativePath {
    value.try_into().expect("representable relative path")
}

fn artifact_path(value: &str) -> ArtifactPath {
    value.try_into().expect("representable artifact path")
}

fn file() -> FileLocation {
    FileLocation {
        source_name: source_name("ethos-zero"),
        relative_path: relative_path("ethos/signal.ethos"),
    }
}

fn generation() -> Generation {
    Generation {
        file: file(),
        artifact: artifact_path("src/generated/signal.rs"),
    }
}

fn frame(body: FrameBody) -> Frame {
    Frame {
        channel_contract_id: CHANNEL_CONTRACT_ID,
        channel_wire_revision: CHANNEL_WIRE_REVISION,
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

fn assert_frame_round_trip(value: Frame) {
    let encoded = value.encode_length_prefixed().expect("encode signal");
    assert_eq!(
        Frame::decode_length_prefixed(&encoded).expect("decode signal"),
        value
    );
}

fn assert_text_round_trip<T>(value: T)
where
    T: Datomic + Clone + std::fmt::Debug + PartialEq,
{
    let text = Datomic::portion(&value).canonical_text();
    assert_eq!(
        Text::<T>::from(text.as_ref())
            .embody()
            .expect("Datomic text embodiment"),
        value
    );
}

#[test]
fn every_root_executes_through_datomic_text_and_rkyv_frames() {
    let assembly = AssemblySummary {
        file: file(),
        artifact: artifact_path("src/generated/signal.rs"),
    };
    let requests = [
        Request::Generate(GenerationRequest { file: file() }),
        Request::Observe(ObservationSelection::Assemblies),
        Request::Subscribe(SubscriptionRequest { file: file() }),
        Request::Unsubscribe(SubscriptionRequest { file: file() }),
    ];
    for request in requests {
        assert_text_round_trip(request.clone());
        assert_frame_round_trip(frame(FrameBody::Request(request)));
    }

    let replies = [
        Reply::Generated(generation()),
        Reply::Observed(Observation::Assemblies(AssemblySnapshot {
            assemblies: Assemblies(vec![assembly]),
        })),
        Reply::GenerationRejected(GenerationRefusal::UnknownSource(source_name("missing"))),
        Reply::GenerationRejected(GenerationRefusal::FileAbsent(file())),
        Reply::GenerationRejected(GenerationRefusal::ImportUnresolved(file())),
        Reply::GenerationRejected(GenerationRefusal::InvalidRelativePath(relative_path(
            "../outside",
        ))),
        Reply::GenerationRejected(GenerationRefusal::InvalidEthos(SyntaxFault {
            extent: SourceExtent {
                extent_start: ExtentStart(0),
                extent_end: ExtentEnd(17),
            },
            reason: "unexpected interface section"
                .try_into()
                .expect("representable reason"),
        })),
        Reply::GenerationRejected(GenerationRefusal::RustProjectionRejected(ProjectionFault {
            reason: "unrepresentable generated identifier"
                .try_into()
                .expect("representable reason"),
        })),
    ];
    for reply in replies {
        assert_text_round_trip(reply.clone());
        assert_frame_round_trip(frame(FrameBody::Reply(reply)));
    }

    let refusal = Refusal::InvalidRelativePath(relative_path("../outside"));
    assert_text_round_trip(refusal.clone());
    assert_frame_round_trip(frame(FrameBody::Refusal(refusal)));

    let events = [
        Stream::GenerationStarted(GenerationRequest { file: file() }),
        Stream::GenerationCompleted(generation()),
        Stream::GenerationRefused(GenerationRefusal::InvalidRelativePath(relative_path(
            "../outside",
        ))),
    ];
    for event in events {
        assert_text_round_trip(event.clone());
        assert_frame_round_trip(frame(FrameBody::Event(event)));
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
fn envelope_metadata_is_validated_on_encode_and_decode() {
    let wrong_protocol = Frame {
        protocol_version: ProtocolVersion::new(0, 2, 0),
        ..frame(FrameBody::Request(Request::Observe(
            ObservationSelection::Assemblies,
        )))
    };
    assert!(matches!(
        wrong_protocol.encode_length_prefixed(),
        Err(FrameCodecError::UnsupportedProtocol { .. })
    ));
    assert!(matches!(
        Frame::decode_length_prefixed(&raw_length_prefixed(&wrong_protocol)),
        Err(FrameCodecError::UnsupportedProtocol { .. })
    ));

    let wrong_contract = Frame {
        channel_contract_id: ChannelContractId(2),
        ..frame(FrameBody::Request(Request::Observe(
            ObservationSelection::Assemblies,
        )))
    };
    assert!(matches!(
        wrong_contract.encode_length_prefixed(),
        Err(FrameCodecError::WrongChannelContract { .. })
    ));
    assert!(matches!(
        Frame::decode_length_prefixed(&raw_length_prefixed(&wrong_contract)),
        Err(FrameCodecError::WrongChannelContract { .. })
    ));

    let wrong_revision = Frame {
        channel_wire_revision: ChannelWireRevision(2),
        ..frame(FrameBody::Request(Request::Observe(
            ObservationSelection::Assemblies,
        )))
    };
    assert!(matches!(
        wrong_revision.encode_length_prefixed(),
        Err(FrameCodecError::WrongChannelWireRevision { .. })
    ));
    assert!(matches!(
        Frame::decode_length_prefixed(&raw_length_prefixed(&wrong_revision)),
        Err(FrameCodecError::WrongChannelWireRevision { .. })
    ));
}
