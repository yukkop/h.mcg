pub(crate) use bevy::prelude::*;

pub fn node_system(
    mut interaction_query: Query<
        (&Interaction, &Transform),
        (Changed<Interaction>, With<crate::graph::structs::Node>),
    >,
) {
}
