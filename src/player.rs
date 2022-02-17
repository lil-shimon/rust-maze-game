use crate::maze::{Maze, MazeCell, MazeKind};
use crate::maze::MazeKind::Start;

#[derive(PartialEq, Debug)]
struct Player {
    width: usize,
    height: usize,
}

// impl Player {
//     fn init(width: usize, height: usize) -> Self {
//         Self { width, height }
//     }
//
//     fn init_player(maze_width: usize, maze_height: usize, maze: Vec<Vec<Maze>>) -> Self {
//         let mut player = Player { width: 0, height: 0 };
//         for i in 0..maze_width
//         {
//             for j in 0..maze_height
//             {
//                 if MazeCell::compare(&maze[i as usize][j as usize].kind, &Start)
//                 {
//                     // player = Player::init(i, j);
//                     break;
//                 }
//             }
//         }
//         player
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::maze::Maze;
//     use crate::maze::MazeFlag::{Hide, Visited};
//     use crate::maze::MazeKind::{Path, Start};
//     use crate::player::Player;
//
//     #[test]
//     fn should_create_player() {
//         let maze_width = 5;
//         let maze_height = 5;
//         let maze = vec![vec![Maze { kind: Start, flag: Visited }], vec![Maze { kind: Path, flag: Hide }], vec![Maze { kind: Path, flag: Hide }], vec![Maze { kind: Path, flag: Hide }], vec![Maze { kind: Path, flag: Hide }],
//                         vec![Maze { kind: Start, flag: Visited }], vec![Maze { kind: Path, flag: Hide }], vec![Maze { kind: Path, flag: Hide }], vec![Maze { kind: Path, flag: Hide }], vec![Maze { kind: Path, flag: Hide }],
//                         vec![Maze { kind: Path, flag: Visited }], vec![Maze { kind: Path, flag: Hide }], vec![Maze { kind: Path, flag: Hide }], vec![Maze { kind: Path, flag: Hide }], vec![Maze { kind: Path, flag: Hide }],
//                         vec![Maze { kind: Path, flag: Visited }], vec![Maze { kind: Path, flag: Hide }], vec![Maze { kind: Path, flag: Hide }], vec![Maze { kind: Path, flag: Hide }], vec![Maze { kind: Path, flag: Hide }],
//                         vec![Maze { kind: Path, flag: Visited }], vec![Maze { kind: Path, flag: Hide }], vec![Maze { kind: Path, flag: Hide }], vec![Maze { kind: Path, flag: Hide }], vec![Maze { kind: Path, flag: Hide }],
//         ];
//         let player = Player::init_player(maze_width, maze_height, maze);
//         let test_player = Player::init(0, 0);
//         assert_eq!(player, test_player)
//     }
// }
