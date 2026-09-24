//! Identifiers, units, measures and usage extents shared by all objects.

use serde::{Deserialize, Serialize};

use crate::number::Number;

/// Authority code: `"4326"` or `4326`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Code {
    Integer(i64),
    String(String),
}

/// Identifier version: a string or a number.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Version {
    Number(Number),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Id {
    pub authority: String,
    pub code: Code,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority_citation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// Units that may be written as a bare string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PredefinedUnit {
    Metre,
    Degree,
    Unity,
}

/// The `type` of a [`CustomUnit`], which is required and gives the kind of quantity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnitType {
    LinearUnit,
    AngularUnit,
    ScaleUnit,
    TimeUnit,
    ParametricUnit,
    Unit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomUnit {
    pub r#type: UnitType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_factor: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<Id>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Unit {
    Predefined(PredefinedUnit),
    Custom(CustomUnit),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValueAndUnit {
    pub value: Number,
    pub unit: Unit,
}

/// A number in the default unit of its context (metre or degree), or a value with its unit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Measure {
    Value(Number),
    ValueAndUnit(ValueAndUnit),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BBox {
    pub south_latitude: Number,
    pub west_longitude: Number,
    pub north_latitude: Number,
    pub east_longitude: Number,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerticalExtent {
    pub minimum: Number,
    pub maximum: Number,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemporalExtent {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Usage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<BBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_extent: Option<VerticalExtent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_extent: Option<TemporalExtent>,
}
