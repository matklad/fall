package ru.matklad

import ru.matklad.proto.InputEvent

data class GridPosition(val x: Int = 0, val y: Int = 0)

data class ViewState(
        val lines: List<List<Pair<String, TextStyle>>> = emptyList(),
        val cursor: GridPosition = GridPosition(),
        val syntaxTree: String = ""
)

fun inputEvent(f: InputEvent.Builder.() -> Unit): InputEvent =
        InputEvent.newBuilder().apply(f).build()
