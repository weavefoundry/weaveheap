## WeaveHeap Roadmap (Rust core)

This roadmap tracks high-level goals for the first five releases, with a detailed
step-by-step plan to reach 0.1.0. The project uses a Rust core that exposes a
stable C ABI and initial WIT/Component Model surfaces for WASM, with idiomatic
zero-copy bindings for Python (PEP 3118/Arrow/DLPack), Node.js (TypedArray via
N-API), Swift (SwiftPM System Library), Go (cgo), and Java (JNI). The core
centers on a region allocator and capability handles (generational indices,
borrow states) to guarantee deterministic lifetimes and safe borrows across
runtimes.

### Release goals

- **0.1.0 — MVP foundation**: Rust workspace, region allocator, capability
  handles, basic types (bytes, string, buffers), error and memory model,
  stable C ABI, minimal Node.js and Python views, minimal Web/WASM target,
  CLI with doctor and copy/alloc counters, samples, and docs.
- **0.2.0 — Views and bindings**: PEP 3118 buffers and Arrow C-Data/DLPack for
  Python, robust TypedArray views for Node.js (N-API), Swift wrapper (SwiftPM
  System Library), initial Go/Java adapters, packaging improvements (pip wheels,
  npm prebuilds, SwiftPM/Gradle templates), better cross-compilation UX.
- **0.3.0 — Structured/tabular types**: Map/Vec/iterator APIs, Arrow tables and
  nested types (piggyback Arrow layouts), streaming/iterators, improved
  concurrency (thread-safe regions), memory64/WASM integration polish, tracing
  and diagnostics.
- **0.4.0 — Safety and performance**: Zero-copy where safe, arena/pool patterns,
  lifetime-safe handle APIs, borrow state invariants, fuzzing/sanitizers,
  fragmentation/compaction strategies, incremental performance work, and DX
  polish across all targets.
- **0.5.0 — Ecosystem expansion**: Arrow/Polars and DuckDB extension prototypes,
  tensor interop via DLPack, distribution story (prebuilt artifacts), stability
  hardening, and release automation.

---

### 0.1.0 detailed plan (MVP)

Focus: Deliver a usable MVP that provides a region allocator and capability
handles with a stable C ABI, plus minimal language views for Node.js and Python,
and a small WASM target. Ship a CLI that includes `doctor` and copy/alloc
counters, along with documentation and samples.

#### 1) Workspace scaffolding
- Create a Rust workspace with crates:
  - `weaveheap-core`: region allocator, capability handles (generational indices,
    borrow states), copy/alloc counters, error types.
  - `weaveheap-capi`: stable C ABI surface and small convenience shims.
  - `weaveheap-node`: minimal N-API wrapper exposing TypedArray views.
  - `weaveheap-python`: minimal Python wrapper exposing PEP 3118-compatible
    buffer/memoryview for bytes/strings; packaging skeleton.
  - `weaveheap-wasm`: minimal `wasm32-unknown-unknown` build and initial WIT
    definitions for basic buffers/strings; thin JS glue stub.
  - `weaveheap-cli`: end-user CLI (`weaveheap`) providing doctor and counters.
  - `samples/`: tiny examples to exercise region/borrow APIs from each language.

#### 2) ABI, memory, and error model
- Establish a stable C ABI surface convention (prefix, naming, versioning):
  - `weaveheap_<module>_<function>(..., weaveheap_error* out_err)` style
  - All strings returned are UTF-8, owned by the Rust core; provide
    `weaveheap_free_string(const char*)` and `weaveheap_free_bytes(uint8_t*, size_t)`
  - Opaque `weaveheap_handle_t` represented as `uint64_t` (or `uintptr_t`) with
    generation to prevent ABA/use-after-free
- Error model:
  - A compact `weaveheap_error` struct with `{ code: int32_t, const char* message }`
  - Map Rust `Result<T, E>` to C: fill `out_err->code != 0` on error
  - Provide `weaveheap_error_clear(weaveheap_error*)` to release message buffers
- Memory model:
  - Region-based allocation and deterministic cleanup; explicit free APIs
  - Borrow states (shared/unique) validated at the C boundary to preserve safety
  - WASM: memory64-compatible layouts and a clear host/guest ownership contract

#### 3) Region allocator and capability handles
- Implement region creation/destruction, buffer allocation within a region, and
  safe release semantics; ensure thread-safety where applicable.
- Define capability handles with generational indices and borrow state tracking;
  prevent stale handle reuse; add cheap runtime checks and counters.
- Provide minimal RAII wrappers internally; keep the C ABI explicit and simple.

#### 4) Basic types and views
- Types: `bytes`, `string (UTF-8)`, and sized buffers; document alignment and
  ownership rules.
- Node.js: TypedArray view descriptors (pointer, length, offset) via N-API; avoid
  copies for read-only/shared borrows.
- Python: memoryview/PEP 3118 exposure for bytes/buffers; basic Arrow C-Data
  pointers gated behind a feature flag for experiments.
- WASM: export simple buffer/string functions and a tiny JS glue to validate
  roundtrips.

#### 5) Language bindings (thin prototypes)
- C header: emit a single `.h` with function prototypes, error types, and free
  utilities; optionally a small `.c` for convenience.
- Node wrapper: small `node-addon-api`/N-API module to expose region and buffer
  views; include a sample script demonstrating zero-copy read.
- Python wrapper: CPython C-API or cffi-based thin layer exporting memoryview; a
  sample to show reading/writing without copy when rules allow.
- Swift (minimal): SwiftPM System Library referencing the C header; one demo call.
- WASM (minimal): document how to build for `wasm32-unknown-unknown` with a stub.

#### 6) CLI
- `weaveheap doctor`: check for toolchains (Rust, Node toolchain, Python toolchain,
  Xcode for Swift, Android NDK optional), reporting actionable guidance.
- `weaveheap stats`: read and print copy/alloc counters per region/handle.
- `weaveheap viz` (text-only for 0.1): dump regions and borrow states to aid
  mental model and debugging.

#### 7) Samples
- Buffer sample: create a region, allocate a buffer, share into Node and Python,
  verify zero-copy semantics.
- String sample: echo/transform without materialization.
- (Optional) Tiny Arrow sample: one column of primitive values via Arrow C-Data
  using experimental flag.

#### 8) Tooling and CI
- GitHub Actions: build `weaveheap-core` and `weaveheap-cli` for macOS and Linux;
  run unit tests.
- Integration test: Node and Python samples load the shared library and perform a
  smoke test (allocate region, view buffer, release).
- WASM build check: ensure `wasm32-unknown-unknown` compiles and the JS stub runs
  a minimal roundtrip locally.

#### 9) Documentation
- `README` quickstart and link to this roadmap.
- Docs pages for: memory & error model, regions/borrows, per-language setup/run
  steps, and the CLI tools.
- End-to-end tutorial using the buffer sample across Rust → Node → Python.

#### 10) Release and versioning
- Tag `v0.1.0`; attach CLI binaries for macOS (arm64/x86_64) and Linux (x86_64).
- Publish `weaveheap-core` and `weaveheap-cli` to crates.io.
- Publish pre-release npm package for Node wrapper and a test PyPI wheel for
  Python (macOS/Linux); SwiftPM System Library template as a repo or archive.

#### 11) Acceptance checklist
- Region allocator and handle lifetimes validated by unit tests.
- Copy/alloc counters observable via CLI and bindings.
- C header compiles; Node wrapper and Python wrapper can view buffers zero-copy
  under documented borrow rules.
- WASM target builds and performs a basic string/buffer roundtrip.
- Docs clearly explain memory ownership, borrows, and error handling.
