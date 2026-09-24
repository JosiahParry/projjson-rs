//! Datums, ellipsoids, prime meridians and datum ensembles.

use serde::{Deserialize, Serialize};

use crate::{
    common::{Id, Measure},
    crs::Crs,
    number::Number,
};

/// The `type` of an [`Ellipsoid`]: `"Ellipsoid"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EllipsoidType {
    Ellipsoid,
}

/// The `type` of a [`PrimeMeridian`]: `"PrimeMeridian"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrimeMeridianType {
    PrimeMeridian,
}

/// The `type` of a [`DatumEnsemble`]: `"DatumEnsemble"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DatumEnsembleType {
    DatumEnsemble,
}

/// The `type` of a [`GeodeticReferenceFrame`]: `"GeodeticReferenceFrame"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeodeticReferenceFrameType {
    GeodeticReferenceFrame,
}

/// The `type` of a [`DynamicGeodeticReferenceFrame`]: `"DynamicGeodeticReferenceFrame"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DynamicGeodeticReferenceFrameType {
    DynamicGeodeticReferenceFrame,
}

/// The `type` of a [`VerticalReferenceFrame`]: `"VerticalReferenceFrame"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerticalReferenceFrameType {
    VerticalReferenceFrame,
}

/// The `type` of a [`DynamicVerticalReferenceFrame`]: `"DynamicVerticalReferenceFrame"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DynamicVerticalReferenceFrameType {
    DynamicVerticalReferenceFrame,
}

/// The `type` of a [`TemporalDatum`]: `"TemporalDatum"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TemporalDatumType {
    TemporalDatum,
}

/// The `type` of a [`ParametricDatum`]: `"ParametricDatum"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParametricDatumType {
    ParametricDatum,
}

/// The `type` of an [`EngineeringDatum`]: `"EngineeringDatum"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EngineeringDatumType {
    EngineeringDatum,
}

/// An ellipsoid, in one of the three shapes allowed by the schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Ellipsoid {
    InverseFlattening(FlattenedEllipsoid),
    SemiMinorAxis(TwoAxisEllipsoid),
    Sphere(Sphere),
}

identified_object! {
    /// Ellipsoid given by its semi-major axis and inverse flattening.
    pub struct FlattenedEllipsoid {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<EllipsoidType>,
        pub name: String,
        pub semi_major_axis: Measure,
        pub inverse_flattening: Number,
    }
}

identified_object! {
    /// Ellipsoid given by its semi-major and semi-minor axes.
    pub struct TwoAxisEllipsoid {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<EllipsoidType>,
        pub name: String,
        pub semi_major_axis: Measure,
        pub semi_minor_axis: Measure,
    }
}

identified_object! {
    /// Sphere given by its radius.
    pub struct Sphere {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<EllipsoidType>,
        pub name: String,
        pub radius: Measure,
    }
}

identified_object! {
    pub struct PrimeMeridian {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<PrimeMeridianType>,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub longitude: Option<Measure>,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnsembleMember {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<Id>>,
}

identified_object! {
    pub struct DatumEnsemble {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<DatumEnsembleType>,
        pub name: String,
        /// Required by the schema, but PROJ accepts ensembles without members.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub members: Option<Vec<EnsembleMember>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ellipsoid: Option<Ellipsoid>,
        pub accuracy: String,
    }
}

usage_object! {
    pub struct GeodeticReferenceFrame {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<GeodeticReferenceFrameType>,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub anchor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub anchor_epoch: Option<Number>,
        pub ellipsoid: Ellipsoid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prime_meridian: Option<PrimeMeridian>,
    }
}

usage_object! {
    pub struct DynamicGeodeticReferenceFrame {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<DynamicGeodeticReferenceFrameType>,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub anchor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub anchor_epoch: Option<Number>,
        pub frame_reference_epoch: Number,
        pub ellipsoid: Ellipsoid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prime_meridian: Option<PrimeMeridian>,
    }
}

usage_object! {
    pub struct VerticalReferenceFrame {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<VerticalReferenceFrameType>,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub anchor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub anchor_epoch: Option<Number>,
    }
}

usage_object! {
    pub struct DynamicVerticalReferenceFrame {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<DynamicVerticalReferenceFrameType>,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub anchor: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub anchor_epoch: Option<Number>,
        pub frame_reference_epoch: Number,
    }
}

usage_object! {
    pub struct TemporalDatum {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<TemporalDatumType>,
        pub name: String,
        pub calendar: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub time_origin: Option<String>,
    }
}

usage_object! {
    pub struct ParametricDatum {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<ParametricDatumType>,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub anchor: Option<String>,
    }
}

usage_object! {
    pub struct EngineeringDatum {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<EngineeringDatumType>,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub anchor: Option<String>,
    }
}

/// Datum of a geodetic CRS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeodeticDatum {
    GeodeticReferenceFrame(GeodeticReferenceFrame),
    DynamicGeodeticReferenceFrame(DynamicGeodeticReferenceFrame),
}

/// Datum of a vertical CRS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerticalDatum {
    VerticalReferenceFrame(VerticalReferenceFrame),
    DynamicVerticalReferenceFrame(DynamicVerticalReferenceFrame),
}

/// Any datum.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Datum {
    GeodeticReferenceFrame(GeodeticReferenceFrame),
    DynamicGeodeticReferenceFrame(DynamicGeodeticReferenceFrame),
    VerticalReferenceFrame(VerticalReferenceFrame),
    DynamicVerticalReferenceFrame(DynamicVerticalReferenceFrame),
    TemporalDatum(TemporalDatum),
    ParametricDatum(ParametricDatum),
    EngineeringDatum(EngineeringDatum),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeformationModel {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeoidModel {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolation_crs: Option<Crs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
}
