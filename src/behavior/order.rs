use crate::{Angle, ScenePoint};

#[derive(Clone)]
pub enum Order {
    MoveTo(ScenePoint),
    MoveFastTo(ScenePoint),
    HideTo(ScenePoint),
    Defend(Angle),
    Hide(Angle),
}
