use raylib::prelude::*;

use crate::game_data::GameData;

pub enum SceneSwitch{
    None,
    Push(Box<dyn Scene>),
    Replace(Box<dyn Scene>),
    Pop,
    Quit,
}

pub trait Scene{
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _date:&mut GameData){}

    fn handle_input(&mut self, _rl:&mut RaylibHandle, _data:&mut GameData)->SceneSwitch {
        SceneSwitch::None
    }

    fn update(&mut self, _dt:f32, _data: &mut GameData)->SceneSwitch{
        SceneSwitch::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData);

    fn on_exit(&mut self, _rl:&mut RaylibHandle, _data:&mut GameData){}
}