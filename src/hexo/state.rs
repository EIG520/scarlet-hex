use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;

pub type Coord =  (i16, i16);

#[derive(Clone, Copy)]
pub enum Player {
    Plr1,
    Plr2,
}

const UNITS_POS: [Coord; 3] = [(1,0), (1,-1), (0,1)];

#[derive(Clone)]
pub struct State {
    player_one: HashSet<Coord>,
    player_two: HashSet<Coord>,

    pub is_player_one_turn: bool,
    has_additional_turn: bool,

    win_status: Option<Player>,
    evaluations: Vec<i32>,
    turns: Vec<Coord>
}

impl State {
    pub fn is_open(&self, tile: Coord) -> bool {
        return !self.player_one.contains(&tile) && !self.player_two.contains(&tile);
    }

    pub fn play_unchecked(&mut self, tile: Coord) {
        self.turns.push(tile);

        let mut eval = *self.evaluations.last().unwrap();

        for (x,y) in UNITS_POS {
            match self.is_player_one_turn {
                true => eval += self.blocks(tile, x, y),
                false => eval -= self.blocks(tile, x, y)                
            }
        }

        match self.is_player_one_turn {
            true => self.player_one.insert(tile),
            false => self.player_two.insert(tile)
        };

        let plrmap = match self.is_player_one_turn {
            true => &self.player_one,
            false => &self.player_two,
        };
        let nplrmap = match self.is_player_one_turn {
            false => &self.player_one,
            true => &self.player_two,
        }; 

        for (x,y) in UNITS_POS {
            // Check for winning
            let mut count: i32 = -1;
            let mut temp_coord = tile;
            while plrmap.contains(&temp_coord) {
                count += 1;
                temp_coord.0 += x;
                temp_coord.1 += y;
            }
            temp_coord = tile;
            while plrmap.contains(&temp_coord) {
                count += 1;
                temp_coord.0 -= x;
                temp_coord.1 -= y;
            }

            if count >= 6 {
                match self.is_player_one_turn {
                    true => self.win_status = Some(Player::Plr1),
                    false => self.win_status = Some(Player::Plr2)
                }
            }
            // my EVIL, rEVULsive EVAL is on another lEVEL as it EVOLves differently based on the order moves are played
            // Update position evaluation
            if self.winnable(nplrmap, plrmap, tile, x, y) {
                match self.is_player_one_turn {
                    true => eval += 2,
                    false => eval -= 2
                }
            }
        }


        self.evaluations.push(eval);

        self.is_player_one_turn = self.is_player_one_turn ^ !self.has_additional_turn;
        self.has_additional_turn = !self.has_additional_turn;
    }

    pub fn winnable(&self, nplrmap:&HashSet<Coord>, plrmap:&HashSet<Coord>,tile: Coord, x: i16, y: i16) -> bool {
        let mut count: i32 = -1;
        let mut ncount: i32 = -2;
        let mut temp_coord = tile;
        while !nplrmap.contains(&temp_coord) && count < 6 {
            if plrmap.contains(&temp_coord) { ncount += 1; }
            count += 1;
            temp_coord.0 += x;
            temp_coord.1 += y;
        }
        temp_coord = tile;
        while !nplrmap.contains(&temp_coord) && count < 12 {
            if plrmap.contains(&temp_coord) { ncount += 1; }
            count += 1;
            temp_coord.0 -= x;
            temp_coord.1 -= y;
        }

        return count >= 6 && ncount > 0;
    }

    pub fn blocks(&self, tile: Coord, x: i16, y: i16) -> i32 {
        let plrmap = match self.is_player_one_turn {
            true => &self.player_one,
            false => &self.player_two,
        };
        let nplrmap = match self.is_player_one_turn {
            false => &self.player_one,
            true => &self.player_two,
        }; 

        let mut blkcnt = 0;

        let mut count: i32 = -1;
        let mut temp_coord = tile;
        while !nplrmap.contains(&temp_coord) && count < 3 {
            count += 1;
            temp_coord.0 += x;
            temp_coord.1 += y;
        }

        if nplrmap.contains(&temp_coord) {
            if self.winnable(plrmap, nplrmap, temp_coord, x, y) {
                blkcnt += 1;
            }
        }

        temp_coord = tile;
        while !nplrmap.contains(&temp_coord) && count < 6 {
            count += 1;
            temp_coord.0 -= x;
            temp_coord.1 -= y;
        }
        if nplrmap.contains(&temp_coord) {
            if self.winnable(plrmap, nplrmap, temp_coord, x, y) {
                blkcnt += 1;
            }
        }

        return blkcnt;
    }

    pub fn unplay(&mut self) {
        let tile = self.turns.pop().unwrap();

        self.player_one.remove(&tile);
        self.player_two.remove(&tile);
        self.evaluations.pop();

        self.has_additional_turn = !self.has_additional_turn;
        self.is_player_one_turn = self.is_player_one_turn ^ !self.has_additional_turn;

        self.win_status = None;
    }

    pub fn eval(&self) -> i32{
        return *self.evaluations.last().unwrap();
    }

    pub fn play(&mut self, tile: Coord) -> Option<()> {
        if self.is_open(tile) {
            self.play_unchecked(tile);
            return Some(());
        }
        return None;
    }

    pub fn get_winner(&self) -> Option<Player> {
        return self.win_status;
    }

    pub fn get_focused_tiles(&self) -> Vec<Coord> {
        let mut tiles = HashSet::<Coord>::new();
        for tile in if self.turns.len() >= 5 {&self.turns[self.turns.len()-5..]} else {&self.turns} {
            for (x,y) in (-3..=3).cartesian_product(-3..=3) {
                if self.is_open((tile.0 + x, tile.1 + y)) {
                    tiles.insert((tile.0 + x, tile.1 + y));
                }
            }
        }

        tiles.iter().map(|&c| c).collect()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            player_one: HashSet::new(),
            player_two: HashSet::new(),
            is_player_one_turn: true,
            has_additional_turn: false,
            win_status: None,
            evaluations: vec![0],
            turns: vec![]
        }
    }
}