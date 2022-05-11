/*
    delete this shit now
*/

use bevy::prelude::*;

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_pieces)
            .add_system(detection_system);
    }
}

fn spawn_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // Spawning white pawns
    for i in 0..8 {
        commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("white/pawn.png"),
            transform: Transform {
                translation: Vec3::new(super::OFFSET + i as f32* super::SQUARE_SIZE, super::OFFSET + super::SQUARE_SIZE, 0.0),
                scale: Vec3::new(0.4, 0.4, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Piece::white(Kind::Pawn, (i, 1), EnPassantStates::Waiting));
    }

    // spawn white king
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("white/king.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 4.0 * super::SQUARE_SIZE, super::OFFSET, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::white(Kind::King, (4, 0), EnPassantStates::Done));
    
    // spawn white queen
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("white/queen.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 3.0 * super::SQUARE_SIZE, super::OFFSET, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::white(Kind::Queen, (3, 0), EnPassantStates::Done));
        
    // spawn white bishop left
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("white/bishop.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 2.0 * super::SQUARE_SIZE, super::OFFSET, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::white(Kind::Bishop, (2, 0), EnPassantStates::Done));
     
    // spawn white bishop right
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("white/bishop.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 5.0 * super::SQUARE_SIZE, super::OFFSET, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::white(Kind::Bishop, (5, 0), EnPassantStates::Done));
 
    // spawn white knight left
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("white/knight.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 1.0 * super::SQUARE_SIZE, super::OFFSET, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::white(Kind::Knight, (1, 0), EnPassantStates::Done));
 
    // spawn white knight right
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("white/knight.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 6.0 * super::SQUARE_SIZE, super::OFFSET, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::white(Kind::Knight, (6, 0), EnPassantStates::Done));

    // spawn white rook left
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("white/rook.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET, super::OFFSET, 0.0),
            scale: Vec3::new(0.4, 0.4, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::white(Kind::Rook, (0, 0), EnPassantStates::Done));

    // spawn white rook right
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("white/rook.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 7.0 * super::SQUARE_SIZE, super::OFFSET, 0.0),
            scale: Vec3::new(0.4, 0.4, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::white(Kind::Rook, (7, 0), EnPassantStates::Done));
 
 
 
    // Spawning black pawns
    for i in 0..8 {
        commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("black/pawn.png"),
            transform: Transform {
                translation: Vec3::new(super::OFFSET + i as f32 * super::SQUARE_SIZE, super::OFFSET + super::SQUARE_SIZE * 6.0, 0.0),
                scale: Vec3::new(0.4, 0.4, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Piece::black(Kind::Pawn, (i, 6), EnPassantStates::Waiting));
    }

    // spawn black king
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("black/king.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 4.0 * super::SQUARE_SIZE, super::OFFSET + 7.0 * super::SQUARE_SIZE, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::black(Kind::King, (4, 7), EnPassantStates::Done));

    // spawn black queen
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("black/queen.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 3.0 * super::SQUARE_SIZE, super::OFFSET + 7.0 * super::SQUARE_SIZE, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::black(Kind::Queen, (3, 7), EnPassantStates::Done));
        
    // spawn black bishop left
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("black/bishop.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 2.0 * super::SQUARE_SIZE, super::OFFSET + 7.0 * super::SQUARE_SIZE, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::black(Kind::Bishop, (2, 7), EnPassantStates::Done));
    
    // spawn black bishop right
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("black/bishop.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 5.0 * super::SQUARE_SIZE, super::OFFSET + 7.0 * super::SQUARE_SIZE, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::black(Kind::Bishop, (5, 7), EnPassantStates::Done));

    // spawn black knight left
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("black/knight.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 1.0 * super::SQUARE_SIZE, super::OFFSET + 7.0 * super::SQUARE_SIZE, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::black(Kind::Knight, (1, 7), EnPassantStates::Done));

    // spawn black knight right
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("black/knight.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 6.0 * super::SQUARE_SIZE, super::OFFSET + 7.0 * super::SQUARE_SIZE, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::black(Kind::Knight, (6, 7), EnPassantStates::Done));

    // spawn black rook left
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("black/rook.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET, super::OFFSET + 7.0 * super::SQUARE_SIZE, 0.0),
            scale: Vec3::new(0.4, 0.4, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::black(Kind::Rook, (0, 7), EnPassantStates::Done));

    // spawn black rook right
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("black/rook.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 7.0 * super::SQUARE_SIZE, super::OFFSET + 7.0 * super::SQUARE_SIZE, 0.0),
            scale: Vec3::new(0.4, 0.4, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::black(Kind::Rook, (7, 7), EnPassantStates::Done));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Queen,
    King,
    Rook,
    Bishop,
    Knight,
    Pawn,
    NoPiece,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    Black,
    White,
}

#[derive(Component, Debug, Clone)]
pub struct Piece {
    pub kind: Kind,
    pub color: PieceColor,
    pub position: (i32, i32),
    pub moves: Vec<(i32, i32)>,
    pub en_passant: EnPassantStates,
}

#[derive(Component)]
pub struct Point;

#[derive(Component, Debug, Clone)]
pub enum EnPassantStates {
    Ready,
    Waiting,
    Done,
}

pub fn in_check(pieces: Vec<Piece>, has_color: PieceColor) -> bool {
    let mut king_position = (-1, -1);

    for p in pieces.clone() {
        match p.color {
            has_color => {
                match p.kind {
                    Kind::King => {
                        king_position = p.position.clone();
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    for p in pieces.clone() {
        for possible_move in p.moves {
            if possible_move == king_position {
                return true;
            }
        }
    }

    false
}

impl Piece {
    fn white(kind: Kind, position: (i32, i32), en_passant_state: EnPassantStates) -> Piece {
        Piece {
            kind,
            color: PieceColor::White,
            position,
            moves: Vec::new(),
            en_passant: en_passant_state,
        }
    }
    
    fn black(kind: Kind, position: (i32, i32), en_passant_state: EnPassantStates) -> Piece {
        Piece {
            kind,
            color: PieceColor::Black,
            position,
            moves: Vec::new(),
            en_passant: en_passant_state,
        }
    }
    
    pub fn promotion(&mut self, to: Kind) {
        if self.kind == Kind::Pawn {
            // Issue: Add => piece despawn and respawning new piece
    
            self.kind = to;
        }
    }
}

fn detection_system(
    mouse_button_input: ResMut<Input<MouseButton>>,
    mut piece_query: Query<(&mut Transform, &mut Piece)>,
    windows: Res<Windows>,
    mut commands: Commands
) {
    let window = windows.get_primary().unwrap();
    if let Some(pos) = window.cursor_position() {
        let x: i32 = (pos.x / super::SQUARE_SIZE) as i32;
        let y: i32 = (pos.y / super::SQUARE_SIZE) as i32;

        let mut transform_x = super::OFFSET;
        let mut transform_y = super::OFFSET;

        let mut pieces_on_the_board: Vec<Piece> = Vec::new();

        if mouse_button_input.just_released(MouseButton::Left) {
            piece_query.for_each( | query_info | {
                pieces_on_the_board.push(query_info.1.clone());
            });

            piece_query.for_each_mut( | (mut transform_piece,mut piece) | {
                if piece.position.0 == x && piece.position.1 == y {
                    piece.calculate_pseudo_legal_moves(pieces_on_the_board.clone());

                    let postitions = piece.moves.clone();

                    if postitions.len() > 0 {
                        transform_x += postitions[0].0 as f32 * super::SQUARE_SIZE;
                        transform_y += postitions[0].1 as f32 * super::SQUARE_SIZE;

                        piece.position.0 = postitions[0].0;
                        piece.position.1 = postitions[0].1;
    
                        transform_piece.translation = Vec3::new(transform_x, transform_y, 1.0);
                        
                        info!("{:?}", postitions);
                    }
                }
            });
        }

        if mouse_button_input.just_pressed(MouseButton::Right) {
            piece_query.for_each( | query_info | {
                pieces_on_the_board.push(query_info.1.clone());
                
            });

            piece_query.for_each_mut( | mut transform_and_piece | {
                if transform_and_piece.1.position.0 == x && transform_and_piece.1.position.1 == y {
                    transform_and_piece.1.calculate_pseudo_legal_moves(pieces_on_the_board.clone());
                    let positions = transform_and_piece.1.moves.clone();

                    for position in positions {
                        let point_position = Vec2::new(super::SQUARE_SIZE * position.0 as f32 + super::OFFSET, super::SQUARE_SIZE * position.1 as f32 + super::OFFSET);

                        info!("{:?}", position);

                        commands
                            .spawn()
                            .insert(Point)
                            .insert_bundle( SpriteBundle {
                                sprite: Sprite {
                                    color: Color::rgb(255.0, 0.0, 0.0),
                                    ..default()
                                },
                                transform: Transform {
                                    translation: point_position.extend(0.0),
                                    scale: Vec3::new(20.0, 20.0, 1.0),
                                    ..default()
                                },
                                ..default()
                        });
                    }
                }
            });
        }
    }
}
