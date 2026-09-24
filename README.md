# projjson-rs

This repo provides a `serde` compatible representation of `projjson` and schmea validation.

This crate is based on on the [v0.7 schema](https://proj.org/en/latest/schemas/v0.7/projjson.schema.json).

Note that `projjson` only provides a struct representation that can be serialized and deserialized from projjson. It _does not_ perform conversions between WKT2, WKT1, or Proj4 strings.

### Usage

```rust
use projjson::ProjJson;

let json = r#"{"type":"GeographicCRS","name":"WGS 84","datum":{"type":"GeodeticReferenceFrame","name":"World Geodetic System 1984","ellipsoid":{"name":"WGS 84","semi_major_axis":6378137,"inverse_flattening":298.257223563}},"id":{"authority":"EPSG","code":4326}}"#;

let doc = json.parse::<ProjJson>().unwrap();

assert_eq!(doc.to_string(), json);
```

## Testing

We rely on the tests from PROJ's [`test/unit/test_io.cpp`](https://github.com/OSGeo/PROJ/blob/master/test/unit/test_io.cpp) for correctness.

## Related work

- [PROJJSON specification](https://proj.org/en/latest/specifications/projjson.html): the format this crate implements, maintained by [PROJ](https://proj.org).
- [proj4rs](https://github.com/3liz/proj4rs): pure Rust port of proj4js for coordinate transformations. PROJJSON support there is discussed in [3liz/proj4rs#48](https://github.com/3liz/proj4rs/issues/48).
- [proj4wkt-rs](https://github.com/3liz/proj4wkt-rs): companion to proj4rs that converts WKT1/WKT2 to PROJ strings.
- [epsg-utils](https://github.com/yutannihilation/epsg-utils-rs): EPSG lookup, WKT2 parsing and WKT2 ↔ PROJJSON conversion. Its PROJJSON reader handles projected CRSs only (as of 0.0.3).
- [crs-definitions](https://crates.io/crates/crs-definitions): CRS definitions for EPSG codes.
- [geoscribe](https://crates.io/crates/geoscribe): geospatial metadata reader/writer with an embedded CRS registry.
- [oxiproj](https://crates.io/crates/oxiproj): pure Rust port of PROJ, with an ISO 19111 CRS model and WKT support in [oxiproj-crs](https://crates.io/crates/oxiproj-crs).
- [proj-wkt](https://crates.io/crates/proj-wkt): WKT and PROJ string parser for proj-core CRS definitions.
- [spatialreference.org](https://spatialreference.org): PROJJSON, WKT and PROJ string exports for EPSG and ESRI codes, used for this crate's test fixtures.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
