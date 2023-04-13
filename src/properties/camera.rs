/// A camera's projection.
#[derive(Clone, Debug)]
pub enum Projection {
    /// Describes an orthographic projection.
    Orthographic(Orthographic),

    /// Describes a perspective projection.
    Perspective(Perspective),
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub name: Option<String>,

    pub projection: Projection,

    pub extras: json::Extras,

    #[cfg(feature = "extensions")]
    pub extensions: properties::Extensions,
}

impl Camera {
    pub fn from_json(json: &json::Camera) -> Self {
        let projection = match json.type_.unwrap() {
            json::camera::Type::Orthographic => {
                Projection::Orthographic(Orthographic::from_json(&json.orthographic))
            }
            json::camera::Type::Perspective => {
                Projection::Perspective(Perspective::from_json(&json.perspective))
            }
        };

        Self {
            name: json.name.clone(),
            projection,
            extras: json.extras.clone(),
            #[cfg(feature = "extensions")]
            extensions: json.extensions.clone(),
        }
    }
}

/// Values for an orthographic camera.
#[derive(Clone, Debug)]
pub struct Orthographic {
    /// The horizontal magnification of the view.
    pub xmag: f32,

    /// The vertical magnification of the view.
    pub ymag: f32,

    /// The distance to the far clipping plane.
    pub zfar: f32,

    /// The distance to the near clipping plane.
    pub znear: f32,

    /// Extension specific data.
    #[cfg(feature = "extensions")]
    pub extensions: properties::Extensions,

    /// Optional application specific data.
    pub extras: json::Extras,
}

impl Orthographic {
    pub fn from_json(json: &Option<json::camera::Orthographic>) -> Self {
        let json = json.as_ref().unwrap();
        Self {
            xmag: json.xmag,
            ymag: json.ymag,
            zfar: json.zfar,
            znear: json.znear,
            extras: json.extras.clone(),
            #[cfg(feature = "extensions")]
            extensions: json.extensions.clone(),
        }
    }
}

/// Values for a perspective camera.
#[derive(Clone, Debug)]
pub struct Perspective {
    /// Aspect ratio of the field of view.
    pub aspect_ratio: Option<f32>,

    /// The vertical field of view in radians.
    pub yfov: f32,

    /// The distance to the far clipping plane.
    pub zfar: Option<f32>,

    /// The distance to the near clipping plane.
    pub znear: f32,

    /// Extension specific data.
    #[cfg(feature = "extensions")]
    pub extensions: properties::Extensions,

    /// Optional application specific data.
    pub extras: json::Extras,
}

impl Perspective {
    pub fn from_json(json: &Option<json::camera::Perspective>) -> Self {
        let json = json.as_ref().unwrap();
        Self {
            aspect_ratio: json.aspect_ratio,
            yfov: json.yfov,
            zfar: json.zfar,
            znear: json.znear,
            extras: json.extras.clone(),
            #[cfg(feature = "extensions")]
            extensions: json.extensions.clone(),
        }
    }
}
