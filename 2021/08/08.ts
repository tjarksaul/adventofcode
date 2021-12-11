import { promises as fs } from "fs"

export interface SevenSegment {
    input: string[]
    output: string[]
}

export async function readFile() {
    return fs.readFile('08.input', { encoding: 'utf8' })
}

export function mapData(input: string): SevenSegment[] {
    return input.split('\n').map(line => {
        const [inputLine, outputLine] = line.split(' | ')
        const input = inputLine.split(' ')
        const output = outputLine.split(' ')

        return { input, output }
    })
}
