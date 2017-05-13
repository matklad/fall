package ru.matklad

import ru.matklad.proto.InputEvent

data class GridPosition(val x: Int = 0, val y: Int = 0)

data class ViewState(
        val lines: List<String> = emptyList(),
        val cursor: GridPosition = GridPosition()
)

fun inputEvent(f: InputEvent.Builder.() -> Unit): InputEvent =
        InputEvent.newBuilder().apply(f).build()
