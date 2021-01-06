import { MoriaGame } from "../game"
import { MazeView } from "./maze-view";
import { HeroView } from "./hero-view";

import p5 = require('p5')
import { RoomView } from "./cell-view";

/**
 * GameView
 */
export class GameView {
    public readonly game: MoriaGame;

    public readonly width: number;
    public readonly height: number;

    constructor(game: MoriaGame) {
        this.game = game;

        this.height = this.game.nRows * RoomView.width + 1;
        this.width = this.game.nCols * RoomView.width + 1;
    }

    public draw(p: p5) {
        p.background(0);
        const mv = new MazeView(this.game.maze());
        mv.draw(p);
        const hero = this.game.getHero();
        const hv = new HeroView(hero);
        hv.draw(p);

        const level = this.game.getLevel() + 1;
        document.getElementById("nLevel").innerHTML = `Level ${level}`;
        const life = hero.life.toString();
        document.getElementById("life").innerHTML = `Life ${life}`;
    }
}
