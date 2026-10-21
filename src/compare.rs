// Comparación de los dos rellenos: no basta con que "se vean iguales" en la
// ventana, así que comparo las posiciones (x, y) que genera cada algoritmo.

use std::collections::HashSet;

use crate::pipeline::{primitive_assembly, rasterize_in};
use crate::triangle::TriangleMode;
use crate::vertex::Vertex;

pub struct Comparison {
    pub barycentric_count: usize,
    pub scanline_count: usize,
    /// Fragmentos repetidos (misma posición dos veces) dentro de cada algoritmo.
    pub barycentric_duplicates: usize,
    pub scanline_duplicates: usize,
    /// Posiciones que solo genera uno de los dos algoritmos.
    pub only_barycentric: Vec<(i32, i32)>,
    pub only_scanline: Vec<(i32, i32)>,
    /// ¿Invertir el orden de los vértices cambia el resultado?
    pub barycentric_changes_when_reversed: bool,
    pub scanline_changes_when_reversed: bool,
}

impl Comparison {
    pub fn same_positions(&self) -> bool {
        self.only_barycentric.is_empty() && self.only_scanline.is_empty()
    }
}

fn positions(triangles: &[[Vertex; 3]], mode: TriangleMode, w: i32, h: i32) -> Vec<(i32, i32)> {
    rasterize_in(triangles, mode, w, h)
        .iter()
        .map(|f| (f.x, f.y))
        .collect()
}

fn sorted(mut list: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    list.sort_unstable();
    list
}

pub fn compare(vertices: &[Vertex], width: i32, height: i32) -> Comparison {
    let triangles = primitive_assembly(vertices);
    let reversed: Vec<[Vertex; 3]> = triangles.iter().map(|[a, b, c]| [*c, *b, *a]).collect();

    let bary = positions(&triangles, TriangleMode::FilledBarycentric, width, height);
    let scan = positions(&triangles, TriangleMode::FilledScanline, width, height);

    let bary_set: HashSet<(i32, i32)> = bary.iter().copied().collect();
    let scan_set: HashSet<(i32, i32)> = scan.iter().copied().collect();

    let mut only_barycentric: Vec<_> = bary_set.difference(&scan_set).copied().collect();
    let mut only_scanline: Vec<_> = scan_set.difference(&bary_set).copied().collect();
    only_barycentric.sort_unstable();
    only_scanline.sort_unstable();

    let bary_reversed = positions(&reversed, TriangleMode::FilledBarycentric, width, height);
    let scan_reversed = positions(&reversed, TriangleMode::FilledScanline, width, height);

    Comparison {
        barycentric_count: bary.len(),
        scanline_count: scan.len(),
        barycentric_duplicates: bary.len() - bary_set.len(),
        scanline_duplicates: scan.len() - scan_set.len(),
        only_barycentric,
        only_scanline,
        barycentric_changes_when_reversed: sorted(bary) != sorted(bary_reversed),
        scanline_changes_when_reversed: sorted(scan) != sorted(scan_reversed),
    }
}
