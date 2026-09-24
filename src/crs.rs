//! Coordinate reference systems.

use serde::{Deserialize, Serialize};

use crate::{
    cs::CoordinateSystem,
    datum::{
        DatumEnsemble, DeformationModel, EngineeringDatum, GeodeticDatum, GeoidModel,
        ParametricDatum, TemporalDatum, VerticalDatum,
    },
    operation::{AbridgedTransformation, Conversion},
};

/// The `type` of a [`GeodeticCrs`]: `"GeodeticCRS"` or `"GeographicCRS"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeodeticCrsType {
    #[serde(rename = "GeodeticCRS")]
    GeodeticCrs,
    #[serde(rename = "GeographicCRS")]
    GeographicCrs,
}

/// The `type` of a [`ProjectedCrs`]: `"ProjectedCRS"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectedCrsType {
    #[serde(rename = "ProjectedCRS")]
    ProjectedCrs,
}

/// The `type` of a [`VerticalCrs`]: `"VerticalCRS"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerticalCrsType {
    #[serde(rename = "VerticalCRS")]
    VerticalCrs,
}

/// The `type` of a [`CompoundCrs`]: `"CompoundCRS"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompoundCrsType {
    #[serde(rename = "CompoundCRS")]
    CompoundCrs,
}

/// The `type` of a [`BoundCrs`]: `"BoundCRS"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BoundCrsType {
    #[serde(rename = "BoundCRS")]
    BoundCrs,
}

/// The `type` of an [`EngineeringCrs`]: `"EngineeringCRS"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EngineeringCrsType {
    #[serde(rename = "EngineeringCRS")]
    EngineeringCrs,
}

/// The `type` of a [`ParametricCrs`]: `"ParametricCRS"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParametricCrsType {
    #[serde(rename = "ParametricCRS")]
    ParametricCrs,
}

/// The `type` of a [`TemporalCrs`]: `"TemporalCRS"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TemporalCrsType {
    #[serde(rename = "TemporalCRS")]
    TemporalCrs,
}

/// The `type` of a [`DerivedGeodeticCrs`]: `"DerivedGeodeticCRS"` or `"DerivedGeographicCRS"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DerivedGeodeticCrsType {
    #[serde(rename = "DerivedGeodeticCRS")]
    DerivedGeodeticCrs,
    #[serde(rename = "DerivedGeographicCRS")]
    DerivedGeographicCrs,
}

/// The `type` of a [`DerivedProjectedCrs`]: `"DerivedProjectedCRS"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DerivedProjectedCrsType {
    #[serde(rename = "DerivedProjectedCRS")]
    DerivedProjectedCrs,
}

/// The `type` of a [`DerivedVerticalCrs`]: `"DerivedVerticalCRS"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DerivedVerticalCrsType {
    #[serde(rename = "DerivedVerticalCRS")]
    DerivedVerticalCrs,
}

/// The `type` of a [`DerivedEngineeringCrs`]: `"DerivedEngineeringCRS"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DerivedEngineeringCrsType {
    #[serde(rename = "DerivedEngineeringCRS")]
    DerivedEngineeringCrs,
}

/// The `type` of a [`DerivedParametricCrs`]: `"DerivedParametricCRS"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DerivedParametricCrsType {
    #[serde(rename = "DerivedParametricCRS")]
    DerivedParametricCrs,
}

/// The `type` of a [`DerivedTemporalCrs`]: `"DerivedTemporalCRS"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DerivedTemporalCrsType {
    #[serde(rename = "DerivedTemporalCRS")]
    DerivedTemporalCrs,
}

/// Any coordinate reference system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Crs {
    Geodetic(Box<GeodeticCrs>),
    Projected(Box<ProjectedCrs>),
    Compound(Box<CompoundCrs>),
    Bound(Box<BoundCrs>),
    Vertical(Box<VerticalCrs>),
    Engineering(Box<EngineeringCrs>),
    Parametric(Box<ParametricCrs>),
    Temporal(Box<TemporalCrs>),
    DerivedGeodetic(Box<DerivedGeodeticCrs>),
    DerivedProjected(Box<DerivedProjectedCrs>),
    DerivedVertical(Box<DerivedVerticalCrs>),
    DerivedEngineering(Box<DerivedEngineeringCrs>),
    DerivedParametric(Box<DerivedParametricCrs>),
    DerivedTemporal(Box<DerivedTemporalCrs>),
}

impl Crs {
    pub fn name(&self) -> Option<&str> {
        match self {
            Crs::Geodetic(c) => Some(&c.name),
            Crs::Projected(c) => Some(&c.name),
            Crs::Compound(c) => Some(&c.name),
            Crs::Bound(c) => c.name.as_deref(),
            Crs::Vertical(c) => Some(&c.name),
            Crs::Engineering(c) => Some(&c.name),
            Crs::Parametric(c) => Some(&c.name),
            Crs::Temporal(c) => Some(&c.name),
            Crs::DerivedGeodetic(c) => Some(&c.name),
            Crs::DerivedProjected(c) => Some(&c.name),
            Crs::DerivedVertical(c) => Some(&c.name),
            Crs::DerivedEngineering(c) => Some(&c.name),
            Crs::DerivedParametric(c) => Some(&c.name),
            Crs::DerivedTemporal(c) => Some(&c.name),
        }
    }
}

usage_object! {
    /// Geodetic or geographic CRS.
    pub struct GeodeticCrs {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<GeodeticCrsType>,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub datum: Option<GeodeticDatum>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub datum_ensemble: Option<DatumEnsemble>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coordinate_system: Option<CoordinateSystem>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deformation_models: Option<Vec<DeformationModel>>,
    }
}

usage_object! {
    pub struct ProjectedCrs {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<ProjectedCrsType>,
        pub name: String,
        pub base_crs: GeodeticCrs,
        pub conversion: Conversion,
        pub coordinate_system: CoordinateSystem,
    }
}

usage_object! {
    pub struct VerticalCrs {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<VerticalCrsType>,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub datum: Option<VerticalDatum>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub datum_ensemble: Option<DatumEnsemble>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coordinate_system: Option<CoordinateSystem>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub geoid_model: Option<GeoidModel>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub geoid_models: Option<Vec<GeoidModel>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deformation_models: Option<Vec<DeformationModel>>,
    }
}

usage_object! {
    pub struct CompoundCrs {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<CompoundCrsType>,
        pub name: String,
        pub components: Vec<Crs>,
    }
}

usage_object! {
    pub struct BoundCrs {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<BoundCrsType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        pub source_crs: Crs,
        pub target_crs: Crs,
        pub transformation: AbridgedTransformation,
    }
}

usage_object! {
    pub struct EngineeringCrs {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<EngineeringCrsType>,
        pub name: String,
        pub datum: EngineeringDatum,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coordinate_system: Option<CoordinateSystem>,
    }
}

usage_object! {
    pub struct ParametricCrs {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<ParametricCrsType>,
        pub name: String,
        pub datum: ParametricDatum,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coordinate_system: Option<CoordinateSystem>,
    }
}

usage_object! {
    pub struct TemporalCrs {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<TemporalCrsType>,
        pub name: String,
        pub datum: TemporalDatum,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub coordinate_system: Option<CoordinateSystem>,
    }
}

usage_object! {
    /// Derived geodetic or derived geographic CRS.
    pub struct DerivedGeodeticCrs {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<DerivedGeodeticCrsType>,
        pub name: String,
        pub base_crs: GeodeticCrs,
        pub conversion: Conversion,
        pub coordinate_system: CoordinateSystem,
    }
}

usage_object! {
    pub struct DerivedProjectedCrs {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<DerivedProjectedCrsType>,
        pub name: String,
        pub base_crs: ProjectedCrs,
        pub conversion: Conversion,
        pub coordinate_system: CoordinateSystem,
    }
}

usage_object! {
    pub struct DerivedVerticalCrs {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<DerivedVerticalCrsType>,
        pub name: String,
        pub base_crs: VerticalCrs,
        pub conversion: Conversion,
        pub coordinate_system: CoordinateSystem,
    }
}

usage_object! {
    pub struct DerivedEngineeringCrs {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<DerivedEngineeringCrsType>,
        pub name: String,
        pub base_crs: EngineeringCrs,
        pub conversion: Conversion,
        pub coordinate_system: CoordinateSystem,
    }
}

usage_object! {
    pub struct DerivedParametricCrs {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<DerivedParametricCrsType>,
        pub name: String,
        pub base_crs: ParametricCrs,
        pub conversion: Conversion,
        pub coordinate_system: CoordinateSystem,
    }
}

usage_object! {
    pub struct DerivedTemporalCrs {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<DerivedTemporalCrsType>,
        pub name: String,
        pub base_crs: TemporalCrs,
        pub conversion: Conversion,
        pub coordinate_system: CoordinateSystem,
    }
}
