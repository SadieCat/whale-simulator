// Whale Simulator (C) 2022 Sadie Powell <sadie@witchery.services>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod entity;
mod game;

use std::io;
use std::process;
use std::thread;
use std::time::{Duration, Instant};

use clap::Parser;

use game::GameState;

#[derive(Parser)]
#[clap(author, version)]
pub struct Args {
    /// How long a game should last for.
    #[clap(default_value_t = 600, long, short, value_name = "SECONDS")]
    pub round_length: u64,

    /// How many times the game should update per second.
    #[clap(default_value_t = 30, long, short, value_name = "COUNT")]
    pub tick_rate: u32,
}

fn main() {
    let args = Args::parse();
    let stdout = io::stdout();

    let mut game = GameState::new(stdout.lock()).unwrap_or_else(|err| {
        eprintln!("An error occurred whilst initializing the game:");
        eprintln!("{}.", err);
        process::exit(1);
    });

    // The time at which the game should end.
    let game_end = Instant::now() + Duration::from_secs(args.round_length);

    // Calculate how long a tick should last for.
    let tick_max = Duration::from_secs(1) / args.tick_rate;

    let mut tick_start = Instant::now();
    while game.tick() && tick_start < game_end {
        // Calculate how long left on this tick and sleep until it is over.
        let tick_length = Instant::now() - tick_start;
        thread::sleep(tick_max - tick_length);

        // Prepare for the next tick.
        tick_start = Instant::now();
    }
    game.end();
}
