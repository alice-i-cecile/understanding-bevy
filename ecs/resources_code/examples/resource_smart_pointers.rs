use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
struct Score(u32);

// These derives let us use Player as a key in our HashMap later
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Player {
    Player1,
    Player2,
}

struct Winner(Player);

// The Display trait lets us control how these types are printed
impl core::fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Player::Player1 => "Player 1",
                Player::Player2 => "Player 2",
            }
        )
    }
}

impl core::fmt::Display for Winner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    App::build()
        .add_plugins(MinimalPlugins)
        // Compound types like this are their own type, so can be fetched nicely by our scheduler
        .init_resource::<HashMap<Player, Score>>()
        .add_startup_system(initialize_scores.system())
        // By the completely unfair rules of our game, Player1 wins ties
        .add_resource(Winner(Player::Player1))
        .add_system(update_score.system())
        .add_system(determine_winner.system())
        .add_system(show_winner.system())
        .run();
}

// Rather than trying to specify a starting value at compile time, we can initialize it with its Default value
// Then we can set it within a system using more complex logic
fn initialize_scores(mut score_map: ResMut<HashMap<Player, Score>>) {
    score_map.insert(Player::Player1, Score(0));
    score_map.insert(Player::Player2, Score(0));
}

// We're modifying the score_map here, so we need to access them mutably with ResMut
// Note that we need mut in front of the parameter name as well
fn update_score(mut score_map: ResMut<HashMap<Player, Score>>) {
    let mut rng = rand::thread_rng();

    for (_, score) in score_map.iter_mut() {
        // We need to access the 0th field of our simple tuple struct Score
        *score = Score(score.0 + rng.gen_range(0..10));
    }
}

// We're only reading our score_map, but need to write to our winner parameter
fn determine_winner(score_map: Res<HashMap<Player, Score>>, mut winner: ResMut<Winner>) {
    // Notice how Rust automatically derefences score_map here
    // This works when we're trying to assign a resource or component to a value
    // Or when we're using a method doesn't exist on our wrapper typ
    let player_1_score = score_map.get(&Player::Player1).unwrap();
    let player_2_score = score_map.get(&Player::Player2).unwrap();

    // You can impl std::comp::Ord on your types to overload your comparison operators
    if player_1_score.0 >= player_2_score.0 {
        // The automatic dereferencing doesn't work here, because we're trying to assign to, rather than access the value
        // So Rust can't infer what we want to do
        *winner = Winner(Player::Player1);
    } else {
        *winner = Winner(Player::Player2);
    }
}

// Finally, we just need to read the scores and winner to print them
fn show_winner(score_map: Res<HashMap<Player, Score>>, winner: Res<Winner>) {
    let player_1_score = score_map.get(&Player::Player1).unwrap();
    let player_2_score = score_map.get(&Player::Player2).unwrap();

    println!("Player 1's score: {}", player_1_score);
    println!("Player 2's score: {}", player_2_score);
    // We want to print the winner, not the reference to the winner
    println!("Right now, {} is the winner!", *winner);
}
