use crate::tetris::Rotation;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Shape {
    Straight,
    Square,
    T,
    L,
    LRev,
    S,
    Z
}

impl Shape {
    pub fn get_draw_grid(&self, rot: Rotation) -> [[u8; 4]; 4] {
        match self {
            Shape::Straight => {
                if rot.is_horizontal() {
                    [
                        [1, 1, 1, 1],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0]
                    ]
                } else {
                    [
                        [1, 0, 0, 0],
                        [1, 0, 0, 0],
                        [1, 0, 0, 0],
                        [1, 0, 0, 0]
                    ]
                }
            },
            Shape::Square => {
                [
                    [1, 1, 0, 0],
                    [1, 1, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0]
                ]
            },
            Shape::T => {
                match rot {
                    Rotation::North => {
                        [
                            [1, 0, 0, 0],
                            [1, 1, 0, 0],
                            [1, 0, 0, 0],
                            [0, 0, 0, 0]
                        ]
                    },
                    Rotation::East => {
                        [
                            [0, 1, 0, 0],
                            [1, 1, 1, 0],
                            [0, 0, 0, 0],
                            [0, 0, 0, 0]
                        ]
                    },
                    Rotation::South => {
                        [
                            [0, 1, 0, 0],
                            [1, 1, 0, 0],
                            [0, 1, 0, 0],
                            [0, 0, 0, 0]
                        ]
                    },
                    Rotation::West => {
                        [
                            [1, 1, 1, 0],
                            [0, 1, 0, 0],
                            [0, 0, 0, 0],
                            [0, 0, 0, 0]
                        ]
                    },
                }
            },
            Shape::L => {
                match rot {
                    Rotation::North => {
                        [
                            [1, 0, 0, 0],
                            [1, 0, 0, 0],
                            [1, 1, 0, 0],
                            [0, 0, 0, 0]
                        ]
                    },
                    Rotation::East => {
                        [
                            [1, 1, 1, 0],
                            [1, 0, 0, 0],
                            [0, 0, 0, 0],
                            [0, 0, 0, 0]
                        ]
                    },
                    Rotation::South => {
                        [
                            [1, 1, 0, 0],
                            [0, 1, 0, 0],
                            [0, 1, 0, 0],
                            [0, 0, 0, 0]
                        ]
                    },
                    Rotation::West => {
                        [
                            [0, 0, 1, 0],
                            [1, 1, 1, 0],
                            [0, 0, 0, 0],
                            [0, 0, 0, 0]
                        ]
                    },
                }
            },
            Shape::S => {
                if rot.is_horizontal() {
                    [
                        [0, 1, 1, 0],
                        [1, 1, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0]
                    ]
                } else {
                    [
                        [1, 0, 0, 0],
                        [1, 1, 0, 0],
                        [0, 1, 0, 0],
                        [0, 0, 0, 0]
                    ]
                }
            },
            Shape::Z => {
                if rot.is_horizontal() {
                    [
                        [1, 1, 0, 0],
                        [0, 1, 1, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0]
                    ]
                } else {
                    [
                        [0, 1, 0, 0],
                        [1, 1, 0, 0],
                        [1, 0, 0, 0],
                        [0, 0, 0, 0]
                    ]
                }
            },
            Shape::LRev => {
                match rot {
                    Rotation::North => {
                        [
                            [1, 1, 0, 0],
                            [1, 0, 0, 0],
                            [1, 0, 0, 0],
                            [0, 0, 0, 0]
                        ]
                    },
                    Rotation::East => {
                        [
                            [1, 0, 0, 0],
                            [1, 1, 1, 0],
                            [0, 0, 0, 0],
                            [0, 0, 0, 0]
                        ]
                    },
                    Rotation::South => {
                        [
                            [0, 1, 0, 0],
                            [0, 1, 0, 0],
                            [1, 1, 0, 0],
                            [0, 0, 0, 0]
                        ]
                    },
                    Rotation::West => {
                        [
                            [1, 1, 1, 0],
                            [0, 0, 1, 0],
                            [0, 0, 0, 0],
                            [0, 0, 0, 0]
                        ]
                    },
                }
            },
        }
    }
}