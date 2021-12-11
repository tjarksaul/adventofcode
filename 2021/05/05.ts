import { error } from "console"
import { promises as fs } from "fs"

export interface NavigationInstruction {
    x1: number
    x2: number
    y1: number
    y2: number
}

export interface InstructionSet {
    instructions: NavigationInstruction[]
    maxX: number
    maxY: number
}

export async function readFile() {
    return fs.readFile('05.input', { encoding: 'utf8' })
}

export function mapData(data: string): InstructionSet {
    let maxX = 0, maxY = 0
    const instructions = data.split('\n').map(line => {
        const parts = line.split(' -> ')

        let [x1, y1] = parts[0].split(',').map(part => parseInt(part, 10))
        let [x2, y2] = parts[1].split(',').map(part => parseInt(part, 10))

        maxX = Math.max(maxX, x1, x2)
        maxY = Math.max(maxY, y1, y2)

        return { x1, y1, x2, y2 }
    })

    return {
        instructions, maxX, maxY
    }
}

export abstract class Grid {
    rows: number[][]

    constructor(protected readonly instructionSet: InstructionSet) {
        const height = instructionSet.maxY + 1
        const width = instructionSet.maxX + 1
        this.rows = Array.from({ length: width }).map(() => Array.from({ length: height }).map(() => 0))
    }

    abstract navigate(): void

    get points(): number {
        return this.rows.reduce((total, row) =>
            row.reduce((carry, value) =>
                carry + (value < 2 ? 0 : 1), total
            ), 0)
    }

    toString(): string {
        return this.rows.map(row =>
            row.map(column =>
                column === 0 ? '.' : column
            ).join('')
        ).join('\n')
    }

}