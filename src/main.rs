use sfml::graphics::{
    Color, RectangleShape, RenderTarget, RenderWindow, Shape, Sprite, Text, Transformable, View, CircleShape
};
use sfml::system::{Clock, Vector2f};
use sfml::window::{mouse, ContextSettings, Event, Key, Style};
use std::{thread, time};
use sfml::graphics::ShaderType::Vertex;

struct Dot<'a> {
    shape:  CircleShape<'a>,
    position: Vector2f,
    size: i32,
    color: Color
}
impl <'a>Dot<'a> {

    fn new(position: Vector2f, color: Color) -> Dot<'a> {
        let size = 30;
        let mut d = Dot {
            position: position,
            size: size,
            color: color,
            shape:  CircleShape::new(size as f32, 100),
        };
        d.shape.set_fill_color(color);
        d.shape.set_position(position);
        return d;
    }
    fn setPos(&mut self, x: i32, y: i32) {
        self.shape.set_position(Vector2f::new((x - self.size) as f32, (y - self.size) as f32));
    }
    fn draw(&self, window: &mut RenderWindow) {
        window.draw(&self.shape)
    }
}



fn main() {
    // Create the window of the application
    let mut window = RenderWindow::new(
        (3840, 2160),
        "Graphics",
        Style::FULLSCREEN,
        &ContextSettings::default(),
    );
    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);

    let mut dot = Dot::new(Vector2f::new(0 as f32, 0 as f32), Color::WHITE);

    while window.is_open() {

        while let Some(event) = window.poll_event() {
            match(event) {
                Event::KeyPressed { code, .. }=> {
                    if Key::Escape == code {
                        window.close()
                    }
                }
                Event::MouseMoved { x, y} => {
                    dot.setPos(x, y);
                }
                Event::Closed => {
                    window.close()
                }
                _ => { println!("Event") }
            }
        }

        window.clear(Color::BLACK);
        dot.draw(&mut window);
        window.display()
    }


}