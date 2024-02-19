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
        .init_resource::<VirtualTerminal>()
        .init_resource::<FontHandlers>()
        .add_systems(PreStartup, (setup_camera_and_terminal,font_setup))
        .add_systems(Startup, init_virtual_cells)
        .add_systems(PostStartup, add_render_to_cells)
        .add_systems(Update, keyboard_input)
        .run();
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Resource)]
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
            term_rows: 40,
            term_columns: 30,
            term_font_size: 40.0,
            default_bg: bevy::prelude::Color::GRAY,
            default_fg: bevy::prelude::Color::WHITE,
        }
    }
}


#[derive(Resource)]
struct FontHandlers {

    normal: Handle<Font>,


}

impl Default for FontHandlers {

    fn default() -> Self {

        FontHandlers{

            normal: Handle::weak_from_u128(101),
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

fn font_setup(asset_server: Res<AssetServer>, mut font_handlers: ResMut<FontHandlers>) {
    let big_handle : Handle<Font>= asset_server.load("fonts/DejaVuSansMono.ttf");
    font_handlers.normal = big_handle;
  
}

fn setup_camera_and_terminal(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2dBundle::default());
    // Text with one section
 
}

fn init_virtual_cells(mut commands: Commands,terminal_res: Res<VirtualTerminal>, asset_server: Res<AssetServer>) {

    let rows = terminal_res.term_rows;
    let columns = terminal_res.term_columns;

    for y in 0..rows{
        for x in 0..columns {
            commands.spawn((VirtualCell::new(x, y)));



        } 
    }
    
}

fn add_render_to_cells(
    query_cells: Query<(Entity, &VirtualCell)>,
    terminal_res: Res<VirtualTerminal>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    font_handlers: Res<FontHandlers>
) {


    let mut fontsize = terminal_res.term_font_size;
    

    let pixel_shift = fontsize/2.0;

    for (entity_id, cellii) in query_cells.iter() {
        commands.entity(entity_id).insert(
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                &cellii.symbol,
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font: font_handlers.normal.clone(),
                    font_size: fontsize ,
                    ..default()
                },
            ) // Set the justification of the Text
            .with_text_justify(JustifyText::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(cellii.row as f32 *fontsize),
                left: Val::Px(cellii.column as f32 *pixel_shift),
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
