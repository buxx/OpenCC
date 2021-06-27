use crate::behavior::ItemBehavior;
use crate::map::Map;
use crate::scene::item::{SceneItem, SceneItemModifier};
use crate::util::angle;

pub fn digest_standing_behavior(scene_item: &SceneItem, _map: &Map) -> Vec<SceneItemModifier> {
    let mut scene_item_modifiers: Vec<SceneItemModifier> = vec![];

    // Visible enemy
    // TODO: Choose enemy by opacity/team repartition etc
    if let Some((weapon, visibility)) = scene_item.reachable_visible_scene_items_visibilities().first() {
        scene_item_modifiers.push(SceneItemModifier::ChangeBehavior(
            ItemBehavior::EngageSceneItem(*weapon, visibility.to_scene_item_id.expect("visible_scene_items_visibilities must return only visibilities with to_scene_item_id")),
        ));
        scene_item_modifiers.push(SceneItemModifier::ChangeLookingDirection(angle(
            &visibility.to_scene_point,
            &scene_item.position,
        )));
    }

    scene_item_modifiers
}
