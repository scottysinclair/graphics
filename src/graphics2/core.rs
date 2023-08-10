use sfml::graphics::{CircleShape, Color, Drawable, RenderStates, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::system::{Vector2f, Vector2i};
use sfml::window::{ContextSettings, Style};

pub(crate) struct Screen {
    pub(crate) position: Vector2f,
    pub(crate) scale: f32,
    pub(crate) renderWindow: RenderWindow,
}

impl Screen {
    pub(crate) fn new(scale: f32) -> Screen {
        let mut s = Screen {
            position: Vector2f::new(0., 0.),
            scale: scale,
            renderWindow: RenderWindow::new(
                (3840, 2400),
//(1024, 768),
"Graphics",
Style::FULLSCREEN,
//Style::CLOSE,
&ContextSettings::default(),
            ),
        };
        s.renderWindow.set_position(Vector2i::new(500, 500));
        s.renderWindow.set_framerate_limit(60);
        s.renderWindow.set_vertical_sync_enabled(true);
        s
    }
    pub(crate) fn clear(&mut self, color: Color) {
        self.renderWindow.clear(color)
    }
    pub(crate) fn draw_world(&mut self, world: &mut World) {
        world.draw(self);
    }
    pub(crate) fn draw_direct(&mut self, thing: &dyn Drawable) {
        self.renderWindow.draw(thing);
    }
    pub(crate) fn display(&mut self) {
        self.renderWindow.display()
    }


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

pub(crate) struct World<'s> {
    pub(crate) things: Vec<Box<dyn Thing<'s> + 's>>,
}

pub(crate) trait Thing<'a>: Drawable {
    fn get_mass(&self) -> f32;
    fn set_mass(&mut self, mass: f32);
    fn get_position(&self) -> Vector2f;
    fn set_position(&mut self, position: Vector2f);
    fn get_speed(&self) -> Vector2f;
    fn set_speed(&mut self, speed: Vector2f);
    fn get_bounciness(&self) -> f32;
    fn draw_on_screen(&mut self, screen: &mut Screen);
    fn as_drawable(&self) -> &dyn Drawable;
}


impl<'s> World<'s> {
    pub(crate) fn new() -> World<'s> {
        World {
            things: Vec::new()
        }
    }
    pub(crate) fn add(&mut self, thing: Box<dyn Thing<'s> + 's>) {
        self.things.push(thing);
    }
    pub(crate) fn draw(&mut self, screen: &mut Screen) {
        self.things.iter_mut().for_each(|t| t.draw_on_screen(screen));
    }
}


impl<'s> Drawable for World<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(&'a self, target: &mut dyn RenderTarget, states: &RenderStates<'texture, 'shader, 'shader_texture>) {
        self.things.iter().for_each(|t| target.draw(t.as_ref().as_drawable()));
    }
}