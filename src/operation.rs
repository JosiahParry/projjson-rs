//! Coordinate operations: conversions, transformations and their parameters.

use serde::{Deserialize, Serialize};

use crate::{common::Unit, crs::Crs, number::Number};

/// The `type` of a [`Method`]: `"OperationMethod"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MethodType {
    OperationMethod,
}

/// The `type` of a [`ParameterValue`]: `"ParameterValue"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParameterValueType {
    ParameterValue,
}

/// The `type` of a [`Conversion`]: `"Conversion"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConversionType {
    Conversion,
}

/// The `type` of a [`Transformation`]: `"Transformation"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransformationType {
    Transformation,
}

/// The `type` of an [`AbridgedTransformation`]: `"AbridgedTransformation"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AbridgedTransformationType {
    AbridgedTransformation,
}

/// The `type` of a [`PointMotionOperation`]: `"PointMotionOperation"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PointMotionOperationType {
    PointMotionOperation,
}

/// The `type` of a [`ConcatenatedOperation`]: `"ConcatenatedOperation"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConcatenatedOperationType {
    ConcatenatedOperation,
}

identified_object! {
    pub struct Method {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<MethodType>,
        pub name: String,
    }
}

/// A parameter value: a number, or a string such as a grid file name.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterValueKind {
    Number(Number),
    String(String),
}

identified_object! {
    pub struct ParameterValue {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<ParameterValueType>,
        pub name: String,
        pub value: ParameterValueKind,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unit: Option<Unit>,
    }
}

identified_object! {
    pub struct Conversion {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<ConversionType>,
        pub name: String,
        pub method: Method,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parameters: Option<Vec<ParameterValue>>,
    }
}

usage_object! {
    pub struct Transformation {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<TransformationType>,
        pub name: String,
        pub source_crs: Crs,
        pub target_crs: Crs,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub interpolation_crs: Option<Crs>,
        pub method: Method,
        pub parameters: Vec<ParameterValue>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub accuracy: Option<String>,
    }
}

identified_object! {
    /// The transformation of a bound CRS.
    pub struct AbridgedTransformation {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<AbridgedTransformationType>,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub source_crs: Option<Crs>,
        pub method: Method,
        pub parameters: Vec<ParameterValue>,
    }
}

usage_object! {
    pub struct PointMotionOperation {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<PointMotionOperationType>,
        pub name: String,
        pub source_crs: Crs,
        pub method: Method,
        pub parameters: Vec<ParameterValue>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub accuracy: Option<String>,
    }
}

/// A conversion, transformation or point motion operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SingleOperation {
    Conversion(Box<Conversion>),
    Transformation(Box<Transformation>),
    PointMotionOperation(Box<PointMotionOperation>),
}

usage_object! {
    pub struct ConcatenatedOperation {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub r#type: Option<ConcatenatedOperationType>,
        pub name: String,
        pub source_crs: Crs,
        pub target_crs: Crs,
        pub steps: Vec<SingleOperation>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub accuracy: Option<String>,
    }
}
