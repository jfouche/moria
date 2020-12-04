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
var Direction;
(function (Direction) {
    Direction[Direction["UP"] = 0] = "UP";
    Direction[Direction["DOWN"] = 1] = "DOWN";
    Direction[Direction["LEFT"] = 2] = "LEFT";
    Direction[Direction["RIGHT"] = 3] = "RIGHT";
})(Direction || (Direction = {}));
var Offset = (function () {
    function Offset(x, y) {
        this.x = x;
        this.y = y;
    }
    return Offset;
}());
;
function directionOffset(dir) {
    switch (dir) {
        case 0:
            return new Offset(0, -1);
        case 1:
            return new Offset(0, 1);
        case 2:
            return new Offset(-1, 0);
        case 3:
            return new Offset(1, 0);
        default:
            break;
    }
    return undefined;
}
