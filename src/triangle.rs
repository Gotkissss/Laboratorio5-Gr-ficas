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
    FilledScanline,
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

/// x donde la arista (a, b) cruza la recta horizontal y.
/// Multiplico antes de dividir para que, si el resultado es exacto, salga exacto.
fn edge_x_at(a: Point, b: Point, y: f64) -> f64 {
    a.x + ((y - a.y) * (b.x - a.x)) / (b.y - a.y)
}

/// Algoritmo 2: recorrer la imagen fila por fila. En cada fila se busca dónde
/// entra y dónde sale el triángulo y se generan los fragmentos de ese tramo.
///
/// Reglas para no dejar huecos ni repetir píxeles:
/// - Una fila (centro yc) se cruza con una arista si y_arriba <= yc < y_abajo.
///   Así un vértice justo en una fila cuenta para la arista que "empieza" ahí
///   y no para la que "termina", y las aristas horizontales nunca se usan.
/// - En el tramo se toman los centros con x_izq <= xc < x_der.
pub fn triangle_scanline(
    v1: &Vertex,
    v2: &Vertex,
    v3: &Vertex,
    width: i32,
    height: i32,
) -> Vec<Fragment> {
    let mut points = [to_point(v1), to_point(v2), to_point(v3)];

    // Mismo criterio que el baricéntrico: sin área no hay interior.
    if edge_function(points[0], points[1], points[2]) == 0.0 {
        return Vec::new();
    }

    // Ordeno por y: top arriba, mid en medio, bottom abajo (y crece hacia abajo).
    points.sort_by(|p, q| p.y.total_cmp(&q.y));
    let [top, mid, bottom] = points;

    // Filas cuyo centro cumple top.y <= y + 0.5 < bottom.y, recortadas a la imagen.
    let y_start = ((top.y - 0.5).ceil() as i32).max(0);
    let y_end = ((bottom.y - 0.5).ceil() as i32).min(height); // exclusivo

    let color = v1.color;
    let mut fragments = Vec::new();

    for y in y_start..y_end {
        let yc = y as f64 + 0.5;

        // La arista larga (top -> bottom) siempre cruza la fila.
        let x_long = edge_x_at(top, bottom, yc);
        // La arista corta depende de si estamos arriba o abajo del vértice del medio.
        let x_short = if yc < mid.y {
            edge_x_at(top, mid, yc)
        } else {
            edge_x_at(mid, bottom, yc)
        };

        let (x_left, x_right) = if x_long < x_short {
            (x_long, x_short)
        } else {
            (x_short, x_long)
        };

        // Píxeles con x_left <= x + 0.5 < x_right.
        let x_start = ((x_left - 0.5).ceil() as i32).max(0);
        let x_end = ((x_right - 0.5).ceil() as i32).min(width); // exclusivo

        for x in x_start..x_end {
            fragments.push(Fragment::new(x, y, color));
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

    #[test]
    fn scanline_degenerado_no_genera_nada() {
        let fragments = triangle_scanline(&v(0.0, 0.0), &v(5.0, 5.0), &v(10.0, 10.0), 100, 100);
        assert!(fragments.is_empty());
    }

    #[test]
    fn scanline_rectangulo_de_dos_triangulos_sin_huecos_ni_repetidos() {
        // Dos triángulos que forman el cuadrado [0, 10) x [0, 10).
        let mut fragments =
            triangle_scanline(&v(0.0, 0.0), &v(10.0, 0.0), &v(10.0, 10.0), 100, 100);
        fragments.extend(triangle_scanline(
            &v(0.0, 0.0),
            &v(10.0, 10.0),
            &v(0.0, 10.0),
            100,
            100,
        ));
        assert_eq!(fragments.len(), 100);
    }
}
