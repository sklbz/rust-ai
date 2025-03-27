#[allow(dead_code)]
pub fn is_move_valid(pgn: &str, move_code: &str) -> bool {
    if is_king_in_check(pgn, move_code) {
        return false;
    }

    true
}

#[allow(dead_code)]
fn is_king_in_check(pgn: &str, move_code: &str) -> bool {
    let _info = [pgn, move_code];
    false
}
