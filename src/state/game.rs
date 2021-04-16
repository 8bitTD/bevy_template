use bevy::{prelude::*};
use super::super::state::*;
use rand::random;
use rand::Rng;
pub struct Velocity(Vec2);
pub struct SpawnSprite { pub count: i32 }//Spawn用のスプライト
pub struct Timer{pub count: f32 }//fps計測用のタイマー　0.2秒ごとに更新するため
pub struct FpsText;//fps計測用のテキスト
pub struct CountText;//カウント用のテキスト

pub fn fps(//fps計測する処理
    mut query: Query<&mut Text, With<FpsText>>,
    mut counter: ResMut<Timer>,
    time: Res<Time>, 
){
    counter.count += time.delta_seconds();
    if counter.count < 0.2 {return;}
    let fps = (1.0 / time.delta_seconds()) as i32;
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{0}: {1}","Fps", fps);
    }
    counter.count = 0.0;
}

pub fn count(//Spriteの数を計測する処理
    sprites_query: Query<(Entity, &Sprite)>,
    mut counter: ResMut<SpawnSprite>,
    mut query: Query<&mut Text, With<CountText>>,
){
    let num: i32 = sprites_query.iter().len() as i32;
    if counter.count != num {
        counter.count = num;
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{0}: {1}","Count",counter.count);
        }
    }
}

pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(ClearColor(Color::rgb(0.45, 0.45, 0.45)));
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(5.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
            size: Size::new(Val::Px(110.0), Val::Px(30.0)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: button_materials.normal.clone(),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Title",
                TextStyle {
                    font: asset_server.load(font::E),
                    font_size: font::SIZE,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default(),
            ),
            ..Default::default()
        });
    }).insert(ReleaseResource);
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(40.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
            size: Size::new(Val::Px(110.0), Val::Px(30.0)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: button_materials.normal.clone(),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "GameOver",
                TextStyle {
                    font: asset_server.load(font::E),
                    font_size: font::SIZE,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default(),
            ),
            ..Default::default()
        });
    }).insert(ReleaseResource);
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(75.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
            size: Size::new(Val::Px(110.0), Val::Px(30.0)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: button_materials.normal.clone(),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Ending",
                TextStyle {
                    font: asset_server.load(font::E),
                    font_size: font::SIZE,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default(),
            ),
            ..Default::default()
        });
    }).insert(ReleaseResource);
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
            size: Size::new(Val::Px(110.0), Val::Px(30.0)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: button_materials.normal.clone(),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Spawn",
                TextStyle {
                    font: asset_server.load(font::E),
                    font_size: font::SIZE,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default(),
            ),
            ..Default::default()
        });
    }).insert(ReleaseResource);
    commands.spawn_bundle(ButtonBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(40.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
            size: Size::new(Val::Px(110.0), Val::Px(30.0)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: button_materials.normal.clone(),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Delete",
                TextStyle {
                    font: asset_server.load(font::E),
                    font_size: font::SIZE,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default(),
            ),
            ..Default::default()
        });
    }).insert(ReleaseResource);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(ReleaseResource);
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "State: Game",
            TextStyle {
                font: asset_server.load(font::E),
                font_size: 30.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(0.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }).insert(ReleaseResource);
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Count:",
            TextStyle {
                font: asset_server.load(font::E),
                font_size: 30.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(30.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }).insert(CountText).insert(ReleaseResource);
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Fps:",
            TextStyle {
                font: asset_server.load(font::E),
                font_size: 30.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(0.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }).insert(FpsText).insert(ReleaseResource);
}

pub fn update_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut sprites_query: Query<(Entity, &Sprite)>,
) {
    for (interaction, mut material,children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let text = &text_query.get_mut(children[0]).unwrap().sections[0].value;
                if text == "Spawn"{
                    let texture_handle = asset_server.load(sprite::SPRITE);
                    let material = materials.add(texture_handle.into());
                    let rnd = rand::thread_rng().gen_range(1..system::SPAWN);
                    for _ in 0..rnd {
                        commands.spawn_bundle(SpriteBundle {
                            material: material.clone(),
                            transform: Transform::from_scale(Vec3::splat(0.1)),
                            ..Default::default()
                        })
                        .insert(Velocity(
                            7.0 * Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5),
                        ))
                        .insert(ReleaseResource);
                    }
                }
                else if text == "Delete"{
                    for (entity, _) in sprites_query.iter_mut() {
                        commands.entity(entity).despawn_recursive();
                    }
                }
                else if text == "Title"{ state.set(GameState::Title).unwrap(); }
                else if text == "GameOver" { state.set(GameState::GameOver).unwrap(); }
                else if text == "Ending" { state.set(GameState::Ending).unwrap(); }
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => { *material = button_materials.hovered.clone(); }
            Interaction::None => {*material = button_materials.normal.clone(); }
        }
    }
}

pub fn move_sprite(
    windows: Res<Windows>, 
    mut sprites: Query<(&mut Transform, &mut Velocity)>,
    time: Res<Time>, 
) {
    let window = windows.get_primary().unwrap();
    let width = window.width();
    let height = window.height();
    let left = width / -2.0;
    let right = width / 2.0;
    let bottom = height / -2.0;
    let top = height / 2.0;
    let dt = time.delta_seconds() * system::FPS;
    for (mut transform, mut velocity) in sprites.iter_mut(){
        let tx = transform.translation.x + velocity.0.x * dt;
        let ty = transform.translation.y + velocity.0.y * dt;
        if left < tx && tx < right { transform.translation.x = tx;}
        else{velocity.0.x = -velocity.0.x;}
        if bottom < ty && ty < top {transform.translation.y = ty;}
        else{velocity.0.y = -velocity.0.y;}
        if transform.translation.x < left{transform.translation.x = left;}
        else if transform.translation.x > right{transform.translation.x = right;}
        if transform.translation.y > top {transform.translation.y = top;}
        else if transform.translation.y < bottom{transform.translation.y = bottom;}
    }
}
