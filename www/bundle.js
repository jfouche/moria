/******/ (() => { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ({

/***/ "./app/app.ts":
/*!********************!*
  !*** ./app/app.ts ***!
  \********************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony import */ var _game__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./game */ "./app/game.ts");
/* harmony import */ var _views__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./views */ "./app/views.ts");


let game;
function updateInfo() {
    let levelElt = document.getElementById("nLevel");
    levelElt.innerHTML = game.getLevel().toString();
}
let sketch = function (p) {
    p.setup = function () {
        game = new _game__WEBPACK_IMPORTED_MODULE_0__.MoriaGame(8, 10, 5);
        let canvas = p.createCanvas(game.width, game.height);
        canvas.parent('game');
        p.frameRate(10);
    };
    p.draw = function () {
        p.background(0);
        let view = new _views__WEBPACK_IMPORTED_MODULE_1__.GameView(game);
        view.draw(p);
        updateInfo();
    };
    p.keyPressed = function () {
        if (game.getHero().life <= 0) {
            return;
        }
        if (p.keyCode === p.UP_ARROW) {
            game.moveHero(0);
        }
        else if (p.keyCode === p.DOWN_ARROW) {
            game.moveHero(1);
        }
        else if (p.keyCode === p.LEFT_ARROW) {
            game.moveHero(2);
        }
        else if (p.keyCode === p.RIGHT_ARROW) {
            game.moveHero(3);
        }
    };
};
let myp5 = new p5(sketch);


/***/ }),

/***/ "./app/game.ts":
/*!*********************!*
  !*** ./app/game.ts ***!
  \*********************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "MoriaGame": () => /* binding */ MoriaGame
/* harmony export */ });
/* harmony import */ var _hero__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./hero */ "./app/hero.ts");
/* harmony import */ var _maze__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./maze */ "./app/maze.ts");


class MoriaGame {
    constructor(nRows, nCols, nLevels) {
        this.nRows = nRows;
        this.nCols = nCols;
        let mazeGenerator = new _maze__WEBPACK_IMPORTED_MODULE_1__.MazeGenerator();
        this.mazes = [];
        for (let i = 0; i < nLevels; i++) {
            this.mazes.push(mazeGenerator.newMaze(this.nRows, this.nCols));
        }
        this.currentLevel = 0;
        let maze = this.maze();
        this.hero = new _hero__WEBPACK_IMPORTED_MODULE_0__.Hero(maze.upstair.col, maze.upstair.row);
        this.width = maze.width;
        this.height = maze.height;
        this.initLevel();
    }
    getLevel() {
        return this.currentLevel;
    }
    initLevel() {
        let maze = this.maze();
        this.hero.moveTo(maze.upstair.col, maze.upstair.row);
        maze.cell(this.hero.y, this.hero.x).visited = true;
        this.checkVisibility();
    }
    maze() {
        return this.mazes[this.currentLevel];
    }
    moveHero(direction) {
        if (this.canMove(direction)) {
            this.hero.move(direction);
            this.maze().cell(this.hero.y, this.hero.x).visited = true;
            if (this.hero.x === this.maze().downstair.col && this.hero.y === this.maze().downstair.row) {
                this.currentLevel++;
                this.initLevel();
            }
            this.checkVisibility();
        }
    }
    canMove(direction) {
        let cellBorders = this.maze().cell(this.hero.y, this.hero.x).borders;
        return (direction === 3 && !cellBorders.right)
            || (direction === 2 && !cellBorders.left)
            || (direction === 0 && !cellBorders.top)
            || (direction === 1 && !cellBorders.bottom);
    }
    checkVisibility() {
        let x;
        let y;
        let cell;
        let maze = this.maze();
        let reset = () => {
            x = this.hero.x;
            y = this.hero.y;
            cell = maze.cell(y, x);
        };
        let next = () => {
            cell = maze.cell(y, x);
            cell.visited = true;
        };
        reset();
        while (!cell.borders.top) {
            y -= 1;
            next();
        }
        reset();
        while (!cell.borders.right) {
            x += 1;
            next();
        }
        reset();
        while (!cell.borders.bottom) {
            y += 1;
            next();
        }
        reset();
        while (!cell.borders.left) {
            x -= 1;
            next();
        }
    }
    getHero() {
        return this.hero;
    }
}


/***/ }),

/***/ "./app/hero.ts":
/*!*********************!*
  !*** ./app/hero.ts ***!
  \*********************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "Hero": () => /* binding */ Hero
/* harmony export */ });
class Hero {
    constructor(x, y) {
        this._x = x;
        this._y = y;
        this._life = 100;
    }
    get x() {
        return this._x;
    }
    get y() {
        return this._y;
    }
    get life() {
        return this._life;
    }
    moveTo(x, y) {
        this._x = x;
        this._y = y;
    }
    move(dir) {
        let dx = 0, dy = 0;
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
    }
}


/***/ }),

/***/ "./app/maze.ts":
/*!*********************!*
  !*** ./app/maze.ts ***!
  \*********************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "Maze": () => /* binding */ Maze,
/* harmony export */   "MazeGenerator": () => /* binding */ MazeGenerator,
/* harmony export */   "CellBorders": () => /* binding */ CellBorders,
/* harmony export */   "Cell": () => /* binding */ Cell,
/* harmony export */   "Stair": () => /* binding */ Stair
/* harmony export */ });
class Maze {
    constructor(nRows, nCols) {
        this.nRows = nRows;
        this.nCols = nCols;
        this.height = nRows * Cell.cellWidth + 1;
        this.width = nCols * Cell.cellWidth + 1;
        this.grid = [];
        for (var r = 0; r < this.nRows; r++) {
            this.grid[r] = [];
            for (var c = 0; c < this.nCols; c++) {
                this.grid[r][c] = new Cell(r, c);
            }
        }
        this.upstair = Stair.upstair(0, 0);
        this.downstair = Stair.downstairstair(nRows - 1, nCols - 1);
    }
    cell(row, col) {
        return this.grid[row][col];
    }
}
class MazeGenerator {
    newMaze(nRows, nCols) {
        let maze = new Maze(nRows, nCols);
        let backtracking = [];
        let currentCell = maze.cell(0, 0);
        currentCell.visited = true;
        let finished = false;
        while (!finished) {
            let next = this.getNextNeighbor(maze, currentCell);
            if (next) {
                next.visited = true;
                backtracking.push(currentCell);
                this.removeWallsBetween(currentCell, next);
                currentCell = next;
            }
            else if (backtracking.length > 0) {
                next = backtracking.pop();
                currentCell = next;
            }
            else {
                console.log("FINISH");
                finished = true;
            }
        }
        for (var r = 0; r < nRows; r++) {
            for (var c = 0; c < nCols; c++) {
                maze.cell(r, c).visited = false;
            }
        }
        this.removeRandomWalls(maze, 10);
        return maze;
    }
    getNextNeighbor(maze, cell) {
        let neighbors = [];
        if (cell.row > 0) {
            let left = maze.cell(cell.row - 1, cell.col);
            if (!left.visited) {
                neighbors.push(left);
            }
        }
        if (cell.row < maze.nRows - 1) {
            let right = maze.cell(cell.row + 1, cell.col);
            if (!right.visited) {
                neighbors.push(right);
            }
        }
        if (cell.col > 0) {
            let top = maze.cell(cell.row, cell.col - 1);
            if (!top.visited) {
                neighbors.push(top);
            }
        }
        if (cell.col < maze.nCols - 1) {
            let bottom = maze.cell(cell.row, cell.col + 1);
            if (!bottom.visited) {
                neighbors.push(bottom);
            }
        }
        let next = undefined;
        if (neighbors.length > 0) {
            var r = random(0, neighbors.length);
            next = neighbors[r];
        }
        if (next)
            console.log(`getNextNeighbor(maze, {${cell.row}, ${cell.col}}) => (${next.row}, ${next.col})`);
        else
            console.log(`getNextNeighbor(maze, {${cell.row}, ${cell.col}}) => undefined`);
        return next;
    }
    removeWallsBetween(a, b) {
        if (a.col > b.col) {
            a.borders.left = false;
            b.borders.right = false;
        }
        else if (a.col < b.col) {
            a.borders.right = false;
            b.borders.left = false;
        }
        else if (a.row > b.row) {
            a.borders.top = false;
            b.borders.bottom = false;
        }
        else if (a.row < b.row) {
            a.borders.bottom = false;
            b.borders.top = false;
        }
    }
    removeRandomWalls(maze, n) {
        for (let i = 0; i < n;) {
            let r = random(1, maze.nRows - 2);
            let c = random(1, maze.nCols - 2);
            let cell = maze.cell(r, c);
            let next = random(0, 3);
            switch (next) {
                case 0:
                    if (cell.borders.top) {
                        this.removeWallsBetween(cell, maze.cell(r - 1, c));
                        console.log("remove (%d, %d) : top", c, r);
                        i++;
                    }
                    break;
                case 1:
                    if (cell.borders.right) {
                        this.removeWallsBetween(cell, maze.cell(r, c + 1));
                        console.log("remove (%d, %d) : right", c, r);
                        i++;
                    }
                    break;
                case 2:
                    if (cell.borders.bottom) {
                        this.removeWallsBetween(cell, maze.cell(r + 1, c));
                        console.log("remove (%d, %d) : bottom", c, r);
                        i++;
                    }
                    break;
                case 3:
                    if (cell.borders.left) {
                        this.removeWallsBetween(cell, maze.cell(r, c - 1));
                        console.log("remove (%d, %d) : left", c, r);
                        i++;
                    }
                    break;
                default:
                    break;
            }
        }
    }
}
class CellBorders {
    constructor() {
        this.top = true;
        this.right = true;
        this.bottom = true;
        this.left = true;
    }
}
class Cell {
    constructor(row, col) {
        this.visited = false;
        this.row = row;
        this.col = col;
        this.borders = new CellBorders();
    }
}
Cell.cellWidth = 30;
class Stair {
    constructor(row, col, up) {
        this.row = row;
        this.col = col;
        this.up = up;
    }
    static upstair(row, col) {
        return new Stair(row, col, true);
    }
    static downstairstair(row, col) {
        return new Stair(row, col, false);
    }
}
function random(min, max) {
    let rand = Math.random();
    if (typeof min === 'undefined') {
        return rand;
    }
    else if (typeof max === 'undefined') {
        return rand * min;
    }
    else if (min > max) {
        const tmp = min;
        min = max;
        max = tmp;
    }
    return Math.floor(rand * (max - min) + min);
}


/***/ }),

/***/ "./app/views.ts":
/*!**********************!*
  !*** ./app/views.ts ***!
  \**********************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "MazeView": () => /* binding */ MazeView,
/* harmony export */   "CellView": () => /* binding */ CellView,
/* harmony export */   "StairView": () => /* binding */ StairView,
/* harmony export */   "GameView": () => /* binding */ GameView,
/* harmony export */   "HeroView": () => /* binding */ HeroView
/* harmony export */ });
/* harmony import */ var _maze__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./maze */ "./app/maze.ts");

class MazeView {
    constructor(maze) {
        this.maze = maze;
    }
    draw(p) {
        for (let r = 0; r < this.maze.nRows; r++) {
            for (let c = 0; c < this.maze.nCols; c++) {
                let cell = this.maze.cell(r, c);
                if (cell.visited) {
                    let cv = new CellView(cell);
                    cv.draw(p);
                }
            }
        }
        if (this.maze.cell(this.maze.upstair.row, this.maze.upstair.col).visited) {
            let sv = new StairView(this.maze.upstair);
            sv.draw(p);
        }
        if (this.maze.cell(this.maze.downstair.row, this.maze.downstair.col).visited) {
            let sv = new StairView(this.maze.downstair);
            sv.draw(p);
        }
    }
}
class CellView {
    constructor(cell) {
        this.cell = cell;
    }
    draw(p) {
        let w = _maze__WEBPACK_IMPORTED_MODULE_0__.Cell.cellWidth;
        let x = this.cell.col * _maze__WEBPACK_IMPORTED_MODULE_0__.Cell.cellWidth;
        let y = this.cell.row * _maze__WEBPACK_IMPORTED_MODULE_0__.Cell.cellWidth;
        const bg = '#222222';
        const wallColor = '#EEEEEE';
        const doorColor = '#444444';
        p.stroke(wallColor);
        p.fill(bg);
        p.rect(x, y, w, w);
        const b = 5;
        p.stroke(doorColor);
        if (!this.cell.borders.top) {
            p.line(x + b, y, x + w - b, y);
        }
        if (!this.cell.borders.right) {
            p.line(x + w, y + b, x + w, y + w - b);
        }
        if (!this.cell.borders.bottom) {
            p.line(x + b, y + w, x + w - b, y + w);
        }
        if (!this.cell.borders.left) {
            p.line(x, y + b, x, y + w - b);
        }
    }
    highlight(p) {
        p.noStroke();
        p.fill(255, 255, 255, 255);
        let w = _maze__WEBPACK_IMPORTED_MODULE_0__.Cell.cellWidth;
        let x = this.cell.col * _maze__WEBPACK_IMPORTED_MODULE_0__.Cell.cellWidth;
        let y = this.cell.row * _maze__WEBPACK_IMPORTED_MODULE_0__.Cell.cellWidth;
        p.ellipse(x + w / 2, y + w / 2, w / 2, w / 2);
    }
}
CellView.cellWidth = 30;
class StairView {
    constructor(stair) {
        this.stair = stair;
    }
    draw(p) {
        p.stroke(255);
        if (this.stair.up) {
            p.fill(192, 192, 192);
        }
        else {
            p.fill(70, 70, 70);
        }
        let w = _maze__WEBPACK_IMPORTED_MODULE_0__.Cell.cellWidth - 6;
        let x = this.stair.col * _maze__WEBPACK_IMPORTED_MODULE_0__.Cell.cellWidth + 3;
        let y = this.stair.row * _maze__WEBPACK_IMPORTED_MODULE_0__.Cell.cellWidth + 3;
        p.rect(x, y, w, w);
    }
}
class GameView {
    constructor(game) {
        this.game = game;
    }
    draw(p) {
        p.background(0);
        let mv = new MazeView(this.game.maze());
        mv.draw(p);
        let hero = this.game.getHero();
        let hv = new HeroView(hero);
        hv.draw(p);
        document.getElementById("nLevel").innerHTML = this.game.getLevel().toString();
        document.getElementById("life").innerHTML = hero.life.toString();
    }
}
class HeroView {
    constructor(hero) {
        this.hero = hero;
    }
    draw(p) {
        p.stroke(255);
        if (this.hero.life > 0) {
            p.fill(0, 255, 0);
        }
        else {
            p.fill(80, 0, 0);
        }
        let x = this.hero.x * _maze__WEBPACK_IMPORTED_MODULE_0__.Cell.cellWidth + _maze__WEBPACK_IMPORTED_MODULE_0__.Cell.cellWidth / 2;
        let y = this.hero.y * _maze__WEBPACK_IMPORTED_MODULE_0__.Cell.cellWidth + _maze__WEBPACK_IMPORTED_MODULE_0__.Cell.cellWidth / 2;
        let r = _maze__WEBPACK_IMPORTED_MODULE_0__.Cell.cellWidth / 2 - 1;
        p.ellipse(x, y, r, r);
    }
}


/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		if(__webpack_module_cache__[moduleId]) {
/******/ 			return __webpack_module_cache__[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			// no module.id needed
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => Object.prototype.hasOwnProperty.call(obj, prop)
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/************************************************************************/
/******/ 	// startup
/******/ 	// Load entry module
/******/ 	__webpack_require__("./app/app.ts");
/******/ 	// This entry module used 'exports' so it can't be inlined
/******/ })()
;
//# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIndlYnBhY2s6Ly9tb3JpYS8uL2FwcC9hcHAudHMiLCJ3ZWJwYWNrOi8vbW9yaWEvLi9hcHAvZ2FtZS50cyIsIndlYnBhY2s6Ly9tb3JpYS8uL2FwcC9oZXJvLnRzIiwid2VicGFjazovL21vcmlhLy4vYXBwL21hemUudHMiLCJ3ZWJwYWNrOi8vbW9yaWEvLi9hcHAvdmlld3MudHMiLCJ3ZWJwYWNrOi8vbW9yaWEvd2VicGFjay9ib290c3RyYXAiLCJ3ZWJwYWNrOi8vbW9yaWEvd2VicGFjay9ydW50aW1lL2RlZmluZSBwcm9wZXJ0eSBnZXR0ZXJzIiwid2VicGFjazovL21vcmlhL3dlYnBhY2svcnVudGltZS9oYXNPd25Qcm9wZXJ0eSBzaG9ydGhhbmQiLCJ3ZWJwYWNrOi8vbW9yaWEvd2VicGFjay9ydW50aW1lL21ha2UgbmFtZXNwYWNlIG9iamVjdCIsIndlYnBhY2s6Ly9tb3JpYS93ZWJwYWNrL3N0YXJ0dXAiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6Ijs7Ozs7Ozs7Ozs7OztBQUVrQztBQUNBO0FBSWxDLElBQUksSUFBZSxDQUFDO0FBRXBCLFNBQVMsVUFBVTtJQUNmLElBQUksUUFBUSxHQUFHLFFBQVEsQ0FBQyxjQUFjLENBQUMsUUFBUSxDQUFDLENBQUM7SUFDakQsUUFBUSxDQUFDLFNBQVMsR0FBRyxJQUFJLENBQUMsUUFBUSxFQUFFLENBQUMsUUFBUSxFQUFFLENBQUM7QUFDcEQsQ0FBQztBQUVELElBQUksTUFBTSxHQUFHLFVBQVUsQ0FBTTtJQUN6QixDQUFDLENBQUMsS0FBSyxHQUFHO1FBQ04sSUFBSSxHQUFHLElBQUksNENBQVMsQ0FBQyxDQUFDLEVBQUUsRUFBRSxFQUFFLENBQUMsQ0FBQyxDQUFDO1FBQy9CLElBQUksTUFBTSxHQUFHLENBQUMsQ0FBQyxZQUFZLENBQUMsSUFBSSxDQUFDLEtBQUssRUFBRSxJQUFJLENBQUMsTUFBTSxDQUFDLENBQUM7UUFDckQsTUFBTSxDQUFDLE1BQU0sQ0FBQyxNQUFNLENBQUMsQ0FBQztRQUN0QixD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dkMsSUFBSSxDQUFDLEdBQUcsSUFBSSxDQUFDLElBQUksQ0FBQyxHQUFHLEdBQUcsaURBQWMsQ0FBQztRQUN2QyxNQUFNLEVBQUUsR0FBRyxTQUFTLENBQUM7UUFDckIsTUFBTSxTQUFTLEdBQUcsU0FBUyxDQUFDO1FBQzVCLE1BQU0sU0FBUyxHQUFHLFNBQVMsQ0FBQztRQUc1QixDQUFDLENBQUMsTUFBTSxDQUFDLFNBQVMsQ0FBQyxDQUFDO1FBQ3BCLENBQUMsQ0FBQyxJQUFJLENBQUMsRUFBRSxDQUFDLENBQUM7UUFDWCxDQUFDLENBQUMsSUFBSSxDQUFDLENBQUMsRUFBRSxDQUFDLEVBQUUsQ0FBQyxFQUFFLENBQUMsQ0FBQztRQUdsQixNQUFNLENBQUMsR0FBRyxDQUFDLENBQUM7UUFDWixDQUFDLENBQUMsTUFBTSxDQUFDLFNBQVMsQ0FBQyxDQUFDO1FBQ3BCLElBQUksQ0FBQyxJQUFJLENBQUMsSUFBSSxDQUFDLE9BQU8sQ0FBQyxHQUFHLEVBQUU7WUFDeEIsQ0FBQyxDQUFDLElBQUksQ0FBQyxDQUFDLEdBQUcsQ0FBQyxFQUFFLENBQUMsRUFBRSxDQUFDLEdBQUcsQ0FBQyxHQUFHLENBQUMsRUFBRSxDQUFDLENBQUMsQ0FBQztTQUNsQztRQUNELElBQUksQ0FBQyxJQUFJLENBQUMsSUFBSSxDQUFDLE9BQU8sQ0FBQyxLQUFLLEVBQUU7WUFDMUIsQ0FBQyxDQUFDLElBQUksQ0FBQyxDQUFDLEdBQUcsQ0FBQyxFQUFFLENBQUMsR0FBRyxDQUFDLEVBQUUsQ0FBQyxHQUFHLENBQUMsRUFBRSxDQUFDLEdBQUcsQ0FBQyxHQUFHLENBQUMsQ0FBQyxDQUFDO1NBQzFDO1FBQ0QsSUFBSSxDQUFDLElBQUksQ0FBQyxJQUFJLENBQUMsT0FBTyxDQUFDLE1BQU0sRUFBRTtZQUMzQixDQUFDLENBQUMsSUFBSSxDQUFDLENBQUMsR0FBRyxDQUFDLEVBQUUsQ0FBQyxHQUFHLENBQUMsRUFBRSxDQUFDLEdBQUcsQ0FBQyxHQUFHLENBQUMsRUFBRSxDQUFDLEdBQUcsQ0FBQyxDQUFDLENBQUM7U0FDMUM7UUFD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VUFDQTtVQUNBIiwiZmlsZSI6ImJ1bmRsZS5qcyIsInNvdXJjZXNDb250ZW50IjpbIi8vLyA8cmVmZXJlbmNlIHBhdGg9XCIuLi90eXBpbmdzL3A1LmQudHNcIiAvPlxyXG5cclxuaW1wb3J0IHsgTW9yaWFHYW1lIH0gZnJvbSBcIi4vZ2FtZVwiXHJcbmltcG9ydCB7IEdhbWVWaWV3IH0gZnJvbSBcIi4vdmlld3NcIlxyXG5pbXBvcnQgeyBEaXJlY3Rpb24gfSBmcm9tIFwiLi9oZXJvXCJcclxuXHJcblxyXG5sZXQgZ2FtZTogTW9yaWFHYW1lO1xyXG5cclxuZnVuY3Rpb24gdXBkYXRlSW5mbygpIHtcclxuICAgIGxldCBsZXZlbEVsdCA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKFwibkxldmVsXCIpO1xyXG4gICAgbGV2ZWxFbHQuaW5uZXJIVE1MID0gZ2FtZS5nZXRMZXZlbCgpLnRvU3RyaW5nKCk7XHJcbn1cclxuXHJcbmxldCBza2V0Y2ggPSBmdW5jdGlvbiAocDogYW55KSB7XHJcbiAgICBwLnNldHVwID0gZnVuY3Rpb24gKCkge1xyXG4gICAgICAgIGdhbWUgPSBuZXcgTW9yaWFHYW1lKDgsIDEwLCA1KTtcclxuICAgICAgICBsZXQgY2FudmFzID0gcC5jcmVhdGVDYW52YXMoZ2FtZS53aWR0aCwgZ2FtZS5oZWlnaHQpO1xyXG4gICAgICAgIGNhbnZhcy5wYXJlbnQoJ2dhbWUnKTtcclxuICAgICAgICBwLmZyYW1lUmF0ZSgxMCk7XHJcbiAgICB9O1xyXG5cclxuICAgIHAuZHJhdyA9IGZ1bmN0aW9uICgpIHtcclxuICAgICAgICBwLmJhY2tncm91bmQoMCk7XHJcbiAgICAgICAgbGV0IHZpZXcgPSBuZXcgR2FtZVZpZXcoZ2FtZSk7XHJcbiAgICAgICAgdmlldy5kcmF3KHApO1xyXG4gICAgICAgIHVwZGF0ZUluZm8oKTtcclxuICAgIH07XHJcblxyXG4gICAgcC5rZXlQcmVzc2VkID0gZnVuY3Rpb24gKCkge1xyXG4gICAgICAgIGlmIChnYW1lLmdldEhlcm8oKS5saWZlIDw9IDApIHtcclxuICAgICAgICAgICAgcmV0dXJuO1xyXG4gICAgICAgIH1cclxuICAgICAgICBpZiAocC5rZXlDb2RlID09PSBwLlVQX0FSUk9XKSB7XHJcbiAgICAgICAgICAgIGdhbWUubW92ZUhlcm8oRGlyZWN0aW9uLlVQKTtcclxuICAgICAgICB9IGVsc2UgaWYgKHAua2V5Q29kZSA9PT0gcC5ET1dOX0FSUk9XKSB7XHJcbiAgICAgICAgICAgIGdhbWUubW92ZUhlcm8oRGlyZWN0aW9uLkRPV04pO1xyXG4gICAgICAgIH0gZWxzZSBpZiAocC5rZXlDb2RlID09PSBwLkxFRlRfQVJST1cpIHtcclxuICAgICAgICAgICAgZ2FtZS5tb3ZlSGVybyhEaXJlY3Rpb24uTEVGVCk7XHJcbiAgICAgICAgfSBlbHNlIGlmIChwLmtleUNvZGUgPT09IHAuUklHSFRfQVJST1cpIHtcclxuICAgICAgICAgICAgZ2FtZS5tb3ZlSGVybyhEaXJlY3Rpb24uUklHSFQpO1xyXG4gICAgICAgIH1cclxuICAgIH1cclxufTtcclxuXHJcbmxldCBteXA1ID0gbmV3IHA1KHNrZXRjaCk7IiwiaW1wb3J0IHsgSGVybywgRGlyZWN0aW9uIH0gZnJvbSBcIi4vaGVyb1wiXHJcbmltcG9ydCB7IE1hemUsIE1hemVHZW5lcmF0b3IsIENlbGwgfSBmcm9tIFwiLi9tYXplXCJcclxuXHJcbi8qKlxyXG4gKiBNb3JpYUdhbWVcclxuICovXHJcbmV4cG9ydCBjbGFzcyBNb3JpYUdhbWUge1xyXG4gICAgcHVibGljIHJlYWRvbmx5IG5Sb3dzOiBudW1iZXI7XHJcbiAgICBwdWJsaWMgcmVhZG9ubHkgbkNvbHM6IG51bWJlcjtcclxuICAgIHB1YmxpYyByZWFkb25seSB3aWR0aDogbnVtYmVyO1xyXG4gICAgcHVibGljIHJlYWRvbmx5IGhlaWdodDogbnVtYmVyO1xyXG5cclxuICAgIHByaXZhdGUgaGVybzogSGVybztcclxuICAgIHByaXZhdGUgbWF6ZXM6IE1hemVbXTtcclxuICAgIHByaXZhdGUgY3VycmVudExldmVsOiBudW1iZXI7XHJcblxyXG4gICAgY29uc3RydWN0b3IoblJvd3M6IG51bWJlciwgbkNvbHM6IG51bWJlciwgbkxldmVsczogbnVtYmVyKSB7XHJcbiAgICAgICAgdGhpcy5uUm93cyA9IG5Sb3dzO1xyXG4gICAgICAgIHRoaXMubkNvbHMgPSBuQ29scztcclxuXHJcbiAgICAgICAgbGV0IG1hemVHZW5lcmF0b3IgPSBuZXcgTWF6ZUdlbmVyYXRvcigpO1xyXG4gICAgICAgIHRoaXMubWF6ZXMgPSBbXTtcclxuICAgICAgICBmb3IgKGxldCBpID0gMDsgaSA8IG5MZXZlbHM7IGkrKykge1xyXG4gICAgICAgICAgICB0aGlzLm1hemVzLnB1c2gobWF6ZUdlbmVyYXRvci5uZXdNYXplKHRoaXMublJvd3MsIHRoaXMubkNvbHMpKTtcclxuICAgICAgICB9XHJcblxyXG4gICAgICAgIHRoaXMuY3VycmVudExldmVsID0gMDtcclxuXHJcbiAgICAgICAgbGV0IG1hemUgPSB0aGlzLm1hemUoKTtcclxuICAgICAgICB0aGlzLmhlcm8gPSBuZXcgSGVybyhtYXplLnVwc3RhaXIuY29sLCBtYXplLnVwc3RhaXIucm93KTtcclxuXHJcbiAgICAgICAgdGhpcy53aWR0aCA9IG1hemUud2lkdGg7XHJcbiAgICAgICAgdGhpcy5oZWlnaHQgPSBtYXplLmhlaWdodDtcclxuXHJcbiAgICAgICAgdGhpcy5pbml0TGV2ZWwoKTtcclxuICAgIH1cclxuXHJcbiAgICBwdWJsaWMgZ2V0TGV2ZWwoKTogbnVtYmVyIHtcclxuICAgICAgICByZXR1cm4gdGhpcy5jdXJyZW50TGV2ZWw7XHJcbiAgICB9XHJcblxyXG4gICAgcHJpdmF0ZSBpbml0TGV2ZWwoKSB7XHJcbiAgICAgICAgbGV0IG1hemUgPSB0aGlzLm1hemUoKTtcclxuICAgICAgICB0aGlzLmhlcm8ubW92ZVRvKG1hemUudXBzdGFpci5jb2wsIG1hemUudXBzdGFpci5yb3cpO1xyXG4gICAgICAgIG1hemUuY2VsbCh0aGlzLmhlcm8ueSwgdGhpcy5oZXJvLngpLnZpc2l0ZWQgPSB0cnVlO1xyXG4gICAgICAgIHRoaXMuY2hlY2tWaXNpYmlsaXR5KCk7XHJcbiAgICB9XHJcblxyXG4gICAgcHVibGljIG1hemUoKTogTWF6ZSB7XHJcbiAgICAgICAgcmV0dXJuIHRoaXMubWF6ZXNbdGhpcy5jdXJyZW50TGV2ZWxdO1xyXG4gICAgfVxyXG5cclxuICAgIHB1YmxpYyBtb3ZlSGVybyhkaXJlY3Rpb246IERpcmVjdGlvbikge1xyXG4gICAgICAgIGlmICh0aGlzLmNhbk1vdmUoZGlyZWN0aW9uKSkge1xyXG4gICAgICAgICAgICB0aGlzLmhlcm8ubW92ZShkaXJlY3Rpb24pO1xyXG4gICAgICAgICAgICB0aGlzLm1hemUoKS5jZWxsKHRoaXMuaGVyby55LCB0aGlzLmhlcm8ueCkudmlzaXRlZCA9IHRydWU7XHJcbiAgICAgICAgICAgIGlmICh0aGlzLmhlcm8ueCA9PT0gdGhpcy5tYXplKCkuZG93bnN0YWlyLmNvbCAmJiB0aGlzLmhlcm8ueSA9PT0gdGhpcy5tYXplKCkuZG93bnN0YWlyLnJvdykge1xyXG4gICAgICAgICAgICAgICAgdGhpcy5jdXJyZW50TGV2ZWwrKztcclxuICAgICAgICAgICAgICAgIHRoaXMuaW5pdExldmVsKCk7XHJcbiAgICAgICAgICAgIH1cclxuICAgICAgICAgICAgdGhpcy5jaGVja1Zpc2liaWxpdHkoKTtcclxuICAgICAgICB9XHJcbiAgICB9XHJcblxyXG4gICAgcHVibGljIGNhbk1vdmUoZGlyZWN0aW9uOiBEaXJlY3Rpb24pOiBib29sZWFuIHtcclxuICAgICAgICBsZXQgY2VsbEJvcmRlcnMgPSB0aGlzLm1hemUoKS5jZWxsKHRoaXMuaGVyby55LCB0aGlzLmhlcm8ueCkuYm9yZGVycztcclxuICAgICAgICByZXR1cm4gKGRpcmVjdGlvbiA9PT0gRGlyZWN0aW9uLlJJR0hUICYmICFjZWxsQm9yZGVycy5yaWdodClcclxuICAgICAgICAgICAgfHwgKGRpcmVjdGlvbiA9PT0gRGlyZWN0aW9uLkxFRlQgJiYgIWNlbGxCb3JkZXJzLmxlZnQpXHJcbiAgICAgICAgICAgIHx8IChkaXJlY3Rpb24gPT09IERpcmVjdGlvbi5VUCAmJiAhY2VsbEJvcmRlcnMudG9wKVxyXG4gICAgICAgICAgICB8fCAoZGlyZWN0aW9uID09PSBEaXJlY3Rpb24uRE9XTiAmJiAhY2VsbEJvcmRlcnMuYm90dG9tKTtcclxuICAgIH1cclxuXHJcbiAgICBwcml2YXRlIGNoZWNrVmlzaWJpbGl0eSgpIHtcclxuICAgICAgICBsZXQgeDogbnVtYmVyO1xyXG4gICAgICAgIGxldCB5OiBudW1iZXI7XHJcbiAgICAgICAgbGV0IGNlbGw6IENlbGw7XHJcbiAgICAgICAgbGV0IG1hemUgPSB0aGlzLm1hemUoKTtcclxuICAgICAgICBsZXQgcmVzZXQgPSAoKSA9PiB7XHJcbiAgICAgICAgICAgIHggPSB0aGlzLmhlcm8ueDtcclxuICAgICAgICAgICAgeSA9IHRoaXMuaGVyby55O1xyXG4gICAgICAgICAgICBjZWxsID0gbWF6ZS5jZWxsKHksIHgpO1xyXG4gICAgICAgIH1cclxuICAgICAgICBsZXQgbmV4dCA9ICgpID0+IHtcclxuICAgICAgICAgICAgY2VsbCA9IG1hemUuY2VsbCh5LCB4KTtcclxuICAgICAgICAgICAgY2VsbC52aXNpdGVkID0gdHJ1ZTtcclxuICAgICAgICB9XHJcbiAgICAgICAgcmVzZXQoKTtcclxuICAgICAgICB3aGlsZSAoIWNlbGwuYm9yZGVycy50b3ApIHtcclxuICAgICAgICAgICAgeSAtPSAxO1xyXG4gICAgICAgICAgICBuZXh0KCk7XHJcbiAgICAgICAgfVxyXG4gICAgICAgIHJlc2V0KCk7XHJcbiAgICAgICAgd2hpbGUgKCFjZWxsLmJvcmRlcnMucmlnaHQpIHtcclxuICAgICAgICAgICAgeCArPSAxO1xyXG4gICAgICAgICAgICBuZXh0KCk7XHJcbiAgICAgICAgfVxyXG4gICAgICAgIHJlc2V0KCk7XHJcbiAgICAgICAgd2hpbGUgKCFjZWxsLmJvcmRlcnMuYm90dG9tKSB7XHJcbiAgICAgICAgICAgIHkgKz0gMTtcclxuICAgICAgICAgICAgbmV4dCgpO1xyXG4gICAgICAgIH1cclxuICAgICAgICByZXNldCgpO1xyXG4gICAgICAgIHdoaWxlICghY2VsbC5ib3JkZXJzLmxlZnQpIHtcclxuICAgICAgICAgICAgeCAtPSAxO1xyXG4gICAgICAgICAgICBuZXh0KCk7XHJcbiAgICAgICAgfVxyXG4gICAgfVxyXG5cclxuICAgIHB1YmxpYyBnZXRIZXJvKCkge1xyXG4gICAgICAgIHJldHVybiB0aGlzLmhlcm87XHJcbiAgICB9XHJcbn0iLCJleHBvcnQgY29uc3QgZW51bSBEaXJlY3Rpb24ge1xyXG4gICAgVVAsIERPV04sIExFRlQsIFJJR0hUXHJcbn1cclxuXHJcbi8qKlxyXG4gKiBIZXJvXHJcbiAqL1xyXG5leHBvcnQgY2xhc3MgSGVybyB7XHJcbiAgICBwcml2YXRlIF94OiBudW1iZXI7XHJcbiAgICBwcml2YXRlIF95OiBudW1iZXI7XHJcbiAgICBwcml2YXRlIF9saWZlOiBudW1iZXI7XHJcblxyXG4gICAgY29uc3RydWN0b3IoeDogbnVtYmVyLCB5OiBudW1iZXIpIHtcclxuICAgICAgICB0aGlzLl94ID0geDtcclxuICAgICAgICB0aGlzLl95ID0geTtcclxuICAgICAgICB0aGlzLl9saWZlID0gMTAwO1xyXG4gICAgfVxyXG5cclxuICAgIHB1YmxpYyBnZXQgeCgpOiBudW1iZXIge1xyXG4gICAgICAgIHJldHVybiB0aGlzLl94O1xyXG4gICAgfVxyXG5cclxuICAgIHB1YmxpYyBnZXQgeSgpOiBudW1iZXIge1xyXG4gICAgICAgIHJldHVybiB0aGlzLl95O1xyXG4gICAgfVxyXG5cclxuICAgIHB1YmxpYyBnZXQgbGlmZSgpIHtcclxuICAgICAgICByZXR1cm4gdGhpcy5fbGlmZTtcclxuICAgIH1cclxuXHJcbiAgICBwdWJsaWMgbW92ZVRvKHg6IG51bWJlciwgeTogbnVtYmVyKSB7XHJcbiAgICAgICAgdGhpcy5feCA9IHg7XHJcbiAgICAgICAgdGhpcy5feSA9IHk7XHJcbiAgICB9XHJcblxyXG4gICAgcHVibGljIG1vdmUoZGlyOiBEaXJlY3Rpb24pIHtcclxuICAgICAgICBsZXQgZHggPSAwLCBkeSA9IDA7XHJcbiAgICAgICAgc3dpdGNoIChkaXIpIHtcclxuICAgICAgICAgICAgY2FzZSBEaXJlY3Rpb24uVVA6IGR5ID0gLTE7IGJyZWFrO1xyXG4gICAgICAgICAgICBjYXNlIERpcmVjdGlvbi5ET1dOOiBkeSA9IDE7IGJyZWFrO1xyXG4gICAgICAgICAgICBjYXNlIERpcmVjdGlvbi5MRUZUOiBkeCA9IC0xOyBicmVhaztcclxuICAgICAgICAgICAgY2FzZSBEaXJlY3Rpb24uUklHSFQ6IGR4ID0gMTsgYnJlYWs7XHJcbiAgICAgICAgICAgIGRlZmF1bHQ6IGJyZWFrO1xyXG4gICAgICAgIH1cclxuICAgICAgICB0aGlzLl94ICs9IGR4O1xyXG4gICAgICAgIHRoaXMuX3kgKz0gZHk7XHJcbiAgICAgICAgdGhpcy5fbGlmZS0tO1xyXG4gICAgfVxyXG59XHJcbiIsIi8qKlxyXG4gKiBAY2xhc3MgTWF6ZVxyXG4gKi9cclxuZXhwb3J0IGNsYXNzIE1hemUge1xyXG4gICAgcHVibGljIHJlYWRvbmx5IG5Sb3dzOiBudW1iZXI7XHJcbiAgICBwdWJsaWMgcmVhZG9ubHkgbkNvbHM6IG51bWJlcjtcclxuICAgIHB1YmxpYyByZWFkb25seSB3aWR0aDogbnVtYmVyO1xyXG4gICAgcHVibGljIHJlYWRvbmx5IGhlaWdodDogbnVtYmVyO1xyXG4gICAgcHJpdmF0ZSBncmlkOiBDZWxsW11bXTtcclxuICAgIHB1YmxpYyByZWFkb25seSB1cHN0YWlyOiBTdGFpcjtcclxuICAgIHB1YmxpYyByZWFkb25seSBkb3duc3RhaXI6IFN0YWlyO1xyXG5cclxuICAgIGNvbnN0cnVjdG9yKG5Sb3dzOiBudW1iZXIsIG5Db2xzOiBudW1iZXIpIHtcclxuICAgICAgICB0aGlzLm5Sb3dzID0gblJvd3M7XHJcbiAgICAgICAgdGhpcy5uQ29scyA9IG5Db2xzO1xyXG4gICAgICAgIHRoaXMuaGVpZ2h0ID0gblJvd3MgKiBDZWxsLmNlbGxXaWR0aCArIDE7XHJcbiAgICAgICAgdGhpcy53aWR0aCA9IG5Db2xzICogQ2VsbC5jZWxsV2lkdGggKyAxO1xyXG4gICAgICAgIHRoaXMuZ3JpZCA9IFtdO1xyXG5cclxuICAgICAgICBmb3IgKHZhciByID0gMDsgciA8IHRoaXMublJvd3M7IHIrKykge1xyXG4gICAgICAgICAgICB0aGlzLmdyaWRbcl0gPSBbXTtcclxuICAgICAgICAgICAgZm9yICh2YXIgYyA9IDA7IGMgPCB0aGlzLm5Db2xzOyBjKyspIHtcclxuICAgICAgICAgICAgICAgIHRoaXMuZ3JpZFtyXVtjXSA9IG5ldyBDZWxsKHIsIGMpO1xyXG4gICAgICAgICAgICB9XHJcbiAgICAgICAgfVxyXG4gICAgICAgIHRoaXMudXBzdGFpciA9IFN0YWlyLnVwc3RhaXIoMCwgMCk7XHJcbiAgICAgICAgdGhpcy5kb3duc3RhaXIgPSBTdGFpci5kb3duc3RhaXJzdGFpcihuUm93cyAtIDEsIG5Db2xzIC0gMSk7XHJcbiAgICB9XHJcblxyXG4gICAgcHVibGljIGNlbGwocm93OiBudW1iZXIsIGNvbDogbnVtYmVyKSB7XHJcbiAgICAgICAgcmV0dXJuIHRoaXMuZ3JpZFtyb3ddW2NvbF07XHJcbiAgICB9XHJcbn1cclxuXHJcblxyXG4vKipcclxuICogQGNsYXNzIE1hemVHZW5lcmF0b3JcclxuICovXHJcbmV4cG9ydCBjbGFzcyBNYXplR2VuZXJhdG9yIHtcclxuXHJcbiAgICBwdWJsaWMgbmV3TWF6ZShuUm93czogbnVtYmVyLCBuQ29sczogbnVtYmVyKTogTWF6ZSB7XHJcbiAgICAgICAgbGV0IG1hemUgPSBuZXcgTWF6ZShuUm93cywgbkNvbHMpO1xyXG4gICAgICAgIGxldCBiYWNrdHJhY2tpbmc6IENlbGxbXSA9IFtdO1xyXG4gICAgICAgIGxldCBjdXJyZW50Q2VsbCA9IG1hemUuY2VsbCgwLCAwKTtcclxuICAgICAgICBjdXJyZW50Q2VsbC52aXNpdGVkID0gdHJ1ZTtcclxuICAgICAgICBsZXQgZmluaXNoZWQgPSBmYWxzZTtcclxuICAgICAgICB3aGlsZSAoIWZpbmlzaGVkKSB7XHJcbiAgICAgICAgICAgIGxldCBuZXh0ID0gdGhpcy5nZXROZXh0TmVpZ2hib3IobWF6ZSwgY3VycmVudENlbGwpO1xyXG4gICAgICAgICAgICBpZiAobmV4dCkge1xyXG4gICAgICAgICAgICAgICAgbmV4dC52aXNpdGVkID0gdHJ1ZTtcclxuICAgICAgICAgICAgICAgIGJhY2t0cmFja2luZy5wdXNoKGN1cnJlbnRDZWxsKTtcclxuICAgICAgICAgICAgICAgIHRoaXMucmVtb3ZlV2FsbHNCZXR3ZWVuKGN1cnJlbnRDZWxsLCBuZXh0KTtcclxuICAgICAgICAgICAgICAgIGN1cnJlbnRDZWxsID0gbmV4dDtcclxuICAgICAgICAgICAgfSBlbHNlIGlmIChiYWNrdHJhY2tpbmcubGVuZ3RoID4gMCkge1xyXG4gICAgICAgICAgICAgICAgbmV4dCA9IGJhY2t0cmFja2luZy5wb3AoKTtcclxuICAgICAgICAgICAgICAgIGN1cnJlbnRDZWxsID0gbmV4dDtcclxuICAgICAgICAgICAgfSBlbHNlIHtcclxuICAgICAgICAgICAgICAgIGNvbnNvbGUubG9nKFwiRklOSVNIXCIpO1xyXG4gICAgICAgICAgICAgICAgZmluaXNoZWQgPSB0cnVlO1xyXG4gICAgICAgICAgICB9XHJcbiAgICAgICAgfVxyXG5cclxuICAgICAgICBmb3IgKHZhciByID0gMDsgciA8IG5Sb3dzOyByKyspIHtcclxuICAgICAgICAgICAgZm9yICh2YXIgYyA9IDA7IGMgPCBuQ29sczsgYysrKSB7XHJcbiAgICAgICAgICAgICAgICBtYXplLmNlbGwociwgYykudmlzaXRlZCA9IGZhbHNlO1xyXG4gICAgICAgICAgICB9XHJcbiAgICAgICAgfVxyXG5cclxuICAgICAgICB0aGlzLnJlbW92ZVJhbmRvbVdhbGxzKG1hemUsIDEwKTtcclxuXHJcbiAgICAgICAgcmV0dXJuIG1hemU7XHJcbiAgICB9XHJcblxyXG4gICAgcHJpdmF0ZSBnZXROZXh0TmVpZ2hib3IobWF6ZTogTWF6ZSwgY2VsbDogQ2VsbCk6IENlbGwge1xyXG4gICAgICAgIGxldCBuZWlnaGJvcnM6IENlbGxbXSA9IFtdO1xyXG4gICAgICAgIGlmIChjZWxsLnJvdyA+IDApIHtcclxuICAgICAgICAgICAgbGV0IGxlZnQgPSBtYXplLmNlbGwoY2VsbC5yb3cgLSAxLCBjZWxsLmNvbCk7XHJcbiAgICAgICAgICAgIGlmICghbGVmdC52aXNpdGVkKSB7XHJcbiAgICAgICAgICAgICAgICBuZWlnaGJvcnMucHVzaChsZWZ0KTtcclxuICAgICAgICAgICAgfVxyXG4gICAgICAgIH1cclxuICAgICAgICBpZiAoY2VsbC5yb3cgPCBtYXplLm5Sb3dzIC0gMSkge1xyXG4gICAgICAgICAgICBsZXQgcmlnaHQgPSBtYXplLmNlbGwoY2VsbC5yb3cgKyAxLCBjZWxsLmNvbCk7XHJcbiAgICAgICAgICAgIGlmICghcmlnaHQudmlzaXRlZCkge1xyXG4gICAgICAgICAgICAgICAgbmVpZ2hib3JzLnB1c2gocmlnaHQpO1xyXG4gICAgICAgICAgICB9XHJcbiAgICAgICAgfVxyXG4gICAgICAgIGlmIChjZWxsLmNvbCA+IDApIHtcclxuICAgICAgICAgICAgbGV0IHRvcCA9IG1hemUuY2VsbChjZWxsLnJvdywgY2VsbC5jb2wgLSAxKTtcclxuICAgICAgICAgICAgaWYgKCF0b3AudmlzaXRlZCkge1xyXG4gICAgICAgICAgICAgICAgbmVpZ2hib3JzLnB1c2godG9wKTtcclxuICAgICAgICAgICAgfVxyXG4gICAgICAgIH1cclxuICAgICAgICBpZiAoY2VsbC5jb2wgPCBtYXplLm5Db2xzIC0gMSkge1xyXG4gICAgICAgICAgICBsZXQgYm90dG9tID0gbWF6ZS5jZWxsKGNlbGwucm93LCBjZWxsLmNvbCArIDEpO1xyXG4gICAgICAgICAgICBpZiAoIWJvdHRvbS52aXNpdGVkKSB7XHJcbiAgICAgICAgICAgICAgICBuZWlnaGJvcnMucHVzaChib3R0b20pO1xyXG4gICAgICAgICAgICB9XHJcbiAgICAgICAgfVxyXG5cclxuICAgICAgICBsZXQgbmV4dDogQ2VsbCA9IHVuZGVmaW5lZDtcclxuICAgICAgICBpZiAobmVpZ2hib3JzLmxlbmd0aCA+IDApIHtcclxuICAgICAgICAgICAgdmFyIHIgPSByYW5kb20oMCwgbmVpZ2hib3JzLmxlbmd0aCk7XHJcbiAgICAgICAgICAgIG5leHQgPSBuZWlnaGJvcnNbcl07XHJcbiAgICAgICAgfVxyXG4gICAgICAgIGlmIChuZXh0KVxyXG4gICAgICAgICAgICBjb25zb2xlLmxvZyhgZ2V0TmV4dE5laWdoYm9yKG1hemUsIHske2NlbGwucm93fSwgJHtjZWxsLmNvbH19KSA9PiAoJHtuZXh0LnJvd30sICR7bmV4dC5jb2x9KWApO1xyXG4gICAgICAgIGVsc2VcclxuICAgICAgICAgICAgY29uc29sZS5sb2coYGdldE5leHROZWlnaGJvcihtYXplLCB7JHtjZWxsLnJvd30sICR7Y2VsbC5jb2x9fSkgPT4gdW5kZWZpbmVkYCk7XHJcbiAgICAgICAgcmV0dXJuIG5leHQ7XHJcbiAgICB9XHJcblxyXG4gICAgcHJpdmF0ZSByZW1vdmVXYWxsc0JldHdlZW4oYTogQ2VsbCwgYjogQ2VsbCkge1xyXG4gICAgICAgIGlmIChhLmNvbCA+IGIuY29sKSB7XHJcbiAgICAgICAgICAgIGEuYm9yZGVycy5sZWZ0ID0gZmFsc2U7XHJcbiAgICAgICAgICAgIGIuYm9yZGVycy5yaWdodCA9IGZhbHNlO1xyXG4gICAgICAgIH0gZWxzZSBpZiAoYS5jb2wgPCBiLmNvbCkge1xyXG4gICAgICAgICAgICBhLmJvcmRlcnMucmlnaHQgPSBmYWxzZTtcclxuICAgICAgICAgICAgYi5ib3JkZXJzLmxlZnQgPSBmYWxzZTtcclxuICAgICAgICB9IGVsc2UgaWYgKGEucm93ID4gYi5yb3cpIHtcclxuICAgICAgICAgICAgYS5ib3JkZXJzLnRvcCA9IGZhbHNlO1xyXG4gICAgICAgICAgICBiLmJvcmRlcnMuYm90dG9tID0gZmFsc2U7XHJcbiAgICAgICAgfSBlbHNlIGlmIChhLnJvdyA8IGIucm93KSB7XHJcbiAgICAgICAgICAgIGEuYm9yZGVycy5ib3R0b20gPSBmYWxzZTtcclxuICAgICAgICAgICAgYi5ib3JkZXJzLnRvcCA9IGZhbHNlO1xyXG4gICAgICAgIH1cclxuICAgIH1cclxuXHJcbiAgICBwcml2YXRlIHJlbW92ZVJhbmRvbVdhbGxzKG1hemU6IE1hemUsIG46IG51bWJlcikge1xyXG4gICAgICAgIGZvciAobGV0IGkgPSAwOyBpIDwgbjspIHtcclxuICAgICAgICAgICAgbGV0IHIgPSByYW5kb20oMSwgbWF6ZS5uUm93cyAtIDIpO1xyXG4gICAgICAgICAgICBsZXQgYyA9IHJhbmRvbSgxLCBtYXplLm5Db2xzIC0gMik7XHJcblxyXG4gICAgICAgICAgICBsZXQgY2VsbCA9IG1hemUuY2VsbChyLCBjKTtcclxuICAgICAgICAgICAgbGV0IG5leHQgPSByYW5kb20oMCwgMyk7XHJcbiAgICAgICAgICAgIHN3aXRjaCAobmV4dCkge1xyXG4gICAgICAgICAgICAgICAgY2FzZSAwOlxyXG4gICAgICAgICAgICAgICAgICAgIGlmIChjZWxsLmJvcmRlcnMudG9wKSB7XHJcbiAgICAgICAgICAgICAgICAgICAgICAgIHRoaXMucmVtb3ZlV2FsbHNCZXR3ZWVuKGNlbGwsIG1hemUuY2VsbChyIC0gMSwgYykpO1xyXG4gICAgICAgICAgICAgICAgICAgICAgICBjb25zb2xlLmxvZyhcInJlbW92ZSAoJWQsICVkKSA6IHRvcFwiLCBjLCByKTtcclxuICAgICAgICAgICAgICAgICAgICAgICAgaSsrO1xyXG4gICAgICAgICAgICAgICAgICAgIH1cclxuICAgICAgICAgICAgICAgICAgICBicmVhaztcclxuXHJcbiAgICAgICAgICAgICAgICBjYXNlIDE6XHJcbiAgICAgICAgICAgICAgICAgICAgaWYgKGNlbGwuYm9yZGVycy5yaWdodCkge1xyXG4gICAgICAgICAgICAgICAgICAgICAgICB0aGlzLnJlbW92ZVdhbGxzQmV0d2VlbihjZWxsLCBtYXplLmNlbGwociwgYyArIDEpKTtcclxuICAgICAgICAgICAgICAgICAgICAgICAgY29uc29sZS5sb2coXCJyZW1vdmUgKCVkLCAlZCkgOiByaWdodFwiLCBjLCByKTtcclxuICAgICAgICAgICAgICAgICAgICAgICAgaSsrO1xyXG4gICAgICAgICAgICAgICAgICAgIH1cclxuICAgICAgICAgICAgICAgICAgICBicmVhaztcclxuXHJcbiAgICAgICAgICAgICAgICBjYXNlIDI6XHJcbiAgICAgICAgICAgICAgICAgICAgaWYgKGNlbGwuYm9yZGVycy5ib3R0b20pIHtcclxuICAgICAgICAgICAgICAgICAgICAgICAgdGhpcy5yZW1vdmVXYWxsc0JldHdlZW4oY2VsbCwgbWF6ZS5jZWxsKHIgKyAxLCBjKSk7XHJcbiAgICAgICAgICAgICAgICAgICAgICAgIGNvbnNvbGUubG9nKFwicmVtb3ZlICglZCwgJWQpIDogYm90dG9tXCIsIGMsIHIpO1xyXG4gICAgICAgICAgICAgICAgICAgICAgICBpKys7XHJcbiAgICAgICAgICAgICAgICAgICAgfVxyXG4gICAgICAgICAgICAgICAgICAgIGJyZWFrO1xyXG5cclxuICAgICAgICAgICAgICAgIGNhc2UgMzpcclxuICAgICAgICAgICAgICAgICAgICBpZiAoY2VsbC5ib3JkZXJzLmxlZnQpIHtcclxuICAgICAgICAgICAgICAgICAgICAgICAgdGhpcy5yZW1vdmVXYWxsc0JldHdlZW4oY2VsbCwgbWF6ZS5jZWxsKHIsIGMgLSAxKSk7XHJcbiAgICAgICAgICAgICAgICAgICAgICAgIGNvbnNvbGUubG9nKFwicmVtb3ZlICglZCwgJWQpIDogbGVmdFwiLCBjLCByKTtcclxuICAgICAgICAgICAgICAgICAgICAgICAgaSsrO1xyXG4gICAgICAgICAgICAgICAgICAgIH1cclxuICAgICAgICAgICAgICAgICAgICBicmVhaztcclxuXHJcbiAgICAgICAgICAgICAgICBkZWZhdWx0OlxyXG4gICAgICAgICAgICAgICAgICAgIGJyZWFrO1xyXG4gICAgICAgICAgICB9XHJcbiAgICAgICAgfVxyXG4gICAgfVxyXG59XHJcblxyXG4vKipcclxuICogQGNsYXNzIENlbGxCb3JkZXJzXHJcbiAqL1xyXG5leHBvcnQgY2xhc3MgQ2VsbEJvcmRlcnMge1xyXG4gICAgdG9wOiBib29sZWFuID0gdHJ1ZTtcclxuICAgIHJpZ2h0OiBib29sZWFuID0gdHJ1ZTtcclxuICAgIGJvdHRvbTogYm9vbGVhbiA9IHRydWU7XHJcbiAgICBsZWZ0OiBib29sZWFuID0gdHJ1ZTtcclxufVxyXG5cclxuLyoqXHJcbiAqIEBjbGFzcyBDZWxsXHJcbiAqL1xyXG5leHBvcnQgY2xhc3MgQ2VsbCB7XHJcbiAgICBwdWJsaWMgc3RhdGljIGNlbGxXaWR0aDogbnVtYmVyID0gMzA7XHJcblxyXG4gICAgcHVibGljIHJlYWRvbmx5IHJvdzogbnVtYmVyO1xyXG4gICAgcHVibGljIHJlYWRvbmx5IGNvbDogbnVtYmVyO1xyXG5cclxuICAgIHB1YmxpYyBib3JkZXJzOiBDZWxsQm9yZGVycztcclxuICAgIHB1YmxpYyB2aXNpdGVkOiBib29sZWFuID0gZmFsc2U7XHJcblxyXG4gICAgY29uc3RydWN0b3Iocm93OiBudW1iZXIsIGNvbDogbnVtYmVyKSB7XHJcbiAgICAgICAgdGhpcy5yb3cgPSByb3c7XHJcbiAgICAgICAgdGhpcy5jb2wgPSBjb2w7XHJcbiAgICAgICAgdGhpcy5ib3JkZXJzID0gbmV3IENlbGxCb3JkZXJzKCk7XHJcbiAgICB9XHJcbn1cclxuXHJcbi8qKlxyXG4gKiBTdGFpclxyXG4gKi9cclxuZXhwb3J0IGNsYXNzIFN0YWlyIHtcclxuICAgIHB1YmxpYyByZWFkb25seSByb3c6IG51bWJlcjtcclxuICAgIHB1YmxpYyByZWFkb25seSBjb2w6IG51bWJlcjtcclxuICAgIHB1YmxpYyByZWFkb25seSB1cDogYm9vbGVhbjtcclxuXHJcbiAgICBwcml2YXRlIGNvbnN0cnVjdG9yKHJvdzogbnVtYmVyLCBjb2w6IG51bWJlciwgdXA6IGJvb2xlYW4pIHtcclxuICAgICAgICB0aGlzLnJvdyA9IHJvdztcclxuICAgICAgICB0aGlzLmNvbCA9IGNvbDtcclxuICAgICAgICB0aGlzLnVwID0gdXA7XHJcbiAgICB9XHJcblxyXG4gICAgcHVibGljIHN0YXRpYyB1cHN0YWlyKHJvdzogbnVtYmVyLCBjb2w6IG51bWJlcikge1xyXG4gICAgICAgIHJldHVybiBuZXcgU3RhaXIocm93LCBjb2wsIHRydWUpO1xyXG4gICAgfVxyXG5cclxuICAgIHB1YmxpYyBzdGF0aWMgZG93bnN0YWlyc3RhaXIocm93OiBudW1iZXIsIGNvbDogbnVtYmVyKSB7XHJcbiAgICAgICAgcmV0dXJuIG5ldyBTdGFpcihyb3csIGNvbCwgZmFsc2UpO1xyXG4gICAgfVxyXG59XHJcblxyXG5cclxuZnVuY3Rpb24gcmFuZG9tKG1pbj86IG51bWJlciwgbWF4PzogbnVtYmVyKSB7XHJcbiAgICBsZXQgcmFuZCA9IE1hdGgucmFuZG9tKCk7XHJcbiAgICBpZiAodHlwZW9mIG1pbiA9PT0gJ3VuZGVmaW5lZCcpIHtcclxuICAgICAgICByZXR1cm4gcmFuZDtcclxuICAgIH0gZWxzZSBpZiAodHlwZW9mIG1heCA9PT0gJ3VuZGVmaW5lZCcpIHtcclxuICAgICAgICByZXR1cm4gcmFuZCAqIG1pbjtcclxuICAgIH0gZWxzZVxyXG4gICAgICAgIGlmIChtaW4gPiBtYXgpIHtcclxuICAgICAgICAgICAgY29uc3QgdG1wID0gbWluO1xyXG4gICAgICAgICAgICBtaW4gPSBtYXg7XHJcbiAgICAgICAgICAgIG1heCA9IHRtcDtcclxuICAgICAgICB9XHJcbiAgICByZXR1cm4gTWF0aC5mbG9vcihyYW5kICogKG1heCAtIG1pbikgKyBtaW4pO1xyXG59IiwiaW1wb3J0IHsgTWF6ZSwgQ2VsbCwgU3RhaXIgfSBmcm9tIFwiLi9tYXplXCJcclxuaW1wb3J0IHsgTW9yaWFHYW1lIH0gZnJvbSBcIi4vZ2FtZVwiXHJcbmltcG9ydCB7IEhlcm8gfSBmcm9tIFwiLi9oZXJvXCJcclxuXHJcbi8qKlxyXG4gKiBAY2xhc3MgTWF6ZVZpZXdcclxuICovXHJcbmV4cG9ydCBjbGFzcyBNYXplVmlldyB7XHJcbiAgICBwdWJsaWMgcmVhZG9ubHkgbWF6ZTogTWF6ZTtcclxuXHJcbiAgICBjb25zdHJ1Y3RvcihtYXplOiBNYXplKSB7XHJcbiAgICAgICAgdGhpcy5tYXplID0gbWF6ZTtcclxuICAgIH1cclxuICAgIHB1YmxpYyBkcmF3KHA6IHA1KSB7XHJcbiAgICAgICAgZm9yIChsZXQgciA9IDA7IHIgPCB0aGlzLm1hemUublJvd3M7IHIrKykge1xyXG4gICAgICAgICAgICBmb3IgKGxldCBjID0gMDsgYyA8IHRoaXMubWF6ZS5uQ29sczsgYysrKSB7XHJcbiAgICAgICAgICAgICAgICBsZXQgY2VsbCA9IHRoaXMubWF6ZS5jZWxsKHIsIGMpO1xyXG4gICAgICAgICAgICAgICAgaWYgKGNlbGwudmlzaXRlZCkge1xyXG4gICAgICAgICAgICAgICAgICAgIGxldCBjdiA9IG5ldyBDZWxsVmlldyhjZWxsKTtcclxuICAgICAgICAgICAgICAgICAgICBjdi5kcmF3KHApO1xyXG4gICAgICAgICAgICAgICAgfVxyXG4gICAgICAgICAgICB9XHJcbiAgICAgICAgfVxyXG5cclxuICAgICAgICBpZiAodGhpcy5tYXplLmNlbGwodGhpcy5tYXplLnVwc3RhaXIucm93LCB0aGlzLm1hemUudXBzdGFpci5jb2wpLnZpc2l0ZWQpIHtcclxuICAgICAgICAgICAgbGV0IHN2ID0gbmV3IFN0YWlyVmlldyh0aGlzLm1hemUudXBzdGFpcik7XHJcbiAgICAgICAgICAgIHN2LmRyYXcocCk7XHJcbiAgICAgICAgfVxyXG4gICAgICAgIGlmICh0aGlzLm1hemUuY2VsbCh0aGlzLm1hemUuZG93bnN0YWlyLnJvdywgdGhpcy5tYXplLmRvd25zdGFpci5jb2wpLnZpc2l0ZWQpIHtcclxuICAgICAgICAgICAgbGV0IHN2ID0gbmV3IFN0YWlyVmlldyh0aGlzLm1hemUuZG93bnN0YWlyKTtcclxuICAgICAgICAgICAgc3YuZHJhdyhwKTtcclxuICAgICAgICB9XHJcbiAgICB9XHJcbn1cclxuXHJcbi8qKlxyXG4gKiBAY2xhc3MgQ2VsbFZpZXdcclxuICovXHJcbmV4cG9ydCBjbGFzcyBDZWxsVmlldyB7XHJcbiAgICBwdWJsaWMgc3RhdGljIGNlbGxXaWR0aDogbnVtYmVyID0gMzA7XHJcblxyXG4gICAgcHVibGljIHJlYWRvbmx5IGNlbGw6IENlbGw7XHJcblxyXG4gICAgY29uc3RydWN0b3IoY2VsbDogQ2VsbCkge1xyXG4gICAgICAgIHRoaXMuY2VsbCA9IGNlbGw7XHJcbiAgICB9XHJcblxyXG4gICAgcHVibGljIGRyYXcocDogcDUpIHtcclxuICAgICAgICBsZXQgdyA9IENlbGwuY2VsbFdpZHRoO1xyXG4gICAgICAgIGxldCB4ID0gdGhpcy5jZWxsLmNvbCAqIENlbGwuY2VsbFdpZHRoO1xyXG4gICAgICAgIGxldCB5ID0gdGhpcy5jZWxsLnJvdyAqIENlbGwuY2VsbFdpZHRoO1xyXG4gICAgICAgIGNvbnN0IGJnID0gJyMyMjIyMjInO1xyXG4gICAgICAgIGNvbnN0IHdhbGxDb2xvciA9ICcjRUVFRUVFJztcclxuICAgICAgICBjb25zdCBkb29yQ29sb3IgPSAnIzQ0NDQ0NCc7XHJcblxyXG4gICAgICAgIC8vIFRoZSByb29tXHJcbiAgICAgICAgcC5zdHJva2Uod2FsbENvbG9yKTtcclxuICAgICAgICBwLmZpbGwoYmcpO1xyXG4gICAgICAgIHAucmVjdCh4LCB5LCB3LCB3KVxyXG5cclxuICAgICAgICAvLyBEb29yc1xyXG4gICAgICAgIGNvbnN0IGIgPSA1O1xyXG4gICAgICAgIHAuc3Ryb2tlKGRvb3JDb2xvcik7XHJcbiAgICAgICAgaWYgKCF0aGlzLmNlbGwuYm9yZGVycy50b3ApIHtcclxuICAgICAgICAgICAgcC5saW5lKHggKyBiLCB5LCB4ICsgdyAtIGIsIHkpO1xyXG4gICAgICAgIH1cclxuICAgICAgICBpZiAoIXRoaXMuY2VsbC5ib3JkZXJzLnJpZ2h0KSB7XHJcbiAgICAgICAgICAgIHAubGluZSh4ICsgdywgeSArIGIsIHggKyB3LCB5ICsgdyAtIGIpO1xyXG4gICAgICAgIH1cclxuICAgICAgICBpZiAoIXRoaXMuY2VsbC5ib3JkZXJzLmJvdHRvbSkge1xyXG4gICAgICAgICAgICBwLmxpbmUoeCArIGIsIHkgKyB3LCB4ICsgdyAtIGIsIHkgKyB3KTtcclxuICAgICAgICB9XHJcbiAgICAgICAgaWYgKCF0aGlzLmNlbGwuYm9yZGVycy5sZWZ0KSB7XHJcbiAgICAgICAgICAgIHAubGluZSh4LCB5ICsgYiwgeCwgeSArIHcgLSBiKTtcclxuICAgICAgICB9XHJcbiAgICB9XHJcblxyXG4gICAgcHVibGljIGhpZ2hsaWdodChwOiBwNSkge1xyXG4gICAgICAgIHAubm9TdHJva2UoKTtcclxuICAgICAgICBwLmZpbGwoMjU1LCAyNTUsIDI1NSwgMjU1KTtcclxuICAgICAgICBsZXQgdyA9IENlbGwuY2VsbFdpZHRoO1xyXG4gICAgICAgIGxldCB4ID0gdGhpcy5jZWxsLmNvbCAqIENlbGwuY2VsbFdpZHRoO1xyXG4gICAgICAgIGxldCB5ID0gdGhpcy5jZWxsLnJvdyAqIENlbGwuY2VsbFdpZHRoO1xyXG4gICAgICAgIHAuZWxsaXBzZSh4ICsgdyAvIDIsIHkgKyB3IC8gMiwgdyAvIDIsIHcgLyAyKTtcclxuICAgIH1cclxufVxyXG5cclxuLyoqXHJcbiAqIFN0YWlyVmlld1xyXG4gKi9cclxuZXhwb3J0IGNsYXNzIFN0YWlyVmlldyB7XHJcbiAgICBwdWJsaWMgcmVhZG9ubHkgc3RhaXI6IFN0YWlyO1xyXG5cclxuICAgIGNvbnN0cnVjdG9yKHN0YWlyOiBTdGFpcikge1xyXG4gICAgICAgIHRoaXMuc3RhaXIgPSBzdGFpcjtcclxuICAgIH1cclxuXHJcbiAgICBwdWJsaWMgZHJhdyhwOiBwNSkge1xyXG4gICAgICAgIHAuc3Ryb2tlKDI1NSk7XHJcbiAgICAgICAgaWYgKHRoaXMuc3RhaXIudXApIHtcclxuICAgICAgICAgICAgcC5maWxsKDE5MiwgMTkyLCAxOTIpO1xyXG4gICAgICAgIH1cclxuICAgICAgICBlbHNlIHtcclxuICAgICAgICAgICAgcC5maWxsKDcwLCA3MCwgNzApO1xyXG4gICAgICAgIH1cclxuICAgICAgICBsZXQgdyA9IENlbGwuY2VsbFdpZHRoIC0gNjtcclxuICAgICAgICBsZXQgeCA9IHRoaXMuc3RhaXIuY29sICogQ2VsbC5jZWxsV2lkdGggKyAzO1xyXG4gICAgICAgIGxldCB5ID0gdGhpcy5zdGFpci5yb3cgKiBDZWxsLmNlbGxXaWR0aCArIDM7XHJcbiAgICAgICAgcC5yZWN0KHgsIHksIHcsIHcpO1xyXG4gICAgfVxyXG59XHJcblxyXG4vKipcclxuICogR2FtZVZpZXdcclxuICovXHJcbmV4cG9ydCBjbGFzcyBHYW1lVmlldyB7XHJcbiAgICBwdWJsaWMgcmVhZG9ubHkgZ2FtZTogTW9yaWFHYW1lO1xyXG5cclxuICAgIGNvbnN0cnVjdG9yKGdhbWU6IE1vcmlhR2FtZSkge1xyXG4gICAgICAgIHRoaXMuZ2FtZSA9IGdhbWU7XHJcbiAgICB9XHJcblxyXG4gICAgcHVibGljIGRyYXcocDogcDUpIHtcclxuICAgICAgICBwLmJhY2tncm91bmQoMCk7XHJcbiAgICAgICAgbGV0IG12ID0gbmV3IE1hemVWaWV3KHRoaXMuZ2FtZS5tYXplKCkpO1xyXG4gICAgICAgIG12LmRyYXcocCk7XHJcbiAgICAgICAgbGV0IGhlcm8gPSB0aGlzLmdhbWUuZ2V0SGVybygpO1xyXG4gICAgICAgIGxldCBodiA9IG5ldyBIZXJvVmlldyhoZXJvKTtcclxuICAgICAgICBodi5kcmF3KHApO1xyXG5cclxuICAgICAgICBkb2N1bWVudC5nZXRFbGVtZW50QnlJZChcIm5MZXZlbFwiKS5pbm5lckhUTUwgPSB0aGlzLmdhbWUuZ2V0TGV2ZWwoKS50b1N0cmluZygpO1xyXG4gICAgICAgIGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKFwibGlmZVwiKS5pbm5lckhUTUwgPSBoZXJvLmxpZmUudG9TdHJpbmcoKTtcclxuICAgIH1cclxufVxyXG5cclxuLyoqXHJcbiAqIEhlcm9WaWV3XHJcbiAqL1xyXG5leHBvcnQgY2xhc3MgSGVyb1ZpZXcge1xyXG4gICAgcHJpdmF0ZSBoZXJvOiBIZXJvO1xyXG5cclxuICAgIGNvbnN0cnVjdG9yKGhlcm86IEhlcm8pIHtcclxuICAgICAgICB0aGlzLmhlcm8gPSBoZXJvO1xyXG4gICAgfVxyXG5cclxuICAgIHB1YmxpYyBkcmF3KHA6IHA1KSB7XHJcbiAgICAgICAgcC5zdHJva2UoMjU1KTtcclxuICAgICAgICBpZiAodGhpcy5oZXJvLmxpZmUgPiAwKSB7XHJcbiAgICAgICAgICAgIHAuZmlsbCgwLCAyNTUsIDApO1xyXG4gICAgICAgIH1cclxuICAgICAgICBlbHNlIHtcclxuICAgICAgICAgICAgcC5maWxsKDgwLCAwLCAwKTtcclxuICAgICAgICB9XHJcbiAgICAgICAgbGV0IHggPSB0aGlzLmhlcm8ueCAqIENlbGwuY2VsbFdpZHRoICsgQ2VsbC5jZWxsV2lkdGggLyAyO1xyXG4gICAgICAgIGxldCB5ID0gdGhpcy5oZXJvLnkgKiBDZWxsLmNlbGxXaWR0aCArIENlbGwuY2VsbFdpZHRoIC8gMjtcclxuICAgICAgICBsZXQgciA9IENlbGwuY2VsbFdpZHRoIC8gMiAtIDE7XHJcbiAgICAgICAgcC5lbGxpcHNlKHgsIHksIHIsIHIpO1xyXG4gICAgfVxyXG59IiwiLy8gVGhlIG1vZHVsZSBjYWNoZVxudmFyIF9fd2VicGFja19tb2R1bGVfY2FjaGVfXyA9IHt9O1xuXG4vLyBUaGUgcmVxdWlyZSBmdW5jdGlvblxuZnVuY3Rpb24gX193ZWJwYWNrX3JlcXVpcmVfXyhtb2R1bGVJZCkge1xuXHQvLyBDaGVjayBpZiBtb2R1bGUgaXMgaW4gY2FjaGVcblx0aWYoX193ZWJwYWNrX21vZHVsZV9jYWNoZV9fW21vZHVsZUlkXSkge1xuXHRcdHJldHVybiBfX3dlYnBhY2tfbW9kdWxlX2NhY2hlX19bbW9kdWxlSWRdLmV4cG9ydHM7XG5cdH1cblx0Ly8gQ3JlYXRlIGEgbmV3IG1vZHVsZSAoYW5kIHB1dCBpdCBpbnRvIHRoZSBjYWNoZSlcblx0dmFyIG1vZHVsZSA9IF9fd2VicGFja19tb2R1bGVfY2FjaGVfX1ttb2R1bGVJZF0gPSB7XG5cdFx0Ly8gbm8gbW9kdWxlLmlkIG5lZWRlZFxuXHRcdC8vIG5vIG1vZHVsZS5sb2FkZWQgbmVlZGVkXG5cdFx0ZXhwb3J0czoge31cblx0fTtcblxuXHQvLyBFeGVjdXRlIHRoZSBtb2R1bGUgZnVuY3Rpb25cblx0X193ZWJwYWNrX21vZHVsZXNfX1ttb2R1bGVJZF0obW9kdWxlLCBtb2R1bGUuZXhwb3J0cywgX193ZWJwYWNrX3JlcXVpcmVfXyk7XG5cblx0Ly8gUmV0dXJuIHRoZSBleHBvcnRzIG9mIHRoZSBtb2R1bGVcblx0cmV0dXJuIG1vZHVsZS5leHBvcnRzO1xufVxuXG4iLCIvLyBkZWZpbmUgZ2V0dGVyIGZ1bmN0aW9ucyBmb3IgaGFybW9ueSBleHBvcnRzXG5fX3dlYnBhY2tfcmVxdWlyZV9fLmQgPSAoZXhwb3J0cywgZGVmaW5pdGlvbikgPT4ge1xuXHRmb3IodmFyIGtleSBpbiBkZWZpbml0aW9uKSB7XG5cdFx0aWYoX193ZWJwYWNrX3JlcXVpcmVfXy5vKGRlZmluaXRpb24sIGtleSkgJiYgIV9fd2VicGFja19yZXF1aXJlX18ubyhleHBvcnRzLCBrZXkpKSB7XG5cdFx0XHRPYmplY3QuZGVmaW5lUHJvcGVydHkoZXhwb3J0cywga2V5LCB7IGVudW1lcmFibGU6IHRydWUsIGdldDogZGVmaW5pdGlvbltrZXldIH0pO1xuXHRcdH1cblx0fVxufTsiLCJfX3dlYnBhY2tfcmVxdWlyZV9fLm8gPSAob2JqLCBwcm9wKSA9PiBPYmplY3QucHJvdG90eXBlLmhhc093blByb3BlcnR5LmNhbGwob2JqLCBwcm9wKSIsIi8vIGRlZmluZSBfX2VzTW9kdWxlIG9uIGV4cG9ydHNcbl9fd2VicGFja19yZXF1aXJlX18uciA9IChleHBvcnRzKSA9PiB7XG5cdGlmKHR5cGVvZiBTeW1ib2wgIT09ICd1bmRlZmluZWQnICYmIFN5bWJvbC50b1N0cmluZ1RhZykge1xuXHRcdE9iamVjdC5kZWZpbmVQcm9wZXJ0eShleHBvcnRzLCBTeW1ib2wudG9TdHJpbmdUYWcsIHsgdmFsdWU6ICdNb2R1bGUnIH0pO1xuXHR9XG5cdE9iamVjdC5kZWZpbmVQcm9wZXJ0eShleHBvcnRzLCAnX19lc01vZHVsZScsIHsgdmFsdWU6IHRydWUgfSk7XG59OyIsIi8vIHN0YXJ0dXBcbi8vIExvYWQgZW50cnkgbW9kdWxlXG5fX3dlYnBhY2tfcmVxdWlyZV9fKFwiLi9hcHAvYXBwLnRzXCIpO1xuLy8gVGhpcyBlbnRyeSBtb2R1bGUgdXNlZCAnZXhwb3J0cycgc28gaXQgY2FuJ3QgYmUgaW5saW5lZFxuIl0sInNvdXJjZVJvb3QiOiIifQ==