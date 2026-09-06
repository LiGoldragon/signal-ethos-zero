#![allow(dead_code)]
#![allow(clippy::redundant_closure)]
pub type SourceName = protos::Text;
pub type RelativePath = protos::Text;
pub type ArtifactPath = protos::Text;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileLocation(pub SourceName, pub RelativePath);
impl datom_codec::Datomic for FileLocation {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: SourceName = datom_codec::Positional::position(&mut p)?;
        let p1: RelativePath = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for FileLocation {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenerationRequest(pub FileLocation);
impl datom_codec::Datomic for GenerationRequest {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 1)?;
        let p0: FileLocation = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0))
    }
}
impl protos::Conceivable<datom_codec::Datom> for GenerationRequest {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubscriptionRequest(pub FileLocation);
impl datom_codec::Datomic for SubscriptionRequest {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 1)?;
        let p0: FileLocation = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0))
    }
}
impl protos::Conceivable<datom_codec::Datom> for SubscriptionRequest {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Generation(pub FileLocation, pub ArtifactPath);
impl datom_codec::Datomic for Generation {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: FileLocation = datom_codec::Positional::position(&mut p)?;
        let p1: ArtifactPath = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for Generation {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObservationSelection {
    Assemblies,
}
impl datom_codec::Datomic for ObservationSelection {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Assemblies" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::Assemblies)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for ObservationSelection {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Assemblies => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("Assemblies")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssemblySummary(pub FileLocation, pub ArtifactPath);
impl datom_codec::Datomic for AssemblySummary {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: FileLocation = datom_codec::Positional::position(&mut p)?;
        let p1: ArtifactPath = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for AssemblySummary {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type Assemblies = std::vec::Vec<AssemblySummary>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssemblySnapshot(pub Assemblies);
impl datom_codec::Datomic for AssemblySnapshot {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 1)?;
        let p0: Assemblies = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0))
    }
}
impl protos::Conceivable<datom_codec::Datom> for AssemblySnapshot {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Observation {
    Assemblies(AssemblySnapshot),
}
impl datom_codec::Datomic for Observation {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Assemblies" => {
                std::result::Result::Ok(
                    Self::Assemblies(datom_codec::Carrying::body(v)?),
                )
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Observation {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Assemblies(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Assemblies")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
pub type ExtentStart = protos::Integer;
pub type ExtentEnd = protos::Integer;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceExtent(pub ExtentStart, pub ExtentEnd);
impl datom_codec::Datomic for SourceExtent {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: ExtentStart = datom_codec::Positional::position(&mut p)?;
        let p1: ExtentEnd = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for SourceExtent {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type SyntaxFaultReason = protos::Text;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyntaxFault(pub SourceExtent, pub SyntaxFaultReason);
impl datom_codec::Datomic for SyntaxFault {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: SourceExtent = datom_codec::Positional::position(&mut p)?;
        let p1: SyntaxFaultReason = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for SyntaxFault {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type ProjectionFaultReason = protos::Text;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionFault(pub ProjectionFaultReason);
impl datom_codec::Datomic for ProjectionFault {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 1)?;
        let p0: ProjectionFaultReason = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0))
    }
}
impl protos::Conceivable<datom_codec::Datom> for ProjectionFault {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GenerationRefusal {
    UnknownSource(SourceName),
    FileAbsent(FileLocation),
    ImportUnresolved(FileLocation),
    InvalidRelativePath(RelativePath),
    InvalidEthos(SyntaxFault),
    RustProjectionRejected(ProjectionFault),
}
impl datom_codec::Datomic for GenerationRefusal {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "UnknownSource" => {
                std::result::Result::Ok(
                    Self::UnknownSource(datom_codec::Carrying::body(v)?),
                )
            }
            "FileAbsent" => {
                std::result::Result::Ok(
                    Self::FileAbsent(datom_codec::Carrying::body(v)?),
                )
            }
            "ImportUnresolved" => {
                std::result::Result::Ok(
                    Self::ImportUnresolved(datom_codec::Carrying::body(v)?),
                )
            }
            "InvalidRelativePath" => {
                std::result::Result::Ok(
                    Self::InvalidRelativePath(datom_codec::Carrying::body(v)?),
                )
            }
            "InvalidEthos" => {
                std::result::Result::Ok(
                    Self::InvalidEthos(datom_codec::Carrying::body(v)?),
                )
            }
            "RustProjectionRejected" => {
                std::result::Result::Ok(
                    Self::RustProjectionRejected(datom_codec::Carrying::body(v)?),
                )
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for GenerationRefusal {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::UnknownSource(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("UnknownSource")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::FileAbsent(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("FileAbsent")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::ImportUnresolved(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("ImportUnresolved")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::InvalidRelativePath(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("InvalidRelativePath")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::InvalidEthos(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("InvalidEthos")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::RustProjectionRejected(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("RustProjectionRejected")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Generate(GenerationRequest),
    Observe(ObservationSelection),
    Subscribe(SubscriptionRequest),
    Unsubscribe(SubscriptionRequest),
}
impl datom_codec::Datomic for Request {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Generate" => {
                std::result::Result::Ok(Self::Generate(datom_codec::Carrying::body(v)?))
            }
            "Observe" => {
                std::result::Result::Ok(Self::Observe(datom_codec::Carrying::body(v)?))
            }
            "Subscribe" => {
                std::result::Result::Ok(Self::Subscribe(datom_codec::Carrying::body(v)?))
            }
            "Unsubscribe" => {
                std::result::Result::Ok(
                    Self::Unsubscribe(datom_codec::Carrying::body(v)?),
                )
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Request {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Generate(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Generate")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Observe(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Observe").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Subscribe(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Subscribe")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Unsubscribe(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Unsubscribe")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Response {
    Generated(Generation),
    Observed(Observation),
    GenerationRejected(GenerationRefusal),
    GenerationStarted(GenerationRequest),
    GenerationCompleted(Generation),
    GenerationRefused(GenerationRefusal),
}
impl datom_codec::Datomic for Response {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Generated" => {
                std::result::Result::Ok(Self::Generated(datom_codec::Carrying::body(v)?))
            }
            "Observed" => {
                std::result::Result::Ok(Self::Observed(datom_codec::Carrying::body(v)?))
            }
            "GenerationRejected" => {
                std::result::Result::Ok(
                    Self::GenerationRejected(datom_codec::Carrying::body(v)?),
                )
            }
            "GenerationStarted" => {
                std::result::Result::Ok(
                    Self::GenerationStarted(datom_codec::Carrying::body(v)?),
                )
            }
            "GenerationCompleted" => {
                std::result::Result::Ok(
                    Self::GenerationCompleted(datom_codec::Carrying::body(v)?),
                )
            }
            "GenerationRefused" => {
                std::result::Result::Ok(
                    Self::GenerationRefused(datom_codec::Carrying::body(v)?),
                )
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Response {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Generated(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Generated")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Observed(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Observed")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::GenerationRejected(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("GenerationRejected")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::GenerationStarted(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("GenerationStarted")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::GenerationCompleted(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("GenerationCompleted")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::GenerationRefused(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("GenerationRefused")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
pub trait WireConversion: Sized {
    type Wire;
    fn into_wire(self) -> Self::Wire;
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault>;
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WireFault {
    Text,
}
pub type SourceNameWire = std::string::String;
pub type RelativePathWire = std::string::String;
pub type ArtifactPathWire = std::string::String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FileLocationWire(pub SourceNameWire, pub RelativePathWire);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GenerationRequestWire(pub FileLocationWire);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SubscriptionRequestWire(pub FileLocationWire);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GenerationWire(pub FileLocationWire, pub ArtifactPathWire);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ObservationSelectionWire {
    Assemblies,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AssemblySummaryWire(pub FileLocationWire, pub ArtifactPathWire);
pub type AssembliesWire = std::vec::Vec<AssemblySummaryWire>;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AssemblySnapshotWire(pub AssembliesWire);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ObservationWire {
    Assemblies(AssemblySnapshotWire),
}
pub type ExtentStartWire = i64;
pub type ExtentEndWire = i64;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SourceExtentWire(pub ExtentStartWire, pub ExtentEndWire);
pub type SyntaxFaultReasonWire = std::string::String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SyntaxFaultWire(pub SourceExtentWire, pub SyntaxFaultReasonWire);
pub type ProjectionFaultReasonWire = std::string::String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProjectionFaultWire(pub ProjectionFaultReasonWire);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum GenerationRefusalWire {
    UnknownSource(SourceNameWire),
    FileAbsent(FileLocationWire),
    ImportUnresolved(FileLocationWire),
    InvalidRelativePath(RelativePathWire),
    InvalidEthos(SyntaxFaultWire),
    RustProjectionRejected(ProjectionFaultWire),
}
impl WireConversion for FileLocation {
    type Wire = FileLocationWire;
    fn into_wire(self) -> Self::Wire {
        let FileLocation(p0, p1) = self;
        FileLocationWire(p0.to_string(), p1.to_string())
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let FileLocationWire(p0, p1) = wire;
        Ok(
            FileLocation(
                protos::Text::try_from(p0).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p1).map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for GenerationRequest {
    type Wire = GenerationRequestWire;
    fn into_wire(self) -> Self::Wire {
        let GenerationRequest(p0) = self;
        GenerationRequestWire(<FileLocation as WireConversion>::into_wire(p0))
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let GenerationRequestWire(p0) = wire;
        Ok(GenerationRequest(<FileLocation as WireConversion>::try_from_wire(p0)?))
    }
}
impl WireConversion for SubscriptionRequest {
    type Wire = SubscriptionRequestWire;
    fn into_wire(self) -> Self::Wire {
        let SubscriptionRequest(p0) = self;
        SubscriptionRequestWire(<FileLocation as WireConversion>::into_wire(p0))
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let SubscriptionRequestWire(p0) = wire;
        Ok(SubscriptionRequest(<FileLocation as WireConversion>::try_from_wire(p0)?))
    }
}
impl WireConversion for Generation {
    type Wire = GenerationWire;
    fn into_wire(self) -> Self::Wire {
        let Generation(p0, p1) = self;
        GenerationWire(<FileLocation as WireConversion>::into_wire(p0), p1.to_string())
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let GenerationWire(p0, p1) = wire;
        Ok(
            Generation(
                <FileLocation as WireConversion>::try_from_wire(p0)?,
                protos::Text::try_from(p1).map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for ObservationSelection {
    type Wire = ObservationSelectionWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            ObservationSelection::Assemblies => ObservationSelectionWire::Assemblies,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            ObservationSelectionWire::Assemblies => Ok(ObservationSelection::Assemblies),
        }
    }
}
impl WireConversion for AssemblySummary {
    type Wire = AssemblySummaryWire;
    fn into_wire(self) -> Self::Wire {
        let AssemblySummary(p0, p1) = self;
        AssemblySummaryWire(
            <FileLocation as WireConversion>::into_wire(p0),
            p1.to_string(),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let AssemblySummaryWire(p0, p1) = wire;
        Ok(
            AssemblySummary(
                <FileLocation as WireConversion>::try_from_wire(p0)?,
                protos::Text::try_from(p1).map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for AssemblySnapshot {
    type Wire = AssemblySnapshotWire;
    fn into_wire(self) -> Self::Wire {
        let AssemblySnapshot(p0) = self;
        AssemblySnapshotWire(
            p0
                .into_iter()
                .map(|value| <AssemblySummary as WireConversion>::into_wire(value))
                .collect(),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let AssemblySnapshotWire(p0) = wire;
        Ok(
            AssemblySnapshot(
                p0
                    .into_iter()
                    .map(|value| <AssemblySummary as WireConversion>::try_from_wire(
                        value,
                    ))
                    .collect::<std::result::Result<std::vec::Vec<_>, WireFault>>()?,
            ),
        )
    }
}
impl WireConversion for Observation {
    type Wire = ObservationWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            Observation::Assemblies(value) => {
                ObservationWire::Assemblies(
                    <AssemblySnapshot as WireConversion>::into_wire(value),
                )
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            ObservationWire::Assemblies(value) => {
                Ok(
                    Observation::Assemblies(
                        <AssemblySnapshot as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
        }
    }
}
impl WireConversion for SourceExtent {
    type Wire = SourceExtentWire;
    fn into_wire(self) -> Self::Wire {
        let SourceExtent(p0, p1) = self;
        SourceExtentWire(p0, p1)
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let SourceExtentWire(p0, p1) = wire;
        Ok(SourceExtent(Ok(p0)?, Ok(p1)?))
    }
}
impl WireConversion for SyntaxFault {
    type Wire = SyntaxFaultWire;
    fn into_wire(self) -> Self::Wire {
        let SyntaxFault(p0, p1) = self;
        SyntaxFaultWire(<SourceExtent as WireConversion>::into_wire(p0), p1.to_string())
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let SyntaxFaultWire(p0, p1) = wire;
        Ok(
            SyntaxFault(
                <SourceExtent as WireConversion>::try_from_wire(p0)?,
                protos::Text::try_from(p1).map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for ProjectionFault {
    type Wire = ProjectionFaultWire;
    fn into_wire(self) -> Self::Wire {
        let ProjectionFault(p0) = self;
        ProjectionFaultWire(p0.to_string())
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let ProjectionFaultWire(p0) = wire;
        Ok(ProjectionFault(protos::Text::try_from(p0).map_err(|_| WireFault::Text)?))
    }
}
impl WireConversion for GenerationRefusal {
    type Wire = GenerationRefusalWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            GenerationRefusal::UnknownSource(value) => {
                GenerationRefusalWire::UnknownSource(value.to_string())
            }
            GenerationRefusal::FileAbsent(value) => {
                GenerationRefusalWire::FileAbsent(
                    <FileLocation as WireConversion>::into_wire(value),
                )
            }
            GenerationRefusal::ImportUnresolved(value) => {
                GenerationRefusalWire::ImportUnresolved(
                    <FileLocation as WireConversion>::into_wire(value),
                )
            }
            GenerationRefusal::InvalidRelativePath(value) => {
                GenerationRefusalWire::InvalidRelativePath(value.to_string())
            }
            GenerationRefusal::InvalidEthos(value) => {
                GenerationRefusalWire::InvalidEthos(
                    <SyntaxFault as WireConversion>::into_wire(value),
                )
            }
            GenerationRefusal::RustProjectionRejected(value) => {
                GenerationRefusalWire::RustProjectionRejected(
                    <ProjectionFault as WireConversion>::into_wire(value),
                )
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            GenerationRefusalWire::UnknownSource(value) => {
                Ok(
                    GenerationRefusal::UnknownSource(
                        protos::Text::try_from(value).map_err(|_| WireFault::Text)?,
                    ),
                )
            }
            GenerationRefusalWire::FileAbsent(value) => {
                Ok(
                    GenerationRefusal::FileAbsent(
                        <FileLocation as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            GenerationRefusalWire::ImportUnresolved(value) => {
                Ok(
                    GenerationRefusal::ImportUnresolved(
                        <FileLocation as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            GenerationRefusalWire::InvalidRelativePath(value) => {
                Ok(
                    GenerationRefusal::InvalidRelativePath(
                        protos::Text::try_from(value).map_err(|_| WireFault::Text)?,
                    ),
                )
            }
            GenerationRefusalWire::InvalidEthos(value) => {
                Ok(
                    GenerationRefusal::InvalidEthos(
                        <SyntaxFault as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            GenerationRefusalWire::RustProjectionRejected(value) => {
                Ok(
                    GenerationRefusal::RustProjectionRejected(
                        <ProjectionFault as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
        }
    }
}
impl WireConversion for Request {
    type Wire = RequestWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            Request::Generate(value) => {
                RequestWire::Generate(
                    <GenerationRequest as WireConversion>::into_wire(value),
                )
            }
            Request::Observe(value) => {
                RequestWire::Observe(
                    <ObservationSelection as WireConversion>::into_wire(value),
                )
            }
            Request::Subscribe(value) => {
                RequestWire::Subscribe(
                    <SubscriptionRequest as WireConversion>::into_wire(value),
                )
            }
            Request::Unsubscribe(value) => {
                RequestWire::Unsubscribe(
                    <SubscriptionRequest as WireConversion>::into_wire(value),
                )
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            RequestWire::Generate(value) => {
                Ok(
                    Request::Generate(
                        <GenerationRequest as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            RequestWire::Observe(value) => {
                Ok(
                    Request::Observe(
                        <ObservationSelection as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            RequestWire::Subscribe(value) => {
                Ok(
                    Request::Subscribe(
                        <SubscriptionRequest as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            RequestWire::Unsubscribe(value) => {
                Ok(
                    Request::Unsubscribe(
                        <SubscriptionRequest as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
        }
    }
}
impl WireConversion for Response {
    type Wire = ResponseWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            Response::Generated(value) => {
                ResponseWire::Generated(<Generation as WireConversion>::into_wire(value))
            }
            Response::Observed(value) => {
                ResponseWire::Observed(<Observation as WireConversion>::into_wire(value))
            }
            Response::GenerationRejected(value) => {
                ResponseWire::GenerationRejected(
                    <GenerationRefusal as WireConversion>::into_wire(value),
                )
            }
            Response::GenerationStarted(value) => {
                ResponseWire::GenerationStarted(
                    <GenerationRequest as WireConversion>::into_wire(value),
                )
            }
            Response::GenerationCompleted(value) => {
                ResponseWire::GenerationCompleted(
                    <Generation as WireConversion>::into_wire(value),
                )
            }
            Response::GenerationRefused(value) => {
                ResponseWire::GenerationRefused(
                    <GenerationRefusal as WireConversion>::into_wire(value),
                )
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            ResponseWire::Generated(value) => {
                Ok(
                    Response::Generated(
                        <Generation as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::Observed(value) => {
                Ok(
                    Response::Observed(
                        <Observation as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::GenerationRejected(value) => {
                Ok(
                    Response::GenerationRejected(
                        <GenerationRefusal as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::GenerationStarted(value) => {
                Ok(
                    Response::GenerationStarted(
                        <GenerationRequest as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::GenerationCompleted(value) => {
                Ok(
                    Response::GenerationCompleted(
                        <Generation as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::GenerationRefused(value) => {
                Ok(
                    Response::GenerationRefused(
                        <GenerationRefusal as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
        }
    }
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum RequestWire {
    Generate(GenerationRequestWire),
    Observe(ObservationSelectionWire),
    Subscribe(SubscriptionRequestWire),
    Unsubscribe(SubscriptionRequestWire),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ResponseWire {
    Generated(GenerationWire),
    Observed(ObservationWire),
    GenerationRejected(GenerationRefusalWire),
    GenerationStarted(GenerationRequestWire),
    GenerationCompleted(GenerationWire),
    GenerationRefused(GenerationRefusalWire),
}
