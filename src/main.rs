//! This example illustrates how to create UI text and update it in a system.
//!
//! It displays the current FPS in the top left corner, as well as text that changes color
//! in the bottom right. For text within a scene, please see the text2d example.

use bevy::app::AppExit;
use bevy::prelude::Color as BevyColor;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
//use ratatui::style::Style;
use ratatui::buffer::Cell;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
        .add_systems(Startup, setup)
    
        .add_systems(Update, keyboard_input)
        .run();
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct VirtualTerminal{
    term_rows: u16,
    term_columns: u16,
    term_font_size: u16,
    default_bg: BevyColor,
    default_fg: BevyColor

}

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct VirtualCell{
    fg: BevyColor,
    bg: BevyColor,
    underline_color: Option<BevyColor>,
    skip:bool,
    bold:bool,
    dim:bool,
    italic:bool,
    underlined:bool,
    slow_blink:bool,
    rapid_blink:bool,
    reversed:bool,
    hidden:bool,
    crossed_out:bool,

}

#[derive(Component)]
struct VirtualCellPos{
    x: u16,
    y:u16


}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn(Camera2dBundle::default());
    // Text with one section
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "╬",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/DejaVuSansMono-Oblique.ttf"),
                font_size: 100.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            left: Val::Px(0.0),
            ..default()
        }),
        ColorText,
    ));
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "╬",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/DejaVuSansMono-Oblique.ttf"),
                font_size: 100.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            left: Val::Px(50.0),
            ..default()
        }),
        ColorText,
    ));


}



fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        exit.send(AppExit);
    }
}

fn draw_cell(x: u16, y: u16, glyph_cell: &Cell, mut commands: Commands, asset_server: Res<AssetServer>) {

    let scalar:u16 = 2;
    let glyph_symbol = glyph_cell.symbol();

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            glyph_symbol,
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/DejaVuSansMono-Oblique.ttf"),
                font_size: 30.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(30.0),
            left: Val::Px(30.0),
            width: Val::Px(30.0),
            ..default()
        }),
        ColorText,
    ));

    todo!()
}
