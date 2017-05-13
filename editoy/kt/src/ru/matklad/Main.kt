package ru.matklad

import javafx.application.Application
import javafx.application.Platform
import javafx.scene.canvas.GraphicsContext
import ru.matklad.backend.Backend
import ru.matklad.backend.ProtoBackend
import ru.matklad.backend.Server
import ru.matklad.proto.InputEvent
import tornadofx.App
import tornadofx.UIComponent

val SETTINGS = Settings()


fun main(args: Array<String>) {
    Server.spawn()?.use {
        Application.launch(ToyApp::class.java, *args)
    }
}


fun createBacked(): Backend = ProtoBackend()


class ToyApp : App(ToyView::class) {
    lateinit var backend: Backend

    override fun onBeforeShow(view: UIComponent) {
        view as ToyView
        backend = createBacked()
        backend.updateSource.bind { viewState ->
            Platform.runLater { view.viewModel.set(viewState) }
        }

        view.eventHandler = { inputEvent ->
            view.runAsync { backend.eventSink(inputEvent) }
        }

        val startUpEvents = listOf(
                inputEvent { readyBuilder.build() },
                inputEvent { openFileBuilder.setPath(SETTINGS.defaultFile).build() }
        )
        startUpEvents.forEach { backend.eventSink(it) }
    }

    override fun stop() {
        super.stop()
        backend.close()
    }


}

class GridDrawer(
        private val g: GraphicsContext,
        private val cellSize: Dimension,
        private val cursorWidth: Double
) {
    fun drawText(position: GridPosition, text: String) {
        check('\n' !in text)
        g.fillText(text, position.xx, position.yy + cellSize.height)
    }

    fun drawCursor(position: GridPosition) {
        g.fillRect(position.xx, position.yy, cursorWidth, cellSize.height)
    }

    private val GridPosition.xx: Double get() = x * cellSize.width
    private val GridPosition.yy: Double get() = y * cellSize.height

}


data class Dimension(val width: Double, val height: Double)