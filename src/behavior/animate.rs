use crate::behavior::defend::{digest_defend_order, digest_hide_order};
use crate::behavior::engagement::digest_engage_scene_item_behavior;
use crate::behavior::movement::{
    digest_move_behavior, digest_next_move_order, digest_stop_move_behavior,
};
use crate::behavior::order::Order;
use crate::behavior::standing::digest_standing_behavior;
use crate::behavior::ItemBehavior;
use crate::map::Map;
use crate::scene::item::{SceneItem, SceneItemModifier};
use crate::FrameI;

pub fn digest_next_order(scene_item: &SceneItem, map: &Map) -> Vec<SceneItemModifier> {
    let mut scene_item_modifiers: Vec<SceneItemModifier> = vec![];

    if let Some(next_order) = &scene_item.next_order {
        match next_order {
            Order::MoveTo(move_to_scene_point)
            | Order::MoveFastTo(move_to_scene_point)
            | Order::HideTo(move_to_scene_point) => {
                scene_item_modifiers.extend(digest_next_move_order(
                    scene_item,
                    *move_to_scene_point,
                    next_order,
                    map,
                ));
            }
            Order::Defend(angle) => {
                scene_item_modifiers.extend(digest_defend_order(scene_item, angle, map))
            }
            Order::Hide(angle) => {
                scene_item_modifiers.extend(digest_hide_order(scene_item, angle, map))
            }
        }
    }

    scene_item_modifiers
}

pub fn digest_current_order(_: &SceneItem, _: &Map) -> Vec<SceneItemModifier> {
    // TODO: here, compute state according to order. Ex: if too much fear, move order do not produce escape state
    vec![]
}

pub fn digest_behavior(
    frame_i: FrameI,
    scene_item: &SceneItem,
    map: &Map,
) -> Vec<SceneItemModifier> {
    let mut scene_item_modifiers: Vec<SceneItemModifier> = vec![];

    match &scene_item.behavior {
        ItemBehavior::Standing => {
            scene_item_modifiers.extend(digest_standing_behavior(scene_item, map));
        }
        ItemBehavior::Hide => {
            // TODO: Change digest_standing_behavior name
            scene_item_modifiers.extend(digest_standing_behavior(scene_item, map));
        }
        ItemBehavior::EngageSceneItem(to_scene_item_id) => {
            scene_item_modifiers.extend(digest_engage_scene_item_behavior(
                frame_i,
                scene_item,
                *to_scene_item_id,
                map,
            ));
        }
        ItemBehavior::EngageGridPoint(_) => {
            // TODO
        }
        ItemBehavior::MoveTo(_, grid_path) => {
            scene_item_modifiers.extend(digest_move_behavior(scene_item, grid_path, map));
            scene_item_modifiers.extend(digest_stop_move_behavior(scene_item, grid_path, map));
        }
        ItemBehavior::MoveFastTo(_, grid_path) => {
            scene_item_modifiers.extend(digest_move_behavior(scene_item, grid_path, map));
            scene_item_modifiers.extend(digest_stop_move_behavior(scene_item, grid_path, map));
        }
        ItemBehavior::HideTo(_, grid_path) => {
            scene_item_modifiers.extend(digest_move_behavior(scene_item, grid_path, map));
        }
        ItemBehavior::Unconscious => {}
        ItemBehavior::Dead => {}
    }

    scene_item_modifiers
}
