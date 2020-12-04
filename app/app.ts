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
