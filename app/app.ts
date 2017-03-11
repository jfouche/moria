/// <reference path="../typings/p5.d.ts" />

let game: MoriaGame;

function setup() {
    game = new MoriaGame(8, 10, 5);
    let canvas = createCanvas(game.width, game.height);
    canvas.parent('game');
    frameRate(10);
}

function draw() {
    background(0);
    game.draw();
    updateInfo();
}

function updateInfo() {
    let levelElt = document.getElementById("nLevel");
    levelElt.innerHTML = game.getLevel().toString();
}

function keyPressed() {
    if (keyCode === UP_ARROW) {
        game.moveHero(Direction.UP);
    } else if (keyCode === DOWN_ARROW) {
        game.moveHero(Direction.DOWN);
    } else if (keyCode === LEFT_ARROW) {
        game.moveHero(Direction.LEFT);
    } else if (keyCode === RIGHT_ARROW) {
        game.moveHero(Direction.RIGHT);
    }
}

const enum Direction {
    UP, DOWN, LEFT, RIGHT
}

class Offset {
    x: number;
    y: number;
    constructor(x: number, y: number) {
        this.x = x;
        this.y = y
    }
};

function directionOffset(dir: Direction): Offset {
    switch (dir) {
        case Direction.UP:
            return new Offset(0, -1);
        case Direction.DOWN:
            return new Offset(0, 1);
        case Direction.LEFT:
            return new Offset(-1, 0);
        case Direction.RIGHT:
            return new Offset(1, 0);
        default:
            break;
    }
    return undefined;
}

/**
 * MoriaGame
 */
class MoriaGame {
    public readonly nRows: number;
    public readonly nCols: number;
    public readonly width: number;
    public readonly height: number;

    private mazes: Maze[];
    private hero: Hero;
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
        this.width = maze.width;
        this.height = maze.height;

        this.initLevel();
    }

    public getLevel(): number {
        return this.currentLevel;
    }

    private initLevel() {
        let maze = this.maze();
        this.hero = new Hero(maze.upstair.col, maze.upstair.row);
        maze.cell(this.hero.y, this.hero.x).visited = true;
        this.checkVisibility();
    }

    public maze(): Maze {
        return this.mazes[this.currentLevel];
    }

    public draw() {
        background(0);
        this.maze().draw();
        this.hero.draw();
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
}

/**
 * Hero
 */
class Hero {
    private _x: number;
    private _y: number;

    constructor(x: number, y: number) {
        this._x = x;
        this._y = y;
    }

    public get x(): number {
        return this._x;
    }

    public get y(): number {
        return this._y;
    }

    public draw() {
        stroke(255);
        fill(0, 255, 0);
        let x = this._x * Cell.cellWidth + Cell.cellWidth / 2;
        let y = this._y * Cell.cellWidth + Cell.cellWidth / 2;
        let r = Cell.cellWidth / 2 - 1;
        ellipse(x, y, r, r);
    }

    public moveTo(x: number, y: number) {
        this._x += x;
        this._y += y;
    }

    public move(dir: Direction) {
        let offset = directionOffset(dir);
        this._x += offset.x;
        this._y += offset.y;
    }
}
