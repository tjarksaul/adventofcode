#!/usr/local/bin/ts-node
import { readFileSync } from "fs"

function parseFile(filename: string): string[][] {
    return readFileSync(filename, 'utf8').trim().split(/\r?\n/).map(item => item.split(')'))
}

interface Orbit {
    name: string
    parent?: Orbit
}

function parseOrbits(orbits: string[][]): Map<string, Orbit> {
    let map = new Map<string, Orbit>()
    for (let orbit of orbits) {
        let left = map.get(orbit[0])
        if (!left) {
            left = {
                name: orbit[0],
                parent: undefined,
            }
            map.set(orbit[0], left)
        }
        let right = map.get(orbit[1])
        if (!right) {
            right = {
                name: orbit[1],
                parent: undefined,
            }
            map.set(orbit[1], right)
        }
        right.parent = left
    }
    return map
}

function countParents(orbit: Orbit): number {
    if (orbit.parent) {
        return 1 + countParents(orbit.parent)
    }
    return 0
}

function main() {
    let orbits = parseOrbits(parseFile('06.input'))
    let count = 0
    orbits.forEach((value => count += countParents(value)))
    console.log({ count })
}

function test() {
    let orbits = parseOrbits(parseFile('06.test'))
    let count = 0
    orbits.forEach((value => count += countParents(value)))
    if (count !== 42) {
        console.error(`Test failed, count should be 42 but is ${count}`)
    }
}

test()
main()
