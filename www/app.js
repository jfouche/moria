var game;
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
    }
    else if (keyCode === DOWN_ARROW) {
        game.moveHero(0, 1);
    }
    else if (keyCode === LEFT_ARROW) {
        game.moveHero(-1, 0);
    }
    else if (keyCode === RIGHT_ARROW) {
        game.moveHero(1, 0);
    }
}
var MoriaGame = (function () {
    function MoriaGame(nRows, nCols) {
        this.nRows = nRows;
        this.nCols = nCols;
        var mazeGenerator = new MazeGenerator();
        this.maze = mazeGenerator.newMaze(this.nRows, this.nCols);
        this.width = this.maze.width;
        this.height = this.maze.height;
        this.hero = new Hero(0, 0);
        this.maze.cell(this.hero.y, this.hero.x).visited = true;
    }
    MoriaGame.prototype.draw = function () {
        background(0);
        this.maze.draw();
        this.hero.draw();
    };
    MoriaGame.prototype.moveHero = function (x, y) {
        var cell = this.maze.cell(this.hero.y, this.hero.x);
        var cellBorders = cell.borders;
        if (x === 1 && !cellBorders.right) {
            this.hero.x++;
            this.maze.cell(this.hero.y, this.hero.x).visited = true;
        }
        else if (x === -1 && !cellBorders.left) {
            this.hero.x--;
            this.maze.cell(this.hero.y, this.hero.x).visited = true;
        }
        else if (y === -1 && !cellBorders.top) {
            this.hero.y--;
            this.maze.cell(this.hero.y, this.hero.x).visited = true;
        }
        else if (y === 1 && !cellBorders.bottom) {
            this.hero.y++;
            this.maze.cell(this.hero.y, this.hero.x).visited = true;
        }
    };
    return MoriaGame;
}());
var Hero = (function () {
    function Hero(x, y) {
        this.x = x;
        this.y = y;
    }
    Hero.prototype.draw = function () {
        stroke(255);
        fill(0, 255, 0);
        var x = this.x * Cell.cellWidth + Cell.cellWidth / 2;
        var y = this.y * Cell.cellWidth + Cell.cellWidth / 2;
        var r = Cell.cellWidth / 2 - 1;
        ellipse(x, y, r, r);
    };
    return Hero;
}());
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
                if (cell.visited) {
                    cell.draw();
                }
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
    };
    Cell.prototype.highlight = function () {
        noStroke();
        fill(255, 255, 255, 255);
        var w = Cell.cellWidth;
        var x = this.col * Cell.cellWidth;
        var y = this.row * Cell.cellWidth;
        ellipse(x + w / 2, y + w / 2, w / 2, w / 2);
    };
    return Cell;
}());
Cell.cellWidth = 30;
