import { part9a } from "./09a"
import { part9b } from "./09b"

const example =  [
    [2,1,9,9,9,4,3,2,1,0],
    [3,9,8,7,8,9,4,9,2,1],
    [9,8,5,6,7,8,9,8,9,2],
    [8,7,6,7,8,9,6,7,8,9],
    [9,8,9,9,9,6,5,6,7,8],
]


describe('part 1', () => {
    it('solves the example correctly', () => {
        const input = example

        const solution = part9a(input)

        expect(solution).toEqual(15)
    })
})

describe('part 2', () => {
    it('solves the example correctly', () => {
        const input = example
        
        const solution = part9b(input)

        expect(solution).toEqual(1134)
    })
})