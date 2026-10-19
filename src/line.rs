// Línea con el algoritmo de Bresenham. Devuelve los fragmentos en lugar de
// pintar directo, así el triángulo wireframe es solo "tres líneas juntas".

use crate::fragment::Fragment;
use crate::vertex::Vertex;

pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    // El píxel (x, y) cubre el cuadrado [x, x + 1) x [y, y + 1),
    // así que el píxel de un punto es el piso de su posición.
    let mut x0 = a.position.x.floor() as i32;
    let mut y0 = a.position.y.floor() as i32;
    let x1 = b.position.x.floor() as i32;
    let y1 = b.position.y.floor() as i32;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        fragments.push(Fragment::new(x0, y0, a.color));
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }

    fragments
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn linea_horizontal_incluye_ambos_extremos() {
        let a = Vertex::new(2.0, 5.0, Color::WHITE);
        let b = Vertex::new(6.0, 5.0, Color::WHITE);
        let xs: Vec<i32> = line(&a, &b).iter().map(|f| f.x).collect();
        assert_eq!(xs, vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn linea_de_un_solo_punto() {
        let a = Vertex::new(3.2, 3.7, Color::WHITE);
        assert_eq!(line(&a, &a).len(), 1);
    }
}
