package ru.matklad.backend

import ru.matklad.GridPosition
import ru.matklad.ViewState
import ru.matklad.proto.ViewStateReply

fun viewStateFromProto(viewStateReply: ViewStateReply): ViewState {
    return ViewState(
            viewStateReply.lineList,
            GridPosition(viewStateReply.cursorX, viewStateReply.cursorY),
            viewStateReply.syntaxTree
    )
}
