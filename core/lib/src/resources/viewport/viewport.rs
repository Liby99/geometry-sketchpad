use crate::math::*;

pub static WINDOW_SIZE: [f64; 2] = [960., 720.];

#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub virtual_center: Vector2,
    pub virtual_size: Vector2,
    pub screen_size: Vector2,
    half_virtual_size: Vector2,
    half_screen_size: Vector2,
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new(vec2![0., 0.], vec2![20., 15.], WINDOW_SIZE.into())
    }
}

impl Viewport {
    pub fn new(virtual_center: Vector2, virtual_size: Vector2, screen_size: Vector2) -> Self {
        Self {
            virtual_center,
            virtual_size: vec2![
                virtual_size.x,
                virtual_size.x / screen_size.x * screen_size.y
            ], // Normalize scale
            screen_size,
            half_virtual_size: virtual_size / 2.0,
            half_screen_size: screen_size / 2.0,
        }
    }

    pub fn set_screen_size(&mut self, window_size: Vector2) {
        self.screen_size = window_size;
        self.virtual_size.y = self.virtual_size.x / self.screen_size.x * self.screen_size.y;
        self.half_screen_size = self.screen_size / 2.0;
        self.half_virtual_size = self.virtual_size / 2.0;
    }

    pub fn set_virtual_size_x(&mut self, virtual_size_x: f64) {
        self.virtual_size.x = virtual_size_x;
        self.virtual_size.y = virtual_size_x / self.screen_size.x * self.screen_size.y;
        self.half_virtual_size = self.virtual_size / 2.0;
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.screen_size.y / self.screen_size.x
    }

    pub fn virtual_to_screen_scale(&self) -> f64 {
        self.virtual_size.x / self.screen_size.x
    }

    pub fn screen_width(&self) -> f64 {
        self.screen_size.x
    }

    pub fn screen_height(&self) -> f64 {
        self.screen_size.y
    }

    pub fn virtual_width(&self) -> f64 {
        self.virtual_size.x
    }

    pub fn virtual_height(&self) -> f64 {
        self.virtual_size.y
    }

    pub fn half_screen_width(&self) -> f64 {
        self.half_screen_size.x
    }

    pub fn half_screen_height(&self) -> f64 {
        self.half_screen_size.y
    }

    pub fn half_virtual_width(&self) -> f64 {
        self.half_virtual_size.x
    }

    pub fn half_virtual_height(&self) -> f64 {
        self.half_virtual_size.y
    }

    pub fn x_min(&self) -> f64 {
        self.virtual_center.x - self.half_virtual_size.x
    }

    pub fn x_max(&self) -> f64 {
        self.virtual_center.x + self.half_virtual_size.x
    }

    pub fn y_min(&self) -> f64 {
        self.virtual_center.y - self.half_virtual_size.y
    }

    pub fn y_max(&self) -> f64 {
        self.virtual_center.y + self.half_virtual_size.y
    }

    pub fn virtual_aabb(&self) -> AABB {
        AABB::new(
            self.x_min(),
            self.y_min(),
            self.virtual_width(),
            self.virtual_height(),
        )
    }

    pub fn screen_aabb(&self) -> AABB {
        AABB::new(0., 0., self.screen_width(), self.screen_height())
    }
}
