#[cfg(feature = "extensions")]
#[derive(Debug, Clone)]
pub struct Extension {
    /// Extension specific data.
    pub extensions: Option<json::extensions::root::Root>,

    /// Names of glTF extensions used somewhere in this asset.
    pub extensions_used: Vec<String>,

    /// Names of glTF extensions required to properly load this asset.
    pub extensions_required: Vec<String>,
}

#[cfg(feature = "extensions")]
impl Extension {
    pub fn from_json(json: &json::extensions::Root) -> Self {
        todo!()
    }
}
