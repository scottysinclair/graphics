use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Sprite, Text, Transformable, View, CircleShape, Drawable, RenderStates, Font, VertexBuffer, PrimitiveType, VertexBufferUsage, Vertex};
use sfml::system::{Clock, SfStrConv, sleep, Time, Vector2, Vector2f, Vector2i, Vector2u};
use sfml::window::{mouse, ContextSettings, Event, Key, Style};
use std::{thread, time};
use std::cell::RefCell;
use std::cmp::max;
use std::ops::{Add, Deref, Index, Mul};
use std::rc::{Rc, Weak};
use tiny_skia::{PathBuilder, Point, Stroke};
use rand::Rng;
use tiny_skia::PathSegment::LineTo;
use crate::graphics2::core::{Screen, Thing, World};

pub mod core;

pub(crate) fn graphics_program_2() {
    let background_color = Color::rgb(167,183,255); // Color::rgb(91, 134, 171);
    let grid_color = Color::rgb(130, 130, 130);
    let ball_color = Color::rgb(255, 253, 197);

    let grid_size = 2000;

    let mut world = World::new();
    let mut screen = Screen::new(3.);
    let mut clock = Clock::start();
    let mut rng = rand::thread_rng();

    let mut physics = Physics::new(grid_size);
    let mut grid = Grid::new(grid_size as f32, grid_color, &screen);


    let mut mouse_pressed = false;
    let mut lastMouseWorldPos = Vector2f::new(-1., -1.);
    let mut followBall = -1i32;
    let screen_move_speed = 5.;
    let screen_zoom_speed = 1.;

    let font = Font::from_file("/usr/share/fonts/truetype/ubuntu/UbuntuMono-R.ttf").unwrap();
    let mut position_info = PositionText::new(&font);
    position_info.set_position(Vector2f::new(screen.scale, 0.));

    fn add_ball(screen: &Screen, mass: i32, world: &mut World, x: i32, y: i32, initial_speed: Vector2f, color: Color) {
        let world_coords = screen.translate_to_world_coords(Vector2f::new(x as f32, y as f32));
        world.add(Box::new(Ball::new(world_coords, mass as f32, initial_speed, color) ));
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
                Event::KeyPressed { code, shift, .. } => {
                    let myscreen_speed = if shift  { screen_move_speed * 4. } else { screen_move_speed };
                    if Key::Up == code {
                        screen.position.y += screen.scale * myscreen_speed;
                        position_info.set_position(screen.position);
                        lastMouseWorldPos = screen.translate_to_world_coords(lastMouseWorldPos);
                    }
                    else if Key::Down == code {
                        screen.position.y -= screen.scale * myscreen_speed;
                        position_info.set_position(screen.position);
                        lastMouseWorldPos = screen.translate_to_world_coords(lastMouseWorldPos);
                    }
                    if Key::Left == code {
                        screen.position.x -= screen.scale * myscreen_speed;
                        position_info.set_position(screen.position);
                        lastMouseWorldPos = screen.translate_to_world_coords(lastMouseWorldPos);
                    }
                    else if Key::Right == code {
                        screen.position.x += screen.scale * myscreen_speed;
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
                    add_ball(&screen, rng.gen_range(10..40), &mut world, x, y, inital_speed, ball_color)
                }
                Event::MouseButtonReleased { .. } => {
                    mouse_pressed = false
                }
                Event::Resized { width, height } => {
//                    position_info.set_position(Vector2i::new(width as i32, height as i32));
                }
                Event::MouseWheelScrolled { delta, .. } => {
                    let old_center = screen.translate_to_world_coords(Vector2f::new(screen.renderWindow.size().x as f32 / 2., screen.renderWindow.size().y as f32 / 2.));
                    screen.scale += (delta * screen_zoom_speed);
                    let new_center = screen.translate_to_world_coords(Vector2f::new(screen.renderWindow.size().x as f32 / 2., screen.renderWindow.size().y as f32 / 2.));
                    screen.position.x = old_center.x - (new_center.x - screen.position.x);
                    screen.position.y = old_center.y - (new_center.y - screen.position.y);
                    //position_info.set_position(Vector2f::new(screen.scale, 0.))
                    position_info.set_position(old_center)
                }
                Event::MouseMoved { x, y } => {
                    if (mouse_pressed) {
                        let new_world_coords = screen.translate_to_world_coords(Vector2f::new(x as f32, y as f32));
                        let inital_speed =  new_world_coords - lastMouseWorldPos;
                        add_ball(&screen, rng.gen_range(10..40), &mut world, x, y, inital_speed, ball_color);
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
            let pos = world.things.get(followBall as usize).unwrap().get_position();
            /*
             * get the width of the screen in world coords
             */
            let worldWidth = screen.renderWindow.size().x as f32 * screen.scale;
            let worldHeight = screen.renderWindow.size().y as f32 * screen.scale;
            screen.position = Vector2f::new(pos.x - (worldWidth / 2.), pos.y - (worldHeight / 2.));
            position_info.set_position(screen.position);
            lastMouseWorldPos = screen.translate_to_world_coords(lastMouseWorldPos);
        }

        screen.clear(background_color );
        screen.draw_world(&mut world);
        screen.draw_direct(&position_info);

        //draw_line(&mut screen);

        grid.draw_on_screen(&mut screen);

        screen.display()
    }
}

fn draw_line(screen: &mut Screen) {
    let mut vertex_buffer =
        VertexBuffer::new(PrimitiveType::LINES, 6, VertexBufferUsage::STATIC);

    let vertices = [
        Vertex::with_pos_color((200.0, 300.0).into(), Color::GREEN),
        Vertex::with_pos_color((2000.0, 300.0).into(), Color::GREEN),
    ];
    vertex_buffer.update(&vertices, 0);
    screen.draw_direct(&vertex_buffer);
}



struct Physics {
    accel_due_to_gravity: f32,
    grid_size: i32,
}

impl Physics {
    fn new(grid_size: i32) -> Physics {
        Physics {
            accel_due_to_gravity: -9.8 * 3.,
            grid_size: grid_size
        }
    }
    fn calculate(&self, world: &mut World, elapsedTime: Time) {
        let grid_tolerance = 5;
        let mut new_balls = Vec::new();
        world.things.iter_mut().for_each(|t | {
            let thing = t.as_mut();
            let xdiff = thing.get_position().x as i32 % self.grid_size;
            let ydiff = thing.get_position().y as i32 % self.grid_size;
            let ensure_the_bounce = thing.get_position().y < grid_tolerance as f32 * 2.;
            let not_on_a_grid_coord = xdiff > grid_tolerance && xdiff < (self.grid_size - grid_tolerance) &&  (ydiff > grid_tolerance && ydiff < (self.grid_size - grid_tolerance));
            let outside_the_grid = thing.get_position().x < 0. || thing.get_position().y < 0.;

            if (ensure_the_bounce || not_on_a_grid_coord || outside_the_grid) {
                let forces = self.calculate_forces_on(thing);
                let totalForce = forces.iter().fold(Vector2f::new(0., 0.), |a, b| { a.add(*b) });
                let accel = totalForce / thing.get_mass() as f32;
                thing.set_speed(thing.get_speed() + accel * elapsedTime.as_seconds());
                let new_pos = thing.get_position() + thing.get_speed();
                if (thing.get_position().x >= 0. && new_pos.y < 0. && thing.get_position().y >= 0.) {
                    let normal = Vector2f::new(0., -1.);
                    let dot_product = (thing.get_speed().x * 0.) + (thing.get_speed().y * 1.);
                    let new_speed_x = thing.get_speed().x + (2. * normal.x * dot_product) * thing.get_bounciness();
                    let new_speed_y = thing.get_speed().y + (2. * normal.y * dot_product) * thing.get_bounciness();
                    thing.set_speed(Vector2f::new(new_speed_x, new_speed_y));
                    thing.set_position(thing.get_position() + thing.get_speed());
                }
                else {
                    thing.set_position(thing.get_position() + thing.get_speed());
                }
            }
        });
        world.things.append(&mut new_balls);
    }

    fn calculate_forces_on(&self, thing: &dyn Thing) -> Vec<Vector2f>{
        let mut forces = Vec::new();
        forces.push(Vector2f::new(0., self.accel_due_to_gravity * thing.get_mass()));
        //forces.push(Vector2f::new(800., 0.));
        return forces;
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

struct Grid {
    cell_size: f32,
    color: Color,
    buffers: Vec<VertexBuffer>,
    screen_size: Vector2u
}

impl Grid {
    fn new(cell_size: f32, color: Color, screen: &Screen) -> Grid {
        Grid {
            cell_size: cell_size,
            color: color,
            buffers: Vec::new(),
            screen_size: screen.renderWindow.size()
        }
    }
    fn new_vertex_buffer(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> VertexBuffer {
        let mut vertex_buffer =
            VertexBuffer::new(PrimitiveType::LINES, 2, VertexBufferUsage::STATIC);

        let vertices = [
            Vertex::with_pos_color((x1, y1).into(), self.color),
            Vertex::with_pos_color((x2, y2).into(), self.color),
        ];
        vertex_buffer.update(&vertices, 0);
        vertex_buffer
    }
}

impl <'s>Thing<'s> for Grid {
    fn get_mass(&self) -> f32 {
        0.
    }

    fn set_mass(&mut self, mass: f32) {
    }

    fn get_position(&self) -> Vector2f {
        Vector2f::new(0., 0.)
    }

    fn set_position(&mut self, position: Vector2f) {
    }

    fn get_speed(&self) -> Vector2f {
        Vector2f::new(0., 0.)
    }

    fn set_speed(&mut self, speed: Vector2f) {
    }

    fn get_bounciness(&self) -> f32 {
        0.
    }



    fn draw_on_screen(&mut self, screen: &mut Screen) {
        self.buffers.clear();

        let screen_bottom_left = Vector2f::new(0., self.screen_size.y as f32);
        let world_bottom_left = screen.translate_to_world_coords(screen_bottom_left );

        let screen_bottom_right = Vector2f::new(screen.renderWindow.size().x as f32, screen.renderWindow.size().y as f32);
        let world_bottom_right = screen.translate_to_world_coords(screen_bottom_right );

        let screen_top_left = Vector2f::new(0., 0.);
        let world_top_left = screen.translate_to_world_coords(screen_top_left );


        let times_y_to_origin = world_bottom_left.y as i32 / self.cell_size as i32;
        let start_y = self.cell_size *  times_y_to_origin as f32;
        let mut current_y = 0.;
        let max_lines = 9000;

        while(self.buffers.len() < max_lines && current_y < world_top_left.y) {
            let coord = screen.translate_to_screen_coords(Vector2f::new(0., current_y));
            self.buffers.push( self.new_vertex_buffer(coord.x, coord.y, screen.renderWindow.size().x as f32, coord.y) );
            current_y += self.cell_size;
        }

        let times_x_to_origin = world_bottom_left.x as i32 / self.cell_size as i32;
        let start_x = self.cell_size *  times_x_to_origin as f32;
        let mut current_x = 0.;
        while(self.buffers.len() < max_lines && current_x < world_bottom_right.x) {
            let coord = screen.translate_to_screen_coords(Vector2f::new(current_x, 0.));
            self.buffers.push( self.new_vertex_buffer(coord.x, coord.y, coord.x, 0.) );
            current_x += self.cell_size;
        }
        screen.draw_direct(self);
    }

    fn as_drawable(&self) -> &dyn Drawable {
        self
    }
}

impl Drawable for Grid {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(&'a self, target: &mut dyn RenderTarget, states: &RenderStates<'texture, 'shader, 'shader_texture>) {
        self.buffers.iter().for_each(|b| {
            self.buffers.iter().for_each(|v|target.draw(v));
        });
    }
}



pub(crate) struct Ball<'s> {
    //renderWindow: &'s RenderWindow,
    circle: Option<CircleShape<'s>>,
    color: Color,
    pub(crate) mass: f32,
    pub(crate) position: Vector2f,
    pub(crate) speed: Vector2f,
    bounciness: f32
}


impl<'s> Ball<'s> {
    pub(crate) fn new(position: Vector2f, mass: f32, initial_speed: Vector2f, color: Color) -> Self {
        let mut me = Self {
            //      renderWindow: renderWindow,
            circle: None,
            color: color,
            mass: mass,
            position: position,
            speed: initial_speed,
            bounciness: 0.97
        };
        me.create_circle_shape(10.);
        me
    }
    fn create_circle_shape(&mut self, radius: f32) {
        let mut circle = CircleShape::new(radius, 50);
        circle.set_position(Vector2f::new(0f32, 0f32));
        circle.set_fill_color(Color::BLUE);
        circle.set_outline_color(self.color);
        self.circle = Some(circle)
    }
}
impl<'s> Thing<'s> for Ball<'s> {
    fn get_mass(&self) -> f32 {
        self.mass
    }
    fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
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
    fn set_speed(&mut self, speed: Vector2f) {
        self.speed = speed;
    }
    fn get_bounciness(&self) -> f32 {
        self.bounciness
    }
    fn draw_on_screen(&mut self, screen: &mut Screen) {
        //let y_position = self.renderWindow.size().y as i32 - self.position.y;
        let screen_coords = screen.translate_to_screen_coords(self.position);
        let radius_on_screen = self.mass as f32 / screen.scale;
        if (screen_coords.x >= -radius_on_screen * 2. && screen_coords.x  <= screen.renderWindow.size().x as f32 + radius_on_screen * 2. &&
            screen_coords.y >= -radius_on_screen * 2.  && screen_coords.y  <= screen.renderWindow.size().y as f32 + radius_on_screen * 2.) {
            if (self.circle.is_none()) {
                self.create_circle_shape(radius_on_screen);
            }
            self.circle.as_mut().unwrap().set_radius(radius_on_screen);
            self.circle.as_mut().unwrap().set_position(Vector2f::new(screen_coords.x - radius_on_screen, screen_coords.y - radius_on_screen));
            screen.draw_direct(self)
        }
        else {
            self.circle = None;
        }
    }
    fn as_drawable(&self) -> &dyn Drawable { self }
}
impl<'s> Drawable for Ball<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(&'a self, target: &mut dyn RenderTarget, states: &RenderStates<'texture, 'shader, 'shader_texture>) {
        if (self.circle.is_some()) {
            target.draw(self.circle.as_ref().unwrap());
        }
    }
}

