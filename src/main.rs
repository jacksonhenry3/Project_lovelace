use bevy::{math::vec2, prelude::*, render::primitives::Aabb, sprite::MaterialMesh2dBundle};
use bevy_prototype_lyon::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod resources;
use logic_gate::{input_node::InputNode, value::Value, ChipPlugin};
use resources::*;
mod click_and_drag;
use click_and_drag::*;
mod utils;
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},

};

use utils::{get_mouse_position, point_in_region};
use world_control::WorldInteractionPlugin;
pub mod logic_gate;

mod world_control;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            resources::ResourcesPlugin,
            WorldInteractionPlugin,
            ChipPlugin
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (click_and_drag_system,input_node_click_system))
        .add_plugins(ShapePlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mesh_handles: Res<MeshHandles>,
    material_handles: Res<MaterialHandles>,
    mut z_height_manager: ResMut<z_height_manager>,
    mut meshes:ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    colors: Res<Colors>,
) {
    commands.spawn((Camera2dBundle::default(), Draggable::default()));

    let board = commands.spawn((world_control::make_board(&mut
        meshes,
        &material_handles))).id();

    // let mesh_handle = mesh_handles[MeshType::NotGate].clone();
    // let material_handle = material_handles[ColorPallet::Blue].clone();

    logic_gate::chips::NotBundle::spawn(&mut commands,board,&mut z_height_manager, &mesh_handles, &material_handles, vec2(0.0,0.0));
    logic_gate::chips::NotBundle::spawn(&mut commands,board,&mut z_height_manager, &mesh_handles, &material_handles, vec2(0.0,0.0));
    logic_gate::chips::NotBundle::spawn(&mut commands,board,&mut z_height_manager, &mesh_handles, &material_handles, vec2(0.0,0.0));
    logic_gate::chips::NotBundle::spawn(&mut commands,board,&mut z_height_manager, &mesh_handles, &material_handles, vec2(0.0,0.0));

    logic_gate::chips::AndBundle::spawn(&mut commands, board, &mut z_height_manager, &mesh_handles, &material_handles, vec2(0.0, 0.0));
    logic_gate::chips::AndBundle::spawn(&mut commands, board, &mut z_height_manager, &mesh_handles, &material_handles, vec2(0.0, 0.0));
    logic_gate::chips::AndBundle::spawn(&mut commands, board, &mut z_height_manager, &mesh_handles, &material_handles, vec2(0.0, 0.0));
    logic_gate::chips::AndBundle::spawn(&mut commands, board, &mut z_height_manager, &mesh_handles, &material_handles, vec2(0.0, 0.0));

    // 


//     for i in 0..10 {
//         let x_position = i as f32 * 10.0+0.01 ; // Adjust this to set the positions

//         let child = commands.spawn((
//             Draggable {
//                 being_dragged: false,
//                 click_offset: Vec2::ZERO,
//             },
//             MaterialMesh2dBundle {
//                 mesh: mesh_handle.clone(),
//                 material: material_handle.clone(),
//                 transform: Transform::from_xyz(x_position, 0.0, x_position),
//                 ..Default::default()
//             },
//         )).id();
//         commands.entity(board).push_children(&[child]);
//     }

//     // draw a line
//     // draw_line(&mut commands,vec2(0.0, 0.0), vec2(100.0, 100.0), 10.0,&mut meshes, material_handles[ColorPallet::Green].clone());
//     let num_points = 10; // The number of points in the sine wave
// let amplitude = 100.0; // The amplitude of the sine wave
// let frequency = 0.01; // The frequency of the sine wave

// let mut path = Vec::new(); // The array of points

// for i in 0..num_points {
//     let x = 100.0*i as f32 / num_points as f32; // x goes from 0 to 1
//     let y = amplitude * (frequency * x * 2.0 * std::f32::consts::PI).sin(); // y = amplitude * sin(frequency * x)
//     path.push(Vec2::new(x, y));
// }
//     draw_path(&mut commands, &path, 10.0,&mut meshes, material_handles[ColorPallet::Green].clone());


    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(18.75),
                        ..default()
                    },
                    background_color: colors[ColorPallet::InterfaceColorHighlight].into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn((NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                ..default()
                            },
                            background_color: colors[ColorPallet::InterfaceColor].into(),
                            ..default()
                        },Interaction::None))
                        .with_children(|parent| {
                            // text
                            parent.spawn((
                                TextBundle::from_section(
                                    "Project Lovelace",
                                    TextStyle {
                                        font: asset_server
                                            .load("fonts/Fira_Code/static/FiraCode-Bold.ttf"),
                                        font_size: 30.0,
                                        ..default()
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::horizontal(Val::Auto),
                                    ..default()
                                }),
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                            ));
                        });
                });

                
            // right vertical fill
            parent
                .spawn((NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Percent(18.75),
                        ..default()
                    },
                    background_color: colors[ColorPallet::InterfaceColor].into(),
                    ..default()
                },Interaction::None))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        TextBundle::from_section(
                            "Scrolling list",
                            TextStyle {
                                font: asset_server
                                    .load("fonts/Fira_Code/FiraCode-VariableFont_wght.ttf"),
                                font_size: 25.,
                                ..default()
                            },
                        ),
                        Label,
                    ));
                    // List with hidden overflow
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
                                height: Val::Percent(50.),
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                            background_color: Color::rgb(0.10, 0.10, 0.10).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Moving panel
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    ScrollingList::default(),
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                ))
                                .with_children(|parent| {
                                    // List items
                                    for i in 0..32 {
                                        parent.spawn((
                                            TextBundle::from_section(
                                                format!("Item {i}"),
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/Fira_Code/FiraCode-VariableFont_wght.ttf"),
                                                    font_size: 20.,
                                                    ..default()
                                                },
                                            ),
                                            Label,
                                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                        ));
                                    }
                                });
                        });
                });
            

        });
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}

pub(crate) fn input_node_click_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Value, &GlobalTransform, &Aabb), With<InputNode>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return
    }
    let (camera, camera_transform) = camera_query.single();
    // allows for entitys with a click and drag component to be dragged. The entity with the largest z index will be prioritized.
    let Some(mouse_position) = get_mouse_position(camera, camera_transform, windows) else {
        return;
    };


    for (mut input_node, global_transform, aabb) in query.iter_mut() {
        if let Some(_) = point_in_region(
            mouse_position,
            global_transform.translation().xy(),
            aabb.half_extents.xy(),
        ) {
            input_node.0 = !input_node.0;
            break;
        }
    }
}