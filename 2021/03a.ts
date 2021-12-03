import { promises as fs } from "fs"

async function readFile() {
    return fs.readFile('03.input', { encoding: 'utf8' })
}

function calculateCounts(data: string): string {
    const lines = data.split('\n')
    const bits = lines[0].length
    const counts = Array.from({length: bits}, () => 0)

    lines.forEach(line => {
        [...line].forEach((char, i) => {
            const num = parseInt(char, 10)
            counts[i] += num
        })
    })

    return counts.reduce((prev, cur) => {
        const val = cur > (lines.length / 2) ? 1 : 0
        return `${prev}${val}`
    },'')
}

function calculateResult(counts: string): number {
    const gamma = parseInt(counts, 2)
    const epsilonString = [...counts].map(char => char === '1' ? '0': '1').join('')
    const epsilon = parseInt(epsilonString, 2)

    return gamma * epsilon
}

async function main() {
    const file = await readFile()
    const counts = calculateCounts(file)
    console.log({ counts })
    const result = calculateResult(counts)

    console.log({ result })
}

main().finally(() => process.exit(0))