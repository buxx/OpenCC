pub mod animate;
pub mod defend;
pub mod engagement;
pub mod movement;
pub mod order;
pub mod standing;
pub mod util;

use crate::physics::GridPoint;
use crate::{GridPath, SceneItemId, ScenePoint};
use crate::gameplay::weapon::SceneItemWeapon;

pub enum ItemBehavior {
    Dead,
    Unconscious,
    Standing,
    HideTo(ScenePoint, GridPath),
    MoveTo(ScenePoint, GridPath),
    MoveFastTo(ScenePoint, GridPath),
    EngageSceneItem(SceneItemWeapon, SceneItemId),
    EngageGridPoint(GridPoint),
    Hide,
}
