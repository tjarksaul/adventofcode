#!/usr/local/bin/ts-node

import * as util from "util"
import { readFile } from "fs"

class ParseError extends Error {
}

interface IMoon {
    position: Coordinates
    velocity: Coordinates
}

interface Coordinates {
    x: number;
    y: number;
    z: number
}

class Moon implements IMoon {
    position: Coordinates
    velocity: Coordinates

    get potentialEnergy(): number {
        return Math.abs(this.position.x) + Math.abs(this.position.y) + Math.abs(this.position.z)
    }

    get kineticEnergy(): number {
        return Math.abs(this.velocity.x) + Math.abs(this.velocity.y) + Math.abs(this.velocity.z)
    }

    get totalEnergy(): number {
        return this.potentialEnergy * this.kineticEnergy
    }

    static parse(input: string): Moon {
        const match = input.match(/<x= ?(-?\d+), ?y=(-?\d+), z=(-?\d+)>/)
        if (match.length !== 4) {
            throw new ParseError(`Expected 3 matches but insted got ${match.length - 1}`)
        }
        const x = parseInt(match[1], 10)
        const y = parseInt(match[2], 10)
        const z = parseInt(match[3], 10)
        return new Moon({ x, y, z })
    }

    constructor(position: Coordinates) {
        this.position = position
        this.velocity = { x: 0, y: 0, z: 0 }
    }

    applyVelocity(): void {
        const { position, velocity } = this
        for (let p of ['x', 'y', 'z']) {
            position[p] += velocity[p]
        }
    }

    print(): void {
        const { position, velocity } = this
        console.log(`pos=<x=${position.x}, y=${position.y}, z=${position.z}>, vel=<x=${velocity.x}, y=${velocity.y}, z=${velocity.z}>`)
    }
}

function updateVelocities(moon1: IMoon, moon2: IMoon) {
    for (let p of ['x', 'y', 'z']) {
        if (moon1.position[p] === moon2.position[p]) {
            continue
        }
        const update = moon1.position[p] < moon2.position[p] ? 1 : -1
        moon1.velocity[p] += update
        moon2.velocity[p] -= update
    }
}

function update(moons: Moon[]): void {
    for (let i = 0; i < moons.length; i++) {
        for (let j = i + 1; j < moons.length; j++) {
            updateVelocities(moons[i], moons[j])
        }
    }
    moons.forEach(moon => moon.applyVelocity())
}

function log(step: number, moons: Moon[]): void {
    console.log(`After ${step} steps:`)
    moons.forEach(moon => moon.print())
    console.log('')
}

function simulate(moons: Moon[], steps: number): void {
    log(0, moons)
    for (let step = 1; step <= steps; step++) {
        update(moons)
        if (steps <= 10 || step % 10 === 0) {
            log(step, moons)
        }
    }
}

async function parseFile(filename: string): Promise<Moon[]> {
    const rf = util.promisify(readFile)
    const fileContent = await rf(filename, { encoding: 'utf8' })
    return fileContent
        .trim()
        .split(/\r?\n/)
        .map(Moon.parse)
}

async function main() {
    const moons = await parseFile('12.input')
    simulate(moons, 1000)
    const totalEnergy = moons.map(moon => moon.totalEnergy).reduce((carry, item) => carry + item, 0)
    console.log({ totalEnergy })
}

main()
