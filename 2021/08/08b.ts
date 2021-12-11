import { mapData, readFile, SevenSegment } from "./08"

export function findMapping(inputs: string[]): Record<string, number> {
    const input = inputs.map(segments => [...segments].sort().join('')).filter((v, i, self) => self.indexOf(v) === i)
    const inputItems = input.sort((a, b)  => a.length - b.length)
    const numberMapping: string[] = Array.from({ length: 10 }).map(() => '')
    numberMapping[1] = inputItems.shift()
    numberMapping[7] = inputItems.shift()
    numberMapping[4] = inputItems.shift()
    numberMapping[8] = inputItems.pop()
    let twoThreeFive = inputItems.slice(0, 3)
    let zeroSixNine = inputItems.slice(3, 6)
    numberMapping[3] = twoThreeFive.find(item => [...numberMapping[1]].every(segment => item.includes(segment)))
    twoThreeFive = twoThreeFive.filter(item => item !== numberMapping[3])
    numberMapping[9] = zeroSixNine.find(item => [...numberMapping[4]].every(segment => item.includes(segment)))
    zeroSixNine = zeroSixNine.filter(item => item !== numberMapping[9])
    numberMapping[0] = zeroSixNine.find(item => [...numberMapping[1]].every(segment => item.includes(segment)))
    numberMapping[6] = zeroSixNine.filter(item => item !== numberMapping[0]).pop()
    numberMapping[5] = twoThreeFive.find(item => [...item].every(segment => numberMapping[6].includes(segment)))
    numberMapping[2] = twoThreeFive.filter(item => item !== numberMapping[5]).pop()

    return numberMapping.reduce((carry, cur, idx) => {
        carry[cur] = idx
        return carry
    }, {} as Record<string, number>)
}

export function calculateSum(input: SevenSegment[]): number {
    return input.reduce((carry, { input, output }) => {
        const mapping = findMapping(input)
        const digits = output.map(item => {
            const segments = [...item].sort().join('')
            return mapping[segments]
        }).join('')

        return carry + parseInt(digits, 10)
    }, 0)
}

async function main(): Promise<void> {
    const data = await readFile()
    const input = mapData(data)

    const result = calculateSum(input)

    console.log(result)
}

if (require.main === module) {
    main().catch(e => console.error(e)).finally(() => process.exit(0))
}