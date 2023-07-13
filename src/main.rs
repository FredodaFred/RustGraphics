use minifb::Window;
use minifb::WindowOptions;
use std::{thread, time};
use rand::prelude::*;

fn main() {

    const width: usize= 1000;
    const height: usize = 1000;

    #[derive(Clone, Copy)]
    struct Color{r: u8, g: u8, b: u8}
    impl Color{
        fn difference(&self, other: Color) -> Color{
            Color { r: self.r.abs_diff(other.r), b: self.b.abs_diff(other.b), g: self.g.abs_diff(other.g) }
        }
        fn add(&self, other: Color)->Color {
            Color {r: self.r + other.r, b: self.b + other.b, g: self.g + other.g}
        }
    }
    #[derive(Clone, Copy)]
    struct Point{x: usize, y: usize}
    const WHITE: Color = Color{r: 255, g: 255, b: 255};
    const BLACK: Color = Color{r: 0, g: 0, b: 0};


    
    let mut window = Window::new("Test", width, height, WindowOptions::default()).unwrap();
    window.set_title("My New Title!");
    // Set background color to bright red

    fn from_u8_rgb(c: Color) -> u32 {
        let (r, g, b) = (c.r as u32, c.g as u32, c.b as u32);
        (r << 16) | (g << 8) | b
    }

    //TODO
    //Pass in buffer instead of entire window?
    fn linear_gradient(c1: Color, c2: Color, mut window: Window){
        let mut buffer: Vec<u32> = vec![0; width * height];
        let dr_dx: f64 = ((c2.r as f64) - (c1.r as f64))/ width as f64;
        let dg_dx: f64 = ((c2.g as f64) - (c1.g as f64))/ width as f64;
        let db_dx: f64 = ((c2.b as f64) - (c1.b as f64))/ width as f64;
        
        let mut count: usize = 0;
        for byte in buffer.iter_mut(){
            let x: usize = count % width;
            let r: f64 = (c1.r as f64) + dr_dx*(x as f64);
            let g: f64 = (c1.g as f64) + dg_dx*(x as f64);
            let b: f64 = (c1.b as f64) + db_dx*(x as f64);
            let c: Color = Color{r: r as u8, g: g as u8, b: b as u8};
            let val: u32 = from_u8_rgb(c);
            *byte = val;
            count += 1;
        }
        while window.is_open(){
            window.update_with_buffer(&buffer, width, height);
        }
    }
    
    //TODO
    //Pass in buffer instead of entire window?
    fn glowing_animation(colors: Vec<Color>, milli_cycle: u64, mut window: Window){
        while window.is_open(){
            if colors.len() == 0{
                panic!("No colors for animation added");
            }
            if colors.len() == 1{
                let val: u32 = from_u8_rgb(colors[0]);
                let buffer: Vec<u32> = vec![val; width * height];
                window.update_with_buffer(&buffer, width, height).unwrap();
                thread::sleep(time::Duration::from_millis(milli_cycle));
                continue;
            }
            for i in 0..=colors.len()-2{
                let c1: Color = colors[i];
                let c2: Color = colors[i+1];
                let dr_dx: f64 = ((c2.r as f64) - (c1.r as f64))/ milli_cycle as f64;
                let dg_dx: f64 = ((c2.g as f64) - (c1.g as f64))/ milli_cycle as f64;
                let db_dx: f64 = ((c2.b as f64) - (c1.b as f64))/ milli_cycle as f64;
        
                for t in 0..=milli_cycle{
                    let r: f64 = (c1.r as f64) + dr_dx*(t as f64);
                    let g: f64 = (c1.g as f64) + dg_dx*(t as f64);
                    let b: f64 = (c1.b as f64) + db_dx*(t as f64);
                    let c: Color = Color{r: r as u8, g: g as u8, b: b as u8};
                    let val: u32 = from_u8_rgb(c);
                    let buffer: Vec<u32> = vec![val; width * height];
                    window.update_with_buffer(&buffer, width, height).unwrap();
                    thread::sleep(time::Duration::from_millis(1));
                }
            }
            for i in (1..=colors.len()-1).rev(){
                let c1: Color = colors[i];
                let c2: Color = colors[i-1];
                let dr_dx: f64 = ((c2.r as f64) - (c1.r as f64))/ milli_cycle as f64;
                let dg_dx: f64 = ((c2.g as f64) - (c1.g as f64))/ milli_cycle as f64;
                let db_dx: f64 = ((c2.b as f64) - (c1.b as f64))/ milli_cycle as f64;
        
                for t in 0..=milli_cycle{
                    let r: f64 = (c1.r as f64) + dr_dx*(t as f64);
                    let g: f64 = (c1.g as f64) + dg_dx*(t as f64);
                    let b: f64 = (c1.b as f64) + db_dx*(t as f64);
                    let c: Color = Color{r: r as u8, g: g as u8, b: b as u8};
                    let val: u32 = from_u8_rgb(c);
                    let buffer: Vec<u32> = vec![val; width * height];
                    window.update_with_buffer(&buffer, width, height).unwrap();
                    thread::sleep(time::Duration::from_millis(1));
                }
            }
        }
    }

    fn draw_spiral(c1: Color, c2: Color, mut window: Window){
        // https://en.wikipedia.org/wiki/Logarithmic_spiral
        // x = a*e^(k*phi)*cos(phi)
        // y = a*e^(k*phi)*sin(phi)
        let buffer: Vec<u32> = vec![from_u8_rgb(c2); width * height];

        const a: usize = 3;
        let center_x: f64 =( width / 2) as f64;
        let center_y: f64 = (height / 2) as f64;
        todo!("spiral")      
    }



    fn draw_square(c1: Color, corner: Point, len: usize, buffer: &mut Vec<u32>){
        let mut idx = 0;
        for byte in buffer.iter_mut(){

            let x: usize = idx % width;
            let y: usize = idx / height;
            if (x >= corner.x && x <= corner.x + len) && (y >= corner.y && y <= corner.y + len){
                *byte = from_u8_rgb(c1);
            }
            idx += 1;
        }
    }
    fn draw_rect(rect: Rect, buffer: &mut Vec<u32>){     
        let mut idx = 0;
        for byte in buffer.iter_mut(){

            let x: usize = idx % width;
            let y: usize = idx / height;
            if (x >= rect.point.x && x <= rect.point.x + rect.xlen) && (y >= rect.point.y && y <= rect.point.y + rect.ylen){
                *byte = from_u8_rgb(rect.color);
            }
            idx += 1;
        }
    }

    #[derive(Clone, Copy)]
    struct Square{
        point: Point,
        length: usize,
        dx: isize,
        dy: isize,
        color: Color,
    }

    #[derive(Clone, Copy)]
    struct Rect{
        point: Point,
        xlen: usize,
        ylen: usize,
        dx: isize,
        dy: isize,
        color: Color,
    }
    ///
    /// c1: Color - Color of first square
    /// c2: Color - Color of second square
    /// milli_cylce: a cycle in milliseconds for the entire animation to complete
    /// winoww: Window - reference to full window
    /// Could we change this to a vector of square objects?
    fn crossing_squares(squares_p: Vec<Square>, mut window: Window){
        // const sq_len: usize = 100;
        if squares_p.len() == 0{
            let buffer: Vec<u32> = vec![from_u8_rgb(WHITE); width * height];
            window.update_with_buffer(&buffer, width, height).unwrap();
            return;
        }
        let mut buffer: Vec<u32> = vec![from_u8_rgb(WHITE); width * height];
        let mut squares: Vec<Square> = Vec::new();
        squares_p.clone_into(&mut squares);
        // let mut rng: ThreadRng = rand::thread_rng();
        
        while window.is_open(){
            
            //Draw squares on.
            buffer = vec![from_u8_rgb(WHITE); width * height];

            for square in squares.iter_mut(){
                draw_square(square.color, square.point, square.length, &mut buffer);
            }
            
            // Bounds protection and Change the derivatives
            // Change direction if it goes too far left
            for mut square in squares.iter_mut() {

                if square.point.x <= 0 || square.point.x >= width-square.length {
                    square.dx *= -1;
                }
    
                if square.point.y <= 0|| square.point.y >= height-square.length {
                    square.dy *= -1;
                }

                //Modify the place on screen
                square.point.x = ((square.point.x as isize) + square.dx) as usize;
                square.point.y = ((square.point.y as isize) + square.dy) as usize;

            }
            window.update_with_buffer(&buffer, width, height).unwrap();
            thread::sleep(time::Duration::from_millis(1));

            // Get the color combination
            // color it in
        }
    }

    struct Triangle{
        p1: Point,
        p2: Point,
        p3: Point,
        color: Color
    }
    fn draw_triangle(mut triangle: Triangle, mut window: Window){
        //order points by height
        if triangle.p1.y > triangle.p2.y{
            let temp = triangle.p1;
            triangle.p1 = triangle.p2;
            triangle.p2 = temp;
        }
        if triangle.p2.y > triangle.p3.y && triangle.p1.y > triangle.p3.y{
            let temp1 = triangle.p1;
            let temp2 = triangle.p2;
            triangle.p1 = triangle.p3;
            triangle.p2 = triangle.p1;
            triangle.p3 = triangle.p2;
        }
        else if triangle.p2.y > triangle.p3.y {
            let temp = triangle.p2;
            triangle.p2 = triangle.p3;
            triangle.p3 = temp;
        }
        let dx_dy12 = (triangle.p1.x + triangle.p2.x);


        let mut buffer: Vec<u32> = vec![from_u8_rgb(WHITE); width * height];
        let mut count: usize = 0;

        for byte in buffer.iter_mut(){
            let x: usize = count % width;
            let y: usize = count/width;
            
            count += 1;
        }


    }
    fn rotating_triangle(mut window: Window){
        let mut buffer: Vec<u32> = vec![from_u8_rgb(WHITE); width * height];
        let mut count = 0;

        for byte in buffer.iter_mut(){
            let x: usize = count % width;
            let y: usize = count/width;
            
            
            count += 1;
        }
    }
    let mut colors: Vec<Color> = Vec::new();
    colors.push(Color{r: 160, g: 32, b: 240});
    colors.push(Color{r: 0, g: 0, b: 255});
    colors.push(Color{r: 255, g: 192, b: 203});

    glowing_animation(colors, 500, window);

    //linear_gradient(Color{r: 160, g: 32, b: 240}, Color{r: 255, g: 192, b: 203}, window);
    //draw_square(Color{r: 160, g: 32, b: 240}, Point{x: 50, y: 50}, 100, window);
    // let mut squares: Vec<Square> = Vec::new();
    // let sq1: Square = Square { point: Point{x: 110, y: 100}, 
    //                            length: 400, 
    //                            dx: -3, 
    //                            dy: 4, 
    //                            color: Color{r: 255, g: 150, b: 10}
    //                         };
    // let sq2: Square = Square { point: Point{x: 600, y: 700}, 
    //                         length: 200, 
    //                         dx: -3, 
    //                         dy: -5, 
    //                         color: Color{r: 10, g: 150, b: 240}
    //                     };
    // let sq3: Square = Square { point: Point{x: 560, y: 500}, 
    //                 length: 130, 
    //                 dx: 5, 
    //                 dy: -2, 
    //                 color: Color{r: 153, g: 50, b: 204}//	rgb(153,50,204)
    //             };

    // squares.push(sq1);
    // squares.push(sq2);
    // squares.push(sq3);
    // crossing_squares(squares, window)

}
