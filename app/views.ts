/**
 * @class MazeView
 */
class MazeView {
    public readonly maze: Maze;

    constructor(maze: Maze) {
        this.maze = maze;
    }
    public draw() {
        for (let r = 0; r < this.maze.nRows; r++) {
            for (let c = 0; c < this.maze.nCols; c++) {
                let cell = this.maze.cell(r, c);
                if (cell.visited) {
                    let cv = new CellView(cell);
                    cv.draw();
                }
            }
        }

        if (this.maze.cell(this.maze.upstair.row, this.maze.upstair.col).visited) {
            let sv = new StairView(this.maze.upstair);
            sv.draw();
        }
        if (this.maze.cell(this.maze.downstair.row, this.maze.downstair.col).visited) {
            let sv = new StairView(this.maze.downstair);
            sv.draw();
        }
    }
}

/**
 * @class CellView
 */
class CellView {
    public static cellWidth: number = 30;

    public readonly cell: Cell;

    constructor(cell: Cell) {
        this.cell = cell;
    }

    public draw() {
        let w = Cell.cellWidth;
        let x = this.cell.col * Cell.cellWidth;
        let y = this.cell.row * Cell.cellWidth;
        const b = 4;

        noStroke();
        fill(15, 15, 15);

        rect(x, y, w, w)
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
    }

    public highlight() {
        noStroke();
        fill(255, 255, 255, 255);
        let w = Cell.cellWidth;
        let x = this.cell.col * Cell.cellWidth;
        let y = this.cell.row * Cell.cellWidth;
        ellipse(x + w / 2, y + w / 2, w / 2, w / 2);
    }
}

/**
 * StairView
 */
class StairView {
    public readonly stair: Stair;

    constructor(stair: Stair) {
        this.stair = stair;
    }

    public draw() {
        stroke(255);
        if (this.stair.up) {
            fill(192, 192, 192);
        }
        else {
            fill(70, 70, 70);
        }
        let w = Cell.cellWidth - 6;
        let x = this.stair.col * Cell.cellWidth + 3;
        let y = this.stair.row * Cell.cellWidth + 3;
        rect(x, y, w, w);
    }
}

/**
 * GameView
 */
class GameView {
    public readonly game: MoriaGame;

    constructor(game: MoriaGame) {
        this.game = game;
    }

    public draw() {
        background(0);
        let mv = new MazeView(this.game.maze());
        mv.draw();
        let hv = new HeroView(this.game.hero);
        hv.draw();
    }
}

/**
 * HeroView
 */
class HeroView {
    private hero: Hero;

    constructor(hero: Hero) {
        this.hero = hero;
    }


    public draw() {
        stroke(255);
        fill(0, 255, 0);
        let x = this.hero.x * Cell.cellWidth + Cell.cellWidth / 2;
        let y = this.hero.y * Cell.cellWidth + Cell.cellWidth / 2;
        let r = Cell.cellWidth / 2 - 1;
        ellipse(x, y, r, r);
    }
}