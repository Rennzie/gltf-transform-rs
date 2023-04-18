#[cfg(feature = "extensions")]
#[derive(Debug, Clone)]
pub struct Extension<T> {
    /// Extension specific data.
    pub extensions: T,

    /// Names of glTF extensions used somewhere in this asset.
    pub extensions_used: Option<Vec<String>>,

    /// Names of glTF extensions required to properly load this asset.
    pub extensions_required: Option<Vec<String>>,
}

#[cfg(feature = "extensions")]
impl<T> Extension<T> {
    pub fn from_json(json: T, used: Option<Vec<String>>, required: Option<Vec<String>>) -> Self {
        Self {
            extensions: json,
            extensions_used: used,
            extensions_required: required,
        }
    }
}
