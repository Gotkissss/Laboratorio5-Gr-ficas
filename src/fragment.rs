// Un fragmento es "un candidato a píxel": la posición entera del píxel que
// cubre la primitiva y el color con el que se quiere pintar.
// Todavía no hay profundidad porque el laboratorio es en 2D.

use crate::color::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fragment {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl Fragment {
    pub const fn new(x: i32, y: i32, color: Color) -> Self {
        Fragment { x, y, color }
    }
}
