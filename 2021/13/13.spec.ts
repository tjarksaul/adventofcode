import { countDots } from "./part1"
import { mapData, PaperData } from "./base"
import { getResult } from "./part2"

const inputString = `6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5`

const resultPart2 = `#####
#...#
#...#
#...#
#####
.....
.....`

describe('day 13', () => {
    let input: PaperData

    beforeAll(() => {
        input = mapData(inputString)
    })

    describe('part 1', () => {
        it('should solve the example correctly', () => {
            const result = countDots(input, 1)

            expect(result).toBe(17)
        })
    })

    describe('part 2', () => {
        it('should solve the example correctly', () => {
            const result = getResult(input)

            expect(result).toBe(resultPart2)
        })
    })
})