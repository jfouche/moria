var game;
function setup() {
    game = new MoriaGame(8, 10, 5);
    createCanvas(game.width, game.height);
    frameRate(10);
}
function draw() {
    background(0);
    game.draw();
}
function keyPressed() {
    if (keyCode === UP_ARROW) {
        game.moveHero(0);
    }
    else if (keyCode === DOWN_ARROW) {
        game.moveHero(1);
    }
    else if (keyCode === LEFT_ARROW) {
        game.moveHero(2);
    }
    else if (keyCode === RIGHT_ARROW) {
        game.moveHero(3);
    }
}
var Direction;
(function (Direction) {
    Direction[Direction["UP"] = 0] = "UP";
    Direction[Direction["DOWN"] = 1] = "DOWN";
    Direction[Direction["LEFT"] = 2] = "LEFT";
    Direction[Direction["RIGHT"] = 3] = "RIGHT";
})(Direction || (Direction = {}));
var Offset = (function () {
    function Offset(x, y) {
        this.x = x;
        this.y = y;
    }
    return Offset;
}());
;
function directionOffset(dir) {
    switch (dir) {
        case 0:
            return new Offset(0, -1);
        case 1:
            return new Offset(0, 1);
        case 2:
            return new Offset(-1, 0);
        case 3:
            return new Offset(1, 0);
        default:
            break;
    }
    return undefined;
}
var MoriaGame = (function () {
    function MoriaGame(nRows, nCols, nLevels) {
        this.nRows = nRows;
        this.nCols = nCols;
        var mazeGenerator = new MazeGenerator();
        this.mazes = [];
        for (var i = 0; i < nLevels; i++) {
            this.mazes.push(mazeGenerator.newMaze(this.nRows, this.nCols));
        }
        this.currentLevel = 0;
        var maze = this.maze();
        this.width = maze.width;
        this.height = maze.height;
        this.initLevel();
    }
    MoriaGame.prototype.initLevel = function () {
        var maze = this.maze();
        this.hero = new Hero(maze.upstair.col, maze.upstair.row);
        maze.cell(this.hero.y, this.hero.x).visited = true;
        this.checkVisibility();
    };
    MoriaGame.prototype.maze = function () {
        return this.mazes[this.currentLevel];
    };
    MoriaGame.prototype.draw = function () {
        background(0);
        this.maze().draw();
        this.hero.draw();
    };
    MoriaGame.prototype.moveHero = function (direction) {
        if (this.canMove(direction)) {
            this.hero.move(direction);
            this.maze().cell(this.hero.y, this.hero.x).visited = true;
            if (this.hero.x === this.maze().downstair.col && this.hero.y === this.maze().downstair.row) {
                this.currentLevel++;
                this.initLevel();
            }
            this.checkVisibility();
        }
    };
    MoriaGame.prototype.canMove = function (direction) {
        var cellBorders = this.maze().cell(this.hero.y, this.hero.x).borders;
        return (direction === 3 && !cellBorders.right)
            || (direction === 2 && !cellBorders.left)
            || (direction === 0 && !cellBorders.top)
            || (direction === 1 && !cellBorders.bottom);
    };
    MoriaGame.prototype.checkVisibility = function () {
        var _this = this;
        var x;
        var y;
        var cell;
        var maze = this.maze();
        var reset = function () {
            x = _this.hero.x;
            y = _this.hero.y;
            cell = maze.cell(y, x);
        };
        var next = function () {
            cell = maze.cell(y, x);
            cell.visited = true;
        };
        reset();
        while (!cell.borders.top) {
            y -= 1;
            next();
        }
        reset();
        while (!cell.borders.right) {
            x += 1;
            next();
        }
        reset();
        while (!cell.borders.bottom) {
            y += 1;
            next();
        }
        reset();
        while (!cell.borders.left) {
            x -= 1;
            next();
        }
    };
    return MoriaGame;
}());
var Hero = (function () {
    function Hero(x, y) {
        this._x = x;
        this._y = y;
    }
    Object.defineProperty(Hero.prototype, "x", {
        get: function () {
            return this._x;
        },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(Hero.prototype, "y", {
        get: function () {
            return this._y;
        },
        enumerable: true,
        configurable: true
    });
    Hero.prototype.draw = function () {
        stroke(255);
        fill(0, 255, 0);
        var x = this._x * Cell.cellWidth + Cell.cellWidth / 2;
        var y = this._y * Cell.cellWidth + Cell.cellWidth / 2;
        var r = Cell.cellWidth / 2 - 1;
        ellipse(x, y, r, r);
    };
    Hero.prototype.moveTo = function (x, y) {
        this._x += x;
        this._y += y;
    };
    Hero.prototype.move = function (dir) {
        var offset = directionOffset(dir);
        this._x += offset.x;
        this._y += offset.y;
    };
    return Hero;
}());
