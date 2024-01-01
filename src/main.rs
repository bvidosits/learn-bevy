use bevy::prelude::*;
use pig::PigPlugin;

mod pig;
mod sprite;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Resource)]
pub struct Coins(pub f32);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Learn Bevy".to_string(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(PigPlugin)
        .init_resource::<sprite::ASCIISpriteSheet>()
        .insert_resource(Coins(100.0))
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run()
}

fn setup(mut commands: Commands, ascii_sprite_sheet_handle: Res<sprite::ASCIISpriteSheet>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(sprite::calc_sprite_idx(0, 4)),
            texture_atlas: ascii_sprite_sheet_handle.0.clone(),
            transform: Transform::from_scale(Vec3::splat(3.0)),
            ..default()
        },
        Player { speed: 100.0 },
    ));
}

fn character_movement(
    mut query: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut query {
        let movement_amount = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_amount;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_amount;
        }
    }
}
