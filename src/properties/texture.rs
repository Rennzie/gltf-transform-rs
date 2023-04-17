#[derive(Debug, Clone)]
pub struct Texture(json::Texture);
impl Texture {
    pub fn new(json: &json::Texture) -> Self {
        Texture(json.clone())
    }
}
