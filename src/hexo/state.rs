use std::collections::HashSet;

type Coord =  (i16, i16);

pub enum Player {
    Plr1,
    Plr2,
}

const UNITS_POS: [Coord; 3] = [(1,0), (1,-1), (0,1)];

pub struct State {
    player_one: HashSet<Coord>,
    player_two: HashSet<Coord>,

    is_player_one_turn: bool,
    has_additional_turn: bool,

    win_status: Option<Player>,
}

impl State {
    pub fn is_open(&self, tile: Coord) -> bool {
        return !self.player_one.contains(&tile) && !self.player_two.contains(&tile);
    }

    pub fn play_unchecked(&mut self, tile: Coord) {
        match self.is_player_one_turn {
            true => self.player_one.insert(tile),
            false => self.player_two.insert(tile)
        };

        self.is_player_one_turn = self.is_player_one_turn ^ !self.has_additional_turn;
        self.has_additional_turn = !self.has_additional_turn;


    }

    pub fn play(&mut self, tile: Coord) -> Option<()> {
        if self.is_open(tile) {
            self.play_unchecked(tile);
            return Some(());
        }
        return None;
    }
}