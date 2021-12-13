import { getFile } from "../base"

export interface PaperData {
    dots: Dot[]
    instructions: FoldingInstruction[]
}

export interface Dot {
    x: number
    y: number
}

export type Axis = 'x' | 'y'

export interface FoldingInstruction {
    axis: Axis
    position: number
}

export async function readFile() {
    return getFile('13/input.txt')
}

export function mapData(input: string): PaperData {
    const [dotData, instructionData] = input.trim().split('\n\n')

    const dots = dotData.split('\n').map(line => {
        const [x, y] = line.trim().split(',').map(coordinate => parseInt(coordinate, 10))
        return { x, y }
    })
    const instructions = instructionData.split('\n').map(line => {
        const data = line.trim().split(' ').pop()
        const [axis, position] = data.split('=')
        return {
            axis: axis as Axis,
            position: parseInt(position, 10),
        }
    })

    return {
        dots,
        instructions,
    }
}

export function setupPaper(dots: Dot[]): boolean[][] {
    const [x, y] = dots.reduce(([maxX, maxY], { x, y }) => [Math.max(maxX, x), Math.max(maxY, y)], [0, 0])

    const paper = Array.from({ length: y + 1 }).map(() => Array.from({ length: x + 1 }).map(() => false))

    for (let i = 0; i < dots.length; i++) {
        const { x, y } = dots[i];
        paper[y][x] = true
    }

    return paper
}

export function fold(paper: boolean[][], instructions: FoldingInstruction[]): boolean[][] {
    return instructions.reduce((paper, { axis, position }) => {
        console.log(`fold along ${axis}=${position}`)
        if (axis === 'y') {
            for (let i = position + 1; i < paper.length; i++) {
                const row = paper[i];
                for (let j = 0; j < row.length; j++) {
                    const item = row[j];
                    if (item) {
                        paper[paper.length - i - 1][j] = true
                    }
                }
            }
            paper = paper.slice(0, position)
        } else {
            for (let i = 0; i < paper.length; i++) {
                const row = paper[i];
                for (let j = position + 1; j < row.length; j++) {
                    const item = row[j];
                    if (item) {
                        row[row.length - j - 1] = true
                    }
                }
                paper[i] = row.slice(0, position)
            }
        }
        return paper
    }, paper)
}

export function visualizePaper(paper: boolean[][]): string {
    let output = ''
    for (let i = 0; i < paper.length; i++) {
        const row = paper[i];
        for (let j = 0; j < row.length; j++) {
            const item = row[j];
            output += item ? '#' : '.'
        }
        output += '\n'
    }

    return output.trim()
}