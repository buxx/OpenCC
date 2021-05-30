use crate::ui::order::OrderMarker;
use crate::{SceneItemId, WindowPoint};
use std::time::Duration;

pub mod order;
pub mod vertical_menu;

const SCENE_ITEM_MENU_WIDTH: f32 = 71.0;
const SCENE_ITEM_MENU_HEIGHT: f32 = 68.0;
const SCENE_ITEM_MENU_ITEM_HEIGHT: f32 = 15.0;

pub enum UiComponent {
    SceneItemMenu,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UserEvent {
    CursorMove(WindowPoint),
    Click(WindowPoint),
    RightClick(WindowPoint),
    AreaSelection(WindowPoint, WindowPoint),
    DrawMovePaths,
    BeginDragOrderMarker(SceneItemId),
    MoveDrag,
    ReleaseDrag,
}

pub enum Dragging {
    OrderMarker(SceneItemId),
}

#[derive(Debug, PartialEq)]
pub struct CursorImmobile(pub Duration, pub UserEvent);

#[derive(Debug)]
pub enum SceneItemPrepareOrder {
    Move(usize),     // scene_item usize
    MoveFast(usize), // scene_item usize
    Hide(usize),     // scene_item usize
}

#[derive(Clone)]
pub enum MenuItem {
    Move,
    MoveFast,
    Hide,
}
