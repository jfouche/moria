import { Maze } from "../maze"
import { RoomView, StairView } from "./cell-view";

import p5 = require('p5')

/**
 * @class MazeView
 */
export class MazeView {
    public readonly maze: Maze;

    constructor(maze: Maze) {
        this.maze = maze;
    }
    public draw(p: p5) {
        for (let r = 0; r < this.maze.nRows; r++) {
            for (let c = 0; c < this.maze.nCols; c++) {
                let cell = this.maze.cell(r, c);
                if (cell.visited) {
                    let cv = new RoomView(cell);
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
