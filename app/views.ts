import { Maze, Cell, Stair } from "./maze"
import { MoriaGame } from "./game"
import { Hero } from "./hero"

/**
 * @class MazeView
 */
export class MazeView {
    public readonly maze: Maze;

    constructor(maze: Maze) {
        this.maze = maze;
    }
    public draw(p: p5) {
        for (let r = 0; r < this.maze.nRows; r++) {
            for (let c = 0; c < this.maze.nCols; c++) {
                let cell = this.maze.cell(r, c);
                if (cell.visited) {
                    let cv = new CellView(cell);
                    cv.draw(p);
                }
            }
        }

        if (this.maze.cell(this.maze.upstair.row, this.maze.upstair.col).visited) {
            let sv = new StairView(this.maze.upstair);
            sv.draw(p);
        }
        if (this.maze.cell(this.maze.downstair.row, this.maze.downstair.col).visited) {
            let sv = new StairView(this.maze.downstair);
            sv.draw(p);
        }
    }
}

/**
 * @class CellView
 */
export class CellView {
    public static cellWidth: number = 30;

    public readonly cell: Cell;

    constructor(cell: Cell) {
        this.cell = cell;
    }

    public draw(p: p5) {
        let w = Cell.cellWidth;
        let x = this.cell.col * Cell.cellWidth;
        let y = this.cell.row * Cell.cellWidth;
        const b = 4;

        p.noStroke();
        p.fill(25, 25, 25);

        p.rect(x, y, w, w)
        p.stroke(255);
        p.noFill();
        p.line(x, y, x, y + b);
        p.line(x + w, y, x + w, y + b);
        p.line(x + w, y, x + w - b, y);
        p.line(x + w, y + w, x + w - b, y + w);
        p.line(x + w, y + w, x + w, y + w - b);
        p.line(x, y + w, x, y + w - b);
        p.line(x, y + w, x + b, y + w);
        p.line(x, y, x + b, y);
        if (this.cell.borders.top) {
            p.line(x, y, x + w, y);
        }
        if (this.cell.borders.right) {
            p.line(x + w, y, x + w, y + w);
        }
        if (this.cell.borders.bottom) {
            p.line(x + w, y + w, x, y + w);
        }
        if (this.cell.borders.left) {
            p.line(x, y + w, x, y);
        }
    }

    public highlight(p: p5) {
        p.noStroke();
        p.fill(255, 255, 255, 255);
        let w = Cell.cellWidth;
        let x = this.cell.col * Cell.cellWidth;
        let y = this.cell.row * Cell.cellWidth;
        p.ellipse(x + w / 2, y + w / 2, w / 2, w / 2);
    }
}

/**
 * StairView
 */
export class StairView {
    public readonly stair: Stair;

    constructor(stair: Stair) {
        this.stair = stair;
    }

    public draw(p: p5) {
        p.stroke(255);
        if (this.stair.up) {
            p.fill(192, 192, 192);
        }
        else {
            p.fill(70, 70, 70);
        }
        let w = Cell.cellWidth - 6;
        let x = this.stair.col * Cell.cellWidth + 3;
        let y = this.stair.row * Cell.cellWidth + 3;
        p.rect(x, y, w, w);
    }
}

/**
 * GameView
 */
export class GameView {
    public readonly game: MoriaGame;

    constructor(game: MoriaGame) {
        this.game = game;
    }

    public draw(p: p5) {
        p.background(0);
        let mv = new MazeView(this.game.maze());
        mv.draw(p);
        let hero = this.game.getHero();
        let hv = new HeroView(hero);
        hv.draw(p);

        document.getElementById("nLevel").innerHTML = this.game.getLevel().toString();
        document.getElementById("life").innerHTML = hero.life.toString();
    }
}

/**
 * HeroView
 */
export class HeroView {
    private hero: Hero;

    constructor(hero: Hero) {
        this.hero = hero;
    }

    public draw(p: p5) {
        p.stroke(255);
        if (this.hero.life > 0) {
            p.fill(0, 255, 0);
        }
        else {
            p.fill(80, 0, 0);
        }
        let x = this.hero.x * Cell.cellWidth + Cell.cellWidth / 2;
        let y = this.hero.y * Cell.cellWidth + Cell.cellWidth / 2;
        let r = Cell.cellWidth / 2 - 1;
        p.ellipse(x, y, r, r);
    }
}