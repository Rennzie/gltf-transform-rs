#[derive(Debug, Clone)]
pub struct Scene(json::Scene);
impl Scene {
    pub fn new(json: &json::Scene) -> Self {
        Scene(json.clone())
    }
}
