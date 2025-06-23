use crate::utils::error;
use sdl2::{
    pixels::{
        Color
    }
};

pub type RcThemeManager = std::rc::Rc<std::cell::RefCell<ThemeManager>>;

static TITLE_NAME: &'static str = "game of life";
static WINDOW_WIDTH: u32 = 1280;
static WINDOW_HEIGHT: u32 = 720;
static WINDOW_BG_COLOR: Color = Color::WHITE;
static CAMERA_BG_COLOR: Color = Color::GRAY;

#[derive(Debug)]
pub struct ThemeManager {
    title_name: String,
    window_init_width: u32,
    window_init_height: u32,
    window_bg_color: Color,
    camera_bg_color: Color,
    default_widget_color: Color,
}

impl ThemeManager {
    #[allow(unused)]
    pub fn default_init() -> Result<Self, error::DynError> {
        let theme_manager = ThemeManager {
            title_name: TITLE_NAME.to_string(),
            window_init_width: WINDOW_WIDTH,
            window_init_height: WINDOW_HEIGHT,
            window_bg_color: WINDOW_BG_COLOR,
            camera_bg_color: CAMERA_BG_COLOR,
            default_widget_color: WINDOW_BG_COLOR,
        };
        Ok(theme_manager)
    }

    pub fn title_name(&self) -> &str {
        self.title_name.as_str()
    }

    pub fn initial_width(&self) -> u32 {
        self.window_init_width
    }

    pub fn initial_height(&self) -> u32 {
        self.window_init_height
    }

    pub fn window_bg_color(&self) -> Color {
        self.window_bg_color.clone()
    }

    pub fn camera_bg_color(&self) -> Color {
        self.camera_bg_color.clone()
    }

    pub fn default_widget_color(&self) -> Color {
        self.default_widget_color.clone()
    }
}
