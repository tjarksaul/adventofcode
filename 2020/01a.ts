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
    for (let existing of numbers) {
        const sum = existing + number
        if (sum === 2020) {
            console.log(`Result: ${existing * number}`)
        }
    }
    numbers.push(number)
}

readInput()