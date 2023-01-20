pub trait TwoPlayerGameState {
    type GameState: TwoPlayerGameState;
    type GameMove;

    fn get_possible_moves(&self) -> Vec<Self::GameMove>;
    fn next_state_with_move(&self, m: &Self::GameMove) -> Self::GameState;
    fn score_state(&self) -> f32;
    fn is_game_over(&self) -> bool;
}


pub fn minimax_alpha_beta<M>(state: &impl TwoPlayerGameState<GameMove=M>, depth: u32, alpha: f32, beta: f32, is_maximizing: bool) -> (f32, Option<M>)
{
    let mut alpha = alpha;
    let mut beta = beta;

    if state.is_game_over(){
        return (state.score_state(), None);
    }

    if depth == 0 {
        return (state.score_state(), None);
    }


    if is_maximizing {
        let mut max_score = f32::NEG_INFINITY;
        let mut best_move: Option<M> = None;
        for m in state.get_possible_moves(){
            let next_state = state.next_state_with_move(&m);

            let (score, _) = minimax_alpha_beta(&next_state, depth - 1, alpha, beta, false);

            if score > max_score  {
                max_score = score;
                best_move = Some(m)
            }

            if max_score > alpha {
                alpha = max_score;
            }

            if max_score >= beta {
                break;
            }
        }
        return (max_score, best_move);
    } else {
        let mut best_move: Option<M> = None;
        let mut min_score = f32::INFINITY;
        for m in state.get_possible_moves(){
            let next_state = state.next_state_with_move(&m);
            let (score, _) = minimax_alpha_beta(&next_state, depth - 1, alpha, beta, true);
            if score < min_score {
                min_score = score;
                best_move = Some(m)
            }

            if min_score < beta {
                beta = min_score;
            }

            if min_score <= alpha {
                break;
            }
        }
        return (min_score, best_move);
    }
}