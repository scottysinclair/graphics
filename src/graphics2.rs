use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Sprite, Text, Transformable, View, CircleShape, Drawable, RenderStates, Font};
use sfml::system::{Clock, SfStrConv, sleep, Time, Vector2, Vector2f, Vector2i, Vector2u};
use sfml::window::{mouse, ContextSettings, Event, Key, Style};
use std::{thread, time};
use std::cell::RefCell;
use std::ops::{Add, Deref, Index, Mul};
use std::rc::{Rc, Weak};
use sfml::graphics::ShaderType::Vertex;
use tiny_skia::Point;

pub(crate) fn main2() {
    let mut world = World::new();
    let mut screen = Screen::new();
    let mut clock = Clock::start();

    let mut physics = Physics::new();

    let font = Font::from_file("/usr/share/fonts/truetype/ubuntu/UbuntuMono-R.ttf").unwrap();
    let mut position_info = PositionText::new(&font);
    position_info.set_position(Vector2f::new(screen.scale, 0.));

    loop {
        while let Some(event) = screen.renderWindow.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                Event::MouseButtonPressed { x, y, .. } => {
                    let worldCoords = screen.translate_to_world_coords(Vector2f::new(x as f32, y as f32));
                    world.add(Ball::new(screen.translate_to_world_coords(Vector2f::new(x as f32, y as f32))) );
                }
                Event::Resized { width, height } => {
//                    position_info.set_position(Vector2i::new(width as i32, height as i32));
                }
                Event::MouseWheelScrolled { delta, .. } => {
                    screen.scale += (delta * 0.1);
                    position_info.set_position(Vector2f::new(screen.scale, 0.))
                }
                    _ => {}
            }
        }
        let elapsedTime : Time = clock.elapsed_time();
        clock.restart();
        physics.calculate(&mut world, elapsedTime);

        screen.clear(Color::BLACK);
        screen.drawWorld(&mut world);
        screen.drawDirect(&position_info);
        screen.display()
    }
}

struct Physics {
    accel_due_to_gravity: f32
}

impl Physics {
    fn new() -> Physics {
        Physics {
            accel_due_to_gravity: -9.8 * 3.4
        }
    }
    fn calculate(&self, world: &mut World, elapsedTime: Time) {
        world.things.iter_mut().for_each(|thing : &mut Ball| {
            let forces = self.calculate_forces_on(&thing);
            let totalForce = forces.iter().fold(Vector2f::new(0., 0.), |a, b| { a.add(*b) });
            let accel = totalForce.mul(thing.mass as f32);
            thing.speed += accel * elapsedTime.as_seconds();
            thing.set_position(thing.get_position() + thing.speed);
        });
    }

    fn calculate_forces_on(&self, thing: &Ball) -> Vec<Vector2f>{
        let mut forces = Vec::new();
        forces.push(Vector2f::new(0., self.accel_due_to_gravity / thing.mass));
        return forces;
    }
}


struct Screen {
    position: Vector2f,
    scale: f32,
    renderWindow: RenderWindow,
}

impl Screen {
    pub(crate) fn translate_to_world_coords(&self, screen_coards: Vector2f) -> Vector2f {
        let world_x = screen_coards.x * self.scale;
        let world_y = (self.renderWindow.size().y as f32 - screen_coards.y) * self.scale;
        Vector2f::new(world_x, world_y)
    }

    pub(crate) fn translate_to_screen_coords(&self, world_coords: Vector2f) -> Vector2f {
        let screen_x = world_coords.x / self.scale;
        let screen_y = self.renderWindow.size().y as f32 - (world_coords.y / self.scale);
        Vector2f::new(screen_x, screen_y)
    }
}

struct World<'s> {
    things: Vec<Ball<'s>>,
}

trait Thing: Drawable {
    fn get_mass(&self) -> f32;
    fn get_position(&self) -> Vector2f;
    fn set_position(&mut self, position: Vector2f);
    fn get_speed(&self) -> Vector2f;
    fn set_speed(&mut self, speed: Vector2f);
    fn draw_on_screen(&mut self, screen: &mut Screen);
}

impl Screen {
    fn new() -> Screen {
        let mut s = Screen {
            position: Vector2f::new(0., 0.),
            scale: 1.,
            renderWindow: RenderWindow::new(
                //(3840, 2400),
                (1024, 768),
                "Graphics",
                Style::CLOSE,
                &ContextSettings::default(),
            ),
        };
        s.renderWindow.set_position(Vector2i::new(500, 500));
        s.renderWindow.set_framerate_limit(60);
        s.renderWindow.set_vertical_sync_enabled(true);
        s
    }
    fn clear(&mut self, color: Color) {
        self.renderWindow.clear(color)
    }
    fn drawWorld(&mut self, world: &mut World) {
        world.draw(self);
    }
    fn drawDirect(&mut self, thing: &dyn Drawable) {
        self.renderWindow.draw(thing);
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
    fn draw(&mut self, screen: &mut Screen) {
        self.things.iter_mut().for_each(|t| t.draw_on_screen(screen));
    }
}


impl<'s> Drawable for World<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(&'a self, target: &mut dyn RenderTarget, states: &RenderStates<'texture, 'shader, 'shader_texture>) {
        self.things.iter().for_each(|t| target.draw(t));
    }
}

struct Ball<'s> {
    //renderWindow: &'s RenderWindow,
    circle: CircleShape<'s>,
    size: i32,
    mass: f32,
    position: Vector2f,
    speed: Vector2f,
}


impl<'s> Ball<'s> {
    fn new(position: Vector2f) -> Self {
        let radius = 10 as u8;
        let mut circle = CircleShape::new(radius as f32, 50);
        circle.set_position(Vector2f::new(0f32, 0f32));
        Self {
            //      renderWindow: renderWindow,
            circle: circle,
            size: 10,
            mass: 1.,
            position: position,
            speed: Vector2f::new(0., 0.),
        }
    }
}

impl<'s> Drawable for Ball<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(&'a self, target: &mut dyn RenderTarget, states: &RenderStates<'texture, 'shader, 'shader_texture>) {
        target.draw(&self.circle);
    }
}

impl<'s> Thing for Ball<'s> {
    fn get_mass(&self) -> f32 {
        self.mass
    }
    fn get_position(&self) -> Vector2f {
        self.position
    }
    fn set_position(&mut self, new_position: Vector2f) {
        self.position = new_position;
        self.size = 30;
        self.circle.set_radius(self.size as f32);
    }
    fn get_speed(&self) -> Vector2f {
        self.speed
    }
    fn set_speed(&mut self, _: Vector2f) {
        todo!()
    }
    fn draw_on_screen(&mut self, screen: &mut Screen) {
        //let y_position = self.renderWindow.size().y as i32 - self.position.y;
        let screen_coords = screen.translate_to_screen_coords(self.position);
        let radius_on_screen = self.size as f32 / screen.scale;
        self.circle.set_radius(radius_on_screen);
        self.circle.set_position(Vector2f::new(screen_coords.x - radius_on_screen, screen_coords.y - radius_on_screen));
        screen.drawDirect(self)
    }
}

struct PositionText<'s> {
    text: Text<'s>,
}

impl<'s> PositionText<'s> {
    fn new(font: &'s Font) -> Self {
        Self {
            text: Text::new("0,0", font, 20)
        }
    }
    fn set_position(&mut self, position: Vector2f) {
        self.text.set_string(&format!("{},{}", position.x, position.y))
    }
}

impl<'s> Drawable for PositionText<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(&'a self, target: &mut dyn RenderTarget, states: &RenderStates<'texture, 'shader, 'shader_texture>) {
        target.draw(&self.text);
    }
}
