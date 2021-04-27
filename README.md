# Ray Tracing in Rust
[![Lint](https://github.com/arthur-targaryen/ray-tracing-rust/actions/workflows/lint.yml/badge.svg?branch=main)](https://github.com/arthur-targaryen/ray-tracing-rust/actions/workflows/lint.yml)
[![Test](https://github.com/arthur-targaryen/ray-tracing-rust/actions/workflows/test.yml/badge.svg)](https://github.com/arthur-targaryen/ray-tracing-rust/actions/workflows/test.yml)

![Example scene render](./image.png)

Ray Tracer written in Rust.
Thanks to Peter Shirley for the book [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
I followed to write this ray tracer.

## Using it
First, [install Rust](https://www.rust-lang.org/tools/install).
Then build and run the project:
```bash
$ cargo build --release
$ ./target/release/ray_tracing > image.ppm
```