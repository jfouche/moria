
/**
 * Hero
 */
struct Hero {
    pos: Position,
    maxLife: u32,
    life: u32
}

impl Hero {
    // pub fn x(): number {
    //     return self._x;
    // }

    // public get y(): number {
    //     return self._y;
    // }

    pub fn life() -> u32 {
        self.life;
    }

    pub fn go_to(room: &Room) {
        self._x = cell.col;
        self._y = cell.row;
    }

    pub fn move_to(cell: Cell) {
        self.go_to(cell);
        self.life -= 1;
    }

    pub fn isOn(cell: Cell) {
        return self._x == cell.col && self._y == cell.row;
    }

    pub fn change_life(&mut self, content: number) {
        self._life += content;
        if (self._life > self.maxLife) {self._life = self.maxLife};
        if (self._life < 0) {self._life = 0;}
    }
}
