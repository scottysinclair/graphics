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
use crate::graphics2::physics::Physics;
use crate::graphics2::ball::Ball;
use crate::graphics2::grid::Grid;


pub mod core;
pub mod physics;
pub mod ball;
pub mod grid;

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

        grid.draw_on_screen(&mut screen);

        screen.display()
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



