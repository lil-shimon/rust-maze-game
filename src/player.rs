struct Player {
    width: u8,
    height: u8,
}

impl Player {
    fn init(width: u8, height: u8) -> Self {
        Self { width, height }
    }
}