/**
 * @class Maze
 */
export class Maze {
    public readonly nRows: number;
    public readonly nCols: number;
    public readonly width: number;
    public readonly height: number;
    private grid: Cell[][];
    public readonly upstair: Stair;
    public readonly downstair: Stair;

    constructor(nRows: number, nCols: number) {
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

    public cell(row: number, col: number) {
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
            var r = random(0, neighbors.length);
            next = neighbors[r];
        }
        return next;
    }

    private removeWallsBetween(a: Cell, b: Cell) {
        if (a.col > b.col) {
            a.borders.left = false;
            b.borders.right = false;
        } else if (a.col < b.col) {
            a.borders.right = false;
            b.borders.left = false;
        } else if (a.row > b.row) {
            a.borders.top = false;
            b.borders.bottom = false;
        } else if (a.row < b.row) {
            a.borders.bottom = false;
            b.borders.top = false;
        }
    }

    private removeRandomWalls(maze: Maze, n: number) {
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

/**
 * @class CellBorders
 */
export class CellBorders {
    top: boolean = true;
    right: boolean = true;
    bottom: boolean = true;
    left: boolean = true;
}

/**
 * @class Cell
 */
export class Cell {
    public static cellWidth: number = 30;

    public readonly row: number;
    public readonly col: number;

    public borders: CellBorders;
    public visited: boolean = false;

    constructor(row: number, col: number) {
        this.row = row;
        this.col = col;
        this.borders = new CellBorders();
    }
}

/**
 * Stair
 */
export class Stair {
    public readonly row: number;
    public readonly col: number;
    public readonly up: boolean;

    private constructor(row: number, col: number, up: boolean) {
        this.row = row;
        this.col = col;
        this.up = up;
    }

    public static upstair(row: number, col: number) {
        return new Stair(row, col, true);
    }

    public static downstairstair(row: number, col: number) {
        return new Stair(row, col, false);
    }
}

function random(min: number, max: number) {
    return Math.floor(min + Math.random() * (max - min + 1));
}