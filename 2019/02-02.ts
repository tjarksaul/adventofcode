#!/usr/local/bin/ts-node
import { readFileSync } from "fs"

class InvalidOpcodeError extends Error {
    constructor(opcode) {
        super()
        this.message = `Invalid Opcode ${opcode}`
        this.name = 'Error'
    }
}

enum Instruction {
    ADDITION = 1,
    MULTIPLICATION,
    HALT = 99,
}

function handleOpcode(opcode: number): Instruction {
    switch (opcode) {
        case 1:
            // addition
            return Instruction.ADDITION
        case 2:
            // multiplication
            return Instruction.MULTIPLICATION
        case 99:
            // halt
            return Instruction.HALT
        default:
            throw new InvalidOpcodeError(opcode)
    }
}

function performInstruction(instruction: Instruction, index: number, input: number[]): number[] {
    if (instruction != Instruction.ADDITION && instruction != Instruction.MULTIPLICATION) {
        return input
    }
    let left = input[input[index + 1]]
    let right = input[input[index + 2]]
    let result_index = input[index + 3]
    switch (instruction) {
        case Instruction.ADDITION:
            input[result_index] = left + right
            break
        case Instruction.MULTIPLICATION:
            input[result_index] = left * right
            break
    }
    return input
}

function handle(input: number[]): number[] {
    let instructionPointer = 0
    let nextInstruction = handleOpcode(input[instructionPointer])
    while (nextInstruction != Instruction.HALT) {
        input = performInstruction(nextInstruction, instructionPointer, input)

        instructionPointer = nextInstructionPointer(instructionPointer)
        nextInstruction = handleOpcode(input[instructionPointer])
    }
    return input
}

function nextInstructionPointer(index: number): number {
    return index + 4
}

function parseFile(filename: string): number[] {
    return readFileSync(filename, 'utf8').trim().split(/,/).map(item => parseInt(item, 10))
}

function main() {
    const input = parseFile('02.input')
    let noun
    let verb
    outer:
        for (noun = 0; noun <= 99; noun++) {
            for (verb = 0; verb <= 99; verb++) {
                let theInput = [...input]
                theInput[1] = noun
                theInput[2] = verb
                try {
                    const result = handle(theInput)
                    if (result[0] === 19690720) {
                        console.log({
                            noun, verb,
                            output: result[0],
                            final: 100 * noun + verb
                        })
                        break outer
                    }
                } catch {}
            }
        }
}

main()