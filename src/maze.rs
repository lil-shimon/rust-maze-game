use std::collections::BTreeMap;

/*
 * 迷路の種類
 */
#[derive(PartialEq, Debug)]
pub enum MazeKind {
    Path,
    Wall,
    Start,
    End,
}

/*
 * 迷路の状態（見える見えない）
 */
#[derive(PartialEq, Debug)]
pub enum MazeFlag {
    Visited,
    Hide,
}

/*
 * 方向
 */
enum MazeDirection {
    Up,
    Down,
    Left,
    Right,
}

pub type MazeKindAnnotation = BTreeMap<String, MazeKind>;

#[derive(PartialEq, Debug)]
pub struct MazeCell {
    pub(crate) kind: MazeKind,
    pub(crate) flag: MazeFlag,
}

#[derive(PartialEq, Debug)]
pub struct MazeRow {
    cells: Vec<MazeCell>,
}

#[derive(PartialEq, Debug)]
pub struct Maze {
    width: usize,
    height: usize,
    rows: Vec<MazeRow>,
}

impl MazeCell {
    fn create(kind: MazeKind, flag: MazeFlag) -> Self {
        Self {
            kind,
            flag,
        }
    }

    pub fn compare(a: &MazeKind, b: &MazeKind) -> bool {
        a == b
    }
}

impl MazeRow {
    fn create(cells: Vec<MazeCell>) -> Self {
        Self { cells }
    }
}

impl Maze {
    fn init(width: usize, height: usize, rows: Vec<MazeRow>) -> Self {
        Self {
            width,
            height,
            rows,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::maze::{Maze, MazeCell, MazeFlag, MazeKind, MazeRow};

    #[test]
    fn should_create_maze_cell() {
        let kind = MazeKind::Start;
        let flag = MazeFlag::Visited;
        assert_eq!(&MazeCell::create(kind, flag), &MazeCell {
            kind: MazeKind::Start,
            flag: MazeFlag::Visited,
        })
    }

    #[test]
    fn should_create_maze_row() {
        let mut cells: Vec<MazeCell> = vec![];
        for i in 0..5 {
            let cell = MazeCell::create(MazeKind::Path, MazeFlag::Hide);
            cells.push(cell);
        }
        assert_eq!(MazeRow::create(cells), MazeRow {
            cells: vec!(MazeCell { kind: MazeKind::Path, flag: MazeFlag::Hide },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Hide },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Hide },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Hide },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Hide })
        })
    }

    #[test]
    fn should_create_maze() {
        let width = 5;
        let height = 5;

        let mut rows = vec![];
        for i in 0..5
        {
            let mut cells: Vec<MazeCell> = vec![];
            for i in 0..5
            {
                let cell = MazeCell::create(MazeKind::Path, MazeFlag::Visited);
                cells.push(cell)
            }
            let row = MazeRow::create(cells);
            rows.push(row)
        }
        assert_eq!(Maze::init(width, height, rows), Maze {
            width: 5,
            height: 5,
            rows: vec!(
                MazeRow {
                    cells: vec!(
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                    )
                },
                MazeRow {
                    cells: vec!(
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                    )
                },
                MazeRow {
                    cells: vec!(
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                    )
                },
                MazeRow {
                    cells: vec!(
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                    )
                },
                MazeRow {
                    cells: vec!(
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                        MazeCell { kind: MazeKind::Path, flag: MazeFlag::Visited },
                    )
                },
            ),
        })
    }
}