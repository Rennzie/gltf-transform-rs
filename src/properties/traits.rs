use crate::buffer::Data;

pub trait FromJson<T> {
    fn from_json(json: &T, doc: &gltf_json::Root, buffers: &[Data]) -> Self;
}

pub trait ToJson<T> {
    fn to_json(&self, index: usize) -> T;
}
