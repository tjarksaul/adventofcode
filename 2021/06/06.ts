import { promises as fs } from "fs"

export async function readFile() {
    return fs.readFile('06.input', { encoding: 'utf8' })
}

export function mapData(data: string): number[] {
    return data.split(',').map(val => parseInt(val, 10))
}
