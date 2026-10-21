use std::time::Instant;

use raylib::prelude::*;

use rellenar_triangulos::benchmark::{benchmark_sets, measure};
use rellenar_triangulos::color::Color as Rgb;
use rellenar_triangulos::compare::compare;
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
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--compare") {
        run_comparison();
        return;
    }
    if args.iter().any(|a| a == "--measure") {
        run_measurement();
        return;
    }
    if args.iter().any(|a| a == "--benchmark-triangles") {
        run_profile_benchmark(&args);
        return;
    }
    run_window();
}

/// Busca el valor que sigue a una opción, por ejemplo `--mode scan`.
fn option_value<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

/// `cargo flamegraph -- --benchmark-triangles [--mode bari|scan|wire|all] [--iterations N]`
/// Rasteriza muchas veces los casos de prueba y termina sin abrir la ventana,
/// para que el perfilador solo vea el trabajo de rasterización.
fn run_profile_benchmark(args: &[String]) {
    let modes = match option_value(args, "--mode").unwrap_or("all") {
        "bari" => vec![TriangleMode::FilledBarycentric],
        "scan" => vec![TriangleMode::FilledScanline],
        "wire" => vec![TriangleMode::Wireframe],
        _ => vec![
            TriangleMode::FilledBarycentric,
            TriangleMode::FilledScanline,
        ],
    };
    let iterations: u32 = option_value(args, "--iterations")
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    let set = benchmark_sets().remove(0);
    for mode in modes {
        let start = Instant::now();
        let mut total_fragments = 0;
        for _ in 0..iterations {
            let fragments = rasterize(std::hint::black_box(&set.triangles), mode);
            total_fragments += std::hint::black_box(fragments).len();
        }
        println!(
            "{}: {} iteraciones sobre '{}' en {:?} ({} fragmentos por iteración)",
            mode_name(mode),
            iterations,
            set.name,
            start.elapsed(),
            total_fragments / iterations.max(1) as usize
        );
    }
}

/// `cargo run --release -- --measure`
/// Mide cada modo sobre los mismos triángulos y reporta la mediana.
/// La tabla se imprime en Markdown y se guarda en resultados/mediciones.md.
fn run_measurement() {
    const REPETITIONS: u32 = 11;
    if cfg!(debug_assertions) {
        println!("OJO: esto es un build de debug, usa `cargo run --release -- --measure`.\n");
    }

    let modes = [
        TriangleMode::Wireframe,
        TriangleMode::FilledBarycentric,
        TriangleMode::FilledScanline,
    ];

    let mut table = String::new();
    table.push_str(&format!(
        "Mediana de {REPETITIONS} repeticiones (después de 1 de calentamiento).\n\n"
    ));
    table.push_str("| Conjunto | Modo | Fragmentos | Iteraciones | Mediana total | Min | Max | Por rasterización | ns/fragmento |\n");
    table.push_str("|---|---|---:|---:|---:|---:|---:|---:|---:|\n");

    for set in benchmark_sets() {
        println!(
            "Midiendo: {} ({} triángulos)",
            set.name,
            set.triangles.len()
        );
        for mode in modes {
            let m = measure(&set.triangles, mode, set.iterations, REPETITIONS);
            let row = format!(
                "| {} | {} | {} | {} | {:.2} ms | {:.2} ms | {:.2} ms | {:.3} µs | {:.2} |\n",
                set.name,
                mode_name(mode),
                m.fragments,
                m.iterations,
                m.median().as_secs_f64() * 1e3,
                m.min().as_secs_f64() * 1e3,
                m.max().as_secs_f64() * 1e3,
                m.median_per_iteration().as_secs_f64() * 1e6,
                m.ns_per_fragment()
            );
            table.push_str(&row);
        }
    }

    println!("\n{table}");
    std::fs::create_dir_all("resultados").expect("no se pudo crear la carpeta resultados");
    std::fs::write("resultados/mediciones.md", &table).expect("no se pudo guardar la tabla");
    println!("Tabla guardada en resultados/mediciones.md");
}

/// `cargo run --release -- --compare`
/// Compara las posiciones que generan los dos rellenos en cada caso de prueba.
fn run_comparison() {
    println!(
        "{:<40} {:>7} {:>7} {:>7} {:>7} {:>6} {:>6} {:>8}",
        "caso", "bari", "scan", "solo B", "solo S", "rep B", "rep S", "inverso"
    );
    let mut all_equal = true;
    for scene in scenes::test_cases() {
        let r = compare(&scene.vertices, WIDTH, HEIGHT);
        let reversed_ok = !r.barycentric_changes_when_reversed && !r.scanline_changes_when_reversed;
        println!(
            "{:<40} {:>7} {:>7} {:>7} {:>7} {:>6} {:>6} {:>8}",
            scene.name,
            r.barycentric_count,
            r.scanline_count,
            r.only_barycentric.len(),
            r.only_scanline.len(),
            r.barycentric_duplicates,
            r.scanline_duplicates,
            if reversed_ok { "igual" } else { "CAMBIA" }
        );
        if !r.same_positions() {
            all_equal = false;
            let sample: Vec<_> = r.only_barycentric.iter().take(4).collect();
            if !sample.is_empty() {
                println!("    ejemplo solo baricentrico: {:?}", sample);
            }
            let sample: Vec<_> = r.only_scanline.iter().take(4).collect();
            if !sample.is_empty() {
                println!("    ejemplo solo scanline:     {:?}", sample);
            }
        }
    }
    println!();
    if all_equal {
        println!("Ambos algoritmos generan exactamente las mismas posiciones en todos los casos.");
    } else {
        println!("Hay diferencias entre los algoritmos (ver arriba).");
    }
}

fn run_window() {
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
