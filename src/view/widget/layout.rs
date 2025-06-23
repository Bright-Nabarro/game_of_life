use sdl2::{
    render,
    rect,
};

use crate::{
    utils::error,
    view::theme,
};
use super::{
    widget_base,
    camera,
};

use std::{
    rc::Rc,
    cell::RefCell,
};

pub struct Layout {
    direction: Direction,
    children: Vec<LayoutItem>,
    left_percentage: f32,
}

#[derive(PartialEq, Eq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

struct LayoutItem {
    content: LayoutContent,
    percentage: Option<f32>,
}

impl LayoutItem {
    fn new(content: LayoutContent, percentage: Option<f32>) -> Self
    {
        LayoutItem { content, percentage }
    }

    pub fn rendering(&mut self, canvas: Rc<RefCell<render::WindowCanvas>>,
        rect: rect::Rect) -> error::Result<()>
    {
        match &mut self.content {
            LayoutContent::Layout(layout) => {
                layout.rendering(canvas.clone(), rect)?;
            },
            LayoutContent::Widget(widget) => {
                let texture = widget.rendering_on_texture(rect.width(), rect.height())?;
                let mut ref_canvas = canvas.borrow_mut();
                ref_canvas.copy(texture, None, rect)?;
            },
        }
        Ok(())
    }
}

pub enum LayoutContent {
    Layout(Layout),
    Widget(Box<dyn widget_base::Widget>),
}


impl<T: widget_base::Widget + 'static> From<T> for LayoutContent {
    fn from(widget: T) -> Self {
        LayoutContent::Widget(Box::new(widget))
    }
}

impl From<Layout> for LayoutContent {
    fn from(layout: Layout) -> Self {
        LayoutContent::Layout(layout)
    }
}

impl LayoutContent {
    pub fn rendering(&self) {
    }
}

impl Layout {
    pub fn new(direction: Direction) -> Self {
        Layout {
            direction,
            children: Vec::new(),
            left_percentage: 1.,
        }
    }

    pub fn add_child<T: Into<LayoutContent>>(&mut self,
        content: T, hint_pct: Option<f32>)
    -> &mut Self
    {
        let percentage = match hint_pct {
            None => None,
            Some(pct) if pct > self.left_percentage => None,
            Some(pct) => {
                self.left_percentage -= pct;
                Some(pct)
            }
        };

        let layout_item = LayoutItem::new(content.into(), percentage);
        self.children.push(layout_item);
        self
    }

    fn compute_child_percentage(&self) -> Vec<f32>{
        let none_cnt = self.children
            .iter()
            .filter(|item| item.percentage.is_none())
            .count();
        
        let none_pec = if none_cnt > 0 {
            self.left_percentage / none_cnt as f32
        } else {
            0.
        };

        self.children
            .iter()
            .map(|item| item.percentage.unwrap_or(none_pec))
            .collect()
    }

    fn compute_child_rect(&self, rect: & rect::Rect) -> Vec<rect::Rect> {
        let percentages = self.compute_child_percentage();
        let is_horizontal = self.direction == Direction::Horizontal;
        let mut cur_pos = if is_horizontal { rect.y } else { rect.x };

        percentages
            .iter()
            .map(|pct| {
                if is_horizontal {
                    let height = (rect.height() as f32 * pct) as u32;
                    let width = rect.width();
                    let rect = rect::Rect::new(rect.x, cur_pos, width, height);
                    cur_pos += height as i32;
                    return rect;
                } else {
                    let width = (rect.width() as f32 * pct) as u32;
                    let height = rect.height();
                    let rect = rect::Rect::new(cur_pos, rect.y, width, height);
                    cur_pos += width as i32;
                    return rect;
                }
            })
            .collect()
     
        //let mut rect_vec = Vec::<rect::Rect>::with_capacity(each_pec.len());
        //if self.direction == Direction::Horizontal {
        //    let mut next_pos = rect.y;
        //    for pec in each_pec.iter() {
        //        let x = rect.x;
        //        let y = next_pos;
        //        let width = rect.width();
        //        let height = (rect.height() as f32 * pec) as u32;
        //        next_pos += height;
        //        rect_vec.push(rect::Rect::new(x, y, width, height));
        //    }
        //} else {
        //    let mut next_pos = rect.x;
        //    for pec in each_pec.iter() {
        //        let x = next_pos;
        //        let y = rect.y;
        //        let width = (rect.width() as f32 * pec) as u32;
        //        let height = rect.height();
        //        next_pos += width;
        //        rect_vec.push(rect::Rect::new(x, y, width, height));
        //    }
        //}
        //
        //rect_vec
    }

    pub fn rendering(&mut self, canvas: Rc<RefCell<render::WindowCanvas>>,
        rect: rect::Rect) -> error::Result<()>
    {
        let child_rects = self.compute_child_rect(&rect);
        child_rects
            .iter()
            .enumerate()
            .try_for_each(|(idx, rect)| -> error::Result<()>  {
                self.children[idx].rendering(canvas.clone(), rect.clone())?;
                Ok(())
            })?;
        
        Ok(())
    }
}


pub struct LayoutManager {
    widget_base_builder: widget_base::WidgetBaseBuilder,
}

impl LayoutManager {
    pub fn new(theme_manager: theme::RcThemeManager,
        canvas: Rc<RefCell<render::WindowCanvas>>) 
        -> Self
    {
        let widget_base_builder = widget_base::WidgetBaseBuilder::new(theme_manager, canvas);
        LayoutManager { widget_base_builder }
    }

    fn get_blank(&self) -> widget_base::BlankWidget {
        widget_base::BlankWidget::new(self.widget_base_builder.build(), true)
    }

    fn get_camera(&self) -> error::Result<camera::Camera> {
        camera::Camera::new(self.widget_base_builder.build(), 0.5)
    }

    pub fn get_default_layout(&self) -> error::Result<Layout> {
        let mut root_layout = Layout::new(Direction::Horizontal);
        let camera = self.get_camera()?;
        root_layout
            .add_child(self.get_blank(), None)
            .add_child(camera, Some(0.8))
            .add_child(self.get_blank(), None);
        Ok(root_layout)
    }
}
