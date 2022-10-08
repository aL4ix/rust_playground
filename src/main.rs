extern crate core;

use std::sync::{Arc, Mutex};

struct TextureCreator {
    some_memory: i32,
}

struct Texture<'tex_creator> {
    tex: &'tex_creator i32,
}

impl TextureCreator {
    fn create_texture(&self) -> Texture {
        Texture {
            tex: &self.some_memory
        }
    }
}

struct LazyTexture<'tex_creator> {
    actual_texture: Option<Texture<'tex_creator>>,
}

trait AnyTexture<'tex_creator> {
    fn get_tex(&mut self, tex_creator: &'tex_creator TextureCreator) -> &Texture;
}

impl<'tex_creator> AnyTexture<'tex_creator> for LazyTexture<'tex_creator> {
    fn get_tex(&mut self, tex_creator: &'tex_creator TextureCreator) -> &Texture {
        self.actual_texture = Some(tex_creator.create_texture());
        self.actual_texture.as_ref().unwrap()
    }
}

struct Polygon<'tex_creator> {
    tex: Option<Arc<Mutex<dyn AnyTexture<'tex_creator>>>>,
}

struct Text<'tex_creator> {
    lazy_tex: Arc<Mutex<LazyTexture<'tex_creator>>>,
}

impl<'tex_creator> Text<'tex_creator> {
    fn build(&self) -> Polygon<'tex_creator> {
        Polygon {
            tex: Some(Arc::clone(&self.lazy_tex))
        }
    }
}

fn main() {
    println!("Hello, world!");
}
