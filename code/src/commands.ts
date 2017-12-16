import backend from './backend'

export default {
    status() {
        let status = backend.status()
        console.log(status)
    }
}