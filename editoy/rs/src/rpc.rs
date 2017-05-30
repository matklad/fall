use std::path::PathBuf;

use futures::{self, Future, Sink, Stream};
use grpc::{RequestOptions, SingleResponse, StreamingResponse};

use ediproto::{InputEvent as EventRequest, EventReply, ViewStateRequest, ViewStateReply, Editor as ProtoEditor};
use model::{Editor, InputEvent};
use std::sync::Mutex;

pub struct EditorServerImpl<E: Editor> {
    editor: Mutex<(E,
                   futures::sync::mpsc::Sender<ViewStateReply>,
                   Option<futures::sync::mpsc::Receiver<ViewStateReply>>)>,
}

impl<E: Default + Editor> Default for EditorServerImpl<E> {
    fn default() -> Self {
        let (sink, stream) = futures::sync::mpsc::channel::<ViewStateReply>(64);
        EditorServerImpl {
            editor: Mutex::new((E::default(), sink, Some(stream)))
        }
    }
}

impl<E: Editor> ProtoEditor for EditorServerImpl<E> {
    fn event(&self, _: RequestOptions, p: EventRequest) -> SingleResponse<EventReply> {
        let mut g = self.editor.lock().unwrap();
        g.0.apply(input_event_from_proto(p));
        let view_state = g.0.render();

        let reply = futures::finished(EventReply::new());
        let sink = g.1.clone();
        SingleResponse::no_metadata(
            sink.send(view_state)
                .map_err(|_| unreachable!())
                .and_then(|_| reply)
        )
    }

    fn updates(&self, _: RequestOptions, _: ViewStateRequest) -> StreamingResponse<ViewStateReply> {
        let mut g = self.editor.lock().unwrap();
        let mut stream = None;
        ::std::mem::swap(&mut stream, &mut g.2);
        let stream = stream.unwrap();
        let stream = stream.map_err(|_| unreachable!());

        StreamingResponse::no_metadata(stream)
    }
}

fn input_event_from_proto(mut p: EventRequest) -> InputEvent {
    if p.has_ready() {
        InputEvent::Ready
    } else if p.has_move_cursor() {
        let mc = p.get_move_cursor();
        InputEvent::MoveCursor(mc.direction, mc.amount)
    } else if p.has_insert_text() {
        InputEvent::InsertText(p.take_insert_text().text)
    } else if p.has_open_file() {
        InputEvent::OpenFile(PathBuf::from(p.take_open_file().path))
    } else if p.has_save_file() {
        InputEvent::SaveFile
    } else {
        panic!()
    }
}
