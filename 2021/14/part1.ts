import { mapData, readFile } from "./base"

export function createPolymer(template: string[], mapping: Record<string, string>, steps: number): string[] {
    if (steps === 0) {
        return template
    }

    const polymer = [...template]
    template.forEach((item, idx, template) => {
        if (idx === template.length - 1) {
            return
        }
        const next = template[idx + 1]
        const pair = `${item}${next}`
        const insert = mapping[pair]

        if (!insert) {
            throw Error("This shouldn't really happen. 🤔")
        }

        const position = idx + idx + 1
        polymer.splice(position, 0, insert)
    })

    return createPolymer(polymer, mapping, steps - 1)
}

export function countResult(polymer: string[]): number {
    const counts = polymer.reduce((counts, cur) => {
        if (!counts[cur]) {
            counts[cur] = 0
        }
        counts[cur] = counts[cur] + 1
        return counts
    }, {} as Record<string, number>)

    const [max, min] = Object.keys(counts).reduce(([max, min], cur) =>{
        // console.log(`${cur}: ${counts[cur]}`)
        return [
            Math.max(max, counts[cur]),
            Math.min(min, counts[cur])
        ]},
        [0, Number.MAX_SAFE_INTEGER]
    )

    return max - min
}

async function main(): Promise<void> {
    const data = await readFile()
    const { template, mapping } = mapData(data)

    const polymer = createPolymer(template, mapping, 10)
    const result = countResult(polymer)

    console.log(result)
}

if (require.main === module) {
    main().catch(e => console.error(e)).finally(() => process.exit(0))
}
