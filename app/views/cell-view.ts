import { Room, Stair } from "../maze"

import p5 = require('p5')
import { LifeFlask } from "../item";
import { LifeFlaskView } from "./item-view";

/**
 * @class CellView
 */
export class RoomView {
    public static width: number = 30;

    public readonly cell: Room;
    public readonly x;
    public readonly y;

    constructor(cell: Room) {
        this.cell = cell;
        this.x = this.cell.col * RoomView.width;
        this.y = this.cell.row * RoomView.width;
    }

    public draw(p: p5) {
        const w = RoomView.width;
        const x = this.x;
        const y = this.y;
        const bg = '#222222';
        const wallColor = '#EEEEEE';
        const doorColor = '#444444';

        // The room
        p.stroke(wallColor);
        p.fill(bg);
        p.rect(x, y, w, w)

        // Doors
        const b = 5;
        p.stroke(doorColor);
        if (!this.cell.borders.top) {
            p.line(x + b, y, x + w - b, y);
        }
        if (!this.cell.borders.right) {
            p.line(x + w, y + b, x + w, y + w - b);
        }
        if (!this.cell.borders.bottom) {
            p.line(x + b, y + w, x + w - b, y + w);
        }
        if (!this.cell.borders.left) {
            p.line(x, y + b, x, y + w - b);
        }

        // Item
        if (this.cell.item) {
            if (this.cell.item instanceof LifeFlask) {
                const lfView = new LifeFlaskView(this.cell.item);
                lfView.draw(p, this);
            }
        }
    }

    public highlight(p: p5) {
        p.noStroke();
        p.fill(255, 255, 255, 255);
        let w = RoomView.width;
        let x = this.cell.col * RoomView.width;
        let y = this.cell.row * RoomView.width;
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
        let w = RoomView.width - 6;
        let x = this.stair.col * RoomView.width + 3;
        let y = this.stair.row * RoomView.width + 3;
        p.rect(x, y, w, w);
    }
}
