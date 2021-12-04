import { promises as fs } from "fs"

interface Input {
    values: number[]
    fields: Field[]
}

interface Entry {
    value: number
    marked: boolean
}

class Field {
    columns: Entry[][]
    rows: Entry[][]
    valueMap: Record<number, Entry> = {}

    constructor(base: string) {
        this.columns = Array.from({ length: 5 }).map(() => Array.from({ length: 5 }))
        this.rows = Array.from({ length: 5 }).map(() => Array.from({ length: 5 }))
        this.constructEntries(base)
    }

    private constructEntries(base: string) {
        const rows = base.split('\n')
        rows.forEach((row, rowIndex) => {
            let column = 0
            for (let i = 3; i <= row.length + 1; i += 3) {
                const val = row.slice(i - 3, i);

                // console.log({ rowIndex, column, val })
                const value = parseInt(val.trim(), 10)
                const entry: Entry = { value, marked: false }
                this.rows[rowIndex][column] = entry
                this.columns[column][rowIndex] = entry
                this.valueMap[value] = entry
                column++
            }
        })

        // console.log(this.columns, this.rows)
    }

    draw(value: number) {
        if (this.valueMap[value]) {
            this.valueMap[value].marked = true
        }
    }

    isWinning(): boolean {
        return this.rows.some(row => isWinning(row)) || this.columns.some(column => isWinning(column))
    }

    calculateWinningValue(drawn: number): number {
        const sum = this.rows.reduce((carry, row) => carry + row.reduce((carry, entry) => carry + (entry.marked ? 0 : entry.value), 0), 0)

        return sum * drawn
    }

    toString() {
        return this.rows.map(row => {
            return row.map(column => `[${column.value < 10 ? '0' : ''}${column.value}:${column.marked ? '1' : '0'}]`).join(' ')
        }).join('\n')
    }
}

function isWinning(entries: Entry[]): boolean {
    return entries.every(entry => entry.marked)
}

async function readFile() {
    return fs.readFile('04.input', { encoding: 'utf8' })
}

function parseInput(string: string): number[] {
    return string.split(',').map(item => parseInt(item, 10))
}

function mapData(data: string): Input {
    const inputs = data.split('\n\n')

    const values = parseInput(inputs.shift())
    const fields = inputs.map(field => {
        return new Field(field)
    })

    return { values, fields }
}

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
