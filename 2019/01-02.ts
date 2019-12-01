import { readFileSync } from 'fs'

function calculateFuel(mass: number): number {
    return Math.floor(mass / 3) - 2
}

function getFuel(mass: number): number {
    let fuel = 0
    do {
        mass = calculateFuel(mass)
        if (mass > 0) {
            fuel += mass
        }
    } while (mass > 0)
    return fuel
}

function parseFile(filename: string): number[] {
    return readFileSync(filename, 'utf8').trim().split(/\r?\n/).map(line => parseInt(line, 10))
}

let result = parseFile('01-01.input').map(getFuel).reduce((prev, cur) => prev + cur, 0)

console.log({ result })