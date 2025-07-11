<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

# Polygon Filling Project Instructions

This is a Rust project that implements polygon filling algorithms using scanline techniques.

## Project Goals
- Implement polygon filling for complex polygons with multiple vertices
- Handle polygons with holes (exclusion areas)
- Generate BMP images with filled polygons
- Support different colors for fill and border

## Key Components
- Point structure for 2D coordinates
- Color structure for RGB values
- PolygonFiller struct with scanline algorithm implementation
- Support for polygon-with-hole rendering

## Usage Patterns
- Use the scanline algorithm for efficient polygon filling
- Handle edge cases for polygon intersections
- Maintain proper color management for borders and fills
- Generate output in BMP format for compatibility

## Code Style
- Use clear, descriptive variable names
- Implement proper error handling
- Follow Rust conventions and best practices
- Keep functions focused and modular
