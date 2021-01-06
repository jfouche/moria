import { Cell } from "../maze"

import p5 = require('p5')

/**
 * @class CellView
 */
export class CellView {
    public static width: number = 30;

    public readonly cell: Cell;

    constructor(cell: Cell) {
        this.cell = cell;
    }

    public draw(p: p5) {
        let w = CellView.width;
        let x = this.cell.col * CellView.width;
        let y = this.cell.row * CellView.width;
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
    }

    public highlight(p: p5) {
        p.noStroke();
        p.fill(255, 255, 255, 255);
        let w = CellView.width;
        let x = this.cell.col * CellView.width;
        let y = this.cell.row * CellView.width;
        p.ellipse(x + w / 2, y + w / 2, w / 2, w / 2);
    }
}
