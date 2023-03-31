use super::element::Element;
use crate::buffer::Data;
use crate::properties::traits::{FromJson, ToJson};
use bytemuck::Pod;
use gltf_json::accessor::{ComponentType, Type as ElementType};
use gltf_json::{accessor, Accessor as JsonAccessor, Index, Value};
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericAccessor<CT, ET>
where
    CT: Pod,
    ET: Element<CT> + Clone,
{
    phantom: PhantomData<CT>,
    /// The typed array containing the accessor's data.
    array: Option<Vec<ET>>,

    /// The data type of an elements components in the attribute.
    /// Supports all GLSL data types.
    pub component_type: ComponentType,

    /// Specifies if the attribute is a scalar, vector, or matrix.
    pub element_type: ElementType,

    /// Specifies whether integer data values should be normalized.
    pub normalized: bool,

    /// Sparse storage of attributes that deviate from their initialization
    /// value.
    /// TODO: handle sparse accessors
    pub sparse: bool,

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub name: Option<String>,

    /// Optional application specific data.
    #[cfg(feature = "extras")]
    pub extras_extensions: ExtrasExtension,
}

impl<CT, ET> FromJson<JsonAccessor> for GenericAccessor<CT, ET>
where
    CT: Pod + std::fmt::Debug,
    ET: Element<CT> + Clone,
{
    fn from_json(accessor_json: &JsonAccessor, json: &gltf_json::Root, buffers: &[Data]) -> Self {
        let element_type = accessor_json.type_.unwrap();
        let component_type = accessor_json.component_type.unwrap().0;

        let array = if let Some(view_index) = accessor_json.buffer_view {
            let buffer_view_json = json
                .buffer_views
                .get(view_index.value())
                .expect("Missing buffer view for index={view_index}");

            let buffer = buffers.get(buffer_view_json.buffer.value());

            // Tightly packed array of bytes for this accessor
            if let Some(buffer) = buffer {
                let bytes = get_compact_bytes::<CT, ET>(
                    buffer,
                    buffer_view_json,
                    accessor_json,
                    element_type.multiplicity(),
                    component_type.size(),
                );
                let typed_array = bytemuck::cast_slice::<u8, CT>(&bytes).to_vec();
                Some(ET::to_element_vec(typed_array))
            } else {
                // TODO: should throw if no buffer is found but we have a buffer view
                None
            }
        } else {
            // TODO: Handle sparse accessors
            if accessor_json.sparse.is_some() {
                todo!()
            }
            todo!()
        };

        GenericAccessor {
            phantom: PhantomData,
            array,
            component_type,
            element_type,
            normalized: accessor_json.normalized,
            sparse: accessor_json.sparse.is_some(),
            #[cfg(feature = "names")]
            name: accessor_json.name.clone(),
            #[cfg(feature = "extras")]
            extras_extensions: ExtrasExtension::from_json(
                &accessor_json.extras,
                &accessor_json.extensions,
            ),
        }
    }
}

/// Returns a tightly packed vector of bytes for the given accessor
fn get_compact_bytes<CT, ET>(
    buffer: &Data,
    buffer_view_json: &gltf_json::buffer::View,
    accessor_json: &JsonAccessor,
    components_per_element: usize,
    bytes_per_component: usize,
) -> Vec<u8>
where
    CT: Pod + std::fmt::Debug,
    ET: Element<CT> + Clone,
{
    let buffer_view_byte_length = buffer_view_json.byte_length as usize;
    let buffer_view_byte_offset = buffer_view_json.byte_offset.unwrap_or(0);
    let byte_offset = (accessor_json.byte_offset + buffer_view_byte_offset) as usize;
    let bytes_per_element = bytes_per_component * components_per_element;
    let array_capacity = components_per_element * accessor_json.count as usize;

    let mut bytes = Vec::with_capacity(array_capacity);

    match buffer_view_json.byte_stride {
        // We deconstruct interleaved buffers to make them easier to work with and mutate
        Some(stride) => {
            for i in 0..(buffer_view_byte_length - byte_offset) {
                let offset = byte_offset + (i * stride as usize);
                bytes.extend_from_slice(&buffer[offset..offset + bytes_per_element]);
            }
        }
        None => {
            bytes.extend_from_slice(&buffer[byte_offset..(byte_offset + buffer_view_byte_length)]);
        }
    }

    bytes
}

impl<CT, ET> ToJson<(JsonAccessor, gltf_json::buffer::View)> for GenericAccessor<CT, ET>
where
    CT: Pod + std::fmt::Debug,
    ET: Element<CT> + Clone,
{
    fn to_json(&self, index: usize) -> (JsonAccessor, gltf_json::buffer::View) {
        let has_buffer = self.array.is_some();
        (
            JsonAccessor {
                buffer_view: if has_buffer {
                    Some(Index::new(index as u32))
                } else {
                    None
                },
                byte_offset: 0,
                component_type: gltf_json::validation::Checked::Valid(
                    accessor::GenericComponentType(self.component_type),
                ),
                count: self.get_count() as u32,
                #[cfg(feature = "names")]
                name: self.name.clone(),
                normalized: self.normalized,
                sparse: None,
                type_: gltf_json::validation::Checked::Valid(self.element_type),
                extras: None,     // TODO: handle extras
                extensions: None, // TODO: handle extensions
                min: self.get_min_to_json(),
                max: self.get_max_to_json(),
            },
            gltf_json::buffer::View {
                buffer: Index::new(0),
                byte_offset: Some(0),
                byte_length: self.get_count() as u32 * self.component_type.size() as u32,
                byte_stride: None,
                target: None,
                name: None,
                extras: None,
                extensions: None,
            },
        )
    }
}

impl<CT, ET> GenericAccessor<CT, ET>
where
    CT: Pod + std::fmt::Debug,
    ET: Element<CT> + Clone,
{
    pub fn get_min(&self) -> Option<ET> {
        todo!()
    }

    pub fn get_max(&self) -> Option<ET> {
        todo!()
    }

    fn get_min_to_json(&self) -> Option<Value> {
        None
    }

    fn get_max_to_json(&self) -> Option<Value> {
        None
    }

    /// The number of elements within the Accessor array - not to be confused
    /// with the number of components or the number of bytes.
    /// For example a Vec3 accessor with 12 components will have a count of 4.
    pub fn get_count(&self) -> usize {
        if let Some(array) = &self.array {
            array.len()
        } else {
            0
        }
    }

    /// The type of each component in the attribute.
    pub fn get_type(&self) -> ElementType {
        self.element_type
    }

    /// The returns the number of components within an element
    /// i.e a Vec2 will have a component size of 2.
    pub fn get_element_size(&self) -> usize {
        self.element_type.multiplicity()
    }

    /// The number of bytes in a single component (value in the raw array)
    /// i.e an f32 & u32 will have a component size of 4 bytes.
    pub fn get_component_size(&self) -> usize {
        self.component_type.size()
    }

    /// Return the component type of the accessor
    pub fn get_component_type(&self) -> ComponentType {
        self.component_type
    }

    /// The number of bytes in the accessor
    pub fn get_byte_length(&self) -> usize {
        if let Some(array) = &self.array {
            array.len() * self.component_type.size()
        } else {
            0
        }
    }

    pub fn get_element(&self, index: usize) -> Option<&ET> {
        // Return the element at the given index in the array, if there is one.
        if let Some(array) = &self.array {
            Some(&array[index])
        } else {
            None
        }
    }

    pub fn get_array(&self) -> Option<&Vec<ET>> {
        if let Some(array) = &self.array {
            Some(array)
        } else {
            None
        }
    }

    pub fn set_array(&mut self, array: Vec<ET>) {
        self.array = Some(array);
    }
}
