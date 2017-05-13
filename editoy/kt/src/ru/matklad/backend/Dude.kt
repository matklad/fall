package ru.matklad.backend

import ru.matklad.ViewState
import ru.matklad.proto.InputEvent

typealias EventSink = (InputEvent) -> Unit
interface UpdateSource {
    fun bind(callback: (ViewState) -> Unit)
}

interface Backend : AutoCloseable {
    val eventSink: EventSink
    val updateSource: UpdateSource
}

class UpdateSourceImpl : UpdateSource {
    var callback: ((ViewState) -> Unit)? = null

    fun emit(state: ViewState) {
        val callback = checkNotNull(this.callback) { "not yet bound" }
        callback(state)
    }

    override fun bind(callback: (ViewState) -> Unit) {
        check(this.callback == null) { "already bound" }
        this.callback = callback
    }
}
