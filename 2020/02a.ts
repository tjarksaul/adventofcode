import { readFileSync } from 'fs'

interface PasswordPolicy {
    min: number
    max: number
    letter: string
    password: string
}

function readInput() {
    return readFileSync('02.input', { encoding: 'utf-8' })
}

function transformInput(input: string) {
    return input.split('\n').map(line => {
        let [amounts, letter, password] = line.split(' ', 3)
        let [min, max] = amounts.split('-').map(val => parseInt(val, 10))
        letter = letter.charAt(0)
        return { min, max, letter, password }
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

function checkPassword({ min, max, letter, password }: PasswordPolicy): boolean {
    const count = countCharacter(letter, password)
    return count >= min && count <= max
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