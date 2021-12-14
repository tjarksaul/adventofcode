import { getFile } from "../base"

export interface PolymerTemplate {
    template: string[]
    mapping: Record<string, string>
}

export async function readFile() {
    return getFile('14/input.txt')
}

export function mapData(input: string): PolymerTemplate {
    const [templateData, mappingData] = input.trim().split('\n\n')

    const template = templateData.trim().split('')

    const mapping = mappingData.split('\n').reduce((map, line) => {
        const [src, dst] = line.trim().split(' -> ')
        map[src] = dst
        return map
    }, {})

    return {
        template,
        mapping,
    }
}
