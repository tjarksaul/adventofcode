import { promises as fs } from "fs"

export async function readFile() {
    return fs.readFile('09.input', { encoding: 'utf8' })
}

export function mapData(input: string): number[][] {
    return input.trim().split('\n').map(line => 
        line.trim().split('').map(item => parseInt(item, 10))
    )
}

export interface LowPoint {
    row: number
    col: number
}