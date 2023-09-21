use crate::common::*;
use crate::geometry::*;
use crate::material::*;
use crate::ray::*;
use crate::vector::*;
use crate::camera::*;
use image::{ImageBuffer, Rgb};

pub struct Renderer {
  samples: u32,
  resolution: Vec2u,
}

impl Renderer {
  pub fn new(resolution: (u32, u32), samples: u32) -> Self {
    Self {
      samples,
      resolution: Vec2u::from(resolution),
    }
  }

  pub fn render(&self, scene: &Scene, camera: &Camera) -> ImageBuffer {
    let pixels = vec![0; 3 * resolution.x as usize * resolution.y as usize];
    let mut buffer = ImageBuffer::from_raw(resolution.x, resolution.y, pixels).unwrap();

    // TODO
    
    buffer
  }
}
