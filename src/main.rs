mod map;
use bevy::asset::LoadState;

// Sprites are 24x24
// Character spritesheet is 18x22

mod prelude {
    pub use bevy::prelude::*;
    pub const SCREEN_WIDTH: f32 = 1024.0;
    pub const SCREEN_HEIGHT: f32 = 768.0;
}

use prelude::*;

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
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
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

fn coords_to_sprite(x: u16, y: u16) -> usize {
    ((y * 18) + x) as usize
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let terrain_handle = asset_server.get_handle("textures/creatures.png");
    let texture_atlas = TextureAtlas::from_grid(terrain_handle, Vec2::new(24.0, 24.0), 18, 22, None, None);
    let atlas_handle = texture_atlases.add(texture_atlas);

    let index_start: usize = coords_to_sprite(14, 4);
    let index_end: usize = coords_to_sprite(14, 5);

    let animation_indices = AnimationIndices { first: index_start, last: index_end  };

    // set up a scene to display our texture atlas
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::splat(1.0),
                ..default()
            },
            sprite: TextureAtlasSprite::new(animation_indices.first),
            texture_atlas: atlas_handle.clone(),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
    ));
}