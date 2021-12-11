import { mapData, readFile, LowPoint } from "./09"

export function findLowPoints(input: number[][]): LowPoint[] {
    return input.reduce((carry, row, rowIdx) => {
        const lows = row.reduce((carry, item, idx) => {
            let left = Number.MAX_SAFE_INTEGER
            if (idx > 0) {
                left = row[idx - 1]
            }
            let right = Number.MAX_SAFE_INTEGER
            if (idx < row.length - 1) {
                right = row[idx + 1]
            }
            let above = Number.MAX_SAFE_INTEGER
            if (rowIdx > 0) {
                above = input[rowIdx - 1][idx]
            }
            let below = Number.MAX_SAFE_INTEGER
            if (rowIdx < input.length - 1) {
                below = input[rowIdx + 1][idx]
            }
            if (item < left && item < right && item < above && item < below) {
                carry.push({ row: rowIdx, col: idx })
            }
            return carry
        }, [] as LowPoint[])
        return carry.concat(lows)
    }, [] as LowPoint[])
}

export function part9a(input: number[][]): number {
    return findLowPoints(input)
        .reduce((carry, { row, col }) =>
            carry + 1 + input[row][col],
            0,
        )
}

async function main(): Promise<void> {
    const data = await readFile()
    const input = mapData(data)

    const result = part9a(input)

    console.log(result)
}

if (require.main === module) {
    main().catch(e => console.error(e)).finally(() => process.exit(0))
}