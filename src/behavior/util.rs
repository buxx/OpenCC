use crate::behavior::movement::find_cover_grid_point;
use crate::behavior::order::Order;
use crate::behavior::ItemBehavior;
use crate::gameplay::squad::Squad;
use crate::map::Map;
use crate::physics::util::{grid_point_from_scene_point, scene_point_from_grid_point};
use crate::physics::GridPoint;
use crate::scene::item::SceneItemModifier;
use crate::scene::main::{DebugPoint, MainStateModifier};
use crate::{Angle, Message, ScenePoint};

pub fn take_cover_messages(
    reference_scene_point: &ScenePoint,
    angle: Angle,
    frame_i: u32,
    squad: &Squad,
    behavior: &ItemBehavior,
    map: &Map,
) -> Vec<Message> {
    let mut messages = vec![];
    let mut already_used_cover_grid_points: Vec<GridPoint> = vec![];

    for (member_id, formation_position) in squad.member_positions(reference_scene_point, angle) {
        if let Some((cover_grid_point, debug_grid_points)) = find_cover_grid_point(
            &grid_point_from_scene_point(&formation_position, map),
            map,
            &already_used_cover_grid_points,
        ) {
            for debug_grid_point in debug_grid_points.iter() {
                messages.push(Message::MainStateMessage(MainStateModifier::NewDebugPoint(
                    DebugPoint {
                        frame_i: frame_i + 120,
                        scene_point: scene_point_from_grid_point(debug_grid_point, map),
                    },
                )))
            }

            let cover_scene_point = scene_point_from_grid_point(&cover_grid_point, map);
            if let Some(new_order) = match behavior {
                ItemBehavior::Dead | ItemBehavior::Unconscious => None,
                ItemBehavior::Standing | ItemBehavior::MoveTo(_, _) => {
                    Some(Order::MoveTo(cover_scene_point))
                }
                ItemBehavior::MoveFastTo(_, _) => Some(Order::MoveFastTo(cover_scene_point)),
                ItemBehavior::EngageSceneItem(_)
                | ItemBehavior::EngageGridPoint(_)
                | ItemBehavior::HideTo(_, _)
                | ItemBehavior::Hide => Some(Order::HideTo(cover_scene_point)),
            } {
                already_used_cover_grid_points.push(cover_grid_point);
                messages.push(Message::SceneItemMessage(
                    member_id,
                    SceneItemModifier::SetNextOrder(new_order),
                ));
            }
        }
    }

    messages
}
