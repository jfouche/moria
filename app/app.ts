/// <reference path="../typings/p5.d.ts" />

let maze: Maze;

function setup() {
    maze = new Maze(10, 10);
    createCanvas(maze.width, maze.height);
}

function draw() {
  background(124);
  maze.draw();
}

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

    public draw() {
        for (let rows of this.grid) {
            for (let cell of rows) {
                cell.draw();
            }
        }
    }
}

class Cell {
    public static cellWidth: number = 20;

    public readonly row: number;
    public readonly col: number;

    constructor(row: number, col: number) {
        this.row = row;
        this.col = col;
    }

    public draw() {
        stroke(255);
        noFill();
        let w = Cell.cellWidth;
        let x = this.col * Cell.cellWidth;
        let y = this.row * Cell.cellWidth;
        line(x, y, x+w, y);
        line(x+w, y, x+w, y+w);
        line(x+w, y+w, x, y+w);
        line(x, y+w, x, y);
    }
}