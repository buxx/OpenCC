use crate::behavior::ItemBehavior;
use crate::map::Map;
use crate::scene::item::{SceneItem, SceneItemModifier};
use crate::Angle;

pub fn digest_defend_order(
    _scene_item: &SceneItem,
    angle: &Angle,
    _map: &Map,
) -> Vec<SceneItemModifier> {
    let mut scene_item_modifiers: Vec<SceneItemModifier> = vec![];

    scene_item_modifiers.push(SceneItemModifier::LeaderIndicateTakeCover);
    scene_item_modifiers.push(SceneItemModifier::ChangeBehavior(ItemBehavior::Standing));
    scene_item_modifiers.push(SceneItemModifier::ChangeLookingDirection(*angle));
    scene_item_modifiers.push(SceneItemModifier::SwitchToNextOrder);

    scene_item_modifiers
}

pub fn digest_hide_order(
    _scene_item: &SceneItem,
    angle: &Angle,
    _map: &Map,
) -> Vec<SceneItemModifier> {
    let mut scene_item_modifiers: Vec<SceneItemModifier> = vec![];

    scene_item_modifiers.push(SceneItemModifier::LeaderIndicateTakeCover);
    scene_item_modifiers.push(SceneItemModifier::ChangeBehavior(ItemBehavior::Hide));
    scene_item_modifiers.push(SceneItemModifier::ChangeLookingDirection(*angle));
    scene_item_modifiers.push(SceneItemModifier::SwitchToNextOrder);

    scene_item_modifiers
}
