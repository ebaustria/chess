use crate::{ColLabel, Piece, Position, PositionLabel, Tile};
use bevy::ecs::component::Component;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PieceType {
    PAWN,
    BISHOP,
    KNIGHT,
    ROOK,
    QUEEN,
    KING
}

#[derive(Component, Debug, PartialEq, Copy, Clone)]
pub enum Team {
    White,
    Black,
    None,
}

pub struct KingData {
    pub(crate) position: Position,
    pub(crate) available_moves: Vec<Position>,
}

pub fn init_piece_data(current_position: Position) -> (&'static str, Team, PieceType) {
    let Position { position_label, .. } = current_position;
    let PositionLabel { row_label, col_label } = position_label;

    if row_label == 7 {
        return ("bP", Team::Black, PieceType::PAWN);
    }

    if row_label == 2 {
        return ("wP", Team::White, PieceType::PAWN);
    }

    if row_label == 8 {
        let data = match col_label {
            ColLabel::A => ("bR", Team::Black, PieceType::ROOK),
            ColLabel::B => ("bN", Team::Black, PieceType::KNIGHT),
            ColLabel::C => ("bB", Team::Black, PieceType::BISHOP),
            ColLabel::D => ("bQ", Team::Black, PieceType::QUEEN),
            ColLabel::E => ("bK", Team::Black, PieceType::KING),
            ColLabel::F => ("bB", Team::Black, PieceType::BISHOP),
            ColLabel::G => ("bN", Team::Black, PieceType::KNIGHT),
            ColLabel::H => ("bR", Team::Black, PieceType::ROOK)
        };
        return data;
    }

    
    match col_label {
        ColLabel::A => ("wR", Team::White, PieceType::ROOK),
        ColLabel::B => ("wN", Team::White, PieceType::KNIGHT),
        ColLabel::C => ("wB", Team::White, PieceType::BISHOP),
        ColLabel::D => ("wQ", Team::White, PieceType::QUEEN),
        ColLabel::E => ("wK", Team::White, PieceType::KING),
        ColLabel::F => ("wB", Team::White, PieceType::BISHOP),
        ColLabel::G => ("wN", Team::White, PieceType::KNIGHT),
        ColLabel::H => ("wR", Team::White, PieceType::ROOK)
    }
}

pub fn get_possible_moves_for_piece(piece: &Piece, board: &[[Tile; 8]; 8]) -> Vec<Position> {
    match piece.piece_type {
        PieceType::PAWN => possible_moves_for_pawn(piece, board),
        PieceType::BISHOP => possible_moves_for_bishop(piece, board),
        PieceType::KNIGHT => possible_moves_for_knight(piece, board),
        PieceType::ROOK => possible_moves_for_rook(piece, board),
        PieceType::QUEEN => possible_moves_for_queen(piece, board),
        PieceType::KING => possible_moves_for_king(piece, board),
    }
}

fn possible_moves_for_knight(piece: &Piece, board: &[[Tile; 8]; 8]) -> Vec<Position> {
    let mut result = Vec::new();
    let col = piece.position.position_label.col_label as i8;
    let row = (piece.position.position_label.row_label - 1) as i8;

    let up_big = (row + 2) as usize;
    let down_big: i8 = row - 2;
    let left_big: i8 = col - 2;
    let right_big = (col + 2) as usize;

    let left_small: i8 = col - 1;
    let right_small = (col + 1) as usize;
    let up_small = (row + 1) as usize;
    let down_small = row - 1;

    if up_big < 8 {
        if left_small > -1 {
            add_position(piece.team, &board[up_big][left_small as usize], &mut result);
        }
        if right_small < 8 {
            add_position(piece.team, &board[up_big][right_small], &mut result);
        }
    }

    if down_big > -1 {
        if left_small > -1 {
            add_position(piece.team, &board[down_big as usize][left_small as usize], &mut result);
        }
        if right_small < 8 {
            add_position(piece.team, &board[down_big as usize][right_small], &mut result);
        }
    }

    if left_big > -1 {
        if up_small < 8 {
            add_position(piece.team, &board[up_small][left_big as usize], &mut result);
        }
        if down_small > -1 {
            add_position(piece.team, &board[down_small as usize][left_big as usize], &mut result);
        }
    }

    if right_big < 8 {
        if up_small < 8 {
            add_position(piece.team, &board[up_small][right_big], &mut result);
        }
        if down_small > -1 {
            add_position(piece.team, &board[down_small as usize][right_big], &mut result);
        }
    }

    result
}

fn add_position(team: Team, tile: &Tile, result: &mut Vec<Position>) {
    if tile.team != team {
        result.push(tile.position);
    }
}

fn possible_moves_for_king(piece: &Piece, board: &[[Tile; 8]; 8]) -> Vec<Position> {
    let mut result = Vec::new();
    let col = piece.position.position_label.col_label as i8;
    let row = (piece.position.position_label.row_label - 1) as i8;

    let row_upper = (row + 1) as usize;
    let row_lower: i8 = row - 1;
    let col_left: i8 = col - 1;
    let col_right = (col + 1) as usize;

    // check above
    if row_upper < 8 {
        add_position(piece.team, &board[row_upper][col as usize], &mut result);
        if col_left > -1 {
            add_position(piece.team, &board[row_upper][col_left as usize], &mut result);
        }
        if col_right < 8 {
            add_position(piece.team, &board[row_upper][col_right], &mut result);
        }
    }

    // check below
    if row_lower > -1 {
        add_position(piece.team, &board[row_lower as usize][col as usize], &mut result);
        if col_left > -1 {
            add_position(piece.team, &board[row_lower as usize][col_left as usize], &mut result);
        }
        if col_right < 8 {
            add_position(piece.team, &board[row_lower as usize][col_right], &mut result);
        }
    }

    if col_left > -1 {
        add_position(piece.team, &board[row as usize][col_left as usize], &mut result);
    }
    if col_right < 8 {
        add_position(piece.team, &board[row as usize][col_right], &mut result);
    }

    result
}

fn possible_moves_for_queen(piece: &Piece, board: &[[Tile; 8]; 8]) -> Vec<Position> {
    let mut result = Vec::new();
    let radius: i8 = 1;

    upper_vertical_moves(piece, board, radius as u8, &mut result);
    lower_vertical_moves(piece, board, radius, &mut result);
    left_horizontal_moves(piece, board, radius, &mut result);
    right_horizontal_moves(piece, board, radius as u8, &mut result);
    upper_left_diagonal_moves(piece, board, radius, &mut result);
    upper_right_diagonal_moves(piece, board, radius as u8, &mut result);
    lower_left_diagonal_moves(piece, board, radius, &mut result);
    lower_right_diagonal_moves(piece, board, radius, &mut result);

    result
}

fn possible_moves_for_rook(piece: &Piece, board: &[[Tile; 8]; 8]) -> Vec<Position> {
    let mut result = Vec::new();
    let radius: i8 = 1;

    upper_vertical_moves(piece, board, radius as u8, &mut result);
    lower_vertical_moves(piece, board, radius, &mut result);
    left_horizontal_moves(piece, board, radius, &mut result);
    right_horizontal_moves(piece, board, radius as u8, &mut result);

    result
}

fn possible_moves_for_bishop(piece: &Piece, board: &[[Tile; 8]; 8]) -> Vec<Position> {
    let mut result = Vec::new();
    let radius: i8 = 1;

    upper_left_diagonal_moves(piece, board, radius, &mut result);
    upper_right_diagonal_moves(piece, board, radius as u8, &mut result);
    lower_left_diagonal_moves(piece, board, radius, &mut result);
    lower_right_diagonal_moves(piece, board, radius, &mut result);

    result
}

fn lower_vertical_moves(piece: &Piece, board: &[[Tile; 8]; 8], radius: i8, positions: &mut Vec<Position>) {
    let col = piece.position.position_label.col_label as usize;
    let row_check = piece.position.position_label.row_label as i8 - 1 - radius;

    if row_check > -1 && board[row_check as usize][col].team != piece.team {
        let tile: &Tile = &board[row_check as usize][col];
        if tile.team != Team::None {
            positions.push(tile.position);
            return;
        }
        positions.push(tile.position);
        lower_vertical_moves(piece, board, radius + 1, positions);
    }
}

fn upper_vertical_moves(piece: &Piece, board: &[[Tile; 8]; 8], radius: u8, positions: &mut Vec<Position>) {
    let col = piece.position.position_label.col_label as usize;
    let row_check = (piece.position.position_label.row_label - 1 + radius) as usize;

    if row_check < 8 && board[row_check][col].team != piece.team {
        let tile: &Tile = &board[row_check][col];
        if tile.team != Team::None {
            positions.push(tile.position);
            return;
        }
        positions.push(tile.position);
        upper_vertical_moves(piece, board, radius + 1, positions);
    }
}

fn right_horizontal_moves(piece: &Piece, board: &[[Tile; 8]; 8], radius: u8, positions: &mut Vec<Position>) {
    let col_check = (piece.position.position_label.col_label as u8 + radius) as usize;
    let row = (piece.position.position_label.row_label - 1) as usize;

    if col_check < 8 && board[row][col_check].team != piece.team {
        let tile: &Tile = &board[row][col_check];
        if tile.team != Team::None {
            positions.push(tile.position);
            return;
        }
        positions.push(tile.position);
        right_horizontal_moves(piece, board, radius + 1, positions);
    }
}

fn left_horizontal_moves(piece: &Piece, board: &[[Tile; 8]; 8], radius: i8, positions: &mut Vec<Position>) {
    let col_check: i8 = piece.position.position_label.col_label as i8 - radius;
    let row = (piece.position.position_label.row_label - 1) as usize;

    if col_check > -1 && board[row][col_check as usize].team != piece.team {
        let tile: &Tile = &board[row][col_check as usize];
        if tile.team != Team::None {
            positions.push(tile.position);
            return;
        }
        positions.push(tile.position);
        left_horizontal_moves(piece, board, radius + 1, positions);
    }
}

fn lower_right_diagonal_moves(piece: &Piece, board: &[[Tile; 8]; 8], radius: i8, positions: &mut Vec<Position>) {
    let row_check: i8 = piece.position.position_label.row_label as i8 - 1 - radius;
    let col_check = (piece.position.position_label.col_label as i8 + radius) as usize;

    if row_check > -1 && col_check < 8 && board[row_check as usize][col_check].team != piece.team {
        let tile: &Tile = &board[row_check as usize][col_check];
        if tile.team != Team::None {
            positions.push(tile.position);
            return;
        }
        positions.push(tile.position);
        lower_right_diagonal_moves(piece, board, radius + 1, positions);
    }
}

fn lower_left_diagonal_moves(piece: &Piece, board: &[[Tile; 8]; 8], radius: i8, positions: &mut Vec<Position>) {
    let row_check: i8 = piece.position.position_label.row_label as i8 - 1 - radius;
    let col_check: i8 = piece.position.position_label.col_label as i8 - radius;

    if row_check > -1 && col_check > -1 && board[row_check as usize][col_check as usize].team != piece.team {
        let tile: &Tile = &board[row_check as usize][col_check as usize];
        if tile.team != Team::None {
            positions.push(tile.position);
            return;
        }
        positions.push(tile.position);
        lower_left_diagonal_moves(piece, board, radius + 1, positions);
    }
}

fn upper_right_diagonal_moves(piece: &Piece, board: &[[Tile; 8]; 8], radius: u8, positions: &mut Vec<Position>) {
    let row_check = (piece.position.position_label.row_label - 1 + radius) as usize;
    let col_check = (piece.position.position_label.col_label as u8 + radius) as usize;

    if row_check < 8 && col_check < 8 && board[row_check][col_check].team != piece.team {
        let tile: &Tile = &board[row_check][col_check];
        if tile.team != Team::None {
            positions.push(tile.position);
            return;
        }
        positions.push(tile.position);
        upper_right_diagonal_moves(piece, board, radius + 1, positions);
    }
}

fn upper_left_diagonal_moves(piece: &Piece, board: &[[Tile; 8]; 8], radius: i8, positions: &mut Vec<Position>) {
    let row_check = (piece.position.position_label.row_label as i8 - 1 + radius) as usize;
    let col_check: i8 = piece.position.position_label.col_label as i8 - radius;

    if row_check < 8 && col_check > -1 && board[row_check][col_check as usize].team != piece.team {
        let tile: &Tile = &board[row_check][col_check as usize];
        if tile.team != Team::None {
            positions.push(tile.position);
            return;
        }
        positions.push(tile.position);
        upper_left_diagonal_moves(piece, board, radius + 1, positions);
    }
}

fn possible_moves_for_pawn(piece: &Piece, board: &[[Tile; 8]; 8]) -> Vec<Position> {
    let mut result = Vec::new();
    let col = piece.position.position_label.col_label as usize;
    let row = piece.position.position_label.row_label as usize;
    if piece.team == Team::White {
        if row == 2 {
            let row_label = row + 1;
            if board[row_label][col].team == Team::None {
                result.push(board[row_label][col].position);
            }
        }

        if board[row][col].team == Team::None {
            result.push(board[row][col].position);
        }

        attack_moves_for_pawn(board, row, col, Team::Black, &mut result);
        return result;
    }

    // Get moves for black pawns
    if row == 7 {
        let row_label = row - 3;
        if board[row_label][col].team == Team::None {
            result.push(board[row_label][col].position);
        }
    }

    let row_label = row - 2;
    if board[row_label][col].team == Team::None {
        result.push(board[row_label][col].position);
    }

    attack_moves_for_pawn(board, row_label, col, Team::White, &mut result);
    result
}

fn attack_moves_for_pawn(
    board: &[[Tile; 8]; 8],
    row: usize,
    col: usize,
    team: Team,
    result: &mut Vec<Position>
) {
    if col < 7 && board[row][col + 1].team == team {
        result.push(board[row][col + 1].position);
    }

    if col > 0 && board[row][col - 1].team == team {
        result.push(board[row][col - 1].position);
    }
}
