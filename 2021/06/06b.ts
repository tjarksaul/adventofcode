import { mapData, readFile } from "./06"

function iterate(fish: number[], days: number): number {
    let ages = Array.from({ length: 9 }).map(() => 0)
    fish.forEach(age => ages[age] += 1)

    for (let day = 0; day < days; day++) {
        ages[(day + 7) % 9] += ages[day % 9]
    }
    return ages.reduce((prev, cur) => prev + cur, 0)
}

async function main() {
    const data = await readFile()
    const input = mapData(data)

    const count = iterate(input, 256)

    console.log(`Final state: ${count} fish`)
}

main().catch(e => console.error(e)).finally(() => process.exit(0))
