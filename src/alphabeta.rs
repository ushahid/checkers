pub trait TwoPlayerGameState {
    type GameState: TwoPlayerGameState;
    type GameMove;
    type Player;

    fn get_possible_moves(&self) -> Vec<Self::GameMove>;
    fn next_state_with_move(&self, m: &Self::GameMove) -> Self::GameState;
    fn score_state(&self, turn: &Self::Player) -> f32;
    fn is_game_over(&self) -> bool;
}


pub fn minimax_alpha_beta<S>(state: &S, depth: u32, alpha: f32, beta: f32, is_maximizing: bool, player: &S::Player) -> (f32, Option<S::GameMove>)
    where S: TwoPlayerGameState<GameState=S>
{
    let mut a = alpha;
    let mut b = beta;

    if depth == 0 || state.is_game_over(){
        return (state.score_state(player), None);
    }


    if is_maximizing {
        let mut max_score = f32::NEG_INFINITY;
        let mut best_move: Option<S::GameMove> = None;

        for m in state.get_possible_moves(){
            let next_state = state.next_state_with_move(&m);

            let (score, _) = minimax_alpha_beta(&next_state, depth - 1, a, b, false, player);

            if score > max_score  {
                max_score = score;
                best_move = Some(m)
            }

            if max_score > a {
                a = max_score;
            }

            if max_score >= b {
                break;
            }
        }
        return (max_score, best_move);
    } else {
        let mut best_move: Option<S::GameMove> = None;
        let mut min_score = f32::INFINITY;
        for m in state.get_possible_moves(){
            let next_state = state.next_state_with_move(&m);
            let (score, _) = minimax_alpha_beta(&next_state, depth - 1, a, b, true, player);
            if score < min_score {
                min_score = score;
                best_move = Some(m)
            }

            if min_score < b {
                b = min_score;
            }

            if min_score <= a {
                break;
            }
        }
        return (min_score, best_move);
    }
}