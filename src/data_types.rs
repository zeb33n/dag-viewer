use std::ops::{self, AddAssign, SubAssign};
// rgba
pub type Colour = u32;

pub trait Connector {
    fn set_colour(&mut self, colour: Colour);
    fn draw(&self);
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

pub struct Curve {
    pub a: VecF2,
    pub b: VecF2,
    pub c: VecF2,
    pub d: VecF2,
    pub colour: Colour,
}

impl Curve {
    pub fn new(a: VecF2, b: VecF2, c: VecF2, d: VecF2) -> Self {
        Self {
            a,
            b,
            c,
            d,
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
    pub colour: Colour,
    pub edges: Vec<usize>,
    pub label: String,
    pub dependents: Vec<usize>,
}

impl Node {
    pub fn new(label: &str) -> Self {
        Self {
            is_fake_node: false,
            position: VecF2 { x: 0.0, y: 0.0 },
            colour: 0xFF000055,
            edges: vec![],
            label: label.to_string(),
            dependents: vec![],
        }
    }

    pub fn new_fake_node() -> Self {
        Self {
            is_fake_node: true,
            position: VecF2 { x: 0.0, y: 0.0 },
            colour: 0x00000000,
            edges: vec![],
            label: "".to_string(),
            dependents: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct VecF2 {
    pub x: f32,
    pub y: f32,
}

pub struct VecI2 {
    pub x: i32,
    pub y: i32,
}

impl VecF2 {
    pub fn from_veci2(vec: VecI2) -> Self {
        Self {
            x: vec.x as f32,
            y: vec.y as f32,
        }
    }

    pub fn as_veci2(&self) -> VecI2 {
        VecI2 {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
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

impl VecF2 {
    pub fn dot(&self, rhs: &VecF2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl VecI2 {
    pub fn from_vecf2(vec: VecF2) -> Self {
        Self {
            x: vec.x as i32,
            y: vec.y as i32,
        }
    }

    pub fn as_vecf2(&self) -> VecF2 {
        VecF2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

pub struct Circle {
    pub center: VecF2,
    pub radius: f32,
}

pub type NodeHandle = usize;
