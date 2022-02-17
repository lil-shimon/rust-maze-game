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
