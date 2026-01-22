use raylib::prelude::*;

use crate::game_data::GameData;
use crate::scenes::{Scene, SceneSwitch};
use crate::select_scene::SelectScene;
use crate::settings_scene::SettingsScene;
use crate::utils::*;

pub struct MenuScene {
    title_texture: Option<Texture2D>,
    background_texture: Option<Texture2D>,
}

impl MenuScene {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let title_texture = rl
            .load_texture(thread, "Assets/title_image.png")
            .expect("Failed to load title image");

        let menu_texture = rl
            .load_texture(thread, "Assets/MenuBack.png")
            .expect("Failed to load menu background image");

        Self {
            title_texture: Some(title_texture),
            background_texture: Some(menu_texture),
        }
    }
}

impl Scene for MenuScene {
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData, _thread: &RaylibThread) {}

    fn handle_input(
        &mut self,
        rl: &mut RaylibHandle,
        data: &mut GameData,
        thread: &RaylibThread,
    ) -> SceneSwitch {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let click = rl.get_mouse_position();

            let center_x = data.screen_width as f32 / 2.0;
            let button_width = 320.0;
            let button_height = 50.0;
            let button_spacing = 20.0;
            let first_button_y = 360.0;

            let play_button_rectangle = Rectangle::new(
                (center_x - button_width / 2.0) as f32,
                first_button_y + 30.0,
                button_width,
                button_height,
            );
            let settings_button_rectangle = Rectangle::new(
                play_button_rectangle.x,
                play_button_rectangle.y + button_height + button_spacing,
                button_width,
                button_height,
            );

            if check_collision_point_rect(&click, &play_button_rectangle) {
                println!("Play button clicked");
                let select_scene =
                    SelectScene::new(rl, thread, data.screen_width, data.screen_height);
                return SceneSwitch::Push(Box::new(select_scene));
            } else if check_collision_point_rect(&click, &settings_button_rectangle) {
                println!("Settings button clicked");
                let settings_scene = SettingsScene::new(rl, thread);
                return SceneSwitch::Push(Box::new(settings_scene));
            }
        }

        SceneSwitch::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData) {
        d.clear_background(Color::WHITE);

        let center_x = data.screen_width as f32 / 2.0;
        let button_width = 320.0;
        let button_height = 50.0;
        let button_spacing = 20.0;
        let title_y = 45.0;
        let first_button_y = 360.0;

        // Resizing the background image to fill screen
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

            let source = Rectangle::new(0.0, 0.0, tex_w, tex_h);
            let dest = Rectangle::new(dest_x, dest_y, dest_w, dest_h);

            d.draw_texture_pro(texture, source, dest, Vector2::zero(), 0.0, Color::WHITE);
        }

        let play_button = Rectangle {
            x: center_x - button_width / 2.0,
            y: first_button_y + 30.0,
            width: button_width,
            height: button_height,
        };

        let settings_button = Rectangle {
            x: play_button.x,
            y: play_button.y + button_height + button_spacing,
            width: button_width,
            height: button_height,
        };

        if let Some(texture) = &self.title_texture {
            let scale = 1.2;

            let tex_w = texture.width as f32 * scale;
            let x = center_x - tex_w / 2.0;

            d.draw_texture_ex(texture, Vector2::new(x, title_y), 0.0, scale, Color::WHITE);
        }

        let play_text = "Play";
        let play_text_size = 30;
        let play_text_width = d.measure_text(play_text, play_text_size);

        let settings_text = "Settings";
        let settings_text_size = 30;
        let settings_text_width = d.measure_text(settings_text, settings_text_size);

        d.draw_rectangle_rounded(play_button, 0.4, 12, Color::BURLYWOOD);
        d.draw_text(
            play_text,
            (play_button.x + (button_width - play_text_width as f32) / 2.0) as i32,
            (play_button.y + (button_height - play_text_size as f32) / 2.0) as i32,
            play_text_size,
            Color::BLACK,
        );

        d.draw_rectangle_rounded(settings_button, 0.4, 12, Color::BURLYWOOD);
        d.draw_text(
            settings_text,
            (settings_button.x + (button_width - settings_text_width as f32) / 2.0) as i32,
            (settings_button.y + (button_height - settings_text_size as f32) / 2.0) as i32,
            settings_text_size,
            Color::BLACK,
        );
    }

    fn on_exit(&mut self, rl: &mut RaylibHandle, _data: &mut GameData, thread: &RaylibThread) {
        if let Some(texture) = self.title_texture.take() {
            unsafe { rl.unload_texture(thread, texture.make_weak()) };
        }
    }

    fn update(&mut self, _dt: f32, _data: &mut GameData) -> SceneSwitch {
        SceneSwitch::None
    }
}
