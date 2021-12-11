import { Grid, mapData, readFile } from "./05"

class GridB extends Grid {
    navigate(): void {
        this.instructionSet.instructions.forEach(({ x1, y1, x2, y2 }) => {
            const length = Math.abs((x1 === x2) ? y2 - y1 : x2 - x1) + 1
            Array.from({ length }).map((_, i) => {
                const x = (x1 === x2) ? x1 : (x2 > x1 ? x1 + i : x1 - i)
                const y = (y1 === y2) ? y1 : (y2 > y1 ? y1 + i : y1 - i)
                return [x, y]
            }).forEach(([x, y]) => {
                this.rows[y][x] += 1
            })
        })
    }
}

async function main() {
    const data = await readFile()
    const input = mapData(data)

    const grid = new GridB(input)
    grid.navigate()

    console.log(grid.toString())

    console.log(`Total points: ${grid.points}`)
}

main().catch(e => console.error(e)).finally(() => process.exit(0))
