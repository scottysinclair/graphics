use sfml::graphics::{Color, Drawable, PrimitiveType, RenderStates, RenderTarget, Vertex, VertexBuffer, VertexBufferUsage};
use sfml::system::{Vector2f, Vector2u};
use crate::graphics2::core::{Screen, Thing};

pub(crate) struct Grid {
    cell_size: f32,
    color: Color,
    buffers: Vec<VertexBuffer>,
    screen_size: Vector2u
}

impl Grid {
    pub(crate) fn new(cell_size: f32, color: Color, screen: &Screen) -> Grid {
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