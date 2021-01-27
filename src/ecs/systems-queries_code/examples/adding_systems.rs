// In this example, we show the structure of a reasonably complex slime volleyball game
// Some of the details around handling graphics are omitted for brevity

use bevy::app::startup_stage;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        // Startup systems are only run once
        .add_startup_system(setup_cameras.system())
        .add_resource(ArenaDimensions { x: 800.0, y: 600.0 })
        .add_startup_system(create_arena.system())
        // Ordinary systems run every frame
        .add_system(collisions.system())
        // We don't need a kinematics system: position and velocity are updated automatically
        // As long as we use Transform and include TransformPlugin from the DefaultPlugins
        // You can add multiple copies of the same function as different systems
        // System functions can be generic, allowing you to specialize behavior based on type
        .add_system(controls::<Player1>.system())
        .add_system(controls::<Player2>.system())
        // You can control when systems run by setting the Stage they are in
        // Every system in a Stage must complete before the scheduler can advance to the next
        // These are only in a separate stage for demonstration purposes
        .add_startup_system_to_stage(
            startup_stage::POST_STARTUP,
            create_slime::<Player1>.system(),
        )
        .add_startup_system_to_stage(
            startup_stage::POST_STARTUP,
            create_slime::<Player2>.system(),
        )
        .add_startup_system_to_stage(startup_stage::POST_STARTUP, create_ball.system())
        // We can modify our AppBuilder in whatever order we want
        // Although be careful because the methods are processed in order
        .init_resource::<Score>()
        .add_system(display_score.system())
        // We're passing information about a point being scored between our systems with an event
        .add_event::<ScoreEvent>()
        // We couldn't swap this line with the one above, because it relies on the Score resource
        .add_system_to_stage(stage::POST_UPDATE, check_for_points.system())
        // By moving this to a later stage, we can be sure that these runs after we finish checking for points
        .add_system_to_stage(stage::LAST, update_score.system())
        .add_system_to_stage(stage::LAST, reset_ball.system())
        .run();
}

fn setup_cameras(commands: &mut Commands) {
    // Spawn a 2D camera with commands.spawn and Camera2dBundle

    // Spawn a UI camera with commands.spawn and CameraUiBundle
}

struct ArenaDimensions {
    x: f32,
    y: f32,
}

// This is a marker component to determine whether an object collides
struct Collides;

fn create_arena(commands: &mut Commands, arena_dimensions: Res<'_, &'static ArenaDimensions>) {
    // Read in the dimensions from our ArenaDimensions resource

    // Use commands.spawn and SpriteBundle to draw the arena
    // Also add a Collides marker component to the arena using .with
}

// We want to receive the Transform (i.e. a position, velocity etc.)
// for all objects that have the Collides marker component
// So we have a WorldQuery of Transform, and use a QueryFilter of With<Collides>
fn collisions(query: Query<&mut Transform, With<Collides>>) {
    // Handle collisions and set new positions and velocities
}

// These are marker components to denote ownership of various objects
// If we had const generics, we could make this a struct that contained an integer instead :(
struct Player1;
struct Player2;

// Input handling in Bevy is done using Events
// We listen for the input events, then update the velocity of our slimes accordingly
// By making this function generic, we can ensure that players only control the correct slime
// We ensure that this function works for any generic type
// by adding the P: Component trait bound
fn controls<P: Component>(
    slime_query: Query<&mut Transform, (With<Slime>, With<P>)>,
    mut input_event_reader: Local<EventReader<KeyboardInput>>,
    input_events: Res<Events<KeyboardInput>>,
) {
    // Read keyboard inputs

    // Apply the appropriate forces to the correct slime
}

// Another marker component
struct Slime;

// This function spawns a slime, under the control of the appropriate player
fn create_slime<P: Component>(commands: &mut Commands) {
    // Use commands.spawn and SpriteBundle to draw each slime
    // Make sure to add all of the relevant marker components using .with:
    // Collides, Slime and the generic type P to denote which player it belongs to
}

// Yet another marker component!
struct Ball;

fn create_ball(commands: &mut Commands) {
    // Use the same pattern to draw the ball
    // It needs the Collides and Ball marker components
}

#[derive(Default)]
struct Score {
    player_1: usize,
    player_2: usize,
}

// This system will need more arguments to handle rendering appropriately
fn display_score(score: Res<Score>) {
    // Display the score, updating each frame
}

// We can create a custom event type to pass information about scoring between systems
struct ScoreEvent {
    scoring_player: u8,
}

fn check_for_points(
    query: Query<&Transform, With<Ball>>,
    arena_dimensions: Res<'_, &'static ArenaDimensions>,
    score_events: ResMut<ScoreEvent>,
) {
    // Check if the ball is low enough to touch the ground

    // Then use the arena_dimensions to determine whose side of the court it's on

    // Emit an event containing the identity of the player who scored
}

fn update_score(
    mut score: ResMut<Score>,
    mut score_event_reader: Local<EventReader<ScoreEvent>>,
    score_events: Res<Events<KeyboardInput>>,
) {
    // Parse our ScoreEvents, and update our Score resource accordingly
}

// Multiple systems can safely read from the same Event stream
// They are non-consuming, and always persist for one or two ticks
fn reset_ball(
    mut query: Query<&mut Transform, With<Ball>>,
    mut score_event_reader: Local<EventReader<ScoreEvent>>,
    score_events: Res<Events<KeyboardInput>>,
) {
    // If a ScoreEvent has been added to our reader since this system last ran

    // Then, reset the ball's position
}
