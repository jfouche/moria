import { Stair } from "../maze"
import { CellView } from "./cell-view";

import p5 = require('p5')

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
        let w = CellView.width - 6;
        let x = this.stair.col * CellView.width + 3;
        let y = this.stair.row * CellView.width + 3;
        p.rect(x, y, w, w);
    }
}
