import { fold, mapData, PaperData, readFile, setupPaper } from "./base";

export function countDots(input: PaperData, steps: number | undefined): number {
    let paper = setupPaper(input.dots)

    let { instructions } = input
    if (steps) {
        instructions = instructions.slice(0, steps)
    }

    paper = fold(paper, instructions)

    return count(paper)
}

function count(paper: boolean[][]): number {
    return paper.reduce((carry, row) => row.reduce((carry, item) => carry + (item ? 1 : 0), carry), 0)
}

async function main(): Promise<void> {
    const data = await readFile()
    const input = mapData(data)

    const result = countDots(input, 1)

    console.log(result)
}

if (require.main === module) {
    main().catch(e => console.error(e)).finally(() => process.exit(0))
}
