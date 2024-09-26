use bevy::prelude::*;
use crate::entities::Position;

pub fn move_entities(mut query: Query<(&mut Position, &mut Transform)>) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}
