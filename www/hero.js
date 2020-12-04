var Direction;
(function (Direction) {
    Direction[Direction["UP"] = 0] = "UP";
    Direction[Direction["DOWN"] = 1] = "DOWN";
    Direction[Direction["LEFT"] = 2] = "LEFT";
    Direction[Direction["RIGHT"] = 3] = "RIGHT";
})(Direction || (Direction = {}));
var Hero = (function () {
    function Hero(x, y) {
        this._x = x;
        this._y = y;
        this._life = 100;
    }
    Object.defineProperty(Hero.prototype, "x", {
        get: function () {
            return this._x;
        },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(Hero.prototype, "y", {
        get: function () {
            return this._y;
        },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(Hero.prototype, "life", {
        get: function () {
            return this._life;
        },
        enumerable: true,
        configurable: true
    });
    Hero.prototype.moveTo = function (x, y) {
        this._x = x;
        this._y = y;
    };
    Hero.prototype.move = function (dir) {
        var dx = 0, dy = 0;
        switch (dir) {
            case 0:
                dy = -1;
                break;
            case 1:
                dy = 1;
                break;
            case 2:
                dx = -1;
                break;
            case 3:
                dx = 1;
                break;
            default: break;
        }
        this._x += dx;
        this._y += dy;
        this._life--;
    };
    return Hero;
}());
