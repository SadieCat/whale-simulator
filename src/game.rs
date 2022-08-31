// Whale Simulator (C) 2022 Sadie Powell <sadie@witchery.services>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::io::{StdoutLock, Write};
use std::time::{Duration, Instant};

use rand::{thread_rng, Rng};

use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::AsyncReader;
use termion::{clear, cursor};

use crate::entity::{Boat, Direction, Harpoon, Krill, Player, BOAT, HARPOON, KRILL};

/// The emoji used to represent deaths.
const DEATH: &'static str = "\u{1F480}";

/// The raw control sequence to terminate a line.
const LINE_TERMINATOR: &'static str = "\r\n";

/// The emoji used to represent the sea.
const WAVE: &'static str = "\u{1F30A}";

/// Represents a position on the game screen.
pub type Point = (u16, u16);

/// Encapsulates the game state.
pub struct GameState<'a> {
    /// Whether the game is actively runing.
    alive: bool,

    /// Boats which have been spawned.
    boats: Vec<Boat>,

    /// Harpoons which have been spawned.
    harpoons: Vec<Harpoon>,

    /// Krill which have been spawned.
    krill: Vec<Krill>,

    /// The player entity.
    player: Player,

    /// The time at which boats will be moved next.
    next_boat_move: Instant,

    /// The time at which the next boat will be spawned.
    next_boat_spawn: Instant,

    /// The time at which harpoons will be moved next.
    next_harpoon_move: Instant,

    /// The time at which the next krill will be spawned.
    next_krill: Instant,

    /// The size of the terminal we are drawing to.
    size: Point,

    /// The standard input stream.
    stdin: Keys<AsyncReader>,

    /// The standard output stream.
    stdout: RawTerminal<StdoutLock<'a>>,
}

impl<'a> GameState<'a> {
    /// Handles drawing the game on tick.
    fn draw(&mut self) {
        // Reset the terminal.
        write!(self.stdout, "{}{}", clear::All, cursor::Hide).unwrap();

        // Score bar.
        write!(self.stdout, "{}", cursor::Goto(1, 2)).unwrap();
        write!(self.stdout, "  {}  {: <5}", KRILL, self.player.krill_eaten).unwrap();
        write!(self.stdout, "  {}  {: <5}", DEATH, self.player.harpoon_count).unwrap();
        write!(self.stdout, "  {}  {}", self.player.ratio_emoji(), self.player.ratio()).unwrap();

        // Wave line.
        write!(self.stdout, "{}", cursor::Goto(1, 5)).unwrap();
        for _ in 0..self.size.0 / 2 {
            write!(self.stdout, "{}", WAVE).unwrap();
        }

        // Krill.
        for krill in &self.krill {
            let pos = krill.position();
            write!(self.stdout, "{}", cursor::Goto(pos.0, pos.1)).unwrap();
            write!(self.stdout, "{}", KRILL).unwrap();
        }

        // Boats.
        for boat in &self.boats {
            write!(self.stdout, "{}", cursor::Goto(boat.position(), 4   )).unwrap();
            write!(self.stdout, "{}", BOAT).unwrap();
        }

        // Harpoons.
        for harpoon in &self.harpoons {
            let pos = harpoon.position();
            write!(self.stdout, "{}", cursor::Goto(pos.0, pos.1)).unwrap();
            write!(self.stdout, "{}", HARPOON).unwrap();
        }

        // Player.
        let pos = self.player.position();
        write!(self.stdout, "{}", cursor::Goto(pos.0, pos.1)).unwrap();
        write!(self.stdout, "{}", self.player.emoji()).unwrap();

        // Flush all updates.
        self.stdout.flush().unwrap();
    }

    /// Prints statistics and cleans up the terminal.
    pub fn end(&mut self) {
        // Show the game statistics.
        write!(self.stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
        write!(self.stdout, "Thanks for playing Whale Simulator!{}", LINE_TERMINATOR).unwrap();
        write!(self.stdout, "You ate {} delicious krill and were harpooned {} time(s).{}", self.player.krill_eaten, self.player.harpoon_count, LINE_TERMINATOR).unwrap();
        write!(self.stdout, "Your krill/death ratio was {}.{}", self.player.ratio(), LINE_TERMINATOR).unwrap();

        // Attempt to reset the terminal back to how it was before.
        write!(self.stdout, "{}", cursor::Show).unwrap();
        self.stdout.flush().unwrap();
    }

    /// Handles keyboard input on tick.
    fn input(&mut self) {
        while let Some(Ok(key)) = self.stdin.next() {
            match key {
                Key::Up | Key::Char('w') | Key::Char('k') => {
                    self.player.migrate(&self.size, Direction::Up)
                }
                Key::Down | Key::Char('s') | Key::Char('j') => {
                    self.player.migrate(&self.size, Direction::Down)
                }
                Key::Left | Key::Char('a') | Key::Char('h') => {
                    self.player.migrate(&self.size, Direction::Left)
                }
                Key::Right | Key::Char('d') | Key::Char('l') => {
                    self.player.migrate(&self.size, Direction::Right)
                }
                Key::Esc | Key::Char('q') => self.alive = false,
                _ => (),
            }
        }
    }

    pub fn new(stdout: StdoutLock<'a>) -> Result<Self, String> {
        let async_stdin = termion::async_stdin().keys();

        let raw_stdout = stdout
            .into_raw_mode()
            .map_err(|err| format!("Unable to switch STDOUT to raw mode: {}", err))?;

        let size = termion::terminal_size()
            .map_err(|err| format!("Unable to retrieve the terminal size: {}", err))?;

        // Check we have space to render the UI and give a bit of play space.
        if size.0 < 20 || size.1 < 15 {
            return Err(format!("The terminal must be at least 20x15 (currently {}x{})", size.0, size.1));
        }

        Ok(GameState {
            alive: true,
            boats: Vec::new(),
            harpoons: Vec::new(),
            krill: Vec::new(),
            next_boat_move: Instant::now(),
            next_boat_spawn: Instant::now(),
            next_harpoon_move: Instant::now(),
            next_krill: Instant::now(),
            player: Player::new(&size),
            size,
            stdin: async_stdin,
            stdout: raw_stdout,
        })
    }

    /// Handles game behaviours on tick.
    fn think(&mut self) {
        // Check if the player has been jabbed with a harpoon.
        let harpoon_count = self.harpoons.len();
        self.harpoons.retain(|h| h.position() != self.player.position());
        if harpoon_count - self.harpoons.len() != 0 {
            self.player.harpoon();
        }

        // Check if the player has eaten any krill.
        let krill_count = self.krill.len();
        self.krill.retain(|k| k.position() != self.player.position());
        self.player.krill_eaten +=  krill_count - self.krill.len();

        // Check if any boats need to be culled or moved.
        if self.next_boat_move < Instant::now() {
            self.boats.retain(|b| b.position() + 2 < self.size.0);
            for boat in &mut self.boats {
                boat.migrate();
            }
            self.next_boat_move = Instant::now() + Duration::from_millis(1_000);
        }

        // Check if an harpoons need to be culled or moved.
        if self.next_harpoon_move < Instant::now() {
            self.harpoons.retain(|b| b.position().1 + 1 < self.size.1);
            for harpoon in &mut self.harpoons {
                harpoon.migrate();
            }
            self.next_harpoon_move = Instant::now() + Duration::from_millis(250)
        }

        // Potentially spawn some more krill for the player to eat.
        let max_krill = self.size.0 * self.size.1 / 100;
        if self.krill.len() < max_krill as usize && self.next_krill < Instant::now() {
            self.krill.push(Krill::new(self.size));
            self.next_krill = Instant::now() + Duration::from_millis(thread_rng().gen_range(500..5_000));
        }

        // Potentially spawn some new boats.
        if self.next_boat_spawn < Instant::now() {
            self.boats.push(Boat::new());
            self.next_boat_spawn = Instant::now() + Duration::from_millis(thread_rng().gen_range(2_500..5_000));
        }
        // Potentially spawn some new harpoons.
        for boat in &mut self.boats {
            if boat.harpoon_time() {
                self.harpoons.push(Harpoon::new(boat))
            }
        }
    }

    /// Called every time the game needs to update.
    pub fn tick(&mut self) -> bool {
        self.input();
        self.think();
        self.draw();
        self.alive
    }
}
