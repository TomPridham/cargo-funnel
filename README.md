a tool similar to [sortpack](https://www.npmjs.com/package/sortpack), but for rust. itis pretty basic, but does what i need it to. if you have feature requests or encounter bugs, please open an issue

just run `cargo funnel` in your rust project and that's it!

### Differences from cargo-sort
[cargo-sort](https://github.com/DevinR528/cargo-sort) only sorts dependencies. `cargo-funnel` will do that, as well as sort things under the `[package]` key according to the order defined [here](https://doc.rust-lang.org/cargo/reference/manifest.html#the-package-section)
