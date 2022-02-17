/*
 * 迷路の種類
 */
enum MazeKind {
    Path,
    Wall,
    Start,
    End
}

/*
 * 迷路の状態（見える見えない）
 */
enum MazeFlag {
    Visited,
    Hide
}

/*
 * 方向
 */
enum MazeDirection {
    Up,
    Down,
    Left,
    Right
}

const MAZE_WIDTH: u8 = 5;
const MAZE_HEIGHT: u8 = 5;