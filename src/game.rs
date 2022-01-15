pub enum Direction {
    UP, DOWN, LEFT, RIGHT
}


/**
 * MoriaGame
 */
pub struct MoriaGame {
    // public readonly nRows: number;
    // public readonly nCols: number;

    // private hero: Hero;
    // private mazes: Maze[];
    currentLevel: u32
}

impl MoriaGame {
    pub fn new(nRows: u32, nCols: u32, nLevels: u32) {
        self.nRows = nRows;
        self.nCols = nCols;

        // Create all levels
        let mazeGenerator = MazeGenerator::new();
        self.mazes = [];
        for i in 0..nLevels {
            self.mazes.push(mazeGenerator.newMaze(self.nRows, self.nCols));
        }

        // Add a hero
        // this.hero = new Hero();

        // initiate the game at level 0
        // this.currentLevel = 0;
        // const maze = this.maze();
        // this.hero.goTo(maze.upstair);
        // maze.visit(maze.upstair);
        // this.checkVisibility();
    }

    /**
     * 
     */
    pub fn get_level(&self) -> u32 {
        self.currentLevel;
    }

    /**
     * 
     */
    pub fn maze(&self) -> &Maze {
        self.mazes[this.currentLevel];
    }

    /**
     * 
     * @param direction 
     */
    pub fn move_hero(&self, direction: Direction) {
        if let Some(newCell) = self.getRoom(direction) {
            self.hero.moveTo(newCell);
            self.maze().visit(newCell);
            self.checkVisibility();
        }
    }

    /**
     * 
     * @param direction 
     */
    fn get_room(direction: Direction) -> Option<Room> {
        let cell = self.maze().cell(self.hero.y, self.hero.x);
        let borders = cell.borders;
        if (direction === Direction.RIGHT && !borders.right) {
            return new Cell::new(cell.row, cell.col + 1);
        } else if (direction === Direction.LEFT && !borders.left) {
            return new Cell(cell.row, cell.col - 1);
        } if (direction === Direction.UP && !borders.top) {
            return new Cell(cell.row - 1, cell.col);
        } if (direction === Direction.DOWN && !borders.bottom) {
            return new Cell(cell.row + 1, cell.col);
        }
        return undefined;
    }

    /**
     * 
     */
    fn check_visibility() {
        let x: number;
        let y: number;
        let cell: Room;
        const maze = self.maze();
        const reset = () => {
            x = self.hero.x;
            y = self.hero.y;
            cell = maze.cell(y, x);
        }
        const next = () => {
            cell = maze.cell(y, x);
            cell.visit();
        }
        reset();
        while (!cell.borders.top) {
            y -= 1;
            next();
        }
        reset();
        while (!cell.borders.right) {
            x += 1;
            next();
        }
        reset();
        while (!cell.borders.bottom) {
            y += 1;
            next();
        }
        reset();
        while (!cell.borders.left) {
            x -= 1;
            next();
        }
    }

    pub fn get_hero() {
        self.hero;
    }

    pub fn do_action() {
        // check if hero in on a stair
        if (self.hero.isOn(self.maze().downstair)) {
            self.currentLevel++;
            const maze = self.maze();
            const upstair = maze.upstair;
            self.hero.moveTo(upstair);
            self.maze().visit(upstair);
            self.checkVisibility();
        } else if (self.hero.isOn(self.maze().upstair)) {
            self.currentLevel--;
            const maze = self.maze();
            const downstair = maze.downstair;
            self.hero.moveTo(downstair);
            self.maze().visit(downstair);
            self.checkVisibility();
        }
    }
}