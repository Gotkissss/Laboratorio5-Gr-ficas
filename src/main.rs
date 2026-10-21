use std::time::Instant;

use raylib::prelude::*;

use rellenar_triangulos::color::Color as Rgb;
use rellenar_triangulos::framebuffer::{Framebuffer, HEIGHT, WIDTH};
use rellenar_triangulos::pipeline::{primitive_assembly, process_fragments, rasterize};
use rellenar_triangulos::scenes;
use rellenar_triangulos::triangle::TriangleMode;

const BACKGROUND: u32 = 0x1E1E2E;

fn mode_name(mode: TriangleMode) -> &'static str {
    match mode {
        TriangleMode::Wireframe => "Wireframe",
        TriangleMode::FilledBarycentric => "Relleno baricentrico",
        TriangleMode::FilledScanline => "Relleno scanline",
    }
}

fn main() {
    let (mut window, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Laboratorio 5 - Rellenar triángulos")
        .build();
    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT, Rgb::from_hex(BACKGROUND));

    // La textura se crea una sola vez y en cada cuadro solo se actualizan sus píxeles.
    let image = Image::gen_image_color(WIDTH, HEIGHT, Color::BLACK);
    let mut texture = window
        .load_texture_from_image(&thread, &image)
        .expect("no se pudo crear la textura");

    let all_scenes = scenes::all();
    let mut scene_index = 0;
    let mut mode = TriangleMode::FilledScanline;
    let mut show_help = true;
    let mut show_magnifier = false;

    while !window.window_should_close() {
        // --- Entrada ---
        if window.is_key_pressed(KeyboardKey::KEY_ONE) {
            mode = TriangleMode::Wireframe;
        }
        if window.is_key_pressed(KeyboardKey::KEY_TWO) {
            mode = TriangleMode::FilledBarycentric;
        }
        if window.is_key_pressed(KeyboardKey::KEY_THREE) {
            mode = TriangleMode::FilledScanline;
        }
        if window.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            scene_index = (scene_index + 1) % all_scenes.len();
        }
        if window.is_key_pressed(KeyboardKey::KEY_LEFT) {
            scene_index = (scene_index + all_scenes.len() - 1) % all_scenes.len();
        }
        if window.is_key_pressed(KeyboardKey::KEY_H) {
            show_help = !show_help;
        }
        if window.is_key_pressed(KeyboardKey::KEY_Z) {
            show_magnifier = !show_magnifier;
        }

        // --- Pipeline ---
        let scene = &all_scenes[scene_index];
        let triangles = primitive_assembly(&scene.vertices);

        // Solo medimos la generación de fragmentos (no la textura ni la ventana).
        let start = Instant::now();
        let fragments = rasterize(&triangles, mode);
        let raster_time = start.elapsed();

        framebuffer.clear();
        process_fragments(&mut framebuffer, &fragments);

        texture
            .update_texture(&framebuffer.to_rgba_bytes())
            .expect("no se pudo actualizar la textura");

        // --- Dibujo ---
        let mouse = window.get_mouse_position();
        let mut d = window.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE);

        if show_magnifier {
            draw_magnifier(&mut d, &texture, mouse);
        }

        let hud = format!(
            "{} | {} | {} fragmentos | raster: {:.3} ms",
            mode_name(mode),
            scene.name,
            fragments.len(),
            raster_time.as_secs_f64() * 1000.0
        );
        d.draw_rectangle(0, HEIGHT - 26, WIDTH, 26, Color::new(0, 0, 0, 180));
        d.draw_text(&hud, 8, HEIGHT - 20, 14, Color::RAYWHITE);

        if show_help {
            d.draw_rectangle(WIDTH - 250, 0, 250, 112, Color::new(0, 0, 0, 180));
            let lines = [
                "1/2/3: wireframe/bari/scanline",
                "<- ->: cambiar escena",
                "Z: lupa (sigue al mouse)",
                "H: ocultar esta ayuda",
                "ESC: salir",
            ];
            for (i, line) in lines.iter().enumerate() {
                d.draw_text(line, WIDTH - 242, 8 + i as i32 * 20, 14, Color::RAYWHITE);
            }
        }
    }
}

/// Dibuja una lupa en la esquina inferior izquierda que aumenta 8 veces la
/// zona alrededor del mouse. Sirve para ver los bordes píxel por píxel.
fn draw_magnifier(d: &mut RaylibDrawHandle, texture: &Texture2D, mouse: Vector2) {
    const SOURCE_SIZE: f32 = 24.0;
    const ZOOM: f32 = 8.0;
    let size = SOURCE_SIZE * ZOOM;

    let source = Rectangle::new(
        (mouse.x - SOURCE_SIZE / 2.0).floor(),
        (mouse.y - SOURCE_SIZE / 2.0).floor(),
        SOURCE_SIZE,
        SOURCE_SIZE,
    );
    let dest = Rectangle::new(8.0, HEIGHT as f32 - 34.0 - size, size, size);

    d.draw_rectangle_rec(dest, Color::BLACK);
    d.draw_texture_pro(texture, source, dest, Vector2::zero(), 0.0, Color::WHITE);
    d.draw_rectangle_lines_ex(dest, 2.0, Color::RAYWHITE);
    d.draw_rectangle_lines_ex(source, 1.0, Color::RAYWHITE);
}
