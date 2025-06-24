use sdl2::{
    render::{
        self,
        Texture,
        TextureCreator,
        Canvas,
    },
    pixels::Color,
    rect::{
        Rect,
        Point
    },
    video,
    pixels::PixelFormatEnum,
};
use std::{
    rc::Rc,
    cell::RefCell,
};
use crate::{
    utils::error,
    model::grid,
};
use super::super::theme;
use super::widget_base;

static CELL_PIXEL_UPPER: usize = 128;
pub struct ViewGrid {
    scale: usize,   // 一个cell对应几个像素
    cell_grid: Rc<RefCell<grid::CellGrid>>,
    theme_manager: theme::RcThemeManager,
}

impl ViewGrid {
    pub fn new(scale: usize, cell_grid: Rc<RefCell<grid::CellGrid>>, theme_manager: theme::RcThemeManager)
        -> error::Result<Self>
    {
        if scale < 0 || scale > CELL_PIXEL_UPPER {
            return Err(String::from("Invalid Scale").into());
        }
        Ok(ViewGrid { scale, cell_grid, theme_manager})
    }
    
    pub fn fill_grid(&self, canvas: Rc<RefCell<render::WindowCanvas>>, texture: &mut Texture)
        -> error::Result<()>
    {
        self.draw_lines(canvas.clone(), texture)?;
        Ok(())
    }

    fn draw_lines(&self, canvas: Rc<RefCell<render::WindowCanvas>>, texture: &mut Texture)
        -> error::Result<()>
    {
        let query = texture.query();
        let mut canvas = canvas.borrow_mut();
        canvas.set_draw_color(self.theme_manager.borrow().grid_line_color());
        canvas.with_texture_canvas(texture, |canvas|{
            for i in 0 .. query.width as i32 {
                let x = i * self.scale as i32 + i;
                canvas.draw_line((x, 0), (x, query.height as i32)).unwrap();
            }
            for i in 0 .. query.height as i32 {
                let y = i * self.scale as i32 + i;
                canvas.draw_line((0, y), (query.width as i32, y)).unwrap();
            }
        })?;
        Ok(())
    }

    pub fn paint_grid_lines(&self, line_color: Color,
        canvas: Rc<RefCell<render::WindowCanvas>>, texture: &mut Texture)
    {
        let query = texture.query();
        let width = query.width;
        let height = query.height;
        let mut canvas = canvas.borrow_mut();
        for i in 0..width {
            
        }
    }

    fn get_rect(&self, x: i32, y: i32) -> Rect {
        Rect::new(x, y, self.scale as u32, self.scale as u32)
    }
}

pub struct Camera {
    widget_base: widget_base::WidgetBase,
    pos: (i32, i32),
    view_grid: ViewGrid,
    texture_cache: Option<Texture>,
}

impl Camera {
    pub fn new(widget_base: widget_base::WidgetBase, theme_manager: theme::RcThemeManager)
        -> Result<Self, error::DynError>
    {
        let pos = (0, 0);
        let cell_grid = grid::CellGrid::new(1000);
        let cell_grid = Rc::new(RefCell::new(cell_grid));
        let view_grid = ViewGrid::new(20, cell_grid, theme_manager.clone())?;
        
        Ok(Camera { widget_base, pos, view_grid, texture_cache: None })
    }
    
    fn fill_base_element(&self, texture: &mut Texture) -> Result<(), error::DynError> {
        let bg_color = {
            let theme_manager = self.widget_base.theme_manager.borrow();
            theme_manager.camera_bg_color()
        };

        self.widget_base.canvas.borrow_mut().with_texture_canvas(
            texture, |canvas| {
            canvas.set_draw_color(bg_color);
            canvas.clear();
        })?;
        self.view_grid.fill_grid(self.widget_base.canvas.clone(), texture)?;
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

