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

function getParents(orbit: Orbit): string[] {
    if (orbit.parent) {
        return [orbit.parent.name].concat(getParents(orbit.parent))
    }
    return []
}

function getDistance(orbits: Map<string, Orbit>): number {
    let youParent = orbits.get('YOU').parent
    let santaParent = orbits.get('SAN').parent
    if (!youParent || !santaParent) {
        console.error('No parent found')
        return
    }
    const youParents = getParents(youParent)
    const santaParents = getParents(santaParent)
    for (let i = 0; i <= youParents.length; i++) {
        const item = youParents[i]
        const santaIndex = santaParents.indexOf(item)
        if (santaIndex !== -1) {
            return i + 1 + santaIndex + 1
        }
    }
    return -1
}

function main() {
    let orbits = parseOrbits(parseFile('06.input'))
    const distance = getDistance(orbits)
    console.log({ distance })
}

function test() {
    let orbits = parseOrbits(parseFile('06-02.test'))
    const distance = getDistance(orbits)
    if (distance !== 4) {
        console.error(`Test failed, distance should be 4 but is ${distance}`)
    }
}

test()
main()
