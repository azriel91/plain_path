# 〽️ Plain Path

Expands `~` in a path if present.

```rust
use std::path::Path;
use plain_path::PlainPathExt;

let path = Path::new("~/.ssh/config").plain()?;

// 🍏: "/Users/<user>/.ssh/config"
// 🐧: "/home/<user>/.ssh/config"
println!("{}", path.display());
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
