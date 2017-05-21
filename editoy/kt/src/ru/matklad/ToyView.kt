package ru.matklad

import javafx.beans.binding.Bindings
import javafx.beans.property.SimpleLongProperty
import javafx.beans.property.SimpleObjectProperty
import javafx.beans.value.ObservableValue
import javafx.scene.Parent
import javafx.scene.canvas.Canvas
import javafx.scene.control.TextArea
import javafx.scene.input.KeyCode
import javafx.scene.input.KeyEvent
import javafx.scene.paint.Color
import javafx.stage.FileChooser
import ru.matklad.proto.Amount
import ru.matklad.proto.Direction
import ru.matklad.proto.InputEvent
import tornadofx.*
import java.util.concurrent.Callable


class ToyView : View() {
    val windowSize = (SETTINGS.editorSize.height / SETTINGS.cellSize.height).toInt()
    var windowStart = 0
    val viewModel = SimpleObjectProperty(ViewState()).apply {
        addListener { _, _, newValue ->
            val cursorLine = newValue.cursor.y
            windowStart += maxOf(0, cursorLine - (windowStart + windowSize - 2))
            windowStart -= maxOf(0, windowStart - cursorLine + 1)
            windowStart = maxOf(0, windowStart)
            redraw(newValue)
        }
    }
    var lastInputEvent = 0L
    val lastFrameTime = SimpleLongProperty()


    var eventHandler: (InputEvent) -> Unit by singleAssign()

    private val canvas = prepareCanvas(SETTINGS.editorSize).apply {
        addEventHandler(KeyEvent.ANY) { key ->
            val event = when {
                key.isControlDown && key.code == KeyCode.O -> {
                    val file = FileChooser().showOpenDialog(this@ToyView.currentWindow)
                    file?.let { inputEvent { openFileBuilder.setPath(it.path).build() } }
                }
                key.isControlDown && key.code == KeyCode.S -> {
                    inputEvent { saveFileBuilder.build() }
                }
                else -> keyPressedToEvent(key)
            }
            if (event != null) {
                fireEvent(event)
            }
        }
    }

    override val root: Parent = borderpane {
        center = canvas
        right = TextArea().apply {
            isEditable = false
            isFocusTraversable = false
            textProperty().bind(Bindings.select<String>(viewModel, "syntaxTree"))
        }
        bottom = borderpane {
            style {
                borderColor += box(top = Color.BLACK,
                        left = null, right = null, bottom = null
                )
            }
            left = label(Bindings.format("%3d lex %3d parse %3d draw %s",
                    viewModel.map { it.lexingTime / 1_000_000 },
                    viewModel.map { it.parsingTime / 1_000_000 },
                    lastFrameTime.divide(1_000_000),
                    viewModel.map { it.openedFile?.toString() + (if (it.isDirty) "*" else "") }
            ))
            right = label(Bindings.select<GridPosition>(viewModel, "cursor"))
            children.style {
                font = SETTINGS.font
            }
        }
    }


    private fun fireEvent(event: InputEvent) {
        lastInputEvent = System.nanoTime()
        eventHandler(event)
    }

    private fun redraw(state: ViewState) {
        lastFrameTime.set(System.nanoTime() - lastInputEvent)
        val g = canvas.graphicsContext2D
        g.font = SETTINGS.font
        g.fill = SETTINGS.defaultBackground
        g.fillRect(0.0, 0.0, SETTINGS.editorSize.width, SETTINGS.editorSize.height)
        val grid = GridDrawer(g, SETTINGS.cellSize, SETTINGS.cursorWidth)
        redraw(grid, state, windowStart to (windowStart + windowSize))
    }
}


private fun keyPressedToEvent(key: KeyEvent): InputEvent? {
    fun movement(code: KeyCode): Pair<Direction, Amount>? {
        val amount = if (key.code.isArrowKey) Amount.CHAR else Amount.PAGE
        val direction = when (code) {
            KeyCode.RIGHT, KeyCode.END -> Direction.RIGHT
            KeyCode.LEFT, KeyCode.HOME -> Direction.LEFT
            KeyCode.UP, KeyCode.PAGE_UP -> Direction.UP
            KeyCode.DOWN, KeyCode.PAGE_DOWN -> Direction.DOWN
            else -> return null
        }
        return direction to amount
    }

    if (key.eventType == KeyEvent.KEY_PRESSED) {
        movement(key.code)?.let { (direction, amount) ->
            return inputEvent { moveCursorBuilder.setDirection(direction).setAmount(amount).build() }
        }
    }

    if (key.eventType == KeyEvent.KEY_TYPED && key.character != KeyEvent.CHAR_UNDEFINED && !key.isControlDown) {
        val text = if (key.character == "\r") "\n" else key.character
        return inputEvent { insertTextBuilder.setText(text).build() }
    }
    return null
}

private fun prepareCanvas(dimension: Dimension): Canvas {
    val canvas = Canvas(dimension.width, dimension.height)
    canvas.isFocusTraversable = true
    return canvas
}

private fun redraw(grid: GridDrawer, state: ViewState, window: Pair<Int, Int>) {
    val (lo, hi) = window
    val lines = state.lines.subList(lo, minOf(hi, state.lines.size))
    for ((y, line) in lines.withIndex()) {
        var x = 0
        for ((text, style) in line) {
            grid.drawText(GridPosition(x, y), text, style)
            x += text.length
        }
    }

    grid.drawCursor(state.cursor.copy(y = state.cursor.y - window.first))
}


fun <T, U> ObservableValue<T>.map(f: (T) -> U): ObservableValue<U> =
        Bindings.createObjectBinding(Callable<U> { f(this.value) }, this)
