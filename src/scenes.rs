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

/// Casos de prueba del laboratorio. Todos usan el mismo color para que se
/// puedan comparar las imágenes de ambos algoritmos.
#[rustfmt::skip]
pub fn test_cases() -> Vec<Scene> {
    let c = Color::from_hex(0x89B4FA);
    vec![
        Scene { name: "1a. Pequeno", vertices: vec![
            v(100.0, 100.0, c), v(108.0, 104.0, c), v(102.0, 110.0, c),
        ]},
        Scene { name: "1b. Grande", vertices: vec![
            v(40.0, 560.0, c), v(760.0, 500.0, c), v(380.0, 30.0, c),
        ]},
        Scene { name: "2a. Borde horizontal", vertices: vec![
            v(100.0, 400.0, c), v(500.0, 400.0, c), v(260.0, 120.0, c),
        ]},
        Scene { name: "2b. Borde vertical", vertices: vec![
            v(300.0, 80.0, c), v(300.0, 520.0, c), v(650.0, 260.0, c),
        ]},
        Scene { name: "3. Orden inverso (1b al reves)", vertices: vec![
            v(380.0, 30.0, c), v(760.0, 500.0, c), v(40.0, 560.0, c),
        ]},
        Scene { name: "4. Parcialmente fuera", vertices: vec![
            v(-150.0, -80.0, c), v(500.0, 150.0, c), v(120.0, 700.0, c),
        ]},
        Scene { name: "5. Colineales", vertices: vec![
            v(100.0, 100.0, c), v(300.0, 250.0, c), v(500.0, 400.0, c),
        ]},
        Scene { name: "Extra: arista compartida", vertices: vec![
            v(200.0, 150.0, c), v(600.0, 150.0, c), v(600.0, 450.0, c),
            v(200.0, 150.0, c), v(600.0, 450.0, c), v(200.0, 450.0, c),
        ]},
        Scene { name: "Extra: vertices en centros de pixel", vertices: vec![
            v(100.5, 100.5, c), v(300.5, 100.5, c), v(300.5, 300.5, c),
            v(100.5, 100.5, c), v(300.5, 300.5, c), v(100.5, 300.5, c),
            v(500.5, 120.5, c), v(700.5, 320.5, c), v(500.5, 520.5, c),
        ]},
        Scene { name: "Extra: coordenadas con decimales", vertices: vec![
            v(120.3, 80.7, c), v(611.9, 140.2, c), v(333.33, 517.77, c),
        ]},
        fan(),
        random_triangles(),
    ]
}

/// Abanico de triángulos alrededor del centro: todos comparten aristas, así que
/// sirve para ver si hay píxeles repetidos o huecos entre triángulos vecinos.
pub fn fan() -> Scene {
    let segments = 24;
    let (cx, cy, radius) = (400.0_f32, 300.0_f32, 260.0_f32);
    let palette = [0x89B4FA, 0xA6E3A1, 0xF9E2AF, 0xF38BA8, 0xCBA6F7, 0x94E2D5];
    let mut vertices = Vec::new();
    for i in 0..segments {
        let a0 = i as f32 / segments as f32 * std::f32::consts::TAU;
        let a1 = (i + 1) as f32 / segments as f32 * std::f32::consts::TAU;
        let color = Color::from_hex(palette[i % palette.len()]);
        vertices.push(v(cx, cy, color));
        vertices.push(v(cx + radius * a0.cos(), cy + radius * a0.sin(), color));
        vertices.push(v(cx + radius * a1.cos(), cy + radius * a1.sin(), color));
    }
    Scene {
        name: "Extra: abanico (aristas compartidas)",
        vertices,
    }
}

/// Generador pseudoaleatorio muy simple (xorshift) para no agregar dependencias.
/// Siempre usa la misma semilla, así los resultados se pueden repetir.
pub struct Xorshift(u64);

impl Xorshift {
    pub fn new(seed: u64) -> Self {
        Xorshift(seed)
    }

    /// Número en [0, 1).
    pub fn next_f32(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 40) as f32 / (1u64 << 24) as f32
    }
}

/// Triángulos con coordenadas al azar. Cada uno vive dentro de su propia celda
/// de 40x40 para que no se encimen (así un píxel repetido sí indica un error).
/// La cuadrícula empieza en -20 para que los de las orillas queden medio afuera.
pub fn random_triangles() -> Scene {
    let mut rng = Xorshift::new(0x5EED_1234);
    let c = Color::from_hex(0xFAB387);
    let cell = 40.0;
    let mut vertices = Vec::new();
    for row in 0..16 {
        for col in 0..21 {
            let x0 = col as f32 * cell - 20.0;
            let y0 = row as f32 * cell - 20.0;
            for _ in 0..3 {
                let x = x0 + rng.next_f32() * cell;
                let y = y0 + rng.next_f32() * cell;
                vertices.push(v(x, y, c));
            }
        }
    }
    Scene {
        name: "Extra: 336 triangulos aleatorios",
        vertices,
    }
}

/// Todas las escenas que se pueden ver en la ventana (con las flechas).
pub fn all() -> Vec<Scene> {
    let mut scenes = vec![showcase()];
    scenes.extend(test_cases());
    scenes
}
