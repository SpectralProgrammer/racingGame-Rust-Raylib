use crate::game_data::{CarChoice, ControlChoice, GameData, TrackChoice};
use crate::scenes::{Scene, SceneSwitch};
use raylib::prelude::*;

const CAR_SCALE: f32 = 0.05;
const CAR_SPRITE_ROT_OFFSET: f32 = -90.0; // sprite artwork faces up

struct CarSprites {
    straight: Texture2D,
    left: Texture2D,
    right: Texture2D,
}

pub struct GameScene {
    player_position: Vector2,
    player_direction: f32,
    player_speed: f32,
    player_acceleration: f32,
    player_rot_vel: f32,

    track_texture: Option<Texture2D>,
    track_image: Option<Image>,
    car_sprites: Option<CarSprites>,
}

impl GameScene {
    pub fn new(
        _rl: &mut RaylibHandle,
        _thread: &RaylibThread,
        player_position: Vector2,
        player_direction: f32,
    ) -> Self {
        Self {
            player_position,
            player_direction,
            player_speed: 0.0,
            player_acceleration: 0.0,
            player_rot_vel: 0.0,
            track_texture: None,
            track_image: None,
            car_sprites: None,
        }
    }

    // -------- Find first white pixel on track --------
    fn find_spawn_pixel(
        image: &mut Image,
        screen_w: f32,
        screen_h: f32,
        tex_w: f32,
        tex_h: f32,
    ) -> Vector2 {
        let scale = (screen_w / tex_w).max(screen_h / tex_h);

        for y in 0..image.height {
            for x in 0..image.width {
                let c = image.get_color(x, y);

                if c.r > 200 && c.g > 200 && c.b > 200 {
                    return Vector2 {
                        x: (x as f32 * scale) + (screen_w - tex_w * scale) / 2.0,
                        y: (y as f32 * scale) + (screen_h - tex_h * scale) / 2.0 + 35.0,
                    };
                }
            }
        }

        Vector2 {
            x: screen_w / 2.0,
            y: screen_h / 2.0,
        }
    }

    // -------- Offset spawn so entire car is on white --------
    fn offset_spawn_for_car(spawn: Vector2, direction_deg: f32, car_tex: &Texture2D) -> Vector2 {
        let half_len = (car_tex.width as f32 * CAR_SCALE) / 2.0;
        let rad = direction_deg.to_radians();
        let backward = Vector2::new(rad.cos(), rad.sin());

        spawn - backward * half_len
    }

    fn load_track(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        track: &TrackChoice,
        data: &GameData,
    ) {
        let path = track_path(track);

        let mut image = Image::load_image(path).expect("Failed to load track image");

        let texture = rl
            .load_texture(thread, path)
            .expect("Failed to load track texture");

        let spawn = Self::find_spawn_pixel(
            &mut image,
            data.screen_width as f32,
            data.screen_height as f32,
            texture.width as f32,
            texture.height as f32,
        );

        self.player_position = spawn;
        self.player_direction = 180.0; // FACE LEFT

        self.track_image = Some(image);
        self.track_texture = Some(texture);
    }

    fn load_car_sprites(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, car: &CarChoice) {
        let (folder, base) = match car {
            CarChoice::Car1 => ("Assets/frames/car1", "1"),
            CarChoice::Car2 => ("Assets/frames/car2", "2"),
            CarChoice::Car3 => ("Assets/frames/car3", "3"),
            CarChoice::Car4 => ("Assets/frames/car4", "4"),
        };

        self.car_sprites = Some(CarSprites {
            straight: rl
                .load_texture(thread, &format!("{}/{}.png", folder, base))
                .unwrap(),
            left: rl
                .load_texture(thread, &format!("{}/{}_left.png", folder, base))
                .unwrap(),
            right: rl
                .load_texture(thread, &format!("{}/{}_right.png", folder, base))
                .unwrap(),
        });
    }
}

impl Scene for GameScene {
    fn on_enter(&mut self, rl: &mut RaylibHandle, data: &mut GameData, thread: &RaylibThread) {
        data.race_time = 0.0;
        data.race_started = false;

        if let Some(track) = &data.selected_track {
            self.load_track(rl, thread, track, data);
        }

        let car = data.selected_car.unwrap_or(CarChoice::Car1);
        self.load_car_sprites(rl, thread, &car);

        // ---- Final spawn correction (FULLY ON WHITE) ----
        if let Some(sprites) = &self.car_sprites {
            self.player_position = Self::offset_spawn_for_car(
                self.player_position,
                self.player_direction,
                &sprites.straight,
            );
        }
    }

    fn handle_input(
        &mut self,
        rl: &mut RaylibHandle,
        data: &mut GameData,
        _thread: &RaylibThread,
    ) -> SceneSwitch {
        self.player_acceleration = 0.0;
        self.player_rot_vel = 0.0;

        match data.selected_control {
            Some(ControlChoice::Keyboard) | None => {
                if rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP) {
                    self.player_acceleration = 1.0;
                } else if rl.is_key_down(KeyboardKey::KEY_S)
                    || rl.is_key_down(KeyboardKey::KEY_DOWN)
                {
                    self.player_acceleration = -1.0;
                }

                if rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                    self.player_rot_vel = 1.0;
                } else if rl.is_key_down(KeyboardKey::KEY_A)
                    || rl.is_key_down(KeyboardKey::KEY_LEFT)
                {
                    self.player_rot_vel = -1.0;
                }
            }
            Some(ControlChoice::Controller) => {}
        }

        SceneSwitch::None
    }

    fn update(&mut self, dt: f32, data: &mut GameData) -> SceneSwitch {
        if !data.race_started && self.player_speed.abs() > 1.0 {
            data.race_started = true;
        }

        if data.race_started {
            data.race_time += dt;
        }

        let (accel_rate, brake_rate, drag, max_speed, handling) = match data.selected_car {
            None | Some(CarChoice::Car1) => (200.0, 300.0, 4.0, 400.0, 120.0),
            Some(CarChoice::Car2) => (100.0, 100.0, 1.0, 1000.0, 240.0),
            Some(CarChoice::Car3) => (500.0, 500.0, 8.0, 200.0, 300.0),
            Some(CarChoice::Car4) => (600.0, 200.0, 4.0, 400.0, 60.0),
        };

        let accel = if self.player_acceleration > 0.0 {
            accel_rate
        } else if self.player_acceleration < 0.0 {
            -brake_rate
        } else {
            0.0
        };

        self.player_speed += accel * dt;
        if accel == 0.0 {
            self.player_speed -= self.player_speed * drag * dt;
        }

        self.player_speed = self.player_speed.clamp(-0.5 * max_speed, max_speed);

        let steering = self.player_rot_vel * handling;
        let speed_factor = (self.player_speed.abs() / max_speed).clamp(0.4, 1.0);

        self.player_direction = (self.player_direction + steering * speed_factor * dt) % 360.0;

        let rad = self.player_direction.to_radians();
        let forward = Vector2::new(rad.cos(), rad.sin());

        self.player_position += forward * self.player_speed * dt;

        SceneSwitch::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData) {
        d.clear_background(Color::BLACK);

        let minutes = (data.race_time / 60.0).floor() as i32;
        let seconds = (data.race_time % 60.0) as i32;
        let milliseconds = ((data.race_time * 1000.0) % 1000.0) as i32;

        let timer_text = format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds);

        if let Some(track) = &self.track_texture {
            let tex_w = track.width as f32;
            let tex_h = track.height as f32;
            let win_w = data.screen_width as f32;
            let win_h = data.screen_height as f32;
            let scale = (win_w / tex_w).max(win_h / tex_h);

            d.draw_texture_pro(
                track,
                Rectangle::new(0.0, 0.0, tex_w, tex_h),
                Rectangle {
                    x: (win_w - tex_w * scale) / 2.0,
                    y: (win_h - tex_h * scale) / 2.0,
                    width: tex_w * scale,
                    height: tex_h * scale,
                },
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }

        let sprites = self.car_sprites.as_ref().unwrap();
        let tex = if self.player_rot_vel > 0.2 {
            &sprites.right
        } else if self.player_rot_vel < -0.2 {
            &sprites.left
        } else {
            &sprites.straight
        };

        let dest = Rectangle {
            x: self.player_position.x,
            y: self.player_position.y,
            width: tex.width as f32 * CAR_SCALE,
            height: tex.height as f32 * CAR_SCALE,
        };

        d.draw_texture_pro(
            tex,
            Rectangle::new(0.0, 0.0, tex.width as f32, tex.height as f32),
            dest,
            Vector2 {
                x: dest.width / 2.0,
                y: dest.height / 2.0,
            },
            self.player_direction + CAR_SPRITE_ROT_OFFSET,
            Color::WHITE,
        );

        d.draw_text(
            &timer_text,
            10, // X (top-left)
            10, // Y
            30, // font size
            Color::WHITE,
        );

        let car_name = match data.selected_car {
            None | Some(CarChoice::Car1) => "Default car",
            Some(CarChoice::Car2) => "High inertia car",
            Some(CarChoice::Car3) => "Responsive car",
            Some(CarChoice::Car4) => "Stubborn car",
        };

        let font_size = 25;
        let padding = 10;

        // Measure text width so it aligns to bottom right
        let text_width = d.measure_text(car_name, font_size);

        let x = data.screen_width - text_width - padding;
        let y = data.screen_height - font_size - padding;

        d.draw_text(car_name, x, y, font_size, Color::WHITE);
    }

    fn on_exit(&mut self, _: &mut RaylibHandle, _: &mut GameData, _: &RaylibThread) {}
}

fn track_path(track: &TrackChoice) -> &'static str {
    match track {
        TrackChoice::Track1 => "Assets/track1.png",
        TrackChoice::Track2 => "Assets/track2.png",
        TrackChoice::Track3 => "Assets/track3.png",
        TrackChoice::Track4 => "Assets/track4.png",
    }
}