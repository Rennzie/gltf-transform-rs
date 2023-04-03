pub struct ExtrasExtension {
    /// Optional application specific data.
    pub extras: json::Extras,

    /// Extension specific data.
    pub extensions: Option<json::extensions::root::Root>,

    /// Names of glTF extensions used somewhere in this asset.
    pub extensions_used: Vec<String>,

    /// Names of glTF extensions required to properly load this asset.
    pub extensions_required: Vec<String>,
}
