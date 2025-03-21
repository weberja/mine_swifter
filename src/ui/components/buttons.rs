use bevy::{prelude::*, ui::FocusPolicy};

use crate::{assets::UiAssets, states::AppState};

/// Marker struct for buttons
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component, Default, Debug, PartialEq)]
#[require(Node, FocusPolicy(|| FocusPolicy::Block), Interaction, ImageNode, ButtonActive)]
pub struct SpriteButton;

#[derive(Component, Default)]
pub struct ButtonActive;

impl Plugin for SpriteButton {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Self::button_interaction.run_if(not(in_state(AppState::LoadingAssets))),
        );
    }
}

impl SpriteButton {
    #[allow(clippy::complexity)]
    pub fn button_interaction(
        mut interaction_query: Query<
            (&Interaction, &mut ImageNode),
            (Changed<Interaction>, With<SpriteButton>, With<ButtonActive>),
        >,
        ui_assets: Res<UiAssets>,
    ) {
        for (interaction, mut image) in &mut interaction_query {
            match *interaction {
                Interaction::Hovered => image.image = ui_assets.button_hover.clone(),
                Interaction::Pressed => image.image = ui_assets.button_pressed.clone(),
                Interaction::None => image.image = ui_assets.button.clone(),
            }
        }
    }

    pub fn image(image: Handle<Image>) -> ImageNode {
        ImageNode::new(image.clone()).with_mode(bevy::ui::widget::NodeImageMode::Sliced(
            TextureSlicer {
                border: BorderRect::rectangle(10., 20.),
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                max_corner_scale: 1.0,
            },
        ))
    }
}
