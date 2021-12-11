import { promises as fs } from "fs"

type Direction = 'forward' | 'down' | 'up'

interface Position {
    depth: number
    horizontal: number
}

interface NavigationInstruction {
    direction: Direction
    steps: number
}

async function readFile() {
    return fs.readFile('02.input', { encoding: 'utf8' })
}

function mapData(data: string): NavigationInstruction[] {
    return data.split('\n').map(line => {
        const parts = line.split(' ')

        // todo: check for invalid directions here
        const direction = parts[0] as Direction
        const steps = parseInt(parts[1], 10)

        return { direction, steps }
    })
}

function navigate({ horizontal, depth }: Position, { direction, steps }: NavigationInstruction): Position {
    switch (direction) {
        case 'forward':
            return { depth, horizontal: horizontal + steps }
        case 'down':
            return { horizontal, depth: depth + steps }
        case 'up':
            return { horizontal, depth: depth - steps }
    }
}

async function main() {
    const data = await readFile()
    const instructions = mapData(data)

    const initialPosition: Position = { horizontal: 0, depth: 0 }
    const result = instructions.reduce((position, instruction) => navigate(position, instruction), initialPosition)

    const final = result.depth * result.horizontal

    console.log({ ...result, final })
}

main().finally(() => process.exit(0))