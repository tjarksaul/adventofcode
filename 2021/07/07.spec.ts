import { iterate as part1 } from './07a'
import { iterate as part2 } from './07b'

describe('part 1', () => {
    it('solves the example correctly', () => {
        const input = [16,1,2,0,4,2,7,1,2,14]

        const solution = part1(input)

        expect(solution.position).toEqual(2)
        expect(solution.fuel).toEqual(37)
    })
})

describe('part 2', () => {
    it('solves the example correctly', () => {
        const input = [16,1,2,0,4,2,7,1,2,14]

        const solution = part2(input)

        expect(solution.position).toEqual(5)
        expect(solution.fuel).toEqual(168)
    })
})
