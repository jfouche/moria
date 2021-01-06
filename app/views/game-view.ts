import { MoriaGame } from "../game"
import { MazeView } from "./maze-view";
import { HeroView } from "./hero-view";

import p5 = require('p5')
import { CellView } from "./cell-view";

/**
 * GameView
 */
export class GameView {
    public readonly game: MoriaGame;

    public readonly width: number;
    public readonly height: number;

    constructor(game: MoriaGame) {
        this.game = game;

        this.height = this.game.nRows * CellView.width + 1;
        this.width = this.game.nCols * CellView.width + 1;
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
