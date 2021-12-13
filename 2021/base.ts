import { promises as fs } from "fs"

export async function getFile(file: string): Promise<string> {
    return fs.readFile(file, { encoding: 'utf8' })
}
