import { promises as fs } from "fs"

async function readFile() {
    return fs.readFile('03.input', { encoding: 'utf8' })
}

function mapData(data: string): string[][] {
    const lines = data.split('\n')

    return lines.map(line => [...line])
}

async function calculateCounts(data: string[][], position: number, keepLessCommon: boolean): Promise<string[]> {
    if (data.length === 1) {
        return data[0]
    }

    const ones: string[][] = []
    const zeros: string[][] = []

    data.forEach(line => {
        if (line[position] === '1') {
            ones.push(line)
        } else {
            zeros.push(line)
        }
    })

    let mostCommon: string[][]
    let lessCommon: string[][]

    if (ones.length >= zeros.length) {
        mostCommon = ones
        lessCommon = zeros
    } else {
        lessCommon = ones
        mostCommon = zeros
    }

    if (keepLessCommon) {
        return calculateCounts(lessCommon, position + 1, keepLessCommon)
    } else {
        return calculateCounts(mostCommon, position + 1, keepLessCommon)
    }
}

function calculateResult(oxyRating: string[], co2Rating: string[]): number {
    const oxyValue = parseInt(oxyRating.join(''), 2)
    const co2Value = parseInt(co2Rating.join(''), 2)

    return oxyValue * co2Value
}

async function main() {
    const file = await readFile()
    const mapped = mapData(file)
    const [oxyValue, co2Value] = await Promise.all([
        calculateCounts(mapped, 0, false),
        calculateCounts(mapped, 0, true),
    ])
    console.log({ oxyValue, co2Value })
    const result = calculateResult(oxyValue, co2Value)

    console.log({ result })
}

main().finally(() => process.exit(0))