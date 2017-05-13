use std::path::PathBuf;

use futures::{self, Future, Sink, Stream};
use grpc::{GrpcRequestOptions, GrpcSingleResponse, GrpcStreamingResponse};

use ediproto::{InputEvent as EventRequest, EventReply, ViewStateRequest, ViewStateReply, Editor as ProtoEditor};
use model::{Editor, ViewState, InputEvent};
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
    fn event(&self, _: GrpcRequestOptions, p: EventRequest) -> GrpcSingleResponse<EventReply> {
        let mut g = self.editor.lock().unwrap();
        let view_state = g.0.apply(input_event_from_proto(p));

        let reply = futures::finished(EventReply::new());
        let sink = g.1.clone();
        let response = view_state_to_proto(view_state);
        GrpcSingleResponse::no_metadata(
            sink.send(response)
                .map_err(|_| unreachable!())
                .and_then(|_| reply)
        )
    }

    fn updates(&self, _: GrpcRequestOptions, _: ViewStateRequest) -> GrpcStreamingResponse<ViewStateReply> {
        let mut g = self.editor.lock().unwrap();
        let mut stream = None;
        ::std::mem::swap(&mut stream, &mut g.2);
        let stream = stream.unwrap();
        let stream = stream.map_err(|_| unreachable!());

        GrpcStreamingResponse::no_metadata(stream)
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
    } else {
        panic!()
    }
}

fn view_state_to_proto(s: ViewState) -> ViewStateReply {
    let mut result = ViewStateReply::new();
    for line in s.lines {
        result.mut_line().push(line)
    }
    result.cursorX = s.cursor.x as i32;
    result.cursorY = s.cursor.y as i32;
    result.syntax_tree = s.syntax_tree;
    result
}
