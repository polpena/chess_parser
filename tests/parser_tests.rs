use chess_parser::parse_pgn;

#[test]
fn test_parse_simple_game() {
    let pgn_data = r#"
    [Event "Simple Game"]
    [White "Player1"]
    [Black "Player2"]
    [Result "1-0"]

    1.e4 e5 2.Nf3 Nc6 3.Bb5 a6 1-0
    "#;

    match parse_pgn(pgn_data) {
        Ok(game) => {
            assert_eq!(game.metadata.get("Event"), Some(&"Simple Game".to_string()));
            assert_eq!(game.turns.len(), 3);
            assert_eq!(game.result, "1-0");
        }
        Err(e) => panic!("Parsing failed: {}", e),
    }
}

#[test]
fn test_parse_with_promotion() {
    let pgn_data = r#"
    [Event "Promotion Test"]
    [White "Player1"]
    [Black "Player2"]
    [Result "0-1"]

    1.e4 c5 2.b4 cxb4 3.a3=Q bxa3 4.Nxa3 Nc6 5.Nb5 Nd4 6.Nc7+ Qxc7 7.Rxa7 Nxc2+ 8.Qxc2 Qxc2 9.Rxa8 Qxc1+ 0-1
    "#;

    match parse_pgn(pgn_data) {
        Ok(game) => {
            assert_eq!(game.result, "0-1");

            let mv_prom = game.turns[2].white_move.as_ref().unwrap();
            let mv_nopr = game.turns[2].black_move.as_ref().unwrap();
            assert_eq!(mv_prom.promotion, 'Q');
            assert_eq!(mv_nopr.promotion, ' ');
        }
        Err(e) => panic!("Parsing failed: {}", e),
    }
}

#[test]
fn test_parse_castling() {
    let pgn_data = r#"
    [Event "Castling Test"]
    [White "Player1"]
    [Black "Player2"]
    [Result "1-0"]

    1.e4 e5 2.Nf3 Nc6 3.Bb5 Nf6 4.O-O O-O-O!! 5.Re1 Re8 6.Bxc6 dxc6 7.d3 Bg4 1-0
    "#;

    match parse_pgn(pgn_data) {
        Ok(game) => {
            let white_move = game.turns[3].white_move.as_ref().unwrap();
            let black_move = game.turns[3].black_move.as_ref().unwrap();

            assert_eq!(white_move.full_str, "O-O");
            assert_eq!(white_move.special, true);
            assert_eq!(white_move.promotion, ' ');
            assert_eq!(white_move.annotation, "");

            assert_eq!(black_move.full_str, "O-O-O!!");
            assert_eq!(black_move.special, true);
            assert_eq!(black_move.promotion, ' ');
            assert_eq!(black_move.annotation, "!!");
        }
        Err(e) => panic!("Parsing failed: {}", e),
    }
}

#[test]
fn test_parse_en_passant() {
    let pgn_data = r#"
    [Event "En Passant Test"]
    [White "Player1"]
    [Black "Player2"]
    [Result "1-0"]

    1.e4 d5 2.exd5 c6 3.dxc6 1-0
    "#;

    match parse_pgn(pgn_data) {
        Ok(game) => {
            let mv = game.turns[2].white_move.as_ref().unwrap();
            assert_eq!(mv.full_str, "dxc6");
            assert_eq!(mv.capture, true);
            assert_eq!(mv.piece, 'p');
            assert_eq!(mv.special, false);
            assert_eq!(mv.annotation, "");
            assert_eq!(mv.loc_col, 'c');
            assert_eq!(mv.loc_row, 6);
            assert_eq!(mv.disambig, "d");
        }
        Err(e) => panic!("Parsing failed: {}", e),
    }
}

#[test]
fn test_whitespace() {
    let pgn_text = "1. e4   e5   2.Nf3 Nc6\n3. Bb5 a6 1-0";
    match parse_pgn(pgn_text) {
        Ok(game) => {
            assert_eq!(game.turns.len(), 3);
            assert_eq!(game.metadata.len(), 0);

            assert_eq!("e4", game.turns[0].white_move.as_ref().unwrap().full_str);
            assert_eq!("e5", game.turns[0].black_move.as_ref().unwrap().full_str);

            assert_eq!("Nf3", game.turns[1].white_move.as_ref().unwrap().full_str);
            assert_eq!("Nc6", game.turns[1].black_move.as_ref().unwrap().full_str);

            assert_eq!("Bb5", game.turns[2].white_move.as_ref().unwrap().full_str);
            assert_eq!("a6", game.turns[2].black_move.as_ref().unwrap().full_str);
        }
        Err(e) => panic!("Parsing failed: {}", e),
    }
}

#[test]
fn test_chess_game() {
    let pgn_text = r#"
    [Event "Test Game"]
    [White "Player1"]
    [Black "Player2"]
    [Result "1-0"]

    1.e4 e5 2.Nf3 Nc6 3.Bb5 a6 1-0
    "#;
    let game = parse_pgn(pgn_text).unwrap();
    assert_eq!(game.metadata.get("Event"), Some(&"Test Game".to_string()));
    assert_eq!(game.result, "1-0");
    assert_eq!(game.turns.len(), 3);
}

#[test]
fn test_turn_number() {
    let pgn_text = "12.e4 1-0";
    let game = parse_pgn(pgn_text).unwrap();
    assert_eq!(game.turns.len(), 1);
    assert_eq!(game.turns[0].turn_number, 12);
}

#[test]
fn test_chess_move() {
    let pgn_text = "1.Nf3+ 1-0";
    let game = parse_pgn(pgn_text).unwrap();
    let turn = &game.turns[0];

    let move_info = turn.white_move.as_ref().unwrap();
    assert_eq!(move_info.full_str, "Nf3+");
    assert_eq!(move_info.piece, 'N');
    assert_eq!(move_info.capture, false);
    assert_eq!(move_info.special, false);
    assert_eq!(move_info.disambig, "");
    assert_eq!(move_info.loc_col, 'f');
    assert_eq!(move_info.loc_row, 3);
    assert_eq!(move_info.annotation, "+");
}

#[test]
fn test_pawn_move() {
    let pgn_text = "1.e4 e8=Q 0-1";
    let game = parse_pgn(pgn_text).unwrap();
    let turn = &game.turns[0];

    let move_white = turn.white_move.as_ref().unwrap();
    let move_black = turn.black_move.as_ref().unwrap();
    assert_eq!(move_white.full_str, "e4");
    assert_eq!(move_white.piece, 'p');
    assert_eq!(move_white.special, false);
    assert_eq!(move_white.promotion, ' ');
    assert_eq!(move_white.loc_col, 'e');
    assert_eq!(move_white.loc_row, 4);
    assert_eq!(move_white.disambig, "");

    assert_eq!(move_black.full_str, "e8=Q");
    assert_eq!(move_black.piece, 'p');
    assert_eq!(move_black.special, false);
    assert_eq!(move_black.promotion, 'Q');
    assert_eq!(move_black.loc_col, 'e');
    assert_eq!(move_black.loc_row, 8);
    assert_eq!(move_black.disambig, "");
}

#[test]
fn test_pawn_capture() {
    let pgn_text = "1.exd5 exf8=N 0-1";
    let game = parse_pgn(pgn_text).unwrap();
    let turn = &game.turns[0];

    let white_move = turn.white_move.as_ref().unwrap();
    let black_move = turn.black_move.as_ref().unwrap();
    assert_eq!(white_move.full_str, "exd5");
    assert_eq!(white_move.capture, true);
    assert_eq!(white_move.special, false);
    assert_eq!(white_move.piece, 'p');
    assert_eq!(white_move.disambig, "e");
    assert_eq!(white_move.promotion, ' ');
    assert_eq!(white_move.loc_col, 'd');
    assert_eq!(white_move.loc_row, 5);

    assert_eq!(black_move.full_str, "exf8=N");
    assert_eq!(black_move.capture, true);
    assert_eq!(black_move.special, false);
    assert_eq!(black_move.piece, 'p');
    assert_eq!(black_move.disambig, "e");
    assert_eq!(black_move.promotion, 'N');
    assert_eq!(black_move.loc_col, 'f');
    assert_eq!(black_move.loc_row, 8);
}

#[test]
fn test_piece_symbols() {
    let pgn_text = "1.Ke1 Qe8 2.Re1 Be7 3.Ne2 e4 1/2-1/2";
    let game = parse_pgn(pgn_text).unwrap();
    let turns = game.turns;

    assert_eq!(turns.len(), 3);
    assert_eq!(turns[0].white_move.as_ref().unwrap().piece, 'K');
    assert_eq!(turns[0].black_move.as_ref().unwrap().piece, 'Q');
    assert_eq!(turns[1].white_move.as_ref().unwrap().piece, 'R');
    assert_eq!(turns[1].black_move.as_ref().unwrap().piece, 'B');
    assert_eq!(turns[2].white_move.as_ref().unwrap().piece, 'N');
    assert_eq!(turns[2].black_move.as_ref().unwrap().piece, 'p');
}

#[test]
fn test_location() {
    let pgn_text = "1.e4 h8 1-0";
    let game = parse_pgn(pgn_text).unwrap();
    let turn = &game.turns[0];

    assert_eq!(turn.white_move.as_ref().unwrap().loc_col, 'e');
    assert_eq!(turn.white_move.as_ref().unwrap().loc_row, 4);
    assert_eq!(turn.black_move.as_ref().unwrap().loc_col, 'h');
    assert_eq!(turn.black_move.as_ref().unwrap().loc_row, 8);
}

#[test]
fn test_capture() {
    let pgn_text = "1.dxe5=Q Qd5xd8 1-0";
    let game = parse_pgn(pgn_text).unwrap();
    let turn = &game.turns[0];

    let white_move = turn.white_move.as_ref().unwrap();
    let black_move = turn.black_move.as_ref().unwrap();

    assert_eq!(white_move.capture, true);
    assert_eq!(white_move.special, false);
    assert_eq!(white_move.piece, 'p');
    assert_eq!(white_move.promotion, 'Q');
    assert_eq!(white_move.loc_col, 'e');
    assert_eq!(white_move.loc_row, 5);
    assert_eq!(white_move.disambig, "d");

    assert_eq!(black_move.capture, true);
    assert_eq!(black_move.special, false);
    assert_eq!(black_move.piece, 'Q');
    assert_eq!(black_move.promotion, ' ');
    assert_eq!(black_move.loc_col, 'd');
    assert_eq!(black_move.loc_row, 8);
    assert_eq!(black_move.disambig, "d5");
}

#[test]
fn test_promotion() {
    let pgn_text = "1.e8=Q e7 2. e6=B 1-0";
    let game = parse_pgn(pgn_text).unwrap();

    assert_eq!(game.turns[0].white_move.as_ref().unwrap().promotion, 'Q');
    assert_eq!(game.turns[0].black_move.as_ref().unwrap().promotion, ' ');
    assert_eq!(game.turns[1].white_move.as_ref().unwrap().promotion, 'B');
}

#[test]
fn test_annotation() {
    let pgn_text = "1.e4+ e5# 2.Nf3!? Nc6?! 1/2-1/2";
    let game = parse_pgn(pgn_text).unwrap();

    assert_eq!(game.turns[0].white_move.as_ref().unwrap().annotation, "+");
    assert_eq!(game.turns[0].black_move.as_ref().unwrap().annotation, "#");
    assert_eq!(game.turns[1].white_move.as_ref().unwrap().annotation, "!?");
    assert_eq!(game.turns[1].black_move.as_ref().unwrap().annotation, "?!");
}

#[test]
fn test_game_termination() {
    let pgn_text = "1.e4 1-0";
    let game = parse_pgn(pgn_text).unwrap();
    assert_eq!(game.result, "1-0");
    assert_eq!(game.turns.len(), 1);
    assert_eq!(game.turns[0].black_move.as_ref().unwrap().full_str, "");

    let pgn_text2 = "1.e4 e5 1-0";
    let game2 = parse_pgn(pgn_text2).unwrap();
    assert_eq!(game2.result, "1-0");
    assert_eq!(game2.turns.len(), 1);
    assert_eq!(game2.turns[0].black_move.as_ref().unwrap().full_str, "e5");
}

#[test]
fn test_game_results() {
    let pgn_text1 = "1.e4 e5 1-0";
    let pgn_text2 = "1.e4 e5 0-1";
    let pgn_text3 = "1.e4 e5 1/2-1/2";
    let game1 = parse_pgn(pgn_text1).unwrap();
    let game2 = parse_pgn(pgn_text2).unwrap();
    let game3 = parse_pgn(pgn_text3).unwrap();

    assert_eq!(game1.result, "1-0");
    assert_eq!(game2.result, "0-1");
    assert_eq!(game3.result, "1/2-1/2");
}

#[test]
fn test_columns_and_rows() {
    let columns = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let rows = [1, 2, 3, 4, 5, 6, 7, 8];
    for &col in &columns {
        for &row in &rows {
            let location = format!("{}{}", col, row);
            let pgn_text = format!("1.{} 1-0", location);
            let game = parse_pgn(&pgn_text).unwrap();
            let mv = &game.turns[0].white_move.as_ref().unwrap();

            assert_eq!(mv.full_str, location);
            assert_eq!(mv.loc_col, col);
            assert_eq!(mv.loc_row, row);
        }
    }
}

#[test]
fn test_move_disambiguation() {
    let pgn_text = "1.Nbd7 N1f6 2. Qb5b6 Re8 1-0";
    let game = parse_pgn(pgn_text).unwrap();

    assert_eq!(game.turns[0].white_move.as_ref().unwrap().disambig, "b");
    assert_eq!(game.turns[0].black_move.as_ref().unwrap().disambig, "1");
    assert_eq!(game.turns[1].white_move.as_ref().unwrap().disambig, "b5");
    assert_eq!(game.turns[1].black_move.as_ref().unwrap().disambig, "");
}

#[test]
fn test_metadata() {
    let pgn_text = r#"
    [Event "Test Event"]
    [Site "Test Site"]
    [Date "2021.07.23"]
    [Round "1"]
    [White "White Player"]
    [Black "Black Player"]
    [Result "1-0"]

    1.e4 e5 1-0
    "#;
    let game = parse_pgn(pgn_text).unwrap();
    assert_eq!(game.metadata.get("Event"), Some(&"Test Event".to_string()));
    assert_eq!(game.metadata.get("Site"), Some(&"Test Site".to_string()));
    assert_eq!(game.metadata.get("Date"), Some(&"2021.07.23".to_string()));
    assert_eq!(game.metadata.get("Round"), Some(&"1".to_string()));
    assert_eq!(
        game.metadata.get("White"),
        Some(&"White Player".to_string())
    );
    assert_eq!(
        game.metadata.get("Black"),
        Some(&"Black Player".to_string())
    );
    assert_eq!(game.result, "1-0");
}

#[test]
fn test_invalid_piece() {
    let pgn_data = r#"[Example "nonexistent piece"]
    1. Ue4 O-O 1-0"#;

    let result = parse_pgn(&pgn_data);
    assert!(result.is_err(), "Parser should fail on invalid syntax");
    if let Err(e) = result {
        println!("Parsing error as expected: {}", e);
    }
}

#[test]
fn test_invalid_column() {
    let data = "1. i4 O-O 1-0";

    let result = parse_pgn(data);
    assert!(result.is_err(), "Parser should fail on invalid syntax");
    if let Err(e) = result {
        println!("Parsing error as expected: {}", e);
    }
}

#[test]
fn test_invalid_row() {
    let data = "1. e9 O-O 1-0";

    let result = parse_pgn(data);
    assert!(result.is_err(), "Parser should fail on invalid syntax");
    if let Err(e) = result {
        println!("Parsing error as expected: {}", e);
    }
}

#[test]
fn test_invalid_promotion() {
    let data = "1. e8=K O-O 1-0";

    let result = parse_pgn(data);
    assert!(result.is_err(), "Parser should fail on invalid syntax");
    if let Err(e) = result {
        println!("Parsing error as expected: {}", e);
    }
}

#[test]
fn test_invalid_promotion2() {
    let data = "1. Qe8=B O-O 1-0";

    let result = parse_pgn(data);
    assert!(result.is_err(), "Parser should fail on invalid syntax");
    if let Err(e) = result {
        println!("Parsing error as expected: {}", e);
    }
}

#[test]
fn test_invalid_turn() {
    let data = "1. c5 O-O Nf3 2. e4 1-0";

    let result = parse_pgn(data);
    assert!(result.is_err(), "Parser should fail on invalid syntax");
    if let Err(e) = result {
        println!("Parsing error as expected: {}", e);
    }
}

#[test]
fn test_invalid_missing_result() {
    let data: &str = "1. e4 e5";

    let result = parse_pgn(data);
    assert!(result.is_err(), "Parser should fail on invalid syntax");
    if let Err(e) = result {
        println!("Parsing error as expected: {}", e);
    }
}
#[test]
fn test_invalid_result() {
    let data: &str = "1. e4 e5 1-1";

    let result = parse_pgn(data);
    assert!(result.is_err(), "Parser should fail on invalid syntax");
    if let Err(e) = result {
        println!("Parsing error as expected: {}", e);
    }
}

#[test]
fn test_invalid_weird_move() {
    let data: &str = "1. Ka4e5e6 e4 1-0";

    let result = parse_pgn(data);
    assert!(result.is_err(), "Parser should fail on invalid syntax");
    if let Err(e) = result {
        println!("Parsing error as expected: {}", e);
    }
}

#[test]
fn test_invalid_missing_location() {
    let data = r#"
        1. c Nf3
        2. O-O O-O-O
        3. exd5 Qh5#
        4. Bb5+ Rxe7 1-0"#;

    let result = parse_pgn(data);
    assert!(result.is_err(), "Parser should fail on invalid syntax");
    if let Err(e) = result {
        println!("Parsing error as expected: {}", e);
    }
}

#[test]
fn test_invalid_turn_number() {
    let data = "xy. exd5 Qh5# 1-0";

    let result = parse_pgn(data);
    assert!(result.is_err(), "Parser should fail on invalid syntax");
    if let Err(e) = result {
        println!("Parsing error as expected: {}", e);
    }
}

#[test]
fn test_invalid_annotation() {
    let data = "1. exd5?!? Qh5# 1-0";

    let result = parse_pgn(data);
    assert!(result.is_err(), "Parser should fail on invalid syntax");
    if let Err(e) = result {
        println!("Parsing error as expected: {}", e);
    }
}

#[test]
fn test_invalid_metadata() {
    let data = "[key \"value\" \"value2\"] 1. e4 1-0";

    let result = parse_pgn(data);
    assert!(result.is_err(), "Parser should fail on invalid syntax");
    if let Err(e) = result {
        println!("Parsing error as expected: {}", e);
    }
}
