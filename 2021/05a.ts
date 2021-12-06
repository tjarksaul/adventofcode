import { Grid, mapData, readFile } from "./05"

class GridA extends Grid {
    navigate(): void {
        this.instructionSet.instructions.forEach(({ x1, y1, x2, y2 }) => {
            if (x1 === x2) {
                let [start, end] = [y1, y2]
                if (y1 > y2) {
                    [start, end] = [y2, y1]
                }
                for (let y = start; y <= end; y++) {
                    this.rows[y][x1] += 1
                }
            } else if (y1 === y2) {
                let [start, end] = [x1, x2]
                if (x1 > x2) {
                    [start, end] = [x2, x1]
                }
                for (let x = start; x <= end; x++) {
                    this.rows[y1][x] += 1
                }
            } else {
                // for now we ignore diagonal lines
            }
        })
    }
}

async function main() {
    const data = await readFile()
    const input = mapData(data)

    const grid = new GridA(input)
    grid.navigate()

    console.log(grid.toString())

    console.log(`Total points: ${grid.points}`)
}

main().catch(e => console.error(e)).finally(() => process.exit(0))
