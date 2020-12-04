/**
 * MoriaGame
 */
class MoriaGame {
    public readonly nRows: number;
    public readonly nCols: number;
    public readonly width: number;
    public readonly height: number;

    private hero: Hero;
    private mazes: Maze[];
    private currentLevel: number;

    constructor(nRows: number, nCols: number, nLevels: number) {
        this.nRows = nRows;
        this.nCols = nCols;

        let mazeGenerator = new MazeGenerator();
        this.mazes = [];
        for (let i = 0; i < nLevels; i++) {
            this.mazes.push(mazeGenerator.newMaze(this.nRows, this.nCols));
        }

        this.currentLevel = 0;

        let maze = this.maze();
        this.hero = new Hero(maze.upstair.col, maze.upstair.row);

        this.width = maze.width;
        this.height = maze.height;

        this.initLevel();
    }

    public getLevel(): number {
        return this.currentLevel;
    }

    private initLevel() {
        let maze = this.maze();
        this.hero.moveTo(maze.upstair.col, maze.upstair.row);
        maze.cell(this.hero.y, this.hero.x).visited = true;
        this.checkVisibility();
    }

    public maze(): Maze {
        return this.mazes[this.currentLevel];
    }

    public moveHero(direction: Direction) {
        if (this.canMove(direction)) {
            this.hero.move(direction);
            this.maze().cell(this.hero.y, this.hero.x).visited = true;
            if (this.hero.x === this.maze().downstair.col && this.hero.y === this.maze().downstair.row) {
                this.currentLevel++;
                this.initLevel();
            }
            this.checkVisibility();
        }
    }

    public canMove(direction: Direction): boolean {
        let cellBorders = this.maze().cell(this.hero.y, this.hero.x).borders;
        return (direction === Direction.RIGHT && !cellBorders.right)
            || (direction === Direction.LEFT && !cellBorders.left)
            || (direction === Direction.UP && !cellBorders.top)
            || (direction === Direction.DOWN && !cellBorders.bottom);
    }

    private checkVisibility() {
        let x: number;
        let y: number;
        let cell: Cell;
        let maze = this.maze();
        let reset = () => {
            x = this.hero.x;
            y = this.hero.y;
            cell = maze.cell(y, x);
        }
        let next = () => {
            cell = maze.cell(y, x);
            cell.visited = true;
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

    public getHero() {
        return this.hero;
    }
}