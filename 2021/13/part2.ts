import { fold, mapData, PaperData, readFile, setupPaper, visualizePaper } from "./base"

export function getResult(input: PaperData): string {
    let paper = setupPaper(input.dots)

    const { instructions } = input

    paper = fold(paper, instructions)

    return visualizePaper(paper)
}

async function main(): Promise<void> {
    const data = await readFile()
    const input = mapData(data)

    const result = getResult(input)

    console.log(result)
}

if (require.main === module) {
    main().catch(e => console.error(e)).finally(() => process.exit(0))
}
