Mediana de 11 repeticiones (después de 1 de calentamiento).

| Conjunto | Modo | Fragmentos | Iteraciones | Mediana total | Min | Max | Por rasterización | ns/fragmento |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Todos los casos de prueba | Wireframe | 43420 | 20 | 9.20 ms | 9.14 ms | 9.48 ms | 460.235 µs | 10.60 |
| Todos los casos de prueba | Relleno baricentrico | 1228631 | 20 | 287.18 ms | 271.38 ms | 311.43 ms | 14358.905 µs | 11.69 |
| Todos los casos de prueba | Relleno scanline | 1228631 | 20 | 95.08 ms | 88.46 ms | 102.01 ms | 4754.220 µs | 3.87 |
| 1b. Un triangulo grande | Wireframe | 1723 | 100 | 0.82 ms | 0.79 ms | 2.72 ms | 8.200 µs | 4.76 |
| 1b. Un triangulo grande | Relleno baricentrico | 180600 | 100 | 185.06 ms | 181.01 ms | 186.51 ms | 1850.584 µs | 10.25 |
| 1b. Un triangulo grande | Relleno scanline | 180600 | 100 | 88.37 ms | 70.29 ms | 114.39 ms | 883.735 µs | 4.89 |
| 1a. Un triangulo pequeno | Wireframe | 27 | 100000 | 56.01 ms | 54.46 ms | 58.12 ms | 0.560 µs | 20.74 |
| 1a. Un triangulo pequeno | Relleno baricentrico | 34 | 100000 | 45.08 ms | 44.77 ms | 46.18 ms | 0.450 µs | 13.24 |
| 1a. Un triangulo pequeno | Relleno scanline | 34 | 100000 | 15.16 ms | 14.99 ms | 15.29 ms | 0.151 µs | 4.44 |
| 336 triangulos aleatorios pequenos | Wireframe | 17669 | 300 | 103.25 ms | 95.56 ms | 118.86 ms | 344.152 µs | 19.48 |
| 336 triangulos aleatorios pequenos | Relleno baricentrico | 35632 | 300 | 213.41 ms | 211.71 ms | 221.36 ms | 711.373 µs | 19.96 |
| 336 triangulos aleatorios pequenos | Relleno scanline | 35632 | 300 | 43.35 ms | 41.68 ms | 46.37 ms | 144.512 µs | 4.06 |
