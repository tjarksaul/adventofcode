import { part10a } from "./10a"
import { part10b } from "./10b"

const example =  [
    '[({(<(())[]>[[{[]{<()<>>',
    '[(()[<>])]({[<{<<[]>>(',
    '{([(<{}[<>[]}>{[]{[(<()>',
    '(((({<>}<{<{<>}{[]{[]{}',
    '[[<[([]))<([[{}[[()]]]',
    '[{[{({}]{}}([{[{{{}}([]',
    '{<[[]]>}<{[{[{[]{()[[[]',
    '[<(<(<(<{}))><([]([]()',
    '<{([([[(<>()){}]>(<<{{',
    '<{([{{}}[<[[[<>{}]]]>[]]',
]


describe('part 1', () => {
    it('solves the example correctly', () => {
        const input = example

        const solution = part10a(input)

        expect(solution).toEqual(26397)
    })
})

describe('part 2', () => {
    it('solves the example correctly', () => {
        const input = example

        const solution = part10b(input)

        expect(solution).toEqual(288957)
    })
})
