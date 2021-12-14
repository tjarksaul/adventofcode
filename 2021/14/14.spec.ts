import { countResult, createPolymer } from "./part1"

const template = ['N', 'N', 'C', 'B']
const mappings: Record<string, string> = {
    CH: 'B',
    HH: 'N',
    CB: 'H',
    NH: 'C',
    HB: 'C',
    HC: 'B',
    HN: 'C',
    NN: 'C',
    BH: 'H',
    NC: 'B',
    NB: 'B',
    BN: 'B',
    BB: 'N',
    BC: 'B',
    CC: 'N',
    CN: 'C',
}

describe('part 1', () => {
    it('should solve the example correctly', () => {
        const polymer = createPolymer(template, mappings, 10)

        console.log(polymer)

        const result = countResult(polymer)

        expect(result).toBe(1588)
    })
})
