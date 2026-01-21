
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
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _date:&mut GameData, _thread: &RaylibThread){}

    fn handle_input(&mut self, _rl:&mut RaylibHandle, _data:&mut GameData, _thread: &RaylibThread) -> SceneSwitch {
        SceneSwitch::None
    }

    fn update(&mut self, _dt:f32, _data: &mut GameData)->SceneSwitch{
        SceneSwitch::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData);

    fn on_exit(&mut self, _rl:&mut RaylibHandle, _data:&mut GameData, _thread: &RaylibThread){}
}

pub struct SceneManager{
    scenes: Vec<Box<dyn Scene>>,
    quit: bool
}

impl SceneManager{
    pub fn new(rl: &mut RaylibHandle, initial: Box<dyn Scene>, data: &mut GameData, thread: &RaylibThread) -> Self{
        let mut manager = Self{
            scenes: vec![initial],
            quit: false
        };
        manager.scenes.last_mut().unwrap().on_enter(rl, data, thread);
        manager
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, dt: f32, data: &mut GameData, thread: &RaylibThread){
        if let Some(scene) = self.scenes.last_mut(){
            let switch = scene.handle_input(rl, data, thread);
            self.apply_switch(switch, rl, data, thread);
        }

        if let Some(scene) = self.scenes.last_mut(){
            let switch = scene.update(dt, data);
            self.apply_switch(switch, rl, data, thread);
        }
    }

    pub fn draw(&self, d:&mut RaylibDrawHandle, data:&mut GameData){
        if let Some(scene)=self.scenes.last(){
            scene.draw(d,data);
        }
    }

    pub fn apply_switch(&mut self, switch:SceneSwitch, rl:&mut RaylibHandle, data: &mut GameData, thread: &RaylibThread){
        match switch{
            SceneSwitch::None=>{},
            SceneSwitch::Push(mut scene)=>{
                scene.on_enter(rl,data, thread);
                self.scenes.push(scene);
            },
            SceneSwitch::Replace(mut scene)=>{
                if let Some(mut old_scene)=self.scenes.pop(){
                    old_scene.on_exit(rl,data, thread);
                }
                scene.on_enter(rl,data, thread);
                self.scenes.push(scene);
            }
            SceneSwitch::Pop=>{
                if let Some(mut old_scene)=self.scenes.pop(){
                    old_scene.on_exit(rl,data, thread);
                }
            },
            SceneSwitch::Quit=>{
                self.quit=true;
            }
        }
    }
    
    pub fn should_quit(&self) -> bool{
        self.quit || self.scenes.is_empty()
    }
    
}