mod gamemap;
use bevy::asset::LoadState;

// Sprites are 24x24
// Character spritesheet is 18x22

mod prelude {
    pub use bevy::prelude::*;
    pub const SCREEN_WIDTH: usize = 1024;
    pub const SCREEN_HEIGHT: usize = 768;
    pub const MAP_WIDTH: usize = 40;
    pub const MAP_HEIGHT: usize = 30;
    pub const TILE_SIZE: f32 = 24.0;
    pub use crate::gamemap::*;
}

use prelude::*;

const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.0;
const MAP_TILE_WIDTH: usize = MAP_WIDTH;
const MAP_TILE_HEIGHT: usize = MAP_HEIGHT;
const HALF_MAP_PIXEL_WIDTH: f32 =
    MAP_TILE_WIDTH as f32 * TILE_SIZE as f32 / 2.0;
const HALF_MAP_PIXEL_HEIGHT: f32 =
    MAP_TILE_HEIGHT as f32 * TILE_SIZE / 2.0;
const WIDTH_CENTER_OFFSET: f32 = HALF_MAP_PIXEL_WIDTH - HALF_TILE_SIZE;
const HEIGHT_CENTER_OFFSET: f32 = HALF_MAP_PIXEL_HEIGHT - HALF_TILE_SIZE;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
}

#[derive(Resource, Default)]
struct TerrainSpriteHandles {
    handles: Vec<HandleUntyped>,
}

fn main() {
    App::new()
        .init_resource::<TerrainSpriteHandles>()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Northlands".into(),
                    resolution: (SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32).into(),
                    ..default()
                }),
                ..default()
            },
        )
            .set(ImagePlugin::default_nearest()))
        .add_state::<AppState>()
        .add_systems(OnEnter(AppState::Setup), load_textures)
        .add_systems(Update, check_textures.run_if(in_state(AppState::Setup)))
        .add_systems(OnEnter(AppState::Finished), setup)
        .add_systems(Update, animate_sprite_two_frame)
        .run();
}

fn load_textures(mut terrain_sprite_handles: ResMut<TerrainSpriteHandles>, asset_server: Res<AssetServer>) {
    // load multiple, individual sprites from a folder
    terrain_sprite_handles.handles = asset_server.load_folder("textures").unwrap();
}

fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    terrain_sprite_handles: ResMut<TerrainSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    if let LoadState::Loaded = asset_server
        .get_group_load_state(terrain_sprite_handles.handles.iter().map(|handle| handle.id()))
    {
        next_state.set(AppState::Finished);
    }
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite_two_frame(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                indices.last
            };
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let terrain_handle = asset_server.get_handle("textures/world.png");
    let texture_atlas = TextureAtlas::from_grid(terrain_handle, Vec2::new(24.0, 24.0), 55, 39, None, None);
    let atlas_handle = texture_atlases.add(texture_atlas);

    // set up a scene to display our texture atlas
    commands.spawn(Camera2dBundle::default());

    // Init a map
    let map: GameMap = GameMap::new();

    // Render the tiles that compose the map to the screen
    for (index, tile) in map.tiles.iter().enumerate() {
        let coords = GameMap::index_to_map(index);
        commands.spawn((
            SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(
                        coords.0 as f32 * TILE_SIZE - WIDTH_CENTER_OFFSET,
                        coords.1 as f32 * TILE_SIZE - HEIGHT_CENTER_OFFSET,
                        0.0
                    ),
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                sprite: TextureAtlasSprite::new(tile.index),
                texture_atlas: atlas_handle.clone(),
                ..default()
            }
        ));
    }
}