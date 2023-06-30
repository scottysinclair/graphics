use sfml::graphics::{
    Color, RectangleShape, RenderTarget, RenderWindow, Shape, Sprite, Text, Transformable, View, CircleShape
};
use sfml::system::{Clock, Vector2f, Vector2i};
use sfml::window::{mouse, ContextSettings, Event, Key, Style};
use std::{thread, time};
use std::ops::Index;
use sfml::graphics::ShaderType::Vertex;

struct Dot<'a> {
    shape:  CircleShape<'a>,
    position: Vector2i,
    size: i32,
    colorIndex: usize
}
impl <'a>Dot<'a> {

    fn new(x : i32, y: i32, size: i32, colorIndex: usize) -> Dot<'a> {
        let mut d = Dot {
            position: Vector2i::new(x,y),
            size: size,
            colorIndex: colorIndex,
            shape:  CircleShape::new(size as f32, 100),
        };
        return d;
    }
    fn changeSize(&mut self, delta: i32) {
        self.size += delta;
    }
    fn setPos(&mut self, x: i32, y: i32) {
        self.position = Vector2i::new(x, y);
    }
    fn draw(&mut self, window: &mut RenderWindow, colors: &Vec<Color>) {
        self.colorIndex = self.colorIndex % colors.len();
        self.shape.set_fill_color(colors[self.colorIndex]);
        self.shape.set_radius(self.size as f32);
        self.shape.set_position(Vector2f::new((self.position.x - self.size) as f32,  (self.position.y - self.size) as f32));
        window.draw(&self.shape)
    }
}



fn main() {

    let mut colors = Vec::new();
    colors.push(Color::WHITE);
    colors.push(Color::RED);
    colors.push(Color::GREEN);
    colors.push(Color::BLUE);
    colors.push(Color::YELLOW);
    colors.push(Color::MAGENTA);
    colors.push(Color::CYAN);
    let mut currentColor = 0;

    // Create the window of the application
    let mut window = RenderWindow::new(
        (3840, 2160),
        "Graphics",
        Style::FULLSCREEN,
        &ContextSettings::default(),
    );
    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);

    let mut dots = Vec::new();
    dots.push(Dot::new(0, 0, 32, currentColor));

    let mut mousePressed = false;

    while window.is_open() {

        while let Some(event) = window.poll_event() {
            match(event) {
                Event::KeyPressed { code, .. }=> {
                    if Key::Escape == code {
                        window.close()
                    }
                    else if (Key::Space == code) {
                        let c=  colors.pop().unwrap();
                        colors.insert(0, c);
                    }
                }
                Event::MouseMoved { x, y} => {
                    let mouseDot = dots.first_mut().unwrap();
                    mouseDot.setPos(x, y);
                    if (mousePressed) {
                        let size = mouseDot.size;
                        dots.push(Dot::new(x, y, size, currentColor));
                        currentColor = (currentColor + 1) % colors.len();
                    }
                }
                Event::MouseButtonPressed { x, y, .. } => {
                    mousePressed = true;
                }
                Event::MouseButtonReleased { .. } => {
                    mousePressed = false
                }
                Event::MouseWheelScrolled { delta, ..} => {
                    dots.first_mut().unwrap().changeSize(delta as i32);
                    //dots.iter_mut().for_each(|d| d.changeSize(delta as i32));
                }
                Event::Closed => {
                    window.close()
                }
                _ => { println!("Event") }
            }
        }

        window.clear(Color::BLACK);
        dots.iter_mut().for_each(|d| d.draw(&mut window, &colors));
        window.display()
    }


}