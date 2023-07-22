use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Sprite, Text, Transformable, View, CircleShape, Drawable};
use sfml::system::{Clock, sleep, Time, Vector2f, Vector2i, Vector2u};
use sfml::window::{mouse, ContextSettings, Event, Key, Style};
use std::{thread, time};
use std::ops::{Add, Index, Mul};
use sfml::graphics::ShaderType::Vertex;
use tiny_skia::Point;

//https://github.com/jeremyletang/rust-sfml/blob/master/examples/music-stream.rs

struct Dot<'a> {
    shape:  CircleShape<'a>,
    position: Vector2f,
    size: i32,
    colorIndex: usize,
    mass: f32,
    forces: Vec<Vector2f>,
    speed: Vector2f
}

impl <'a>Dot<'a> {
    fn new(x : i32, y: i32, size: i32, colorIndex: usize, speed: Vector2i) -> Dot<'a> {
        let mut d = Dot {
            position: Vector2f::new(x as f32,y as f32),
            size: size,
            colorIndex: colorIndex,
            shape:  CircleShape::new(size as f32, 100),
            mass: 1f32,
            forces: Vec::new(),
            speed: Vector2f::new(speed.x as f32, -speed.y as f32)
        };
        d.forces.push(Vector2f::new(0f32, 9.8f32));
        return d;
    }
    fn changeSize(&mut self, delta: i32) {
        self.size += delta;
    }
    fn setPos(&mut self, x: i32, y: i32) {
        self.position = Vector2f::new(x as f32, y as f32);
    }
    fn calculateNewPos(&mut self, elapsedTime: f32)  {
//        let totalForce = self.forces.iter().reduce(|a, b| a.add(b)).unwrap_or(&Vector2f::new(0f32, 0f32));
        let totalForce = Vector2f::new(0f32, 1f32);
        let acc = Vector2f::new(totalForce.x * self.mass, totalForce.y * self.mass);
        self.speed = self.speed.add(acc.mul(elapsedTime));
        self.position = self.position.add(self.speed);
    }
    fn draw(&mut self, screen: &mut Screen) {
        self.colorIndex = self.colorIndex % screen.colors.len();
        self.shape.set_fill_color(screen.colors[self.colorIndex]);
        self.shape.set_radius(self.size as f32);
        self.shape.set_position(Vector2f::new((self.position.x as i32 - self.size) as f32,  screen.translate_y(self.position.y as i32 + self.size)));
        screen.drawObject(&self.shape)
    }
}



struct Screen {
    window: RenderWindow,
    colors: Vec<Color>,
    size: Vector2u
}

impl Screen {
    fn new(width: u32, height: u32) -> Screen {
        let mut s = Screen {
            window: RenderWindow::new(
            (width, height),
            "Graphics",
            Style::FULLSCREEN,
            &ContextSettings::default(),
            ),
            colors: Vec::new(),
            size: Vector2u::new(width, height)
        };
        s.colors.push(Color::WHITE);
        s.colors.push(Color::RED);
        s.colors.push(Color::GREEN);
        s.colors.push(Color::BLUE);
        s.colors.push(Color::YELLOW);
        s.colors.push(Color::MAGENTA);
        s.colors.push(Color::CYAN);

        s.window.set_framerate_limit(60);
        s.window.set_vertical_sync_enabled(true);
        s
    }

    fn translate_y(&self, posY: i32) -> f32 {
        self.size.y as f32 - posY as f32
    }

    fn color_cycle(&mut self) {
        let c =  self.colors.pop().unwrap();
        self.colors.insert(0, c);
    }

    fn drawObject(&mut self, object: &dyn Drawable) {
        self.window.draw(object)
    }

    fn draw(&mut self, dots: &mut Vec<Dot>) {
        dots.iter_mut().for_each(|d| d.draw(self));
    }

}


fn calculateNewPositions(elapsedTime: Time, dots: &mut Vec<Dot>) {
    dots.iter_mut().for_each(|d|
        if (d.colorIndex  >= 0) {
            if (d.position.y <= 1000f32 && d.speed.y < 0f32) {
                if (d.speed.y.abs() < 0.2f32) {
                    d.speed.y = 0f32;
                    d.speed.x = 0f32;
                    d.position.y = 1000f32;
                }
                else {
                    d.speed.y *= -0.85f32;
                }
            }
            let accelY = -20f32;
            let changeInSpeedX = 0f32;
            let changeInSpeedY = accelY * elapsedTime.as_seconds();
            //println!("{}", changeInSpeedY);
            d.speed.x += changeInSpeedX;
            d.speed.y +=  changeInSpeedY;
            //println!("dsy {}", d.speed.y);
            d.position.y += d.speed.y;
            d.position.x += d.speed.x;
        }
    )
}




fn main() {
    let mut screen = Screen::new(3840, 2160);

    let mut clock = Clock::start();
    let mut currentColor = 0;

    // Create the window of the application

    let mut dots = Vec::new();
    //dots.push(Dot::new(0, 0, 32, currentColor, Vector2i::new(0,0)));

    let mut lastMousePos = Vector2i::new(-1, -1);
    let mut windSpeed = Vector2i::new(0,0);
    let mut mousePressed = false;
    let mut size = 10;

    while screen.window.is_open() {
        while let Some(event) = screen.window.poll_event() {
            let elapsedTime = clock.elapsed_time();
            match(event) {
                Event::KeyPressed { code, .. }=> {
                    if Key::Escape == code {
                        screen.window.close()
                    }
                    else if (Key::Space == code) {
                        screen.color_cycle();
                    }
                }
                Event::MouseMoved { x, y} => {
                    if (mousePressed) {
                        dots.push(Dot::new(x, screen.translate_y(y) as i32, size, currentColor, Vector2i::new(x - lastMousePos.x, y - lastMousePos.y)));
                        currentColor = (currentColor + 1) % screen.colors.len();
                    }
                    lastMousePos.x = x;
                    lastMousePos.y = y;
                }
                Event::MouseButtonPressed { x, y, .. } => {
                    mousePressed = true;
                }
                Event::MouseButtonReleased { .. } => {
                    mousePressed = false
                }
                Event::MouseWheelScrolled { delta, ..} => {
                   // dots.first_mut().unwrap().changeSize(delta as i32);
                    dots.iter_mut().for_each(|d| d.changeSize(delta as i32));
                    size += delta as i32;
                }
                Event::Closed => {
                    screen.window.close()
                }
                _ => { println!("Event") }
            }
        }

        screen.window.clear(Color::BLACK);
        let timeSinceLastRender : Time = clock.elapsed_time();
        println!("tslr {}", timeSinceLastRender.as_seconds());
        calculateNewPositions(timeSinceLastRender, &mut dots);
        screen.draw( &mut dots);
        screen.window.display();
        clock.restart();
        sleep(Time::milliseconds(1000 / 100))
    }


}