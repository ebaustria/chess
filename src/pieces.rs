use crate::{ ColLabel, Piece, Position, PositionLabel, Team, Tile };

#[derive(PartialEq, Copy, Clone)]
pub enum PieceType {
    PAWN,
    BISHOP,
    KNIGHT,
    ROOK,
    QUEEN,
    KING
}

pub fn get_piece_data(current_position: Position) -> (&'static str, Team, PieceType) {
    let Position { position_label, .. } = current_position;
    let PositionLabel { row_label, col_label } = position_label;

    if row_label == 7 {
        return ("bP", Team::BLACK, PieceType::PAWN);
    }

    if row_label == 2 {
        return ("wP", Team::WHITE, PieceType::PAWN);
    }

    if row_label == 8 {
        let data = match col_label {
            ColLabel::A => ("bR", Team::BLACK, PieceType::ROOK),
            ColLabel::B => ("bN", Team::BLACK, PieceType::KNIGHT),
            ColLabel::C => ("bB", Team::BLACK, PieceType::BISHOP),
            ColLabel::D => ("bQ", Team::BLACK, PieceType::QUEEN),
            ColLabel::E => ("bK", Team::BLACK, PieceType::KING),
            ColLabel::F => ("bB", Team::BLACK, PieceType::BISHOP),
            ColLabel::G => ("bN", Team::BLACK, PieceType::KNIGHT),
            ColLabel::H => ("bR", Team::BLACK, PieceType::ROOK)
        };
        return data;
    }

    let data = match col_label {
        ColLabel::A => ("wR", Team::WHITE, PieceType::ROOK),
        ColLabel::B => ("wN", Team::WHITE, PieceType::KNIGHT),
        ColLabel::C => ("wB", Team::WHITE, PieceType::BISHOP),
        ColLabel::D => ("wQ", Team::WHITE, PieceType::QUEEN),
        ColLabel::E => ("wK", Team::WHITE, PieceType::KING),
        ColLabel::F => ("wB", Team::WHITE, PieceType::BISHOP),
        ColLabel::G => ("wN", Team::WHITE, PieceType::KNIGHT),
        ColLabel::H => ("wR", Team::WHITE, PieceType::ROOK)
    };
    return data;
}

pub fn get_possible_moves_for_piece(piece: &Piece, board: &[[Tile; 8]; 8]) -> Vec<Position> {
    return match piece.piece_type {
        PieceType::PAWN => possible_moves_for_pawn(piece, board),
        PieceType::BISHOP => Vec::new(),
        PieceType::KNIGHT => Vec::new(),
        PieceType::ROOK => Vec::new(),
        PieceType::QUEEN => Vec::new(),
        PieceType::KING => Vec::new(),
    };
}

fn possible_moves_for_pawn(piece: &Piece, board: &[[Tile; 8]; 8]) -> Vec<Position> {
    let mut result = Vec::new();
    if piece.team == Team::WHITE {
        let pos_label = piece.position.position_label;
        if pos_label.row_label == 2 {
            let row_label = pos_label.row_label + 1;
            if board[row_label as usize][pos_label.col_label as usize].team == Team::NONE {
                result.push(board[row_label as usize][pos_label.col_label as usize].position);
            }
        }

        if board[pos_label.row_label as usize][pos_label.col_label as usize].team == Team::NONE {
            result.push(board[pos_label.row_label as usize][pos_label.col_label as usize].position);
        }

        if board[pos_label.row_label as usize][(pos_label.col_label as u8 + 1) as usize].team == Team::BLACK {
            result.push(board[pos_label.row_label as usize][(pos_label.col_label as u8 + 1) as usize].position);
        }

        if board[pos_label.row_label as usize][(pos_label.col_label as u8 - 1) as usize].team == Team::BLACK {
            result.push(board[pos_label.row_label as usize][(pos_label.col_label as u8 - 1) as usize].position);
        }
        return result;
    }
    // TODO Handle move for black pawn.
    return result;
}