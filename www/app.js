function setup() {
    createCanvas(400, 400);
}
function draw() {
    background(124);
}
var Maze = (function () {
    function Maze(nRows, nCols) {
        this.nRows = nRows;
        this.nCols = nCols;
    }
    return Maze;
}());
var Cell = (function () {
    function Cell(row, col) {
        this.row = row;
        this.col = col;
    }
    Cell.cellWidth = 20;
    return Cell;
}());
