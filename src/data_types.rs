use std::ops::{self, AddAssign, SubAssign};
// rgba
pub type Colour = u32;

pub trait ColourExt {
    fn set_transparency(&mut self, transparency: u8);
}

impl ColourExt for Colour {
    fn set_transparency(&mut self, transparency: u8) {
        let mut bytes = self.to_be_bytes();
        bytes[3] = transparency;
        *self = u32::from_be_bytes(bytes);
    }
}

#[derive(Clone)]
pub struct Line {
    pub a: VecF2,
    pub b: VecF2,
    pub colour: Colour,
}

impl Line {
    pub fn new(a: VecF2, b: VecF2) -> Self {
        Self {
            a,
            b,
            colour: 0x00000055,
        }
    }
}

#[derive(Clone)]

pub struct Path {
    pub from: NodeHandle,
    pub to: NodeHandle,
    pub line_segments: Vec<Line>,
}

impl Path {
    pub fn new(to: NodeHandle, from: NodeHandle) -> Self {
        Self {
            from: from,
            to: to,
            line_segments: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub is_fake_node: bool,
    pub position: VecF2,
    pub radius: f32,
    pub colour: Colour,
    pub label: String,
    pub label_size: f32,
    pub label_colour: Colour,
    pub edges: Vec<usize>,
    pub dependents: Vec<usize>,
    pub bicone: bool,
}

impl Node {
    pub fn new(label: &str) -> Self {
        Self {
            is_fake_node: false,
            position: VecF2 { x: 0.0, y: 0.0 },
            radius: 30.0,
            colour: 0xFF000055,
            label_colour: 0x00000055,
            label_size: 10.0,
            edges: vec![],
            label: label.to_string(),
            dependents: vec![],
            bicone: false,
        }
    }

    pub fn new_fake_node() -> Self {
        Self {
            is_fake_node: true,
            position: VecF2 { x: 0.0, y: 0.0 },
            radius: 30.0,
            colour: 0x00000000,
            label_colour: 0x00000000,
            label_size: 0.0,
            edges: vec![],
            label: "".to_string(),
            dependents: vec![],
            bicone: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct VecF2 {
    pub x: f32,
    pub y: f32,
}

impl ops::Add<VecF2> for VecF2 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for VecF2 {
    fn add_assign(&mut self, rhs: VecF2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<VecF2> for VecF2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for VecF2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul<f32> for VecF2 {
    type Output = Self;
    fn mul(self, other: f32) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl ops::MulAssign<f32> for VecF2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

pub struct Circle {
    pub center: VecF2,
    pub radius: f32,
}

pub type NodeHandle = usize;
