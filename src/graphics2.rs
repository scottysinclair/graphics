use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Sprite, Text, Transformable, View, CircleShape, Drawable, RenderStates, Font, VertexBuffer, PrimitiveType, VertexBufferUsage, Vertex};
use sfml::system::{Clock, SfStrConv, sleep, Time, Vector2, Vector2f, Vector2i, Vector2u};
use sfml::window::{mouse, ContextSettings, Event, Key, Style};
use std::{thread, time};
use std::cell::RefCell;
use std::ops::{Add, Deref, Index, Mul};
use std::rc::{Rc, Weak};
use tiny_skia::{PathBuilder, Point, Stroke};
use rand::Rng;
use tiny_skia::PathSegment::LineTo;
use crate::graphics2::core::{Ball, Screen, Thing, World};

pub mod core;

pub(crate) fn main2() {
    let mut world = World::new();
    let mut screen = Screen::new(3.);
    let mut clock = Clock::start();
    let mut rng = rand::thread_rng();

    let mut physics = Physics::new();
    let mut grid = Grid::new(100., &screen);


    let mut mouse_pressed = false;
    let mut lastMouseWorldPos = Vector2f::new(-1., -1.);
    let mut followBall = -1i32;
    let screen_move_speed = 5.;
    let screen_zoom_speed = 1.;

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
                    add_ball(&screen, rng.gen_range(10..40), &mut world, x, y, inital_speed)
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
    screen.drawDirect(&vertex_buffer);
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
        let grid_size = 5000;
        let grid_tolerance = 3;
        let mut new_balls = Vec::new();
        world.things.iter_mut().for_each(|thing : &mut Ball| {
            let xdiff = thing.position.x as i32 % grid_size;
            let ydiff = thing.position.y as i32 % grid_size;
            if  (xdiff > grid_tolerance && xdiff < (grid_size - grid_tolerance)) &&  (ydiff > grid_tolerance && ydiff < (grid_size - grid_tolerance)) {
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
            }
        });
        world.things.append(&mut new_balls);
    }

    fn calculate_forces_on(&self, thing: &Ball) -> Vec<Vector2f>{
        let mut forces = Vec::new();
        forces.push(Vector2f::new(0., self.accel_due_to_gravity * thing.mass));
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
    lines_x: Vec<i32>,
    lines_y: Vec<i32>,
    buffers: Vec<VertexBuffer>,
    screen_size: Vector2u
}

impl Grid {
    fn new(cell_size: f32, screen: &Screen) -> Grid {
        Grid {
            cell_size: cell_size,
            lines_x: Vec::new(),
            lines_y: Vec::new(),
            buffers: Vec::new(),
            screen_size: screen.renderWindow.size()
        }
    }
}

impl Thing for Grid {
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
        /*
        let screen_bottom_left = Vector2f::new(0., self.screen_size.y as f32);
        let world_bottom_left = screen.translate_to_screen_coords(screen_bottom_left );

        let screen_bottom_right = Vector2f::new(screen.renderWindow.size().x as f32, screen.renderWindow.size().y as f32);
        let world_bottom_right = screen.translate_to_screen_coords(screen_bottom_right );

        let screen_top_left = Vector2f::new(0., 0.);
        let world_top_left = screen.translate_to_screen_coords(screen_top_left );

        let screen_top_right= Vector2f::new(screen.renderWindow.size().x as f32, 0.);
        let world_top_right = screen.translate_to_screen_coords(screen_top_right );

        let times_y_to_origin = world_bottom_left.y as i32 / self.cell_size as i32;
        let start_y = 0.; //self.cell_size *  times_y_to_origin as f32;
        self.lines_y.clear();
        let mut current_y = start_y;
        while(current_y < world_top_left.y) {
            self.lines_y.push(screen.translate_to_screen_coords(Vector2f::new(0., current_y)).y as i32);
            current_y += self.cell_size;
        }

        let times_x_to_origin = world_bottom_left.x as i32 / self.cell_size as i32;
        let start_x = self.cell_size *  times_x_to_origin as f32;
        self.lines_x.clear();
        let mut current_x = start_x;
        while(current_x < world_bottom_right.x) {
            self.lines_x.push(screen.translate_to_screen_coords(Vector2f::new(current_x, 0.)).x as i32);
            current_x += self.cell_size;
        }
         */
        /*
        self.buffers.clear();
        self.lines_y.iter().for_each(|y| {
            let mut vertex_buffer =
                VertexBuffer::new(PrimitiveType::LINES, 6, VertexBufferUsage::STATIC);

            let vertices = [
                Vertex::with_pos_color((0., *y as f32).into(), Color::GREEN),
                Vertex::with_pos_color((self.screen_size.x as f32, *y as f32).into(), Color::GREEN),
            ];
            vertex_buffer.update(&vertices, 0);
            self.buffers.push(vertex_buffer)
        });*/
    }
}

impl Drawable for Grid {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(&'a self, target: &mut dyn RenderTarget, states: &RenderStates<'texture, 'shader, 'shader_texture>) {
        self.buffers.iter().for_each(|b| {
            self.buffers.iter().for_each(|v|target.draw(v));
        });
    }
}