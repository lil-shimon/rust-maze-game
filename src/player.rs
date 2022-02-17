use crate::maze::{Maze, MazeKind};

struct Player {
    width: u8,
    height: u8,
}

impl Player {
    fn init(width: u8, height: u8) -> Self {
        Self { width, height }
    }

    fn init_player(maze_width: u8, maze_height: u8, maze: Maze) -> Self {
        let mut player = Player { width: 0, height: 0 };
        for i in 0..maze_width {
            for j in 0..maze_height {
                if maze[i][j].kind == MazeKind::Start {
                    player = Player::init(i, j);
                }
            }
        }
        player
    }
}

