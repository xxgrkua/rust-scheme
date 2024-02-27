# rust-scheme

Scheme(R<sup>5</sup>RS) in Rust with turtle graphics

ref: [Scheme R<sup>5</sup>RS](http://www.schemers.org/Documents/Standards/R5RS/r5rs.pdf)

Difference from R<sup>5</sup>RS:

- vectors are self-evaluating (like R<sup>7</sup>RS)
- ...

## Building

The source code can be compiled into an ordinary console program or WebAssembly.

### Console

Run `cargo build --release`

The generated executable file is under `./target/release/`

### WebAssembly

You need to get `wasm-pack` installed.

Run `wasm-pack build`

The generated WebAssembly and so on are under `./pkg/`.
