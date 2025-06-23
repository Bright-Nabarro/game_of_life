use crate::utils::error;
use crate::view::{
    renderer::{
        RendererManager
    },
    theme::{
        self,
        ThemeManager,
    },
    widget::{
        layout,
        camera,
        widget_base,
    },
};

use sdl2::{
    event::{
        Event
    },
};

use std::{
    rc::Rc,
    cell::RefCell,
};


pub struct GameContext {
    sdl_context: sdl2::Sdl,
    theme_manager: theme::RcThemeManager,
    renderer_manager: RendererManager,
    layout_manager: layout::LayoutManager,
}

impl GameContext {
    pub fn init() -> Result<Self, error::DynError> {
        let mut sdl_context = sdl2::init()?;
        let theme_manager = ThemeManager::default_init()?;
        let theme_manager = Rc::new(RefCell::new(theme_manager));
        let renderer_manager = RendererManager::new(theme_manager.clone(), &mut sdl_context)?;
        let layout_manager = layout::LayoutManager::new(theme_manager.clone(), renderer_manager.get_shared_canvas());

        let game_context = GameContext{
            sdl_context,
            theme_manager,
            renderer_manager,
            layout_manager,
        };
        Ok(game_context)
    }
}

pub struct GameManager {
    game_context: GameContext,
    event_pump: sdl2::EventPump,
}

impl GameManager {
    pub fn new(game_context: GameContext) -> Result<Self, error::DynError> {
        let event_pump = game_context.sdl_context.event_pump()?;
        let controller_manager = GameManager{
            game_context,
            event_pump
        };
        Ok(controller_manager)
    }

    pub fn main_loop(&mut self) -> Result<(), error::DynError>{
        let mut layout = self.game_context.layout_manager.get_default_layout()?;
        loop {
            if self.handle_event()? {
                break;
            }
            self.game_context.renderer_manager.views_iterator(&mut layout)?;
        };
        
        Ok(())
    }
    
    fn handle_event(&mut self) -> Result<bool, error::DynError> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    return Ok(true);
                },
                _ => {}
            }
        }

        Ok(false)
    }
}

