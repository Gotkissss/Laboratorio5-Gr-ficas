use raylib::prelude::*;

use rellenar_triangulos::color::Color as Rgb;
use rellenar_triangulos::framebuffer::{Framebuffer, HEIGHT, WIDTH};
use rellenar_triangulos::pipeline::{primitive_assembly, process_fragments, rasterize};
use rellenar_triangulos::scenes;

fn main() {
    let (mut window, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Laboratorio 5 - Rellenar triángulos")
        .build();
    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT, Rgb::from_hex(0x1E1E2E));

    // La textura se crea una sola vez y en cada cuadro solo se actualizan sus píxeles.
    let image = Image::gen_image_color(WIDTH, HEIGHT, Color::BLACK);
    let mut texture = window
        .load_texture_from_image(&thread, &image)
        .expect("no se pudo crear la textura");

    let scene = scenes::showcase();
    let triangles = primitive_assembly(&scene.vertices);

    while !window.window_should_close() {
        framebuffer.clear();
        let fragments = rasterize(&triangles);
        process_fragments(&mut framebuffer, &fragments);

        texture
            .update_texture(&framebuffer.to_rgba_bytes())
            .expect("no se pudo actualizar la textura");

        let mut d = window.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }
}
