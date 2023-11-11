const WINNING_COUNTS: &'static [u8] = &[14, 19, 25, 31, 38, 46, 55, 65, 75];

const POINTS_TABLE: &'static[[u8; 8]; 9] = &[
    [3, 4, 5, 7, 8, 9, 11, 12],
    [4, 6, 8, 9, 11, 13, 15, 17],
    [5, 7, 10, 12, 15, 17, 20, 22],
    [6, 9, 12, 15, 19, 22, 25, 28],
    [7, 11, 15, 19, 23, 27, 30, 34],
    [9, 13, 18, 23, 28, 32, 37, 41],
    [11, 16, 22, 27, 33, 38, 44, 50],
    [14, 20, 27, 33, 40, 46, 53, 59],
    [18, 25, 32, 39, 47, 54, 61, 68]
];

pub fn calc_score(player_score: u16, player_level: u8) -> String {
    format!("{0}/{1} {2}% min {3}pts",
        player_score,
        WINNING_COUNTS[player_level as usize],
        points_percent(player_score, player_level),
        player_losing_score(player_score, player_level))
}

pub fn points_percent(player_score: u16, player_level: u8) -> String {
    let big_score: u32 = player_score as u32 * 100 / WINNING_COUNTS[player_level as usize] as u32;
    return big_score.to_string()
}

pub fn player_losing_score(player_score: u16, player_level: u8) -> u8 {
    let losing_scores: [u8; 8] = POINTS_TABLE[player_level as usize];
    let mut score: u8 = 0;
    for i  in 0u8..8u8 {
        if player_score >= losing_scores[i as usize] as u16 {
            score = i+1;
        } else {
            return score
        }
    }
    return score
}