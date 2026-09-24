//! Schema constraints that the types alone do not enforce.
//!
//! * `id` and `ids` are mutually exclusive (all identified objects).
//! * A geodetic or vertical CRS has exactly one of `datum` and `datum_ensemble`.
//! * A vertical CRS does not have both `geoid_model` and `geoid_models`.

use std::fmt;

use crate::{
    CoordinateMetadata, ProjJson,
    common::{CustomUnit, Id, Measure, Unit, Usage, ValueAndUnit, VerticalExtent},
    crs::*,
    cs::{Axis, CoordinateSystem, Meridian},
    datum::*,
    operation::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationErrorKind {
    /// Both `id` and `ids` are present.
    IdAndIds,
    /// Both `datum` and `datum_ensemble` are present.
    DatumAndDatumEnsemble,
    /// Neither `datum` nor `datum_ensemble` is present.
    MissingDatum,
    /// Both `geoid_model` and `geoid_models` are present.
    GeoidModelAndGeoidModels,
}

impl fmt::Display for ValidationErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::IdAndIds => "`id` and `ids` are mutually exclusive",
            Self::DatumAndDatumEnsemble => "`datum` and `datum_ensemble` are mutually exclusive",
            Self::MissingDatum => "one of `datum` or `datum_ensemble` is required",
            Self::GeoidModelAndGeoidModels => {
                "`geoid_model` and `geoid_models` are mutually exclusive"
            }
        })
    }
}

/// One step into a document: an object member or an array index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Segment {
    Field(&'static str),
    Index(usize),
}

/// Where in the document an error was found, displayed as `$.components[1].datum`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Location {
    /// Innermost segment first: each parent appends its own as the error is returned.
    segments: Vec<Segment>,
}

impl Location {
    /// Segments from the root of the document down to the offending object.
    pub fn segments(&self) -> impl Iterator<Item = &Segment> {
        self.segments.iter().rev()
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("$")?;
        self.segments().try_for_each(|segment| match segment {
            Segment::Field(name) => write!(f, ".{name}"),
            Segment::Index(i) => write!(f, "[{i}]"),
        })
    }
}

/// A schema constraint violation and where it was found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub location: Location,
    pub kind: ValidationErrorKind,
}

impl ValidationError {
    fn new(kind: ValidationErrorKind) -> Self {
        Self {
            location: Location::default(),
            kind,
        }
    }

    pub(crate) fn in_field(mut self, name: &'static str) -> Self {
        self.location.segments.push(Segment::Field(name));
        self
    }

    fn in_index(mut self, i: usize) -> Self {
        self.location.segments.push(Segment::Index(i));
        self
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.location, self.kind)
    }
}

impl std::error::Error for ValidationError {}

/// Check the schema constraints of an object and everything it contains.
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}

impl<T: Validate> Validate for Option<T> {
    fn validate(&self) -> Result<(), ValidationError> {
        self.as_ref().map_or(Ok(()), T::validate)
    }
}

impl<T: Validate> Validate for Vec<T> {
    fn validate(&self) -> Result<(), ValidationError> {
        self.iter()
            .enumerate()
            .try_for_each(|(i, v)| v.validate().map_err(|e| e.in_index(i)))
    }
}

impl<T: Validate> Validate for Box<T> {
    fn validate(&self) -> Result<(), ValidationError> {
        (**self).validate()
    }
}

pub(crate) fn check_id_ids(id: &Option<Id>, ids: &Option<Vec<Id>>) -> Result<(), ValidationError> {
    if id.is_some() && ids.is_some() {
        return Err(ValidationError::new(ValidationErrorKind::IdAndIds));
    }
    Ok(())
}

fn check_datum<D, E>(datum: &Option<D>, ensemble: &Option<E>) -> Result<(), ValidationError> {
    match (datum.is_some(), ensemble.is_some()) {
        (true, true) => Err(ValidationError::new(
            ValidationErrorKind::DatumAndDatumEnsemble,
        )),
        (false, false) => Err(ValidationError::new(ValidationErrorKind::MissingDatum)),
        _ => Ok(()),
    }
}

/// Validate the common members, then each listed member.
macro_rules! validate_members {
    ($($ty:ty { $($field:ident),* $(,)? })+) => {
        $(
            impl Validate for $ty {
                fn validate(&self) -> Result<(), ValidationError> {
                    self.validate_common()?;
                    $(
                        self.$field
                            .validate()
                            .map_err(|e| e.in_field(stringify!($field)))?;
                    )*
                    Ok(())
                }
            }
        )+
    };
}

/// Validate whichever variant is present.
macro_rules! validate_variants {
    ($($ty:ident { $($variant:ident),+ $(,)? })+) => {
        $(
            impl Validate for $ty {
                fn validate(&self) -> Result<(), ValidationError> {
                    match self {
                        $( $ty::$variant(v) => v.validate(), )+
                    }
                }
            }
        )+
    };
}

// Common objects without `$schema`

impl Validate for CustomUnit {
    fn validate(&self) -> Result<(), ValidationError> {
        check_id_ids(&self.id, &self.ids)
    }
}

impl Validate for Unit {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            Unit::Predefined(_) => Ok(()),
            Unit::Custom(u) => u.validate(),
        }
    }
}

impl Validate for ValueAndUnit {
    fn validate(&self) -> Result<(), ValidationError> {
        self.unit.validate().map_err(|e| e.in_field("unit"))
    }
}

impl Validate for Measure {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            Measure::Value(_) => Ok(()),
            Measure::ValueAndUnit(v) => v.validate(),
        }
    }
}

impl Validate for VerticalExtent {
    fn validate(&self) -> Result<(), ValidationError> {
        self.unit.validate().map_err(|e| e.in_field("unit"))
    }
}

impl Validate for Usage {
    fn validate(&self) -> Result<(), ValidationError> {
        self.vertical_extent
            .validate()
            .map_err(|e| e.in_field("vertical_extent"))
    }
}

impl Validate for EnsembleMember {
    fn validate(&self) -> Result<(), ValidationError> {
        check_id_ids(&self.id, &self.ids)
    }
}

impl Validate for GeoidModel {
    fn validate(&self) -> Result<(), ValidationError> {
        self.interpolation_crs
            .validate()
            .map_err(|e| e.in_field("interpolation_crs"))
    }
}

impl Validate for CoordinateMetadata {
    fn validate(&self) -> Result<(), ValidationError> {
        self.crs.validate().map_err(|e| e.in_field("crs"))
    }
}

// CRSs with a datum or datum ensemble

impl Validate for GeodeticCrs {
    fn validate(&self) -> Result<(), ValidationError> {
        self.validate_common()?;
        check_datum(&self.datum, &self.datum_ensemble)?;
        self.datum.validate().map_err(|e| e.in_field("datum"))?;
        self.datum_ensemble
            .validate()
            .map_err(|e| e.in_field("datum_ensemble"))?;
        self.coordinate_system
            .validate()
            .map_err(|e| e.in_field("coordinate_system"))
    }
}

impl Validate for VerticalCrs {
    fn validate(&self) -> Result<(), ValidationError> {
        self.validate_common()?;
        check_datum(&self.datum, &self.datum_ensemble)?;
        if self.geoid_model.is_some() && self.geoid_models.is_some() {
            return Err(ValidationError::new(
                ValidationErrorKind::GeoidModelAndGeoidModels,
            ));
        }
        self.datum.validate().map_err(|e| e.in_field("datum"))?;
        self.datum_ensemble
            .validate()
            .map_err(|e| e.in_field("datum_ensemble"))?;
        self.coordinate_system
            .validate()
            .map_err(|e| e.in_field("coordinate_system"))?;
        self.geoid_model
            .validate()
            .map_err(|e| e.in_field("geoid_model"))?;
        self.geoid_models
            .validate()
            .map_err(|e| e.in_field("geoid_models"))
    }
}

validate_members! {
    CoordinateSystem { axis }
    Axis { meridian, unit }
    Meridian { longitude }

    FlattenedEllipsoid { semi_major_axis }
    TwoAxisEllipsoid { semi_major_axis, semi_minor_axis }
    Sphere { radius }
    PrimeMeridian { longitude }
    DatumEnsemble { members, ellipsoid }
    GeodeticReferenceFrame { ellipsoid, prime_meridian }
    DynamicGeodeticReferenceFrame { ellipsoid, prime_meridian }
    VerticalReferenceFrame {}
    DynamicVerticalReferenceFrame {}
    TemporalDatum {}
    ParametricDatum {}
    EngineeringDatum {}

    ProjectedCrs { base_crs, conversion, coordinate_system }
    CompoundCrs { components }
    BoundCrs { source_crs, target_crs, transformation }
    EngineeringCrs { datum, coordinate_system }
    ParametricCrs { datum, coordinate_system }
    TemporalCrs { datum, coordinate_system }
    DerivedGeodeticCrs { base_crs, conversion, coordinate_system }
    DerivedProjectedCrs { base_crs, conversion, coordinate_system }
    DerivedVerticalCrs { base_crs, conversion, coordinate_system }
    DerivedEngineeringCrs { base_crs, conversion, coordinate_system }
    DerivedParametricCrs { base_crs, conversion, coordinate_system }
    DerivedTemporalCrs { base_crs, conversion, coordinate_system }

    Method {}
    ParameterValue { unit }
    Conversion { method, parameters }
    Transformation { source_crs, target_crs, interpolation_crs, method, parameters }
    AbridgedTransformation { source_crs, method, parameters }
    PointMotionOperation { source_crs, method, parameters }
    ConcatenatedOperation { source_crs, target_crs, steps }
}

validate_variants! {
    Ellipsoid { InverseFlattening, SemiMinorAxis, Sphere }
    GeodeticDatum { GeodeticReferenceFrame, DynamicGeodeticReferenceFrame }
    VerticalDatum { VerticalReferenceFrame, DynamicVerticalReferenceFrame }
    Datum {
        GeodeticReferenceFrame,
        DynamicGeodeticReferenceFrame,
        VerticalReferenceFrame,
        DynamicVerticalReferenceFrame,
        TemporalDatum,
        ParametricDatum,
        EngineeringDatum,
    }
    Crs {
        Geodetic,
        Projected,
        Compound,
        Bound,
        Vertical,
        Engineering,
        Parametric,
        Temporal,
        DerivedGeodetic,
        DerivedProjected,
        DerivedVertical,
        DerivedEngineering,
        DerivedParametric,
        DerivedTemporal,
    }
    SingleOperation { Conversion, Transformation, PointMotionOperation }
    ProjJson {
        Crs,
        Datum,
        DatumEnsemble,
        Ellipsoid,
        PrimeMeridian,
        SingleOperation,
        ConcatenatedOperation,
        CoordinateMetadata,
    }
}
