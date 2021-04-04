use core::f32;

use bevy::prelude::*;
use rand::prelude::*;
use rand::Rng;
use noise::{NoiseFn, Perlin};
use bevy_prototype_lyon::prelude::*;
use bevy::render::pass::ClearColor;
use std::time::Duration;


struct StartPoint;
struct Materials {
    point_material: Handle<ColorMaterial>,
}

struct Dimension {
    x: i32,
    y: i32,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

struct SpawnTimer(Timer);
impl Default for SpawnTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(50), true))
    }
}

fn point_movement(mut point_position: Query<(&StartPoint, &mut Transform)>) {
    for (_point, mut transform) in point_position.iter_mut() {
        transform.translation.y += 2.;
    }
}

fn spawn_circle(
    commands: &mut Commands,
    materials: Res<Materials>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    commands.
        spawn(SpriteBundle {
            material: materials.point_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(StartPoint)
        .with(Position {
            //x: (random::<f32>() * window.width()) as i32,
            x: 100,
            y: (random::<f32>() * window.width()) as i32,
        });
}

fn point_spawner(
    commands: &mut Commands,
    materials: Res<Materials>,
    time: Res<Time>,
    mut timer: Local<SpawnTimer>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    
    if timer.0.tick(time.delta_seconds() * 10000.0).finished() {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0.0..window.width()) as f32;
        let y = rng.gen_range(0.0..window.height()) as f32;

        let draw_line = shapes::Line(Vec2::new(x,y), Vec2::new(x+10.0,y));


        commands
            .spawn(GeometryBuilder::build_as(
                &draw_line,
                materials.point_material.clone(), 
                TessellationMode::Fill(FillOptions::default()), 
                Transform::default()));
    }
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(Materials {
        point_material: materials.add(Color::rgba(0.7, 0.7, 0.7, 0.8).into()),
    });
}

fn main() {
    App::build()
    .add_resource(WindowDescriptor {
        title: "Randomness".to_string(),
        width: 500.0, 
        height: 500.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_startup_system(setup.system())
    .add_system(point_spawner.system())
    .run();
}