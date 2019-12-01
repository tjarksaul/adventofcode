import {readFileSync} from "fs";

function getFuel(mass: number): number {
    return Math.floor(mass / 3) - 2
}

function parseFile(filename: string): number[] {
    return readFileSync(filename, 'utf8').trim().split(/\r?\n/).map(line => parseInt(line, 10))
}

const result = parseFile('01-01.input').map(getFuel).reduce((prev, cur) => prev + cur, 0)

console.log({result})