import {container} from './container'

let RECORDS = []

function putRecord(tag: string, duration: number) {
    RECORDS.push([tag, duration])
    if (RECORDS.length > 100) {
        RECORDS = lastChunk(RECORDS)
    }
}

function lastChunk(array) {
    return array.slice(array.length - 10)
}

export function reportDuration<T>(tag: string, f: () => T): T {
    let start = Date.now()
    let result = f()
    let finish = Date.now()
    putRecord(tag, finish - start)
    container.textDocumentContentProvider.eventEmitter.fire(container.uris.status)
    return result
}

export function profileResultsAsString() {
    let result = ""
    let chunk = lastChunk(RECORDS)
    chunk.reverse()
    for (let [tag, duration] of chunk) {
        result += `${tag}: ${duration} ms\n`
    }
    return result
}