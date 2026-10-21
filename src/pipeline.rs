// Las etapas del "pipeline" que usamos en el laboratorio:
//   vértices -> ensamblaje de primitivas -> rasterización -> procesamiento de fragmentos

use crate::fragment::Fragment;
use crate::framebuffer::{Framebuffer, HEIGHT, WIDTH};
use crate::triangle::{
    triangle_barycentric_into, triangle_scanline_into, triangle_wireframe_into, TriangleMode,
};
use crate::vertex::Vertex;

/// Agrupa los vértices de tres en tres. Si sobran vértices al final se ignoran.
pub fn primitive_assembly(vertices: &[Vertex]) -> Vec<[Vertex; 3]> {
    vertices
        .chunks_exact(3)
        .map(|chunk| [chunk[0], chunk[1], chunk[2]])
        .collect()
}

/// Rasteriza usando el tamaño por defecto del framebuffer.
pub fn rasterize(triangles: &[[Vertex; 3]], mode: TriangleMode) -> Vec<Fragment> {
    rasterize_in(triangles, mode, WIDTH, HEIGHT)
}

/// Igual que `rasterize`, pero con un tamaño de imagen cualquiera
/// (lo uso en las pruebas para tener imágenes pequeñas).
pub fn rasterize_in(
    triangles: &[[Vertex; 3]],
    mode: TriangleMode,
    width: i32,
    height: i32,
) -> Vec<Fragment> {
    // Primera versión: cada triángulo devolvía su propio Vec y luego se copiaba
    // con `extend` al Vec total. El flamegraph mostró que esa copia y el
    // crecimiento de los Vec se llevaban gran parte del tiempo, así que ahora
    // todos los triángulos escriben directo en un solo Vec con capacidad reservada.
    let mut fragments = Vec::with_capacity(estimate_fragments(triangles, mode, width, height));
    for [v1, v2, v3] in triangles {
        // Lo único que cambia entre modos es cómo se generan los fragmentos;
        // el ensamblaje y el procesamiento de fragmentos son los mismos.
        match mode {
            TriangleMode::Wireframe => {
                triangle_wireframe_into(v1, v2, v3, width, height, &mut fragments)
            }
            TriangleMode::FilledBarycentric => {
                triangle_barycentric_into(v1, v2, v3, width, height, &mut fragments)
            }
            TriangleMode::FilledScanline => {
                triangle_scanline_into(v1, v2, v3, width, height, &mut fragments)
            }
        }
    }
    fragments
}

/// Estimación barata de cuántos fragmentos van a salir, para reservar memoria
/// una sola vez: la mitad del área de la caja envolvente (recortada a la imagen)
/// en los rellenos, o la suma de los lados del contorno en wireframe.
fn estimate_fragments(
    triangles: &[[Vertex; 3]],
    mode: TriangleMode,
    width: i32,
    height: i32,
) -> usize {
    let mut total = 0.0_f32;
    for [a, b, c] in triangles {
        let (pa, pb, pc) = (a.position, b.position, c.position);
        total += match mode {
            TriangleMode::Wireframe => {
                let side = |p: crate::vertex::Vec2, q: crate::vertex::Vec2| {
                    (p.x - q.x).abs().max((p.y - q.y).abs()) + 1.0
                };
                side(pa, pb) + side(pb, pc) + side(pc, pa)
            }
            _ => {
                let min_x = pa.x.min(pb.x).min(pc.x).max(0.0);
                let max_x = pa.x.max(pb.x).max(pc.x).min(width as f32);
                let min_y = pa.y.min(pb.y).min(pc.y).max(0.0);
                let max_y = pa.y.max(pb.y).max(pc.y).min(height as f32);
                ((max_x - min_x).max(0.0) * (max_y - min_y).max(0.0)) / 2.0
            }
        };
    }
    total as usize
}

/// Procesamiento de fragmentos: por ahora solo se escribe el color en el framebuffer.
pub fn process_fragments(framebuffer: &mut Framebuffer, fragments: &[Fragment]) {
    for fragment in fragments {
        framebuffer.set_pixel(fragment.x, fragment.y, fragment.color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn ensamblaje_agrupa_de_tres_en_tres() {
        let v = Vertex::new(0.0, 0.0, Color::WHITE);
        assert_eq!(primitive_assembly(&[v; 7]).len(), 2);
    }
}
