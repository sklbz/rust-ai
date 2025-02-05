pub fn is_move_valid(pgn: &str, move_code: &str) -> bool {
    if (is_king_in_check(pgn, move_code)) {
        false
    }

    true
}

fn is_king_in_check(pgn: &str, move_code: &str) -> bool {
    false
}
