var game;
function setup() {
    game = new MoriaGame(8, 10, 5);
    var canvas = createCanvas(game.width, game.height);
    canvas.parent('game');
    frameRate(10);
}
function draw() {
    background(0);
    var view = new GameView(game);
    view.draw();
    updateInfo();
}
function updateInfo() {
    var levelElt = document.getElementById("nLevel");
    levelElt.innerHTML = game.getLevel().toString();
}
function keyPressed() {
    if (game.getHero().life <= 0) {
        return;
    }
    if (keyCode === UP_ARROW) {
        game.moveHero(0);
    }
    else if (keyCode === DOWN_ARROW) {
        game.moveHero(1);
    }
    else if (keyCode === LEFT_ARROW) {
        game.moveHero(2);
    }
    else if (keyCode === RIGHT_ARROW) {
        game.moveHero(3);
    }
}
