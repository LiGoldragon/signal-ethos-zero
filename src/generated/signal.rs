use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
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
pub const INTERFACE_VERSION: ProtocolVersion = ProtocolVersion::new(0u16, 3u16, 0u16);
pub const CHANNEL_CONTRACT_ID: ChannelContractId = ChannelContractId(1u32);
pub const CHANNEL_WIRE_REVISION: ChannelWireRevision = ChannelWireRevision(3u16);
pub const PROTOCOL_VERSION: ProtocolVersion = INTERFACE_VERSION;
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SourceName(String);
impl SourceName {
    pub fn try_from_string(
        value: String,
    ) -> std::result::Result<Self, datomic::UnrepresentableString> {
        datomic::DatomicString::try_from(value).map(|value| Self(value.as_ref().to_owned()))
    }
}
impl std::convert::TryFrom<String> for SourceName {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value)
    }
}
impl<'a> std::convert::TryFrom<&'a str> for SourceName {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value.to_owned())
    }
}
impl AsRef<str> for SourceName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct RelativePath(String);
impl RelativePath {
    pub fn try_from_string(
        value: String,
    ) -> std::result::Result<Self, datomic::UnrepresentableString> {
        datomic::DatomicString::try_from(value).map(|value| Self(value.as_ref().to_owned()))
    }
}
impl std::convert::TryFrom<String> for RelativePath {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value)
    }
}
impl<'a> std::convert::TryFrom<&'a str> for RelativePath {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value.to_owned())
    }
}
impl AsRef<str> for RelativePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ArtifactPath(String);
impl ArtifactPath {
    pub fn try_from_string(
        value: String,
    ) -> std::result::Result<Self, datomic::UnrepresentableString> {
        datomic::DatomicString::try_from(value).map(|value| Self(value.as_ref().to_owned()))
    }
}
impl std::convert::TryFrom<String> for ArtifactPath {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value)
    }
}
impl<'a> std::convert::TryFrom<&'a str> for ArtifactPath {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value.to_owned())
    }
}
impl AsRef<str> for ArtifactPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
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
pub struct SubscriptionRequest {
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
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ExtentStart(pub i64);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ExtentEnd(pub i64);
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SourceExtent {
    pub extent_start: ExtentStart,
    pub extent_end: ExtentEnd,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SyntaxFaultReason(String);
impl SyntaxFaultReason {
    pub fn try_from_string(
        value: String,
    ) -> std::result::Result<Self, datomic::UnrepresentableString> {
        datomic::DatomicString::try_from(value).map(|value| Self(value.as_ref().to_owned()))
    }
}
impl std::convert::TryFrom<String> for SyntaxFaultReason {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value)
    }
}
impl<'a> std::convert::TryFrom<&'a str> for SyntaxFaultReason {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value.to_owned())
    }
}
impl AsRef<str> for SyntaxFaultReason {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SyntaxFault {
    pub extent: SourceExtent,
    pub reason: SyntaxFaultReason,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProjectionFaultReason(String);
impl ProjectionFaultReason {
    pub fn try_from_string(
        value: String,
    ) -> std::result::Result<Self, datomic::UnrepresentableString> {
        datomic::DatomicString::try_from(value).map(|value| Self(value.as_ref().to_owned()))
    }
}
impl std::convert::TryFrom<String> for ProjectionFaultReason {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value)
    }
}
impl<'a> std::convert::TryFrom<&'a str> for ProjectionFaultReason {
    type Error = datomic::UnrepresentableString;
    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        Self::try_from_string(value.to_owned())
    }
}
impl AsRef<str> for ProjectionFaultReason {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProjectionFault {
    pub reason: ProjectionFaultReason,
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum GenerationRefusal {
    UnknownSource(SourceName),
    FileAbsent(FileLocation),
    ImportUnresolved(FileLocation),
    InvalidRelativePath(RelativePath),
    InvalidEthos(SyntaxFault),
    RustProjectionRejected(ProjectionFault),
}
impl datomic::Datomic for SourceName {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        Ok(Self(
            <datomic::DatomicString as datomic::Datomic>::embody(portion)?
                .as_ref()
                .to_owned(),
        ))
    }
    fn portion(&self) -> protos::Portion {
        datomic::DatomicString::try_from(self.0.clone()).map_or_else(
            |_| datomic::PortionBuilding::bare("wire-invalid"),
            |value| datomic::Datomic::portion(&value),
        )
    }
}
impl datomic::Datomic for RelativePath {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        Ok(Self(
            <datomic::DatomicString as datomic::Datomic>::embody(portion)?
                .as_ref()
                .to_owned(),
        ))
    }
    fn portion(&self) -> protos::Portion {
        datomic::DatomicString::try_from(self.0.clone()).map_or_else(
            |_| datomic::PortionBuilding::bare("wire-invalid"),
            |value| datomic::Datomic::portion(&value),
        )
    }
}
impl datomic::Datomic for ArtifactPath {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        Ok(Self(
            <datomic::DatomicString as datomic::Datomic>::embody(portion)?
                .as_ref()
                .to_owned(),
        ))
    }
    fn portion(&self) -> protos::Portion {
        datomic::DatomicString::try_from(self.0.clone()).map_or_else(
            |_| datomic::PortionBuilding::bare("wire-invalid"),
            |value| datomic::Datomic::portion(&value),
        )
    }
}
impl datomic::Datomic for FileLocation {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        let Some(parts) =
            datomic::PortionViewing::structural(portion, protos::StructuralEnclosure::Braced)
        else {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Shape,
            ));
        };
        if parts.len() != 2usize {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Arity,
            ));
        }
        Ok(Self {
            source_name: <SourceName as datomic::Datomic>::embody(&parts[0usize])?,
            relative_path: <RelativePath as datomic::Datomic>::embody(&parts[1usize])?,
        })
    }
    fn portion(&self) -> protos::Portion {
        datomic::PortionBuilding::structural(
            "",
            protos::StructuralEnclosure::Braced,
            vec![
                datomic::Datomic::portion(&self.source_name),
                datomic::Datomic::portion(&self.relative_path),
            ],
        )
    }
}
impl datomic::Datomic for GenerationRequest {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        let Some(parts) =
            datomic::PortionViewing::structural(portion, protos::StructuralEnclosure::Braced)
        else {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Shape,
            ));
        };
        if parts.len() != 1usize {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Arity,
            ));
        }
        Ok(Self {
            file: <FileLocation as datomic::Datomic>::embody(&parts[0usize])?,
        })
    }
    fn portion(&self) -> protos::Portion {
        datomic::PortionBuilding::structural(
            "",
            protos::StructuralEnclosure::Braced,
            vec![datomic::Datomic::portion(&self.file)],
        )
    }
}
impl datomic::Datomic for SubscriptionRequest {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        let Some(parts) =
            datomic::PortionViewing::structural(portion, protos::StructuralEnclosure::Braced)
        else {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Shape,
            ));
        };
        if parts.len() != 1usize {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Arity,
            ));
        }
        Ok(Self {
            file: <FileLocation as datomic::Datomic>::embody(&parts[0usize])?,
        })
    }
    fn portion(&self) -> protos::Portion {
        datomic::PortionBuilding::structural(
            "",
            protos::StructuralEnclosure::Braced,
            vec![datomic::Datomic::portion(&self.file)],
        )
    }
}
impl datomic::Datomic for Generation {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        let Some(parts) =
            datomic::PortionViewing::structural(portion, protos::StructuralEnclosure::Braced)
        else {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Shape,
            ));
        };
        if parts.len() != 2usize {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Arity,
            ));
        }
        Ok(Self {
            file: <FileLocation as datomic::Datomic>::embody(&parts[0usize])?,
            artifact: <ArtifactPath as datomic::Datomic>::embody(&parts[1usize])?,
        })
    }
    fn portion(&self) -> protos::Portion {
        datomic::PortionBuilding::structural(
            "",
            protos::StructuralEnclosure::Braced,
            vec![
                datomic::Datomic::portion(&self.file),
                datomic::Datomic::portion(&self.artifact),
            ],
        )
    }
}
impl datomic::Datomic for ObservationSelection {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Assemblies)) {
            return Ok(Self::Assemblies);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Assemblies => datomic::PortionBuilding::bare(stringify!(Assemblies)),
        }
    }
}
impl datomic::Datomic for AssemblySummary {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        let Some(parts) =
            datomic::PortionViewing::structural(portion, protos::StructuralEnclosure::Braced)
        else {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Shape,
            ));
        };
        if parts.len() != 2usize {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Arity,
            ));
        }
        Ok(Self {
            file: <FileLocation as datomic::Datomic>::embody(&parts[0usize])?,
            artifact: <ArtifactPath as datomic::Datomic>::embody(&parts[1usize])?,
        })
    }
    fn portion(&self) -> protos::Portion {
        datomic::PortionBuilding::structural(
            "",
            protos::StructuralEnclosure::Braced,
            vec![
                datomic::Datomic::portion(&self.file),
                datomic::Datomic::portion(&self.artifact),
            ],
        )
    }
}
impl datomic::Datomic for Assemblies {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        Ok(Self(<Vec<AssemblySummary> as datomic::Datomic>::embody(
            portion,
        )?))
    }
    fn portion(&self) -> protos::Portion {
        <Vec<AssemblySummary> as datomic::Datomic>::portion(&self.0)
    }
}
impl datomic::Datomic for AssemblySnapshot {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        let Some(parts) =
            datomic::PortionViewing::structural(portion, protos::StructuralEnclosure::Braced)
        else {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Shape,
            ));
        };
        if parts.len() != 1usize {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Arity,
            ));
        }
        Ok(Self {
            assemblies: <Assemblies as datomic::Datomic>::embody(&parts[0usize])?,
        })
    }
    fn portion(&self) -> protos::Portion {
        datomic::PortionBuilding::structural(
            "",
            protos::StructuralEnclosure::Braced,
            vec![datomic::Datomic::portion(&self.assemblies)],
        )
    }
}
impl datomic::Datomic for Observation {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(Assemblies)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::Assemblies(
                <AssemblySnapshot as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Assemblies(value) => datomic::PortionBuilding::headed(
                stringify!(Assemblies),
                protos::Separator::Period,
                <AssemblySnapshot as datomic::Datomic>::portion(value),
            ),
        }
    }
}
impl datomic::Datomic for ExtentStart {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        Ok(Self(<i64 as datomic::Datomic>::embody(portion)?))
    }
    fn portion(&self) -> protos::Portion {
        datomic::Datomic::portion(&self.0)
    }
}
impl datomic::Datomic for ExtentEnd {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        Ok(Self(<i64 as datomic::Datomic>::embody(portion)?))
    }
    fn portion(&self) -> protos::Portion {
        datomic::Datomic::portion(&self.0)
    }
}
impl datomic::Datomic for SourceExtent {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        let Some(parts) =
            datomic::PortionViewing::structural(portion, protos::StructuralEnclosure::Braced)
        else {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Shape,
            ));
        };
        if parts.len() != 2usize {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Arity,
            ));
        }
        Ok(Self {
            extent_start: <ExtentStart as datomic::Datomic>::embody(&parts[0usize])?,
            extent_end: <ExtentEnd as datomic::Datomic>::embody(&parts[1usize])?,
        })
    }
    fn portion(&self) -> protos::Portion {
        datomic::PortionBuilding::structural(
            "",
            protos::StructuralEnclosure::Braced,
            vec![
                datomic::Datomic::portion(&self.extent_start),
                datomic::Datomic::portion(&self.extent_end),
            ],
        )
    }
}
impl datomic::Datomic for SyntaxFaultReason {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        Ok(Self(
            <datomic::DatomicString as datomic::Datomic>::embody(portion)?
                .as_ref()
                .to_owned(),
        ))
    }
    fn portion(&self) -> protos::Portion {
        datomic::DatomicString::try_from(self.0.clone()).map_or_else(
            |_| datomic::PortionBuilding::bare("wire-invalid"),
            |value| datomic::Datomic::portion(&value),
        )
    }
}
impl datomic::Datomic for SyntaxFault {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        let Some(parts) =
            datomic::PortionViewing::structural(portion, protos::StructuralEnclosure::Braced)
        else {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Shape,
            ));
        };
        if parts.len() != 2usize {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Arity,
            ));
        }
        Ok(Self {
            extent: <SourceExtent as datomic::Datomic>::embody(&parts[0usize])?,
            reason: <SyntaxFaultReason as datomic::Datomic>::embody(&parts[1usize])?,
        })
    }
    fn portion(&self) -> protos::Portion {
        datomic::PortionBuilding::structural(
            "",
            protos::StructuralEnclosure::Braced,
            vec![
                datomic::Datomic::portion(&self.extent),
                datomic::Datomic::portion(&self.reason),
            ],
        )
    }
}
impl datomic::Datomic for ProjectionFaultReason {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        Ok(Self(
            <datomic::DatomicString as datomic::Datomic>::embody(portion)?
                .as_ref()
                .to_owned(),
        ))
    }
    fn portion(&self) -> protos::Portion {
        datomic::DatomicString::try_from(self.0.clone()).map_or_else(
            |_| datomic::PortionBuilding::bare("wire-invalid"),
            |value| datomic::Datomic::portion(&value),
        )
    }
}
impl datomic::Datomic for ProjectionFault {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        let Some(parts) =
            datomic::PortionViewing::structural(portion, protos::StructuralEnclosure::Braced)
        else {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Shape,
            ));
        };
        if parts.len() != 1usize {
            return Err(datomic::PortionViewing::fault(
                portion,
                datomic::FaultProblem::Arity,
            ));
        }
        Ok(Self {
            reason: <ProjectionFaultReason as datomic::Datomic>::embody(&parts[0usize])?,
        })
    }
    fn portion(&self) -> protos::Portion {
        datomic::PortionBuilding::structural(
            "",
            protos::StructuralEnclosure::Braced,
            vec![datomic::Datomic::portion(&self.reason)],
        )
    }
}
impl datomic::Datomic for GenerationRefusal {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(UnknownSource)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::UnknownSource(
                <SourceName as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(FileAbsent)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::FileAbsent(
                <FileLocation as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(ImportUnresolved)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::ImportUnresolved(
                <FileLocation as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(InvalidRelativePath)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::InvalidRelativePath(
                <RelativePath as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(InvalidEthos)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::InvalidEthos(
                <SyntaxFault as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(RustProjectionRejected)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::RustProjectionRejected(
                <ProjectionFault as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::UnknownSource(value) => datomic::PortionBuilding::headed(
                stringify!(UnknownSource),
                protos::Separator::Period,
                <SourceName as datomic::Datomic>::portion(value),
            ),
            Self::FileAbsent(value) => datomic::PortionBuilding::headed(
                stringify!(FileAbsent),
                protos::Separator::Period,
                <FileLocation as datomic::Datomic>::portion(value),
            ),
            Self::ImportUnresolved(value) => datomic::PortionBuilding::headed(
                stringify!(ImportUnresolved),
                protos::Separator::Period,
                <FileLocation as datomic::Datomic>::portion(value),
            ),
            Self::InvalidRelativePath(value) => datomic::PortionBuilding::headed(
                stringify!(InvalidRelativePath),
                protos::Separator::Period,
                <RelativePath as datomic::Datomic>::portion(value),
            ),
            Self::InvalidEthos(value) => datomic::PortionBuilding::headed(
                stringify!(InvalidEthos),
                protos::Separator::Period,
                <SyntaxFault as datomic::Datomic>::portion(value),
            ),
            Self::RustProjectionRejected(value) => datomic::PortionBuilding::headed(
                stringify!(RustProjectionRejected),
                protos::Separator::Period,
                <ProjectionFault as datomic::Datomic>::portion(value),
            ),
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Generate(GenerationRequest),
    Observe(ObservationSelection),
    Subscribe(SubscriptionRequest),
    Unsubscribe(SubscriptionRequest),
}
impl datomic::Datomic for Request {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(Generate)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::Generate(
                <GenerationRequest as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(Observe)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::Observe(
                <ObservationSelection as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(Subscribe)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::Subscribe(
                <SubscriptionRequest as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(Unsubscribe)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::Unsubscribe(
                <SubscriptionRequest as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Generate(value) => datomic::PortionBuilding::headed(
                stringify!(Generate),
                protos::Separator::Period,
                <GenerationRequest as datomic::Datomic>::portion(value),
            ),
            Self::Observe(value) => datomic::PortionBuilding::headed(
                stringify!(Observe),
                protos::Separator::Period,
                <ObservationSelection as datomic::Datomic>::portion(value),
            ),
            Self::Subscribe(value) => datomic::PortionBuilding::headed(
                stringify!(Subscribe),
                protos::Separator::Period,
                <SubscriptionRequest as datomic::Datomic>::portion(value),
            ),
            Self::Unsubscribe(value) => datomic::PortionBuilding::headed(
                stringify!(Unsubscribe),
                protos::Separator::Period,
                <SubscriptionRequest as datomic::Datomic>::portion(value),
            ),
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Reply {
    Generated(Generation),
    Observed(Observation),
    GenerationRejected(GenerationRefusal),
}
impl datomic::Datomic for Reply {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(Generated)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::Generated(<Generation as datomic::Datomic>::embody(
                &headed.body,
            )?));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(Observed)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::Observed(<Observation as datomic::Datomic>::embody(
                &headed.body,
            )?));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(GenerationRejected)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::GenerationRejected(
                <GenerationRefusal as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Generated(value) => datomic::PortionBuilding::headed(
                stringify!(Generated),
                protos::Separator::Period,
                <Generation as datomic::Datomic>::portion(value),
            ),
            Self::Observed(value) => datomic::PortionBuilding::headed(
                stringify!(Observed),
                protos::Separator::Period,
                <Observation as datomic::Datomic>::portion(value),
            ),
            Self::GenerationRejected(value) => datomic::PortionBuilding::headed(
                stringify!(GenerationRejected),
                protos::Separator::Period,
                <GenerationRefusal as datomic::Datomic>::portion(value),
            ),
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    InvalidRelativePath(RelativePath),
}
impl datomic::Datomic for Refusal {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(InvalidRelativePath)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::InvalidRelativePath(
                <RelativePath as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::InvalidRelativePath(value) => datomic::PortionBuilding::headed(
                stringify!(InvalidRelativePath),
                protos::Separator::Period,
                <RelativePath as datomic::Datomic>::portion(value),
            ),
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Stream {
    GenerationStarted(GenerationRequest),
    GenerationCompleted(Generation),
    GenerationRefused(GenerationRefusal),
}
impl datomic::Datomic for Stream {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(GenerationStarted)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::GenerationStarted(
                <GenerationRequest as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(GenerationCompleted)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::GenerationCompleted(
                <Generation as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        if let Some(headed) = datomic::PortionViewing::headed(portion)
            && headed.head.as_ref() == stringify!(GenerationRefused)
            && headed.separator == protos::Separator::Period
        {
            return Ok(Self::GenerationRefused(
                <GenerationRefusal as datomic::Datomic>::embody(&headed.body)?,
            ));
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::GenerationStarted(value) => datomic::PortionBuilding::headed(
                stringify!(GenerationStarted),
                protos::Separator::Period,
                <GenerationRequest as datomic::Datomic>::portion(value),
            ),
            Self::GenerationCompleted(value) => datomic::PortionBuilding::headed(
                stringify!(GenerationCompleted),
                protos::Separator::Period,
                <Generation as datomic::Datomic>::portion(value),
            ),
            Self::GenerationRefused(value) => datomic::PortionBuilding::headed(
                stringify!(GenerationRefused),
                protos::Separator::Period,
                <GenerationRefusal as datomic::Datomic>::portion(value),
            ),
        }
    }
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FrameBody {
    Request(Request),
    Reply(Reply),
    Refusal(Refusal),
    Event(Stream),
}
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct Frame {
    pub channel_contract_id: ChannelContractId,
    pub channel_wire_revision: ChannelWireRevision,
    pub protocol_version: ProtocolVersion,
    pub body: FrameBody,
}
