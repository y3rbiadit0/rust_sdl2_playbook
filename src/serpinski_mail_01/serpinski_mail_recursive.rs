extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::video::Window;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub fn serpinski_mail_recursive(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::RGB(255, 0, 0)); // Set draw color to white

    let depth = 9; // Recursion depth for the Sierpinski triangle
    let top = Point::new(WIDTH as i32 / 2, 50);
    let left = Point::new(50, HEIGHT as i32 - 50);
    let right = Point::new(WIDTH as i32 - 50, HEIGHT as i32 - 50);

    draw_triangle(canvas, depth, top, left, right);
}

fn draw_triangle(canvas: &mut Canvas<Window>, depth: u32, top: Point, left: Point, right: Point) {
    if depth == 0 {
        // Draw the triangle when the recursion depth is 0
        canvas.draw_line(top, left).unwrap();
        canvas.draw_line(left, right).unwrap();
        canvas.draw_line(right, top).unwrap();
    } else {
        // Calculate midpoints of the sides of the triangle
        let mid_top_left = Point::new((top.x + left.x) / 2, (top.y + left.y) / 2);
        let mid_top_right = Point::new((top.x + right.x) / 2, (top.y + right.y) / 2);
        let mid_left_right = Point::new((left.x + right.x) / 2, (left.y + right.y) / 2);

        // Recursive calls for the three smaller triangles
        draw_triangle(canvas, depth - 1, top, mid_top_left, mid_top_right);
        draw_triangle(canvas, depth - 1, mid_top_left, left, mid_left_right);
        draw_triangle(canvas, depth - 1, mid_top_right, mid_left_right, right);
    }
}
