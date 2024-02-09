use crate::piece::*;
use crate::board::Move;

impl Piece {
    pub fn is_move_valid(&mut self, pos: Position, pieces: &Vec<Piece>, game_history: &Vec<Move>) -> (bool, i8) {
        let mut king_safety: Vec<Position> = Vec::new();

        let moves: Vec<Position> = self.get_moves(pieces, game_history);
        let mut en_passant: i8 = 0;

        if game_history.len() > 1 {
            let last_move: &Move = &game_history[game_history.len() - 1];

            match self.color {
                PieceColor::White => {
                    if last_move.piece.kind == Kind::Pawn && self.kind == Kind::Pawn && pos.1 - 1 == last_move.to.1 && pos.0 == last_move.to.0 {
                        en_passant = -1;
                    } 
                },
                PieceColor::Black => {
                    if last_move.piece.kind == Kind::Pawn && self.kind == Kind::Pawn && pos.1 + 1 == last_move.to.1 && pos.0 == last_move.to.0 {
                        en_passant = 1;
                    } 
                }
            }
        }

        (moves.contains(&pos), en_passant)
    }

    pub fn get_moves(&self, pieces: &Vec<Piece>, moves: &Vec<Move>) -> Vec<Position> {
        let mut possible_moves: Vec<Position> = Vec::new();

        match self.kind {
            Kind::Queen => {
                possible_moves.extend(self.check_horizontal(pieces));
                possible_moves.extend(self.check_vertical(pieces));
                possible_moves.extend(self.check_diagonal_right(pieces));
                possible_moves.extend(self.check_diagonal_left(pieces));
            },
            Kind::King => {
                possible_moves.extend(self.check_king_moves(moves, pieces));
            },
            Kind::Rook => {
                possible_moves.extend(self.check_horizontal(pieces));
                possible_moves.extend(self.check_vertical(pieces));
            },
            Kind::Bishop => {
                possible_moves.extend(self.check_diagonal_right(pieces));
                possible_moves.extend(self.check_diagonal_left(pieces));
            },
            Kind::Knight => {
                possible_moves.extend(self.check_horse_moves(pieces));
            },
            Kind::Pawn => {
                possible_moves.extend(self.check_pawn_moves(pieces, moves));
            }
        }

        possible_moves
    }

    fn is_king_safe(&self, pieces: &Vec<Piece>, moves: &Vec<Move>) ->  bool {
        let king: Piece = match self.kind {
            Kind::King => self.clone(),
            _ => Piece::get_king(pieces, self.color),
        };

        let mut possible_enemy_moves: Vec<Vec<Position>> = Vec::new();
        let mut possible_team_moves: Vec<Vec<Position>> = Vec::new();

        for i in 0..pieces.len() {
            if pieces[i].color != self.color {
                possible_enemy_moves.push(pieces[i].get_moves(pieces, moves));
            } else {
                possible_team_moves.push(pieces[i].get_moves(pieces, moves));
            }
        }

        // Is king safe method approach
        // 
        // 1. Check if king is in check
        // 2. if king is in check, check if the current piece can block the check
        // 3. if No piece can block the check send check mate
        //

        true
    }

    fn get_king(pieces: &Vec<Piece>, searching: PieceColor) -> Piece {
        for i in 0..pieces.len() {
            if pieces[i].kind == Kind::King && pieces[i].color == searching {
                return pieces[i].clone();
            }
        }

        panic!("no king found!");
    }

    fn check_king_moves(&self, moves: &Vec<Move>, pieces: &Vec<Piece>) -> Vec<Position> {
        let mut possible_moves: Vec<Position> = Vec::new();

        for i in -1..2 {
            for j in -1..2 {
                if self.pos.0 as i8 + i > -1 
                && self.pos.1 as i8 + j > -1
                && self.pos.0 as i8 + i < 8
                && self.pos.1 as i8 + j < 8 
                {
                    let mut encounter: (bool, PieceColor) = (false, PieceColor::White);

                    for piece in pieces {
                        if self.pos.0 as i8 + i == piece.pos.0 as i8 && self.pos.1 as i8 + j == piece.pos.1 as i8 {
                            encounter = (true, piece.color);
                        }
                    }

                    if !encounter.0 {
                        possible_moves.push(((self.pos.0 as i8 + i) as u8, (self.pos.1 as i8 + j) as u8))
                    } else {
                        if encounter.1 != self.color {
                            possible_moves.push(((self.pos.0 as i8 + i) as u8, (self.pos.1 as i8 + j) as u8))
                        }
                    }
                }
            }
        }

        for i in 0..possible_moves.len() {
            if possible_moves[i] == self.pos {
                possible_moves.remove(i);
                break;
            }
        }

        let mut possible_enemy_moves: Vec<Vec<Position>> = Vec::new();
        let mut index: Vec<usize> = vec![];
        let mut offset: usize = 0;

        for i in 0..pieces.len() {
            if pieces[i].color != self.color && pieces[i].kind != Kind::King {
                possible_enemy_moves.push(pieces[i].get_moves(pieces, moves));
            }
        }

        for i in 0..possible_moves.len() {
            for j in 0..possible_enemy_moves.len() {
                if possible_enemy_moves[j].contains(&possible_moves[i]) {
                    index.push(i);
                }
            }   
        }

        for i in 0..index.len() {
            possible_moves.remove(index[i - offset]);
            if offset <= i {
                offset += 1;
            }
        }

        possible_moves
    }

    fn check_pawn_moves(&self, pieces: &Vec<Piece>, moves: &Vec<Move>) -> Vec<Position> {
        let mut possible_position: Vec<Position> = Vec::new();

        match self.color {
            PieceColor::White => {
                let mut encounter: bool = false;

                if self.pos.1 == 1 {
                    for i in 1..=2 {
                        for piece in pieces {
                            if self.pos.0 == piece.pos.0 && self.pos.1 + i == piece.pos.1 {
                                encounter = true;
                            }
                        }
                    }

                    if !encounter {
                        possible_position.push((self.pos.0, self.pos.1 + 2));
                    }
                }

                if moves.len() > 0 {
                    let last_move: &Move = &moves[moves.len() - 1];

                    if self.pos.1 == 4 && last_move.piece.kind == Kind::Pawn && last_move.to.1 == 4 && last_move.from.1 == 6 && self.color != last_move.piece.color {
                        if self.pos.0 as i8 - 1 > -1 && self.pos.0 - 1 == last_move.to.0 {
                            possible_position.push((self.pos.0 - 1, self.pos.1 + 1));
                        }
    
                        if self.pos.0 + 1 == last_move.to.0 {
                            possible_position.push((self.pos.0 + 1, self.pos.1 + 1));
                        }
                    }
                }

                if self.pos.1 + 1 < 8 {
                    encounter = false;

                    for piece in pieces {
                        if self.pos.0 as i8 > 0 && piece.pos.0 == self.pos.0 - 1 && piece.pos.1 == self.pos.1 + 1 && self.color != piece.color {
                            possible_position.push((self.pos.0 - 1, self.pos.1 + 1))
                        }

                        if self.pos.0 + 1 < 8 && piece.pos.0 == self.pos.0 + 1 && piece.pos.1 == self.pos.1 + 1 && self.color != piece.color {
                            possible_position.push((self.pos.0 + 1, self.pos.1 + 1))
                        }

                        if self.pos.0 == piece.pos.0 && self.pos.1 + 1 == piece.pos.1 {
                            encounter = true;
                        }
                    }

                    if !encounter {
                        possible_position.push((self.pos.0, self.pos.1 + 1));
                    }
                }
            },
            PieceColor::Black => {
                let mut encounter: bool = false;

                if self.pos.1 == 6 {
                    for i in 1..=2 {
                        for piece in pieces {
                            if self.pos.0 == piece.pos.0 && self.pos.1 - i == piece.pos.1 {
                                encounter = true;
                            }
                        }
                    }

                    if !encounter {
                        possible_position.push((self.pos.0, self.pos.1 - 2));
                    }
                }

                if moves.len() > 0 {
                    let last_move: &Move = &moves[moves.len() - 1];

                    if self.pos.1 == 3 && last_move.piece.kind == Kind::Pawn && last_move.to.1 == 3 && last_move.from.1 == 1 && self.color != last_move.piece.color {
                        if self.pos.0 as i8 - 1 > -1 && self.pos.0 - 1 == last_move.to.0 {
                            possible_position.push((self.pos.0 - 1, self.pos.1 - 1));
                        }
    
                        if self.pos.0 + 1 == last_move.to.0 {
                            possible_position.push((self.pos.0 + 1, self.pos.1 - 1));
                        }
                    }
                }

                if self.pos.1 as i8 - 1 > -1 {
                    encounter = false;

                    for piece in pieces {
                        if self.pos.0 as i8 > -1 && piece.pos.0 == self.pos.0 - 1 && piece.pos.1 == self.pos.1 - 1 && self.color != piece.color {
                            possible_position.push((self.pos.0 - 1, self.pos.1 - 1))
                        }

                        if self.pos.0 + 1 < 8 && piece.pos.0 == self.pos.0 + 1 && piece.pos.1 == self.pos.1 - 1 && self.color != piece.color {
                            possible_position.push((self.pos.0 + 1, self.pos.1 - 1))
                        }

                        if self.pos.0 == piece.pos.0 && self.pos.1 - 1 == piece.pos.1 {
                            encounter = true;
                        }
                    }

                    if !encounter {
                        possible_position.push((self.pos.0, self.pos.1 - 1));
                    }
                }
            }
        }

        possible_position
    }

    fn check_horse_moves(&self, pieces: &Vec<Piece>) -> Vec<Position> {
        let mut possible_positions: Vec<Position> = Vec::new();

        let moves_to_check: Vec<(i8, i8)> = vec![
            (self.pos.0 as i8 - 2, self.pos.1 as i8 + 1),
            (self.pos.0 as i8 - 2, self.pos.1 as i8 - 1),
            (self.pos.0 as i8 + 2, self.pos.1 as i8 + 1),
            (self.pos.0 as i8 + 2, self.pos.1 as i8 - 1),
            (self.pos.0 as i8 - 1, self.pos.1 as i8 + 2),
            (self.pos.0 as i8 - 1, self.pos.1 as i8 - 2),
            (self.pos.0 as i8 + 1, self.pos.1 as i8 + 2),
            (self.pos.0 as i8 + 1, self.pos.1 as i8 - 2),
        ];

        for piece_move in moves_to_check {
            if piece_move.0 > -1 && piece_move.0 < 8 
            && piece_move.1 > -1 && piece_move.1 < 8 {
                let mut encounter: (bool, PieceColor) = (false, PieceColor::White);

                for piece in pieces {
                    if piece.pos.0 == piece_move.0 as u8 && piece.pos.1 == piece_move.1 as u8 {
                        encounter = (true, piece.color);
                        break;
                    }
                }

                if encounter.0 {
                    if self.color != encounter.1 {
                        possible_positions.push((piece_move.0 as u8, piece_move. 1 as u8));
                    }
                } else {
                    possible_positions.push((piece_move.0 as u8, piece_move. 1 as u8));
                }
            }
        }

        possible_positions
    }

    fn check_diagonal_left(&self, pieces: &Vec<Piece>) -> Vec<(u8, u8)> {
        let mut possible_positions: Vec<(u8, u8)> = Vec::new();

        let mut j: u8 = 0;

        'finished_diagonal_left_up: for i in 1..=self.pos.0 {
            let mut encounter: (bool, PieceColor) = (false, PieceColor::White);
            j += 1;

            for piece in pieces {
                if self.pos.0 - i == piece.pos.0 && self.pos.1 + j == piece.pos.1 {
                    encounter = (true, piece.color);
                    break;
                }
            }

            if encounter.0 {
                if encounter.1 != self.color {
                    possible_positions.push((self.pos.0 - i, self.pos.1 + j));
                }

                break 'finished_diagonal_left_up;
            } else {
                possible_positions.push((self.pos.0 - i, self.pos.1 + j));
            }
        }

        j = 0;
        
        'finished_diagonal_left_down: for i in 1..=self.pos.0 {
            let mut encounter: (bool, PieceColor) = (false, PieceColor::White);
            j += 1;

            for piece in pieces {
                if self.pos.0 - i == piece.pos.0 && self.pos.1 - j == piece.pos.1 {
                    encounter = (true, piece.color);
                    break;
                }
            }

            if encounter.0 {
                if encounter.1 != self.color {
                    possible_positions.push((self.pos.0 - i, self.pos.1 - j));
                }

                break 'finished_diagonal_left_down;
            } else {
                possible_positions.push((self.pos.0 - i, self.pos.1 - j));
            }
        }

        possible_positions
    }

    fn check_diagonal_right(&self, pieces: &Vec<Piece>) -> Vec<Position> {
        let mut possible_positions: Vec<Position> = Vec::new();
        let mut j: u8 = 0;

        'finished_diagonal_right_up: for i in self.pos.0+1..8 {
            let mut encounter: (bool, PieceColor) = (false, PieceColor::White);
            j += 1;

            for piece in pieces {
                if i == piece.pos.0 && self.pos.1 + j == piece.pos.1 {
                    encounter = (true, piece.color);
                }
            }

            if encounter.0 {
                if encounter.1 != self.color {
                    possible_positions.push((i, self.pos.1 + j));
                }

                break 'finished_diagonal_right_up;
            } else {
                possible_positions.push((i, self.pos.1 + j));
            }
        }

        'finished_diagonal_right_down: for i in 1..=self.pos.1 {
            let mut encounter: (bool, PieceColor) = (false, PieceColor::White);

            for piece in pieces {
                if self.pos.0 + i == piece.pos.0 && self.pos.1 - i == piece.pos.1 {
                    encounter = (true, piece.color);
                }
            }

            if encounter.0 {
                if encounter.1 != self.color {
                    possible_positions.push((self.pos.0 + i, self.pos.1 - i));
                }

                break 'finished_diagonal_right_down;
            } else {
                possible_positions.push((self.pos.0 + i, self.pos.1 - i));
            }
        }

        possible_positions
    }

    fn check_vertical(&self, pieces: &Vec<Piece>) -> Vec<Position> {
        let mut possible_positions: Vec<Position> = Vec::new();

        'finished_up: for i in self.pos.1+1..8 {
            let mut encounter: (bool, PieceColor) = (false, PieceColor::Black);

            for piece in pieces {
                if i == piece.pos.1 && self.pos.0 == piece.pos.0 {
                    encounter = (true, piece.color);
                    break;
                }
            }

            if encounter.0 {
                if self.color != encounter.1 {
                    possible_positions.push((self.pos.0, i));
                }

                break 'finished_up;
            } else {
                possible_positions.push((self.pos.0, i));
            }
        }

        'finished_down: for i in 1..=self.pos.1 {
            let mut encounter: (bool, PieceColor) = (false, PieceColor::Black);

            for piece in pieces {
                if (self.pos.1 - i) == piece.pos.1 && self.pos.0 == piece.pos.0 {
                    encounter = (true, piece.color);
                    break;
                }
            }

            if encounter.0 {
                if self.color != encounter.1 {
                    possible_positions.push((self.pos.0, self.pos.1 - i));
                }

                break 'finished_down;
            } else {
                possible_positions.push((self.pos.0, self.pos.1 - i));
            }
        }

        possible_positions
    }

    fn check_horizontal(&self, pieces: &Vec<Piece>) -> Vec<Position> {
        let mut possible_positions: Vec<Position> = Vec::new();

        'finished_right: for i in self.pos.0+1..8 {
            let mut encounter: (bool, PieceColor) = (false, PieceColor::Black);

            for piece in pieces {
                if piece.pos.0 == i && piece.pos.1 == self.pos.1 {
                    encounter = (true, piece.color);
                }
            }

            if encounter.0 {
                if encounter.1 != self.color {
                    possible_positions.push((i, self.pos.1));
                }

                break 'finished_right;
            } else {
                possible_positions.push((i, self.pos.1));
            }
        }

        'finished_left: for i in 1..=self.pos.0 {
            let mut encounter: (bool, PieceColor) = (false, PieceColor::Black);

            for piece in pieces {
                if piece.pos.0 == (self.pos.0 - i) && piece.pos.1 == self.pos.1 {
                    encounter = (true, piece.color);
                }
            }

            if encounter.0 {
                if encounter.1 != self.color {
                    possible_positions.push((self.pos.0 - i, self.pos.1));
                }

                break 'finished_left;
            } else {
                possible_positions.push((self.pos.0 - i, self.pos.1));
            }
        }

        possible_positions
    }
}
