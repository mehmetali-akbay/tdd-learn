# Serde — Serialization & Deserialization

serde, serde_json, toml, derive macros, custom serializers, enums

**6 topics | Progressive difficulty**

```bash
cargo test -p serde_learn
```

## Topics

1. **JSON Basics** — `Serialize`/`Deserialize` derive, `serde_json`
2. **Custom Field Names & Defaults** — `#[serde(rename)]`, `#[serde(default)]`, skip
3. **Enums in Serde** — Tagged, untagged, adjacently tagged enums
4. **Nested & Optional Fields** — `Option<T>`, `Vec<T>`, flatten
5. **Custom Serializers** — `serialize_with`, `deserialize_with`
6. **Advanced — Multiple Formats** — TOML via serde; format-agnostic code
