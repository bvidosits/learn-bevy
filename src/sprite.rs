use bevy::prelude::*;

#[derive(Resource)]
pub struct ASCIISpriteSheet(pub Handle<TextureAtlas>);

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

pub fn calc_sprite_idx(column: u8, row: u8) -> usize {
    (row * 16 + column) as usize // (row * column_count) + column
}
