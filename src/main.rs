use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let ascii_handle = asset_server.load("VGA9x16.png");
    let ascii_atlas =
        TextureAtlas::from_grid(ascii_handle, Vec2::new(9.0, 16.0), 16, 16, None, None);
    let ascii_atlas_handle = texture_atlases.add(ascii_atlas);

    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteSheetBundle {
        texture_atlas: ascii_atlas_handle,
        sprite: TextureAtlasSprite::new(16 * 4),
        transform: Transform::from_scale(Vec3::splat(6.0)),
        ..default()
    });
}

fn character_movement(
    mut characters: Query<(&mut Transform, &TextureAtlasSprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut characters {
        if input.pressed(KeyCode::W) {
            transform.translation.y += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += 100.0 * time.delta_seconds();
        }
    }
}
