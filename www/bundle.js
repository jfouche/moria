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
                        console.log(`remove (${c}, ${r}) : top`);
                        i++;
                    }
                    break;
                case 1:
                    if (cell.borders.right) {
                        this.removeWallsBetween(cell, maze.cell(r, c + 1));
                        console.log(`remove (${c}, ${r}) : right`);
                        i++;
                    }
                    break;
                case 2:
                    if (cell.borders.bottom) {
                        this.removeWallsBetween(cell, maze.cell(r + 1, c));
                        console.log(`remove (${c}, ${r}) : bottom`);
                        i++;
                    }
                    break;
                case 3:
                    if (cell.borders.left) {
                        this.removeWallsBetween(cell, maze.cell(r, c - 1));
                        console.log(`remove (${c}, ${r}) : left`);
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
    if (min > max) {
        [min, max] = [max, min];
    }
    return Math.floor(Math.random() * (max - min) + min);
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
//# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIndlYnBhY2s6Ly9tb3JpYS8uL2FwcC9hcHAudHMiLCJ3ZWJwYWNrOi8vbW9yaWEvLi9hcHAvZ2FtZS50cyIsIndlYnBhY2s6Ly9tb3JpYS8uL2FwcC9oZXJvLnRzIiwid2VicGFjazovL21vcmlhLy4vYXBwL21hemUudHMiLCJ3ZWJwYWNrOi8vbW9yaWEvLi9hcHAvdmlld3MudHMiLCJ3ZWJwYWNrOi8vbW9yaWEvd2VicGFjay9ib290c3RyYXAiLCJ3ZWJwYWNrOi8vbW9yaWEvd2VicGFjay9ydW50aW1lL2RlZmluZSBwcm9wZXJ0eSBnZXR0ZXJzIiwid2VicGFjazovL21vcmlhL3dlYnBhY2svcnVudGltZS9oYXNPd25Qcm9wZXJ0eSBzaG9ydGhhbmQiLCJ3ZWJwYWNrOi8vbW9yaWEvd2VicGFjay9ydW50aW1lL21ha2UgbmFtZXNwYWNlIG9iamVjdCIsIndlYnBhY2s6Ly9tb3JpYS93ZWJwYWNrL3N0YXJ0dXAiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6Ijs7Ozs7Ozs7Ozs7OztBQUVrQztBQUNBO0FBSWxDLElBQUksSUFBZSxDQUFDO0FBRXBCLFNBQVMsVUFBVTtJQUNmLElBQUksUUFBUSxHQUFHLFFBQVEsQ0FBQyxjQUFjLENBQUMsUUFBUSxDQUFDLENBQUM7SUFDakQsUUFBUSxDQUFDLFNBQVMsR0FBRyxJQUFJLENBQUMsUUFBUSxFQUFFLENBQUMsUUFBUSxFQUFFLENBQUM7QUFDcEQsQ0FBQztBQUVELElBQUksTUFBTSxHQUFHLFVBQVUsQ0FBTTtJQUN6QixDQUFDLENBQUMsS0FBSyxHQUFHO1FBQ04sSUFBSSxHQUFHLElBQUksNENBQVMsQ0FBQyxDQUFDLEVBQUUsRUFBRSxFQUFFLENBQUMsQ0FBQyxDQUFDO1FBQy9CLElBQUksTUFBTSxHQUFHLENBQUMsQ0FBQyxZQUFZLENBQUMsSUFBSSxDQUFDLEtBQUssRUFBRSxJQUFJLENBQUMsTUFBTSxDQUFDLENBQUM7UUFDckQsTUFBTSxDQUFDLE1BQU0sQ0FBQyxNQUFNLENBQUMsQ0FBQztRQUN0QixD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O1FBQzFELElBQUksQ0FBQyxHQUFHLElBQUksQ0FBQyxJQUFJLENBQUMsQ0FBQyxHQUFHLGlEQUFjLEdBQUcsaURBQWMsR0FBRyxDQUFDLENBQUM7UUFDMUQsSUFBSSxDQUFDLEdBQUcsaURBQWMsR0FBRyxDQUFDLEdBQUcsQ0FBQyxDQUFDO1FBQy9CLENBQUMsQ0FBQyxPQUFPLENBQUMsQ0FBQyxFQUFFLENBQUMsRUFBRSxDQUFDLEVBQUUsQ0FBQyxDQUFDLENBQUM7SUFDMUIsQ0FBQztDQUNKOzs7Ozs7O1VDOUpEO1VBQ0E7O1VBRUE7VUFDQTtVQUNBO1VBQ0E7VUFDQTtVQUNBO1VBQ0E7VUFDQTtVQUNBO1VBQ0E7VUFDQTtVQUNBOztVQUVBO1VBQ0E7O1VBRUE7VUFDQTtVQUNBOzs7OztXQ3JCQTtXQUNBO1dBQ0E7V0FDQTtXQUNBLHdDQUF3Qyx5Q0FBeUM7V0FDakY7V0FDQTtXQUNBLEU7Ozs7O1dDUEEsc0Y7Ozs7O1dDQUE7V0FDQTtXQUNBO1dBQ0Esc0RBQXNELGtCQUFrQjtXQUN4RTtXQUNBLCtDQUErQyxjQUFjO1dBQzdELEU7Ozs7VUNOQTtVQUNBO1VBQ0E7VUFDQSIsImZpbGUiOiJidW5kbGUuanMiLCJzb3VyY2VzQ29udGVudCI6WyIvLy8gPHJlZmVyZW5jZSBwYXRoPVwiLi4vdHlwaW5ncy9wNS5kLnRzXCIgLz5cclxuXHJcbmltcG9ydCB7IE1vcmlhR2FtZSB9IGZyb20gXCIuL2dhbWVcIlxyXG5pbXBvcnQgeyBHYW1lVmlldyB9IGZyb20gXCIuL3ZpZXdzXCJcclxuaW1wb3J0IHsgRGlyZWN0aW9uIH0gZnJvbSBcIi4vaGVyb1wiXHJcblxyXG5cclxubGV0IGdhbWU6IE1vcmlhR2FtZTtcclxuXHJcbmZ1bmN0aW9uIHVwZGF0ZUluZm8oKSB7XHJcbiAgICBsZXQgbGV2ZWxFbHQgPSBkb2N1bWVudC5nZXRFbGVtZW50QnlJZChcIm5MZXZlbFwiKTtcclxuICAgIGxldmVsRWx0LmlubmVySFRNTCA9IGdhbWUuZ2V0TGV2ZWwoKS50b1N0cmluZygpO1xyXG59XHJcblxyXG5sZXQgc2tldGNoID0gZnVuY3Rpb24gKHA6IGFueSkge1xyXG4gICAgcC5zZXR1cCA9IGZ1bmN0aW9uICgpIHtcclxuICAgICAgICBnYW1lID0gbmV3IE1vcmlhR2FtZSg4LCAxMCwgNSk7XHJcbiAgICAgICAgbGV0IGNhbnZhcyA9IHAuY3JlYXRlQ2FudmFzKGdhbWUud2lkdGgsIGdhbWUuaGVpZ2h0KTtcclxuICAgICAgICBjYW52YXMucGFyZW50KCdnYW1lJyk7XHJcbiAgICAgICAgcC5mcmFtZVJhdGUoMTApO1xyXG4gICAgfTtcclxuXHJcbiAgICBwLmRyYXcgPSBmdW5jdGlvbiAoKSB7XHJcbiAgICAgICAgcC5iYWNrZ3JvdW5kKDApO1xyXG4gICAgICAgIGxldCB2aWV3ID0gbmV3IEdhbWVWaWV3KGdhbWUpO1xyXG4gICAgICAgIHZpZXcuZHJhdyhwKTtcclxuICAgICAgICB1cGRhdGVJbmZvKCk7XHJcbiAgICB9O1xyXG5cclxuICAgIHAua2V5UHJlc3NlZCA9IGZ1bmN0aW9uICgpIHtcclxuICAgICAgICBpZiAoZ2FtZS5nZXRIZXJvKCkubGlmZSA8PSAwKSB7XHJcbiAgICAgICAgICAgIHJldHVybjtcclxuICAgICAgICB9XHJcbiAgICAgICAgaWYgKHAua2V5Q29kZSA9PT0gcC5VUF9BUlJPVykge1xyXG4gICAgICAgICAgICBnYW1lLm1vdmVIZXJvKERpcmVjdGlvbi5VUCk7XHJcbiAgICAgICAgfSBlbHNlIGlmIChwLmtleUNvZGUgPT09IHAuRE9XTl9BUlJPVykge1xyXG4gICAgICAgICAgICBnYW1lLm1vdmVIZXJvKERpcmVjdGlvbi5ET1dOKTtcclxuICAgICAgICB9IGVsc2UgaWYgKHAua2V5Q29kZSA9PT0gcC5MRUZUX0FSUk9XKSB7XHJcbiAgICAgICAgICAgIGdhbWUubW92ZUhlcm8oRGlyZWN0aW9uLkxFRlQpO1xyXG4gICAgICAgIH0gZWxzZSBpZiAocC5rZXlDb2RlID09PSBwLlJJR0hUX0FSUk9XKSB7XHJcbiAgICAgICAgICAgIGdhbWUubW92ZUhlcm8oRGlyZWN0aW9uLlJJR0hUKTtcclxuICAgICAgICB9XHJcbiAgICB9XHJcbn07XHJcblxyXG5sZXQgbXlwNSA9IG5ldyBwNShza2V0Y2gpOyIsImltcG9ydCB7IEhlcm8sIERpcmVjdGlvbiB9IGZyb20gXCIuL2hlcm9cIlxyXG5pbXBvcnQgeyBNYXplLCBNYXplR2VuZXJhdG9yLCBDZWxsIH0gZnJvbSBcIi4vbWF6ZVwiXHJcblxyXG4vKipcclxuICogTW9yaWFHYW1lXHJcbiAqL1xyXG5leHBvcnQgY2xhc3MgTW9yaWFHYW1lIHtcclxuICAgIHB1YmxpYyByZWFkb25seSBuUm93czogbnVtYmVyO1xyXG4gICAgcHVibGljIHJlYWRvbmx5IG5Db2xzOiBudW1iZXI7XHJcbiAgICBwdWJsaWMgcmVhZG9ubHkgd2lkdGg6IG51bWJlcjtcclxuICAgIHB1YmxpYyByZWFkb25seSBoZWlnaHQ6IG51bWJlcjtcclxuXHJcbiAgICBwcml2YXRlIGhlcm86IEhlcm87XHJcbiAgICBwcml2YXRlIG1hemVzOiBNYXplW107XHJcbiAgICBwcml2YXRlIGN1cnJlbnRMZXZlbDogbnVtYmVyO1xyXG5cclxuICAgIGNvbnN0cnVjdG9yKG5Sb3dzOiBudW1iZXIsIG5Db2xzOiBudW1iZXIsIG5MZXZlbHM6IG51bWJlcikge1xyXG4gICAgICAgIHRoaXMublJvd3MgPSBuUm93cztcclxuICAgICAgICB0aGlzLm5Db2xzID0gbkNvbHM7XHJcblxyXG4gICAgICAgIGxldCBtYXplR2VuZXJhdG9yID0gbmV3IE1hemVHZW5lcmF0b3IoKTtcclxuICAgICAgICB0aGlzLm1hemVzID0gW107XHJcbiAgICAgICAgZm9yIChsZXQgaSA9IDA7IGkgPCBuTGV2ZWxzOyBpKyspIHtcclxuICAgICAgICAgICAgdGhpcy5tYXplcy5wdXNoKG1hemVHZW5lcmF0b3IubmV3TWF6ZSh0aGlzLm5Sb3dzLCB0aGlzLm5Db2xzKSk7XHJcbiAgICAgICAgfVxyXG5cclxuICAgICAgICB0aGlzLmN1cnJlbnRMZXZlbCA9IDA7XHJcblxyXG4gICAgICAgIGxldCBtYXplID0gdGhpcy5tYXplKCk7XHJcbiAgICAgICAgdGhpcy5oZXJvID0gbmV3IEhlcm8obWF6ZS51cHN0YWlyLmNvbCwgbWF6ZS51cHN0YWlyLnJvdyk7XHJcblxyXG4gICAgICAgIHRoaXMud2lkdGggPSBtYXplLndpZHRoO1xyXG4gICAgICAgIHRoaXMuaGVpZ2h0ID0gbWF6ZS5oZWlnaHQ7XHJcblxyXG4gICAgICAgIHRoaXMuaW5pdExldmVsKCk7XHJcbiAgICB9XHJcblxyXG4gICAgcHVibGljIGdldExldmVsKCk6IG51bWJlciB7XHJcbiAgICAgICAgcmV0dXJuIHRoaXMuY3VycmVudExldmVsO1xyXG4gICAgfVxyXG5cclxuICAgIHByaXZhdGUgaW5pdExldmVsKCkge1xyXG4gICAgICAgIGxldCBtYXplID0gdGhpcy5tYXplKCk7XHJcbiAgICAgICAgdGhpcy5oZXJvLm1vdmVUbyhtYXplLnVwc3RhaXIuY29sLCBtYXplLnVwc3RhaXIucm93KTtcclxuICAgICAgICBtYXplLmNlbGwodGhpcy5oZXJvLnksIHRoaXMuaGVyby54KS52aXNpdGVkID0gdHJ1ZTtcclxuICAgICAgICB0aGlzLmNoZWNrVmlzaWJpbGl0eSgpO1xyXG4gICAgfVxyXG5cclxuICAgIHB1YmxpYyBtYXplKCk6IE1hemUge1xyXG4gICAgICAgIHJldHVybiB0aGlzLm1hemVzW3RoaXMuY3VycmVudExldmVsXTtcclxuICAgIH1cclxuXHJcbiAgICBwdWJsaWMgbW92ZUhlcm8oZGlyZWN0aW9uOiBEaXJlY3Rpb24pIHtcclxuICAgICAgICBpZiAodGhpcy5jYW5Nb3ZlKGRpcmVjdGlvbikpIHtcclxuICAgICAgICAgICAgdGhpcy5oZXJvLm1vdmUoZGlyZWN0aW9uKTtcclxuICAgICAgICAgICAgdGhpcy5tYXplKCkuY2VsbCh0aGlzLmhlcm8ueSwgdGhpcy5oZXJvLngpLnZpc2l0ZWQgPSB0cnVlO1xyXG4gICAgICAgICAgICBpZiAodGhpcy5oZXJvLnggPT09IHRoaXMubWF6ZSgpLmRvd25zdGFpci5jb2wgJiYgdGhpcy5oZXJvLnkgPT09IHRoaXMubWF6ZSgpLmRvd25zdGFpci5yb3cpIHtcclxuICAgICAgICAgICAgICAgIHRoaXMuY3VycmVudExldmVsKys7XHJcbiAgICAgICAgICAgICAgICB0aGlzLmluaXRMZXZlbCgpO1xyXG4gICAgICAgICAgICB9XHJcbiAgICAgICAgICAgIHRoaXMuY2hlY2tWaXNpYmlsaXR5KCk7XHJcbiAgICAgICAgfVxyXG4gICAgfVxyXG5cclxuICAgIHB1YmxpYyBjYW5Nb3ZlKGRpcmVjdGlvbjogRGlyZWN0aW9uKTogYm9vbGVhbiB7XHJcbiAgICAgICAgbGV0IGNlbGxCb3JkZXJzID0gdGhpcy5tYXplKCkuY2VsbCh0aGlzLmhlcm8ueSwgdGhpcy5oZXJvLngpLmJvcmRlcnM7XHJcbiAgICAgICAgcmV0dXJuIChkaXJlY3Rpb24gPT09IERpcmVjdGlvbi5SSUdIVCAmJiAhY2VsbEJvcmRlcnMucmlnaHQpXHJcbiAgICAgICAgICAgIHx8IChkaXJlY3Rpb24gPT09IERpcmVjdGlvbi5MRUZUICYmICFjZWxsQm9yZGVycy5sZWZ0KVxyXG4gICAgICAgICAgICB8fCAoZGlyZWN0aW9uID09PSBEaXJlY3Rpb24uVVAgJiYgIWNlbGxCb3JkZXJzLnRvcClcclxuICAgICAgICAgICAgfHwgKGRpcmVjdGlvbiA9PT0gRGlyZWN0aW9uLkRPV04gJiYgIWNlbGxCb3JkZXJzLmJvdHRvbSk7XHJcbiAgICB9XHJcblxyXG4gICAgcHJpdmF0ZSBjaGVja1Zpc2liaWxpdHkoKSB7XHJcbiAgICAgICAgbGV0IHg6IG51bWJlcjtcclxuICAgICAgICBsZXQgeTogbnVtYmVyO1xyXG4gICAgICAgIGxldCBjZWxsOiBDZWxsO1xyXG4gICAgICAgIGxldCBtYXplID0gdGhpcy5tYXplKCk7XHJcbiAgICAgICAgbGV0IHJlc2V0ID0gKCkgPT4ge1xyXG4gICAgICAgICAgICB4ID0gdGhpcy5oZXJvLng7XHJcbiAgICAgICAgICAgIHkgPSB0aGlzLmhlcm8ueTtcclxuICAgICAgICAgICAgY2VsbCA9IG1hemUuY2VsbCh5LCB4KTtcclxuICAgICAgICB9XHJcbiAgICAgICAgbGV0IG5leHQgPSAoKSA9PiB7XHJcbiAgICAgICAgICAgIGNlbGwgPSBtYXplLmNlbGwoeSwgeCk7XHJcbiAgICAgICAgICAgIGNlbGwudmlzaXRlZCA9IHRydWU7XHJcbiAgICAgICAgfVxyXG4gICAgICAgIHJlc2V0KCk7XHJcbiAgICAgICAgd2hpbGUgKCFjZWxsLmJvcmRlcnMudG9wKSB7XHJcbiAgICAgICAgICAgIHkgLT0gMTtcclxuICAgICAgICAgICAgbmV4dCgpO1xyXG4gICAgICAgIH1cclxuICAgICAgICByZXNldCgpO1xyXG4gICAgICAgIHdoaWxlICghY2VsbC5ib3JkZXJzLnJpZ2h0KSB7XHJcbiAgICAgICAgICAgIHggKz0gMTtcclxuICAgICAgICAgICAgbmV4dCgpO1xyXG4gICAgICAgIH1cclxuICAgICAgICByZXNldCgpO1xyXG4gICAgICAgIHdoaWxlICghY2VsbC5ib3JkZXJzLmJvdHRvbSkge1xyXG4gICAgICAgICAgICB5ICs9IDE7XHJcbiAgICAgICAgICAgIG5leHQoKTtcclxuICAgICAgICB9XHJcbiAgICAgICAgcmVzZXQoKTtcclxuICAgICAgICB3aGlsZSAoIWNlbGwuYm9yZGVycy5sZWZ0KSB7XHJcbiAgICAgICAgICAgIHggLT0gMTtcclxuICAgICAgICAgICAgbmV4dCgpO1xyXG4gICAgICAgIH1cclxuICAgIH1cclxuXHJcbiAgICBwdWJsaWMgZ2V0SGVybygpIHtcclxuICAgICAgICByZXR1cm4gdGhpcy5oZXJvO1xyXG4gICAgfVxyXG59IiwiZXhwb3J0IGNvbnN0IGVudW0gRGlyZWN0aW9uIHtcclxuICAgIFVQLCBET1dOLCBMRUZULCBSSUdIVFxyXG59XHJcblxyXG4vKipcclxuICogSGVyb1xyXG4gKi9cclxuZXhwb3J0IGNsYXNzIEhlcm8ge1xyXG4gICAgcHJpdmF0ZSBfeDogbnVtYmVyO1xyXG4gICAgcHJpdmF0ZSBfeTogbnVtYmVyO1xyXG4gICAgcHJpdmF0ZSBfbGlmZTogbnVtYmVyO1xyXG5cclxuICAgIGNvbnN0cnVjdG9yKHg6IG51bWJlciwgeTogbnVtYmVyKSB7XHJcbiAgICAgICAgdGhpcy5feCA9IHg7XHJcbiAgICAgICAgdGhpcy5feSA9IHk7XHJcbiAgICAgICAgdGhpcy5fbGlmZSA9IDEwMDtcclxuICAgIH1cclxuXHJcbiAgICBwdWJsaWMgZ2V0IHgoKTogbnVtYmVyIHtcclxuICAgICAgICByZXR1cm4gdGhpcy5feDtcclxuICAgIH1cclxuXHJcbiAgICBwdWJsaWMgZ2V0IHkoKTogbnVtYmVyIHtcclxuICAgICAgICByZXR1cm4gdGhpcy5feTtcclxuICAgIH1cclxuXHJcbiAgICBwdWJsaWMgZ2V0IGxpZmUoKSB7XHJcbiAgICAgICAgcmV0dXJuIHRoaXMuX2xpZmU7XHJcbiAgICB9XHJcblxyXG4gICAgcHVibGljIG1vdmVUbyh4OiBudW1iZXIsIHk6IG51bWJlcikge1xyXG4gICAgICAgIHRoaXMuX3ggPSB4O1xyXG4gICAgICAgIHRoaXMuX3kgPSB5O1xyXG4gICAgfVxyXG5cclxuICAgIHB1YmxpYyBtb3ZlKGRpcjogRGlyZWN0aW9uKSB7XHJcbiAgICAgICAgbGV0IGR4ID0gMCwgZHkgPSAwO1xyXG4gICAgICAgIHN3aXRjaCAoZGlyKSB7XHJcbiAgICAgICAgICAgIGNhc2UgRGlyZWN0aW9uLlVQOiBkeSA9IC0xOyBicmVhaztcclxuICAgICAgICAgICAgY2FzZSBEaXJlY3Rpb24uRE9XTjogZHkgPSAxOyBicmVhaztcclxuICAgICAgICAgICAgY2FzZSBEaXJlY3Rpb24uTEVGVDogZHggPSAtMTsgYnJlYWs7XHJcbiAgICAgICAgICAgIGNhc2UgRGlyZWN0aW9uLlJJR0hUOiBkeCA9IDE7IGJyZWFrO1xyXG4gICAgICAgICAgICBkZWZhdWx0OiBicmVhaztcclxuICAgICAgICB9XHJcbiAgICAgICAgdGhpcy5feCArPSBkeDtcclxuICAgICAgICB0aGlzLl95ICs9IGR5O1xyXG4gICAgICAgIHRoaXMuX2xpZmUtLTtcclxuICAgIH1cclxufVxyXG4iLCIvKipcclxuICogQGNsYXNzIE1hemVcclxuICovXHJcbmV4cG9ydCBjbGFzcyBNYXplIHtcclxuICAgIHB1YmxpYyByZWFkb25seSBuUm93czogbnVtYmVyO1xyXG4gICAgcHVibGljIHJlYWRvbmx5IG5Db2xzOiBudW1iZXI7XHJcbiAgICBwdWJsaWMgcmVhZG9ubHkgd2lkdGg6IG51bWJlcjtcclxuICAgIHB1YmxpYyByZWFkb25seSBoZWlnaHQ6IG51bWJlcjtcclxuICAgIHByaXZhdGUgZ3JpZDogQ2VsbFtdW107XHJcbiAgICBwdWJsaWMgcmVhZG9ubHkgdXBzdGFpcjogU3RhaXI7XHJcbiAgICBwdWJsaWMgcmVhZG9ubHkgZG93bnN0YWlyOiBTdGFpcjtcclxuXHJcbiAgICBjb25zdHJ1Y3RvcihuUm93czogbnVtYmVyLCBuQ29sczogbnVtYmVyKSB7XHJcbiAgICAgICAgdGhpcy5uUm93cyA9IG5Sb3dzO1xyXG4gICAgICAgIHRoaXMubkNvbHMgPSBuQ29scztcclxuICAgICAgICB0aGlzLmhlaWdodCA9IG5Sb3dzICogQ2VsbC5jZWxsV2lkdGggKyAxO1xyXG4gICAgICAgIHRoaXMud2lkdGggPSBuQ29scyAqIENlbGwuY2VsbFdpZHRoICsgMTtcclxuICAgICAgICB0aGlzLmdyaWQgPSBbXTtcclxuXHJcbiAgICAgICAgZm9yICh2YXIgciA9IDA7IHIgPCB0aGlzLm5Sb3dzOyByKyspIHtcclxuICAgICAgICAgICAgdGhpcy5ncmlkW3JdID0gW107XHJcbiAgICAgICAgICAgIGZvciAodmFyIGMgPSAwOyBjIDwgdGhpcy5uQ29sczsgYysrKSB7XHJcbiAgICAgICAgICAgICAgICB0aGlzLmdyaWRbcl1bY10gPSBuZXcgQ2VsbChyLCBjKTtcclxuICAgICAgICAgICAgfVxyXG4gICAgICAgIH1cclxuICAgICAgICB0aGlzLnVwc3RhaXIgPSBTdGFpci51cHN0YWlyKDAsIDApO1xyXG4gICAgICAgIHRoaXMuZG93bnN0YWlyID0gU3RhaXIuZG93bnN0YWlyc3RhaXIoblJvd3MgLSAxLCBuQ29scyAtIDEpO1xyXG4gICAgfVxyXG5cclxuICAgIHB1YmxpYyBjZWxsKHJvdzogbnVtYmVyLCBjb2w6IG51bWJlcikge1xyXG4gICAgICAgIHJldHVybiB0aGlzLmdyaWRbcm93XVtjb2xdO1xyXG4gICAgfVxyXG59XHJcblxyXG5cclxuLyoqXHJcbiAqIEBjbGFzcyBNYXplR2VuZXJhdG9yXHJcbiAqL1xyXG5leHBvcnQgY2xhc3MgTWF6ZUdlbmVyYXRvciB7XHJcblxyXG4gICAgcHVibGljIG5ld01hemUoblJvd3M6IG51bWJlciwgbkNvbHM6IG51bWJlcik6IE1hemUge1xyXG4gICAgICAgIGxldCBtYXplID0gbmV3IE1hemUoblJvd3MsIG5Db2xzKTtcclxuICAgICAgICBsZXQgYmFja3RyYWNraW5nOiBDZWxsW10gPSBbXTtcclxuICAgICAgICBsZXQgY3VycmVudENlbGwgPSBtYXplLmNlbGwoMCwgMCk7XHJcbiAgICAgICAgY3VycmVudENlbGwudmlzaXRlZCA9IHRydWU7XHJcbiAgICAgICAgbGV0IGZpbmlzaGVkID0gZmFsc2U7XHJcbiAgICAgICAgd2hpbGUgKCFmaW5pc2hlZCkge1xyXG4gICAgICAgICAgICBsZXQgbmV4dCA9IHRoaXMuZ2V0TmV4dE5laWdoYm9yKG1hemUsIGN1cnJlbnRDZWxsKTtcclxuICAgICAgICAgICAgaWYgKG5leHQpIHtcclxuICAgICAgICAgICAgICAgIG5leHQudmlzaXRlZCA9IHRydWU7XHJcbiAgICAgICAgICAgICAgICBiYWNrdHJhY2tpbmcucHVzaChjdXJyZW50Q2VsbCk7XHJcbiAgICAgICAgICAgICAgICB0aGlzLnJlbW92ZVdhbGxzQmV0d2VlbihjdXJyZW50Q2VsbCwgbmV4dCk7XHJcbiAgICAgICAgICAgICAgICBjdXJyZW50Q2VsbCA9IG5leHQ7XHJcbiAgICAgICAgICAgIH0gZWxzZSBpZiAoYmFja3RyYWNraW5nLmxlbmd0aCA+IDApIHtcclxuICAgICAgICAgICAgICAgIG5leHQgPSBiYWNrdHJhY2tpbmcucG9wKCk7XHJcbiAgICAgICAgICAgICAgICBjdXJyZW50Q2VsbCA9IG5leHQ7XHJcbiAgICAgICAgICAgIH0gZWxzZSB7XHJcbiAgICAgICAgICAgICAgICBjb25zb2xlLmxvZyhcIkZJTklTSFwiKTtcclxuICAgICAgICAgICAgICAgIGZpbmlzaGVkID0gdHJ1ZTtcclxuICAgICAgICAgICAgfVxyXG4gICAgICAgIH1cclxuXHJcbiAgICAgICAgZm9yICh2YXIgciA9IDA7IHIgPCBuUm93czsgcisrKSB7XHJcbiAgICAgICAgICAgIGZvciAodmFyIGMgPSAwOyBjIDwgbkNvbHM7IGMrKykge1xyXG4gICAgICAgICAgICAgICAgbWF6ZS5jZWxsKHIsIGMpLnZpc2l0ZWQgPSBmYWxzZTtcclxuICAgICAgICAgICAgfVxyXG4gICAgICAgIH1cclxuXHJcbiAgICAgICAgdGhpcy5yZW1vdmVSYW5kb21XYWxscyhtYXplLCAxMCk7XHJcblxyXG4gICAgICAgIHJldHVybiBtYXplO1xyXG4gICAgfVxyXG5cclxuICAgIHByaXZhdGUgZ2V0TmV4dE5laWdoYm9yKG1hemU6IE1hemUsIGNlbGw6IENlbGwpOiBDZWxsIHtcclxuICAgICAgICBsZXQgbmVpZ2hib3JzOiBDZWxsW10gPSBbXTtcclxuICAgICAgICBpZiAoY2VsbC5yb3cgPiAwKSB7XHJcbiAgICAgICAgICAgIGxldCBsZWZ0ID0gbWF6ZS5jZWxsKGNlbGwucm93IC0gMSwgY2VsbC5jb2wpO1xyXG4gICAgICAgICAgICBpZiAoIWxlZnQudmlzaXRlZCkge1xyXG4gICAgICAgICAgICAgICAgbmVpZ2hib3JzLnB1c2gobGVmdCk7XHJcbiAgICAgICAgICAgIH1cclxuICAgICAgICB9XHJcbiAgICAgICAgaWYgKGNlbGwucm93IDwgbWF6ZS5uUm93cyAtIDEpIHtcclxuICAgICAgICAgICAgbGV0IHJpZ2h0ID0gbWF6ZS5jZWxsKGNlbGwucm93ICsgMSwgY2VsbC5jb2wpO1xyXG4gICAgICAgICAgICBpZiAoIXJpZ2h0LnZpc2l0ZWQpIHtcclxuICAgICAgICAgICAgICAgIG5laWdoYm9ycy5wdXNoKHJpZ2h0KTtcclxuICAgICAgICAgICAgfVxyXG4gICAgICAgIH1cclxuICAgICAgICBpZiAoY2VsbC5jb2wgPiAwKSB7XHJcbiAgICAgICAgICAgIGxldCB0b3AgPSBtYXplLmNlbGwoY2VsbC5yb3csIGNlbGwuY29sIC0gMSk7XHJcbiAgICAgICAgICAgIGlmICghdG9wLnZpc2l0ZWQpIHtcclxuICAgICAgICAgICAgICAgIG5laWdoYm9ycy5wdXNoKHRvcCk7XHJcbiAgICAgICAgICAgIH1cclxuICAgICAgICB9XHJcbiAgICAgICAgaWYgKGNlbGwuY29sIDwgbWF6ZS5uQ29scyAtIDEpIHtcclxuICAgICAgICAgICAgbGV0IGJvdHRvbSA9IG1hemUuY2VsbChjZWxsLnJvdywgY2VsbC5jb2wgKyAxKTtcclxuICAgICAgICAgICAgaWYgKCFib3R0b20udmlzaXRlZCkge1xyXG4gICAgICAgICAgICAgICAgbmVpZ2hib3JzLnB1c2goYm90dG9tKTtcclxuICAgICAgICAgICAgfVxyXG4gICAgICAgIH1cclxuXHJcbiAgICAgICAgbGV0IG5leHQ6IENlbGwgPSB1bmRlZmluZWQ7XHJcbiAgICAgICAgaWYgKG5laWdoYm9ycy5sZW5ndGggPiAwKSB7XHJcbiAgICAgICAgICAgIHZhciByID0gcmFuZG9tKDAsIG5laWdoYm9ycy5sZW5ndGgpO1xyXG4gICAgICAgICAgICBuZXh0ID0gbmVpZ2hib3JzW3JdO1xyXG4gICAgICAgIH1cclxuICAgICAgICByZXR1cm4gbmV4dDtcclxuICAgIH1cclxuXHJcbiAgICBwcml2YXRlIHJlbW92ZVdhbGxzQmV0d2VlbihhOiBDZWxsLCBiOiBDZWxsKSB7XHJcbiAgICAgICAgaWYgKGEuY29sID4gYi5jb2wpIHtcclxuICAgICAgICAgICAgYS5ib3JkZXJzLmxlZnQgPSBmYWxzZTtcclxuICAgICAgICAgICAgYi5ib3JkZXJzLnJpZ2h0ID0gZmFsc2U7XHJcbiAgICAgICAgfSBlbHNlIGlmIChhLmNvbCA8IGIuY29sKSB7XHJcbiAgICAgICAgICAgIGEuYm9yZGVycy5yaWdodCA9IGZhbHNlO1xyXG4gICAgICAgICAgICBiLmJvcmRlcnMubGVmdCA9IGZhbHNlO1xyXG4gICAgICAgIH0gZWxzZSBpZiAoYS5yb3cgPiBiLnJvdykge1xyXG4gICAgICAgICAgICBhLmJvcmRlcnMudG9wID0gZmFsc2U7XHJcbiAgICAgICAgICAgIGIuYm9yZGVycy5ib3R0b20gPSBmYWxzZTtcclxuICAgICAgICB9IGVsc2UgaWYgKGEucm93IDwgYi5yb3cpIHtcclxuICAgICAgICAgICAgYS5ib3JkZXJzLmJvdHRvbSA9IGZhbHNlO1xyXG4gICAgICAgICAgICBiLmJvcmRlcnMudG9wID0gZmFsc2U7XHJcbiAgICAgICAgfVxyXG4gICAgfVxyXG5cclxuICAgIHByaXZhdGUgcmVtb3ZlUmFuZG9tV2FsbHMobWF6ZTogTWF6ZSwgbjogbnVtYmVyKSB7XHJcbiAgICAgICAgZm9yIChsZXQgaSA9IDA7IGkgPCBuOykge1xyXG4gICAgICAgICAgICBsZXQgciA9IHJhbmRvbSgxLCBtYXplLm5Sb3dzIC0gMik7XHJcbiAgICAgICAgICAgIGxldCBjID0gcmFuZG9tKDEsIG1hemUubkNvbHMgLSAyKTtcclxuXHJcbiAgICAgICAgICAgIGxldCBjZWxsID0gbWF6ZS5jZWxsKHIsIGMpO1xyXG4gICAgICAgICAgICBsZXQgbmV4dCA9IHJhbmRvbSgwLCAzKTtcclxuICAgICAgICAgICAgc3dpdGNoIChuZXh0KSB7XHJcbiAgICAgICAgICAgICAgICBjYXNlIDA6XHJcbiAgICAgICAgICAgICAgICAgICAgaWYgKGNlbGwuYm9yZGVycy50b3ApIHtcclxuICAgICAgICAgICAgICAgICAgICAgICAgdGhpcy5yZW1vdmVXYWxsc0JldHdlZW4oY2VsbCwgbWF6ZS5jZWxsKHIgLSAxLCBjKSk7XHJcbiAgICAgICAgICAgICAgICAgICAgICAgIGNvbnNvbGUubG9nKGByZW1vdmUgKCR7Y30sICR7cn0pIDogdG9wYCk7XHJcbiAgICAgICAgICAgICAgICAgICAgICAgIGkrKztcclxuICAgICAgICAgICAgICAgICAgICB9XHJcbiAgICAgICAgICAgICAgICAgICAgYnJlYWs7XHJcblxyXG4gICAgICAgICAgICAgICAgY2FzZSAxOlxyXG4gICAgICAgICAgICAgICAgICAgIGlmIChjZWxsLmJvcmRlcnMucmlnaHQpIHtcclxuICAgICAgICAgICAgICAgICAgICAgICAgdGhpcy5yZW1vdmVXYWxsc0JldHdlZW4oY2VsbCwgbWF6ZS5jZWxsKHIsIGMgKyAxKSk7XHJcbiAgICAgICAgICAgICAgICAgICAgICAgIGNvbnNvbGUubG9nKGByZW1vdmUgKCR7Y30sICR7cn0pIDogcmlnaHRgKTtcclxuICAgICAgICAgICAgICAgICAgICAgICAgaSsrO1xyXG4gICAgICAgICAgICAgICAgICAgIH1cclxuICAgICAgICAgICAgICAgICAgICBicmVhaztcclxuXHJcbiAgICAgICAgICAgICAgICBjYXNlIDI6XHJcbiAgICAgICAgICAgICAgICAgICAgaWYgKGNlbGwuYm9yZGVycy5ib3R0b20pIHtcclxuICAgICAgICAgICAgICAgICAgICAgICAgdGhpcy5yZW1vdmVXYWxsc0JldHdlZW4oY2VsbCwgbWF6ZS5jZWxsKHIgKyAxLCBjKSk7XHJcbiAgICAgICAgICAgICAgICAgICAgICAgIGNvbnNvbGUubG9nKGByZW1vdmUgKCR7Y30sICR7cn0pIDogYm90dG9tYCk7XHJcbiAgICAgICAgICAgICAgICAgICAgICAgIGkrKztcclxuICAgICAgICAgICAgICAgICAgICB9XHJcbiAgICAgICAgICAgICAgICAgICAgYnJlYWs7XHJcblxyXG4gICAgICAgICAgICAgICAgY2FzZSAzOlxyXG4gICAgICAgICAgICAgICAgICAgIGlmIChjZWxsLmJvcmRlcnMubGVmdCkge1xyXG4gICAgICAgICAgICAgICAgICAgICAgICB0aGlzLnJlbW92ZVdhbGxzQmV0d2VlbihjZWxsLCBtYXplLmNlbGwociwgYyAtIDEpKTtcclxuICAgICAgICAgICAgICAgICAgICAgICAgY29uc29sZS5sb2coYHJlbW92ZSAoJHtjfSwgJHtyfSkgOiBsZWZ0YCk7XHJcbiAgICAgICAgICAgICAgICAgICAgICAgIGkrKztcclxuICAgICAgICAgICAgICAgICAgICB9XHJcbiAgICAgICAgICAgICAgICAgICAgYnJlYWs7XHJcblxyXG4gICAgICAgICAgICAgICAgZGVmYXVsdDpcclxuICAgICAgICAgICAgICAgICAgICBicmVhaztcclxuICAgICAgICAgICAgfVxyXG4gICAgICAgIH1cclxuICAgIH1cclxufVxyXG5cclxuLyoqXHJcbiAqIEBjbGFzcyBDZWxsQm9yZGVyc1xyXG4gKi9cclxuZXhwb3J0IGNsYXNzIENlbGxCb3JkZXJzIHtcclxuICAgIHRvcDogYm9vbGVhbiA9IHRydWU7XHJcbiAgICByaWdodDogYm9vbGVhbiA9IHRydWU7XHJcbiAgICBib3R0b206IGJvb2xlYW4gPSB0cnVlO1xyXG4gICAgbGVmdDogYm9vbGVhbiA9IHRydWU7XHJcbn1cclxuXHJcbi8qKlxyXG4gKiBAY2xhc3MgQ2VsbFxyXG4gKi9cclxuZXhwb3J0IGNsYXNzIENlbGwge1xyXG4gICAgcHVibGljIHN0YXRpYyBjZWxsV2lkdGg6IG51bWJlciA9IDMwO1xyXG5cclxuICAgIHB1YmxpYyByZWFkb25seSByb3c6IG51bWJlcjtcclxuICAgIHB1YmxpYyByZWFkb25seSBjb2w6IG51bWJlcjtcclxuXHJcbiAgICBwdWJsaWMgYm9yZGVyczogQ2VsbEJvcmRlcnM7XHJcbiAgICBwdWJsaWMgdmlzaXRlZDogYm9vbGVhbiA9IGZhbHNlO1xyXG5cclxuICAgIGNvbnN0cnVjdG9yKHJvdzogbnVtYmVyLCBjb2w6IG51bWJlcikge1xyXG4gICAgICAgIHRoaXMucm93ID0gcm93O1xyXG4gICAgICAgIHRoaXMuY29sID0gY29sO1xyXG4gICAgICAgIHRoaXMuYm9yZGVycyA9IG5ldyBDZWxsQm9yZGVycygpO1xyXG4gICAgfVxyXG59XHJcblxyXG4vKipcclxuICogU3RhaXJcclxuICovXHJcbmV4cG9ydCBjbGFzcyBTdGFpciB7XHJcbiAgICBwdWJsaWMgcmVhZG9ubHkgcm93OiBudW1iZXI7XHJcbiAgICBwdWJsaWMgcmVhZG9ubHkgY29sOiBudW1iZXI7XHJcbiAgICBwdWJsaWMgcmVhZG9ubHkgdXA6IGJvb2xlYW47XHJcblxyXG4gICAgcHJpdmF0ZSBjb25zdHJ1Y3Rvcihyb3c6IG51bWJlciwgY29sOiBudW1iZXIsIHVwOiBib29sZWFuKSB7XHJcbiAgICAgICAgdGhpcy5yb3cgPSByb3c7XHJcbiAgICAgICAgdGhpcy5jb2wgPSBjb2w7XHJcbiAgICAgICAgdGhpcy51cCA9IHVwO1xyXG4gICAgfVxyXG5cclxuICAgIHB1YmxpYyBzdGF0aWMgdXBzdGFpcihyb3c6IG51bWJlciwgY29sOiBudW1iZXIpIHtcclxuICAgICAgICByZXR1cm4gbmV3IFN0YWlyKHJvdywgY29sLCB0cnVlKTtcclxuICAgIH1cclxuXHJcbiAgICBwdWJsaWMgc3RhdGljIGRvd25zdGFpcnN0YWlyKHJvdzogbnVtYmVyLCBjb2w6IG51bWJlcikge1xyXG4gICAgICAgIHJldHVybiBuZXcgU3RhaXIocm93LCBjb2wsIGZhbHNlKTtcclxuICAgIH1cclxufVxyXG5cclxuLyoqXHJcbiAqIEEgc2hvcnRjdXQgdG8gZ2V0IGEgcmFuZG9tIG51bWJlciBiZXR3ZWVuIGBtaW5gIGFuZCBgbWF4YFxyXG4gKiBAcGFyYW0gbWluIFxyXG4gKiBAcGFyYW0gbWF4IFxyXG4gKi9cclxuZnVuY3Rpb24gcmFuZG9tKG1pbjogbnVtYmVyLCBtYXg6IG51bWJlcikge1xyXG4gICAgaWYgKG1pbiA+IG1heCkge1xyXG4gICAgICAgIFttaW4sIG1heF0gPSBbbWF4LCBtaW5dXHJcbiAgICB9XHJcbiAgICByZXR1cm4gTWF0aC5mbG9vcihNYXRoLnJhbmRvbSgpICogKG1heCAtIG1pbikgKyBtaW4pO1xyXG59IiwiaW1wb3J0IHsgTWF6ZSwgQ2VsbCwgU3RhaXIgfSBmcm9tIFwiLi9tYXplXCJcclxuaW1wb3J0IHsgTW9yaWFHYW1lIH0gZnJvbSBcIi4vZ2FtZVwiXHJcbmltcG9ydCB7IEhlcm8gfSBmcm9tIFwiLi9oZXJvXCJcclxuXHJcbi8qKlxyXG4gKiBAY2xhc3MgTWF6ZVZpZXdcclxuICovXHJcbmV4cG9ydCBjbGFzcyBNYXplVmlldyB7XHJcbiAgICBwdWJsaWMgcmVhZG9ubHkgbWF6ZTogTWF6ZTtcclxuXHJcbiAgICBjb25zdHJ1Y3RvcihtYXplOiBNYXplKSB7XHJcbiAgICAgICAgdGhpcy5tYXplID0gbWF6ZTtcclxuICAgIH1cclxuICAgIHB1YmxpYyBkcmF3KHA6IHA1KSB7XHJcbiAgICAgICAgZm9yIChsZXQgciA9IDA7IHIgPCB0aGlzLm1hemUublJvd3M7IHIrKykge1xyXG4gICAgICAgICAgICBmb3IgKGxldCBjID0gMDsgYyA8IHRoaXMubWF6ZS5uQ29sczsgYysrKSB7XHJcbiAgICAgICAgICAgICAgICBsZXQgY2VsbCA9IHRoaXMubWF6ZS5jZWxsKHIsIGMpO1xyXG4gICAgICAgICAgICAgICAgaWYgKGNlbGwudmlzaXRlZCkge1xyXG4gICAgICAgICAgICAgICAgICAgIGxldCBjdiA9IG5ldyBDZWxsVmlldyhjZWxsKTtcclxuICAgICAgICAgICAgICAgICAgICBjdi5kcmF3KHApO1xyXG4gICAgICAgICAgICAgICAgfVxyXG4gICAgICAgICAgICB9XHJcbiAgICAgICAgfVxyXG5cclxuICAgICAgICBpZiAodGhpcy5tYXplLmNlbGwodGhpcy5tYXplLnVwc3RhaXIucm93LCB0aGlzLm1hemUudXBzdGFpci5jb2wpLnZpc2l0ZWQpIHtcclxuICAgICAgICAgICAgbGV0IHN2ID0gbmV3IFN0YWlyVmlldyh0aGlzLm1hemUudXBzdGFpcik7XHJcbiAgICAgICAgICAgIHN2LmRyYXcocCk7XHJcbiAgICAgICAgfVxyXG4gICAgICAgIGlmICh0aGlzLm1hemUuY2VsbCh0aGlzLm1hemUuZG93bnN0YWlyLnJvdywgdGhpcy5tYXplLmRvd25zdGFpci5jb2wpLnZpc2l0ZWQpIHtcclxuICAgICAgICAgICAgbGV0IHN2ID0gbmV3IFN0YWlyVmlldyh0aGlzLm1hemUuZG93bnN0YWlyKTtcclxuICAgICAgICAgICAgc3YuZHJhdyhwKTtcclxuICAgICAgICB9XHJcbiAgICB9XHJcbn1cclxuXHJcbi8qKlxyXG4gKiBAY2xhc3MgQ2VsbFZpZXdcclxuICovXHJcbmV4cG9ydCBjbGFzcyBDZWxsVmlldyB7XHJcbiAgICBwdWJsaWMgc3RhdGljIGNlbGxXaWR0aDogbnVtYmVyID0gMzA7XHJcblxyXG4gICAgcHVibGljIHJlYWRvbmx5IGNlbGw6IENlbGw7XHJcblxyXG4gICAgY29uc3RydWN0b3IoY2VsbDogQ2VsbCkge1xyXG4gICAgICAgIHRoaXMuY2VsbCA9IGNlbGw7XHJcbiAgICB9XHJcblxyXG4gICAgcHVibGljIGRyYXcocDogcDUpIHtcclxuICAgICAgICBsZXQgdyA9IENlbGwuY2VsbFdpZHRoO1xyXG4gICAgICAgIGxldCB4ID0gdGhpcy5jZWxsLmNvbCAqIENlbGwuY2VsbFdpZHRoO1xyXG4gICAgICAgIGxldCB5ID0gdGhpcy5jZWxsLnJvdyAqIENlbGwuY2VsbFdpZHRoO1xyXG4gICAgICAgIGNvbnN0IGJnID0gJyMyMjIyMjInO1xyXG4gICAgICAgIGNvbnN0IHdhbGxDb2xvciA9ICcjRUVFRUVFJztcclxuICAgICAgICBjb25zdCBkb29yQ29sb3IgPSAnIzQ0NDQ0NCc7XHJcblxyXG4gICAgICAgIC8vIFRoZSByb29tXHJcbiAgICAgICAgcC5zdHJva2Uod2FsbENvbG9yKTtcclxuICAgICAgICBwLmZpbGwoYmcpO1xyXG4gICAgICAgIHAucmVjdCh4LCB5LCB3LCB3KVxyXG5cclxuICAgICAgICAvLyBEb29yc1xyXG4gICAgICAgIGNvbnN0IGIgPSA1O1xyXG4gICAgICAgIHAuc3Ryb2tlKGRvb3JDb2xvcik7XHJcbiAgICAgICAgaWYgKCF0aGlzLmNlbGwuYm9yZGVycy50b3ApIHtcclxuICAgICAgICAgICAgcC5saW5lKHggKyBiLCB5LCB4ICsgdyAtIGIsIHkpO1xyXG4gICAgICAgIH1cclxuICAgICAgICBpZiAoIXRoaXMuY2VsbC5ib3JkZXJzLnJpZ2h0KSB7XHJcbiAgICAgICAgICAgIHAubGluZSh4ICsgdywgeSArIGIsIHggKyB3LCB5ICsgdyAtIGIpO1xyXG4gICAgICAgIH1cclxuICAgICAgICBpZiAoIXRoaXMuY2VsbC5ib3JkZXJzLmJvdHRvbSkge1xyXG4gICAgICAgICAgICBwLmxpbmUoeCArIGIsIHkgKyB3LCB4ICsgdyAtIGIsIHkgKyB3KTtcclxuICAgICAgICB9XHJcbiAgICAgICAgaWYgKCF0aGlzLmNlbGwuYm9yZGVycy5sZWZ0KSB7XHJcbiAgICAgICAgICAgIHAubGluZSh4LCB5ICsgYiwgeCwgeSArIHcgLSBiKTtcclxuICAgICAgICB9XHJcbiAgICB9XHJcblxyXG4gICAgcHVibGljIGhpZ2hsaWdodChwOiBwNSkge1xyXG4gICAgICAgIHAubm9TdHJva2UoKTtcclxuICAgICAgICBwLmZpbGwoMjU1LCAyNTUsIDI1NSwgMjU1KTtcclxuICAgICAgICBsZXQgdyA9IENlbGwuY2VsbFdpZHRoO1xyXG4gICAgICAgIGxldCB4ID0gdGhpcy5jZWxsLmNvbCAqIENlbGwuY2VsbFdpZHRoO1xyXG4gICAgICAgIGxldCB5ID0gdGhpcy5jZWxsLnJvdyAqIENlbGwuY2VsbFdpZHRoO1xyXG4gICAgICAgIHAuZWxsaXBzZSh4ICsgdyAvIDIsIHkgKyB3IC8gMiwgdyAvIDIsIHcgLyAyKTtcclxuICAgIH1cclxufVxyXG5cclxuLyoqXHJcbiAqIFN0YWlyVmlld1xyXG4gKi9cclxuZXhwb3J0IGNsYXNzIFN0YWlyVmlldyB7XHJcbiAgICBwdWJsaWMgcmVhZG9ubHkgc3RhaXI6IFN0YWlyO1xyXG5cclxuICAgIGNvbnN0cnVjdG9yKHN0YWlyOiBTdGFpcikge1xyXG4gICAgICAgIHRoaXMuc3RhaXIgPSBzdGFpcjtcclxuICAgIH1cclxuXHJcbiAgICBwdWJsaWMgZHJhdyhwOiBwNSkge1xyXG4gICAgICAgIHAuc3Ryb2tlKDI1NSk7XHJcbiAgICAgICAgaWYgKHRoaXMuc3RhaXIudXApIHtcclxuICAgICAgICAgICAgcC5maWxsKDE5MiwgMTkyLCAxOTIpO1xyXG4gICAgICAgIH1cclxuICAgICAgICBlbHNlIHtcclxuICAgICAgICAgICAgcC5maWxsKDcwLCA3MCwgNzApO1xyXG4gICAgICAgIH1cclxuICAgICAgICBsZXQgdyA9IENlbGwuY2VsbFdpZHRoIC0gNjtcclxuICAgICAgICBsZXQgeCA9IHRoaXMuc3RhaXIuY29sICogQ2VsbC5jZWxsV2lkdGggKyAzO1xyXG4gICAgICAgIGxldCB5ID0gdGhpcy5zdGFpci5yb3cgKiBDZWxsLmNlbGxXaWR0aCArIDM7XHJcbiAgICAgICAgcC5yZWN0KHgsIHksIHcsIHcpO1xyXG4gICAgfVxyXG59XHJcblxyXG4vKipcclxuICogR2FtZVZpZXdcclxuICovXHJcbmV4cG9ydCBjbGFzcyBHYW1lVmlldyB7XHJcbiAgICBwdWJsaWMgcmVhZG9ubHkgZ2FtZTogTW9yaWFHYW1lO1xyXG5cclxuICAgIGNvbnN0cnVjdG9yKGdhbWU6IE1vcmlhR2FtZSkge1xyXG4gICAgICAgIHRoaXMuZ2FtZSA9IGdhbWU7XHJcbiAgICB9XHJcblxyXG4gICAgcHVibGljIGRyYXcocDogcDUpIHtcclxuICAgICAgICBwLmJhY2tncm91bmQoMCk7XHJcbiAgICAgICAgbGV0IG12ID0gbmV3IE1hemVWaWV3KHRoaXMuZ2FtZS5tYXplKCkpO1xyXG4gICAgICAgIG12LmRyYXcocCk7XHJcbiAgICAgICAgbGV0IGhlcm8gPSB0aGlzLmdhbWUuZ2V0SGVybygpO1xyXG4gICAgICAgIGxldCBodiA9IG5ldyBIZXJvVmlldyhoZXJvKTtcclxuICAgICAgICBodi5kcmF3KHApO1xyXG5cclxuICAgICAgICBkb2N1bWVudC5nZXRFbGVtZW50QnlJZChcIm5MZXZlbFwiKS5pbm5lckhUTUwgPSB0aGlzLmdhbWUuZ2V0TGV2ZWwoKS50b1N0cmluZygpO1xyXG4gICAgICAgIGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKFwibGlmZVwiKS5pbm5lckhUTUwgPSBoZXJvLmxpZmUudG9TdHJpbmcoKTtcclxuICAgIH1cclxufVxyXG5cclxuLyoqXHJcbiAqIEhlcm9WaWV3XHJcbiAqL1xyXG5leHBvcnQgY2xhc3MgSGVyb1ZpZXcge1xyXG4gICAgcHJpdmF0ZSBoZXJvOiBIZXJvO1xyXG5cclxuICAgIGNvbnN0cnVjdG9yKGhlcm86IEhlcm8pIHtcclxuICAgICAgICB0aGlzLmhlcm8gPSBoZXJvO1xyXG4gICAgfVxyXG5cclxuICAgIHB1YmxpYyBkcmF3KHA6IHA1KSB7XHJcbiAgICAgICAgcC5zdHJva2UoMjU1KTtcclxuICAgICAgICBpZiAodGhpcy5oZXJvLmxpZmUgPiAwKSB7XHJcbiAgICAgICAgICAgIHAuZmlsbCgwLCAyNTUsIDApO1xyXG4gICAgICAgIH1cclxuICAgICAgICBlbHNlIHtcclxuICAgICAgICAgICAgcC5maWxsKDgwLCAwLCAwKTtcclxuICAgICAgICB9XHJcbiAgICAgICAgbGV0IHggPSB0aGlzLmhlcm8ueCAqIENlbGwuY2VsbFdpZHRoICsgQ2VsbC5jZWxsV2lkdGggLyAyO1xyXG4gICAgICAgIGxldCB5ID0gdGhpcy5oZXJvLnkgKiBDZWxsLmNlbGxXaWR0aCArIENlbGwuY2VsbFdpZHRoIC8gMjtcclxuICAgICAgICBsZXQgciA9IENlbGwuY2VsbFdpZHRoIC8gMiAtIDE7XHJcbiAgICAgICAgcC5lbGxpcHNlKHgsIHksIHIsIHIpO1xyXG4gICAgfVxyXG59IiwiLy8gVGhlIG1vZHVsZSBjYWNoZVxudmFyIF9fd2VicGFja19tb2R1bGVfY2FjaGVfXyA9IHt9O1xuXG4vLyBUaGUgcmVxdWlyZSBmdW5jdGlvblxuZnVuY3Rpb24gX193ZWJwYWNrX3JlcXVpcmVfXyhtb2R1bGVJZCkge1xuXHQvLyBDaGVjayBpZiBtb2R1bGUgaXMgaW4gY2FjaGVcblx0aWYoX193ZWJwYWNrX21vZHVsZV9jYWNoZV9fW21vZHVsZUlkXSkge1xuXHRcdHJldHVybiBfX3dlYnBhY2tfbW9kdWxlX2NhY2hlX19bbW9kdWxlSWRdLmV4cG9ydHM7XG5cdH1cblx0Ly8gQ3JlYXRlIGEgbmV3IG1vZHVsZSAoYW5kIHB1dCBpdCBpbnRvIHRoZSBjYWNoZSlcblx0dmFyIG1vZHVsZSA9IF9fd2VicGFja19tb2R1bGVfY2FjaGVfX1ttb2R1bGVJZF0gPSB7XG5cdFx0Ly8gbm8gbW9kdWxlLmlkIG5lZWRlZFxuXHRcdC8vIG5vIG1vZHVsZS5sb2FkZWQgbmVlZGVkXG5cdFx0ZXhwb3J0czoge31cblx0fTtcblxuXHQvLyBFeGVjdXRlIHRoZSBtb2R1bGUgZnVuY3Rpb25cblx0X193ZWJwYWNrX21vZHVsZXNfX1ttb2R1bGVJZF0obW9kdWxlLCBtb2R1bGUuZXhwb3J0cywgX193ZWJwYWNrX3JlcXVpcmVfXyk7XG5cblx0Ly8gUmV0dXJuIHRoZSBleHBvcnRzIG9mIHRoZSBtb2R1bGVcblx0cmV0dXJuIG1vZHVsZS5leHBvcnRzO1xufVxuXG4iLCIvLyBkZWZpbmUgZ2V0dGVyIGZ1bmN0aW9ucyBmb3IgaGFybW9ueSBleHBvcnRzXG5fX3dlYnBhY2tfcmVxdWlyZV9fLmQgPSAoZXhwb3J0cywgZGVmaW5pdGlvbikgPT4ge1xuXHRmb3IodmFyIGtleSBpbiBkZWZpbml0aW9uKSB7XG5cdFx0aWYoX193ZWJwYWNrX3JlcXVpcmVfXy5vKGRlZmluaXRpb24sIGtleSkgJiYgIV9fd2VicGFja19yZXF1aXJlX18ubyhleHBvcnRzLCBrZXkpKSB7XG5cdFx0XHRPYmplY3QuZGVmaW5lUHJvcGVydHkoZXhwb3J0cywga2V5LCB7IGVudW1lcmFibGU6IHRydWUsIGdldDogZGVmaW5pdGlvbltrZXldIH0pO1xuXHRcdH1cblx0fVxufTsiLCJfX3dlYnBhY2tfcmVxdWlyZV9fLm8gPSAob2JqLCBwcm9wKSA9PiBPYmplY3QucHJvdG90eXBlLmhhc093blByb3BlcnR5LmNhbGwob2JqLCBwcm9wKSIsIi8vIGRlZmluZSBfX2VzTW9kdWxlIG9uIGV4cG9ydHNcbl9fd2VicGFja19yZXF1aXJlX18uciA9IChleHBvcnRzKSA9PiB7XG5cdGlmKHR5cGVvZiBTeW1ib2wgIT09ICd1bmRlZmluZWQnICYmIFN5bWJvbC50b1N0cmluZ1RhZykge1xuXHRcdE9iamVjdC5kZWZpbmVQcm9wZXJ0eShleHBvcnRzLCBTeW1ib2wudG9TdHJpbmdUYWcsIHsgdmFsdWU6ICdNb2R1bGUnIH0pO1xuXHR9XG5cdE9iamVjdC5kZWZpbmVQcm9wZXJ0eShleHBvcnRzLCAnX19lc01vZHVsZScsIHsgdmFsdWU6IHRydWUgfSk7XG59OyIsIi8vIHN0YXJ0dXBcbi8vIExvYWQgZW50cnkgbW9kdWxlXG5fX3dlYnBhY2tfcmVxdWlyZV9fKFwiLi9hcHAvYXBwLnRzXCIpO1xuLy8gVGhpcyBlbnRyeSBtb2R1bGUgdXNlZCAnZXhwb3J0cycgc28gaXQgY2FuJ3QgYmUgaW5saW5lZFxuIl0sInNvdXJjZVJvb3QiOiIifQ==