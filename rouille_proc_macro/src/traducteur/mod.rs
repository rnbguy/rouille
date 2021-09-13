/// If you want to add support for your language, copy `template.rs` in `src/traducteur`
/// and rename it to your language and modify it correctly.
///
/// Then add it to `[features]` list in `Cargo.toml`
/// and define the module here with cfg attribute. Example,
/// ```ignore
/// #[cfg(feature = "italiano")]
/// pub mod italiano
/// ```

#[cfg(feature = "francais")]
pub mod francais;
