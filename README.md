# Polygon Filling Laboratory - Lab 1

Este proyecto implementa un algoritmo de relleno de polígonos usando la técnica de scanline en Rust.

## Descripción

El proyecto dibuja y rellena múltiples polígonos con diferentes formas y colores:

- **Polígono 1**: Forma compleja de 10 puntos - Amarillo con borde blanco
- **Polígono 2**: Cuadrilátero - Azul con borde blanco  
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

### Polígono 1 (Complejo - 10 puntos)
(165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)

### Polígono 2 (Cuadrilátero)  
(321, 335), (288, 286), (339, 251), (374, 302)

### Polígono 3 (Triángulo)
(377, 249), (411, 197), (436, 249)

### Polígono 4 (Complejo con agujero)
(413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52), (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230), (597, 215), (552, 214), (517, 144), (466, 180)

### Polígono 5 (Agujero dentro del Polígono 4)
(682, 175), (708, 120), (735, 148), (739, 170)

## Colores

- **Fondo**: Negro
- **Polígono 1**: Amarillo (#FFFF00)
- **Polígono 2**: Azul (#0000FF)
- **Polígono 3**: Rojo (#FF0000)
- **Polígono 4**: Verde (#00FF00)
- **Todos los bordes**: Blanco (#FFFFFF)

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut filler = PolygonFiller::new(800, 450);
    
    // Fill background with black
    filler.fill_background(Color::black());
    
    // Draw only Polygon 2 - Blue with white border
    filler.fill_polygon(&get_polygon_2(), Color::blue(), Color::white());
    
    // Save the image
    filler.save("out.bmp")?;
    
    println!("Image saved as out.bmp");
    
    Ok(())
}
