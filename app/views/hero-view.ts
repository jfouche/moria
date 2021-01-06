import { CellView } from "./cell-view";
import { Hero } from "../hero"

import p5 = require('p5')

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
        let x = this.hero.x * CellView.width + CellView.width / 2;
        let y = this.hero.y * CellView.width + CellView.width / 2;
        let r = CellView.width / 2 - 1;
        p.ellipse(x, y, r, r);
    }
}