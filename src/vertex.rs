// Vértice: por ahora solo tiene posición en pantalla (en píxeles) y color.

use crate::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    pub position: Vec2,
    pub color: Color,
}

impl Vertex {
    pub const fn new(x: f32, y: f32, color: Color) -> Self {
        Vertex {
            position: Vec2::new(x, y),
            color,
        }
    }
}
