#!/usr/local/bin/ts-node

import * as util from 'util'
import { readFile } from 'fs'

function createLayers(pixels: number[], dimension: number): number[][] {
    const layers: number[][] = []
    let currentLayer: number[] = []
    layers.push(currentLayer)
    while (pixels.length > 0) {
        let pixel = pixels.shift()

        if (currentLayer.length >= dimension) {
            currentLayer = []
            layers.push(currentLayer)
        }
        currentLayer.push(pixel)
    }
    return layers
}

function countNumber(layer: number[], number: number): number {
    return layer.reduce((prev, cur) => prev + (cur === number ? 1 : 0), 0)
}

async function parseFile(filename: string): Promise<number[]> {
    const rf = util.promisify(readFile)
    const fileContent = await rf(filename, { encoding: 'utf8' })
    return fileContent.trim().split('').map(item => parseInt(item, 10))
}

async function main() {
    const pixels = await parseFile('08.input')
    const dimension = 25 * 6
    // const layers = getLayers(pixels, dimension)
    const layers = createLayers(pixels, dimension)
    const zeros = layers.map(l => countNumber(l, 0))
    const minIndex = zeros.indexOf(Math.min(...zeros))
    const layer = layers[minIndex]

    const result = countNumber(layer, 1) * countNumber(layer, 2)
    console.log({ result })
}

main()