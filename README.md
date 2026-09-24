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

- [PROJJSON specification](https://proj.org/en/latest/specifications/projjson.html)
- [proj4rs](https://github.com/3liz/proj4rs)
- [proj4wkt-rs](https://github.com/3liz/proj4wkt-rs)
- [epsg-utils](https://github.com/yutannihilation/epsg-utils-rs)
- [crs-definitions](https://crates.io/crates/crs-definitions)
- [geoscribe](https://crates.io/crates/geoscribe)
- [oxiproj](https://crates.io/crates/oxiproj)
- [proj-wkt](https://crates.io/crates/proj-wkt)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
