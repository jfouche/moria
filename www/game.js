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
    MoriaGame.prototype.getLevel = function () {
        return this.currentLevel;
    };
    MoriaGame.prototype.initLevel = function () {
        var maze = this.maze();
        this.hero = new Hero(maze.upstair.col, maze.upstair.row);
        maze.cell(this.hero.y, this.hero.x).visited = true;
        this.checkVisibility();
    };
    MoriaGame.prototype.maze = function () {
        return this.mazes[this.currentLevel];
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
