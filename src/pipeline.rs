// Las etapas del "pipeline" que usamos en el laboratorio:
//   vértices -> ensamblaje de primitivas -> rasterización -> procesamiento de fragmentos

use crate::fragment::Fragment;
use crate::framebuffer::{Framebuffer, HEIGHT, WIDTH};
use crate::triangle::triangle_wireframe;
use crate::vertex::Vertex;

/// Agrupa los vértices de tres en tres. Si sobran vértices al final se ignoran.
pub fn primitive_assembly(vertices: &[Vertex]) -> Vec<[Vertex; 3]> {
    vertices
        .chunks_exact(3)
        .map(|chunk| [chunk[0], chunk[1], chunk[2]])
        .collect()
}

/// Rasteriza usando el tamaño por defecto del framebuffer.
pub fn rasterize(triangles: &[[Vertex; 3]]) -> Vec<Fragment> {
    rasterize_in(triangles, WIDTH, HEIGHT)
}

/// Igual que `rasterize`, pero con un tamaño de imagen cualquiera
/// (lo uso en las pruebas para tener imágenes pequeñas).
pub fn rasterize_in(triangles: &[[Vertex; 3]], width: i32, height: i32) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    for [v1, v2, v3] in triangles {
        fragments.extend(triangle_wireframe(v1, v2, v3, width, height));
    }
    fragments
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
