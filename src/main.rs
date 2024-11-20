use clap::{Arg, ArgAction, Command};
use std::collections::HashMap;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Define the CLI using clap
    let matches = Command::new("chess_parser")
        .version("0.1.0")
        .author("Polpena <pprokopena@gmail.com")
        .about("Parses PGN files for chess game analysis.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("parse").about("Parses a PGN file.").arg(
                Arg::new("file")
                    .short('f')
                    .long("file")
                    .value_name("FILENAME")
                    .help("Specifies the PGN file to parse.")
                    .required(true)
                    .action(ArgAction::Set),
            ),
        )
        .subcommand(Command::new("credits").about("Displays credits information."))
        .get_matches();

    match matches.subcommand() {
        Some(("parse", sub_m)) => {
            // Handle the 'parse' command
            let filename = sub_m.get_one::<String>("file").unwrap();

            // Read the PGN file
            let contents = match fs::read_to_string(filename) {
                Ok(c) => c,
                Err(error) => {
                    eprintln!("Error reading file '{}': {}", filename, error);
                    return Ok(());
                }
            };

            match chess_parser::parse_pgn(&contents) {
                Ok(game) => {
                    println!("VALID PGN GAME");
                    println!("Result: {}\n", game.result);

                    println!("Metadata length: {}", game.metadata.len());
                    for (key, value) in &game.metadata {
                        println!("Meta || {} : {}", key, value);
                    }
                    println!();

                    println!("Turn count: {}", game.turns.len());
                    for turn in &game.turns {
                        print!("Turn {}:\t", turn.turn_number);
                        if let Some(white_move) = &turn.white_move {
                            print!("white: {}\t", white_move.full_str);
                        } else {
                            print!("white: None\t");
                        }
                        if let Some(black_move) = &turn.black_move {
                            println!("black: {}", black_move.full_str);
                        } else {
                            println!("black: None");
                        }
                    }
                    println!();

                    // Example of analysis that can be done with parsed data
                    // Identify the opening based on the first few moves
                    let mut opening_moves = Vec::new();
                    for turn in &game.turns {
                        if let Some(white_move) = &turn.white_move {
                            opening_moves.push(white_move.full_str.clone());
                        }
                        if let Some(black_move) = &turn.black_move {
                            opening_moves.push(black_move.full_str.clone());
                        }
                        if opening_moves.len() >= 6 {
                            break; // Analyze the first 6 moves (3 turns)
                        }
                    }

                    // Concatenate the moves into a single string
                    let opening_sequence = opening_moves.join(" ");

                    // A simple mapping of opening sequences to opening names
                    let openings = vec![
                        ("e4 e5 Nf3 Nc6 Bb5 a6", "Ruy Lopez"),
                        ("e4 c5", "Sicilian Defense"),
                        ("d4 d5 c4", "Queen's Gambit"),
                        // etc
                    ];
                    // Identify the opening
                    let opening_name = openings.iter().find_map(|(sequence, name)| {
                        if opening_sequence.starts_with(sequence) {
                            Some(*name)
                        } else {
                            None
                        }
                    });

                    if let Some(name) = opening_name {
                        println!("Identified Opening: {}\n", name);
                    } else {
                        println!("Opening not recognized.\n");
                    }

                    // Analyze piece activity
                    let mut piece_activity: HashMap<char, usize> = HashMap::new();

                    for turn in &game.turns {
                        for chess_move in [&turn.white_move, &turn.black_move] {
                            if let Some(mv) = chess_move {
                                let piece = mv.piece.to_ascii_uppercase();
                                *piece_activity.entry(piece).or_insert(0) += 1;
                            }
                        }
                    }
                    println!("Piece Activity:");
                    for (piece, count) in &piece_activity {
                        let piece_name = match piece {
                            'P' => "Pawn",
                            'N' => "Knight",
                            'B' => "Bishop",
                            'R' => "Rook",
                            'Q' => "Queen",
                            'K' => "King",
                            _ => "Unknown",
                        };
                        println!("Piece: {:<6} Moves: {}", piece_name, count);
                    }
                    println!();

                    // Calculate material balance over time
                    let mut white_material = 39; // Initial material value
                    let mut black_material = 39;

                    let mut material_balance_over_time = Vec::new();

                    for turn in &game.turns {
                        for (player, chess_move) in
                            [("white", &turn.white_move), ("black", &turn.black_move)]
                        {
                            if let Some(mv) = chess_move {
                                if mv.capture {
                                    // For demonstration, subtract an average piece value
                                    let average_piece_value = 1;
                                    if player == "white" {
                                        black_material -= average_piece_value;
                                    } else {
                                        white_material -= average_piece_value;
                                    }
                                }
                            }
                        }

                        let balance = white_material as isize - black_material as isize;
                        material_balance_over_time.push(balance);
                    }

                    println!("Material Balance Over Time (White - Black):");
                    for (i, balance) in material_balance_over_time.iter().enumerate() {
                        println!("After turn {}: {}", i + 1, balance);
                    }
                    println!();
                }
                Err(error) => {
                    eprintln!("Parsing error: {}", error);
                }
            }
        }
        Some(("credits", _)) => {
            // Handle the 'credits' command
            println!("Chess Parser 0.1.0");
            println!("Developed by Polpena <pprokopena@gmail.com");
            println!("This software uses the following open-source crates:");
            println!("- clap");
            println!("- pest");
            println!("Thank you for using Chess Parser!");
        }
        _ => {
            // Shouldn't reach here because of 'arg_required_else_help'
            unreachable!();
        }
    }

    Ok(())
}
