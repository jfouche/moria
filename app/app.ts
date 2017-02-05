/// <reference path="../typings/p5.d.ts" />

let game: MoriaGame;

function setup() {
    game = new MoriaGame(20, 30);
    createCanvas(game.width, game.height);
    frameRate(10);
}

function draw() {
    background(0);
    game.draw();
}

function keyPressed() {
    if (keyCode === UP_ARROW) {
        game.moveHero(0, -1);
    } else if (keyCode === DOWN_ARROW) {
        game.moveHero(0, 1);
    } else if (keyCode === LEFT_ARROW) {
        game.moveHero(-1, 0);
    } else if (keyCode === RIGHT_ARROW) {
        game.moveHero(1, 0);
    }
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
    }

    public draw() {
        background(0);
        this.maze.draw();
        this.hero.draw();
    }

    public moveHero(x: number, y: number) {
        let cell = this.maze.cell(this.hero.y, this.hero.x);
        let cellBorders = cell.borders;
        if (x === 1 && !cellBorders.right) {
            this.hero.x++;
            this.maze.cell(this.hero.y, this.hero.x).visited = true;
        } else if (x === -1 && !cellBorders.left) {
            this.hero.x--;
            this.maze.cell(this.hero.y, this.hero.x).visited = true;
        } else if (y === -1 && !cellBorders.top) {
            this.hero.y--;
            this.maze.cell(this.hero.y, this.hero.x).visited = true;
        } else if (y === 1 && !cellBorders.bottom) {
            this.hero.y++;
            this.maze.cell(this.hero.y, this.hero.x).visited = true;
        }
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

/**
 * @class Maze
 */
class Maze {
    public readonly nRows: number;
    public readonly nCols: number;
    public readonly width: number;
    public readonly height: number;
    private grid: Cell[][];

    constructor(nRows: number, nCols: number) {
        this.nRows = nRows;
        this.nCols = nCols;
        this.height = nRows * Cell.cellWidth + 1;
        this.width = nCols * Cell.cellWidth + 1;
        this.grid = [];

        for (var r = 0; r < this.nRows; r++) {
            this.grid[r] = [];
            for (var c = 0; c < this.nCols; c++) {
                this.grid[r][c] = new Cell(r, c);
            }
        }
    }

    public cell(row: number, col: number) {
        return this.grid[row][col];
    }

    public draw() {
        for (let rows of this.grid) {
            for (let cell of rows) {
                if (cell.visited) {
                    cell.draw();
                }
            }
        }
    }
}


/**
 * @class MazeGenerator
 */
class MazeGenerator {

    public newMaze(nRows: number, nCols: number): Maze {
        let maze = new Maze(nRows, nCols);
        let backtracking: Cell[] = [];
        let currentCell = maze.cell(0, 0);
        currentCell.visited = true;
        let finished = false;
        while (!finished) {
            let next = this.getNextNeighbor(maze, currentCell);
            if (next) {
                next.visited = true;
                backtracking.push(currentCell);
                this.removeWalls(currentCell, next);
                currentCell = next;
            } else if (backtracking.length > 0) {
                next = backtracking.pop();
                currentCell = next;
            } else {
                console.log("FINISH");
                finished = true;
            }
        }

        for (var r = 0; r < nRows; r++) {
            for (var c = 0; c < nCols; c++) {
                maze.cell(r, c).visited = false;
            }
        }

        return maze;
    }

    private getNextNeighbor(maze: Maze, cell: Cell): Cell {
        let neighbors: Cell[] = [];
        if (cell.row > 0) {
            let left = maze.cell(cell.row - 1, cell.col);
            if (!left.visited) {
                neighbors.push(left);
            }
        }
        if (cell.row < maze.nRows - 1) {
            let right = maze.cell(cell.row + 1, cell.col);
            if (!right.visited) {
                neighbors.push(right);
            }
        }
        if (cell.col > 0) {
            let top = maze.cell(cell.row, cell.col - 1);
            if (!top.visited) {
                neighbors.push(top);
            }
        }
        if (cell.col < maze.nCols - 1) {
            let bottom = maze.cell(cell.row, cell.col + 1);
            if (!bottom.visited) {
                neighbors.push(bottom);
            }
        }

        let next: Cell = undefined;
        if (neighbors.length > 0) {
            var r = floor(random(0, neighbors.length));
            next = neighbors[r];
        }
        return next;
    }

    private removeWalls(a: Cell, b: Cell) {
        if (a.col > b.col) {
            a.borders.left = false;
            b.borders.right = false;
        } else if (a.col < b.col) {
            a.borders.right = false;
            b.borders.left = false;
        } else if (a.row > b.row) {
            a.borders.top = false;
            b.borders.bottom = false;
        } else if (a.row < b.row) {
            a.borders.bottom = false;
            b.borders.top = false;
        }
    }
}


/**
 * @class CellBorders
 */
class CellBorders {
    top: boolean = true;
    right: boolean = true;
    bottom: boolean = true;
    left: boolean = true;
}

/**
 * @class Cell
 */
class Cell {
    public static cellWidth: number = 30;

    public readonly row: number;
    public readonly col: number;

    public borders: CellBorders;
    public visited: boolean = false;

    constructor(row: number, col: number) {
        this.row = row;
        this.col = col;
        this.borders = new CellBorders();
    }

    public draw() {
        stroke(255);
        noFill();
        let w = Cell.cellWidth;
        let x = this.col * Cell.cellWidth;
        let y = this.row * Cell.cellWidth;
        if (this.borders.top) {
            line(x, y, x + w, y);
        }
        if (this.borders.right) {
            line(x + w, y, x + w, y + w);
        }
        if (this.borders.bottom) {
            line(x + w, y + w, x, y + w);
        }
        if (this.borders.left) {
            line(x, y + w, x, y);
        }
    }

    public highlight() {
        noStroke();
        fill(255, 255, 255, 255);
        let w = Cell.cellWidth;
        let x = this.col * Cell.cellWidth;
        let y = this.row * Cell.cellWidth;
        ellipse(x + w / 2, y + w / 2, w / 2, w / 2);
    }
}