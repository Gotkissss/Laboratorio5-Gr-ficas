// Funciones para generar los fragmentos de un triángulo.

use crate::fragment::Fragment;
use crate::line::line;
use crate::vertex::Vertex;

/// Contorno del triángulo: tres llamadas a `line`, igual que en el laboratorio anterior.
/// Los fragmentos que caen fuera de la imagen se descartan aquí mismo.
pub fn triangle_wireframe(
    v1: &Vertex,
    v2: &Vertex,
    v3: &Vertex,
    width: i32,
    height: i32,
) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    fragments.extend(line(v1, v2));
    fragments.extend(line(v2, v3));
    fragments.extend(line(v3, v1));
    fragments.retain(|f| f.x >= 0 && f.y >= 0 && f.x < width && f.y < height);
    fragments
}
