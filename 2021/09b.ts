import { mapData, readFile } from "./09"
import { findLowPoints } from "./09a"

interface Node {
    value: number
    visited: boolean
}

function getNeighborCount(input: Node[][], row: number, col: number): number {
    input[row][col] = { value: input[row][col].value, visited: true }
    let count = 0
    if (col > 0) {
        const left = input[row][col - 1]
        if (!left.visited && left.value < 9) {
            count += 1 + getNeighborCount(input, row, col - 1)
        }
    }
    if (col < input[row].length - 1) {
        const right = input[row][col + 1]
        if (!right.visited && right.value < 9) {
            count += 1 + getNeighborCount(input, row, col + 1)
        }
    }
    if (row > 0) {
        const above = input[row - 1][col]
        if (!above.visited && above.value < 9) {
            count += 1 + getNeighborCount(input, row - 1, col)
        }
    }
    if (row < input.length - 1) {
        const below = input[row + 1][col]
        if (!below.visited && below.value < 9) {
            count += 1 + getNeighborCount(input, row + 1, col)
        }
    }
    return count
}

export function part9b(input: number[][]): number {
    const lows = findLowPoints(input)
    const basins = lows.map(({ col, row }) => {
        const inputs = input.map(row => row.map(item => ({ value: item, visited: false })))
        // 1 because the LowPoint also is part of the basin
        return 1 + getNeighborCount(inputs, row, col)
    }, 0)

    return basins.sort((a, b) => b - a).slice(0, 3).reduce((carry, cur) => carry * cur, 1)
}

async function main(): Promise<void> {
    const data = await readFile()
    const input = mapData(data)

    const result = part9b(input)

    console.log(result)
}

if (require.main === module) {
    main().catch(e => console.error(e)).finally(() => process.exit(0))
}