use scarlet_hex::{self, hexo::state::{Player, State}};

pub fn main() {
    let mut state: State = State::default();

    loop {
        let mut inp: String = String::new();
        let _ = std::io::stdin().read_line(&mut inp);

        let mut idcs = inp.split_whitespace();
        state.play((idcs.next().unwrap().parse::<i16>().unwrap(), idcs.next().unwrap().parse::<i16>().unwrap()));

        match state.get_winner() {
            None => {},
            Some(Player::Plr1) => {println!("Player One Wins!");return;}
            Some(Player::Plr2) => {println!("Player Two Wins!");return;}
        }
    }
}