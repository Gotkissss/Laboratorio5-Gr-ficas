// Medición de tiempos. Solo se mide la generación de fragmentos (rasterize):
// nada de texturas, ventana ni esperas entre cuadros.

use std::hint::black_box;
use std::time::{Duration, Instant};

use crate::pipeline::{primitive_assembly, rasterize};
use crate::scenes;
use crate::triangle::TriangleMode;
use crate::vertex::Vertex;

/// Un grupo de triángulos para medir y cuántas veces se rasteriza por medición.
pub struct BenchmarkSet {
    pub name: &'static str,
    pub triangles: Vec<[Vertex; 3]>,
    pub iterations: u32,
}

/// Los conjuntos que uso en el README.
pub fn benchmark_sets() -> Vec<BenchmarkSet> {
    let all_cases: Vec<Vertex> = scenes::test_cases()
        .into_iter()
        .flat_map(|scene| scene.vertices)
        .collect();
    let cases = scenes::test_cases();
    let small = &cases[0];
    let large = &cases[1];

    vec![
        BenchmarkSet {
            name: "Todos los casos de prueba",
            triangles: primitive_assembly(&all_cases),
            iterations: 20,
        },
        BenchmarkSet {
            name: "1b. Un triangulo grande",
            triangles: primitive_assembly(&large.vertices),
            iterations: 100,
        },
        BenchmarkSet {
            name: "1a. Un triangulo pequeno",
            triangles: primitive_assembly(&small.vertices),
            iterations: 100_000,
        },
        BenchmarkSet {
            name: "336 triangulos aleatorios pequenos",
            triangles: primitive_assembly(&scenes::random_triangles().vertices),
            iterations: 300,
        },
    ]
}

pub struct Measurement {
    /// Fragmentos que genera UNA rasterización del conjunto.
    pub fragments: usize,
    pub iterations: u32,
    /// Tiempo de cada repetición (cada una son `iterations` rasterizaciones).
    pub runs: Vec<Duration>,
}

impl Measurement {
    fn sorted_runs(&self) -> Vec<Duration> {
        let mut runs = self.runs.clone();
        runs.sort();
        runs
    }

    pub fn median(&self) -> Duration {
        let runs = self.sorted_runs();
        let n = runs.len();
        if n % 2 == 1 {
            runs[n / 2]
        } else {
            (runs[n / 2 - 1] + runs[n / 2]) / 2
        }
    }

    pub fn min(&self) -> Duration {
        self.sorted_runs()[0]
    }

    pub fn max(&self) -> Duration {
        *self.sorted_runs().last().unwrap()
    }

    /// Tiempo de una sola rasterización (mediana / iteraciones).
    pub fn median_per_iteration(&self) -> Duration {
        self.median() / self.iterations
    }

    /// Nanosegundos por fragmento, para comparar conjuntos de distinto tamaño.
    pub fn ns_per_fragment(&self) -> f64 {
        if self.fragments == 0 {
            return 0.0;
        }
        self.median_per_iteration().as_nanos() as f64 / self.fragments as f64
    }
}

/// Rasteriza `iterations` veces los mismos triángulos, y repite eso `repetitions`
/// veces. Antes hace una repetición de calentamiento que no se cuenta.
pub fn measure(
    triangles: &[[Vertex; 3]],
    mode: TriangleMode,
    iterations: u32,
    repetitions: u32,
) -> Measurement {
    let fragments = rasterize(triangles, mode).len();

    let run_once = || {
        let start = Instant::now();
        for _ in 0..iterations {
            let fragments = rasterize(black_box(triangles), black_box(mode));
            // black_box evita que el compilador elimine el trabajo porque "nadie usa" el resultado.
            black_box(fragments);
        }
        start.elapsed()
    };

    run_once(); // calentamiento
    let runs = (0..repetitions).map(|_| run_once()).collect();

    Measurement {
        fragments,
        iterations,
        runs,
    }
}
