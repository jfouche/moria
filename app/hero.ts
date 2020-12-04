const enum Direction {
    UP, DOWN, LEFT, RIGHT
}

/**
 * Hero
 */
class Hero {
    private _x: number;
    private _y: number;

    constructor(x: number, y: number) {
        this._x = x;
        this._y = y;
    }

    public get x(): number {
        return this._x;
    }

    public get y(): number {
        return this._y;
    }

    public moveTo(x: number, y: number) {
        this._x += x;
        this._y += y;
    }

    public move(dir: Direction) {
        let dx = 0, dy = 0;
        switch (dir) {
            case Direction.UP: dy = -1; break;
            case Direction.DOWN: dy = 1; break;
            case Direction.LEFT: dx = -1; break;
            case Direction.RIGHT: dx = 1; break;
            default: break;
        }
        this._x += dx;
        this._y += dy;
    }
}
