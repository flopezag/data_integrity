# 🔐 NGSI-LD JSON-LD Data Integrity

[![Rust](https://img.shields.io/badge/Rust-🦀-orange?style=flat-square)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-Web--Framework-blue?style=flat-square)](https://docs.rs/axum)
[![OpenAPI](https://img.shields.io/badge/OpenAPI-Generated-green?style=flat-square)](https://swagger.io/specification/)
[![License](https://img.shields.io/badge/license-Apache2.0-blue.svg?style=flat-square)](LICENSE)

This project is a Rust-based microservice that signs and verifies ETSI NGSI-LD entity payloads using Ed25519 digital signatures in compliance with ETSI NGSI-LD API `cross-cutting Context Information Management (CIM);
handling of provenance information in NGSI-LD`.

---

## 📦 Features

- `/info` – Service metadata
- `/config` – Define per-entity signing rules
- `/sign` – Apply per-entity signing logic
- `/verify` – Field-level signature validation
- Auto-generated OpenAPI YAML (`doc/openapi.yaml`)
- Fallback for undefined endpoints (`405`, structured error)
- 🚀 Docker-ready

 3577  cargo run -p mock_server
 3578  cargo run -p sign
 3579  cargo run -p signer

 RUST_LOG=info cargo run -p signer

---

## 📘 Project Structure

```
src/
├── main.rs          # App entrypoint
├── handlers/
│   ├── sign.rs      # /sign logic
│   ├── verify.rs    # /verify logic
│   ├── config.rs    # /config logic
│   └── version.rs   # /info logic
├── openapi.rs       # Utoipa-based OpenAPI generator
build.rs             # Auto-generates doc/openapi.yaml
```

---

## 🛠 API Endpoints

### `GET /info`

Returns service version and uptime.

```json
{
  "version": "0.1.0",
  "repository": "https://github.com/flopezag/data_integrity",
  "uptime_seconds": 123
}
```

### `POST /config`

Store signing rules per entity type.

```json
{
  "entity_type": "Store",
  "properties_to_sign": ["address"]
}
```

Empty `properties_to_sign` → sign all object properties.

---

### `POST /sign`

Signs a JSON-LD NGSI-LD entity using the configured rules.

Example request:

```json
{
  "id": "urn:ngsi-ld:Store:002",
  "type": "Store",
  "address": { "type": "Property", "value": { "city": "Rome" } },
  "location": { "type": "GeoProperty", "value": { "type": "Point", "coordinates": [10, 10] } },
  "@context": "https://uri.etsi.org/ngsi-ld/primer/store-context.jsonld"
}

Received JSON payload:
{
  "id": "urn:ngsi-ld:Notification:b99f41be-5023-11f0-b732-e6ff9c082ab3",
  "type": "Notification",
  "notifiedAt": "2025-06-23T11:18:01.677Z",
  "subscriptionId": "urn:ngsi-ld:Subscription:abf76686-5023-11f0-905d-e6ff9c082ab3",
  "data": [
    {
      "id": "urn:ngsi-ld:EntityType:E1",
      "type": "EntityType",
      "A1": {
        "type": "Property",
        "value": "@none"
      },
      "A2": {
        "type": "Property",
        "value": "@none"
      }
    }
  ]
}
---

### `POST /verify`

Verify each signed field in a document.

Response:

```json
{
  "results": {
    "address": "true",
    "location": "na"
  }
}
```

* `"true"`: proof valid
* `"false"`: proof invalid
* `"na"`: no proof found

---

### 🔁 Fallback Handler

```json
{
  "error": "Endpoint not implemented"
}
```

Returned for any unsupported route or method (status: 405).

---

## 📚 OpenAPI + Swagger

### Auto-generate YAML

```bash
cargo build
# -> Generates ./doc/openapi.yaml
```

### Swagger UI (Optional)

Uncomment Swagger lines in `main.rs` to activate:

```
http://localhost:3000/docs
```

---

## 🐳 Docker Usage

### Build image

```bash
docker build -t ngsild-signer .
```

### Run container

```bash
docker run -p 3000:3000 ngsild-signer
```

You can now call:

```
http://localhost:3000/info
http://localhost:3000/sign
```

---

## 🧪 Run Tests

```bash
cargo test
```

Includes tests for:

* Config-based signing logic
* 405 fallback behavior
* Signature injection
* Signature verification

---

---

## 🔮 Roadmap

Take a look to the file [ROADMAP.md](ROADMAP.md)
---

## 📘 License

Apache 2.0 © 2025 — Built to support ETSI NGSI-LD and JSON-LD `DataIntegrityProof`
