use crate::sprite::{calc_sprite_idx, ASCIISpriteSheet};
use crate::{Coins, Player};
use bevy::prelude::*;

pub struct PigPlugin;

impl Plugin for PigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_pig, age_pig));
    }
}

#[derive(Component)]
pub struct Pig {
    pub lifetime: Timer,
    pub is_dead: bool,
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

        if pig.is_dead {
            if lifetime_secs >= 6.0 {
                commands.entity(pig_entity).despawn();
            }
            continue;
        }

        if lifetime_secs < 2.0 {
            continue;
        }

        texture_atlas_sprite.index = calc_sprite_idx(0, 5);

        if lifetime_secs < 5.0 {
            continue;
        }

        texture_atlas_sprite.index = calc_sprite_idx(8, 5);
        let pig_award: f32 = 15.0;
        coins.0 += pig_award;
        pig.is_dead = true;
        info!(
            "A pig has died and you've been awarded {} coins!",
            pig_award,
        )
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
            is_dead: false,
        },
    ));
    info!(
        "Spent {} coins on a pig, remaining coins: {}",
        pig_price, coins.0
    )
}
