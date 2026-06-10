// rgba
pub type Colour = u32;

pub struct VecF2 {
    pub x: f32,
    pub y: f32,
}

pub struct VecI2 {
    pub x: i32,
    pub y: i32,
}

impl VecI2 {
    pub fn from_vecf2(vec: VecF2) -> Self {
        Self {
            x: vec.x as i32,
            y: vec.y as i32,
        }
    }
}

pub type LogicalNodeHandle = usize;
