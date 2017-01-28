/// <reference path="../typings/p5.d.ts" />

function setup() {
    createCanvas(400, 400);
}

function draw() {
  background(124);
}

class Maze {
    public readonly nRows: number;
    public readonly nCols: number;
    
    constructor(nRows: number, nCols: number) {
        this.nRows = nRows;
        this.nCols = nCols;
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
}