// Title: Bevy Renderer Plus
// Author: Evan Alvarez
// Description: It renders stuff (very cool)

// Use Some Local Files!
mod spawn_ui;

// Use Some Crates!
use std::f32::consts::PI;

use bevy::app::AppExit;
use bevy::core_pipeline::fxaa::Fxaa;
use bevy::window::PresentMode;
use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_obj::*;
use bevy_stl::*;

use spawn_ui::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Renderer Plus (WIP)".into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<UiState>()
        .init_resource::<ModelScale>()
        .add_plugin(ObjPlugin)
        .add_plugin(StlPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(UIPlugin)
        .add_startup_system(setup)
        .add_system(light_actually_moves)
        .add_system(exit_the_app_gui)
        .add_system(file_drag_and_drop)
        .run();
}

#[derive(Default, Resource)]
struct UiState {
    value: f32,
}

#[derive(Default, Resource)]
struct ModelScale {
    value: f32,
}

fn setup(mut commands: Commands) {
    // Creating the 3D camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-5.0, 7.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Fxaa::default());

    // All of this creates a directional light, aswell as builds shadow cascades
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            maximum_distance: 30.0,
            ..default()
        }
        .into(),
        ..default()
    });
}

// This makes the light move just like the sun actually would
fn light_actually_moves(time: Res<Time>, mut query: Query<&mut Transform, With<DirectionalLight>>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() * 0.05);
    }
}

// This handles exiting the application, and the confirmation dialogue
fn exit_the_app_gui(
    mut exit: EventWriter<AppExit>,
    mut context: EguiContexts,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        egui::Window::new("Do you want to exit?")
            .collapsible(false)
            .resizable(false)
            .show(context.ctx_mut(), |ui| {
                ui.horizontal(|ui| {
                    if ui.button("No").clicked() {
                        info!("This part is not done lol")
                    }
                    if ui.button("Yes").clicked() {
                        exit.send(AppExit);
                    }
                })
            });
    }
}

fn gen_dropped_pieces(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
    scale: f32,
) {
    commands
        .spawn(
            PbrBundle {
                transform: Transform::from_translation(position),
                ..Default::default()
            },
        )
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(0., 0., 0.));
                    transform.scale *= Vec3::new(scale, scale, scale);
                    transform
                },
                ..Default::default()
            });
        });
}

// This handles the 'drag-and-drop' system (for files)
fn file_drag_and_drop(
    mut commands: Commands,
    mut events: EventReader<FileDragAndDrop>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut context: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut model_scale: ResMut<ModelScale>,
) {
    egui::Window::new("TOOLS").show(context.ctx_mut(), |ui| {
        ui.label("SCALE");
        ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=5.0).text("Scale (XYZ)"));
        if ui.button("Save").clicked() {
            model_scale.value = ui_state.value;
            info!("new scale is {}", model_scale.value)
        }
    });

    for event in events.iter() {
        if let FileDragAndDrop::DroppedFile {
            window: _,
            path_buf,
        } = event
        {
            let mesh_handle: Handle<Mesh> = asset_server.load(path_buf.clone());

            gen_dropped_pieces(
                &mut commands,
                materials.add(Color::WHITE.into()),
                mesh_handle.clone(),
                Vec3::new(0., 0., 0.),
                model_scale.value,
            );
        }
    }
}
