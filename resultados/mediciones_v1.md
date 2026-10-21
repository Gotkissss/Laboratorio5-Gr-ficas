Mediana de 11 repeticiones (después de 1 de calentamiento).

| Conjunto | Modo | Fragmentos | Iteraciones | Mediana total | Min | Max | Por rasterización | ns/fragmento |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Todos los casos de prueba | Wireframe | 43420 | 20 | 24.98 ms | 23.86 ms | 28.92 ms | 1248.900 µs | 28.76 |
| Todos los casos de prueba | Relleno baricentrico | 1228631 | 20 | 464.11 ms | 456.89 ms | 480.68 ms | 23205.450 µs | 18.89 |
| Todos los casos de prueba | Relleno scanline | 1228631 | 20 | 272.44 ms | 267.86 ms | 285.42 ms | 13621.955 µs | 11.09 |
| 1b. Un triangulo grande | Wireframe | 1723 | 100 | 0.91 ms | 0.90 ms | 1.12 ms | 9.074 µs | 5.27 |
| 1b. Un triangulo grande | Relleno baricentrico | 180600 | 100 | 291.93 ms | 290.61 ms | 296.27 ms | 2919.323 µs | 16.16 |
| 1b. Un triangulo grande | Relleno scanline | 180600 | 100 | 172.21 ms | 166.80 ms | 174.08 ms | 1722.107 µs | 9.54 |
| 1a. Un triangulo pequeno | Wireframe | 27 | 100000 | 70.96 ms | 69.95 ms | 73.51 ms | 0.709 µs | 26.26 |
| 1a. Un triangulo pequeno | Relleno baricentrico | 34 | 100000 | 64.03 ms | 63.52 ms | 73.63 ms | 0.640 µs | 18.82 |
| 1a. Un triangulo pequeno | Relleno scanline | 34 | 100000 | 42.47 ms | 42.11 ms | 43.49 ms | 0.424 µs | 12.47 |
| 336 triangulos aleatorios pequenos | Wireframe | 17669 | 300 | 162.65 ms | 154.61 ms | 166.85 ms | 542.163 µs | 30.68 |
| 336 triangulos aleatorios pequenos | Relleno baricentrico | 35632 | 300 | 258.13 ms | 254.32 ms | 261.95 ms | 860.422 µs | 24.15 |
| 336 triangulos aleatorios pequenos | Relleno scanline | 35632 | 300 | 88.85 ms | 88.32 ms | 89.67 ms | 296.168 µs | 8.31 |
