import { mapData, readFile, SevenSegment } from "./08"

export function count(input: string[]): number {
    const numbers = [2, 4, 3, 7]

    return input.reduce((carry, cur) => {
        if (numbers.some(num => num === cur.length)) {
            return carry + 1
        }
        return carry
    }, 0)
}

function fitData(data: SevenSegment[]): string[] {
    return data.flatMap(({ output }) => output)
}

async function main(): Promise<void> {
    const data = await readFile()
    const input = mapData(data)
    const outputs = fitData(input)

    const result = count(outputs)

    console.log(result)
}

if (require.main === module) {
    main().catch(e => console.error(e)).finally(() => process.exit(0))
}