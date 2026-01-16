
use raylib::prelude::*;
// use measurement::Angles; // To be implemented
use crate::game_data::GameData;
use crate::menu_scene::MenuScene;
use crate::scenes::{Scene, SceneSwitch};

pub struct GameScene{
    player_position: Vector2,
    player_direction: Vector2,  // change to angle, use trig to get movement
    player_speed: f32,
    player_acceleration: f32,
    player_top_speed: f32,
}

impl GameScene{
    pub fn new(player_position:Vector2, player_direction:Vector2) -> Self{
        Self{
            player_position:player_position,
            player_direction:player_direction,
            player_speed:0.0,
            player_acceleration:0.03,
            player_top_speed:50.0
        }
    }
}

impl Scene for GameScene{
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData){
        
    }

    fn handle_input(&mut self, rl: &mut RaylibHandle, _data: &mut GameData) -> SceneSwitch{

        if rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP){
            self.player_speed += self.player_acceleration;
        }
        else if rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_DOWN){
            self.player_speed -= self.player_acceleration;
        }

        if rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT){
            // Modify turn angle
        }
        else if rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_LEFT){
            // Modify turn angle
        }

        if rl.is_key_pressed(KeyboardKey::KEY_P){
            SceneSwitch::None; // TBD: Push to PauseScene
        }
        
        SceneSwitch::None

    }

    fn update(&mut self, _dt:f32, data:&mut GameData) -> SceneSwitch{

        SceneSwitch::None

    }

    fn draw(&self, d: &mut RaylibDrawHandle, _data: &mut GameData){
        d.clear_background(Color::WHITE);

        let center = Vector2::new( _data.screen_width as f32 / 2.0, _data.screen_height as f32 / 2.0);
        let radius = 200.0;
        let thickness = 75.0;
        let segments = 36;

        d.draw_ring(
            center,
            radius - thickness / 2.0, 
            radius + thickness / 2.0, 
            0.0,
            360.0, 
            segments,
            Color::GOLDENROD,
        );

        
        // d.draw_rectangle(200, 300, 300, 50, Color::RED);
        // d.draw_text("This is the game scene!", 210, 305, 20, Color::BLACK);

         let temp_player = Rectangle{ 
            x: 300.0,
            y: 20.0,
            width: 50.0,
            height: 20.0 
        };

        d.draw_rectangle_rounded(temp_player, 0.7, 12, Color::BLUEVIOLET);
    }

    fn on_exit(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData){}

}