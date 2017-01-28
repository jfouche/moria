var maze;
function setup() {
    maze = new Maze(10, 10);
    createCanvas(maze.width, maze.height);
}
function draw() {
    background(124);
    maze.draw();
}
var Maze = (function () {
    function Maze(nRows, nCols) {
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
    Maze.prototype.draw = function () {
        for (var _i = 0, _a = this.grid; _i < _a.length; _i++) {
            var rows = _a[_i];
            for (var _b = 0, rows_1 = rows; _b < rows_1.length; _b++) {
                var cell = rows_1[_b];
                cell.draw();
            }
        }
    };
    return Maze;
}());
var Cell = (function () {
    function Cell(row, col) {
        this.row = row;
        this.col = col;
    }
    Cell.prototype.draw = function () {
        stroke(255);
        noFill();
        var w = Cell.cellWidth;
        var x = this.col * Cell.cellWidth;
        var y = this.row * Cell.cellWidth;
        line(x, y, x + w, y);
        line(x + w, y, x + w, y + w);
        line(x + w, y + w, x, y + w);
        line(x, y + w, x, y);
    };
    Cell.cellWidth = 20;
    return Cell;
}());
