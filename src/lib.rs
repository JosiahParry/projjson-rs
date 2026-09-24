//! Typed read/write support for [PROJJSON](https://proj.org/en/latest/specifications/projjson.html)
//! (schema v0.7).
//!
//! Every member of the schema has a Rust type, and each object keeps the form it was
//! read in (optional `type`, `id` vs `ids`, number vs value-and-unit, integer vs float),
//! so reading and writing a document gives back the same JSON.
//!
//! ```
//! let json = r#"{"type":"GeographicCRS","name":"WGS 84","datum":{"type":"GeodeticReferenceFrame","name":"World Geodetic System 1984","ellipsoid":{"name":"WGS 84","semi_major_axis":6378137,"inverse_flattening":298.257223563}},"id":{"authority":"EPSG","code":4326}}"#;
//!
//! let doc: projjson::ProjJson = json.parse().unwrap();
//! assert_eq!(doc.to_string(), json);
//! ```
//!
//! # The `type` member
//!
//! Each PROJJSON object may carry a `type` string naming what it is, such as
//! `{"type": "Axis", ...}`. It is optional, so every struct has an
//! `r#type: Option<...Type>` field: `None` when the document leaves it out, in which
//! case it is also left out when written back. The `...Type` enums list the strings
//! the schema allows for that object (usually one, e.g. [`cs::AxisType::Axis`] is
//! `"Axis"`), so a wrong `type` is rejected. They also let [`crs::Crs`] and the other
//! enums tell apart objects whose other members look alike.

#[macro_use]
mod macros;

pub mod common;
pub mod crs;
pub mod cs;
pub mod datum;
pub mod number;
pub mod operation;
pub mod validate;

use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{
    crs::Crs,
    datum::{Datum, DatumEnsemble, Ellipsoid, PrimeMeridian},
    number::Number,
    operation::{ConcatenatedOperation, SingleOperation},
    validate::{Validate, ValidationError},
};

/// The `type` of a [`CoordinateMetadata`]: `"CoordinateMetadata"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CoordinateMetadataType {
    CoordinateMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoordinateMetadata {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CoordinateMetadataType>,
    pub crs: Crs,
    #[serde(rename = "coordinateEpoch", skip_serializing_if = "Option::is_none")]
    pub coordinate_epoch: Option<Number>,
}

/// A PROJJSON document: any object allowed at the root of the schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjJson {
    Crs(Crs),
    Datum(Box<Datum>),
    DatumEnsemble(Box<DatumEnsemble>),
    Ellipsoid(Box<Ellipsoid>),
    PrimeMeridian(Box<PrimeMeridian>),
    SingleOperation(SingleOperation),
    ConcatenatedOperation(Box<ConcatenatedOperation>),
    CoordinateMetadata(Box<CoordinateMetadata>),
}

#[derive(Debug)]
pub enum ProjJsonError {
    /// Malformed JSON, or JSON that does not match the PROJJSON types.
    Json(serde_json::Error),
    /// A PROJJSON document that breaks a schema constraint.
    Invalid(ValidationError),
}

impl fmt::Display for ProjJsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProjJsonError::Json(e) => e.fmt(f),
            ProjJsonError::Invalid(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for ProjJsonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProjJsonError::Json(e) => Some(e),
            ProjJsonError::Invalid(e) => Some(e),
        }
    }
}

impl From<serde_json::Error> for ProjJsonError {
    fn from(e: serde_json::Error) -> Self {
        ProjJsonError::Json(e)
    }
}

impl From<ValidationError> for ProjJsonError {
    fn from(e: ValidationError) -> Self {
        ProjJsonError::Invalid(e)
    }
}

impl FromStr for ProjJson {
    type Err = ProjJsonError;

    /// Read a PROJJSON document and check its schema constraints.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let doc: ProjJson = serde_json::from_str(s)?;
        doc.validate()?;
        Ok(doc)
    }
}

/// Writes compact JSON, or indented JSON with the alternate flag (`{:#}`).
impl fmt::Display for ProjJson {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let json = if f.alternate() {
            serde_json::to_string_pretty(self)
        } else {
            serde_json::to_string(self)
        };
        f.write_str(&json.map_err(|_| fmt::Error)?)
    }
}
