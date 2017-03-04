/// <reference path="../typings/p5.d.ts" />

let game: MoriaGame;

function setup() {
    game = new MoriaGame(8, 10);
    createCanvas(game.width, game.height);
    frameRate(10);
}

function draw() {
    background(0);
    game.draw();
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

/**
 * MoriaGame
 */
class MoriaGame {
    public readonly nRows: number;
    public readonly nCols: number;
    public readonly width: number;
    public readonly height: number;

    private maze: Maze;
    private hero: Hero;

    constructor(nRows: number, nCols: number) {
        this.nRows = nRows;
        this.nCols = nCols;

        let mazeGenerator = new MazeGenerator();
        this.maze = mazeGenerator.newMaze(this.nRows, this.nCols);

        this.width = this.maze.width;
        this.height = this.maze.height;

        this.hero = new Hero(0, 0);
        this.maze.cell(this.hero.y, this.hero.x).visited = true;

        this.maze.setUpstair(this.hero.y, this.hero.x);
        this.maze.setDownstair(this.nRows - 1, this.nCols - 1);
    }

    public draw() {
        background(0);
        this.maze.draw();
        this.hero.draw();
    }

    public moveHero(direction: Direction) {
        let cell = this.maze.cell(this.hero.y, this.hero.x);
        let cellBorders = this.maze.cell(this.hero.y, this.hero.x).borders;
        if (direction === Direction.RIGHT && !cellBorders.right) {
            this.move(1, 0);
        } else if (direction === Direction.LEFT && !cellBorders.left) {
            this.move(-1, 0);
        } else if (direction === Direction.UP && !cellBorders.top) {
            this.move(0, -1);
        } else if (direction === Direction.DOWN && !cellBorders.bottom) {
            this.move(0, 1);
        }
    }

    private move(x: number, y: number) {
        this.hero.x += x;
        this.hero.y += y;
        this.maze.cell(this.hero.y, this.hero.x).visited = true;
    }
}

/**
 * Hero
 */
class Hero {
    public x: number;
    public y: number;

    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }

    public draw() {
        stroke(255);
        fill(0, 255, 0);
        let x = this.x * Cell.cellWidth + Cell.cellWidth / 2;
        let y = this.y * Cell.cellWidth + Cell.cellWidth / 2;
        let r = Cell.cellWidth / 2 - 1;
        ellipse(x, y, r, r);
    }
}
