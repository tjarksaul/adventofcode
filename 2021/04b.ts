import { Input, mapData, readFile } from "./04";

function traverse({ values, fields }: Input) {
    const winning = fields.map(() => false)

    for (let i = 0; i < values.length; i++) {
        const value = values[i];
        for (let j = 0; j < fields.length; j++) {
            const field = fields[j];

            field.draw(value)
            if (field.isWinning()) {
                winning[j] = true
                if (winning.reduce((carry, isWinning) => carry + (isWinning ? 0 : 1), 0)  === 0) {
                    console.log('### YOU WON! ###')
                    console.log('## Winning field ##')
                    console.log(field.toString())
                    console.log(`Winning value: ${field.calculateWinningValue(value)}`)
                    return
                }
            }
        }
    }

    return null
}

async function main() {
    const data = await readFile()
    const input = mapData(data)

    traverse(input)
}

main().catch(e => console.error(e)).finally(() => process.exit(0))
