import { readFileSync } from "fs"

class InvalidOpcodeError extends Error {
    constructor(opcode) {
        super()
        this.message = `Invalid Opcode ${opcode}`
        this.name = 'Error'
    }
}

enum Operation {
    ADDITION = 1,
    MULTIPLICATION,
    HALT = 99,
}

function handleOpcode(opcode: number): Operation {
    switch (opcode) {
        case 1:
            // addition
            return Operation.ADDITION
        case 2:
            // multiplication
            return Operation.MULTIPLICATION
        case 99:
            // halt
            return Operation.HALT
        default:
            throw new InvalidOpcodeError(opcode)
    }
}

function performOperation(operation: Operation, index: number, input: number[]): number[] {
    if (operation != Operation.ADDITION && operation != Operation.MULTIPLICATION) {
        return input
    }
    let left = input[input[index + 1]]
    let right = input[input[index + 2]]
    let result_index = input[index + 3]
    switch (operation) {
        case Operation.ADDITION:
            input[result_index] = left + right
            break
        case Operation.MULTIPLICATION:
            input[result_index] = left * right
            break
    }
    return input
}

function initialize(input: number[]) {
    input[1] = 12
    input[2] = 2
    return input
}

function handle(input: number[]): number[] {
    input = initialize(input)
    let currentIndex = 0
    let nextOpcode = handleOpcode(input[currentIndex])
    while (nextOpcode != Operation.HALT) {
        input = performOperation(nextOpcode, currentIndex, input)

        currentIndex = nextIndex(currentIndex)
        nextOpcode = handleOpcode(input[currentIndex])
    }
    return input
}

function nextIndex(index: number): number {
    return index + 4
}

function output(input: number[]) {
    console.log(input)
}

function test() {
    function array_equals(array1, array2) {
        return array1.length === array2.length && array1.every((value, index) => value === array2[index])
    }

    let inputs = [[1, 0, 0, 0, 99], [2, 3, 0, 3, 99], [2, 4, 4, 5, 99, 0], [1, 1, 1, 4, 99, 5, 6, 0, 99]]
    let expected = [[2, 0, 0, 0, 99], [2, 3, 0, 6, 99], [2, 4, 4, 5, 99, 9801], [30, 1, 1, 4, 2, 5, 6, 0, 99]]

    for (let i = 0; i < inputs.length; i++) {
        const input = inputs[i]
        const exp = expected[i]

        let result = handle(input)

        output(result)
        if (!array_equals(result, exp)) {
            console.error(`Test failed: ${result} is not equivalent to expected ${exp}`)
        }
    }
}

function parseFile(filename: string): number[] {
    return readFileSync(filename, 'utf8').trim().split(/,/).map(item => parseInt(item, 10))
}

function live() {
    const input = parseFile('02.input')
    output(handle(input))
}

test()
live()