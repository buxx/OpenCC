use crate::{SceneItemId, ScenePoint, SquadId, WindowPoint};
use std::time::Duration;

pub mod order;
pub mod vertical_menu;

const SCENE_ITEM_MENU_WIDTH: f32 = 71.0;
const SCENE_ITEM_MENU_HEIGHT: f32 = 90.0;
const SCENE_ITEM_MENU_ITEM_HEIGHT: f32 = 15.0;

pub enum UiComponent {
    SquadMenu(SquadId, ScenePoint),
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
    SelectSceneItemIds(Vec<SceneItemId>),
}

pub enum Dragging {
    OrderMarker(SceneItemId),
}

#[derive(Debug, PartialEq)]
pub struct CursorImmobile(pub Duration, pub UserEvent);

#[derive(Debug)]
pub enum SceneItemPrepareOrder {
    // FIXME BS NOW: SquadId
    Move(SquadId),
    MoveFast(SquadId),
    Sneak(SquadId),
    Defend(SquadId),
    Hide(SquadId),
}

#[derive(Clone)]
pub enum MenuItem {
    Move,
    MoveFast,
    Sneak,
    Defend,
    Hide,
}
