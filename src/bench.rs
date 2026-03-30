use scarlet_hex::hexo::state::Player;
use scarlet_hex::{self, hexo::state::State};
use scarlet_hex::engine::mcts::Searcher;


pub fn main() {
    let mut state = State::default();
    let lastmove = (0,0);
    state.play((0,0));


    println!("{:?}", lastmove);

    let mut searcher = Searcher::new(state.clone(), lastmove);

    for _ in 0..1000000 {
        searcher.grow_once();
        // searcher.print();
    }
    println!("{:?}", searcher.get_move());
    // searcher.print();
}