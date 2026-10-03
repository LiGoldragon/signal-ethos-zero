#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type ArtifactPath = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Generation {
    pub file_location: signal_ethos_zero::FileLocation,
    pub artifact_path: ArtifactPath,
}
#[rustfmt::skip]
pub type Start = i64;
#[rustfmt::skip]
pub type End = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Extent {
    pub start: Start,
    pub end: End,
}
#[rustfmt::skip]
pub type Reason = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InvalidEthos_Data {
    pub extent: Extent,
    pub reason: Reason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum GenerationRefusal {
    UnknownSource(signal_ethos_zero::SourceName),
    FileAbsent(signal_ethos_zero::FileLocation),
    ImportUnresolved(signal_ethos_zero::FileLocation),
    InvalidRelativePath(signal_ethos_zero::RelativePath),
    InvalidEthos(InvalidEthos_Data),
    RustProjectionRejected(String),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Observe_Data {
    Assemblies,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Generate(signal_ethos_zero::FileLocation),
    Observe(Observe_Data),
    Subscribe(signal_ethos_zero::FileLocation),
    Unsubscribe(signal_ethos_zero::FileLocation),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Observed_Data {
    Assemblies(std::vec::Vec<Generation>),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Generated(Generation),
    Observed(Observed_Data),
    GenerationRejected(GenerationRefusal),
    GenerationStarted(signal_ethos_zero::FileLocation),
    GenerationCompleted(Generation),
    GenerationRefused(GenerationRefusal),
}
