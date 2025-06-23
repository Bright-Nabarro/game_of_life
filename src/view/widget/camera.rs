use sdl2::{
    render::{
        self,
        Texture,
        TextureCreator,
        Canvas,
    },
    video,
    pixels::PixelFormatEnum,
};
use std::{
    rc::Rc,
    cell::RefCell,
};
use crate::utils::error;
use super::super::theme;
use super::widget_base;

pub struct Camera {
    widget_base: widget_base::WidgetBase,
    scale: f32,     // (0, 1)
    texture_cache: Option<Texture>,
}

impl Camera {
    pub fn new(widget_base: widget_base::WidgetBase, scale: f32)
        -> Result<Self, error::DynError>
    {
        if scale > 1. || scale < 0. {
            return Err(String::from("Invalid arugment scale").into());
        }

        Ok(Camera { widget_base, scale, texture_cache: None })
    }
    
    fn fill_base_element(&self, texture: &mut Texture) -> Result<(), error::DynError> {
        let bg_color = {
            let theme_manager = self.widget_base.theme_manager.borrow();
            theme_manager.camera_bg_color()
        };

        let mut canvas = self.widget_base.canvas.borrow_mut();
        canvas.with_texture_canvas(texture, |canvas| {
            canvas.set_draw_color(bg_color);
            canvas.clear();
        })?;
        Ok(())
    }
}

impl widget_base::Widget for Camera {
    fn widget_base(&self) -> &widget_base::WidgetBase {
        &self.widget_base
    }

    fn rendering_on_texture(&mut self, width: u32, height: u32) 
        -> Result<& render::Texture, error::DynError>
    {
        if let Some(ref value) = self.texture_cache {
            return Ok(value)
        }

        let mut texture = self.create_new_texture(width, height)?;
        self.fill_base_element(&mut texture)?;

        Ok(self.texture_cache.insert(texture))   
    }
}

