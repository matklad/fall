import {backend} from './backend'
import {State} from './state'

export default {
    status() {
        let status = backend.status()
        console.log(status)
    },
    
    showSyntaxTree(state: State) {
        let tree = state.support.showSyntaxTree(state.getText())
        console.log(tree)
    }
}