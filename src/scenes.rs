// Escenas de prueba. Cada escena es una lista de vértices que después
// pasa por el ensamblaje de primitivas (cada 3 vértices = 1 triángulo).

use crate::color::Color;
use crate::vertex::Vertex;

pub struct Scene {
    pub name: &'static str,
    pub vertices: Vec<Vertex>,
}

fn v(x: f32, y: f32, color: Color) -> Vertex {
    Vertex::new(x, y, color)
}

/// Escena "vitrina" con un ejemplo de cada caso que pide el laboratorio.
#[rustfmt::skip]
pub fn showcase() -> Scene {
    let red = Color::from_hex(0xF38BA8);
    let yellow = Color::from_hex(0xF9E2AF);
    let blue = Color::from_hex(0x89B4FA);
    let green = Color::from_hex(0xA6E3A1);
    let peach = Color::from_hex(0xFAB387);
    let mauve = Color::from_hex(0xCBA6F7);
    let teal = Color::from_hex(0x94E2D5);
    let text = Color::from_hex(0xCDD6F4);
    let pink = Color::from_hex(0xF5C2E7);

    Scene {
        name: "Vitrina (todos los casos)",
        vertices: vec![
            // parcialmente fuera (esquina superior izquierda)
            v(-70.0, -50.0, red), v(190.0, 40.0, red), v(20.0, 190.0, red),
            // pequeño
            v(250.0, 40.0, yellow), v(266.0, 47.0, yellow), v(255.0, 60.0, yellow),
            // grande
            v(320.0, 60.0, blue), v(620.0, 120.0, blue), v(420.0, 400.0, blue),
            // con borde horizontal (abajo)
            v(40.0, 380.0, green), v(230.0, 380.0, green), v(130.0, 230.0, green),
            // con borde vertical, vértices en orden horario
            v(660.0, 140.0, peach), v(780.0, 240.0, peach), v(660.0, 330.0, peach),
            // dos triángulos que comparten la diagonal
            v(470.0, 430.0, mauve), v(640.0, 430.0, mauve), v(640.0, 560.0, mauve),
            v(470.0, 430.0, teal), v(640.0, 560.0, teal), v(470.0, 560.0, teal),
            // tres vértices sobre la misma línea (degenerado)
            v(60.0, 450.0, text), v(200.0, 500.0, text), v(340.0, 550.0, text),
            // parcialmente fuera (esquina inferior derecha)
            v(700.0, 470.0, pink), v(880.0, 560.0, pink), v(730.0, 680.0, pink),
        ],
    }
}

/// Todas las escenas que se pueden ver en la ventana (con las flechas).
pub fn all() -> Vec<Scene> {
    vec![showcase()]
}
