// Funciones para generar los fragmentos de un triángulo.
//
// Convención que uso en todo el laboratorio: el píxel (x, y) es el cuadrado
// [x, x + 1) x [y, y + 1) y lo "probamos" en su centro (x + 0.5, y + 0.5).

use crate::fragment::Fragment;
use crate::line::line;
use crate::vertex::Vertex;

/// Cómo se generan los fragmentos de cada triángulo.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriangleMode {
    Wireframe,
    FilledBarycentric,
}

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

/// Punto en f64. Hago las cuentas del relleno en f64 para tener más precisión.
#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn to_point(v: &Vertex) -> Point {
    Point {
        x: v.position.x as f64,
        y: v.position.y as f64,
    }
}

/// "Función de arista": dos veces el área con signo del triángulo (a, b, p).
/// Es positiva de un lado de la recta ab, negativa del otro y cero sobre ella.
fn edge_function(a: Point, b: Point, p: Point) -> f64 {
    (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)
}

/// Algoritmo 1: recorrer la caja que encierra al triángulo y quedarse con los
/// centros de píxel cuyas tres coordenadas baricéntricas indican que están adentro.
pub fn triangle_barycentric(
    v1: &Vertex,
    v2: &Vertex,
    v3: &Vertex,
    width: i32,
    height: i32,
) -> Vec<Fragment> {
    let (a, b, c) = (to_point(v1), to_point(v2), to_point(v3));

    // Área (con signo) del triángulo completo. Si es cero los vértices están
    // alineados: no hay interior y no podemos dividir entre ella.
    let area = edge_function(a, b, c);
    if area == 0.0 {
        return Vec::new();
    }

    // Caja envolvente: solo los píxeles cuyo centro puede caer en el triángulo,
    // recortada a los límites de la imagen.
    let min_x = a.x.min(b.x).min(c.x);
    let max_x = a.x.max(b.x).max(c.x);
    let min_y = a.y.min(b.y).min(c.y);
    let max_y = a.y.max(b.y).max(c.y);

    let x_start = ((min_x - 0.5).ceil() as i32).max(0);
    let x_end = ((max_x - 0.5).floor() as i32).min(width - 1);
    let y_start = ((min_y - 0.5).ceil() as i32).max(0);
    let y_end = ((max_y - 0.5).floor() as i32).min(height - 1);

    let color = v1.color;
    let mut fragments = Vec::new();

    for y in y_start..=y_end {
        for x in x_start..=x_end {
            let p = Point {
                x: x as f64 + 0.5,
                y: y as f64 + 0.5,
            };

            // Coordenadas baricéntricas. Dividir entre el área con signo hace que
            // el resultado no dependa del orden (horario/antihorario) de los vértices.
            let w1 = edge_function(b, c, p) / area;
            let w2 = edge_function(c, a, p) / area;
            let w3 = edge_function(a, b, p) / area;

            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                fragments.push(Fragment::new(x, y, color));
            }
        }
    }

    fragments
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    fn v(x: f32, y: f32) -> Vertex {
        Vertex::new(x, y, Color::WHITE)
    }

    #[test]
    fn baricentrico_genera_fragmentos() {
        let fragments = triangle_barycentric(&v(0.0, 0.0), &v(10.0, 0.0), &v(0.0, 10.0), 100, 100);
        assert!(!fragments.is_empty());
    }

    #[test]
    fn baricentrico_degenerado_no_genera_nada() {
        let fragments = triangle_barycentric(&v(0.0, 0.0), &v(5.0, 5.0), &v(10.0, 10.0), 100, 100);
        assert!(fragments.is_empty());
    }

    #[test]
    fn baricentrico_no_depende_del_orden() {
        let a = triangle_barycentric(&v(1.0, 1.0), &v(30.0, 5.0), &v(12.0, 25.0), 100, 100);
        let b = triangle_barycentric(&v(12.0, 25.0), &v(30.0, 5.0), &v(1.0, 1.0), 100, 100);
        assert_eq!(a.len(), b.len());
    }
}
