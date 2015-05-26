# Rust wrapper around [libucl][libucl]

## Usage

```rust
extern crate ucl;
use ucl::Parser;

let parser = Parser::new();
let result = parser.parse(r#"name = "mort";
section {
    nice = true;
    server = "http://localhost:6666";
    chunk = 1Gb;
}"#).unwrap();

println!("{}", result.fetch_path("section.nice").and_then(|v| v.as_bool()));
```

## Licence

Check out [LICENSE](LICENSE) file.

[libucl]: https://github.com/vstakhov/libucl "Universal configuration library parser"
