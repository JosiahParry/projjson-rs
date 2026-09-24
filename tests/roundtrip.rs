//! Read each fixture, write it back, and compare with the input.
//!
//! `serde_json` is built with `preserve_order` here, so the input is re-serialized
//! with its original key order: the comparison checks members, order and numbers.

use std::{fs, path::Path};

use projjson::{
    ProjJson,
    crs::Crs,
    cs::{Axis, CoordinateSystem},
    operation::SingleOperation,
    validate::Validate,
};
use serde::{Serialize, de::DeserializeOwned};

fn roundtrip_as<T: DeserializeOwned + Serialize + Validate>(input: &str) -> Result<String, String> {
    let parsed: T = serde_json::from_str(input).map_err(|e| format!("parse error: {e}"))?;
    parsed
        .validate()
        .map_err(|e| format!("validation error: {e}"))?;
    serde_json::to_string(&parsed).map_err(|e| format!("write error: {e}"))
}

// Axis and CoordinateSystem are not PROJJSON root objects, but PROJ's tests use them as such
fn roundtrip(input: &str, ty: Option<&str>) -> Result<String, String> {
    match ty {
        Some("Axis") => roundtrip_as::<Axis>(input),
        Some("CoordinateSystem") => roundtrip_as::<CoordinateSystem>(input),
        _ => roundtrip_as::<ProjJson>(input),
    }
}

fn first_difference(a: &str, b: &str) -> String {
    let i = a.bytes().zip(b.bytes()).take_while(|(x, y)| x == y).count();
    let start = i.saturating_sub(60);
    format!(
        "\n  expected: ...{}\n       got: ...{}",
        &a[start..(i + 60).min(a.len())],
        &b[start..(i + 60).min(b.len())]
    )
}

fn fixtures(dir: &str) -> Vec<(String, String)> {
    let mut files: Vec<_> = fs::read_dir(Path::new("tests/data").join(dir))
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    files.sort();
    files
        .into_iter()
        .map(|p| {
            let name = p.file_name().unwrap().to_string_lossy().into_owned();
            (name, fs::read_to_string(&p).unwrap())
        })
        .collect()
}

fn check_dir(dir: &str) {
    let mut failures = vec![];
    let cases = fixtures(dir);
    for (name, input) in &cases {
        let value: serde_json::Value = serde_json::from_str(input).unwrap();
        let expected = serde_json::to_string(&value).unwrap();
        match roundtrip(input, value["type"].as_str()) {
            Ok(got) if got == expected => (),
            Ok(got) => failures.push(format!(
                "{name}: output differs{}",
                first_difference(&expected, &got)
            )),
            Err(e) => failures.push(format!("{name}: {e}")),
        }
    }
    assert!(
        failures.is_empty(),
        "{}/{} failed:\n{}",
        failures.len(),
        cases.len(),
        failures.join("\n")
    );
}

#[test]
fn roundtrip_spatialreference() {
    check_dir("projjson");
}

#[test]
fn roundtrip_proj_test_io() {
    check_dir("proj");
}

#[test]
fn parses_to_expected_variant() {
    let read = |dir: &str, name: &str| {
        let path = Path::new("tests/data").join(dir).join(name);
        fs::read_to_string(path)
            .unwrap()
            .parse::<ProjJson>()
            .unwrap()
    };

    assert!(matches!(
        read("projjson", "epsg4326.json"),
        ProjJson::Crs(Crs::Geodetic(_))
    ));
    assert!(matches!(
        read("projjson", "epsg26986.json"),
        ProjJson::Crs(Crs::Projected(_))
    ));
    assert!(matches!(
        read("projjson", "epsg5972.json"),
        ProjJson::Crs(Crs::Compound(_))
    ));
    assert!(matches!(
        read("proj", "bound_crs.json"),
        ProjJson::Crs(Crs::Bound(_))
    ));
    assert!(matches!(
        read("proj", "vertical_crs.json"),
        ProjJson::Crs(Crs::Vertical(_))
    ));
    assert!(matches!(
        read("proj", "engineering_crs.json"),
        ProjJson::Crs(Crs::Engineering(_))
    ));
    assert!(matches!(
        read("proj", "parametric_crs.json"),
        ProjJson::Crs(Crs::Parametric(_))
    ));
    assert!(matches!(
        read("proj", "temporal_crs.json"),
        ProjJson::Crs(Crs::Temporal(_))
    ));
    assert!(matches!(
        read("proj", "derived_geographic_crs.json"),
        ProjJson::Crs(Crs::DerivedGeodetic(_))
    ));
    assert!(matches!(
        read("proj", "derived_projected_crs.json"),
        ProjJson::Crs(Crs::DerivedProjected(_))
    ));
    assert!(matches!(
        read("proj", "transformation.json"),
        ProjJson::SingleOperation(SingleOperation::Transformation(_))
    ));
    assert!(matches!(
        read("proj", "conversion_utm_zone_south_wrong_id.json"),
        ProjJson::SingleOperation(SingleOperation::Conversion(_))
    ));
    assert!(matches!(
        read("proj", "concatenated_operation.json"),
        ProjJson::ConcatenatedOperation(_)
    ));
    assert!(matches!(
        read("proj", "prime_meridian.json"),
        ProjJson::PrimeMeridian(_)
    ));
    assert!(matches!(
        read("proj", "ellipsoid_sphere.json"),
        ProjJson::Ellipsoid(_)
    ));
    assert!(matches!(
        read("proj", "datum_ensemble_without_ellipsoid.json"),
        ProjJson::DatumEnsemble(_)
    ));
    assert!(matches!(
        read("proj", "dynamic_vertical_reference_frame.json"),
        ProjJson::Datum(_)
    ));
}

#[test]
fn reject_proj_invalid() {
    for (name, input) in fixtures("proj-invalid") {
        let value: serde_json::Value = serde_json::from_str(&input).unwrap();
        assert!(
            roundtrip(&input, value["type"].as_str()).is_err(),
            "{name} should be rejected"
        );
    }
}
