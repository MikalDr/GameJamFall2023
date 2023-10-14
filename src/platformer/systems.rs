use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use std::collections::{HashMap, HashSet};

use bevy_rapier2d::prelude::*;

use crate::game::GameState;

use super::components::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);

    let ldtk_handle = asset_server.load("map\\Typical_2D_platformer_example.ldtk");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });

    //Starts game time, do not shoot this
    start_game_time(commands);
}


// This is not good
pub fn movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &GroundDetection), With<Player>>,
) {
    for (mut velocity, ground_detection) in &mut query {
        let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::A) { 1. } else { 0. };

        velocity.linvel.x = (right - left) * 200.;

        // Jumping
        if input.just_pressed(KeyCode::Space) && (ground_detection.on_ground) {
            velocity.linvel.y = 500.;
        }
    }
}

/// Spawns heron collisions for the walls of a level
///
/// You could just insert a ColliderBundle in to the WallBundle,
/// but this spawns a different collider for EVERY wall tile.
/// This approach leads to bad performance.
///
/// Instead, by flagging the wall tiles and spawning the collisions later,
/// we can minimize the amount of colliding entities.
///
/// The algorithm used here is a nice compromise between simplicity, speed,
/// and a small number of rectangle colliders.
/// In basic terms, it will:
/// 1. consider where the walls are
/// 2. combine wall tiles into flat "plates" in each individual row
/// 3. combine the plates into rectangles across multiple rows wherever possible
/// 4. spawn colliders for each rectangle
pub fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    wall_query.for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
        }
    });

    if !wall_query.is_empty() {
        level_query.for_each(|(level_entity, level_handle)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let level = levels
                    .get(level_handle)
                    .expect("Level should be loaded by this point");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level
                    .level
                    .layer_instances
                    .clone()
                    .expect("Level asset should have layers")[0];

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right edge
                    for x in 0..width + 1 {
                        match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
                let mut prev_row: Vec<Plate> = Vec::new();
                let mut wall_rects: Vec<Rect> = Vec::new();

                // an extra empty row so the algorithm "finishes" the rects that touch the top edge
                plate_stack.push(Vec::new());

                for (y, current_row) in plate_stack.into_iter().enumerate() {
                    for prev_plate in &prev_row {
                        if !current_row.contains(prev_plate) {
                            // remove the finished rect so that the same plate in the future starts a new rect
                            if let Some(rect) = rect_builder.remove(prev_plate) {
                                wall_rects.push(rect);
                            }
                        }
                    }
                    for plate in &current_row {
                        rect_builder
                            .entry(plate.clone())
                            .and_modify(|e| e.top += 1)
                            .or_insert(Rect {
                                bottom: y as i32,
                                top: y as i32,
                                left: plate.left,
                                right: plate.right,
                            });
                    }
                    prev_row = current_row;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for wall_rect in wall_rects {
                        level
                            .spawn_empty()
                            .insert(Collider::cuboid(
                                (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                                (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                            ))
                            .insert(RigidBody::Fixed)
                            .insert(Friction::new(1.0))
                            .insert(Transform::from_xyz(
                                (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                                    / 2.,
                                (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                                    / 2.,
                                0.,
                            ))
                            .insert(GlobalTransform::default());
                    }
                });
            }
        });
    }
}

const ASPECT_RATIO: f32 = 16. / 9.;

#[allow(clippy::type_complexity)]
pub fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        Without<Player>,
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;

        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_handle) in &level_query {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, level) {
                    let level_ratio = level.px_wid as f32 / ldtk_level.level.px_hei as f32;
                    orthographic_projection.viewport_origin = Vec2::ZERO;
                    if level_ratio > ASPECT_RATIO {
                        // level is wider than the screen
                        let height = (level.px_hei as f32 / 9.).round() * 9.;
                        let width = height * ASPECT_RATIO;
                        orthographic_projection.scaling_mode =
                            bevy::render::camera::ScalingMode::Fixed { width, height };
                        camera_transform.translation.x =
                            (player_translation.x - level_transform.translation.x - width / 2.)
                                .clamp(0., level.px_wid as f32 - width);
                        camera_transform.translation.y = 0.;
                    } else {
                        // level is taller than the screen
                        let width = (level.px_wid as f32 / 16.).round() * 16.;
                        let height = width / ASPECT_RATIO;
                        orthographic_projection.scaling_mode =
                            bevy::render::camera::ScalingMode::Fixed { width, height };
                        camera_transform.translation.y =
                            (player_translation.y - level_transform.translation.y - height / 2.)
                                .clamp(0., level.px_hei as f32 - height);
                        camera_transform.translation.x = 0.;
                    }

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;
                }
            }
        }
    }
}

pub fn update_level_selection(
    level_query: Query<(&Handle<LdtkLevel>, &Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    for (level_handle, level_transform) in &level_query {
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let level_bounds = Rect {
                min: Vec2::new(level_transform.translation.x, level_transform.translation.y),
                max: Vec2::new(
                    level_transform.translation.x + ldtk_level.level.px_wid as f32,
                    level_transform.translation.y + ldtk_level.level.px_hei as f32,
                ),
            };

            for player_transform in &player_query {
                if player_transform.translation.x < level_bounds.max.x
                    && player_transform.translation.x > level_bounds.min.x
                    && player_transform.translation.y < level_bounds.max.y
                    && player_transform.translation.y > level_bounds.min.y
                    && !level_selection.is_match(&0, &ldtk_level.level)
                {
                    *level_selection = LevelSelection::Iid(ldtk_level.level.iid.clone());
                }
            }
        }
    }
}

pub fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_for: Query<(Entity, &Collider), Added<GroundDetection>>,
) {
    for (entity, shape) in &detect_ground_for {
        if let Some(cuboid) = shape.as_cuboid() {
            let Vec2 {
                x: half_extents_x,
                y: half_extents_y,
            } = cuboid.half_extents();

            let detector_shape = Collider::cuboid(half_extents_x / 2.0, 2.);

            let sensor_translation = Vec3::new(0., -half_extents_y, 0.);

            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn_empty()
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(detector_shape)
                    .insert(Sensor)
                    .insert(Transform::from_translation(sensor_translation))
                    .insert(GlobalTransform::default())
                    .insert(GroundSensor {
                        ground_detection_entity: entity,
                        intersecting_ground_entities: HashSet::new(),
                    });
            });
        }
    }
}

pub fn ground_detection(
    mut ground_sensors: Query<&mut GroundSensor>,
    mut collisions: EventReader<CollisionEvent>,
    collidables: Query<With<Collider>, Without<Sensor>>,
) {
    for collision_event in collisions.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e2) {
                        sensor.intersecting_ground_entities.insert(*e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e1) {
                        sensor.intersecting_ground_entities.insert(*e2);
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e2) {
                        sensor.intersecting_ground_entities.remove(e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e1) {
                        sensor.intersecting_ground_entities.remove(e2);
                    }
                }
            }
        }
    }
}

pub fn update_on_ground(
    mut ground_detectors: Query<&mut GroundDetection>,
    ground_sensors: Query<&GroundSensor, Changed<GroundSensor>>,
) {
    for sensor in &ground_sensors {
        if let Ok(mut ground_detection) = ground_detectors.get_mut(sensor.ground_detection_entity) {
            ground_detection.on_ground = !sensor.intersecting_ground_entities.is_empty();
        }
    }
}

pub fn restart_level(
    mut commands: Commands,
    level_query: Query<Entity, With<Handle<LdtkLevel>>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::R) {
        for level_entity in &level_query {
            commands.entity(level_entity).insert(Respawn);
        }
    }
}


pub fn apply_player_sprite
(
    mut cmd: Commands,
    mut player_query: Query<Entity, With<Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {

    let texture_handle = asset_server.load("blacknwhite.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(12.0, 12.0), 4, 1, None, Some(Vec2::new(36., 0.)));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let animation_indices = AnimationIndices { first: 0, last: 3 };
    
    if let Ok(plr) = player_query.get_single_mut() {
        let ssb = SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        };
        cmd.entity(plr).insert(ssb);
    }
}


pub fn animate_sprite(
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
                sprite.index + 1
            };
        }
    }
}


pub fn player_debug
(
    player_query: Query<&Transform, With<Player>>
) {
    if let Ok(trans) = player_query.get_single() {
        println!("Player pos: {:?}", trans.translation);
    }
}

pub fn start_game_time(mut commands: Commands){
    commands.insert_resource(NextState(Some(GameState::Running)));
}