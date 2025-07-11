<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

Instrucciones del proyecto de relleno de polígonos

Este proyecto de Rust implementa algoritmos de relleno de polígonos mediante técnicas de líneas de escaneo.

Objetivos del proyecto
- Implementar el relleno de polígonos complejos con múltiples vértices
- Gestionar polígonos con agujeros (áreas de exclusión)
- Generar imágenes BMP con polígonos rellenos
- Admitir diferentes colores para relleno y borde

Componentes clave
- Estructura de puntos para coordenadas 2D
- Estructura de color para valores RGB
- Estructura PolygonFiller con implementación del algoritmo de líneas de escaneo
- Soporte para renderizado de polígonos con agujeros

Patrones de uso
- Usar el algoritmo de líneas de escaneo para un relleno de polígonos eficiente
- Gestionar casos extremos en intersecciones de polígonos
- Mantener una gestión de color adecuada para bordes y rellenos
- Generar la salida en formato BMP para compatibilidad

Estilo del código
- Usar nombres de variable claros y descriptivos
- Implementar un manejo adecuado de errores
- Seguir las convenciones y mejores prácticas de Rust
- Mantener las funciones enfocadas y modulares
