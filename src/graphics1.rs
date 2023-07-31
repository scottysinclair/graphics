use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Sprite, Text, Transformable, View, CircleShape, Drawable, RenderStates, Font};
use sfml::system::{Clock, SfStrConv, sleep, Time, Vector2f, Vector2i, Vector2u};
use sfml::window::{mouse, ContextSettings, Event, Key, Style};
use sfml::audio::{Sound, SoundBuffer, SoundSource};
use std::{thread, time};
use std::ops::{Add, Deref, Index, Mul};
use std::time::SystemTime;
use sfml::graphics::ShaderType::Vertex;
use sfml::LoadResult;
use tiny_skia::Point;

struct Dot<'a> {
    shape: CircleShape<'a>,
    position: Vector2f,
    size: i32,
    colorIndex: usize,
    mass: f32,
    forces: Vec<Vector2f>,
    speed: Vector2f,
    sound: Option<Sound<'a>>,
    soundStartedAt: SystemTime
}

impl<'a> Dot<'a> {
    fn new(x: i32, y: i32, size: i32, colorIndex: usize, speed: Vector2i) -> Dot<'a> {
        let mut d = Dot {
            position: Vector2f::new(x as f32, y as f32),
            size: size,
            colorIndex: colorIndex,
            shape: CircleShape::new(size as f32, 100),
            mass: 1f32,
            forces: Vec::new(),
            speed: Vector2f::new(speed.x as f32, -speed.y as f32),
            sound: None,
            soundStartedAt: SystemTime::now()
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
    fn draw(&mut self, screen: &mut Screen) {
        self.colorIndex = self.colorIndex % screen.colors.len();
        self.shape.set_fill_color(screen.colors[self.colorIndex]);
        self.shape.set_radius(self.size as f32);
        self.shape.set_position(Vector2f::new((self.position.x as i32 - self.size) as f32, screen.translate_y(self.position.y as i32 + self.size)));
        screen.drawObject(&self.shape)
    }
}


struct Screen {
    window: RenderWindow,
    colors: Vec<Color>,
}

impl Screen {
    fn new(width: u32, height: u32, style: Style) -> Screen {
        let mut s = Screen {
            window: RenderWindow::new(
                (width, height),
                "Graphics",
                style,
                &ContextSettings::default(),
            ),
            colors: Vec::new(),
        };
        s.colors.push(Color::WHITE);
        s.colors.push(Color::RED);
        s.colors.push(Color::GREEN);
        s.colors.push(Color::BLUE);
        s.colors.push(Color::YELLOW);
        s.colors.push(Color::MAGENTA);
        s.colors.push(Color::CYAN);

        s.window.set_position(Vector2i::new(2000, 1000));
        s.window.set_framerate_limit(60);
        s.window.set_vertical_sync_enabled(true);
        s
    }

    fn translate_y(&self, posY: i32) -> f32 {
        self.window.size().y as f32 - posY as f32
    }

    fn color_cycle(&mut self) {
        let c = self.colors.pop().unwrap();
        self.colors.insert(0, c);
    }

    fn drawObject(&mut self, object: &dyn Drawable) {
        self.window.draw(object)
    }

    fn draw(&mut self, dots: &mut Vec<Dot>) {
        dots.iter_mut().for_each(|d| d.draw(self));
    }
}


fn calculateNewPositions<'a>(screen: &Screen, elapsedTime: Time, dots: &mut Vec<Dot<'a>>, buffer: &'a SoundBuffer) {
//    let bottom = 100f32;
    let bottom = 0f32;
    let now = SystemTime::now();
    let makeMoreSound = true; dots.iter().filter(|d|d.sound.is_some()).count() < 100;
    dots.iter_mut().for_each(|d| {
        let elapsedMillis = d.soundStartedAt.elapsed().unwrap().as_millis();
        if (elapsedMillis <= 100) {
            d.sound.as_mut().map(|s| s.set_volume(100. - elapsedMillis as f32));
        }
        if (d.soundStartedAt.elapsed().unwrap().as_millis() > 100) {
            d.sound.as_mut().map(|mut s| s.stop());
            d.sound = None;
        }
        if (d.colorIndex >= 0) {
            if (d.position.y < bottom && d.speed.y < 0f32) {
                if (d.speed.y.abs() < 0.2f32) {
                    d.speed.y = 0f32;
                    d.speed.x = 0f32;
                    d.position.y = bottom;
                    d.sound.as_mut().map(|mut s| s.stop());
                    d.sound = None;
                } else {
                    d.speed.y *= -0.85f32;
                    if (d.sound.is_none() && d.soundStartedAt.elapsed().unwrap().as_millis() > 500 && makeMoreSound) {
                        d.sound = Some(Sound::with_buffer(buffer));
                        d.soundStartedAt = SystemTime::now();
                        d.sound.as_mut().map(|s| {
                            s.set_playing_offset(Time::milliseconds(1000));
                            //pitch range from 0.1 to 1
                            //x range is 0 to 1024, so
                            let mut pitch = d.position.x / (screen.window.size().x as f32 * 3f32);
                            if (pitch < 0.1f32) {
                                pitch = 0.1f32;
                            }
                            else if (pitch > 0.5) {
                                pitch = 0.5;
                            }
                            s.set_pitch(pitch);
                            s.set_volume(100.);
                            s.play();
                            //println!("{}", d.speed.y)
                        });
                    }
                }
            }
            let accelY = -20f32;
            let changeInSpeedX = 0f32;
            let changeInSpeedY = if (d.speed.y == 0f32 && d.position.y == bottom) { 0. } else { accelY * elapsedTime.as_seconds() };
            d.speed.x += changeInSpeedX;
            d.speed.y += changeInSpeedY;
            d.position.y += d.speed.y;
            d.position.x += d.speed.x;
        }
    })
}


pub(crate) fn main1() {
     let buffer = SoundBuffer::from_file("sounds/ding.ogg").unwrap();
//     let mut sound = Sound::with_buffer(&buffer);
  //  sound.play();




    let mut screen = //Screen::new(1024, 768, Style::CLOSE);
//        Screen::new(3840, 2160, Style::FULLSCREEN);
        Screen::new(3840, 2400, Style::FULLSCREEN);

    let mut clock = Clock::start();
    let mut currentColor = 0;

    // Create the window of the application

    let mut dots = Vec::new();
    //dots.push(Dot::new(0, 0, 32, currentColor, Vector2i::new(0,0)));

    let mut lastMousePos = Vector2i::new(-1, -1);
    let mut windSpeed = Vector2i::new(0, 0);
    let mut mousePressed = false;
    let mut size = 5;

    let mut biggestElapsedTime = 0.;

    while screen.window.is_open() {
        while let Some(event) = screen.window.poll_event() {
            let elapsedTime = clock.elapsed_time();
            match (event) {
                Event::KeyPressed { code, .. } => {
                    if Key::Escape == code {
                        screen.window.close()
                    } else if (Key::Space == code) {
                        screen.color_cycle();
                    }
                }
                Event::MouseMoved { x, y } => {
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
                Event::MouseWheelScrolled { delta, .. } => {
                    // dots.first_mut().unwrap().changeSize(delta as i32);
                    dots.iter_mut().for_each(|d| d.changeSize(delta as i32));
                    size += delta as i32;
                }
                Event::Closed => {
                    screen.window.close()
                }
                _ => {  }
            }
        }

        screen.window.clear(Color::BLACK);
        let timeSinceLastRender: Time = clock.elapsed_time();
        if (biggestElapsedTime < timeSinceLastRender.as_seconds()) {
            biggestElapsedTime = timeSinceLastRender.as_seconds();
            println!("biggestElapsedTime {}", biggestElapsedTime)
        }
        //println!("tslr {}", timeSinceLastRender.as_seconds());
        calculateNewPositions(&screen, timeSinceLastRender, &mut dots, &buffer);
        screen.draw(&mut dots);
        screen.window.display();
        clock.restart();
        sleep(Time::milliseconds(1000 / 100))
    }
}
