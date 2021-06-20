use crate::behavior::order::Order;
use crate::config::{UI_SPRITE_SHEET_HEIGHT, UI_SPRITE_SHEET_WIDTH};
use crate::{SceneItemId, ScenePoint, Angle};
use ggez::graphics;
use ggez::mint::Point2;

const ORDER_MARKER_START_X: f32 = 0.0;
const ORDER_MARKER_START_Y: f32 = 100.0;
const ORDER_MARKER_WIDTH: f32 = 11.0;
const ORDER_MARKER_HEIGHT: f32 = 11.0;
const ORDER_MARKER_DEFEND_START_Y: f32 = 180.0;
const ORDER_MARKER_DEFEND_WIDTH: f32 = 20.0;
const ORDER_MARKER_DEFEND_HEIGHT: f32 = 7.0;
const ORDER_MARKER_HIDE_START_Y: f32 = 170.0;
const ORDER_MARKER_HIDE_WIDTH: f32 = 20.0;
const ORDER_MARKER_HIDE_HEIGHT: f32 = 7.0;

#[derive(PartialEq, Clone, Debug)]
pub enum OrderMarker {
    MoveTo(SceneItemId, ScenePoint),
    MoveFastTo(SceneItemId, ScenePoint),
    HideTo(SceneItemId, ScenePoint),
    FireTo(SceneItemId, ScenePoint),
    Defend(SceneItemId, Angle),
    Hide(SceneItemId, Angle),
}

impl OrderMarker {
    pub fn new(scene_item_id: SceneItemId, order: &Order) -> Self {
        match order {
            Order::MoveTo(move_to_scene_point) => {
                OrderMarker::MoveTo(scene_item_id, *move_to_scene_point)
            }
            Order::MoveFastTo(move_to_scene_point) => {
                OrderMarker::MoveFastTo(scene_item_id, *move_to_scene_point)
            }
            Order::HideTo(move_to_scene_point) => {
                OrderMarker::HideTo(scene_item_id, *move_to_scene_point)
            }
            Order::Defend(angle) => {
                OrderMarker::Defend(scene_item_id, *angle)
            }
            Order::Hide(angle) => {
                OrderMarker::Hide(scene_item_id, *angle)
            }
        }
    }

    pub fn get_target_scene_point(&self) -> ScenePoint {
        match self {
            OrderMarker::MoveTo(_, scene_point)
            | OrderMarker::MoveFastTo(_, scene_point)
            | OrderMarker::HideTo(_, scene_point)
            | OrderMarker::FireTo(_, scene_point) => *scene_point,
            OrderMarker::Defend(_, _) | OrderMarker::Hide(_, _) => {panic!("Should be called !")}
        }
    }
    pub fn get_scene_item_id(&self) -> SceneItemId {
        match self {
            OrderMarker::MoveTo(scene_item_id, _)
            | OrderMarker::MoveFastTo(scene_item_id, _)
            | OrderMarker::HideTo(scene_item_id, _)
            | OrderMarker::FireTo(scene_item_id, _)
            | OrderMarker::Defend(scene_item_id, _)
            | OrderMarker::Hide(scene_item_id, _) => *scene_item_id,
        }
    }
    pub fn set_scene_point(&mut self, new_scene_point: ScenePoint) {
        match self {
            OrderMarker::MoveTo(_, scene_point)
            | OrderMarker::MoveFastTo(_, scene_point)
            | OrderMarker::HideTo(_, scene_point)
            | OrderMarker::FireTo(_, scene_point) => {
                // FIXME: When fire, re compute "on scene item" ?
                scene_point.x = new_scene_point.x;
                scene_point.y = new_scene_point.y
            }
            OrderMarker::Defend(_, _) | OrderMarker::Hide(_, _) => {panic!("Should not be called !")}
        }
    }

    pub fn sprite_info(&self) -> OrderMarkerSpriteInfo {
        match self {
            OrderMarker::MoveTo(_, _) => OrderMarkerSpriteInfo {
                relative_start_x: ORDER_MARKER_START_X / UI_SPRITE_SHEET_WIDTH,
                relative_start_y: ORDER_MARKER_START_Y / UI_SPRITE_SHEET_HEIGHT,
                relative_width: ORDER_MARKER_WIDTH / UI_SPRITE_SHEET_WIDTH,
                relative_height: ORDER_MARKER_HEIGHT / UI_SPRITE_SHEET_HEIGHT,
                width: ORDER_MARKER_WIDTH,
                height: ORDER_MARKER_HEIGHT,
                half_width: ORDER_MARKER_WIDTH / 2.0,
                half_height: ORDER_MARKER_HEIGHT / 2.0,
            },
            OrderMarker::MoveFastTo(_, _) => OrderMarkerSpriteInfo {
                relative_start_x: ORDER_MARKER_START_X / UI_SPRITE_SHEET_WIDTH,
                relative_start_y: (ORDER_MARKER_START_Y + (ORDER_MARKER_HEIGHT * 1.0))
                    / UI_SPRITE_SHEET_HEIGHT,
                relative_width: ORDER_MARKER_WIDTH / UI_SPRITE_SHEET_WIDTH,
                relative_height: ORDER_MARKER_HEIGHT / UI_SPRITE_SHEET_HEIGHT,
                width: ORDER_MARKER_WIDTH,
                height: ORDER_MARKER_HEIGHT,
                half_width: ORDER_MARKER_WIDTH / 2.0,
                half_height: ORDER_MARKER_HEIGHT / 2.0,
            },
            OrderMarker::HideTo(_, _) => OrderMarkerSpriteInfo {
                relative_start_x: ORDER_MARKER_START_X / UI_SPRITE_SHEET_WIDTH,
                relative_start_y: (ORDER_MARKER_START_Y + (ORDER_MARKER_HEIGHT * 2.0))
                    / UI_SPRITE_SHEET_HEIGHT,
                relative_width: ORDER_MARKER_WIDTH / UI_SPRITE_SHEET_WIDTH,
                relative_height: ORDER_MARKER_HEIGHT / UI_SPRITE_SHEET_HEIGHT,
                width: ORDER_MARKER_WIDTH,
                height: ORDER_MARKER_HEIGHT,
                half_width: ORDER_MARKER_WIDTH / 2.0,
                half_height: ORDER_MARKER_HEIGHT / 2.0,
            },
            OrderMarker::FireTo(_, _) => OrderMarkerSpriteInfo {
                relative_start_x: ORDER_MARKER_START_X / UI_SPRITE_SHEET_WIDTH,
                relative_start_y: (ORDER_MARKER_START_Y + (ORDER_MARKER_HEIGHT * 3.0))
                    / UI_SPRITE_SHEET_HEIGHT,
                relative_width: ORDER_MARKER_WIDTH / UI_SPRITE_SHEET_WIDTH,
                relative_height: ORDER_MARKER_HEIGHT / UI_SPRITE_SHEET_HEIGHT,
                width: ORDER_MARKER_WIDTH,
                height: ORDER_MARKER_HEIGHT,
                half_width: ORDER_MARKER_WIDTH / 2.0,
                half_height: ORDER_MARKER_HEIGHT / 2.0,
            },
            OrderMarker::Defend(_, _) => OrderMarkerSpriteInfo {
                relative_start_x: ORDER_MARKER_START_X / UI_SPRITE_SHEET_WIDTH,
                relative_start_y: ORDER_MARKER_DEFEND_START_Y / UI_SPRITE_SHEET_HEIGHT,
                relative_width: ORDER_MARKER_DEFEND_WIDTH / UI_SPRITE_SHEET_WIDTH,
                relative_height: ORDER_MARKER_DEFEND_HEIGHT / UI_SPRITE_SHEET_HEIGHT,
                width: ORDER_MARKER_DEFEND_WIDTH,
                height: ORDER_MARKER_DEFEND_HEIGHT,
                half_width: ORDER_MARKER_DEFEND_WIDTH / 2.0,
                half_height: ORDER_MARKER_DEFEND_HEIGHT / 2.0,
            },
            OrderMarker::Hide(_, _) => OrderMarkerSpriteInfo {
                relative_start_x: ORDER_MARKER_START_X / UI_SPRITE_SHEET_WIDTH,
                relative_start_y: ORDER_MARKER_HIDE_START_Y / UI_SPRITE_SHEET_HEIGHT,
                relative_width: ORDER_MARKER_HIDE_WIDTH / UI_SPRITE_SHEET_WIDTH,
                relative_height: ORDER_MARKER_HIDE_HEIGHT / UI_SPRITE_SHEET_HEIGHT,
                width: ORDER_MARKER_HIDE_WIDTH,
                height: ORDER_MARKER_HIDE_HEIGHT,
                half_width: ORDER_MARKER_HIDE_WIDTH / 2.0,
                half_height: ORDER_MARKER_HIDE_HEIGHT / 2.0,
            }
        }
    }
}

pub struct OrderMarkerSpriteInfo {
    pub relative_start_x: f32,
    pub relative_start_y: f32,
    pub relative_width: f32,
    pub relative_height: f32,
    pub width: f32,
    pub height: f32,
    pub half_width: f32,
    pub half_height: f32,
}

impl OrderMarkerSpriteInfo {
    pub fn as_draw_params(&self, draw_to_scene_point: &ScenePoint, angle: Angle) -> graphics::DrawParam {
        let dest_scene_point = ScenePoint::new(
            draw_to_scene_point.x - (self.width / 2.0),
            draw_to_scene_point.y - (self.height / 2.0),
        );
        graphics::DrawParam::new()
            .src(graphics::Rect::new(
                self.relative_start_x,
                self.relative_start_y,
                self.relative_width,
                self.relative_height,
            ))
            .dest(dest_scene_point)
            .rotation(angle)
    }

    pub fn rectangle(&self, from_scene_point: &ScenePoint) -> graphics::Rect {
        graphics::Rect::new(
            from_scene_point.x - self.half_width,
            from_scene_point.y - self.half_height,
            self.width,
            self.height,
        )
    }

    pub fn contains(&self, draw_to_scene_point: &ScenePoint, point: &ScenePoint) -> bool {
        self.rectangle(draw_to_scene_point)
            .contains(Point2::from([point.x, point.y]))
    }
}
