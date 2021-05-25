pub mod animate;
pub mod engagement;
pub mod movement;
pub mod order;
pub mod standing;

use crate::physics::GridPoint;
use crate::{GridPath, SceneItemId, ScenePoint};

pub enum ItemBehavior {
    Dead,
    Unconscious,
    Standing,
    HideTo(ScenePoint, GridPath),
    MoveTo(ScenePoint, GridPath),
    MoveFastTo(ScenePoint, GridPath),
    EngageSceneItem(SceneItemId),
    EngageGridPoint(GridPoint),
}
