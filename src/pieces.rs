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
    let col = piece.position.position_label.col_label as usize;
    let row = piece.position.position_label.row_label as usize;
    if piece.team == Team::WHITE {
        if row == 2 {
            let row_label = row + 1;
            if board[row_label][col].team == Team::NONE {
                result.push(board[row_label][col].position);
            }
        }

        if board[row][col].team == Team::NONE {
            result.push(board[row][col].position);
        }

        attack_moves_for_pawn(board, row, col, Team::BLACK, &mut result);
        return result;
    }

    // Get moves for black pawns
    if row == 7 {
        let row_label = row - 3;
        if board[row_label][col].team == Team::NONE {
            result.push(board[row_label][col].position);
        }
    }

    let row_label = row - 2;
    if board[row_label][col].team == Team::NONE {
        result.push(board[row_label][col].position);
    }

    attack_moves_for_pawn(board, row_label, col, Team::WHITE, &mut result);
    return result;
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
