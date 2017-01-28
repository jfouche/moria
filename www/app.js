var maze;
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
var Maze = (function () {
    function Maze(nRows, nCols) {
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
    Maze.prototype.draw = function () {
        for (var _i = 0, _a = this.grid; _i < _a.length; _i++) {
            var rows = _a[_i];
            for (var _b = 0, rows_1 = rows; _b < rows_1.length; _b++) {
                var cell = rows_1[_b];
                cell.draw();
            }
        }
        this.currentCell.highlight();
    };
    Maze.prototype.iterate = function () {
        var next = this.getNextNeighbor();
        if (next) {
            next.visited = true;
            this.backtracking.push(this.currentCell);
            this.removeWalls(this.currentCell, next);
            this.currentCell = next;
        }
        else if (this.backtracking.length > 0) {
            next = this.backtracking.pop();
            this.currentCell = next;
        }
        else {
            console.log("FINISH");
            noLoop();
        }
    };
    Maze.prototype.getNextNeighbor = function () {
        var neighbors = [];
        var cell = this.currentCell;
        if (cell.row > 0) {
            var left = this.grid[cell.row - 1][cell.col];
            if (!left.visited) {
                neighbors.push(left);
            }
        }
        if (cell.row < this.nRows - 1) {
            var right = this.grid[cell.row + 1][cell.col];
            if (!right.visited) {
                neighbors.push(right);
            }
        }
        if (cell.col > 0) {
            var top_1 = this.grid[cell.row][cell.col - 1];
            if (!top_1.visited) {
                neighbors.push(top_1);
            }
        }
        if (cell.col < this.nCols - 1) {
            var bottom = this.grid[cell.row][cell.col + 1];
            if (!bottom.visited) {
                neighbors.push(bottom);
            }
        }
        var next = undefined;
        if (neighbors.length > 0) {
            var r = floor(random(0, neighbors.length));
            next = neighbors[r];
        }
        return next;
    };
    Maze.prototype.removeWalls = function (a, b) {
        if (a.col > b.col) {
            a.borders.left = false;
            b.borders.right = false;
        }
        else if (a.col < b.col) {
            a.borders.right = false;
            b.borders.left = false;
        }
        else if (a.row > b.row) {
            a.borders.top = false;
            b.borders.bottom = false;
        }
        else if (a.row < b.row) {
            a.borders.bottom = false;
            b.borders.top = false;
        }
    };
    return Maze;
}());
var CellBorders = (function () {
    function CellBorders() {
        this.top = true;
        this.right = true;
        this.bottom = true;
        this.left = true;
    }
    return CellBorders;
}());
var Cell = (function () {
    function Cell(row, col) {
        this.visited = false;
        this.row = row;
        this.col = col;
        this.borders = new CellBorders();
    }
    Cell.prototype.draw = function () {
        stroke(255);
        noFill();
        var w = Cell.cellWidth;
        var x = this.col * Cell.cellWidth;
        var y = this.row * Cell.cellWidth;
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
        if (this.visited) {
            noStroke();
            fill(255, 0, 255, 100);
            rect(x, y, w, w);
        }
    };
    Cell.prototype.highlight = function () {
        noStroke();
        fill(255, 255, 255, 255);
        var w = Cell.cellWidth;
        var x = this.col * Cell.cellWidth;
        var y = this.row * Cell.cellWidth;
        ellipse(x + w / 2, y + w / 2, w / 2, w / 2);
    };
    Cell.cellWidth = 30;
    return Cell;
}());
