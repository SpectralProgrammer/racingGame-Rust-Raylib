pub struct GameData {
    pub laps: u32,
    pub screen_width: i32,
    pub screen_height:i32,
}

impl GameData{
    pub fn new(width: i32, height:i32) -> Self{
        Self{
            laps:0,
            screen_width: width,
            screen_height: height
        }
    }

    pub fn lap(&mut self){
        self.laps +=1;
    }
}