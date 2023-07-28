use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Sprite, Text, Transformable, View, CircleShape, Drawable, RenderStates, Font};
use sfml::system::{Clock, SfStrConv, sleep, Time, Vector2f, Vector2i, Vector2u};
use sfml::window::{mouse, ContextSettings, Event, Key, Style};
use std::{thread, time};
use std::ops::{Add, Deref, Index, Mul};
use sfml::graphics::ShaderType::Vertex;
use tiny_skia::Point;

pub(crate) fn main2() {
    let mut window = RenderWindow::new(
        (1024, 768),
        "Custom drawable",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    let mut ball = Ball::new();
    let font = Font::from_file("/usr/share/fonts/truetype/ubuntu/UbuntuMono-R.ttf").unwrap();
    let mut position_info = PositionText::new(&font);

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                Event::MouseMoved { x, y } => {
                    ball.set_position(Vector2i::new(x, y));
                    //position_info.set_position(ball.position);
                },
                Event::Resized { width, height } => {
                    position_info.set_position(Vector2i::new(width as i32, height as i32));
                },
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        window.draw(&ball);
        window.draw(&position_info);
        window.display()
    }
}


struct Ball<'s> {
    //renderWindow: &'s RenderWindow,
    circle: CircleShape<'s>,
    size: i32,
    position: Vector2i,
    speed: Vector2i,
}

impl<'s> Ball<'s> {
    fn new() -> Self {
        let radius = 10 as u8;
        let mut circle = CircleShape::new(radius as f32, 50);
        circle.set_position(Vector2f::new(0f32, 0f32));
        Self {
            //      renderWindow: renderWindow,
            circle: circle,
            size: 10,
            position: Vector2i::new(0, 0),
            speed: Vector2i::new(0, 0),
        }
    }
    fn set_position(&mut self, new_position: Vector2i) {
        self.position = new_position;
        self.size = new_position.x % 200;
        self.circle.set_radius(self.size as f32);
        //let y_position = self.renderWindow.size().y as i32 - self.position.y;
        let y_position = self.position.y;
        self.circle.set_position(Vector2f::new(self.position.x as f32 - self.size as f32, y_position as f32 - self.size as f32));
    }
}

impl<'s> Drawable for Ball<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(&'a self, target: &mut dyn RenderTarget, states: &RenderStates<'texture, 'shader, 'shader_texture>) {
        target.draw(&self.circle);
    }
}

struct PositionText<'s> {
    text: Text<'s>
}

impl <'s> PositionText<'s> {
    fn new(font: &'s Font) -> Self {
        Self {
            text: Text::new("0,0", font, 20)
        }
    }
    fn set_position(&mut self, position: Vector2i) {
        self.text.set_string(&format!("{},{}", position.x, position.y))
    }
}

impl<'s> Drawable for PositionText<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(&'a self, target: &mut dyn RenderTarget, states: &RenderStates<'texture, 'shader, 'shader_texture>) {
        target.draw(&self.text);
    }
}
