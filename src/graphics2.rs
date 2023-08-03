use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Sprite, Text, Transformable, View, CircleShape, Drawable, RenderStates, Font};
use sfml::system::{Clock, SfStrConv, sleep, Time, Vector2, Vector2f, Vector2i, Vector2u};
use sfml::window::{mouse, ContextSettings, Event, Key, Style};
use std::{thread, time};
use std::cell::RefCell;
use std::ops::{Add, Deref, Index, Mul};
use std::rc::{Rc, Weak};
use sfml::graphics::ShaderType::Vertex;
use tiny_skia::Point;
use rand::Rng;

pub(crate) fn main2() {
    let mut world = World::new();
    let mut screen = Screen::new(3.);
    let mut clock = Clock::start();
    let mut rng = rand::thread_rng();

    let mut physics = Physics::new();
    let mut mouse_pressed = false;
    let mut lastMouseWorldPos = Vector2f::new(-1., -1.);
    let mut followBall = -1i32;

    let font = Font::from_file("/usr/share/fonts/truetype/ubuntu/UbuntuMono-R.ttf").unwrap();
    let mut position_info = PositionText::new(&font);
    position_info.set_position(Vector2f::new(screen.scale, 0.));

    fn add_ball(screen: &Screen, mass: i32, world: &mut World, x: i32, y: i32, initial_speed: Vector2f) {
        let world_coords = screen.translate_to_world_coords(Vector2f::new(x as f32, y as f32));
        world.add(Ball::new(world_coords, mass as f32, initial_speed) );
    }

    loop {
        while let Some(event) = screen.renderWindow.poll_event() {
            match event {
                Event::Closed  | Event::KeyPressed { code: Key::Escape, .. } => {
                    if (followBall >= 0) {
                        screen.position = Vector2f::new(0., 0.);
                        followBall = -1;
                    }
                    else {
                        return
                    }
                },
                Event::KeyPressed { code, .. } => {
                    if Key::Up == code {
                        screen.position.y += screen.scale * 1.;
                        position_info.set_position(screen.position);
                        lastMouseWorldPos = screen.translate_to_world_coords(lastMouseWorldPos);
                    }
                    else if Key::Down == code {
                        screen.position.y -= screen.scale * 1.;
                        position_info.set_position(screen.position);
                        lastMouseWorldPos = screen.translate_to_world_coords(lastMouseWorldPos);
                    }
                    if Key::Left == code {
                        screen.position.x -= screen.scale * 1.;
                        position_info.set_position(screen.position);
                        lastMouseWorldPos = screen.translate_to_world_coords(lastMouseWorldPos);
                    }
                    else if Key::Right == code {
                        screen.position.x += screen.scale * 1.;
                        position_info.set_position(screen.position);
                        lastMouseWorldPos = screen.translate_to_world_coords(lastMouseWorldPos);
                    }
                    else if Key::F == code {
                        followBall = world.things.len() as i32 - 1;
                    }
                },
                Event::MouseButtonPressed { x, y, .. } => {
                    mouse_pressed = true;
                    let new_world_coords = screen.translate_to_world_coords(Vector2f::new(x as f32, y as f32));
                    let inital_speed =  new_world_coords - lastMouseWorldPos;
                    add_ball(&screen, rng.gen_range(10..40), &mut world, x, y, inital_speed)
                }
                Event::MouseButtonReleased { .. } => {
                    mouse_pressed = false
                }
                Event::Resized { width, height } => {
//                    position_info.set_position(Vector2i::new(width as i32, height as i32));
                }
                Event::MouseWheelScrolled { delta, .. } => {
                    screen.scale += (delta * 0.1);
                    position_info.set_position(Vector2f::new(screen.scale, 0.))
                }
                Event::MouseMoved { x, y } => {
                    if (mouse_pressed) {
                        let new_world_coords = screen.translate_to_world_coords(Vector2f::new(x as f32, y as f32));
                        let inital_speed =  new_world_coords - lastMouseWorldPos;
                        add_ball(&screen, rng.gen_range(10..40), &mut world, x, y, inital_speed);
                    }
                    let world_coords = screen.translate_to_world_coords( Vector2f::new(x as f32, y as f32) );
                    position_info.set_position(world_coords);
                    lastMouseWorldPos = world_coords;
                }
                    _ => {}
            }
        }
        let elapsedTime : Time = clock.elapsed_time();
        clock.restart();
        physics.calculate(&mut world, elapsedTime);

        if (followBall >= 0) {
            let pos = world.things.get(followBall as usize).unwrap().position;
            /*
             * get the width of the screen in world coords
             */
            let worldWidth = screen.renderWindow.size().x as f32 * screen.scale;
            let worldHeight = screen.renderWindow.size().y as f32 * screen.scale;
            screen.position = Vector2f::new(pos.x - (worldWidth / 2.), pos.y - (worldHeight / 2.));
            position_info.set_position(screen.position);
            lastMouseWorldPos = screen.translate_to_world_coords(lastMouseWorldPos);
        }

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
            accel_due_to_gravity: -9.8 * 3.
        }
    }
    fn calculate(&self, world: &mut World, elapsedTime: Time) {
        world.things.iter_mut().for_each(|thing : &mut Ball| {
            let forces = self.calculate_forces_on(&thing);
            let totalForce = forces.iter().fold(Vector2f::new(0., 0.), |a, b| { a.add(*b) });
            let accel = totalForce / thing.mass as f32;
            thing.speed += accel * elapsedTime.as_seconds();
            thing.set_position(thing.get_position() + thing.speed);
            if (thing.position.y <= 10.) {
                let normal = Vector2f::new(0., -1.);
                let dot_product = (thing.speed.x * 0.) + (thing.speed.y * 1.);
                thing.speed.x += ((2. * normal.x * dot_product) * thing.get_bounciness());
                thing.speed.y += ((2. * normal.y * dot_product) * thing.get_bounciness());
            }
        });
    }

    fn calculate_forces_on(&self, thing: &Ball) -> Vec<Vector2f>{
        let mut forces = Vec::new();
        forces.push(Vector2f::new(0., self.accel_due_to_gravity * thing.mass));
        //forces.push(Vector2f::new(800., 0.));
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
        /*
         * translate the screen cooards to the screen at world position 0, then scale
         *  40s, 20s, scale 2 ->  screen (40 * 2) + 20 = 100w
         *  60s, 20s, scale 2 -> screen (60 * 2) + -20 = 100w
         */
        let world_x = screen_coards.x * self.scale + self.position.x;
        /*
         * translate the screen cooards to the screen at world position 0, then scale
         *  40s, 20s, scale 2 ->  screen (40 * 2) + 20 = 100w
         *  60s, 20s, scale 2 -> screen (60 * 2) + -20 = 100w
         */
        let screen_y = self.renderWindow.size().y as f32 - screen_coards.y;
        let world_y = screen_y * self.scale + self.position.y;
        Vector2f::new(world_x, world_y)
    }

    pub(crate) fn translate_to_screen_coords(&self, world_coords: Vector2f) -> Vector2f {
        /*
         * translate the world coords to the position of the screen, then scale
         * 100w, 20s, scale 2 -> (100 - 20) / 2 = screen 40
         * 100w, -20s, scale 2 -> (100 - -20) /  2 = screen 60
         */
        let screen_x = (world_coords.x - self.position.x) / self.scale;
        /*
         * translate the world coords to the position of the screen, then scale
         * 100w, 20s,  scale 2 -> 100 - 20 / 2 = screen 40
         * 100w, -20s, scale 2 -> 100 - -20 / 2 = screen 60
         */
        let mut screen_y = (world_coords.y - self.position.y) / self.scale;
        /*
         * the screen has inverted coords with 0 as top, so fix this
         */
        screen_y = self.renderWindow.size().y as f32 - screen_y;
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
    fn get_bounciness(&self) -> f32;
    fn draw_on_screen(&mut self, screen: &mut Screen);
}

impl Screen {
    fn new(scale: f32) -> Screen {
        let mut s = Screen {
            position: Vector2f::new(0., 0.),
            scale: scale,
            renderWindow: RenderWindow::new(
                (3840, 2400),
       //         (1024, 768),
                "Graphics",
                Style::FULLSCREEN, //Style::CLOSE,
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
    mass: f32,
    position: Vector2f,
    speed: Vector2f,
    bounciness: f32
}


impl<'s> Ball<'s> {
    fn new(position: Vector2f, mass: f32, initial_speed: Vector2f) -> Self {
        let radius = 10 as u8;
        let mut circle = CircleShape::new(radius as f32, 50);
        circle.set_position(Vector2f::new(0f32, 0f32));
        Self {
            //      renderWindow: renderWindow,
            circle: circle,
            mass: mass,
            position: position,
            speed: initial_speed,
            bounciness: 0.98
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
    }
    fn get_speed(&self) -> Vector2f {
        self.speed
    }
    fn set_speed(&mut self, _: Vector2f) {
    }
    fn get_bounciness(&self) -> f32 {
        self.bounciness
    }
    fn draw_on_screen(&mut self, screen: &mut Screen) {
        //let y_position = self.renderWindow.size().y as i32 - self.position.y;
        let screen_coords = screen.translate_to_screen_coords(self.position);
        let radius_on_screen = self.mass as f32 / screen.scale;
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
