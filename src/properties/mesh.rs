#[derive(Debug, Clone)]
pub struct Mesh(json::Mesh);
impl Mesh {
    pub fn new(json: &json::Mesh) -> Self {
        Mesh(json.clone())
    }
}
