import { promises as fs } from "fs"

async function readFile() {
    return fs.readFile('01.input', { encoding: 'utf8' })
}

function mapData(data: string): number[] {
    return data.split('\n').map(line => parseInt(line, 10))
}

function calculateResult(data: number[]): number {
    let count = 0

    for (let i = 3; i < data.length; i++) {
        const previous = data[i-3] + data[i-2] + data[i-1]
        const current = data[i-2] + data[i-1] + data[i]

        if (current > previous) {
            count += 1
        }
    }

    return count
}

async function main() {
    const file = await readFile()
    const mapped = mapData(file)
    const result = calculateResult(mapped)

    console.log({ result })
}

main().finally(() => process.exit(0))