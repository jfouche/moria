var game;
function setup() {
    game = new MoriaGame(8, 10);
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
        this.maze.setUpstair(this.hero.y, this.hero.x);
        this.maze.setDownstair(this.nRows - 1, this.nCols - 1);
    }
    MoriaGame.prototype.draw = function () {
        background(0);
        this.maze.draw();
        this.hero.draw();
    };
    MoriaGame.prototype.moveHero = function (direction) {
        var cell = this.maze.cell(this.hero.y, this.hero.x);
        var cellBorders = this.maze.cell(this.hero.y, this.hero.x).borders;
        if (direction === 3 && !cellBorders.right) {
            this.move(1, 0);
        }
        else if (direction === 2 && !cellBorders.left) {
            this.move(-1, 0);
        }
        else if (direction === 0 && !cellBorders.top) {
            this.move(0, -1);
        }
        else if (direction === 1 && !cellBorders.bottom) {
            this.move(0, 1);
        }
    };
    MoriaGame.prototype.move = function (x, y) {
        this.hero.move(x, y);
        this.maze.cell(this.hero.y, this.hero.x).visited = true;
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
    Hero.prototype.move = function (x, y) {
        this._x += x;
        this._y += y;
    };
    return Hero;
}());
