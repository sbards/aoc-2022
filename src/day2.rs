use crate::{DaySolution, FromInput};

#[derive(Debug)]
pub struct Day2(Vec<(OpponentMove, PlayerMove)>);

#[derive(Debug)]
enum OpponentMove {
    A,
    B,
    C,
}

#[derive(Debug)]
enum PlayerMove {
    X,
    Y,
    Z,
}

#[derive(Debug)]
enum MatchResult {
    WIN,
    DRAW,
    LOSS,
}

impl FromInput for Day2 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .map(|line| {
                    (
                        OpponentMove::from_char(
                            &line.chars().nth(0).expect("missing opponent's move"),
                        ),
                        PlayerMove::from_char(&line.chars().nth(2).expect("missing player's move")),
                    )
                })
                .collect(),
        )
    }
}

impl DaySolution for Day2 {
    fn part_one(&self) -> String {
        self.0
            .iter()
            .map(|(opp, player)| Day2::score_match(&Day2::play_match(opp, player), player))
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.0
            .iter()
            .map(|(opp, player)| {
                let result = MatchResult::from_player_move(&player);
                let player_move = PlayerMove::find_move(&result, opp);
                Day2::score_match(&result, &player_move)
            })
            .sum::<usize>()
            .to_string()
    }
}

impl Day2 {
    fn score_match(match_result: &MatchResult, player_move: &PlayerMove) -> usize {
        match_result.value() + player_move.value()
    }

    fn play_match(opponent_move: &OpponentMove, player_move: &PlayerMove) -> MatchResult {
        match (opponent_move, player_move) {
            (OpponentMove::A, PlayerMove::Y)
            | (OpponentMove::B, PlayerMove::Z)
            | (OpponentMove::C, PlayerMove::X) => MatchResult::WIN,
            (OpponentMove::A, PlayerMove::X)
            | (OpponentMove::B, PlayerMove::Y)
            | (OpponentMove::C, PlayerMove::Z) => MatchResult::DRAW,
            (OpponentMove::A, PlayerMove::Z)
            | (OpponentMove::B, PlayerMove::X)
            | (OpponentMove::C, PlayerMove::Y) => MatchResult::LOSS,
        }
    }
}

impl OpponentMove {
    fn from_char(input: &char) -> OpponentMove {
        match input {
            'A' => OpponentMove::A,
            'B' => OpponentMove::B,
            'C' => OpponentMove::C,
            _ => panic!("Bad assumptions"),
        }
    }
}

impl PlayerMove {
    fn value(&self) -> usize {
        match self {
            PlayerMove::X => 1,
            PlayerMove::Y => 2,
            PlayerMove::Z => 3,
        }
    }

    fn from_char(input: &char) -> PlayerMove {
        match input {
            'X' => PlayerMove::X,
            'Y' => PlayerMove::Y,
            'Z' => PlayerMove::Z,
            _ => panic!("Bad assumptions"),
        }
    }

    fn find_move(result: &MatchResult, opponent_move: &OpponentMove) -> PlayerMove {
        match (result, opponent_move) {
            (MatchResult::WIN, OpponentMove::A) => PlayerMove::Y,
            (MatchResult::WIN, OpponentMove::B) => PlayerMove::Z,
            (MatchResult::WIN, OpponentMove::C) => PlayerMove::X,
            (MatchResult::DRAW, OpponentMove::A) => PlayerMove::X,
            (MatchResult::DRAW, OpponentMove::B) => PlayerMove::Y,
            (MatchResult::DRAW, OpponentMove::C) => PlayerMove::Z,
            (MatchResult::LOSS, OpponentMove::A) => PlayerMove::Z,
            (MatchResult::LOSS, OpponentMove::B) => PlayerMove::X,
            (MatchResult::LOSS, OpponentMove::C) => PlayerMove::Y,
        }
    }
}

impl MatchResult {
    fn value(&self) -> usize {
        match self {
            MatchResult::WIN => 6,
            MatchResult::DRAW => 3,
            MatchResult::LOSS => 0,
        }
    }

    fn from_player_move(input: &PlayerMove) -> MatchResult {
        match input {
            PlayerMove::X => MatchResult::LOSS,
            PlayerMove::Y => MatchResult::DRAW,
            PlayerMove::Z => MatchResult::WIN,
        }
    }
}
