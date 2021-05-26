use crate::behavior::ItemBehavior;
use crate::config::VISIBILITY_FIRSTS;
use crate::map::Map;
use crate::physics::util::grid_point_from_scene_point;
use crate::scene::item::SceneItem;
use crate::{GridPath, ScenePoint};
use bresenham::Bresenham;

#[derive(Debug, Clone)]
pub struct Visibility {
    pub from_scene_point: ScenePoint,
    pub from_scene_id: usize,
    pub to_scene_point: ScenePoint,
    pub to_scene_item_id: Option<usize>,
    pub path_final_opacity: f32,
    pub to_scene_item_opacity: f32,
    pub opacity_segments: Vec<(ScenePoint, f32)>,
    pub visible: bool,
}

impl Visibility {
    pub fn with_scene_item_target(
        scene_item_from: &SceneItem,
        to_scene_item: &SceneItem,
        map: &Map,
    ) -> Self {
        let to_scene_point = &to_scene_item.position;
        let mut opacity_segments: Vec<(ScenePoint, f32)> = vec![];
        let mut grid_path: GridPath = vec![];
        let mut path_final_opacity: f32 = 0.0;

        // Compute line pixels
        let pixels = Bresenham::new(
            (
                scene_item_from.position.x as isize,
                scene_item_from.position.y as isize,
            ),
            (to_scene_point.x as isize, to_scene_point.y as isize),
        );

        // Compute opacity segments
        for (pixel_x, pixel_y) in pixels {
            let grid_point =
                grid_point_from_scene_point(&ScenePoint::new(pixel_x as f32, pixel_y as f32), map);
            if !grid_path.contains(&grid_point) {
                let terrain_tile = map
                    .terrain
                    .tiles
                    .get(&(grid_point.x as u32, grid_point.y as u32))
                    .expect("Work with path only in map !");
                // Firsts tiles opacity are ignored
                let grid_point_opacity = if grid_path.len() <= VISIBILITY_FIRSTS {
                    0.0
                } else {
                    terrain_tile.opacity
                };
                path_final_opacity += grid_point_opacity;
                grid_path.push(grid_point);
                opacity_segments.push((
                    ScenePoint::new(pixel_x as f32, pixel_y as f32),
                    path_final_opacity,
                ));
            }
        }

        let to_scene_item_opacity_modifier_by_behavior: f32 = match to_scene_item.behavior {
            ItemBehavior::Dead => 0.0,
            ItemBehavior::Unconscious => 0.0,
            ItemBehavior::Standing => 0.5,
            ItemBehavior::HideTo(_, _) => -1.0,
            ItemBehavior::MoveTo(_, _) => 1.0,
            ItemBehavior::MoveFastTo(_, _) => 2.0,
            // FIXME BS NOW: quand il y a des tirs seulement
            ItemBehavior::EngageSceneItem(_) => 0.0,
            ItemBehavior::EngageGridPoint(_) => 0.0,
        };

        // TODO: Target opacity is modified by firing/running, etc
        let to_scene_item_opacity = path_final_opacity - to_scene_item_opacity_modifier_by_behavior;
        let visible = to_scene_item_opacity < 0.5;

        Self {
            from_scene_id: scene_item_from.id,
            from_scene_point: scene_item_from.position,
            to_scene_point: to_scene_point.clone(),
            to_scene_item_id: Some(to_scene_item.id),
            opacity_segments,
            path_final_opacity,
            to_scene_item_opacity,
            visible,
        }
    }
}
