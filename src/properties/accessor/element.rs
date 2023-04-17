use std::convert::TryInto;

// ---- Generic Element Types -----------
pub type Scalar<T> = [T; 1];
pub type Vec2<T> = [T; 2];
pub type Vec3<T> = [T; 3];
pub type Vec4Mat2<T> = [T; 4];
pub type Mat3<T> = [T; 9];
pub type Mat4<T> = [T; 16];

pub trait Element<CT> {
    fn to_element_vec(typed_array: Vec<CT>) -> Vec<Self>
    where
        Self: Sized;
}

// -------- U16 -------------------------------------------------

impl Element<u16> for Scalar<u16> {
    fn to_element_vec(typed_array: Vec<u16>) -> Vec<Scalar<u16>>
    where
        Self: Sized,
    {
        typed_array
            .chunks_exact(1)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<Scalar<u16>>>()
    }
}

impl Element<u16> for Vec2<u16> {
    fn to_element_vec(typed_array: Vec<u16>) -> Vec<Vec2<u16>>
    where
        Self: Sized,
    {
        typed_array
            .chunks_exact(2)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<Vec2<u16>>>()
    }
}

impl Element<u16> for Vec3<u16> {
    fn to_element_vec(typed_array: Vec<u16>) -> Vec<Vec3<u16>>
    where
        Self: Sized,
    {
        typed_array
            .chunks_exact(3)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<Vec3<u16>>>()
    }
}

impl Element<u16> for Vec4Mat2<u16> {
    fn to_element_vec(typed_array: Vec<u16>) -> Vec<Vec4Mat2<u16>>
    where
        Self: Sized,
    {
        typed_array
            .chunks_exact(3)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<Vec4Mat2<u16>>>()
    }
}

impl Element<u16> for Mat3<u16> {
    fn to_element_vec(typed_array: Vec<u16>) -> Vec<Mat3<u16>>
    where
        Self: Sized,
    {
        typed_array
            .chunks_exact(3)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<Mat3<u16>>>()
    }
}

impl Element<u16> for Mat4<u16> {
    fn to_element_vec(typed_array: Vec<u16>) -> Vec<Mat4<u16>>
    where
        Self: Sized,
    {
        typed_array
            .chunks_exact(3)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<Mat4<u16>>>()
    }
}

// --------- F32 -------------------------------------------------

impl Element<f32> for Scalar<f32> {
    fn to_element_vec(typed_array: Vec<f32>) -> Vec<Scalar<f32>>
    where
        Self: Sized,
    {
        typed_array
            .chunks_exact(1)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<Scalar<f32>>>()
    }
}

impl Element<f32> for Vec2<f32> {
    fn to_element_vec(typed_array: Vec<f32>) -> Vec<Vec2<f32>>
    where
        Self: Sized,
    {
        typed_array
            .chunks_exact(2)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<Vec2<f32>>>()
    }
}

impl Element<f32> for Vec3<f32> {
    fn to_element_vec(typed_array: Vec<f32>) -> Vec<Vec3<f32>>
    where
        Self: Sized,
    {
        typed_array
            .chunks_exact(3)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<Vec3<f32>>>()
    }
}

impl Element<f32> for Vec4Mat2<f32> {
    fn to_element_vec(typed_array: Vec<f32>) -> Vec<Vec4Mat2<f32>>
    where
        Self: Sized,
    {
        typed_array
            .chunks_exact(3)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<Vec4Mat2<f32>>>()
    }
}

impl Element<f32> for Mat3<f32> {
    fn to_element_vec(typed_array: Vec<f32>) -> Vec<Mat3<f32>>
    where
        Self: Sized,
    {
        typed_array
            .chunks_exact(3)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<Mat3<f32>>>()
    }
}

impl Element<f32> for Mat4<f32> {
    fn to_element_vec(typed_array: Vec<f32>) -> Vec<Mat4<f32>>
    where
        Self: Sized,
    {
        typed_array
            .chunks_exact(3)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<Mat4<f32>>>()
    }
}
