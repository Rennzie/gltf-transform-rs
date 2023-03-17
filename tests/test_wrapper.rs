use gltf::mesh::Bounds;
use gltf_transform_rs as gltf;
use std::io::Read;
use std::{fs, io};

#[test]
fn test_accessor_bounds() {
    // file derived from minimal.gltf with changed min/max values
    let file = fs::File::open("tests/minimal_accessor_min_max.gltf").unwrap();
    let mut reader = io::BufReader::new(file);
    let mut buffer = vec![];
    reader.read_to_end(&mut buffer).unwrap();
    let gltf = gltf::Gltf::from_slice(&buffer).unwrap();
    let mesh = &gltf.meshes().next().unwrap();
    let prim = mesh.primitives().next().unwrap();
    let bounds = prim.bounding_box();
    assert_eq!(
        bounds,
        Bounds {
            min: [-0.03, -0.04, -0.05],
            max: [1.0, 1.01, 0.02]
        }
    );
}
