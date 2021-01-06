import { Item } from './item'

/**
 * @class Maze
 */
export class Maze {
    public readonly nRows: number;
    public readonly nCols: number;
    private grid: Room[][];
    public readonly upstair: Stair;
    public readonly downstair: Stair;

    constructor(nRows: number, nCols: number) {
        this.nRows = nRows;
        this.nCols = nCols;
        this.grid = [];

        for (var r = 0; r < this.nRows; r++) {
            this.grid[r] = [];
            for (var c = 0; c < this.nCols; c++) {
                this.grid[r][c] = new Room(r, c);
            }
        }
        this.upstair = Stair.upstair(0, 0);
        this.downstair = Stair.downstairstair(nRows - 1, nCols - 1);
    }

    public cell(row: number, col: number) {
        return this.grid[row][col];
    }

    clear() {
        this.grid.map((row) => {
            row.map((cell) => {
                cell.clear();
            })
        })
    }

    visit(cell: Cell) {
        this.cell(cell.row, cell.col).visit();
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
    public readonly row: number;
    public readonly col: number;

    constructor(row: number, col: number) {
        this.row = row;
        this.col = col;
    }
}

/**
 * @class Room
 */
export class Room extends Cell {
    public borders: CellBorders;
    public visited: boolean = false;
    public item: Item = undefined;

    constructor(row: number, col: number) {
        super(row, col);
        this.borders = new CellBorders();
    }

    visit() {
        this.visited = true;
    }

    clear() {
        this.visited = false;
    }
}

/**
 * Stair
 */
export class Stair extends Cell {
    public readonly up: boolean;

    private constructor(row: number, col: number, up: boolean) {
        super(row, col);
        this.up = up;
    }

    public static upstair(row: number, col: number) {
        return new Stair(row, col, true);
    }

    public static downstairstair(row: number, col: number) {
        return new Stair(row, col, false);
    }
}
