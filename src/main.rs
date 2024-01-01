use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
pub struct Pig {
    pub lifetime: Timer,
}

#[derive(Resource)]
pub struct Coins(pub f32);

#[derive(Resource)]
struct ASCIISpriteSheet(Handle<TextureAtlas>);

impl FromWorld for ASCIISpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let texture_handle = asset_server.load("VGA9x16.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(9.0, 16.0), 16, 16, None, None);

        let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        Self(texture_atlas_handle)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<ASCIISpriteSheet>()
        .insert_resource(Coins(100.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (character_movement, spawn_pig, age_pig))
        .run()
}

fn setup(mut commands: Commands, ascii_sprite_sheet_handle: Res<ASCIISpriteSheet>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(calc_sprite_idx(0, 4)),
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

fn age_pig(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Pig, Entity)>,
    mut coins: ResMut<Coins>,
) {
    for (mut texture_atlas_sprite, mut pig, pig_entity) in &mut query {
        pig.lifetime.tick(time.delta());

        let lifetime_secs = pig.lifetime.elapsed_secs();

        if lifetime_secs < 2.0 {
            continue;
        }

        texture_atlas_sprite.index = calc_sprite_idx(0, 5);

        if lifetime_secs < 5.0 {
            continue;
        }

        texture_atlas_sprite.index = calc_sprite_idx(8, 5);
        coins.0 += 15.0;

        if lifetime_secs >= 6.0 {
            commands.entity(pig_entity).despawn();
        }
    }
}

fn spawn_pig(
    mut commands: Commands,
    ascii_sprite_sheet_handle: Res<ASCIISpriteSheet>,
    input: Res<Input<KeyCode>>,
    mut coins: ResMut<Coins>,
    query: Query<&Transform, With<Player>>,
) {
    let pig_price: f32 = 10.0;

    if !input.just_pressed(KeyCode::Space) || coins.0 < pig_price {
        return;
    }

    coins.0 -= pig_price;

    let player_transform = query.single();
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(calc_sprite_idx(0, 7)),
            texture_atlas: ascii_sprite_sheet_handle.0.clone(),
            transform: *player_transform,
            ..default()
        },
        Pig {
            lifetime: Timer::from_seconds(6.0, TimerMode::Once),
        },
    ));
    info!(
        "Spent {} coins on a pig, remaining coins: {}",
        pig_price, coins.0
    )
}

fn calc_sprite_idx(column: u8, row: u8) -> usize {
    (row * 16 + column) as usize // (row * column_count) + column
}
