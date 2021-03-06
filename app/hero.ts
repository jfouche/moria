import { Cell } from "./maze";

/**
 * Hero
 */
export class Hero {
    private _x: number = 0;
    private _y: number = 0;
    private maxLife = 100;
    private _life: number = this.maxLife;

    public get x(): number {
        return this._x;
    }

    public get y(): number {
        return this._y;
    }

    public get life() {
        return this._life;
    }

    public goTo(cell: Cell) {
        this._x = cell.col;
        this._y = cell.row;
    }

    public moveTo(cell: Cell) {
        this.goTo(cell);
        this._life--;
    }

    public isOn(cell: Cell) {
        return this._x == cell.col && this._y == cell.row;
    }

    public changeLife(content: number) {
        this._life += content;
        if (this._life > this.maxLife) this._life = this.maxLife;
        if (this._life < 0) this._life = 0;
    }
}
