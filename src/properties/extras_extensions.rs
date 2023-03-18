pub struct ExtrasExtension {
    /// Optional application specific data.
    #[cfg(feature = "extras")]
    pub extras: gltf_json::Extras,

    /// Extension specific data.
    #[cfg(feature = "extensions")]
    pub extensions: Option<gltf_json::extensions::root::Root>,

    /// Names of glTF extensions used somewhere in this asset.
    #[cfg(feature = "extensions")]
    pub extensions_used: Vec<String>,

    /// Names of glTF extensions required to properly load this asset.
    #[cfg(feature = "extensions")]
    pub extensions_required: Vec<String>,
}
