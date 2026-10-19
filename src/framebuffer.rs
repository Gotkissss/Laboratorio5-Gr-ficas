// Framebuffer: un arreglo de colores del tamaño de la imagen.
// Todo lo que se dibuja pasa por aquí y después se copia a una textura de raylib.

use crate::color::Color;

/// Tamaño por defecto de la imagen que usamos en todo el laboratorio.
pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 600;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pub buffer: Vec<Color>,
    pub background_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, background_color: Color) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![background_color; (width * height) as usize],
            background_color,
        }
    }

    pub fn clear(&mut self) {
        let background = self.background_color;
        self.buffer.fill(background);
    }

    /// Pinta un píxel. Si está fuera de la imagen simplemente se ignora.
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return;
        }
        let index = (y * self.width + x) as usize;
        self.buffer[index] = color;
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Option<Color> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }
        Some(self.buffer[(y * self.width + x) as usize])
    }

    /// Convierte el buffer a bytes RGBA (el formato que espera la textura de raylib).
    pub fn to_rgba_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.buffer.len() * 4);
        for color in &self.buffer {
            bytes.extend_from_slice(&[color.r, color.g, color.b, 255]);
        }
        bytes
    }
}
