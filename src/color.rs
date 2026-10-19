// Color RGB sencillo. No uso el Color de raylib aquí para que la parte de
// rasterización no dependa de la ventana (así las pruebas corren sin raylib).

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Crea un color a partir de un número tipo 0xRRGGBB.
    pub const fn from_hex(hex: u32) -> Self {
        Color {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const WHITE: Color = Color::new(255, 255, 255);
}
