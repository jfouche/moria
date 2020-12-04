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
        this.upstair = Stair.upstair(0, 0);
        this.downstair = Stair.downstairstair(nRows - 1, nCols - 1);
    }
    Maze.prototype.cell = function (row, col) {
        return this.grid[row][col];
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
                this.removeWallsBetween(currentCell, next);
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
        this.removeRandomWalls(maze, 10);
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
    MazeGenerator.prototype.removeWallsBetween = function (a, b) {
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
    MazeGenerator.prototype.removeRandomWalls = function (maze, n) {
        for (var i = 0; i < n;) {
            var r = floor(random(1, maze.nRows - 2));
            var c = floor(random(1, maze.nCols - 2));
            var cell = maze.cell(r, c);
            var next = floor(random(0, 3));
            switch (next) {
                case 0:
                    if (cell.borders.top) {
                        this.removeWallsBetween(cell, maze.cell(r - 1, c));
                        console.log("remove (%d, %d) : top", c, r);
                        i++;
                    }
                    break;
                case 1:
                    if (cell.borders.right) {
                        this.removeWallsBetween(cell, maze.cell(r, c + 1));
                        console.log("remove (%d, %d) : right", c, r);
                        i++;
                    }
                    break;
                case 2:
                    if (cell.borders.bottom) {
                        this.removeWallsBetween(cell, maze.cell(r + 1, c));
                        console.log("remove (%d, %d) : bottom", c, r);
                        i++;
                    }
                    break;
                case 3:
                    if (cell.borders.left) {
                        this.removeWallsBetween(cell, maze.cell(r, c - 1));
                        console.log("remove (%d, %d) : left", c, r);
                        i++;
                    }
                    break;
                default:
                    break;
            }
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
    Cell.cellWidth = 30;
    return Cell;
}());
var Stair = (function () {
    function Stair(row, col, up) {
        this.row = row;
        this.col = col;
        this.up = up;
    }
    Stair.upstair = function (row, col) {
        return new Stair(row, col, true);
    };
    Stair.downstairstair = function (row, col) {
        return new Stair(row, col, false);
    };
    return Stair;
}());
