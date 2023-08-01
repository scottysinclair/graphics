use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Sprite, Text, Transformable, View, CircleShape, Drawable, RenderStates, Font};
use sfml::system::{Clock, SfStrConv, sleep, Time, Vector2f, Vector2i, Vector2u};
use sfml::window::{mouse, ContextSettings, Event, Key, Style};
use std::{thread, time};
use std::cell::RefCell;
use std::ops::{Add, Deref, Index, Mul};
use std::rc::{Rc, Weak};
use sfml::graphics::ShaderType::Vertex;
use tiny_skia::Point;

pub(crate) fn main2() {
    let mut world =World::new();
    let mut screen = Screen::new();

    let mut ball = Ball::new();
    world.add(ball);

    let font = Font::from_file("/usr/share/fonts/truetype/ubuntu/UbuntuMono-R.ttf").unwrap();
    let mut position_info = PositionText::new(&font);


    loop {
        while let Some(event) = screen.renderWindow.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                Event::MouseMoved { x, y } => {
                    world.things.first_mut().unwrap().set_position(Vector2f::new(x as f32, y as f32));
                    position_info.set_position(Vector2i::new(x, y));
//                    position_info.set_position(Vector2i::new(window.size().x as i32, window.size().y as i32));
                },
                Event::Resized { width, height } => {
//                    position_info.set_position(Vector2i::new(width as i32, height as i32));
                },
                _ => {}
            }
        }

        screen.clear(Color::BLACK);
        screen.draw(&world);
        screen.display()
    }
}

struct Screen {
    position: Vector2f,
    renderWindow: RenderWindow
}

struct World<'s> {
    things: Vec<Ball<'s>>
}

trait Thing {
    fn getMass(&self) -> i32;
    fn getPosition(&self) -> Vector2f;
    fn setPosition(&mut self, position: Vector2f);
    fn getSpeed(&self) -> Vector2f;
    fn setSpeed(&mut self, speed: Vector2f);
}

impl Screen {
    fn new() -> Screen {
        let mut s = Screen {
            position: Vector2f::new(0., 0.),
            renderWindow: RenderWindow::new(
                (3840, 2400),
                "Graphics",
                Style::FULLSCREEN,
                &ContextSettings::default(),
            )
        };
        s.renderWindow.set_position(Vector2i::new(2000, 1000));
        s.renderWindow.set_framerate_limit(60);
        s.renderWindow.set_vertical_sync_enabled(true);
        s
    }
    fn clear(&mut self, color: Color) {
        self.renderWindow.clear(color)
    }
    fn draw(&mut self, world: &World) {
        world.things.iter().for_each(|t| self.renderWindow.draw(t));
    }
    fn display(&mut self) {
        self.renderWindow.display()
    }
}

impl<'s> World<'s> {
  fn new() -> World<'s> {
      World {
          things: Vec::new()
      }
  }
  fn add(&mut self, ball: Ball<'s>) {
      self.things.push(ball);
  }
}

struct Ball<'s> {
    //renderWindow: &'s RenderWindow,
    circle: CircleShape<'s>,
    size: i32,
    mass: i32,
    position: Vector2f,
    speed: Vector2f,
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
            mass: 1,
            position: Vector2f::new(0., 0.),
            speed: Vector2f::new(0., 0.),
        }
    }
    fn set_position(&mut self, new_position: Vector2f) {
        self.position = new_position;
        self.size = new_position.x as i32 % 200;
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

impl<'s> Thing for Ball<'s> {
    fn getMass(&self) -> i32 {
        self.mass
    }
    fn getPosition(&self) -> Vector2f {
        self.position
    }
    fn setPosition(&mut self, _: Vector2f) {
        todo!()
    }
    fn getSpeed(&self) -> Vector2f {
        self.speed
    }
    fn setSpeed(&mut self, _: Vector2f) {
        todo!()
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