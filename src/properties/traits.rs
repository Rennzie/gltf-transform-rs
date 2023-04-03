use super::buffer;

pub trait FromJson<T> {
    fn from_json(json: &T, doc: &json::Root, buffers: &[buffer::Blob]) -> Self;
}

pub trait ToJson<T> {
    fn to_json(&self, index: usize) -> T;
}
