use std::collections::HashSet;

use itertools::Itertools;

use crate::hexo::cset::CoordSet;

pub type Coord =  (i16, i16);

#[derive(Clone, Copy, Debug)]
pub enum Player {
    Plr1,
    Plr2,
}

const UNITS_POS: [Coord; 3] = [(1,0), (1,-1), (0,1)];

#[derive(Clone)]
pub struct State {
    player_one: CoordSet,
    player_two: CoordSet,

    pub is_player_one_turn: bool,
    has_additional_turn: bool,

    win_status: Option<Player>,
    evaluations: Vec<i32>,
    turns: Vec<Coord>
}

impl State {
    pub fn is_open(&self, tile: Coord) -> bool {
        return !self.player_one.contains(tile) && !self.player_two.contains(tile);
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

        // Check for winning
        if plrmap.get_1_run_0(tile) >= 6 || plrmap.get_1_run_1(tile) >= 6 || plrmap.get_1_run_2(tile) >= 6  {
            match self.is_player_one_turn {
                true => {self.win_status = Some(Player::Plr1); eval = 9999},
                false => {self.win_status = Some(Player::Plr2); eval = 9999}
            }
        }

        // my EVIL, rEVULsive EVAL is on another lEVEL as it EVOLves differently based on the order moves are played
        // Update position evaluation
        match self.is_player_one_turn {
            true => eval += self.winnablecnt(nplrmap, plrmap, tile) as i32,
            false => eval -= self.winnablecnt(nplrmap, plrmap, tile) as i32
        }

        self.evaluations.push(eval);

        self.is_player_one_turn = self.is_player_one_turn ^ !self.has_additional_turn;
        self.has_additional_turn = !self.has_additional_turn;
    }

    pub fn winnablecnt(&self, nplrmap:&CoordSet, plrmap:&CoordSet,tile: Coord) -> u32 {
        let mut score = 0;
        
        let count = nplrmap.get_0_run_0(tile);
        let ncount = plrmap.count_1_0r(tile, nplrmap.get_0_run_0r(tile))
            + plrmap.count_1_0l(tile, nplrmap.get_0_run_0l(tile));

        if count >= 6 {
            score += ncount;
        }

        let count = nplrmap.get_0_run_1(tile);
        let ncount = plrmap.count_1_1r(tile, nplrmap.get_0_run_1r(tile))
            + plrmap.count_1_1l(tile, nplrmap.get_0_run_1l(tile));

        if count >= 6 {
            score += ncount;
        }

        let count = nplrmap.get_0_run_2(tile);
        let ncount = plrmap.count_1_2r(tile, nplrmap.get_0_run_2r(tile))
            + plrmap.count_1_2l(tile, nplrmap.get_0_run_2l(tile));

        if count >= 6 {
            score += ncount;
        }

        return score;
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
        while !nplrmap.contains(temp_coord) && count < 3 {
            count += 1;
            temp_coord.0 += x;
            temp_coord.1 += y;
        }

        if nplrmap.contains(temp_coord) {
            blkcnt += self.winnablecnt(plrmap, nplrmap, temp_coord);
        }

        temp_coord = tile;
        while !nplrmap.contains(temp_coord) && count < 6 {
            count += 1;
            temp_coord.0 -= x;
            temp_coord.1 -= y;
        }
        if nplrmap.contains(temp_coord) {
            blkcnt += self.winnablecnt(plrmap, nplrmap, temp_coord);
        }

        return blkcnt as i32;
    }

    pub fn unplay(&mut self) {

        // println!("{:?}", self.turns);

        let tile = self.turns.pop().unwrap();
        self.player_one.remove(tile);
        self.player_two.remove(tile);
        self.evaluations.pop();

        self.has_additional_turn = !self.has_additional_turn;
        self.is_player_one_turn = self.is_player_one_turn ^ !self.has_additional_turn;

        self.win_status = None;
    }

    pub fn eval(&self) -> i32{
        return *self.evaluations.last().unwrap();
    }

    pub fn play(&mut self, tile: Coord) -> Option<()> {
        // println!("play {tile:?} --- {:?}", self.turns);
        if self.is_open(tile) {
            self.play_unchecked(tile);
                // println!("winstat {:?}", self.get_winner());

            return Some(());
        }
        return None;
    }

    pub fn get_winner(&self) -> Option<Player> {
        return self.win_status;
    }

    pub fn get_focused_tiles(&self) -> Vec<Coord> {
        let mut tiles = CoordSet::new();
        for tile in if self.turns.len() >= 5 {&self.turns[self.turns.len()-5..]} else {&self.turns} {
            for (x,y) in (-2..=2).cartesian_product(-2..=2) {
                if self.is_open((tile.0 + x, tile.1 + y)) {
                    tiles.insert((tile.0 + x, tile.1 + y));
                }
            }
        }
        tiles.get_ones()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            player_one: CoordSet::new(),
            player_two: CoordSet::new(),
            is_player_one_turn: true,
            has_additional_turn: false,
            win_status: None,
            evaluations: vec![0],
            turns: vec![]
        }
    }
}