use bevy::prelude::Entity;
use crate::{GameState, get_possible_moves_for_piece, Mut, Piece, Position, simulate_move, Tile};

pub fn prevent_check(selected_piece: &mut Piece, selected_entity: Entity, enemy_piece: &Piece, king_pos: Position, game_state: &GameState) {
    if enemy_piece.team == game_state.turn {
        return;
    }

    selected_piece.available_moves.retain(|position| {
        if position.position_label == enemy_piece.position.position_label {
            return true;
        }

        let mut board_copy: [[Tile; 8]; 8] = game_state.board;

        simulate_move(
            &mut board_copy,
            selected_entity,
            selected_piece.team,
            selected_piece.position.position_label,
            position.position_label
        );

        let available_enemy_moves: Vec<Position> = get_possible_moves_for_piece(&enemy_piece, &board_copy);
        return !available_enemy_moves.iter().any(|&pos| {
            return pos.position_label == king_pos.position_label;
        });
    });
}