use crate::view::theme;
use sdl2::{
    video,
    render,
    pixels,
};

use std::{
    rc::Rc,
    cell::RefCell,
};

use crate::utils::error;
type WindowCanvas = render::Canvas<video::Window>;

pub struct WidgetBase {
    pub(super) theme_manager: Rc<RefCell<theme::ThemeManager>>,
    pub(super) canvas: Rc<RefCell<WindowCanvas>>,
    pub(super) texture_creator: render::TextureCreator<video::WindowContext>,
}

pub struct WidgetBaseBuilder {
    theme_manager: Rc<RefCell<theme::ThemeManager>>,
    canvas: Rc<RefCell<WindowCanvas>>,
}

impl WidgetBaseBuilder {
    pub fn new(theme_manager: Rc<RefCell<theme::ThemeManager>>,
           canvas: Rc<RefCell<WindowCanvas>>)
    -> Self
    {
        WidgetBaseBuilder{ theme_manager, canvas }
    }
    
    pub fn build(&self) -> WidgetBase {
        let texture_creator = {
            let canvas = self.canvas.borrow();
            canvas.texture_creator()
        };

        WidgetBase {
            theme_manager: self.theme_manager.clone(),
            canvas: self.canvas.clone(),
            texture_creator
        }
    }
}


pub type RcTexture = Rc<RefCell<render::Texture>>;
pub trait Widget {
    fn widget_base(&self) -> &WidgetBase;
    fn rendering_on_texture(&mut self, width: u32, height: u32)
        -> Result<&render::Texture, error::DynError>;
    
    fn create_new_texture(&self, width: u32, height: u32)
        -> Result<render::Texture, error::DynError>
    {
        let texture_creator = &self.widget_base().texture_creator;
        let texture = texture_creator.create_texture_target(
            pixels::PixelFormatEnum::RGBA8888,
            width, height)?;

        Ok(texture)
    }

}

pub struct BlankWidget {
    widget_base: WidgetBase,
    transparent: bool,
    texture_cache: Option<render::Texture>,
}

impl BlankWidget {
    pub fn new(widget_base: WidgetBase, transparent: bool) -> Self {
        BlankWidget { widget_base, transparent, texture_cache: None }
    }
    
    fn fill_color(&self, texture: &mut render::Texture, color: pixels::Color)
        -> Result<(), error::DynError>
    {
        let mut canvas = self.widget_base.canvas.borrow_mut();
        canvas.with_texture_canvas(texture, |canvas| {
            canvas.set_draw_color(color);
            canvas.clear();
        })?;
        Ok(())       
    }
}

impl Widget for BlankWidget {
    fn widget_base(&self) -> &WidgetBase {
        &self.widget_base
    }

    fn rendering_on_texture(&mut self, width: u32, height: u32)
        -> Result<&render::Texture, error::DynError>
    {
        if let Some(ref value) = self.texture_cache {
            return Ok(value)
        }

        let mut texture = self.create_new_texture(width, height)?;
        let blank_color = if self.transparent {
            texture.set_blend_mode(render::BlendMode::Blend);
            pixels::Color::RGBA(255, 255, 255, 0)
        } else {
            let theme = self.widget_base.theme_manager.borrow();
            theme.default_widget_color()
        };

        self.fill_color(&mut texture, blank_color)?;
        
        Ok(self.texture_cache.insert(texture))
    }
}

pub struct BlankWidgetBuilder {
    
}
