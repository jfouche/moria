/// <reference path="../typings/p5.d.ts" />

let maze: Maze;

function setup() {
    maze = new Maze(20, 30);
    createCanvas(maze.width, maze.height);
    frameRate(30);
}

function draw() {
  background(124);
  maze.draw();
  maze.iterate();
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
    private currentCell: Cell;
    private backtracking: Cell[];

    constructor(nRows: number, nCols: number) {
        this.nRows = nRows;
        this.nCols = nCols;
        this.height = nRows * Cell.cellWidth + 1;
        this.width = nCols * Cell.cellWidth + 1;
        this.grid = [];
        this.backtracking = [];

        for (var r = 0; r < this.nRows; r++) {
            this.grid[r] = [];
            for (var c = 0; c < this.nCols; c++) {
                this.grid[r][c] = new Cell(r, c);
            }
        }

        this.currentCell = this.grid[0][0];
        this.currentCell.visited = true;
    }

    public draw() {
        for (let rows of this.grid) {
            for (let cell of rows) {
                cell.draw();
            }
        }
        this.currentCell.highlight();
    }

    public iterate() {
        let next = this.getNextNeighbor();
        if (next) {
            next.visited = true;
            this.backtracking.push(this.currentCell);
            this.removeWalls(this.currentCell, next);
            this.currentCell = next;
        } else if (this.backtracking.length > 0) {
            next = this.backtracking.pop();
            this.currentCell = next;
        } else {
            console.log("FINISH");
            noLoop();
        }
    }

    private getNextNeighbor(): Cell {
        let neighbors: Cell[] = [];
        let cell = this.currentCell;
        if (cell.row > 0) {
            let left = this.grid[cell.row-1][cell.col];
            if (!left.visited) {
                neighbors.push(left);
            }
        }
        if (cell.row < this.nRows - 1) {
            let right = this.grid[cell.row+1][cell.col];
            if (!right.visited) {
                neighbors.push(right);
            }
        }
        if (cell.col > 0) {
            let top = this.grid[cell.row][cell.col-1];
            if (!top.visited) {
                neighbors.push(top);
            }
        }
        if (cell.col < this.nCols - 1) {
            let bottom = this.grid[cell.row][cell.col+1];
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
            line(x, y, x+w, y);
        }
        if (this.borders.right) {
            line(x+w, y, x+w, y+w);
        }
        if (this.borders.bottom) {
            line(x+w, y+w, x, y+w);
        }
        if (this.borders.left) {
            line(x, y+w, x, y);
        }
        if (this.visited) {
            noStroke();
            fill(255, 0, 255, 100);
            rect(x, y, w, w);
        }
    }

    public highlight() {
        noStroke();
        fill(255, 255, 255, 255);
        let w = Cell.cellWidth;
        let x = this.col * Cell.cellWidth;
        let y = this.row * Cell.cellWidth;
        ellipse(x + w/2, y + w/2, w/2, w/2);
    }
}