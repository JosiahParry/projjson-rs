# projjson-rs

This repo provides a `serde` compatible representation of `projjson` and schmea validation.

This crate is based on on the [v0.7 schema](https://proj.org/en/latest/schemas/v0.7/projjson.schema.json).

Note that `projjson` only provides a struct representation that can be serialized and deserialized from projjson. It _does not_ perform conversions between WKT2, WKT1, or Proj4 strings.

---

Related work:
