# Chess PGN Parser

A Rust application and library for parsing chess games in PGN (Portable Game Notation) format using a custom grammar. This parser reads PGN files, interprets the game data, and provides structured access to both the metadata and the move sequences contained within. It includes a command-line interface for parsing PGN files and performing basic game analysis.

## Overview

This project implements a PGN parser using the [pest](https://pest.rs/) parsing library in Rust. It defines a custom grammar that accurately represents the structure of PGN files, including metadata headers and the moves of the chess game. The application provides a command-line interface for parsing PGN files and displaying analysis results.

## Table of Contents

- [Features](#features)
- [Technical Description](#technical-description)
  - [Parsing Process](#parsing-process)
    - [Metadata Parsing](#metadata-parsing)
    - [Move Parsing](#move-parsing)
  - [Usage of Parsing Results](#usage-of-parsing-results)
- [Getting Started](#getting-started)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Command-Line Interface](#command-line-interface)
    - [Library Usage](#library-usage)
- [Example](#example)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Accurate Parsing**: Handles standard PGN notation, including complex move annotations.
- **Metadata Extraction**: Parses metadata such as event details, player names, dates, and results.
- **Custom Grammar**: Utilizes a custom-defined grammar for precise control over parsing.
- **Command-Line Interface**: Includes a CLI for parsing PGN files and performing game analysis.
- **Basic Game Analysis**: Provides analysis such as move frequencies, opening identification, piece activity, and material balance.
- **Extensible**: Designed to be extended for additional features like move validation or advanced analysis.

## Technical Description

### Parsing Process

The parser reads a PGN file and processes it in two main parts:

#### Metadata Parsing

The PGN metadata consists of header lines enclosed in square brackets `[]`, each containing a tag name and a tag value in quotes. For example:

```pgn
[Event "F/S Return Match"]
[Site "Belgrade, Serbia JUG"]
[Date "1992.11.04"]
```

The parser uses the following grammar rules to parse the metadata:

```ebnf
metadata      = { "[" ~ meta_key ~ WHITESPACE? ~ "\"" ~ meta_value ~ "\"" ~ "]" }
meta_key      = { ASCII_ALPHA+ }
meta_value    = { (!"\"" ~ ANY)+ }
```

- **Meta Key**: One or more ASCII alphabetic characters.
- **Meta Value**: Any sequence of characters except the closing double quote `"`, allowing spaces and special characters.

#### Move Parsing

After the metadata, the parser processes the sequence of chess moves. Each move includes the turn number, white's move, and black's move. The moves can include various annotations and notation such as castling, captures, promotions, and disambiguations.

The core grammar rules for move parsing are:

```ebnf
chess_game    = { metadata* ~ (turn_number ~ chess_turn)* ~ game_term }

turn_number   = { ASCII_DIGIT+ ~ "." }
chess_turn    = { chess_move ~ chess_move }
chess_move    = { (special | pawn_move | pawn_capture | piece_move) ~ annotation? }
```

- **Special Moves**: Castling moves (`O-O` for kingside and `O-O-O` for queenside).
- **Pawn Moves**: Simple pawn advances and captures.
- **Piece Moves**: Moves involving pieces (King, Queen, Rook, Bishop, Knight) with possible disambiguation.

The parser handles complex move notations, including:

- **Disambiguation**: When two identical pieces can move to the same square, the notation specifies the originating file (column), rank (row), or both.
- **Captures**: Indicated by `x`.
- **Promotions**: Pawn promotions indicated by `=` followed by the piece symbol.
- **Annotations**: Check `+`, checkmate `#`, and other annotations like `!` and `?`.

### Usage of Parsing Results

After parsing, the PGN data is structured into data structures allowing programmatic access to:

- **Metadata**: Event information, players, date, result, etc.
- **Moves**: Detailed move sequences, including annotations and special moves.

This structured data can be used for various purposes:

- **Game Analysis**: Analyzing move patterns, detecting tactics, or evaluating strategies.
- **Visualization**: Rendering the game moves on a chessboard UI.
- **Data Extraction**: Collecting statistics from multiple games, such as opening repertoires or player performance.

## Getting Started

### Installation

Clone the repository and build the project:

```sh
git clone https://github.com/polpena/chess_parser.git
cd chess_parser
cargo build --release
```

Alternatively, you can add the parser as a dependency in your Rust project:

```toml
[dependencies]
chess_parser = { git = "https://github.com/polpena/chess_parser.git" }
```

### Usage

#### Command-Line Interface

The application includes a command-line interface (CLI) for parsing PGN files and performing basic game analysis.

**Running the Application**

After building the project, run the application using:

```sh
cargo run --release -- <SUBCOMMAND> [OPTIONS]
```

**Available Subcommands**

- `parse`: Parses a PGN file and displays analysis.
- `credits`: Displays credits information.
- `help`: Displays help information.

**Parsing a PGN File**

To parse a PGN file and display analysis:

```sh
cargo run --release -- parse --file path/to/game.pgn
```

**Options for `parse`**

- `-f`, `--file <FILENAME>`: Specifies the PGN file to parse (required).

**Example**

```sh
cargo run --release -- parse --file sample_game.pgn
```

**Output**

```
VALID PGN GAME
Result: 1/2-1/2

Metadata:
Event: F/S Return Match
Site: Belgrade, Serbia JUG
Date: 1992.11.04
Round: 29
White: Fischer, Robert J.
Black: Spassky, Boris V.
Result: 1/2-1/2

Turn count: 43
Turn 1:    white: e4     black: e5
Turn 2:    white: Nf3    black: Nc6
...

Identified Opening: Ruy Lopez

Piece Activity:
Piece: Pawn     Moves: 40
Piece: Knight   Moves: 12
Piece: Bishop   Moves: 10
Piece: Rook     Moves: 8
Piece: Queen    Moves: 5
Piece: King     Moves: 4

Material Balance Over Time (White - Black):
After turn 1: 0
After turn 2: 0
...
```

#### Displaying Credits

To display credits information:

```sh
cargo run --release -- credits
```

**Output**

```
Chess Parser 0.1.0
Developed by Polpena <pprokopena@gmail.com>
This software uses the following open-source crates:
- clap
- pest
Thank you for using Chess Parser!
```

#### Library Usage

To use the parser in your Rust code, import the `parse_pgn` function:

```rust
use chess_parser::parse_pgn;
```

## Example

Here's an example demonstrating how to parse a PGN string in a Rust application:

```rust
use chess_parser::parse_pgn;

fn main() {
    let pgn_data = r#"
    [Event "F/S Return Match"]
    [Site "Belgrade, Serbia JUG"]
    [Date "1992.11.04"]
    [Round "29"]
    [White "Fischer, Robert J."]
    [Black "Spassky, Boris V."]
    [Result "1/2-1/2"]

    1.e4 e5 2.Nf3 Nc6 3.Bb5 a6
    4.Ba4 Nf6 5.O-O Be7 6.Re1 b5 7.Bb3 d6 8.c3 O-O 9.h3 Nb8 10.d4 Nbd7
    11.c4 c6 12.cxb5 axb5 13.Nc3 Bb7 14.Bg5 b4 15.Nb1 h6 16.Bh4 c5 17.dxe5
    Nxe4 18.Bxe7 Qxe7 19.exd6 Qf6 20.Nbd2 Nxd6 21.Nc4 Nxc4 22.Bxc4 Nb6
    23.Ne5 Rae8 24.Bxf7+ Rxf7 25.Nxf7 Rxe1+ 26.Qxe1 Kxf7 27.Qe3 Qg5 28.Qxg5
    hxg5 29.b3 Ke6 30.a3 Kd6 31.axb4 cxb4 32.Ra5 Nd5 33.f3 Bc8 34.Kf2 Bf5
    35.Ra7 g6 36.Ra6+ Kc5 37.Ke1 Nf4 38.g3 Nxh3 39.Kd2 Kb5 40.Rd6 Kc5 41.Ra6
    Nf2 42.g4 Bd3 43.Re6 1/2-1/2
    "#;

    match parse_pgn(pgn_data) {
        Ok(game) => {
            let white_player = game.metadata.get("White").unwrap_or(&"Unknown".to_string());
            let black_player = game.metadata.get("Black").unwrap_or(&"Unknown".to_string());
            println!("Parsed game between {} and {}", white_player, black_player);
            println!("Result: {}", game.result);
            // Further processing...
        }
        Err(e) => eprintln!("Error parsing PGN: {}", e),
    }
}
```

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch: `git checkout -b feature/your-feature`.
3. Commit your changes: `git commit -am 'Add your feature'`.
4. Push to the branch: `git push origin feature/your-feature`.
5. Submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

---