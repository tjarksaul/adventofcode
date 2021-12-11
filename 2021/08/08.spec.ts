import exp = require('constants')
import { mapData } from './08'
import { count as part1 } from './08a'
import { calculateSum, findMapping } from './08b'

const bigExample = `be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce`


describe('part 1', () => {
    it('solves the example correctly', () => {
        const input = bigExample

        const parsed = mapData(input)

        const solution = part1(parsed.flatMap(({ output }) => output))

        expect(solution).toEqual(26)
    })
})

describe('part 2', () => {
    it('finds correct possbile mapping', () => {
        const input = 'acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf'
        
        const parsed = mapData(input)[0]

        const mapping = findMapping(parsed.input)

        expect(mapping).toEqual({
            'abcdeg': 0, 
            'ab': 1, 
            'acdfg': 2, 
            'abcdf': 3, 
            'abef': 4, 
            'bcdef': 5, 
            'bcdefg': 6, 
            'abd': 7, 
            'abcdefg': 8, 
            'abcdef': 9,
        })
    })

    it('calculates sum correctly', () => {
        const input = bigExample

        const parsed = mapData(input)

        const result = calculateSum(parsed)

        expect(result).toEqual(61229)
    })
})