import { readFileSync } from 'fs'

interface PasswordPolicy {
    pos1: number
    pos2: number
    letter: string
    password: string
}

function readInput() {
    return readFileSync('02.input', { encoding: 'utf-8' })
}

function transformInput(input: string) {
    return input.split('\n').map(line => {
        let [amounts, letter, password] = line.split(' ', 3)
        let [pos1, pos2] = amounts.split('-').map(val => parseInt(val, 10))
        letter = letter.charAt(0)
        return { pos1, pos2, letter, password }
    })
}

function countCorrectPasswords(policies: PasswordPolicy[]) {
    let correct = 0
    for (const policy of policies) {
        if (checkPassword(policy)) {
            correct += 1
        }
    }
    console.log(`Total number of correct passwords: ${correct}`)
}

function checkPassword({ pos1, pos2, letter, password }: PasswordPolicy): boolean {
    return (password[pos1 - 1] == letter) != (password[pos2 - 1] == letter)
}

function countCharacter(character: string, string: string): number {
    return (string.split(character).length - 1)
}

function main() {
    const input = readInput()
    const policies = transformInput(input)
    countCorrectPasswords(policies)
}

main()