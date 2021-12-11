import { mapData, Output, readFile, step } from "./11"

export function countFlashes(input: number[][], steps: number): number {
    const output = Array.from({ length: steps }).reduce(({ octopi, flashCount }) => {
        const output = step(octopi)
        return { octopi: output.octopi, flashCount: flashCount + output.flashCount }
    }, { octopi: input, flashCount: 0 })

    return (output as Output).flashCount
}

async function main(): Promise<void> {
    const data = await readFile()
    const input = mapData(data)

    const result = countFlashes(input, 100)

    console.log(result)
}

if (require.main === module) {
    main().catch(e => console.error(e)).finally(() => process.exit(0))
}
