// Pruebas de integración: los dos rellenos deben generar exactamente las
// mismas posiciones, sin repetir píxeles y sin depender del orden de los vértices.
//
// Correr con: cargo test --release

use std::collections::HashSet;

use rellenar_triangulos::color::Color;
use rellenar_triangulos::compare::compare;
use rellenar_triangulos::pipeline::rasterize_in;
use rellenar_triangulos::scenes::{test_cases, Xorshift};
use rellenar_triangulos::triangle::{triangle_barycentric, triangle_scanline, TriangleMode};
use rellenar_triangulos::vertex::Vertex;

const W: i32 = 800;
const H: i32 = 600;

#[test]
fn casos_del_laboratorio_coinciden() {
    for scene in test_cases() {
        let r = compare(&scene.vertices, W, H);
        assert!(
            r.same_positions(),
            "{}: solo baricéntrico {:?}, solo scanline {:?}",
            scene.name,
            &r.only_barycentric[..r.only_barycentric.len().min(5)],
            &r.only_scanline[..r.only_scanline.len().min(5)]
        );
        assert_eq!(
            r.barycentric_duplicates, 0,
            "{}: repetidos (bari)",
            scene.name
        );
        assert_eq!(r.scanline_duplicates, 0, "{}: repetidos (scan)", scene.name);
        assert!(!r.barycentric_changes_when_reversed, "{}", scene.name);
        assert!(!r.scanline_changes_when_reversed, "{}", scene.name);
    }
}

#[test]
fn colineales_no_generan_fragmentos() {
    let c = Color::WHITE;
    let (a, b, d) = (
        Vertex::new(10.0, 10.0, c),
        Vertex::new(20.0, 30.0, c),
        Vertex::new(30.0, 50.0, c),
    );
    assert!(triangle_barycentric(&a, &b, &d, W, H).is_empty());
    assert!(triangle_scanline(&a, &b, &d, W, H).is_empty());
    // Tres veces el mismo punto también es degenerado.
    assert!(triangle_barycentric(&a, &a, &a, W, H).is_empty());
    assert!(triangle_scanline(&a, &a, &a, W, H).is_empty());
}

#[test]
fn fuera_de_la_imagen_no_genera_fragmentos() {
    let c = Color::WHITE;
    let (a, b, d) = (
        Vertex::new(-50.0, -50.0, c),
        Vertex::new(-10.0, -40.0, c),
        Vertex::new(-30.0, -5.0, c),
    );
    assert!(triangle_barycentric(&a, &b, &d, W, H).is_empty());
    assert!(triangle_scanline(&a, &b, &d, W, H).is_empty());
}

/// Miles de triángulos con coordenadas decimales al azar, comparados uno por uno.
#[test]
fn triangulos_aleatorios_coinciden_uno_por_uno() {
    let mut rng = Xorshift::new(42);
    let c = Color::WHITE;
    let mut differences = 0;
    for _ in 0..5_000 {
        let mut p = || {
            Vertex::new(
                rng.next_f32() * 300.0 - 20.0,
                rng.next_f32() * 300.0 - 20.0,
                c,
            )
        };
        let (a, b, d) = (p(), p(), p());
        let bary: HashSet<(i32, i32)> = triangle_barycentric(&a, &b, &d, W, H)
            .iter()
            .map(|f| (f.x, f.y))
            .collect();
        let scan: HashSet<(i32, i32)> = triangle_scanline(&a, &b, &d, W, H)
            .iter()
            .map(|f| (f.x, f.y))
            .collect();
        differences += bary.symmetric_difference(&scan).count();
    }
    assert_eq!(differences, 0);
}

/// Un rectángulo dividido en una malla de triángulos con los vértices interiores
/// movidos al azar. Cada píxel del rectángulo debe salir exactamente una vez:
/// ni huecos ni repetidos entre triángulos vecinos. Se repite con 100 mallas.
#[test]
fn malla_cubre_cada_pixel_una_sola_vez() {
    let c = Color::WHITE;
    let (cols, rows) = (30, 20);

    for seed in 1..=100u64 {
        let mut rng = Xorshift::new(seed * 7919);
        let cell = 5.0 + (rng.next_f32() * 15.0).floor();
        let (x0, y0) = (10.0_f32, 10.0_f32);

        // Los puntos del borde se quedan fijos para que el contorno sea el rectángulo.
        let mut grid = vec![vec![(0.0_f32, 0.0_f32); cols + 1]; rows + 1];
        for (j, row) in grid.iter_mut().enumerate() {
            for (i, point) in row.iter_mut().enumerate() {
                let mut x = x0 + i as f32 * cell;
                let mut y = y0 + j as f32 * cell;
                if i > 0 && i < cols && j > 0 && j < rows {
                    x += (rng.next_f32() - 0.5) * cell * 0.4;
                    y += (rng.next_f32() - 0.5) * cell * 0.4;
                }
                *point = (x, y);
            }
        }

        // Cada celda se parte en dos triángulos, alternando la diagonal.
        let mut triangles = Vec::new();
        for j in 0..rows {
            for i in 0..cols {
                let p = |i: usize, j: usize| Vertex::new(grid[j][i].0, grid[j][i].1, c);
                if (i + j) % 2 == 0 {
                    triangles.push([p(i, j), p(i + 1, j), p(i + 1, j + 1)]);
                    triangles.push([p(i, j), p(i + 1, j + 1), p(i, j + 1)]);
                } else {
                    triangles.push([p(i, j), p(i + 1, j), p(i, j + 1)]);
                    triangles.push([p(i + 1, j), p(i + 1, j + 1), p(i, j + 1)]);
                }
            }
        }

        let expected = (cols as f32 * cell) as usize * (rows as f32 * cell) as usize;
        for mode in [
            TriangleMode::FilledBarycentric,
            TriangleMode::FilledScanline,
        ] {
            let fragments = rasterize_in(&triangles, mode, W, H);
            let unique: HashSet<(i32, i32)> = fragments.iter().map(|f| (f.x, f.y)).collect();
            assert_eq!(fragments.len(), expected, "malla {seed}, {mode:?}: total");
            assert_eq!(unique.len(), expected, "malla {seed}, {mode:?}: repetidos");
        }
    }
}
