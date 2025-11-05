# 🔧 Refactoring Plan for `Data Integrity`

## 1. 🧱 Project Structure Refactoring

- ⬜ **Organize responsibilities clearly**:
  - `src/main.rs`: app entry point (config, router).
  - `src/handlers/`: separate modules per endpoint:
    - `config.rs`, `sign.rs`, `verify.rs`, `version.rs`
  - `src/init.rs`: logging and setup logic.
  - `src/lib.rs`: core signing/verification logic.

- ⬜ **Optional: Use workspace layout** (if planning to grow project):
  ```
  data_integrity/
  ├── Cargo.toml         # Workspace manifest
  ├── app/               # Binary crate
  │   └── src/main.rs
  └── core/              # Library crate
      ├── src/lib.rs
      ├── sign.rs
      └── verify.rs
  ```

---

## 2. 🧠 Extract Core Logic

- ✅ The system will receive a notification with an entity data, if the configuration is not created return error 404, if the configuration was created but the list of properties is empty -> sign all the properties, otherwise sign only the property specify in the configuration.
- ⬜ Move cryptographic functions (signing, verifying) into `core` module or crate.
- ⬜ Ensure `handlers` only wrap HTTP logic, delegating to `core::*`.
- ⬜ The generation of public-private key has to be created only once and should be connected to the IdM service
- 🚧 Add tests for core logic.

---

## 3. ⚙️ Config Handling

- ✅ Create a new endpoint to keep configuration details about the subscription rule and properties to sign
- 🚧 Create a `Config` struct to hold:
  - Server port
  - Logging level and path
  - Persistence options (e.g. path to config store)

- ⬜ Use `config` or `serde` + `toml`/`.env` to load config.
- ⬜ Eliminate global `CONFIG_STORE` — inject state using Axum's `Extension`.

---

## 4. 🩵 Logging Improvements

- ⬜  Move `setup_logging()` into `init.rs`.
- 🚧 Configure logging with:
  - File rotation (e.g. via `flexi_logger`)
  - Output formatting (JSON vs plain)
  - Level (`info`, `debug`, etc.)
  - Console duplication

- ⬜  Source config from config file.

---

## 5. 🧪 Testing

- **Unit tests** for:
  - ⬜ `core::sign_payload`, `core::verify_proof`
  - ⬜ Handlers (`config`, `sign`, `verify`) — using Axum test utilities
- **Integration test**:
  - ⬜ `POST /config`, `POST /sign`, then `POST /verify` end-to-end
- ⬜ **Coverage**: Consider tools like `tarpaulin`
- ✅ Create a Mock Server to test the functionalities of the Orion-LD
- 🚧 Generate process http file with the different requests and response to test the content


---

## 6. 🚨 Error Handling

- ⬜ Create an `AppError` type using `thiserror` or `anyhow`.
- ⬜ Implement `IntoResponse` for clean JSON error replies.
- Use appropriate status codes:
  - ⬜ `400 Bad Request` for input issues
  - ⬜ `412 Precondition Required` for missing config
  - ⬜ Avoid generic `405` fallback

---

## 7. 📄 OpenAPI Documentation

- Use `utoipa` to annotate:
  - ⬜ Request types (e.g. `SignRequest`)
  - ⬜ Response types (e.g. `SignResponse`)
  - ⬜ Enums, errors, etc.

- 🚧 Add a route: `GET /openapi.json`
- ⬜ Improve the information of the API description of the different endpoints
- ✅ Integrate with Swagger UI
- ✅ Generate openapi.yaml file under CLI command '--dump-openapi'

---

## 8. 🛡 Fallback & Health

- 🚧 Change `GET /info` by `GET /health` returning:
  ```json
  {
    "status": "ok",
    "version": "1.0.0",
    "repository": "<github repository>",
    "uptime": "3s"
  }
  ```

- ⬜ Update fallback to return consistent JSON:
  ```json
  { "error": "Endpoint not implemented" }
  ```

---

## 9. ⚙️ CI & Quality Assurance

- ⬜ Add GitHub Actions workflow:
  ```yaml
  steps:
    - run: cargo fmt -- --check
    - run: cargo clippy -- -D warnings
    - run: cargo test
  ```

- Optional:
  - ⬜ Add `cargo tarpaulin` for test coverage
  - ⬜ Fuzz `core` logic with `cargo fuzz`

---

## 10. 🔜 Deployment (Docker & Kubernetes)

- Docker:

  - 🚧 Add a Dockerfile with multi-stage build

  - ⬜ Optimize for minimal final image (e.g. scratch or distroless)

- Kubernetes:

  - ⬜ Provide Helm chart or Kubernetes manifests

  - ⬜ Configure readiness/liveness probes

  - ⬜ Enable configuration via environment variables or mounted volumes

---

## 11. 📖 Developer & User Documentation

- Add Markdown documentation under docs/

  - ⬜ Setup instructions

  - ⬜ Configuration file explanation

  - ⬜ API usage examples (with curl)

- Improve README:

  - ⬜ Add badges (CI, coverage)

  - 🚧 Quick start, features, roadmap

---

## ✅ Refactoring Sequence

| Step | Task                                  |
|------|---------------------------------------|
|  1   | Extract core logic into `core` module |
|  2   | Refactor handlers to use core logic   |
|  3   | Move setup/logging into `init.rs`     |
|  4   | Add config file support               |
|  5   | Write unit + integration tests        |
|  6   | Implement structured error handling   |
|  7   | Enhance OpenAPI + fallback            |
|  8   | Add health check endpoint             |
|  9   | Configure CI                          |
| 10   | Deployment (Docker & Kubernetes)      |
| 11   | Developer & User Documentation        |

---

### Possible status

✅ COMPLETED

🚧 IN PROGRESS

⬜ NOT STARTED

❌ CANCELED

