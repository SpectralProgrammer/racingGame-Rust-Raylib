use raylib::prelude::*;

use crate::game_data::GameData;
use crate::menu_scene::MenuScene;
use crate::scenes::{Scene, SceneSwitch};

pub struct GameScene{
    player_position: Vector2,
    player_direction: Vector2,
    player_speed: f32,
    player_acceleration: f32,
    player_top_speed: f32,
    
}

impl GameScene{
    pub fn new(player_position:Vector2, player_direction:Vector2) -> Self {
        Self{
            player_position:player_position,
            player_direction:player_direction,
            player_speed:0.0,
            player_acceleration:10.0,
            player_top_speed:50.0
        }
    }
}

impl Scene for GameScene{
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData){

    }

    fn handle_input(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData)-> SceneSwitch{

    }

    fn update(&mut self, _dt:f32, data:&mut GameData)->SceneSwitch{

    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData){

    }

    fn on_exit(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData){}
}