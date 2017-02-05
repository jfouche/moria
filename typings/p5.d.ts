declare function frameRate(fps: number): number;
declare function noLoop(): void;

declare function createCanvas(width: number, height: number): void;

declare function background(color: number): void;

declare function noStroke(): void;
declare function stroke(color: number): void;

declare function noFill(): void;
declare function fill(r: number, g: number, b: number, a?: number): void;

declare function rect(x: number, y:number, w: number, h: number): void;
declare function line(x1: number, y1:number, x2: number, y2: number): void;
declare function ellipse(x: number, y: number, w: number, h: number): void;


declare function floor(n: number): number;

declare function random(min: number, max: number): number;

declare let keyCode: number;
declare let UP_ARROW: number;
declare let DOWN_ARROW: number;
declare let RIGHT_ARROW: number;
declare let LEFT_ARROW: number;
