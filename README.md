WeaveHeap is a zero-copy, ownership-aware shared heap that lets Python,
JavaScript/Node, Rust, Go, and Java exchange real data structures without
marshaling, built on the WebAssembly Component Model (WIT interfaces, memory64,
and emerging GC) with a native backend for non-WASM hosts. It provides a region
allocator and capability handles (generational indices, borrow states) to
guarantee deterministic lifetimes and safe borrows across runtimes, exposing
idiomatic views in each language—NumPy/PEP 3118 and Arrow C-Data/DLPack in
Python, TypedArray views via N-API in Node, Arrow/Vec<T> in Rust, and clean FFI
for Go/Java. A compact ABI defines bytes, string, Vec<T>, Map<K,V>, and
iterators; nested and tabular types piggyback Arrow layouts to stay zero-copy
while remaining bounds-checked and panic-safe. Tooling ships with weaveheap
doctor (detects accidental copies), copy/alloc counters, and editor adapters
that visualize regions and borrows; packages land as pip/npm/cargo/Gradle
artifacts and run both natively and inside WASM sandboxes. Early integrations
target Arrow/Polars, DuckDB extensions, and tensor libs for ML ops. A V1 demo
moves a 1 GB Arrow table Rust ↔ Python ↔ Node with deterministic lifetimes and
no materialization, showing 10–50× less overhead versus JSON/ctypes/Protobuf
bridges on polyglot services and data/AI pipelines. Result: keep one fast
representation of data and move work—not bytes—across languages, erasing the
everyday FFI tax.
