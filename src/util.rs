use crate::behavior::ItemBehavior;
use crate::config::{MOVE_FAST_VELOCITY, MOVE_HIDE_VELOCITY, MOVE_VELOCITY};
use crate::{Angle, ScenePoint};
use std::f32::consts::FRAC_PI_2;

pub struct Rectangle {
    pub top_left: ScenePoint,
    pub top_right: ScenePoint,
    pub bottom_left: ScenePoint,
    pub bottom_right: ScenePoint,
}

pub fn velocity_for_behavior(behavior: &ItemBehavior) -> Option<f32> {
    match behavior {
        ItemBehavior::MoveTo(_, _) => Some(MOVE_VELOCITY),
        ItemBehavior::MoveFastTo(_, _) => Some(MOVE_FAST_VELOCITY),
        ItemBehavior::HideTo(_, _) => Some(MOVE_HIDE_VELOCITY),
        _ => None,
    }
}

pub fn angle(to_point: &ScenePoint, from_point: &ScenePoint) -> Angle {
    f32::atan2(to_point.y - from_point.y, to_point.x - from_point.x) + FRAC_PI_2
}
