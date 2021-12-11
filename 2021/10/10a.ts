import { LineState, mapData, parseLines, readFile } from "./10";

export function part10a(input: string[]): number {
    return parseLines(input)
        .filter(({ state }) => state === LineState.corrupted)
        .map(({ firstInvalidCharacter }) => {
            switch (firstInvalidCharacter) {
                case ')':
                    return 3
                case ']':
                    return 57
                case '}':
                    return 1197
                case '>':
                    return 25137
                default:
                    return 0
            }
        }).reduce((carry, cur) => carry + cur, 0)
}

async function main(): Promise<void> {
    const data = await readFile()
    const input = mapData(data)

    const result = part10a(input)

    console.log(result)
}

if (require.main === module) {
    main().catch(e => console.error(e)).finally(() => process.exit(0))
}
