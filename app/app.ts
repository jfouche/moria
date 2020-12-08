/// <reference path="../typings/p5.d.ts" />

import { MoriaGame } from "./game"
import { GameView } from "./views"
import { Direction } from "./hero"


let game: MoriaGame;

function updateInfo() {
    let levelElt = document.getElementById("nLevel");
    levelElt.innerHTML = game.getLevel().toString();
}

let sketch = function (p: any) {
    p.setup = function () {
        game = new MoriaGame(8, 10, 5);
        let canvas = p.createCanvas(game.width, game.height);
        canvas.parent('game');
        p.frameRate(10);
    };

    p.draw = function () {
        p.background(0);
        let view = new GameView(game);
        view.draw(p);
        updateInfo();
    };

    p.keyPressed = function () {
        if (game.getHero().life <= 0) {
            return;
        }
        if (p.keyCode === p.UP_ARROW) {
            game.moveHero(Direction.UP);
        } else if (p.keyCode === p.DOWN_ARROW) {
            game.moveHero(Direction.DOWN);
        } else if (p.keyCode === p.LEFT_ARROW) {
            game.moveHero(Direction.LEFT);
        } else if (p.keyCode === p.RIGHT_ARROW) {
            game.moveHero(Direction.RIGHT);
        }
    }
};

let myp5 = new p5(sketch);