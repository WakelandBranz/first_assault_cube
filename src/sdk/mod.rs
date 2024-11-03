pub mod player;
mod weapon;
pub(crate) mod offsets;
pub mod entity_list;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

