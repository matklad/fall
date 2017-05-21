package ru.matklad.backend

import ru.matklad.GridPosition
import ru.matklad.SETTINGS
import ru.matklad.TextStyle
import ru.matklad.ViewState
import ru.matklad.proto.StyledText
import ru.matklad.proto.ViewStateReply
import java.nio.file.Paths

fun viewStateFromProto(viewStateReply: ViewStateReply): ViewState {
    return ViewState(
            viewStateReply.linesList.map { it.rangesList.map(::styledTextFromProto) },
            GridPosition(viewStateReply.cursorX, viewStateReply.cursorY),
            viewStateReply.syntaxTree,
            viewStateReply.lexingTimeNanos,
            viewStateReply.parsingTimeNanos,
            viewStateReply.file.takeIf { it.isNotEmpty() }?.let { Paths.get(it) },
            viewStateReply.dirty
    )
}

private fun styledTextFromProto(text: StyledText): Pair<String, TextStyle> {
    return text.text to attr(text.style)
}

private fun attr(a: String): TextStyle {
    val color = SETTINGS.colors[a]
            ?: error("unknown attribute `$a`")
    return TextStyle(color)
}
