package ru.matklad

import javafx.scene.text.Font
import javafx.scene.text.Text

class Settings {
    val defaultFile = "/home/matklad/projects/fall/fall_gen/src/syntax.fall"
    val buildCommand = ProcessBuilder("/home/matklad/.cargo/bin/cargo", "build", "--manifest-path", "./rs/Cargo.toml")
    val backendCommand = ProcessBuilder("/home/matklad/projects/editoy/rs/target/debug/ediback")

    val font = Font.font("Ubuntu Mono", 18.0)!!

    val editorSize = Dimension(1024.0, 768.0)

    val cellSize = run {
        val t = Text("W")
        t.font = font
        Dimension(t.layoutBounds.width, t.layoutBounds.height)
    }
    val cursorWidth: Double = 2.0
}
