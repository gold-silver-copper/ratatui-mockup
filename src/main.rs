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
        .add_systems(PreStartup, setup_camera_and_terminal)
        .add_systems(Startup, init_virtual_cells)
        .add_systems(PostStartup, add_render_to_cells)
        .add_systems(Update, keyboard_input)
        .run();
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct VirtualTerminal {
    term_rows: u16,
    term_columns: u16,
    term_font_size: f32,
    default_bg: BevyColor,
    default_fg: BevyColor,
}

impl Default for VirtualTerminal {
    fn default() -> Self {
        VirtualTerminal {
            term_rows: 10,
            term_columns: 5,
            term_font_size: 40.0,
            default_bg: bevy::prelude::Color::GRAY,
            default_fg: bevy::prelude::Color::WHITE,
        }
    }
}

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct VirtualCell {
    symbol: String,
    fg: BevyColor,
    bg: BevyColor,
    underline_color: Option<BevyColor>,
    skip: bool,
    bold: bool,
    dim: bool,
    italic: bool,
    underlined: bool,
    slow_blink: bool,
    rapid_blink: bool,
    reversed: bool,
    hidden: bool,
    crossed_out: bool,
    row: u16,
    column: u16,
}

impl VirtualCell {
    fn new(x: u16, y: u16) -> Self {
        VirtualCell {
            symbol: "╬".to_string(),
            fg: bevy::prelude::Color::TOMATO,
            bg: bevy::prelude::Color::ORANGE,
            underline_color: None,
            skip: false,
            bold: false,
            dim: false,
            italic: false,
            underlined: false,
            slow_blink: false,
            rapid_blink: false,
            reversed: false,
            hidden: false,
            crossed_out: false,
            row: y,
            column: x,
        }
    }
}

trait FromVirtualTerminal {
    fn to_cell(given_terminal: &VirtualTerminal) -> VirtualCell;
}

impl FromVirtualTerminal for VirtualCell {
    fn to_cell(given_terminal: &VirtualTerminal) -> VirtualCell {
        todo!()
    }
}

trait FromRatCell {
    fn to_virtual(given_cell: &Cell) -> VirtualCell;
}

impl FromRatCell for VirtualCell {
    fn to_virtual(given_cell: &Cell) -> VirtualCell {
        todo!();
    }
}

fn font_setup(asset_server: Res<AssetServer>) {
    todo!()
}

fn setup_camera_and_terminal(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2dBundle::default());
    // Text with one section
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        VirtualTerminal::default(),
        Text2dBundle::default(),
        // ADD TERMINAL OR CELLS HERE,
    ));
}

fn init_virtual_cells(
    query_terminal: Query<(Entity, &VirtualTerminal)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (e, termii) in query_terminal.iter() {
        let rows = termii.term_rows;
        let columns = termii.term_columns;
        let fg = termii.default_fg;
        let bg = termii.default_bg;

        for x in 0..columns {
            for y in 0..rows {
                let child = commands.spawn((VirtualCell::new(x, y))).id();
                println!("{child:?}");

                commands.entity(e).add_child(child);
            }
        }
    }
}

fn add_render_to_cells(
    query_cells: Query<(Entity, &VirtualCell)>,
    query_terminal: Query<&VirtualTerminal>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut fontsize = 200.0;

    for termii in query_terminal.iter() {
        fontsize = termii.term_font_size;
    }

    let pixel_shift = fontsize / 2.0;

    for (entity_id, cellii) in query_cells.iter() {
        let symbole = &cellii.symbol;

        commands.entity(entity_id).insert(
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                symbole,
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font: asset_server.load("fonts/DejaVuSansMono-Oblique.ttf"),
                    font_size: fontsize,
                    ..default()
                },
            ) // Set the justification of the Text
            .with_text_justify(JustifyText::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(cellii.row as f32 * fontsize),
                left: Val::Px(cellii.column as f32 * pixel_shift),
                ..default()
            }),
        );
    }
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        exit.send(AppExit);
    }
}

fn draw_cell(
    x: u16,
    y: u16,
    glyph_cell: &VirtualCell,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let scalar: u16 = 2;
    let glyph_symbol = &glyph_cell.symbol;

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
        // ADD TERMINAL OR CELLS HERE,
    ));

    todo!()
}
