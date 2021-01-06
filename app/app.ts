import { MoriaGame, Direction } from "./game"
import { GameView } from "./views"

import p5 = require('p5')

let game: MoriaGame;
let gview: GameView;

new p5((p: p5) => {
    p.setup = function () {
        game = new MoriaGame(8, 10, 5);
        gview = new GameView(game);
        let canvas = p.createCanvas(gview.width, gview.height);
        canvas.parent('game');
        p.frameRate(10);
    };

    p.draw = function () {
        p.background(0);
        gview.draw(p);
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
        } else if (p.keyCode === p.ENTER) {
            game.doAction();
        }
    }
});
