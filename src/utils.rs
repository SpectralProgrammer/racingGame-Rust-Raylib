
use raylib::prelude::*;
use rand::Rng;


pub fn check_collision_point_rect(point: &Vector2, rect: &Rectangle) -> bool{
    let in_x = point.x >= rect.x && point.x <= rect.x + rect.width;
    let in_y = point.y >= rect.y && point.y <= rect.y + rect.height;
    
    return in_x && in_y;
}

pub fn random_point(width: i32, height: i32) -> Vector2{
    Vector2::new(0.0,0.0)
}