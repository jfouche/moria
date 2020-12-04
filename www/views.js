var MazeView = (function () {
    function MazeView(maze) {
        this.maze = maze;
    }
    MazeView.prototype.draw = function () {
        for (var r = 0; r < this.maze.nRows; r++) {
            for (var c = 0; c < this.maze.nCols; c++) {
                var cell = this.maze.cell(r, c);
                if (cell.visited) {
                    var cv = new CellView(cell);
                    cv.draw();
                }
            }
        }
        if (this.maze.cell(this.maze.upstair.row, this.maze.upstair.col).visited) {
            var sv = new StairView(this.maze.upstair);
            sv.draw();
        }
        if (this.maze.cell(this.maze.downstair.row, this.maze.downstair.col).visited) {
            var sv = new StairView(this.maze.downstair);
            sv.draw();
        }
    };
    return MazeView;
}());
var CellView = (function () {
    function CellView(cell) {
        this.cell = cell;
    }
    CellView.prototype.draw = function () {
        var w = Cell.cellWidth;
        var x = this.cell.col * Cell.cellWidth;
        var y = this.cell.row * Cell.cellWidth;
        var b = 4;
        noStroke();
        fill(25, 25, 25);
        rect(x, y, w, w);
        stroke(255);
        noFill();
        line(x, y, x, y + b);
        line(x + w, y, x + w, y + b);
        line(x + w, y, x + w - b, y);
        line(x + w, y + w, x + w - b, y + w);
        line(x + w, y + w, x + w, y + w - b);
        line(x, y + w, x, y + w - b);
        line(x, y + w, x + b, y + w);
        line(x, y, x + b, y);
        if (this.cell.borders.top) {
            line(x, y, x + w, y);
        }
        if (this.cell.borders.right) {
            line(x + w, y, x + w, y + w);
        }
        if (this.cell.borders.bottom) {
            line(x + w, y + w, x, y + w);
        }
        if (this.cell.borders.left) {
            line(x, y + w, x, y);
        }
    };
    CellView.prototype.highlight = function () {
        noStroke();
        fill(255, 255, 255, 255);
        var w = Cell.cellWidth;
        var x = this.cell.col * Cell.cellWidth;
        var y = this.cell.row * Cell.cellWidth;
        ellipse(x + w / 2, y + w / 2, w / 2, w / 2);
    };
    CellView.cellWidth = 30;
    return CellView;
}());
var StairView = (function () {
    function StairView(stair) {
        this.stair = stair;
    }
    StairView.prototype.draw = function () {
        stroke(255);
        if (this.stair.up) {
            fill(192, 192, 192);
        }
        else {
            fill(70, 70, 70);
        }
        var w = Cell.cellWidth - 6;
        var x = this.stair.col * Cell.cellWidth + 3;
        var y = this.stair.row * Cell.cellWidth + 3;
        rect(x, y, w, w);
    };
    return StairView;
}());
var GameView = (function () {
    function GameView(game) {
        this.game = game;
    }
    GameView.prototype.draw = function () {
        background(0);
        var mv = new MazeView(this.game.maze());
        mv.draw();
        var hero = this.game.getHero();
        var hv = new HeroView(hero);
        hv.draw();
        document.getElementById("nLevel").innerHTML = this.game.getLevel().toString();
        document.getElementById("life").innerHTML = hero.life.toString();
    };
    return GameView;
}());
var HeroView = (function () {
    function HeroView(hero) {
        this.hero = hero;
    }
    HeroView.prototype.draw = function () {
        stroke(255);
        if (this.hero.life > 0) {
            fill(0, 255, 0);
        }
        else {
            fill(255, 10, 10);
        }
        var x = this.hero.x * Cell.cellWidth + Cell.cellWidth / 2;
        var y = this.hero.y * Cell.cellWidth + Cell.cellWidth / 2;
        var r = Cell.cellWidth / 2 - 1;
        ellipse(x, y, r, r);
    };
    return HeroView;
}());
