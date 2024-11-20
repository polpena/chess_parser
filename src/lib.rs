pub mod parser {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "grammar.pest"]
    pub struct ChessParser;
}

#[derive(Debug)]
pub struct ChessGame {
    pub metadata: std::collections::HashMap<String, String>,
    pub turns: Vec<ChessTurn>,
    pub result: String,
}

#[derive(Debug)]
pub struct ChessTurn {
    pub turn_number: usize,
    pub white_move: Option<ChessMove>,
    pub black_move: Option<ChessMove>,
}

impl Default for ChessTurn {
    fn default() -> Self {
        ChessTurn {
            turn_number: 0,
            white_move: Some(ChessMove::default()),
            black_move: Some(ChessMove::default()),
        }
    }
}

#[derive(Debug)]
pub struct ChessMove {
    pub full_str: String,
    pub special: bool,
    pub capture: bool,
    pub piece: char,
    pub promotion: char,
    pub loc_col: char,
    pub loc_row: i8,
    pub annotation: String,
    pub disambig: String,
}

impl Default for ChessMove {
    fn default() -> Self {
        ChessMove {
            full_str: String::new(),
            special: false,
            capture: false,
            piece: 'p', // p/K/Q/B/R/N
            loc_col: '-',
            loc_row: 0,
            promotion: ' ',
            annotation: String::new(),
            disambig: String::new(),
        }
    }
}

use crate::parser::ChessParser;
use crate::parser::Rule;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;

pub fn parse_pgn_parse_turn_num(pair: Pair<Rule>) -> usize {
    let number_str = pair.as_str().trim_end_matches('.');
    let number: usize = number_str.parse().expect("Failed to parse number");
    return number;
}

pub fn parse_pgn_parse_getchar(rule: Pair<Rule>) -> char {
    return rule.as_str().to_string().chars().nth(0).unwrap();
}

pub fn parse_pgn_parse_row(rule: Pair<Rule>) -> i8 {
    let number: i8 = rule
        .as_str()
        .to_string()
        .parse()
        .expect("Failed to parse number");
    return number;
}

pub fn parse_pgn_parse_location(loc_pair: Pair<Rule>, mv: &mut ChessMove) {
    for loc_ch in loc_pair.into_inner() {
        match loc_ch.as_rule() {
            Rule::column => {
                mv.loc_col = parse_pgn_parse_getchar(loc_ch);
            }
            Rule::row => {
                mv.loc_row = parse_pgn_parse_row(loc_ch);
            }
            _ => {}
        }
    }
}

pub fn parse_pgn_parse_pawn_cap(pair: Pair<Rule>, mv: &mut ChessMove) {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::column => {
                mv.disambig.push(parse_pgn_parse_getchar(inner));
            }
            Rule::capture => {
                mv.capture = true;
            }
            Rule::location => {
                parse_pgn_parse_location(inner, mv);
            }
            Rule::promotion => {
                for promo_ch in inner.into_inner() {
                    if promo_ch.as_rule() == Rule::piece {
                        mv.promotion = parse_pgn_parse_getchar(promo_ch);
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn parse_pgn_parse_pawn_move(pair: Pair<Rule>, mv: &mut ChessMove) {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::location => {
                parse_pgn_parse_location(inner, mv);
            }
            Rule::promotion => {
                for promo_ch in inner.into_inner() {
                    if promo_ch.as_rule() == Rule::piece {
                        mv.promotion = parse_pgn_parse_getchar(promo_ch);
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn parse_pgn_parse_piece(piece_pair: Pair<Rule>, mv: &mut ChessMove) {
    for inner in piece_pair.into_inner() {
        if inner.as_rule() == Rule::piece {
            mv.piece = parse_pgn_parse_getchar(inner);
        } else if inner.as_rule() == Rule::move_options {
            parse_pgn_parse_move_options(inner, mv);
        }
    }
}

pub fn parse_pgn_parse_move_options(move_pair: Pair<Rule>, mv: &mut ChessMove) {
    let parsed = ChessParser::parse(Rule::options_util, move_pair.as_str());
    if let Ok(parsed2) = parsed {
        for out1 in parsed2 {
            for out2 in out1.into_inner() {
                match out2.as_rule() {
                    Rule::disambig => {
                        mv.disambig = out2.as_str().to_string();
                    }
                    Rule::capture => {
                        mv.capture = true;
                    }
                    Rule::location => {
                        parse_pgn_parse_location(out2, mv);
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn parse_pgn_parse_move(move_pair: Pair<Rule>) -> Option<ChessMove> {
    let mut mv = ChessMove::default();
    mv.full_str = move_pair.as_str().trim().to_string();

    for inner in move_pair.into_inner() {
        match inner.as_rule() {
            Rule::pawn_capture => {
                parse_pgn_parse_pawn_cap(inner, &mut mv);
            }
            Rule::pawn_move => {
                parse_pgn_parse_pawn_move(inner, &mut mv);
            }
            Rule::piece_move => {
                parse_pgn_parse_piece(inner, &mut mv);
            }
            Rule::special => {
                mv.special = true;
            }
            Rule::annotation => {
                mv.annotation = inner.as_str().to_string();
            }
            _ => {}
        }
    }

    return Some(mv);
}

pub fn parse_pgn_parse_gameterm(
    result: &mut String,
    term_pair: Pair<Rule>,
    turns: &mut Vec<ChessTurn>,
) {
    let mut has_halfturn = false;
    let mut halfturn = ChessTurn::default();

    for pair in term_pair.into_inner() {
        match pair.as_rule() {
            Rule::result => {
                *result = pair.as_str().to_string();
            }
            Rule::turn_number => {
                has_halfturn = true;
                halfturn.turn_number = parse_pgn_parse_turn_num(pair);
            }
            Rule::chess_move => {
                has_halfturn = true;
                halfturn.white_move = parse_pgn_parse_move(pair);
            }
            _ => {}
        }
    }

    if has_halfturn {
        turns.push(halfturn);
    }
}

pub fn parse_pgn_parse_metadata(metadata: &mut HashMap<String, String>, meta_pair: Pair<Rule>) {
    let mut mkey = String::new();
    let mut mval: String = String::new();

    for meta_pair in meta_pair.into_inner() {
        match meta_pair.as_rule() {
            Rule::meta_key => {
                mkey = meta_pair.as_span().as_str().to_string();
            }
            Rule::meta_value => {
                mval = meta_pair.as_span().as_str().to_string();
            }
            _ => {}
        }
    }
    metadata.insert(mkey, mval);
}

pub fn parse_pgn_parse_turn(turns: &mut Vec<ChessTurn>, turn_pair: Pair<Rule>) {
    let mut turn = ChessTurn::default();
    let mut i = 0;

    for pair in turn_pair.into_inner() {
        match pair.as_rule() {
            Rule::turn_number => {
                turn.turn_number = parse_pgn_parse_turn_num(pair);
            }
            Rule::chess_move => {
                if i == 1 {
                    turn.white_move = parse_pgn_parse_move(pair)
                } else {
                    turn.black_move = parse_pgn_parse_move(pair)
                }
            }
            _ => {}
        }

        i += 1;
    }

    turns.push(turn);
}

pub fn parse_pgn(pgn_text: &str) -> Result<ChessGame, Error<Rule>> {
    let text_breakless = pgn_text.split_whitespace().collect::<Vec<&str>>().join(" ");

    let mut metadata = std::collections::HashMap::new();
    let mut turns = Vec::new();
    let mut result = String::new();

    let res = ChessParser::parse(Rule::chess_game, &text_breakless);

    if res.is_err() {
        return Err(res.err().unwrap());
    }

    match res {
        Ok(parsed) => {
            for pair in parsed {
                match pair.as_rule() {
                    Rule::chess_game => {
                        for info_pair in pair.into_inner() {
                            match info_pair.as_rule() {
                                Rule::game_term => {
                                    parse_pgn_parse_gameterm(&mut result, info_pair, &mut turns)
                                }
                                Rule::metadata => {
                                    parse_pgn_parse_metadata(&mut metadata, info_pair);
                                }
                                Rule::chess_turn => {
                                    parse_pgn_parse_turn(&mut turns, info_pair);
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(_) => {}
    }

    Ok(ChessGame {
        metadata,
        turns,
        result,
    })
}
