# Polygon Filling Laboratory - Lab 1

Este proyecto implementa un algoritmo de relleno de polígonos usando la técnica de scanline en Rust.

## Descripción

El proyecto dibuja y rellena múltiples polígonos con diferentes formas y colores:

- **Polígono 1**: Forma compleja irregular de 10 vértices - Amarillo con borde blanco
- **Polígono 2**: Cuadrilátero ordenado en sentido horario - Azul con borde blanco  
- **Polígono 3**: Triángulo equilátero - Rojo con borde blanco
- **Polígono 4**: Polígono complejo de 18 vértices con agujero interno - Verde con borde blanco
- **Polígono 5**: Agujero cuadrangular dentro del Polígono 4 - No se rellena (solo borde)

## Requisitos

- Rust (edición 2021 o superior)
- Dependencia: `image = "0.24"`

## Compilación y Ejecución

```bash
cargo build --release
cargo run
```

El programa generará un archivo `out.bmp` con todos los polígonos renderizados.

## Estructura del Proyecto

- `src/main.rs`: Implementación principal del algoritmo
- `out.bmp`: Imagen de salida generada (requerido por el laboratorio)
- `Cargo.toml`: Configuración del proyecto y dependencias

## Algoritmo

El proyecto utiliza el algoritmo de scanline para el relleno de polígonos:

1. **Detección de intersecciones**: Para cada línea horizontal (scanline), encuentra las intersecciones con los bordes del polígono
2. **Ordenamiento**: Ordena las intersecciones de izquierda a derecha
3. **Relleno**: Rellena entre pares de intersecciones
4. **Manejo de agujeros**: Para polígonos con agujeros, excluye las áreas internas detectando intersecciones con el polígono del agujero

## Branches Específicas

- `main`: Contiene todos los polígonos renderizados (solo merges de otras branches)
- `Polygon-1`: Solo el polígono 1 (amarillo con borde blanco)
- `Polygon-2`: Solo el polígono 2 (azul con borde blanco)
- `Polygon-3`: Solo el polígono 3 (rojo con borde blanco)  
- `Polygon-4`: Polígono 4 con agujero (verde con borde blanco)

**Nota**: La branch `main` contiene únicamente commits de merge de las otras branches, como se requiere en las especificaciones.

## Coordenadas de los Polígonos

### Polígono 1 (Forma Irregular - 10 vértices)
**Estructura:** Polígono complejo con forma irregular
**Coordenadas:** (165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)

### Polígono 2 (Cuadrilátero - 4 vértices)
**Estructura:** Cuadrilátero simple ordenado en sentido horario
**Coordenadas:** (288, 286), (339, 251), (374, 302), (321, 335)

### Polígono 3 (Triángulo - 3 vértices)
**Estructura:** Triángulo ordenado en sentido horario
**Coordenadas:** (377, 249), (436, 249), (411, 197)

### Polígono 4 (Polígono Complejo - 18 vértices)
**Estructura:** Polígono complejo reorganizado para mejor visualización
**Coordenadas:** (413, 177), (466, 180), (517, 144), (552, 214), (597, 215), (580, 230), (632, 230), (615, 214), (659, 214), (672, 192), (761, 179), (750, 145), (660, 52), (676, 37), (535, 36), (553, 53), (502, 88), (448, 159)

### Polígono 5 (Agujero - 4 vértices)
**Estructura:** Cuadrilátero que forma un agujero dentro del Polígono 4
**Coordenadas:** (682, 175), (739, 170), (735, 148), (708, 120)

## Colores

- **Fondo**: Negro
- **Polígono 1**: Amarillo (#FFFF00)
- **Polígono 2**: Azul (#0000FF)
- **Polígono 3**: Rojo (#FF0000)
- **Polígono 4**: Verde (#00FF00)
- **Todos los bordes**: Blanco (#FFFFFF)
