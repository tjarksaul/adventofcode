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

async function parseFile(filename: string): Promise<number[]> {
    const rf = util.promisify(readFile)
    const fileContent = await rf(filename, { encoding: 'utf8' })
    return fileContent.trim().split('').map(item => parseInt(item, 10))
}

async function main() {
    const pixels = await parseFile('08.input')
    const dimension = 25 * 6
    const layers = createLayers(pixels, dimension).reverse()

    const image = Array.from({ length: 6 }, (_, i) => {
        return Array.from({ length: 25 }, (_, j) => {
            const index = i * 25 + j
            let color: number
            for (let layer of layers) {
                const theColor = layer[index]
                if (theColor === 1 || theColor === 0) {
                    color = theColor
                }
            }
            return color
        })
    })
    const bitmap = image
        .map(row => row.map(pixel => pixel === 0 ? ' ' : '█')
            .reduce((prev, cur) => prev + cur, ''))
        .reduce((prev, cur) => prev + cur + '\n', '')
    console.log(bitmap)
}

main()