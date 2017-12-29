/******/ (function(modules) { // webpackBootstrap
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId])
/******/ 			return installedModules[moduleId].exports;
/******/
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			exports: {},
/******/ 			id: moduleId,
/******/ 			loaded: false
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.loaded = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(0);
/******/ })
/************************************************************************/
/******/ ([
/* 0 */
/*!********************!*\
  !*** ./app/app.ts ***!
  \********************/
/***/ (function(module, exports, __webpack_require__) {

	"use strict";
	Object.defineProperty(exports, "__esModule", { value: true });
	var maze_1 = __webpack_require__(/*! ./maze */ 1);
	var game;
	function setup() {
	    game = new MoriaGame(8, 10, 5);
	    var canvas = createCanvas(game.width, game.height);
	    canvas.parent('game');
	    frameRate(10);
	}
	function draw() {
	    background(0);
	    game.draw();
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
	var MoriaGame = (function () {
	    function MoriaGame(nRows, nCols, nLevels) {
	        this.nRows = nRows;
	        this.nCols = nCols;
	        this.mazes = [];
	        for (var i = 0; i < nLevels; i++) {
	            this.mazes.push(maze_1.createMaze(this.nRows, this.nCols));
	        }
	        this.currentLevel = 0;
	        var maze = this.maze();
	        this.initLevel();
	    }
	    MoriaGame.prototype.getLevel = function () {
	        return this.currentLevel;
	    };
	    MoriaGame.prototype.initLevel = function () {
	        var maze = this.maze();
	        this.hero = new Hero(maze.upstair.col, maze.upstair.row);
	        maze.cell(this.hero.y, this.hero.x).visit();
	        this.checkVisibility();
	    };
	    MoriaGame.prototype.maze = function () {
	        return this.mazes[this.currentLevel];
	    };
	    MoriaGame.prototype.draw = function () {
	        background(0);
	    };
	    MoriaGame.prototype.moveHero = function (direction) {
	        if (this.canMove(direction)) {
	            this.hero.move(direction);
	            this.maze().cell(this.hero.y, this.hero.x).visit();
	            if (this.hero.x === this.maze().downstair.col && this.hero.y === this.maze().downstair.row) {
	                this.currentLevel++;
	                this.initLevel();
	            }
	            this.checkVisibility();
	        }
	    };
	    MoriaGame.prototype.canMove = function (direction) {
	        var cellBorders = this.maze().cell(this.hero.y, this.hero.x).borders;
	        return (direction === 3 && !cellBorders.E)
	            || (direction === 2 && !cellBorders.W)
	            || (direction === 0 && !cellBorders.N)
	            || (direction === 1 && !cellBorders.S);
	    };
	    MoriaGame.prototype.checkVisibility = function () {
	        var _this = this;
	        var x;
	        var y;
	        var cell;
	        var maze = this.maze();
	        var reset = function () {
	            x = _this.hero.x;
	            y = _this.hero.y;
	            cell = maze.cell(y, x);
	        };
	        var next = function () {
	            cell = maze.cell(y, x);
	            cell.visit();
	        };
	        reset();
	        while (!cell.borders.N) {
	            y -= 1;
	            next();
	        }
	        reset();
	        while (!cell.borders.E) {
	            x += 1;
	            next();
	        }
	        reset();
	        while (!cell.borders.S) {
	            y += 1;
	            next();
	        }
	        reset();
	        while (!cell.borders.W) {
	            x -= 1;
	            next();
	        }
	    };
	    return MoriaGame;
	}());
	exports.MoriaGame = MoriaGame;
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


/***/ }),
/* 1 */
/*!*********************!*\
  !*** ./app/maze.ts ***!
  \*********************/
/***/ (function(module, exports) {

	"use strict";
	Object.defineProperty(exports, "__esModule", { value: true });
	function createMaze(nRows, nCols) {
	    var mazeGen = new detail.MazeGenerator();
	    return mazeGen.newMaze(nRows, nCols);
	}
	exports.createMaze = createMaze;
	var detail;
	(function (detail) {
	    var Maze = (function () {
	        function Maze(nRows, nCols) {
	            this.nRows = nRows;
	            this.nCols = nCols;
	            this.grid = [];
	            for (var r = 0; r < this.nRows; r++) {
	                this.grid[r] = [];
	                for (var c = 0; c < this.nCols; c++) {
	                    this.grid[r][c] = new Cell(r, c);
	                }
	            }
	            this.upstair = new Stair(0, 0, true);
	            this.downstair = new Stair(nRows - 1, nCols - 1, false);
	        }
	        Maze.prototype.cell = function (row, col) {
	            return this.grid[row][col];
	        };
	        return Maze;
	    }());
	    var MazeGenerator = (function () {
	        function MazeGenerator() {
	        }
	        MazeGenerator.prototype.newMaze = function (nRows, nCols) {
	            var maze = new Maze(nRows, nCols);
	            var backtracking = [];
	            var currentCell = maze.cell(0, 0);
	            currentCell.visited = true;
	            var finished = false;
	            while (!finished) {
	                var next = this.getNextNeighbor(maze, currentCell);
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
	        };
	        MazeGenerator.prototype.getNextNeighbor = function (maze, cell) {
	            var neighbors = [];
	            if (cell.row > 0) {
	                var left = maze.cell(cell.row - 1, cell.col);
	                if (!left.visited) {
	                    neighbors.push(left);
	                }
	            }
	            if (cell.row < maze.nRows - 1) {
	                var right = maze.cell(cell.row + 1, cell.col);
	                if (!right.visited) {
	                    neighbors.push(right);
	                }
	            }
	            if (cell.col > 0) {
	                var top_1 = maze.cell(cell.row, cell.col - 1);
	                if (!top_1.visited) {
	                    neighbors.push(top_1);
	                }
	            }
	            if (cell.col < maze.nCols - 1) {
	                var bottom = maze.cell(cell.row, cell.col + 1);
	                if (!bottom.visited) {
	                    neighbors.push(bottom);
	                }
	            }
	            var next = undefined;
	            if (neighbors.length > 0) {
	                var r = floor(random(0, neighbors.length));
	                next = neighbors[r];
	            }
	            return next;
	        };
	        MazeGenerator.prototype.removeWallsBetween = function (a, b) {
	            if (a.col > b.col) {
	                a.borders.W = false;
	                b.borders.E = false;
	            }
	            else if (a.col < b.col) {
	                a.borders.E = false;
	                b.borders.W = false;
	            }
	            else if (a.row > b.row) {
	                a.borders.N = false;
	                b.borders.S = false;
	            }
	            else if (a.row < b.row) {
	                a.borders.S = false;
	                b.borders.N = false;
	            }
	        };
	        MazeGenerator.prototype.removeRandomWalls = function (maze, n) {
	            for (var i = 0; i < n;) {
	                var r = floor(random(1, maze.nRows - 2));
	                var c = floor(random(1, maze.nCols - 2));
	                var cell = maze.cell(r, c);
	                var next = floor(random(0, 3));
	                switch (next) {
	                    case 0:
	                        if (cell.borders.N) {
	                            this.removeWallsBetween(cell, maze.cell(r - 1, c));
	                            console.log("remove (%d, %d) : top", c, r);
	                            i++;
	                        }
	                        break;
	                    case 1:
	                        if (cell.borders.E) {
	                            this.removeWallsBetween(cell, maze.cell(r, c + 1));
	                            console.log("remove (%d, %d) : right", c, r);
	                            i++;
	                        }
	                        break;
	                    case 2:
	                        if (cell.borders.S) {
	                            this.removeWallsBetween(cell, maze.cell(r + 1, c));
	                            console.log("remove (%d, %d) : bottom", c, r);
	                            i++;
	                        }
	                        break;
	                    case 3:
	                        if (cell.borders.W) {
	                            this.removeWallsBetween(cell, maze.cell(r, c - 1));
	                            console.log("remove (%d, %d) : left", c, r);
	                            i++;
	                        }
	                        break;
	                    default:
	                        break;
	                }
	            }
	        };
	        return MazeGenerator;
	    }());
	    detail.MazeGenerator = MazeGenerator;
	    var CellBorders = (function () {
	        function CellBorders() {
	            this.N = true;
	            this.E = true;
	            this.S = true;
	            this.W = true;
	        }
	        return CellBorders;
	    }());
	    var Cell = (function () {
	        function Cell(row, col) {
	            this.visited = false;
	            this.row = row;
	            this.col = col;
	            this.borders = new CellBorders();
	        }
	        Cell.prototype.visit = function () {
	            this.visited = true;
	        };
	        return Cell;
	    }());
	    var Stair = (function () {
	        function Stair(row, col, up) {
	            this.row = row;
	            this.col = col;
	            this.up = up;
	        }
	        return Stair;
	    }());
	    detail.Stair = Stair;
	})(detail || (detail = {}));


/***/ })
/******/ ]);
//# sourceMappingURL=app.bundle.js.map