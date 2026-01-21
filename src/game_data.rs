
#[derive(Copy, Clone, PartialEq)]
pub enum ControlChoice{
    Keyboard, Controller
}

#[derive(Copy, Clone, PartialEq)]
pub enum CarChoice{
    Car1, Car2, Car3, Car4
}

#[derive(Copy, Clone, PartialEq)]
pub enum TrackChoice{
    Track1, Track2, Track3, Track4
}

pub struct GameData {
    pub laps: u32,
    pub screen_width: i32,
    pub screen_height:i32,

    pub selected_car: Option<CarChoice>,
    pub selected_track: Option<TrackChoice>,

    pub selected_control: Option<ControlChoice>
}

impl GameData{
    pub fn new(width: i32, height:i32) -> Self{
        Self{
            laps:0,
            screen_width: width,
            screen_height: height,
            selected_car: None,
            selected_track: None,
            selected_control: None
        }
    }
}