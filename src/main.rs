use image::{ImageBuffer, Rgb, RgbImage};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
    
    fn white() -> Self {
        Color::new(255, 255, 255)
    }
    
    fn black() -> Self {
        Color::new(0, 0, 0)
    }
    
    fn yellow() -> Self {
        Color::new(255, 255, 0)
    }
    
    fn red() -> Self {
        Color::new(255, 0, 0)
    }
    
    fn green() -> Self {
        Color::new(0, 255, 0)
    }
    
    fn blue() -> Self {
        Color::new(0, 0, 255)
    }
    
    fn cyan() -> Self {
        Color::new(0, 255, 255)
    }
}

struct PolygonFiller {
    width: u32,
    height: u32,
    image: RgbImage,
}

impl PolygonFiller {
    fn new(width: u32, height: u32) -> Self {
        let image = ImageBuffer::new(width, height);
        PolygonFiller {
            width,
            height,
            image,
        }
    }
    
    fn fill_background(&mut self, color: Color) {
        for pixel in self.image.pixels_mut() {
            *pixel = Rgb([color.r, color.g, color.b]);
        }
    }
    
    fn draw_line(&mut self, p1: Point, p2: Point, color: Color) {
        let dx = (p2.x - p1.x).abs();
        let dy = (p2.y - p1.y).abs();
        let sx = if p1.x < p2.x { 1 } else { -1 };
        let sy = if p1.y < p2.y { 1 } else { -1 };
        let mut err = dx - dy;
        let mut x = p1.x;
        let mut y = p1.y;
        
        loop {
            if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                self.image.put_pixel(x as u32, y as u32, Rgb([color.r, color.g, color.b]));
            }
            
            if x == p2.x && y == p2.y {
                break;
            }
            
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
    
    fn fill_polygon(&mut self, vertices: &[Point], fill_color: Color, border_color: Color) {
        if vertices.len() < 3 {
            return;
        }
        
        // Find bounding box
        let min_y = vertices.iter().map(|p| p.y).min().unwrap();
        let max_y = vertices.iter().map(|p| p.y).max().unwrap();
        
        // Fill polygon using scanline algorithm
        for y in min_y..=max_y {
            let mut intersections = Vec::new();
            
            // Find intersections with polygon edges
            for i in 0..vertices.len() {
                let p1 = vertices[i];
                let p2 = vertices[(i + 1) % vertices.len()];
                
                if (p1.y <= y && p2.y > y) || (p2.y <= y && p1.y > y) {
                    let x = p1.x + ((y - p1.y) * (p2.x - p1.x)) / (p2.y - p1.y);
                    intersections.push(x);
                }
            }
            
            // Sort intersections
            intersections.sort();
            
            // Fill between pairs of intersections
            for chunk in intersections.chunks(2) {
                if chunk.len() == 2 {
                    let x1 = chunk[0];
                    let x2 = chunk[1];
                    
                    for x in x1..=x2 {
                        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                            self.image.put_pixel(x as u32, y as u32, Rgb([fill_color.r, fill_color.g, fill_color.b]));
                        }
                    }
                }
            }
        }
        
        // Draw border
        for i in 0..vertices.len() {
            let p1 = vertices[i];
            let p2 = vertices[(i + 1) % vertices.len()];
            self.draw_line(p1, p2, border_color);
        }
    }
    
    fn fill_polygon_with_hole(&mut self, outer_vertices: &[Point], hole_vertices: &[Point], fill_color: Color, border_color: Color) {
        if outer_vertices.len() < 3 {
            return;
        }
        
        // Find bounding box
        let min_y = outer_vertices.iter().map(|p| p.y).min().unwrap();
        let max_y = outer_vertices.iter().map(|p| p.y).max().unwrap();
        
        // Fill polygon using scanline algorithm with hole detection
        for y in min_y..=max_y {
            let mut outer_intersections = Vec::new();
            let mut hole_intersections = Vec::new();
            
            // Find intersections with outer polygon edges
            for i in 0..outer_vertices.len() {
                let p1 = outer_vertices[i];
                let p2 = outer_vertices[(i + 1) % outer_vertices.len()];
                
                if (p1.y <= y && p2.y > y) || (p2.y <= y && p1.y > y) {
                    let x = p1.x + ((y - p1.y) * (p2.x - p1.x)) / (p2.y - p1.y);
                    outer_intersections.push(x);
                }
            }
            
            // Find intersections with hole polygon edges
            if !hole_vertices.is_empty() {
                for i in 0..hole_vertices.len() {
                    let p1 = hole_vertices[i];
                    let p2 = hole_vertices[(i + 1) % hole_vertices.len()];
                    
                    if (p1.y <= y && p2.y > y) || (p2.y <= y && p1.y > y) {
                        let x = p1.x + ((y - p1.y) * (p2.x - p1.x)) / (p2.y - p1.y);
                        hole_intersections.push(x);
                    }
                }
            }
            
            // Sort intersections
            outer_intersections.sort();
            hole_intersections.sort();
            
            // Fill between pairs of outer intersections, excluding hole areas
            for chunk in outer_intersections.chunks(2) {
                if chunk.len() == 2 {
                    let x1 = chunk[0];
                    let x2 = chunk[1];
                    
                    for x in x1..=x2 {
                        // Check if point is inside hole
                        let mut inside_hole = false;
                        for hole_chunk in hole_intersections.chunks(2) {
                            if hole_chunk.len() == 2 && x >= hole_chunk[0] && x <= hole_chunk[1] {
                                inside_hole = true;
                                break;
                            }
                        }
                        
                        if !inside_hole && x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                            self.image.put_pixel(x as u32, y as u32, Rgb([fill_color.r, fill_color.g, fill_color.b]));
                        }
                    }
                }
            }
        }
        
        // Draw outer border
        for i in 0..outer_vertices.len() {
            let p1 = outer_vertices[i];
            let p2 = outer_vertices[(i + 1) % outer_vertices.len()];
            self.draw_line(p1, p2, border_color);
        }
        
        // Draw hole border
        if !hole_vertices.is_empty() {
            for i in 0..hole_vertices.len() {
                let p1 = hole_vertices[i];
                let p2 = hole_vertices[(i + 1) % hole_vertices.len()];
                self.draw_line(p1, p2, border_color);
            }
        }
    }
    
    fn save(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.image.save(filename)?;
        Ok(())
    }
}

fn get_polygon_1() -> Vec<Point> {
    // Polígono complejo de 10 puntos - forma irregular pero bien estructurada
    vec![
        Point::new(165, 380), Point::new(185, 360), Point::new(180, 330), 
        Point::new(207, 345), Point::new(233, 330), Point::new(230, 360), 
        Point::new(250, 380), Point::new(220, 385), Point::new(205, 410), 
        Point::new(193, 383)
    ]
}

fn get_polygon_2() -> Vec<Point> {
    // Cuadrilátero simple - ordenado en sentido horario
    vec![
        Point::new(288, 286), Point::new(339, 251), Point::new(374, 302), Point::new(321, 335)
    ]
}

fn get_polygon_3() -> Vec<Point> {
    // Triángulo simple - ordenado en sentido horario
    vec![
        Point::new(377, 249), Point::new(436, 249), Point::new(411, 197)
    ]
}

fn get_polygon_4() -> Vec<Point> {
    // Polígono complejo reorganizado para mejor estructura - sentido horario
    vec![
        Point::new(413, 177), Point::new(466, 180), Point::new(517, 144), 
        Point::new(552, 214), Point::new(597, 215), Point::new(580, 230), 
        Point::new(632, 230), Point::new(615, 214), Point::new(659, 214), 
        Point::new(672, 192), Point::new(761, 179), Point::new(750, 145), 
        Point::new(660, 52), Point::new(676, 37), Point::new(535, 36), 
        Point::new(553, 53), Point::new(502, 88), Point::new(448, 159)
    ]
}

fn get_polygon_5() -> Vec<Point> {
    // Agujero rectangular - ordenado en sentido horario
    vec![
        Point::new(682, 175), Point::new(739, 170), Point::new(735, 148), Point::new(708, 120)
    ]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut filler = PolygonFiller::new(800, 450);
    
    // Fill background with black
    filler.fill_background(Color::black());
    
    // Draw all polygons
    // Polygon 1 - Yellow with white border
    filler.fill_polygon(&get_polygon_1(), Color::yellow(), Color::white());
    
    // Polygon 2 - Blue with white border
    filler.fill_polygon(&get_polygon_2(), Color::blue(), Color::white());
    
    // Polygon 3 - Red with white border
    filler.fill_polygon(&get_polygon_3(), Color::red(), Color::white());
    
    // Polygon 4 with hole (Polygon 5) - Green with white border
    filler.fill_polygon_with_hole(&get_polygon_4(), &get_polygon_5(), Color::green(), Color::white());
    
    // Save the image
    filler.save("out.bmp")?;
    
    println!("Image saved as out.bmp");
    
    Ok(())
}
