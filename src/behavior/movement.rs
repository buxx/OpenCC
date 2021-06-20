use crate::behavior::order::Order;
use crate::behavior::ItemBehavior;
use crate::config::{
    COVER_DISTANCE, MOVE_TO_REACHED_WHEN_DISTANCE_INFERIOR_AT,
    STOP_MOVE_ORDER_IF_UNDER_FIRE_INTENSITY,
};
use crate::map::terrain::{grid_points_for_square, TerrainTile};
use crate::map::Map;
use crate::physics::path::find_path;
use crate::physics::util::{grid_point_from_scene_point, scene_point_from_grid_point};
use crate::physics::GridPoint;
use crate::scene::item::{SceneItem, SceneItemModifier};
use crate::util::{angle, velocity_for_behavior};
use crate::{GridPath, ScenePoint};

pub fn digest_next_move_order(
    scene_item: &SceneItem,
    move_to_scene_point: ScenePoint,
    order: &Order,
    map: &Map,
) -> Vec<SceneItemModifier> {
    let mut scene_item_modifiers: Vec<SceneItemModifier> = vec![];

    // TODO: Compute here if it possible (fear, compatible with current order, etc)
    if let Some(mut grid_path) = find_path(
        &map,
        &scene_item.grid_position,
        &grid_point_from_scene_point(&move_to_scene_point, &map),
    ) {
        grid_path.drain(0..1);
        let behavior = match order {
            Order::MoveTo(_) => ItemBehavior::MoveTo(move_to_scene_point, grid_path),
            Order::MoveFastTo(_) => ItemBehavior::MoveFastTo(move_to_scene_point, grid_path),
            Order::HideTo(_) => ItemBehavior::HideTo(move_to_scene_point, grid_path),
            _ => {
                panic!("this code should be called only with move order")
            }
        };
        scene_item_modifiers.push(SceneItemModifier::SwitchToNextOrder);
        scene_item_modifiers.push(SceneItemModifier::ChangeBehavior(behavior));
        if scene_item.is_leader {
            scene_item_modifiers.push(SceneItemModifier::LeaderIndicateMove);
        }
    } else {
        eprintln!("No path found to given scene point {}", move_to_scene_point);
    };

    scene_item_modifiers
}

pub fn digest_move_behavior(
    scene_item: &SceneItem,
    grid_path: &GridPath,
    map: &Map,
) -> Vec<SceneItemModifier> {
    let mut scene_item_modifiers: Vec<SceneItemModifier> = vec![];

    if let Some(going_to_grid_point) = grid_path.first() {
        let going_to_scene_point = scene_point_from_grid_point(going_to_grid_point, &map);

        // Note: angle computed by adding FRAC_PI_2 because sprites are north oriented
        scene_item_modifiers.push(SceneItemModifier::ChangeDisplayAngle(angle(
            &going_to_scene_point,
            &scene_item.position,
        )));

        // Check if scene_point reached
        let distance = going_to_scene_point.distance(scene_item.position);
        let velocity =
            velocity_for_behavior(&scene_item.behavior).expect("must have velocity here");
        if distance < MOVE_TO_REACHED_WHEN_DISTANCE_INFERIOR_AT * velocity {
            scene_item_modifiers.push(SceneItemModifier::ReachMoveGridPoint);
        }
    } else {
        eprintln!("No grid point in grid path !")
    }

    scene_item_modifiers
}

pub fn digest_stop_move_behavior(
    scene_item: &SceneItem,
    _grid_path: &GridPath,
    _map: &Map,
) -> Vec<SceneItemModifier> {
    let mut scene_item_modifiers: Vec<SceneItemModifier> = vec![];

    if scene_item.under_fire_intensity >= STOP_MOVE_ORDER_IF_UNDER_FIRE_INTENSITY {
        scene_item_modifiers.push(SceneItemModifier::CancelOrders(Some(ItemBehavior::Hide)));
    }

    scene_item_modifiers
}

pub fn find_cover_grid_point(
    from_grid_point: &GridPoint,
    map: &Map,
    exclude_grid_points: &Vec<GridPoint>,
) -> Option<(GridPoint, Vec<GridPoint>)> {
    let mut tiles: Vec<(GridPoint, &TerrainTile)> = vec![(
        from_grid_point.clone(),
        map.terrain
            .tiles
            .get(&(from_grid_point.x as u32, from_grid_point.y as u32))
            .unwrap(),
    )];
    let grid_points_for_square =
        grid_points_for_square(&from_grid_point, COVER_DISTANCE, COVER_DISTANCE);
    for grid_point in grid_points_for_square {
        if let Some(tile) = map
            .terrain
            .tiles
            .get(&(grid_point.x as u32, grid_point.y as u32))
        {
            tiles.push((grid_point, tile))
        }
    }
    tiles.sort_by(|(_, tile_a), (_, tile_b)| tile_a.opacity.partial_cmp(&tile_b.opacity).unwrap());

    for (grid_point, _) in tiles.iter().rev() {
        if !exclude_grid_points.contains(grid_point) {
            let grid_points = tiles
                .iter()
                .map(|(p, _)| p.clone())
                .collect::<Vec<GridPoint>>();
            return Some((grid_point.clone(), grid_points));
        }
    }

    None
}
