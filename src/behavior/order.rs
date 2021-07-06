use crate::{Angle, ScenePoint};

#[derive(Clone, Debug)]
pub enum Order {
    MoveTo(ScenePoint),
    MoveFastTo(ScenePoint),
    HideTo(ScenePoint),
    Defend(Angle),
    Hide(Angle),
}
