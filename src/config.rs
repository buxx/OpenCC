use crate::FrameI;

// execute update code 60x per seconds
pub const TARGET_FPS: u32 = 60;
// execute physics code each 10 frames
pub const PHYSICS_EACH: u32 = 10;
// execute animate code each 15 frames
pub const ANIMATE_EACH: u32 = 15;
// execute seek code each 60 frames
pub const SEEK_EACH: u32 = 60;
// change sprite animation tile each 10 frames
pub const SPRITE_EACH: u32 = 10;
// compute interior sprites each 60 frames
pub const INTERIORS_EACH: u32 = 60;
// max of frame_i used to calculate ticks
pub const MAX_FRAME_I: FrameI = 4294967295;
// pixel offset by tick when player move screen display
pub const DISPLAY_OFFSET_BY: f32 = 3.0;
// pixel offset by tick when player move screen display with speed
pub const DISPLAY_OFFSET_BY_SPEED: f32 = 10.0;
// Width of sprite sheet
pub const SCENE_ITEMS_SPRITE_SHEET_WIDTH: f32 = 800.0;
// Height of sprite sheet
pub const SCENE_ITEMS_SPRITE_SHEET_HEIGHT: f32 = 600.0;
// Width of sprite sheet
pub const UI_SPRITE_SHEET_WIDTH: f32 = 800.0;
// Height of sprite sheet
pub const UI_SPRITE_SHEET_HEIGHT: f32 = 600.0;
//
pub const DEFAULT_SELECTED_SQUARE_SIDE: f32 = 14.0;
//
pub const DEFAULT_SELECTED_SQUARE_SIDE_HALF: f32 = DEFAULT_SELECTED_SQUARE_SIDE / 2.0;
//
pub const SCENE_ITEMS_CHANGE_ERR_MSG: &str = "scene_items content change !";
pub const SQUADS_CHANGE_ERR_MSG: &str = "squads content change !";
// Distance from move target point to consider reached
pub const MOVE_TO_REACHED_WHEN_DISTANCE_INFERIOR_AT: f32 = 3.0;
// Velocity of move vector
pub const MOVE_VELOCITY: f32 = 1.0;
// Velocity of move fast vector
pub const MOVE_FAST_VELOCITY: f32 = 2.0;
// Velocity of move hide vector
pub const MOVE_HIDE_VELOCITY: f32 = 0.5;
// Visibility computing must consider firsts tiles differently
pub const VISIBILITY_FIRSTS: usize = 4;
// Increment value when bullet fire
pub const UNDER_FIRE_INTENSITY_INCREMENT: f32 = 30.0;
// Decrement value at animate tick
pub const UNDER_FIRE_INTENSITY_DECREMENT: f32 = 5.0;
// Maximum value of under fire intensity
pub const UNDER_FIRE_INTENSITY_MAX: i32 = 500;
// Stop move order if under fire intensity reached
pub const STOP_MOVE_ORDER_IF_UNDER_FIRE_INTENSITY: f32 = 75.0;
// Coefficient to convert distance from two scene points into meters
pub const DISTANCE_TO_METERS_COEFFICIENT: f32 = 0.3;
// Grid distance to search cover point
pub const COVER_DISTANCE: i32 = 5;
//
pub const DISPLAY_DEFEND_X_OFFSET: f32 = 0.5;
pub const DISPLAY_DEFEND_Y_OFFSET: f32 = 1.5;
