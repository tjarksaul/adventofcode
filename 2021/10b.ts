import { LineState, mapData, parseLines, readFile } from "./10"

export function part10b(input: string[]): number {
    const scores = parseLines(input)
        .filter(({ state }) => state === LineState.incomplete)
        .map(({ remainingInput }) => {
            return remainingInput.reverse().reduce((score, cur) => {
                return 5 * score + scoreFor(cur)
            }, 0)
        })
        .sort((a, b) => a - b)

    const middle = Math.floor(scores.length / 2)
    return scores[middle]
}

function scoreFor(string: string): number {
    switch (string) {
        case '(':
            return 1
        case '[':
            return 2
        case '{':
            return 3
        case '<':
            return 4
        default:
            return 0
    }
}

async function main(): Promise<void> {
    const data = await readFile()
    const input = mapData(data)

    const result = part10b(input)

    console.log(result)
}

if (require.main === module) {
    main().catch(e => console.error(e)).finally(() => process.exit(0))
}
