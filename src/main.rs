mod controller;
mod utils;
mod view;

use controller::game_controller;
use utils::error;

use crate::controller::{
    game_controller::{
        GameContext,
        GameManager,
    },
};

fn main() -> Result<(), error::DynError>{
    let mut game_manager =  {
        let game_context = GameContext::init()?;
        GameManager::new(game_context)?
    };
    
    game_manager.main_loop()?;
    
    Ok(())
}

