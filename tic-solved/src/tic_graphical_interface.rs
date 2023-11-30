use bevy::{prelude::*, winit::WinitSettings};

#[derive(Component)]
struct HelloWorldText;

fn setup_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(ButtonBundle {
        style: Style {
            width: Val::Vw(50.0),
            height: Val::Vh(50.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: Color::BLUE.into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Hello world!",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/Consolas-Font/CONSOLA.ttf"),
                font_size: 100.0,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        HelloWorldText
    ));
    });
}

fn button_system(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>)
    >
) {
    for interaction in &mut interaction_query {
        println!("{:?}", interaction);
    }
}

fn setup_basic_grid(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(NodeBundle {
        style: Style {
            // Make the height of the node fill its parent
            height: Val::Percent(100.0),
            // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
            // As the height is set explicitly, this means the width will adjust to match the height
            aspect_ratio: Some(1.0),
            // Use grid layout for this node
            display: Display::Grid,
            // Add 24px of padding around the grid
            padding: UiRect::all(Val::Px(24.0)),
            // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
            // This creates 4 exactly evenly sized columns
            grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
            // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
            // This creates 4 exactly evenly sized rows
            grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
            // Set a 12px gap/gutter between rows and columns
            row_gap: Val::Px(12.0),
            column_gap: Val::Px(12.0),
            ..default()
        },
        background_color: BackgroundColor(Color::DARK_GRAY),
        ..default()
    }).with_children(|builder| {
        item_rect(builder, Color::ORANGE);
        item_rect(builder, Color::BISQUE);
        item_rect(builder, Color::BLUE);
        item_rect(builder, Color::CRIMSON);

        item_rect(builder, Color::CYAN);
        item_rect(builder, Color::ORANGE_RED);
        item_rect(builder, Color::DARK_GREEN);
        item_rect(builder, Color::FUCHSIA);

        item_rect(builder, Color::TEAL);
        item_rect(builder, Color::ALICE_BLUE);
        item_rect(builder, Color::CRIMSON);
        item_rect(builder, Color::ANTIQUE_WHITE);

        item_rect(builder, Color::YELLOW);
        item_rect(builder, Color::PINK);
        item_rect(builder, Color::YELLOW_GREEN);
        item_rect(builder, Color::SALMON);
    });
}

fn item_rect(builder: &mut ChildBuilder, color: Color) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            });
        });
}

fn tic_board(builder: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    builder.spawn(NodeBundle {
        style: Style {
            // Make the height of the node fill its parent
            height: Val::Percent(100.0),
            // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
            // As the height is set explicitly, this means the width will adjust to match the height
            aspect_ratio: Some(1.0),
            // Use grid layout for this node
            display: Display::Grid,
            // Add 24px of padding around the grid
            padding: UiRect::all(Val::Px(12.0)),
            // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
            // This creates 4 exactly evenly sized columns
            grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
            // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
            // This creates 4 exactly evenly sized rows
            grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
            // Set a 12px gap/gutter between rows and columns
            row_gap: Val::Px(6.0),
            column_gap: Val::Px(6.0),
            ..default()
        },
        background_color: BackgroundColor(Color::DARK_GRAY),
        ..default()
    }).with_children(|builder| {
        // item_rect(builder, Color::ORANGE);
        // item_rect(builder, Color::BISQUE);
        // item_rect(builder, Color::BLUE);

        // item_rect(builder, Color::CYAN);
        // item_rect(builder, Color::ORANGE_RED);
        // item_rect(builder, Color::DARK_GREEN);

        // item_rect(builder, Color::TEAL);
        // item_rect(builder, Color::ALICE_BLUE);
        // item_rect(builder, Color::CRIMSON);
        place_tile(builder, asset_server);
        place_tile(builder, asset_server);
        place_tile(builder, asset_server);
        place_tile(builder, asset_server);
        place_tile(builder, asset_server);
        place_tile(builder, asset_server);
        place_tile(builder, asset_server);
        place_tile(builder, asset_server);
        place_tile(builder, asset_server);
    });
}

fn place_tile(builder: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(Color::TEAL),
                ..default()
            }).with_children(|builder| {
                builder.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            // margin: UiRect::top(Val::VMin(5.)),
                            ..default()
                        },
                        // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    UiImage::new(asset_server.load("images/x.png")),
                ));
            });
            // builder.spawn(SpriteBundle {
            //     texture: asset_server.load("images/x.png"),
            //     ..default()
            // });
        });
}

fn setup_tic_grid(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(NodeBundle {
        style: Style {
            // Make the height of the node fill its parent
            height: Val::Percent(100.0),
            // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
            // As the height is set explicitly, this means the width will adjust to match the height
            aspect_ratio: Some(1.0),
            // Use grid layout for this node
            display: Display::Grid,
            // Add 24px of padding around the grid
            padding: UiRect::all(Val::Px(12.0)),
            grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
            // Set a 12px gap/gutter between rows and columns
            row_gap: Val::Px(6.0),
            column_gap: Val::Px(6.0),
            ..default()
        },
        background_color: BackgroundColor(Color::DARK_GRAY),
        ..default()
    }).with_children(|builder| {
        tic_board(builder, &asset_server);
        tic_board(builder, &asset_server);
        tic_board(builder, &asset_server);
        tic_board(builder, &asset_server);
        tic_board(builder, &asset_server);
        tic_board(builder, &asset_server);
        tic_board(builder, &asset_server);
        tic_board(builder, &asset_server);
        tic_board(builder, &asset_server);
    });
}

pub fn open_window() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        // .add_systems(Startup, setup_button)
        // .add_systems(Update, button_system)
        // .add_systems(Startup, setup_basic_grid)
        .add_systems(Startup, setup_tic_grid)
        .run();
}