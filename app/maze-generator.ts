import { LifeFlask } from "./item";
import { Maze, Room } from "./maze";

/**
 * @class MazeGenerator
 */
export class MazeGenerator {

    public newMaze(nRows: number, nCols: number): Maze {
        let maze = new Maze(nRows, nCols);
        let backtracking: Room[] = [];
        let currentCell = maze.cell(0, 0);
        currentCell.visit();
        let finished = false;
        while (!finished) {
            let next = this.getNextNeighbor(maze, currentCell);
            if (next) {
                next.visit();
                backtracking.push(currentCell);
                this.removeWallsBetween(currentCell, next);
                currentCell = next;
            } else if (backtracking.length > 0) {
                next = backtracking.pop();
                currentCell = next;
            } else {
                finished = true;
            }
        }

        maze.clear();

        // add an item 
        const item = new LifeFlask(50);
        const r = random(1, maze.nRows - 2);
        const c = random(1, maze.nCols - 2);
        maze.cell(r, c).item = item;


        // remove some random walls 
        this.removeRandomWalls(maze, 8);

        return maze;
    }

    private getNextNeighbor(maze: Maze, cell: Room): Room {
        let neighbors: Room[] = [];
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

        let next: Room = undefined;
        if (neighbors.length > 0) {
            var r = random(0, neighbors.length);
            next = neighbors[r];
        }
        return next;
    }

    private removeWallsBetween(a: Room, b: Room) {
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
                        i++;
                    }
                    break;

                case 1:
                    if (cell.borders.right) {
                        this.removeWallsBetween(cell, maze.cell(r, c + 1));
                        i++;
                    }
                    break;

                case 2:
                    if (cell.borders.bottom) {
                        this.removeWallsBetween(cell, maze.cell(r + 1, c));
                        i++;
                    }
                    break;

                case 3:
                    if (cell.borders.left) {
                        this.removeWallsBetween(cell, maze.cell(r, c - 1));
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
 * A shortcut to get a random number between `min` and `max`
 * @param min 
 * @param max 
 */
function random(min: number, max: number) {
    if (min > max) {
        [min, max] = [max, min]
    }
    return Math.floor(Math.random() * (max - min) + min);
}
