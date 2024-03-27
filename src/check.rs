use std::collections::{HashSet};
use bevy::ecs::query::QueryEntityError;
use bevy::prelude::{Entity, Query, Without};
use crate::{GameState, get_possible_moves_for_piece, Piece, PieceType, Position, Selected, simulate_move, Tile};
use crate::pieces::Team;

pub fn prevent_check(
    selected_piece: &mut Piece,
    selected_entity: Entity,
    enemy_piece: &Piece,
    king_pos: Position,
    game_state: &GameState
) {
    if enemy_piece.team == game_state.turn {
        return;
    }

    selected_piece.available_moves.retain(|position| {
        let move_label = position.position_label;
        if move_label == enemy_piece.position.position_label {
            return true;
        }

        let mut board_copy: [[Tile; 8]; 8] = game_state.board;

        simulate_move(
            &mut board_copy,
            selected_entity,
            selected_piece.team,
            selected_piece.position.position_label,
            move_label
        );

        let available_enemy_moves: Vec<Position> = get_possible_moves_for_piece(enemy_piece, &board_copy);
        return !available_enemy_moves.iter().any(|&pos| {
            if selected_piece.piece_type == PieceType::KING {
                return pos.position_label == move_label;
            }
            pos.position_label == king_pos.position_label
        });
    });
}

pub fn check_checkmate(turn: Team, king_pos: Position, board: [[Tile; 8]; 8], query_unselected: Query<(Entity, &mut Piece), Without<Selected>>) -> bool {
    let mut friendly_entities: HashSet<Entity> = HashSet::new();
    let mut enemy_entities: HashSet<Entity> = HashSet::new();

    for (entity, piece) in query_unselected.iter() {
        if turn == piece.team {
            friendly_entities.insert(entity);
        } else {
            enemy_entities.insert(entity);
        }
    }

    for entity in friendly_entities {
        let queried_entity: Result<(Entity, &Piece), QueryEntityError> = query_unselected.get(entity);
        let piece = queried_entity.unwrap().1;


        let mut available_moves: Vec<Position> = get_possible_moves_for_piece(piece, &board);
        available_moves.retain(|position| {
            let move_label = position.position_label;
            if move_label == piece.position.position_label {
                return true;
            }

            let mut board_copy: [[Tile; 8]; 8] = board;

            simulate_move(
                &mut board_copy,
                entity,
                piece.team,
                piece.position.position_label,
                move_label
            );

            let mut retain_move = true;

            for enemy_entity in enemy_entities.iter().copied() {
                let queried_enemy: Result<(Entity, &Piece), QueryEntityError> = query_unselected.get(enemy_entity);
                let enemy_piece = queried_enemy.unwrap().1;

                let available_enemy_moves: Vec<Position> = get_possible_moves_for_piece(enemy_piece, &board_copy);

                retain_move = !available_enemy_moves.iter().any(|&pos| {
                    if piece.piece_type == PieceType::KING {
                        return pos.position_label == move_label;
                    }
                    pos.position_label == king_pos.position_label
                });

                if !retain_move {
                    break;
                }
            }

            retain_move
        });

        if !available_moves.is_empty() {
            return false;
        }
    }

    true
}
