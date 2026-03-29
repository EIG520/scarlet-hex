use scarlet_hex::hexo::state::Player;
use scarlet_hex::{self, hexo::state::State};
use scarlet_hex::engine::mcts::Searcher;


pub fn main() {
    let mut state = State::default();
    let mut lastmove = (0,0);
    state.play((0,0));
    // state.play((0,1));
    // state.play((0,2));
    // state.play((1,0));
    // state.play((2,0));
    // state.play((-1,1));
    // state.play((-2,1));
    // state.play((3,0));
    // state.play((4,0));


    // lastmove = (4,0);


    println!("{:?}", lastmove);

    let mut searcher; // = Searcher::new(state.clone(), lastmove);
    let mut inp: String;

    // for _ in 0..10000 {
    //     searcher.grow_once();
    //     // searcher.print();
    // }
    // println!("{:?}", searcher.get_move());
    // searcher.print();


    loop {
        searcher = Searcher::new(state.clone(), lastmove);
        lastmove = searcher.go(10000);
        state.play(lastmove);
        println!("{:?}", lastmove);
        match state.get_winner() {
            None => {},
            Some(Player::Plr1) => {println!("Player One Wins!");return;}
            Some(Player::Plr2) => {println!("Player Two Wins!");return;}
        }

        
        searcher = Searcher::new(state.clone(), lastmove);
        lastmove = searcher.go(10000);
        state.play(lastmove);
        println!("{:?}", lastmove);
        match state.get_winner() {
            None => {},
            Some(Player::Plr1) => {println!("Player One Wins!");return;}
            Some(Player::Plr2) => {println!("Player Two Wins!");return;}
        }

        inp = String::new();
        let _ = std::io::stdin().read_line(&mut inp);
        let mut idcs = inp.split_whitespace();
        lastmove = (idcs.next().unwrap().parse::<i16>().unwrap(), idcs.next().unwrap().parse::<i16>().unwrap());
        state.play(lastmove);
        match state.get_winner() {
            None => {},
            Some(Player::Plr1) => {println!("Player One Wins!");return;}
            Some(Player::Plr2) => {println!("Player Two Wins!");return;}
        }
        inp = String::new();
        let _ = std::io::stdin().read_line(&mut inp);
        let mut idcs = inp.split_whitespace();
        lastmove = (idcs.next().unwrap().parse::<i16>().unwrap(), idcs.next().unwrap().parse::<i16>().unwrap());
        state.play(lastmove);
                match state.get_winner() {
            None => {},
            Some(Player::Plr1) => {println!("Player One Wins!");return;}
            Some(Player::Plr2) => {println!("Player Two Wins!");return;}
        }
    }
}