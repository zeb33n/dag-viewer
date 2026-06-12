// rgba
pub type Colour = u32;

pub struct VecF2 {
    pub x: f32,
    pub y: f32,
}

pub struct Circle {
    pub center: VecF2,
    pub radius: f32,
}

pub type LogicalNodeHandle = usize;
