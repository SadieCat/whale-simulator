// Whale Simulator (C) 2022 Sadie Powell <sadie@witchery.services>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::time::{Duration, Instant};

use rand::{thread_rng, Rng};

use crate::game::Point;

/// The emoji used to represent pesky fishing boats.
pub const BOAT: &'static str = "\u{26F5}";

// The emoji used to represent spiky harpoons.
pub const HARPOON: &'static str = "\u{21D3}";

/// The emoji used to represent delicious krill.
pub const KRILL: &'static str = "\u{1F990}";


/// The emoji used to represent good ratios.
const RATIO_BAD: &'static str = "\u{1F4C9}";

/// The emoji used to represent good ratios.
const RATIO_GOOD: &'static str = "\u{1F4C8}";

/// The emoji used to represent the player.
const WHALE_ALIVE: &'static str = "\u{1F40B}";

/// The emoji used to represent the harpooned player.
const WHALE_DEAD: &'static str = "\u{1F969}";

/// Directions that a player can move in.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Represents the whale player.
pub struct Player {
    // The time the player is harpooned until.
    harpooned_until: Instant,

    /// The number of times the player has been harpooned.
    pub harpoon_count: usize,

    /// The number of krill the player has eaten.
    pub krill_eaten: usize,

    /// The location of the player.
    position: Point,
}

impl Player {
    /// Retrieves the current emoji used to render the player.
    pub fn emoji(&self) -> &'static str {
        if self.harpooned_until > Instant::now() {
            WHALE_DEAD
        } else {
            WHALE_ALIVE
        }
    }

    /// Marks the player as harpooned.
    pub fn harpoon(&mut self) {
        self.harpooned_until = Instant::now() + Duration::from_secs(2);
        self.harpoon_count += 1;
    }

    /// Moves the player entity in the specified direction.
    pub fn migrate(&mut self, size: &Point, direction: Direction) {
        if self.harpooned_until > Instant::now() {
            return; // Can't move whilst harpooned.
        }
        match direction {
            Direction::Up => {
                if self.position.1 > 6 {
                    self.position.1 -= 1;
                }
            }
            Direction::Down => {
                if self.position.1 + 1 <= size.1 {
                    self.position.1 += 1;
                }
            }
            Direction::Left => {
                if self.position.0 >= 2 {
                    self.position.0 -= 2;
                }
            }
            Direction::Right => {
                if self.position.0 + 2 <= size.0 {
                    self.position.0 += 2;
                }
            }
        }
    }

    /// Creates a new player entity.
    pub fn new(dimensions: &Point) -> Self {
        Player {
            krill_eaten: 0,
            harpoon_count: 0,
            harpooned_until: Instant::now(),
            position: (dimensions.0 / 2, dimensions.1 / 2 + 3),
        }
    }

    /// Retrieves the location of the player.
    pub fn position(&self) -> &Point {
        return &self.position;
    }

    /// Calculates the player's krill/death ratio (hehehe).
    pub fn ratio(&self) -> String {
        if self.harpoon_count == 0 {
            return "\u{221E}".to_string(); // Infinity.
        }
        return format!("{:.3}", self.krill_eaten as f32 / self.harpoon_count as f32);
    }

    /// Retrieves the current emoji used to render the ratio graph.
    pub fn ratio_emoji(&self) -> &'static str {
        if self.harpoon_count <= self.krill_eaten {
            RATIO_GOOD
        } else {
            RATIO_BAD
        }
    }
}

/// Represents a juicy krill waiting to be eaten.
pub struct Krill {
    /// The location of the krill.
    position: Point,
}

impl Krill {
    /// Creates a new krill entity.
    pub fn new(size: Point) -> Self {
        let pos_x = thread_rng().gen_range(0..size.0 / 2) * 2;
        let pos_y = thread_rng().gen_range(7..size.1);
        Krill { position: (pos_x, pos_y) }
    }

    /// Retrieves the location of the krill.
    pub fn position(&self) -> &Point {
        return &self.position;
    }
}

/// Represents a pesky fishing boat.
pub struct Boat {
    /// The time at which the next harpoon will be spawned.
    next_harpoon_spawn: Instant,

    /// The location of the boat.
    position: u16,
}

impl Boat {
    fn next_harpoon_spawn() -> Instant {
        Instant::now() + Duration::from_millis(thread_rng().gen_range(5_000..10_000))
    }

    /// Creates a new boat entity.
    pub fn new() -> Self {
        Boat {
            next_harpoon_spawn: Self::next_harpoon_spawn(),
            position: 0,
        }
    }

    /// Checks if it is harpoon time yet.
    pub fn harpoon_time(&mut self) -> bool {
        if self.next_harpoon_spawn < Instant::now() {
            self.next_harpoon_spawn = Self::next_harpoon_spawn();
            true
        } else {
            false
        }
    }

    /// Migrates the boat across the screen.
    pub fn migrate(&mut self) {
        self.position += 2;
    }

    /// Retrieves the location of the boat.
    pub fn position(&self) -> u16 {
        return self.position;
    }
}

/// Represents a spikey harpoon..
pub struct Harpoon {
    /// The location of the harpoon.
    position: Point,
}

impl Harpoon {
    /// Creates a new boat entity.
    pub fn new(boat: &Boat) -> Self {
        Harpoon {
            position: (boat.position(), 6),
        }
    }

    /// Migrates the harpoon down the screen.
    pub fn migrate(&mut self) {
        self.position.1 += 1;
    }

    /// Retrieves the location of the harpoon.
    pub fn position(&self) -> &Point {
        return &self.position;
    }
}
