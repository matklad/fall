package ru.matklad

import ru.matklad.proto.InputEvent
import java.nio.file.Path

data class GridPosition(val x: Int = 0, val y: Int = 0)

data class ViewState(
        val lines: List<List<Pair<String, TextStyle>>> = emptyList(),
        val cursor: GridPosition = GridPosition(),
        val syntaxTree: String = "",
        val lexingTime: Long = 0,
        val parsingTime: Long = 0,
        val reparseLen: Int = 0,
        val openedFile: Path? = null,
        val isDirty: Boolean = false
)

fun inputEvent(f: InputEvent.Builder.() -> Unit): InputEvent =
        InputEvent.newBuilder().apply(f).build()
