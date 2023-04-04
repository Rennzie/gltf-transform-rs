#[derive(Debug, Clone)]
pub struct Material(json::Material);
impl Material {
    pub fn new(json: &json::Material) -> Self {
        Material(json.clone())
    }
}
