import { LifeFlask } from "../item";
import { RoomView } from "./cell-view";

import p5 = require('p5')

export class LifeFlaskView {
    public readonly flask: LifeFlask;

    constructor(flask: LifeFlask) {
        this.flask = flask;
    }

    public draw(p: p5, room: RoomView) {
        p.push();
        p.fill('#FF0000');
        p.stroke('#FFCCCC');
        const x = room.x + 11;
        const w = RoomView.width - 2 * 11;
        const y = room.y + 7;
        const h = RoomView.width - 2 * 7;
        p.rect(x, y, w, h);
        p.pop();
    }
}