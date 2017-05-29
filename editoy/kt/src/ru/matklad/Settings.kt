package ru.matklad

import javafx.scene.paint.Color
import javafx.scene.text.Font
import javafx.scene.text.Text

class Settings {
    val defaultFile = "/home/matklad/projects/fall/lang/fall/src/syntax.fall"
    val buildCommand = ProcessBuilder("/home/matklad/.cargo/bin/cargo", "build", "--manifest-path", "../rs/Cargo.toml")
    val backendCommand = ProcessBuilder("../rs/target/debug/ediback").apply {
        environment() += "RUST_BACKTRACE" to "short"
    }

    val font = Font.font("Ubuntu Mono", 18.0)!!
    val editorSize = Dimension(1024.0, 768.0)

    val cellSize = run {
        val t = Text("W")
        t.font = font
        Dimension(t.layoutBounds.width, t.layoutBounds.height)
    }
    val cursorWidth: Double = 2.0

    val colors: Map<String, Color> = """
        background 3F3F3F
        text DCDCCC
        keyword F0DFAF
        token DFAF8F
        rule 93E0E3
        string CC9393
        builtin DD6718
        error FF0000
    """.run {
        trim().lines()
                .map { it.trim().split(" ") }
                .map { it[0] to it[1] }
                .map { (name, color) ->
                    name to Color.web("#$color")
                }
                .toMap()
    }

    val defaultBackground = colors["background"]
    val cursorColor = colors["text"]
}
