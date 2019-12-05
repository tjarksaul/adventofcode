#!/usr/local/bin/ts-node
const low = 128392
const high = 643281

function checkForDouble(number: number): boolean {
    const s = toArray(number)
    return (s[0] === s[1] && s[1] !== s[2])
        || (s[0] !== s[1] && s[1] === s[2] && s[2] !== s[3])
        || (s[1] !== s[2] && s[2] === s[3] && s[3] !== s[4])
        || (s[2] !== s[3] && s[3] === s[4] && s[4] !== s[5])
        || (s[3] !== s[4] && s[4] === s[5])
}

function checkIncreasing(number: number): boolean {
    const s = toArray(number)
    return s[0] <= s[1] && s[1] <= s[2] && s[2] <= s[3] && s[3] <= s[4] && s[4] <= s[5]
}

function toArray(number: number): number[] {
    const s = '' + number
    return s.split('').map(n => parseInt(n, 10))
}

function check(number: number): boolean {
    if (('' + number).length !== 6) {
        return false
    }
    if (!checkForDouble(number)) {
        return false
    }
    return checkIncreasing(number)
}

let numbers: number[] = []
for (let n = low; n <= high; n++) {
    if (check(n)) {
        numbers.push(n)
    }
}
console.log({ numbers, length: numbers.length })