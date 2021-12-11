import { promises as fs } from "fs"

export interface Octopus {
    energy: number
    hasFlashed: boolean
}

export interface Output {
    octopi: number[][]
    flashCount: number
}

export async function readFile() {
    return fs.readFile('11.input', { encoding: 'utf8' })
}

export function mapData(input: string): number[][] {
    return input.trim().split('\n').map(line => 
        line.trim().split('').map(item => parseInt(item, 10))
    )
}

export function step (input: number[][]): Output {
    let octopi = input.map(row => row.map(energy => ({ energy, hasFlashed: false })))

    let flashCount = 0
    for (let rowIdx = 0; rowIdx < octopi.length; rowIdx++) {
        const row = octopi[rowIdx];
        for (let j = 0; j < row.length; j++) {
            const item = row[j];
            item.energy += 1
        }
    }

    for (let rowIdx = 0; rowIdx < octopi.length; rowIdx++) {
        const row = octopi[rowIdx];
        for (let j = 0; j < row.length; j++) {
            const item = row[j];
            
            if (item.energy > 9) {
                octopi = flash(octopi, rowIdx, j)
            }
        }
    }

    for (let i = 0; i < octopi.length; i++) {
        const row = octopi[i];
        for (let j = 0; j < row.length; j++) {
            const item = row[j];
            if (item.hasFlashed) {
                flashCount += 1
                item.energy = 0
            }
        }
    }

    return {
        flashCount,
        octopi: octopi.map(row => row.map(({ energy }) => energy))
    }
}

export function flash(input: Octopus[][], row: number, column: number): Octopus[][] {
    const item = input[row][column]
    if (item.hasFlashed) {
        return input
    }
    item.hasFlashed = true
    const indexes = [
        [row - 1, column - 1],
        [row - 1, column],
        [row - 1, column + 1],
        [row, column - 1],
        [row, column + 1],
        [row + 1, column - 1],
        [row + 1, column],
        [row + 1, column + 1],
    ].filter(([r, c]) => r >= 0 && r <= 9 && c >= 0 && c <= 9)

    for (let i = 0; i < indexes.length; i++) {
        const [nr, nc] = indexes[i];
        const neighbor = input[nr][nc]
        neighbor.energy += 1
        if (neighbor.energy > 9) {
            input = flash(input, nr, nc)
        }
    }
    
    return input
}
