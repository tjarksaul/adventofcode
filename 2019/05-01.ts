#!/usr/local/bin/ts-node
import { readFileSync } from 'fs'

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
    SAVE_INPUT,
    OUTPUT,
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
        case 3:
            return Instruction.SAVE_INPUT
        case 4:
            return Instruction.OUTPUT
        case 99:
            // halt
            return Instruction.HALT
        default:
            throw new InvalidOpcodeError(opcode)
    }
}

function performInstruction(instruction: Instruction, index: number, memory: number[], input: number): number[] {
    if (instruction != Instruction.ADDITION && instruction != Instruction.MULTIPLICATION
        && instruction != Instruction.SAVE_INPUT && instruction != Instruction.OUTPUT) {
        return memory
    }
    let left_index = Math.floor(memory[index] / 100) % 10 === 0 ? memory[index + 1] : index + 1
    let right_index = Math.floor(memory[index] / 1000) % 10 === 0 ? memory[index + 2] : index + 2
    let left = memory[left_index]
    let right = memory[right_index]
    let result_index = memory[index + 3]
    switch (instruction) {
        case Instruction.ADDITION:
            memory[result_index] = left + right
            break
        case Instruction.MULTIPLICATION:
            memory[result_index] = left * right
            break
        case Instruction.SAVE_INPUT:
            memory[left_index] = input
            break
        case Instruction.OUTPUT:
            console.log(memory[left_index])
            break
    }
    return memory
}

function handle(memory: number[], input: number): number[] {
    let instructionPointer = 0
    let nextInstruction = handleOpcode(memory[instructionPointer] % 100)
    while (nextInstruction != Instruction.HALT) {
        memory = performInstruction(nextInstruction, instructionPointer, memory, input)

        instructionPointer = nextInstructionPointer(instructionPointer, nextInstruction)
        nextInstruction = handleOpcode(memory[instructionPointer] % 100)
    }
    return memory
}

function nextInstructionPointer(index: number, instruction: Instruction): number {
    switch (instruction) {
        case Instruction.ADDITION:
        case Instruction.MULTIPLICATION:
            return index + 4
        case Instruction.SAVE_INPUT:
        case Instruction.OUTPUT:
            return index + 2
    }
}

function parseFile(filename: string): number[] {
    return readFileSync(filename, 'utf8').trim().split(/,/).map(item => parseInt(item, 10))
}

function main() {
    const memory = parseFile('05.input')
    handle(memory, 1)
}

main()