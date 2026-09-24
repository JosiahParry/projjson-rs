use projjson::{
    ProjJson, ProjJsonError,
    validate::{ValidationError, ValidationErrorKind},
};

const ELLIPSOID: &str =
    r#"{"name":"WGS 84","semi_major_axis":6378137,"inverse_flattening":298.257223563}"#;

fn geographic(extra: &str) -> String {
    format!(r#"{{"type":"GeographicCRS","name":"WGS 84"{extra}}}"#)
}

fn datum(extra: &str) -> String {
    format!(
        r#","datum":{{"type":"GeodeticReferenceFrame","name":"WGS84","ellipsoid":{ELLIPSOID}{extra}}}"#
    )
}

fn ensemble() -> String {
    format!(
        r#","datum_ensemble":{{"name":"WGS 84 ensemble","members":[],"ellipsoid":{ELLIPSOID},"accuracy":"2.0"}}"#
    )
}

const ID: &str = r#","id":{"authority":"EPSG","code":4326}"#;
const IDS: &str = r#","ids":[{"authority":"EPSG","code":4326}]"#;

fn invalid(json: &str) -> ValidationError {
    match json.parse::<ProjJson>() {
        Err(ProjJsonError::Invalid(e)) => e,
        other => panic!("expected a validation error, got {other:?}"),
    }
}

#[test]
fn valid_geographic() {
    geographic(&datum("")).parse::<ProjJson>().unwrap();
    geographic(&ensemble()).parse::<ProjJson>().unwrap();
}

#[test]
fn id_and_ids() {
    let e = invalid(&geographic(&format!("{}{ID}{IDS}", datum(""))));
    assert_eq!(e.kind, ValidationErrorKind::IdAndIds);
    assert_eq!(e.location.to_string(), "$");
}

#[test]
fn nested_id_and_ids_reports_path() {
    let e = invalid(&geographic(&datum(&format!("{ID}{IDS}"))));
    assert_eq!(e.kind, ValidationErrorKind::IdAndIds);
    assert_eq!(e.location.to_string(), "$.datum");

    let compound = format!(
        r#"{{"type":"CompoundCRS","name":"c","components":[{},{}]}}"#,
        geographic(&datum("")),
        geographic(&datum(&format!("{ID}{IDS}"))),
    );
    assert_eq!(
        invalid(&compound).location.to_string(),
        "$.components[1].datum"
    );
}

#[test]
fn id_and_ids_in_custom_unit() {
    let unit =
        format!(r#"{{"type":"LinearUnit","name":"foot","conversion_factor":0.3048{ID}{IDS}}}"#);
    let json = format!(
        r#"{{"type":"Ellipsoid","name":"e","semi_major_axis":{{"value":1,"unit":{unit}}},"inverse_flattening":300}}"#
    );
    let e = invalid(&json);
    assert_eq!(e.kind, ValidationErrorKind::IdAndIds);
    assert_eq!(e.location.to_string(), "$.semi_major_axis.unit");
}

#[test]
fn datum_and_datum_ensemble() {
    let e = invalid(&geographic(&format!("{}{}", datum(""), ensemble())));
    assert_eq!(e.kind, ValidationErrorKind::DatumAndDatumEnsemble);
}

#[test]
fn missing_datum() {
    let e = invalid(&geographic(""));
    assert_eq!(e.kind, ValidationErrorKind::MissingDatum);
}

#[test]
fn geoid_model_and_geoid_models() {
    let json = r#"{"type":"VerticalCRS","name":"v",
        "datum":{"type":"VerticalReferenceFrame","name":"vd"},
        "geoid_model":{"name":"g"},
        "geoid_models":[{"name":"g"}]}"#;
    let e = invalid(json);
    assert_eq!(e.kind, ValidationErrorKind::GeoidModelAndGeoidModels);
}

#[test]
fn error_message() {
    let e = invalid(&geographic(&datum(&format!("{ID}{IDS}"))));
    assert_eq!(
        e.to_string(),
        "$.datum: `id` and `ids` are mutually exclusive"
    );
}
