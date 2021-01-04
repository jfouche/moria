interface Canvas {
    parent(elt: string): void;
}

declare class p5 {
    constructor(object: any);
    frameRate(fps: number): number;
    noLoop(): void;

    createCanvas(width: number, height: number): Canvas;

    background(color: number): void;

    noStroke(): void;
    stroke(color: number | string): void;

    noFill(): void;
    fill(r: number | string, g?: number, b?: number, a?: number): void;

    rect(x: number, y: number, w: number, h: number): void;
    line(x1: number, y1: number, x2: number, y2: number): void;
    ellipse(x: number, y: number, w: number, h: number): void;

    static random(min: number, max: number): number;

    keyCode: number;
    UP_ARROW: number;
    DOWN_ARROW: number;
    RIGHT_ARROW: number;
    LEFT_ARROW: number;
}