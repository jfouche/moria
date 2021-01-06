import { Hero } from "./hero"
import { Cell, Maze, MazeGenerator, Room } from "./maze"

export const enum Direction {
    UP, DOWN, LEFT, RIGHT
}


/**
 * MoriaGame
 */
export class MoriaGame {
    public readonly nRows: number;
    public readonly nCols: number;

    private hero: Hero;
    private mazes: Maze[];
    private currentLevel: number;

    constructor(nRows: number, nCols: number, nLevels: number) {
        this.nRows = nRows;
        this.nCols = nCols;

        // Create all levels
        const mazeGenerator = new MazeGenerator();
        this.mazes = [];
        for (let i = 0; i < nLevels; i++) {
            this.mazes.push(mazeGenerator.newMaze(this.nRows, this.nCols));
        }

        // Add a hero
        this.hero = new Hero();

        // initiate the game at level 0
        this.currentLevel = 0;
        const maze = this.maze();
        this.hero.goTo(maze.upstair);
        maze.visit(maze.upstair);
        this.checkVisibility();
    }

    /**
     * 
     */
    public getLevel(): number {
        return this.currentLevel;
    }

    /**
     * 
     */
    public maze(): Maze {
        return this.mazes[this.currentLevel];
    }

    /**
     * 
     * @param direction 
     */
    public moveHero(direction: Direction) {
        let newCell = this.getRoom(direction);
        if (newCell !== undefined) {
            this.hero.moveTo(newCell);
            this.maze().visit(newCell);
            this.checkVisibility();
        }
    }

    /**
     * 
     * @param direction 
     */
    private getRoom(direction: Direction): Cell {
        const cell = this.maze().cell(this.hero.y, this.hero.x);
        const borders = cell.borders;
        if (direction === Direction.RIGHT && !borders.right) {
            return new Cell(cell.row, cell.col + 1);
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
    private checkVisibility() {
        let x: number;
        let y: number;
        let cell: Room;
        const maze = this.maze();
        const reset = () => {
            x = this.hero.x;
            y = this.hero.y;
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

    public getHero() {
        return this.hero;
    }

    public doAction() {
        // check if hero in on a stair
        if (this.hero.isOn(this.maze().downstair)) {
            this.currentLevel++;
            const maze = this.maze();
            const upstair = maze.upstair;
            this.hero.moveTo(upstair);
            this.maze().visit(upstair);
            this.checkVisibility();
        } else if (this.hero.isOn(this.maze().upstair)) {
            this.currentLevel--;
            const maze = this.maze();
            const downstair = maze.downstair;
            this.hero.moveTo(downstair);
            this.maze().visit(downstair);
            this.checkVisibility();
        }
    }
}