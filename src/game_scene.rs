
use raylib::prelude::*;
use crate::game_data::{CarChoice, ControlChoice, GameData, TrackChoice};
use crate::scenes::{Scene, SceneSwitch};

pub struct GameScene{
    player_position: Vector2,
    player_direction: f32,  
    player_speed: f32,
    player_acceleration: f32,
    player_rot_vel: f32,
    track_texture: Option<Texture2D>,
    track_image: Option<Image>
}

impl GameScene{
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, player_position:Vector2, player_direction:f32) -> Self{

        let track_image = Image::load_image("Assets/track1.png")
        .expect("Failed to load track image");

        let track_texture = rl
        .load_texture(thread, "Assets/track1.png")
        .expect("Failed to load track image");

        Self{
            player_position:player_position,
            player_direction:player_direction,
            player_speed:0.0,
            player_acceleration:0.0,
            player_rot_vel:0.0,
            track_texture: Some(track_texture),
            track_image: Some(track_image)
        }
    }

//     fn is_off_track(&self, data: &GameData) -> bool {
//     let image = match self.track_image.as_ref(){
//         Some(img) => img,
//         None => return false,
//     };

//     let tex_w = image.width as f32;
//     let tex_h = image.height as f32;
//     let win_w = data.screen_width as f32;
//     let win_h = data.screen_height as f32;

//     let scale = (win_w / tex_w).max(win_h / tex_h);
//     let dest_w = (tex_w * scale) + 450.0;
//     let dest_h = (tex_h * scale) + 450.0;

//     let dest_x = (win_w - dest_w) / 2.0;
//     let dest_y = (win_h - dest_h) / 2.0;

//     // Map player screen position to image pixel coordinates
//     let u = ((self.player_position.x - dest_x) / dest_w) * tex_w;
//     let v = ((self.player_position.y - dest_y) / dest_h) * tex_h;

//     // Bounds check
//     if u < 0.0 || v < 0.0 || u >= tex_w || v >= tex_h {
//         return true; 
//     }

//     // Safe color lookup
//     let pixel = image.get_color(u as i32, v as i32);

//     // Check if the pixel is "black" (off-track)
//     pixel.r < 20 && pixel.g < 20 && pixel.b < 20
// }

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

        let (accel_rate,brake_rate,drag,max_speed,handling)=match data.selected_car{
            None | Some(CarChoice::Car1)=> (200.0,300.0,4.0,400.0,120.0),
            Some(CarChoice::Car2)=>(100.0,100.0,1.0,1000.0,240.0),
            Some(CarChoice::Car3)=>(500.0,500.0,8.0,200.0,300.0),
            Some(CarChoice::Car4)=>(200.0,300.0,4.0,400.0,120.0),
        };

        

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

        let steering_strength=self.player_rot_vel * handling;
        let speed_factor=(self.player_speed.abs()/max_speed).clamp(0.2,1.0);

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
            None => {
                    // Resizing the background image to fill screen
                    if let Some(texture) = &self.track_texture{
                        let tex_w = texture.width as f32;
                        let tex_h = texture.height as f32;

                        let win_w = data.screen_width as f32;
                        let win_h = data.screen_height as f32;

                        let scale = (win_w / tex_w).max(win_h / tex_h);

                        let dest_w = (tex_w * scale) + 450.0;
                        let dest_h = (tex_h * scale) + 450.0;

                        let dest_x = (win_w - dest_w) / 2.0;
                        let dest_y = (win_h - dest_h) / 2.0;

                        let source = Rectangle::new(0.0, 0.0, tex_w, tex_h);
                        let dest = Rectangle::new(dest_x, dest_y, dest_w, dest_h);

                        d.draw_texture_pro(
                            texture,
                            source,
                            dest,
                            Vector2::zero(),
                            0.0,
                            Color::WHITE,
                        );
                    }
                }
            }

        match data.selected_car{
            Some(CarChoice::Car1) | None=> {
                d.draw_rectangle_pro(
                car_rect,Vector2{x:car_rect.width/2.0,y:car_rect.width/2.0},
                self.player_direction,
                Color::BLUEVIOLET);
                d.draw_text("Default car",data.screen_width-200,data.screen_height-50,25,Color::WHITE)
            },
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
        }

        d.draw_text(
            &timer_text,
            10,
            10,
            30,
            Color::WHITE,
        );
    }

    fn on_exit(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData, _thread: &RaylibThread){}

}