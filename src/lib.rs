// Biblioteca del renderer. La separo de main.rs para poder usar los mismos
// módulos desde la ventana, desde las pruebas y desde los benchmarks.

pub mod color;
pub mod compare;
pub mod fragment;
pub mod framebuffer;
pub mod line;
pub mod pipeline;
pub mod scenes;
pub mod triangle;
pub mod vertex;
