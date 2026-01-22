
use raylib::prelude::*;
use crate::game_data::{CarChoice, ControlChoice, GameData, TrackChoice};
use crate::scenes::{Scene, SceneSwitch};

pub struct GameScene{
    player_position: Vector2,
    player_direction: f32,  // change to angle, use trig to get movement
    player_speed: f32,
    player_acceleration: f32,
    player_top_speed: f32,
    player_rot_vel: f32,
}

impl GameScene{
    pub fn new(player_position:Vector2, player_direction:f32) -> Self{
        Self{
            player_position:player_position,
            player_direction:player_direction,
            player_speed:0.0,
            player_acceleration:0.0,
            player_top_speed:400.0,
            player_rot_vel:0.0
        }
    }
}

impl Scene for GameScene{
    fn on_enter(&mut self, _rl: &mut RaylibHandle, data: &mut GameData, _thread: &RaylibThread){
            //Resetting timer
            data.race_time = 0.0;
            data.race_started = false;
    }

    fn handle_input(&mut self, rl: &mut RaylibHandle, data: &mut GameData, _thread: &RaylibThread) -> SceneSwitch{

        match data.selected_control{
            Some(ControlChoice::Keyboard) =>{
                self.player_acceleration = 0.0;
                if rl.is_key_down(
                    KeyboardKey::KEY_W) ||
                    rl.is_key_down(KeyboardKey::KEY_UP){
                        self.player_acceleration = 1.0;
                    }
                else if rl.is_key_down(
                    KeyboardKey::KEY_S) ||
                    rl.is_key_down(KeyboardKey::KEY_DOWN){
                        self.player_acceleration=-1.0;
                    }
                self.player_rot_vel = 0.0;
                if rl.is_key_down(
                    KeyboardKey::KEY_D) ||
                    rl.is_key_down(KeyboardKey::KEY_RIGHT){
                        self.player_rot_vel=1.0;
                    }
                else if rl.is_key_down(
                    KeyboardKey::KEY_A) ||
                    rl.is_key_down(KeyboardKey::KEY_LEFT){
                        self.player_rot_vel=-1.0;
                    }
            }
            Some(ControlChoice::Controller) =>{
                //TBD
            }
            None =>{ // Defaults to keyboard controls
                self.player_acceleration = 0.0;
                if rl.is_key_down(
                    KeyboardKey::KEY_W) ||
                    rl.is_key_down(KeyboardKey::KEY_UP){
                        self.player_acceleration = 1.0;
                    }
                else if rl.is_key_down(
                    KeyboardKey::KEY_S) ||
                    rl.is_key_down(KeyboardKey::KEY_DOWN){
                        self.player_acceleration=-1.0;
                    }
                self.player_rot_vel = 0.0;
                if rl.is_key_down(
                    KeyboardKey::KEY_D) ||
                    rl.is_key_down(KeyboardKey::KEY_RIGHT){
                        self.player_rot_vel=1.0;
                    }
                else if rl.is_key_down(
                    KeyboardKey::KEY_A) ||
                    rl.is_key_down(KeyboardKey::KEY_LEFT){
                        self.player_rot_vel=-1.0;
                    }
            }

        }


        if rl.is_key_pressed(KeyboardKey::KEY_P){
            println!("Pause");
        }
        
        SceneSwitch::None

    }

    fn update(&mut self, dt:f32, data:&mut GameData) -> SceneSwitch{

        // Start race timer when player starts moving
        if !data.race_started && self.player_speed.abs() > 1.0{
            data.race_started = true;
        }

        if data.race_started{
            data.race_time += dt;
        }

        // if self.player_speed<=self.player_top_speed{
        //    self.player_speed= f32::max(self.player_speed+self.player_acceleration,-0.5*self.player_top_speed);
        // } else {
        //     self.player_speed=self.player_top_speed;
        // }
        
        // let speed_delta=self.player_speed*_dt;
        // let accel_delta=self.player_acceleration*_dt;
        // let rotation_delta=self.player_rot_vel*_dt;
        // self.player_direction=(self.player_direction+rotation_delta)%360.0;
        // let dir_rad=self.player_direction.to_radians();
        // let velocity:Vector2=Vector2::new(dir_rad.cos(),dir_rad.sin());
        // self.player_position=self.player_position+velocity*speed_delta;
        
        let accel_rate=200.0;
        let brake_rate=300.0;
        let drag =4.0;
        let max_speed=self.player_top_speed;

        let accel= if self.player_acceleration>0.0{
            accel_rate
        } else if self.player_acceleration<0.0{
            -brake_rate
        } else{
            0.0
        };

        self.player_speed+=accel*dt;
        if accel==0.0{
            self.player_speed-=self.player_speed*drag*dt;
        }

        self.player_speed=self.player_speed.clamp(
            -0.5*max_speed,
            max_speed,
        );

        let steering_strength=self.player_rot_vel * 120.0;
        let speed_factor=self.player_speed.abs()/max_speed;

        self.player_direction=(self.player_direction+steering_strength*speed_factor*dt)%360.0;

        let dir_rad=self.player_direction.to_radians();
        let forward=Vector2::new(dir_rad.cos(),dir_rad.sin());

        self.player_position+=forward*self.player_speed*dt;

        SceneSwitch::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData){
        d.clear_background(Color::WHITE);

        let minutes = (data.race_time / 60.0).floor() as i32;
        let seconds = (data.race_time % 60.0) as i32;
        let milliseconds = ((data.race_time * 1000.0) % 1000.0) as i32;

        let timer_text = format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds);

        d.draw_text(
            &timer_text,
            10,
            10,
            30,
            Color::BLACK,
        );
    
        let car_rect = Rectangle{x:self.player_position.x, y:self.player_position.y, width:50.0, height:20.0};
        
        let center = Vector2::new( data.screen_width as f32 / 2.0, data.screen_height as f32 / 2.0);
        let radius = 200.0;
        let thickness = 75.0;
        let segments = 36;

        match data.selected_track{
            Some(TrackChoice::Track1) => d.draw_ring(
                        center,
                        radius - thickness / 2.0, 
                        radius + thickness / 2.0, 
                        0.0,
                        360.0, 
                        segments,
                        Color::KHAKI,
                    ),
            Some(TrackChoice::Track2) => d.draw_ring(
                        center,
                        radius - thickness / 2.0, 
                        radius + thickness / 2.0, 
                        0.0,
                        360.0, 
                        segments,
                        Color::FIREBRICK,
                    ),
            Some(TrackChoice::Track3) => d.draw_ring(
                        center,
                        radius - thickness / 2.0, 
                        radius + thickness / 2.0, 
                        0.0,
                        360.0, 
                        segments,
                        Color::PAPAYAWHIP,
                    ),
            Some(TrackChoice::Track4) => d.draw_ring(
                        center,
                        radius - thickness / 2.0, 
                        radius + thickness / 2.0, 
                        0.0,
                        360.0, 
                        segments,
                        Color::BURLYWOOD,
                    ),
            None => d.draw_ring(
                        center,
                        radius - thickness / 2.0, 
                        radius + thickness / 2.0, 
                        0.0,
                        360.0, 
                        segments,
                        Color::GOLDENROD,
                    )
            }

        match data.selected_car{
            Some(CarChoice::Car1) => d.draw_rectangle_pro(
                car_rect,Vector2{x:car_rect.width/2.0,y:car_rect.width/2.0},
                self.player_direction,
                Color::BLUEVIOLET),
            Some(CarChoice::Car2) => d.draw_rectangle_pro(
                car_rect,Vector2{x:car_rect.width/2.0,y:car_rect.width/2.0},
                self.player_direction,
                Color::NAVY),
            Some(CarChoice::Car3) => d.draw_rectangle_pro(
                car_rect,Vector2{x:car_rect.width/2.0,y:car_rect.width/2.0},
                self.player_direction,
                Color::BROWN),
            Some(CarChoice::Car4) => d.draw_rectangle_pro(
                car_rect,Vector2{x:car_rect.width/2.0,y:car_rect.width/2.0},
                self.player_direction,
                Color::DARKORCHID),
            None => d.draw_rectangle_pro(
                car_rect,Vector2{x:car_rect.width/2.0,y:car_rect.width/2.0},
                self.player_direction,
                Color::GREEN),
        }
    }

    fn on_exit(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData, _thread: &RaylibThread){}

}