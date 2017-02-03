var maze;
function setup() {
    var mazeGenerator = new MazeGenerator();
    maze = mazeGenerator.newMaze(20, 30);
    createCanvas(maze.width, maze.height);
    frameRate(30);
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
    Maze.prototype.cell = function (row, col) {
        return this.grid[row][col];
    };
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
var MazeGenerator = (function () {
    function MazeGenerator() {
    }
    MazeGenerator.prototype.newMaze = function (nRows, nCols) {
        var maze = new Maze(nRows, nCols);
        var backtracking = [];
        var currentCell = maze.cell(0, 0);
        currentCell.visited = true;
        var finished = false;
        while (!finished) {
            var next = this.getNextNeighbor(maze, currentCell);
            if (next) {
                next.visited = true;
                backtracking.push(currentCell);
                this.removeWalls(currentCell, next);
                currentCell = next;
            }
            else if (backtracking.length > 0) {
                next = backtracking.pop();
                currentCell = next;
            }
            else {
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
    };
    MazeGenerator.prototype.getNextNeighbor = function (maze, cell) {
        var neighbors = [];
        if (cell.row > 0) {
            var left = maze.cell(cell.row - 1, cell.col);
            if (!left.visited) {
                neighbors.push(left);
            }
        }
        if (cell.row < maze.nRows - 1) {
            var right = maze.cell(cell.row + 1, cell.col);
            if (!right.visited) {
                neighbors.push(right);
            }
        }
        if (cell.col > 0) {
            var top_1 = maze.cell(cell.row, cell.col - 1);
            if (!top_1.visited) {
                neighbors.push(top_1);
            }
        }
        if (cell.col < maze.nCols - 1) {
            var bottom = maze.cell(cell.row, cell.col + 1);
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
    MazeGenerator.prototype.removeWalls = function (a, b) {
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
    return MazeGenerator;
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
