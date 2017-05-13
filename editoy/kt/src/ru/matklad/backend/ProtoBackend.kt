package ru.matklad.backend

import io.grpc.ManagedChannelBuilder
import io.grpc.Status
import io.grpc.StatusRuntimeException
import io.grpc.stub.StreamObserver
import ru.matklad.proto.EditorGrpc
import ru.matklad.proto.EventReply
import ru.matklad.proto.ViewStateReply
import ru.matklad.proto.ViewStateRequest
import ru.matklad.proto.Direction as ProtoDirection

class ProtoBackend : Backend {
    val channel = ManagedChannelBuilder
            .forAddress("localhost", 9292)
            .maxInboundMessageSize(1_000_000_000)
            .usePlaintext(true)
            .build()!!
    private val stub = EditorGrpc.newStub(channel)
    private val updateSourceImpl = UpdateSourceImpl()

    init {

        stub.updates(ViewStateRequest.getDefaultInstance(), object : StreamObserver<ViewStateReply> {
            override fun onCompleted() =
                    error("Should never complete")

            override fun onNext(value: ViewStateReply) {
                updateSourceImpl.emit(viewStateFromProto(value))
            }

            override fun onError(t: Throwable) {
                if (t is StatusRuntimeException && t.status.code == Status.Code.UNAVAILABLE) {
                    println("Channel shut down")
                    return
                }
                throw RuntimeException(t)
            }
        })
    }

    override val eventSink: EventSink = { event ->
        stub.event(event, object : StreamObserver<EventReply> {
            override fun onNext(value: EventReply) = Unit
            override fun onError(t: Throwable) = throw RuntimeException(t)
            override fun onCompleted() = Unit
        })
    }


    override val updateSource: UpdateSource = updateSourceImpl

    override fun close() {
        channel.shutdown()
    }
}
