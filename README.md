# Laboratorio de Relleno de Polígonos - Lab 1

Este proyecto implementa un algoritmo de relleno de polígonos usando la técnica de scanline en Rust.

## Descripción

El proyecto dibuja y rellena múltiples polígonos con diferentes formas y colores:

- **Polígono 1**: Forma compleja irregular de 10 vértices - Amarillo con borde blanco
- **Polígono 2**: Cuadrilátero ordenado en sentido horario - Azul con borde blanco  
- **Polígono 3**: Triángulo equilátero - Rojo con borde blanco
- **Polígono 4**: Rectángulo verde con agujero rectangular interno - Verde con borde blanco
- **Polígono 5**: Agujero rectangular dentro del Polígono 4 - No se rellena (permanece negro como el fondo)

## Dependencias

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

1. **Detección de intersecciones**: Para cada línea horizontal (scanline) encuentra las intersecciones con los bordes del polígono
2. **Ordenamiento**: Ordena las intersecciones de izquierda a derecha
3. **Relleno**: Rellena entre pares de intersecciones
4. **Manejo de agujeros**: Para polígonos con agujeros excluye las áreas internas detectando intersecciones con el polígono del agujero

## Ramas Específicas

- `main`: Contiene todos los polígonos renderizados (solo merges de otras ramas)
- `Polygon-1`: Solo el polígono 1 (amarillo con borde blanco)
- `Polygon-2`: Solo el polígono 2 (azul con borde blanco)
- `Polygon-3`: Solo el polígono 3 (rojo con borde blanco)  
- `Polygon-4`: Polígono 4 con agujero (verde con borde blanco)

**Nota**: La rama `main` contiene únicamente commits de merge de las otras ramas como se requiere en las especificaciones.

## Coordenadas de los Polígonos

### Polígono 1 (Forma Irregular - 10 vértices)
**Estructura:** Polígono complejo con forma irregular
**Coordenadas:** (165 380) (185 360) (180 330) (207 345) (233 330) (230 360) (250 380) (220 385) (205 410) (193 383)

### Polígono 2 (Cuadrilátero - 4 vértices)
**Estructura:** Cuadrilátero simple ordenado en sentido horario
**Coordenadas:** (288 286) (339 251) (374 302) (321 335)

### Polígono 3 (Triángulo - 3 vértices)
**Estructura:** Triángulo ordenado en sentido horario
**Coordenadas:** (377 249) (436 249) (411 197)

### Polígono 4 (Rectángulo con agujero - 4 vértices)
**Estructura:** Rectángulo verde con agujero rectangular interno
**Coordenadas:** (500 180) (650 180) (650 280) (500 280)

### Polígono 5 (Agujero - 4 vértices)
**Estructura:** Rectángulo que forma un agujero dentro del Polígono 4
**Coordenadas:** (550 210) (600 210) (600 250) (550 250)

## Colores

- **Fondo**: Negro
- **Polígono 1**: Amarillo (#FFFF00)
- **Polígono 2**: Azul (#0000FF)
- **Polígono 3**: Rojo (#FF0000)
- **Polígono 4**: Verde (#00FF00)
- **Todos los bordes**: Blanco (#FFFFFF)
