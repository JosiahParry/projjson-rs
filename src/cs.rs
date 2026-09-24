//! Coordinate systems and axes.

use serde::{Deserialize, Serialize};

use crate::{
    common::{Measure, Unit},
    number::Number,
};

/// The `type` of a [`CoordinateSystem`]: `"CoordinateSystem"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CoordinateSystemType {
    CoordinateSystem,
}

/// The `type` of an [`Axis`]: `"Axis"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AxisType {
    Axis,
}

/// The `type` of a [`Meridian`]: `"Meridian"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MeridianType {
    Meridian,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CoordinateSystemSubtype {
    Cartesian,
    #[serde(rename = "spherical")]
    Spherical,
    #[serde(rename = "ellipsoidal")]
    Ellipsoidal,
    #[serde(rename = "vertical")]
    Vertical,
    #[serde(rename = "ordinal")]
    Ordinal,
    #[serde(rename = "parametric")]
    Parametric,
    #[serde(rename = "affine")]
    Affine,
    TemporalDateTime,
    TemporalCount,
    TemporalMeasure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AxisDirection {
    North,
    NorthNorthEast,
    NorthEast,
    EastNorthEast,
    East,
    EastSouthEast,
    SouthEast,
    SouthSouthEast,
    South,
    SouthSouthWest,
    SouthWest,
    WestSouthWest,
    West,
    WestNorthWest,
    NorthWest,
    NorthNorthWest,
    Up,
    Down,
    GeocentricX,
    GeocentricY,
    GeocentricZ,
    ColumnPositive,
    ColumnNegative,
    RowPositive,
    RowNegative,
    DisplayRight,
    DisplayLeft,
    DisplayUp,
    DisplayDown,
    Forward,
    Aft,
    Port,
    Starboard,
    Clockwise,
    CounterClockwise,
    Towards,
    AwayFrom,
    Future,
    Past,
    Unspecified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RangeMeaning {
    Exact,
    Wraparound,
}

identified_object! {
    pub struct CoordinateSystem {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<CoordinateSystemType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        pub subtype: CoordinateSystemSubtype,
        pub axis: Vec<Axis>,
    }
}

identified_object! {
    pub struct Axis {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<AxisType>,
        pub name: String,
        pub abbreviation: String,
        pub direction: AxisDirection,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub meridian: Option<Meridian>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unit: Option<Unit>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub minimum_value: Option<Number>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub maximum_value: Option<Number>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub range_meaning: Option<RangeMeaning>,
    }
}

identified_object! {
    pub struct Meridian {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<MeridianType>,
        pub longitude: Measure,
    }
}
