#[derive(Debug, Clone)]
pub struct Skin(json::Skin);
impl Skin {
    pub fn new(json: &json::Skin) -> Self {
        Skin(json.clone())
    }
}
