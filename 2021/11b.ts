import { mapData, readFile, step } from "./11"

export function findSynchronisedStep(input: number[][]): number {
    let currentStep = 0
    while (true) {
        currentStep += 1
        const output = step(input)
        if (output.flashCount === 100) {
            return currentStep
        }
        input = output.octopi
    }
}

async function main(): Promise<void> {
    const data = await readFile()
    const input = mapData(data)

    const result = findSynchronisedStep(input)

    console.log(result)
}

if (require.main === module) {
    main().catch(e => console.error(e)).finally(() => process.exit(0))
}
