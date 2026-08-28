//! Hand-written generation-zero projection of `ethos/signal.ethos`.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

pub const INTERFACE_VERSION: ProtocolVersion = ProtocolVersion::new(0, 2, 0);
pub const CHANNEL_CONTRACT_ID: ChannelContractId = ChannelContractId(1);
pub const CHANNEL_WIRE_REVISION: ChannelWireRevision = ChannelWireRevision(2);
pub const PROTOCOL_VERSION: ProtocolVersion = INTERFACE_VERSION;

/// The only binary boundary for this contract.
pub trait SignalFrameCodec: Sized {
    fn encode_length_prefixed(&self) -> Result<Vec<u8>, FrameCodecError>;
    fn decode_length_prefixed(bytes: &[u8]) -> Result<Self, FrameCodecError>;
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProtocolVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl ProtocolVersion {
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChannelContractId(pub u32);

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChannelWireRevision(pub u16);

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SourceName(pub String);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct RelativePath(pub String);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ArtifactPath(pub String);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct FileLocation {
    pub source_name: SourceName,
    pub relative_path: RelativePath,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct GenerationRequest {
    pub file: FileLocation,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Generation {
    pub file: FileLocation,
    pub artifact: ArtifactPath,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum ObservationSelection {
    Assemblies,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct AssemblySummary {
    pub file: FileLocation,
    pub artifact: ArtifactPath,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Assemblies(pub Vec<AssemblySummary>);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct AssemblySnapshot {
    pub assemblies: Assemblies,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Observation {
    Assemblies(AssemblySnapshot),
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExtentStart(pub i64);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExtentEnd(pub i64);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceExtent {
    pub extent_start: ExtentStart,
    pub extent_end: ExtentEnd,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SyntaxFaultReason(pub String);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SyntaxFault {
    pub extent: SourceExtent,
    pub reason: SyntaxFaultReason,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProjectionFaultReason(pub String);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProjectionFault {
    pub reason: ProjectionFaultReason,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum GenerationRefusal {
    UnknownSource(SourceName),
    FileAbsent(FileLocation),
    ImportUnresolved(FileLocation),
    InvalidEthos(SyntaxFault),
    RustProjectionRejected(ProjectionFault),
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Generate(GenerationRequest),
    Observe(ObservationSelection),
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Reply {
    Generated(Generation),
    Observed(Observation),
    GenerationRejected(GenerationRefusal),
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FrameBody {
    Request(Request),
    Reply(Reply),
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Frame {
    pub channel_contract_id: ChannelContractId,
    pub channel_wire_revision: ChannelWireRevision,
    pub protocol_version: ProtocolVersion,
    pub body: FrameBody,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FrameCodecError {
    LengthPrefixMissing,
    LengthMismatch {
        expected: usize,
        found: usize,
    },
    LengthTooLarge,
    ArchiveEncode,
    ArchiveDecode,
    WrongChannelContract {
        expected: ChannelContractId,
        found: ChannelContractId,
    },
    WrongChannelWireRevision {
        expected: ChannelWireRevision,
        found: ChannelWireRevision,
    },
    UnsupportedProtocol {
        expected: ProtocolVersion,
        found: ProtocolVersion,
    },
}

impl SignalFrameCodec for Frame {
    fn encode_length_prefixed(&self) -> Result<Vec<u8>, FrameCodecError> {
        if self.channel_contract_id != CHANNEL_CONTRACT_ID {
            return Err(FrameCodecError::WrongChannelContract {
                expected: CHANNEL_CONTRACT_ID,
                found: self.channel_contract_id,
            });
        }
        if self.channel_wire_revision != CHANNEL_WIRE_REVISION {
            return Err(FrameCodecError::WrongChannelWireRevision {
                expected: CHANNEL_WIRE_REVISION,
                found: self.channel_wire_revision,
            });
        }
        if self.protocol_version != PROTOCOL_VERSION {
            return Err(FrameCodecError::UnsupportedProtocol {
                expected: PROTOCOL_VERSION,
                found: self.protocol_version,
            });
        }
        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map_err(|_| FrameCodecError::ArchiveEncode)?;
        let length = u32::try_from(archive.len()).map_err(|_| FrameCodecError::LengthTooLarge)?;
        let mut frame = Vec::with_capacity(4 + archive.len());
        frame.extend_from_slice(&length.to_le_bytes());
        frame.extend_from_slice(&archive);
        Ok(frame)
    }

    fn decode_length_prefixed(bytes: &[u8]) -> Result<Self, FrameCodecError> {
        let Some(prefix) = bytes.get(..4) else {
            return Err(FrameCodecError::LengthPrefixMissing);
        };
        let expected = u32::from_le_bytes([prefix[0], prefix[1], prefix[2], prefix[3]]) as usize;
        let payload = &bytes[4..];
        if payload.len() != expected {
            return Err(FrameCodecError::LengthMismatch {
                expected,
                found: payload.len(),
            });
        }
        let frame = rkyv::from_bytes::<Self, rkyv::rancor::Error>(payload)
            .map_err(|_| FrameCodecError::ArchiveDecode)?;
        if frame.channel_contract_id != CHANNEL_CONTRACT_ID {
            return Err(FrameCodecError::WrongChannelContract {
                expected: CHANNEL_CONTRACT_ID,
                found: frame.channel_contract_id,
            });
        }
        if frame.channel_wire_revision != CHANNEL_WIRE_REVISION {
            return Err(FrameCodecError::WrongChannelWireRevision {
                expected: CHANNEL_WIRE_REVISION,
                found: frame.channel_wire_revision,
            });
        }
        if frame.protocol_version != PROTOCOL_VERSION {
            return Err(FrameCodecError::UnsupportedProtocol {
                expected: PROTOCOL_VERSION,
                found: frame.protocol_version,
            });
        }
        Ok(frame)
    }
}
