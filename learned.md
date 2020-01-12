# imports and file structure

- main.rs should import functionality as if the crate was download from crates.io. `use torrentino::Torrent;`
- lib.rs is the crate ROOT. imports inside modules like `use crate::somemodule` will reference to lib. if `somemodule` is not declared in lib as `mod somemodule` this will fail. you can optionaly prefix with `pub` to make it public for consumers
- `pub(in module)`, `pub(crate)`, etc. will make it visible inside the crate but not outside.
- module name is declared in `cargo.toml` that is the name you use to import in, for example, main.rs

## mistakes
    - using `mod hashing` in main.rs created some problems: rust compiles 2 targets separately, main and lib. that made the `hashing` module appear in 2 targets. and imports inside the `metadata` module was not referencing correctly, as `hashing` did not exist on the built `main`