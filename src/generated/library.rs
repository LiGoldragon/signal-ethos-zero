#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type SourceName = String;
#[rustfmt::skip]
pub type RelativePath = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FileLocation {
    pub source_name: SourceName,
    pub relative_path: RelativePath,
}
