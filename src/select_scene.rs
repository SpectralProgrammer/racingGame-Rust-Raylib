use raylib::prelude::*;

use crate::game_data::{CarChoice, GameData, TrackChoice};
use crate::game_scene::GameScene;
use crate::scenes::{Scene, SceneSwitch};
use crate::utils::*;

pub struct SelectScene {
    background_texture: Option<Texture2D>,
    car_rects: [Rectangle; 4],
    track_rects: [Rectangle; 4],
    play_rect: Rectangle,
}

impl SelectScene {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        screen_width: i32,
        _screen_height: i32,
    ) -> Self {
        let background_texture = rl
            .load_texture(thread, "Assets/selectBack2.png")
            .expect("Failed to load select background image");

        // Layout variables
        let button_size = 100.0;
        let button_spacing = 20.0;
        let buttons_per_row = 4.0;
        let row_width = buttons_per_row * button_size + (buttons_per_row - 1.0) * button_spacing;
        let row_start_x = (screen_width as f32 - row_width) / 2.0;

        // Track positions
        let track_y = 225.0;
        let mut track_rects = [Rectangle::new(0.0, 0.0, 0.0, 0.0); 4];
        for i in 0..4 {
            track_rects[i] = Rectangle::new(
                row_start_x + i as f32 * (button_size + button_spacing),
                track_y,
                button_size,
                button_size,
            );
        }

        // Car positions
        let car_y = 400.0;
        let mut car_rects = [Rectangle::new(0.0, 0.0, 0.0, 0.0); 4];
        for i in 0..4 {
            car_rects[i] = Rectangle::new(
                row_start_x + i as f32 * (button_size + button_spacing),
                car_y,
                button_size,
                button_size,
            );
        }

        // Play button
        let play_width = 375.0;
        let play_height = 50.0;
        let play_rect = Rectangle::new(
            screen_width as f32 / 2.0 - play_width / 2.0,
            540.0,
            play_width,
            play_height,
        );

        Self {
            background_texture: Some(background_texture),
            car_rects,
            track_rects,
            play_rect,
        }
    }
}

impl Scene for SelectScene {
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData, _thread: &RaylibThread) {}

    fn handle_input(
        &mut self,
        rl: &mut RaylibHandle,
        data: &mut GameData,
        thread: &RaylibThread,
    ) -> SceneSwitch {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let click = rl.get_mouse_position();

            // Play button
            if check_collision_point_rect(&click, &self.play_rect) {
                println!("Play button clicked");
                return SceneSwitch::Push(Box::new(GameScene::new(
                    rl,
                    thread,
                    Vector2::new(100.0, 100.0),
                    90.0,
                )));
            }

            // Track selection
            for (i, rect) in self.track_rects.iter().enumerate() {
                if check_collision_point_rect(&click, rect) {
                    data.selected_track = Some(match i {
                        0 => TrackChoice::Track1,
                        1 => TrackChoice::Track2,
                        2 => TrackChoice::Track3,
                        _ => TrackChoice::Track4,
                    });
                    println!("Track {} selected", i + 1);
                }
            }

            // Car selection
            for (i, rect) in self.car_rects.iter().enumerate() {
                if check_collision_point_rect(&click, rect) {
                    data.selected_car = Some(match i {
                        0 => CarChoice::Car1,
                        1 => CarChoice::Car2,
                        2 => CarChoice::Car3,
                        _ => CarChoice::Car4,
                    });
                    println!("Car {} selected", i + 1);
                }
            }
        }

        SceneSwitch::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData) {
        // Clear background
        d.clear_background(Color::WHITESMOKE);

        let screen_w = data.screen_width as f32;
        let screen_center_x = screen_w / 2.0;

        // Draw background texture
        if let Some(texture) = &self.background_texture {
            let tex_w = texture.width as f32;
            let tex_h = texture.height as f32;
            let win_w = data.screen_width as f32;
            let win_h = data.screen_height as f32;

            let scale = (win_w / tex_w).max(win_h / tex_h);
            let dest_w = tex_w * scale;
            let dest_h = tex_h * scale;
            let dest_x = (win_w - dest_w) / 2.0;
            let dest_y = (win_h - dest_h) / 2.0;

            d.draw_texture_pro(
                texture,
                Rectangle::new(0.0, 0.0, tex_w, tex_h),
                Rectangle::new(dest_x, dest_y, dest_w, dest_h),
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }

        let default_color = Color::BURLYWOOD;
        let selected_color = Color::LEMONCHIFFON;

        // Track buttons
        let track_title = "Track Select";
        let track_title_size = 40;
        let track_title_width = d.measure_text(track_title, track_title_size);
        d.draw_text(
            track_title,
            (screen_center_x - track_title_width as f32 / 2.0) as i32,
            170,
            track_title_size,
            Color::BLACK,
        );

        for (i, rect) in self.track_rects.iter().enumerate() {
            let track = match i {
                0 => TrackChoice::Track1,
                1 => TrackChoice::Track2,
                2 => TrackChoice::Track3,
                _ => TrackChoice::Track4,
            };
            let color = if data.selected_track == Some(track) {
                selected_color
            } else {
                default_color
            };
            d.draw_rectangle_rounded(*rect, 0.4, 12, color);

            let label = format!("Track {}", i + 1);
            let text_w = d.measure_text(&label, 20);
            d.draw_text(
                &label,
                (rect.x + (rect.width - text_w as f32) / 2.0) as i32,
                (rect.y + (rect.height - 20.0) / 2.0) as i32,
                20,
                Color::BLACK,
            );
        }

        // Car buttons
        let car_title = "Car Select";
        let car_title_size = 40;
        let car_title_width = d.measure_text(car_title, car_title_size);
        d.draw_text(
            car_title,
            (screen_center_x - car_title_width as f32 / 2.0) as i32,
            350,
            car_title_size,
            Color::BLACK,
        );

        for (i, rect) in self.car_rects.iter().enumerate() {
            let car = match i {
                0 => CarChoice::Car1,
                1 => CarChoice::Car2,
                2 => CarChoice::Car3,
                _ => CarChoice::Car4,
            };
            let color = if data.selected_car == Some(car) {
                selected_color
            } else {
                default_color
            };
            d.draw_rectangle_rounded(*rect, 0.4, 12, color);

            let label = format!("Car {}", i + 1);
            let text_w = d.measure_text(&label, 20);

            d.draw_text(
                &label,
                (rect.x + (rect.width - text_w as f32) / 2.0) as i32,
                (rect.y + (rect.height - 20.0) / 2.0) as i32,
                20,
                Color::BLACK,
            );
        }

        // Play button
        d.draw_rectangle_rounded(self.play_rect, 0.4, 12, Color::BURLYWOOD);
        let play_text = "Play";
        let play_text_size = 30;
        let play_text_width = d.measure_text(play_text, play_text_size);

        d.draw_text(
            play_text,
            (self.play_rect.x + (self.play_rect.width - play_text_width as f32) / 2.0) as i32,
            (self.play_rect.y + (self.play_rect.height - play_text_size as f32) / 2.0) as i32,
            play_text_size,
            Color::BLACK,
        );
    }

    fn on_exit(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData, _thread: &RaylibThread) {}

    fn update(&mut self, _dt: f32, _data: &mut GameData) -> SceneSwitch {
        SceneSwitch::None
    }
}
