import { promises as fs } from "fs"

export async function readFile() {
    return fs.readFile('10.input', { encoding: 'utf8' })
}

export function mapData(input: string): string[] {
    return input.trim().split('\n').map(line => 
        line.trim()
    )
}

export function parseLines (input: string[]): LineResult[] {
    return input.map(line => {
        const openers: string[] = []
        for (let i = 0; i < line.length; i++) {
            const item = line[i];
            
            if (item === '(' || item === '[' || item === '<' || item === '{') {
                openers.push(item)
            } else {
                if (
                    (item === ')' && openers.pop() !== '(')
                    || (item === ']' && openers.pop() !== '[')
                    || (item === '>' && openers.pop() !== '<')
                    || (item === '}' && openers.pop() !== '{')
                    ) {
                        return { state: LineState.corrupted, firstInvalidCharacter: item }
                    }
            }
        }
        if (openers.length > 0) {
            return { state: LineState.incomplete, remainingInput: openers }
        }
        return { state: LineState.valid }
    })
}

export enum LineState {
    valid,
    corrupted,
    incomplete,
}

export interface LineResult {
    state: LineState
    firstInvalidCharacter?: string
    remainingInput?: string[]
}
