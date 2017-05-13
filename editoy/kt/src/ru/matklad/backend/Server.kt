package ru.matklad.backend

import ru.matklad.SETTINGS
import java.util.concurrent.CompletableFuture

class Server private constructor(
        private val process: Process
) : AutoCloseable {

    override fun close() {
        process.destroyForcibly().waitFor()
    }

    companion object {
        fun spawn(): Server? {
            val buildResult = SETTINGS
                    .buildCommand.redirectError(ProcessBuilder.Redirect.INHERIT)
                    .start()
                    .waitFor()
            if (buildResult != 0) return null

            val serverProcess = SETTINGS
                    .backendCommand.redirectError(ProcessBuilder.Redirect.INHERIT)
                    .start()

            val serverStarted = CompletableFuture<Boolean>()
            Thread {
                for (line in serverProcess.inputStream.bufferedReader().lines()) {
                    println(line)
                    if (line.startsWith("STARTED")) {
                        serverStarted.complete(true)
                    }
                }
                if (serverStarted.isDone.not()) {
                    serverStarted.complete(false)
                }
            }.start()

            if (!serverStarted.get()) return null
            return Server(serverProcess)
        }
    }
}
