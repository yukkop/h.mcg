
pub(crate) use bevy::prelude::*;


pub fn node_system(
    mut interaction_query: Query<
        (&Interaction, &Transform),
        (Changed<Interaction>, With<Node>),
    >,
) {
    println!("interaction for nodes is empty? {}", interaction_query.is_empty());
    for (interaction, _transform) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                println!("Node Clicked");
            }
            Interaction::Hovered => {
                println!("Node Hovered");
            }
            Interaction::None => {
            }
        }
    }
}