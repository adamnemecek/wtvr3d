use super::super::math::{Matrix4,Vector3};
use std::option::Option;

/// # Transform
/// A Transform is a node in the Scene tree. It allows moving into the tree in any direction.
pub struct Transform {
    translation : Vector3,
    rotation : Vector3,
    scale : Vector3,
    dirty : bool,
    dead : bool,
    matrix : Matrix4,
    pub parent : Option<TransformId>,
    pub first_child : Option<TransformId>,
    pub last_child : Option<TransformId>,
    pub next_sibling : Option<TransformId>,
    pub previous_sibling : Option<TransformId>
}

/// # TransformId
/// A type checked id to reference the transforms in an idiomatic way.
#[derive(Hash,Copy, Clone, PartialEq, Eq)]
pub struct TransformId {
    pub index : usize
}

impl Transform {
    /// Creates a new transform from a relative  translation, rotation and scale. Its matrix won't match by default.
    pub fn new(t: Vector3, r : Vector3, s : Vector3) -> Transform {
         Transform {
            translation : t,
            rotation : r,
            scale : s,
            dirty : true,
            dead: false,
            matrix : Matrix4::identity(),
            parent : None,
            first_child : None,
            last_child : None,
            next_sibling : None,
            previous_sibling : None
        }
    }

    pub fn get_position(&self) -> &Vector3 {
        &self.translation
    }

    pub fn get_rotation(&self) -> &Vector3 {
        &self.rotation
    }

    pub fn get_scale(&self) -> &Vector3 {
        &self.scale
    }

    pub fn get_position_mut(&mut self) -> &mut Vector3 {
        &mut self.translation
    }

    pub fn get_rotation_mut(&mut self) -> &mut Vector3 {
        &mut self.rotation
    }

    pub fn get_scale_mut(&mut self) -> &mut Vector3 {
        &mut self.scale
    }

}