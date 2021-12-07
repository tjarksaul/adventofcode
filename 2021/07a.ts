import { mapData, readFile, OptimalPosition } from "./07"

export function iterate(positions: number[]): OptimalPosition {
    const max = Math.max(...positions)
    let best: OptimalPosition | undefined

    for (let i = 0; i < max; i++) {
        const fuel = positions.reduce((carry, cur) => carry + Math.abs(i - cur), 0)

        if (fuel < (best?.fuel || Number.MAX_SAFE_INTEGER)) {
            best = { position: i, fuel }
        }
    }

    return best
}

async function main(): Promise<void> {
    const data = await readFile()
    const input = mapData(data)

    const result = iterate(input)

    console.log(result)
}

if (require.main === module) {
    main().catch(e => console.error(e)).finally(() => process.exit(0))
}