import { countFlashes } from "./11a"
import { findSynchronisedStep } from "./11b"

const example = [
    [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
    [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
    [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
    [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
    [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
    [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
    [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
    [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
    [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
    [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
]


describe('part 1', () => {
    it('solves the example correctly', () => {
        const input = example

        const solution = countFlashes(input, 100)

        expect(solution).toEqual(1656)
    })
})

describe('part 1', () => {
    it('solves the example correctly', () => {
        const input = example

        const solution = findSynchronisedStep(input)

        expect(solution).toEqual(195)
    })
})
