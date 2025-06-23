use sdl2::{
    pixels::{
        Color,
    },
    render::{
        self,
        TextureCreator,
        Canvas,
    },
    video,
    event::{
        Event,
    },
    rect::{
        Rect
    },
};
use crate::utils::error;
use super::{
    widget::{
        camera,
        widget_base,
        layout,
    },
    theme,
};
use std::{
    rc::Rc,
    cell::RefCell,
};

type WindowCanvas = Canvas<video::Window>;


//pub struct Ren

#[allow(unused)]
pub struct RendererManager {
    theme_manager: theme::RcThemeManager,
    video_subsystem: sdl2::VideoSubsystem,
    canvas: Rc<RefCell<WindowCanvas>>,
    widget_base_builder: widget_base::WidgetBaseBuilder,
}

impl RendererManager {
    pub fn new(theme_manager: theme::RcThemeManager, sdl_context: &mut sdl2::Sdl)
        -> Result<RendererManager, error::DynError>
    {
        let video_subsystem = sdl_context.video()?;

        let window = {
            let theme_manager = theme_manager.borrow();
            video_subsystem
                .window(theme_manager.title_name(),
                        theme_manager.initial_width(),
                        theme_manager.initial_height())
                .build()?
        };

        let canvas = window.into_canvas().build()?;
        //let texture_creator = canvas.texture_creator();

        let rc_canvas = Rc::new(RefCell::new(canvas));
        let widget_base_builder = widget_base::WidgetBaseBuilder::new(
            theme_manager.clone(), rc_canvas.clone());
        
        //let camera = {
        //    let ref_theme_manager = theme_manager.borrow();
        //    let camera_width = ref_theme_manager.initial_width();
        //    let camera_height = ref_theme_manager.initial_height() - 140;
        //    camera::CameraManager::new(
        //        theme_manager.clone(),
        //        camera_width,
        //        camera_height,
        //        rc_canvas.clone(),
        //        texture_creator,
        //    )?
        //};
        let renderer_manager = RendererManager {
            theme_manager: theme_manager.clone(),
            video_subsystem,
            canvas: rc_canvas,
            widget_base_builder,
        };

        Ok(renderer_manager)
    }
    pub fn get_shared_canvas(&self) -> Rc<RefCell<WindowCanvas>> {
        self.canvas.clone()
    }
    
    pub fn views_iterator(&mut self, layout: &mut layout::Layout) -> Result<(), error::DynError> {
        let bg_color = {
            let theme_manager = self.theme_manager.borrow();
            theme_manager.window_bg_color()
        };

        self.clear_canvas(bg_color);
        layout.rendering(self.canvas.clone(), self.get_window_rect())?;
        self.canvas.borrow_mut().present();
        Ok(())
    }

    fn get_window_rect(&self) -> Rect {
        let canvas = self.canvas.borrow();
        let (width, height) = canvas.window().size();
        Rect::new(0, 0, width, height)
    }

    fn clear_canvas(&self, color: Color) {
        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(color);
        canvas.clear();
    }
}

