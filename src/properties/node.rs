#[derive(Debug, Clone)]
pub struct Node(json::Node);
impl Node {
    pub fn new(json: &json::Node) -> Self {
        Node(json.clone())
    }
}
