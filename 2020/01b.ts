import * as readline from 'readline'
import * as fs from 'fs'

function readInput() {
    const readInterface = readline.createInterface({
        input: fs.createReadStream('01.input'),
        output: null,
    })

    readInterface.on('line', line => {
        const number = parseInt(line, 10);
        handleInput(number)
    })
}

const numbers: number[] = []

function handleInput(number: number) {
    for (let e1 of numbers) {
        for (let e2 of numbers) {
            const sum = e1 + e2 + number
            if (sum === 2020) {
                console.log(`Result: ${e1 * e2 * number}`)
            }
        }
    }
    numbers.push(number)
}

readInput()