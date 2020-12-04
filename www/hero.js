var Hero = (function () {
    function Hero(x, y) {
        this._x = x;
        this._y = y;
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
    Hero.prototype.moveTo = function (x, y) {
        this._x += x;
        this._y += y;
    };
    Hero.prototype.move = function (dir) {
        var offset = directionOffset(dir);
        this._x += offset.x;
        this._y += offset.y;
    };
    return Hero;
}());
