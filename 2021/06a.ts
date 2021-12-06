import { mapData, readFile } from "./06"

function iterate(fish: number[], days: number): number[] {
    return Array.from({ length: days }).reduce<number[]>(result => {
        return result.reduce((fishes, fish, idx) => {
            if (fish === 0) {
                fishes[idx] = 6
                fishes.push(8)
            } else {
                fishes[idx] = fish - 1
            }
            return fishes
        }, result)
    }, fish)
}

async function main() {
    const data = await readFile()
    const input = mapData(data)

    const fish = iterate(input, 80)

    console.log(`Final state: ${fish.join(',')}}\nwith ${fish.length} fish`)
}

main().catch(e => console.error(e)).finally(() => process.exit(0))
