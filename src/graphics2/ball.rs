use sfml::graphics::{CircleShape, Color, Drawable, RenderStates, RenderTarget, Shape, Transformable};
use sfml::system::Vector2f;
use crate::graphics2::core::{Screen, Thing};

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

