use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

pub fn serpinski_mail_chaos_theory(canvas: &mut WindowCanvas) {
    let mut points = [
        Point::new(100, 700),
        Point::new(400, 10),
        Point::new(700, 700),
    ];

    for &point in &points {
        canvas.draw_point(point).unwrap();
    }
    let mut rng = rand::thread_rng();

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    let mut new_vertex = points[0];
    for _ in 1..100000 {
        let random_number: usize = rng.gen_range(0..=2);
        let random_vertex = points[random_number];
        new_vertex = Point::new(
            (random_vertex.x + new_vertex.x) / 2,
            (random_vertex.y + new_vertex.y) / 2,
        );

        canvas.draw_point(new_vertex).unwrap();
    }
}
