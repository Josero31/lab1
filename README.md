# Polygon Filling Laboratory - Lab 1

Este proyecto implementa un algoritmo de relleno de polígonos usando la técnica de scanline en Rust.

## Descripción

El proyecto dibuja y rellena múltiples polígonos con diferentes formas y colores:

- **Polígono 1**: Forma compleja de 10 puntos - Amarillo con borde blanco
- **Polígono 2**: Cuadrilátero - Cian con borde blanco  
- **Polígono 3**: Triángulo - Rojo con borde blanco
- **Polígono 4**: Polígono complejo con agujero (Polígono 5) - Verde con borde blanco
- **Polígono 5**: Agujero dentro del Polígono 4 - No se rellena

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
- `out.bmp`: Imagen de salida generada
- `Cargo.toml`: Configuración del proyecto y dependencias

## Algoritmo

El proyecto utiliza el algoritmo de scanline para el relleno de polígonos:

1. **Detección de intersecciones**: Para cada línea horizontal (scanline), encuentra las intersecciones con los bordes del polígono
2. **Ordenamiento**: Ordena las intersecciones de izquierda a derecha
3. **Relleno**: Rellena entre pares de intersecciones
4. **Manejo de agujeros**: Para polígonos con agujeros, excluye las áreas internas

## Branches Específicas

- `main`: Contiene todos los polígonos renderizados
- `Polygon-1`: Solo el polígono 1 (amarillo con borde blanco)
- `Polygon-3`: Solo el polígono 3 (rojo con borde blanco)  
- `Polygon-4`: Polígono 4 con agujero (verde con borde blanco)

## Colores

- **Fondo**: Negro
- **Polígono 1**: Amarillo (#FFFF00)
- **Polígono 2**: Cian (#00FFFF)
- **Polígono 3**: Rojo (#FF0000)
- **Polígono 4**: Verde (#00FF00)
- **Todos los bordes**: Blanco (#FFFFFF)
