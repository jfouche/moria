/**
 * @class Maze
 */
export interface Maze {
    readonly nRows: number;
    readonly nCols: number;
    readonly upstair: Stair;
    readonly downstair: Stair;

    cell(row: number, col: number): Cell;
}

/**
 * @class CellBorders
 */
export interface CellBorders {
    readonly N: boolean;
    readonly S: boolean;
    readonly E: boolean;
    readonly W: boolean;
}

/**
 * @class Cell
 */
export interface Cell {
    readonly row: number;
    readonly col: number;
    borders: CellBorders;
    readonly visited: boolean;

    visit(): void;
}

/**
 * Stair
 */
export interface Stair {
    readonly row: number;
    readonly col: number;
    readonly up: boolean;
}

/**
 * 
 * @param nRows 
 * @param nCols 
 */
export function createMaze(nRows: number, nCols: number): Maze {
    let mazeGen = new detail.MazeGenerator();
    return mazeGen.newMaze(nRows, nCols);
}

namespace detail {

    /**
     * @class Maze
     */
    class Maze {
        public readonly nRows: number;
        public readonly nCols: number;
        private grid: Cell[][];
        public readonly upstair: Stair;
        public readonly downstair: Stair;

        constructor(nRows: number, nCols: number) {
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

        public cell(row: number, col: number): Cell {
            return this.grid[row][col];
        }
    }

    /**
     * @class MazeGenerator
     */
    export class MazeGenerator {

        public newMaze(nRows: number, nCols: number): Maze {
            let maze = new Maze(nRows, nCols);
            let backtracking: Cell[] = [];
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
                } else if (backtracking.length > 0) {
                    next = backtracking.pop();
                    currentCell = next;
                } else {
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

        private getNextNeighbor(maze: Maze, cell: Cell): Cell {
            let neighbors: Cell[] = [];
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

            let next: Cell = undefined;
            if (neighbors.length > 0) {
                var r = floor(random(0, neighbors.length));
                next = neighbors[r];
            }
            return next;
        }

        private removeWallsBetween(a: Cell, b: Cell) {
            if (a.col > b.col) {
                a.borders.W = false;
                b.borders.E = false;
            } else if (a.col < b.col) {
                a.borders.E = false;
                b.borders.W = false;
            } else if (a.row > b.row) {
                a.borders.N = false;
                b.borders.S = false;
            } else if (a.row < b.row) {
                a.borders.S = false;
                b.borders.N = false;
            }
        }

        private removeRandomWalls(maze: Maze, n: number) {
            for (let i = 0; i < n;) {
                let r = floor(random(1, maze.nRows - 2));
                let c = floor(random(1, maze.nCols - 2));

                let cell = maze.cell(r, c);
                let next = floor(random(0, 3));
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
        }
    }

    /**
     * @class CellBorders
     */
    class CellBorders {
        N: boolean = true;
        E: boolean = true;
        S: boolean = true;
        W: boolean = true;
    }

    /**
     * @class Cell
     */
    class Cell {
        public readonly row: number;
        public readonly col: number;

        public borders: CellBorders;
        public visited: boolean = false;

        constructor(row: number, col: number) {
            this.row = row;
            this.col = col;
            this.borders = new CellBorders();
        }

        public visit() {
            this.visited = true;
        }
    }

    /**
     * Stair
     */
    export class Stair {
        public readonly row: number;
        public readonly col: number;
        public readonly up: boolean;

        constructor(row: number, col: number, up: boolean) {
            this.row = row;
            this.col = col;
            this.up = up;
        }

    }
}