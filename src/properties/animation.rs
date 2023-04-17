#[derive(Clone, Debug)]
pub struct Animation(json::Animation);
impl Animation {
    pub fn new(json: &json::Animation) -> Self {
        Animation(json.clone())
    }
}
