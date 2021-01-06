import { Hero } from "./hero";

export interface Item {
    
}

export interface Flask extends Item {
    interract(hero: Hero): void;
}

export class LifeFlask implements Flask {
    public readonly content: number;

    constructor(content: number) {
        this.content = content;
    }

    public interract(hero: Hero): void {
        hero.changeLife(this.content);
    }

}