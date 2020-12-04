/// <reference path="../typings/p5.d.ts" />

let game: MoriaGame;

function setup() {
    game = new MoriaGame(8, 10, 5);
    let canvas = createCanvas(game.width, game.height);
    canvas.parent('game');
    frameRate(10);
}

function draw() {
    background(0);
    let view = new GameView(game);
    view.draw();
    updateInfo();
}

function updateInfo() {
    let levelElt = document.getElementById("nLevel");
    levelElt.innerHTML = game.getLevel().toString();
}

function keyPressed() {
    if (keyCode === UP_ARROW) {
        game.moveHero(Direction.UP);
    } else if (keyCode === DOWN_ARROW) {
        game.moveHero(Direction.DOWN);
    } else if (keyCode === LEFT_ARROW) {
        game.moveHero(Direction.LEFT);
    } else if (keyCode === RIGHT_ARROW) {
        game.moveHero(Direction.RIGHT);
    }
}

const enum Direction {
    UP, DOWN, LEFT, RIGHT
}

class Offset {
    x: number;
    y: number;
    constructor(x: number, y: number) {
        this.x = x;
        this.y = y
    }
};

function directionOffset(dir: Direction): Offset {
    switch (dir) {
        case Direction.UP:
            return new Offset(0, -1);
        case Direction.DOWN:
            return new Offset(0, 1);
        case Direction.LEFT:
            return new Offset(-1, 0);
        case Direction.RIGHT:
            return new Offset(1, 0);
        default:
            break;
    }
    return undefined;
}