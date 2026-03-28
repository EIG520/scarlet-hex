use std::collections::HashSet;

type Coord =  (i16, i16);

#[derive(Clone, Copy)]
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

        let plrmap = match self.is_player_one_turn {
            true => &self.player_one,
            false => &self.player_two,
        };

        for (x,y) in UNITS_POS {
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
        }

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

    pub fn get_winner(&self) -> Option<Player> {
        return self.win_status;
    }
}