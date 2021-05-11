use crate::map::Map;
use crate::physics::GridPoint;
use pathfinding::prelude::dijkstra;

pub fn find_path(map: &Map, from: &GridPoint, to: &GridPoint) -> Option<Vec<GridPoint>> {
    match dijkstra(from, |p| map.successors(p), |p| *p == *to) {
        None => None,
        Some(path) => Some(path.0),
    }
}
