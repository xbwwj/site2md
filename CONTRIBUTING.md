# Contributing

## About snapshot test

For web pages we need snapshot test to ensure the parsers work correctly. 

But HTML (and JS, JSON) files can take much disk spaces.

To alleviate this:

1. [minify-html](https://github.com/wilsonzlin/minify-html)
2. brotli

```sh
minhtml --minify-css --minify-js $html
brotli $minified
```

Do not commit HTML into git repo.

To load it in snapshot:

```rust
let compressed_data = include_bytes!("../min.html.br");
let mut decompressor = Decompressor::new(&compressed_data[..], 4096);

let mut html = String::new();
decompressor.read_to_string(&mut html).unwrap();
```
