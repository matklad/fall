// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct InputEvent {
    // message oneof groups
    kind: ::std::option::Option<InputEvent_oneof_kind>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InputEvent {}

#[derive(Clone,PartialEq)]
pub enum InputEvent_oneof_kind {
    ready(InputEvent_Ready),
    move_cursor(InputEvent_MoveCursor),
    insert_text(InputEvent_InsertText),
    open_file(InputEvent_OpenFile),
    save_file(InputEvent_SaveFile),
}

impl InputEvent {
    pub fn new() -> InputEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InputEvent {
        static mut instance: ::protobuf::lazy::Lazy<InputEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InputEvent,
        };
        unsafe {
            instance.get(InputEvent::new)
        }
    }

    // .editor.InputEvent.Ready ready = 1;

    pub fn clear_ready(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_ready(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(InputEvent_oneof_kind::ready(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ready(&mut self, v: InputEvent_Ready) {
        self.kind = ::std::option::Option::Some(InputEvent_oneof_kind::ready(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ready(&mut self) -> &mut InputEvent_Ready {
        if let ::std::option::Option::Some(InputEvent_oneof_kind::ready(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(InputEvent_oneof_kind::ready(InputEvent_Ready::new()));
        }
        match self.kind {
            ::std::option::Option::Some(InputEvent_oneof_kind::ready(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ready(&mut self) -> InputEvent_Ready {
        if self.has_ready() {
            match self.kind.take() {
                ::std::option::Option::Some(InputEvent_oneof_kind::ready(v)) => v,
                _ => panic!(),
            }
        } else {
            InputEvent_Ready::new()
        }
    }

    pub fn get_ready(&self) -> &InputEvent_Ready {
        match self.kind {
            ::std::option::Option::Some(InputEvent_oneof_kind::ready(ref v)) => v,
            _ => InputEvent_Ready::default_instance(),
        }
    }

    // .editor.InputEvent.MoveCursor move_cursor = 2;

    pub fn clear_move_cursor(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_move_cursor(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(InputEvent_oneof_kind::move_cursor(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_move_cursor(&mut self, v: InputEvent_MoveCursor) {
        self.kind = ::std::option::Option::Some(InputEvent_oneof_kind::move_cursor(v))
    }

    // Mutable pointer to the field.
    pub fn mut_move_cursor(&mut self) -> &mut InputEvent_MoveCursor {
        if let ::std::option::Option::Some(InputEvent_oneof_kind::move_cursor(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(InputEvent_oneof_kind::move_cursor(InputEvent_MoveCursor::new()));
        }
        match self.kind {
            ::std::option::Option::Some(InputEvent_oneof_kind::move_cursor(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_move_cursor(&mut self) -> InputEvent_MoveCursor {
        if self.has_move_cursor() {
            match self.kind.take() {
                ::std::option::Option::Some(InputEvent_oneof_kind::move_cursor(v)) => v,
                _ => panic!(),
            }
        } else {
            InputEvent_MoveCursor::new()
        }
    }

    pub fn get_move_cursor(&self) -> &InputEvent_MoveCursor {
        match self.kind {
            ::std::option::Option::Some(InputEvent_oneof_kind::move_cursor(ref v)) => v,
            _ => InputEvent_MoveCursor::default_instance(),
        }
    }

    // .editor.InputEvent.InsertText insert_text = 3;

    pub fn clear_insert_text(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_insert_text(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(InputEvent_oneof_kind::insert_text(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_insert_text(&mut self, v: InputEvent_InsertText) {
        self.kind = ::std::option::Option::Some(InputEvent_oneof_kind::insert_text(v))
    }

    // Mutable pointer to the field.
    pub fn mut_insert_text(&mut self) -> &mut InputEvent_InsertText {
        if let ::std::option::Option::Some(InputEvent_oneof_kind::insert_text(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(InputEvent_oneof_kind::insert_text(InputEvent_InsertText::new()));
        }
        match self.kind {
            ::std::option::Option::Some(InputEvent_oneof_kind::insert_text(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_insert_text(&mut self) -> InputEvent_InsertText {
        if self.has_insert_text() {
            match self.kind.take() {
                ::std::option::Option::Some(InputEvent_oneof_kind::insert_text(v)) => v,
                _ => panic!(),
            }
        } else {
            InputEvent_InsertText::new()
        }
    }

    pub fn get_insert_text(&self) -> &InputEvent_InsertText {
        match self.kind {
            ::std::option::Option::Some(InputEvent_oneof_kind::insert_text(ref v)) => v,
            _ => InputEvent_InsertText::default_instance(),
        }
    }

    // .editor.InputEvent.OpenFile open_file = 4;

    pub fn clear_open_file(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_open_file(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(InputEvent_oneof_kind::open_file(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_open_file(&mut self, v: InputEvent_OpenFile) {
        self.kind = ::std::option::Option::Some(InputEvent_oneof_kind::open_file(v))
    }

    // Mutable pointer to the field.
    pub fn mut_open_file(&mut self) -> &mut InputEvent_OpenFile {
        if let ::std::option::Option::Some(InputEvent_oneof_kind::open_file(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(InputEvent_oneof_kind::open_file(InputEvent_OpenFile::new()));
        }
        match self.kind {
            ::std::option::Option::Some(InputEvent_oneof_kind::open_file(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_open_file(&mut self) -> InputEvent_OpenFile {
        if self.has_open_file() {
            match self.kind.take() {
                ::std::option::Option::Some(InputEvent_oneof_kind::open_file(v)) => v,
                _ => panic!(),
            }
        } else {
            InputEvent_OpenFile::new()
        }
    }

    pub fn get_open_file(&self) -> &InputEvent_OpenFile {
        match self.kind {
            ::std::option::Option::Some(InputEvent_oneof_kind::open_file(ref v)) => v,
            _ => InputEvent_OpenFile::default_instance(),
        }
    }

    // .editor.InputEvent.SaveFile save_file = 5;

    pub fn clear_save_file(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_save_file(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(InputEvent_oneof_kind::save_file(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_save_file(&mut self, v: InputEvent_SaveFile) {
        self.kind = ::std::option::Option::Some(InputEvent_oneof_kind::save_file(v))
    }

    // Mutable pointer to the field.
    pub fn mut_save_file(&mut self) -> &mut InputEvent_SaveFile {
        if let ::std::option::Option::Some(InputEvent_oneof_kind::save_file(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(InputEvent_oneof_kind::save_file(InputEvent_SaveFile::new()));
        }
        match self.kind {
            ::std::option::Option::Some(InputEvent_oneof_kind::save_file(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_save_file(&mut self) -> InputEvent_SaveFile {
        if self.has_save_file() {
            match self.kind.take() {
                ::std::option::Option::Some(InputEvent_oneof_kind::save_file(v)) => v,
                _ => panic!(),
            }
        } else {
            InputEvent_SaveFile::new()
        }
    }

    pub fn get_save_file(&self) -> &InputEvent_SaveFile {
        match self.kind {
            ::std::option::Option::Some(InputEvent_oneof_kind::save_file(ref v)) => v,
            _ => InputEvent_SaveFile::default_instance(),
        }
    }
}

impl ::protobuf::Message for InputEvent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.kind = ::std::option::Option::Some(InputEvent_oneof_kind::ready(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.kind = ::std::option::Option::Some(InputEvent_oneof_kind::move_cursor(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.kind = ::std::option::Option::Some(InputEvent_oneof_kind::insert_text(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.kind = ::std::option::Option::Some(InputEvent_oneof_kind::open_file(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.kind = ::std::option::Option::Some(InputEvent_oneof_kind::save_file(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.kind {
            match v {
                &InputEvent_oneof_kind::ready(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InputEvent_oneof_kind::move_cursor(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InputEvent_oneof_kind::insert_text(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InputEvent_oneof_kind::open_file(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InputEvent_oneof_kind::save_file(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.kind {
            match v {
                &InputEvent_oneof_kind::ready(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InputEvent_oneof_kind::move_cursor(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InputEvent_oneof_kind::insert_text(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InputEvent_oneof_kind::open_file(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &InputEvent_oneof_kind::save_file(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InputEvent {
    fn new() -> InputEvent {
        InputEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<InputEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, InputEvent_Ready>(
                    "ready",
                    InputEvent::has_ready,
                    InputEvent::get_ready,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, InputEvent_MoveCursor>(
                    "move_cursor",
                    InputEvent::has_move_cursor,
                    InputEvent::get_move_cursor,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, InputEvent_InsertText>(
                    "insert_text",
                    InputEvent::has_insert_text,
                    InputEvent::get_insert_text,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, InputEvent_OpenFile>(
                    "open_file",
                    InputEvent::has_open_file,
                    InputEvent::get_open_file,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, InputEvent_SaveFile>(
                    "save_file",
                    InputEvent::has_save_file,
                    InputEvent::get_save_file,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InputEvent>(
                    "InputEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InputEvent {
    fn clear(&mut self) {
        self.clear_ready();
        self.clear_move_cursor();
        self.clear_insert_text();
        self.clear_open_file();
        self.clear_save_file();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InputEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InputEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InputEvent_Ready {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InputEvent_Ready {}

impl InputEvent_Ready {
    pub fn new() -> InputEvent_Ready {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InputEvent_Ready {
        static mut instance: ::protobuf::lazy::Lazy<InputEvent_Ready> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InputEvent_Ready,
        };
        unsafe {
            instance.get(InputEvent_Ready::new)
        }
    }
}

impl ::protobuf::Message for InputEvent_Ready {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InputEvent_Ready {
    fn new() -> InputEvent_Ready {
        InputEvent_Ready::new()
    }

    fn descriptor_static(_: ::std::option::Option<InputEvent_Ready>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<InputEvent_Ready>(
                    "InputEvent_Ready",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InputEvent_Ready {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InputEvent_Ready {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InputEvent_Ready {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InputEvent_MoveCursor {
    // message fields
    pub direction: Direction,
    pub amount: Amount,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InputEvent_MoveCursor {}

impl InputEvent_MoveCursor {
    pub fn new() -> InputEvent_MoveCursor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InputEvent_MoveCursor {
        static mut instance: ::protobuf::lazy::Lazy<InputEvent_MoveCursor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InputEvent_MoveCursor,
        };
        unsafe {
            instance.get(InputEvent_MoveCursor::new)
        }
    }

    // .editor.Direction direction = 1;

    pub fn clear_direction(&mut self) {
        self.direction = Direction::UP;
    }

    // Param is passed by value, moved
    pub fn set_direction(&mut self, v: Direction) {
        self.direction = v;
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    fn get_direction_for_reflect(&self) -> &Direction {
        &self.direction
    }

    fn mut_direction_for_reflect(&mut self) -> &mut Direction {
        &mut self.direction
    }

    // .editor.Amount amount = 2;

    pub fn clear_amount(&mut self) {
        self.amount = Amount::CHAR;
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: Amount) {
        self.amount = v;
    }

    pub fn get_amount(&self) -> Amount {
        self.amount
    }

    fn get_amount_for_reflect(&self) -> &Amount {
        &self.amount
    }

    fn mut_amount_for_reflect(&mut self) -> &mut Amount {
        &mut self.amount
    }
}

impl ::protobuf::Message for InputEvent_MoveCursor {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.direction = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.amount = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.direction != Direction::UP {
            my_size += ::protobuf::rt::enum_size(1, self.direction);
        };
        if self.amount != Amount::CHAR {
            my_size += ::protobuf::rt::enum_size(2, self.amount);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.direction != Direction::UP {
            os.write_enum(1, self.direction.value())?;
        };
        if self.amount != Amount::CHAR {
            os.write_enum(2, self.amount.value())?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InputEvent_MoveCursor {
    fn new() -> InputEvent_MoveCursor {
        InputEvent_MoveCursor::new()
    }

    fn descriptor_static(_: ::std::option::Option<InputEvent_MoveCursor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Direction>>(
                    "direction",
                    InputEvent_MoveCursor::get_direction_for_reflect,
                    InputEvent_MoveCursor::mut_direction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Amount>>(
                    "amount",
                    InputEvent_MoveCursor::get_amount_for_reflect,
                    InputEvent_MoveCursor::mut_amount_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InputEvent_MoveCursor>(
                    "InputEvent_MoveCursor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InputEvent_MoveCursor {
    fn clear(&mut self) {
        self.clear_direction();
        self.clear_amount();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InputEvent_MoveCursor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InputEvent_MoveCursor {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InputEvent_InsertText {
    // message fields
    pub text: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InputEvent_InsertText {}

impl InputEvent_InsertText {
    pub fn new() -> InputEvent_InsertText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InputEvent_InsertText {
        static mut instance: ::protobuf::lazy::Lazy<InputEvent_InsertText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InputEvent_InsertText,
        };
        unsafe {
            instance.get(InputEvent_InsertText::new)
        }
    }

    // string text = 1;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        &mut self.text
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.text, ::std::string::String::new())
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    fn get_text_for_reflect(&self) -> &::std::string::String {
        &self.text
    }

    fn mut_text_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.text
    }
}

impl ::protobuf::Message for InputEvent_InsertText {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.text)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.text.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.text);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.text.is_empty() {
            os.write_string(1, &self.text)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InputEvent_InsertText {
    fn new() -> InputEvent_InsertText {
        InputEvent_InsertText::new()
    }

    fn descriptor_static(_: ::std::option::Option<InputEvent_InsertText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    InputEvent_InsertText::get_text_for_reflect,
                    InputEvent_InsertText::mut_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InputEvent_InsertText>(
                    "InputEvent_InsertText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InputEvent_InsertText {
    fn clear(&mut self) {
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InputEvent_InsertText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InputEvent_InsertText {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InputEvent_OpenFile {
    // message fields
    pub path: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InputEvent_OpenFile {}

impl InputEvent_OpenFile {
    pub fn new() -> InputEvent_OpenFile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InputEvent_OpenFile {
        static mut instance: ::protobuf::lazy::Lazy<InputEvent_OpenFile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InputEvent_OpenFile,
        };
        unsafe {
            instance.get(InputEvent_OpenFile::new)
        }
    }

    // string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.path, ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    fn get_path_for_reflect(&self) -> &::std::string::String {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }
}

impl ::protobuf::Message for InputEvent_OpenFile {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.path)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.path.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.path);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.path.is_empty() {
            os.write_string(1, &self.path)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InputEvent_OpenFile {
    fn new() -> InputEvent_OpenFile {
        InputEvent_OpenFile::new()
    }

    fn descriptor_static(_: ::std::option::Option<InputEvent_OpenFile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    InputEvent_OpenFile::get_path_for_reflect,
                    InputEvent_OpenFile::mut_path_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InputEvent_OpenFile>(
                    "InputEvent_OpenFile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InputEvent_OpenFile {
    fn clear(&mut self) {
        self.clear_path();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InputEvent_OpenFile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InputEvent_OpenFile {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InputEvent_SaveFile {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InputEvent_SaveFile {}

impl InputEvent_SaveFile {
    pub fn new() -> InputEvent_SaveFile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InputEvent_SaveFile {
        static mut instance: ::protobuf::lazy::Lazy<InputEvent_SaveFile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InputEvent_SaveFile,
        };
        unsafe {
            instance.get(InputEvent_SaveFile::new)
        }
    }
}

impl ::protobuf::Message for InputEvent_SaveFile {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InputEvent_SaveFile {
    fn new() -> InputEvent_SaveFile {
        InputEvent_SaveFile::new()
    }

    fn descriptor_static(_: ::std::option::Option<InputEvent_SaveFile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<InputEvent_SaveFile>(
                    "InputEvent_SaveFile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InputEvent_SaveFile {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InputEvent_SaveFile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InputEvent_SaveFile {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EventReply {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EventReply {}

impl EventReply {
    pub fn new() -> EventReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EventReply {
        static mut instance: ::protobuf::lazy::Lazy<EventReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EventReply,
        };
        unsafe {
            instance.get(EventReply::new)
        }
    }
}

impl ::protobuf::Message for EventReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EventReply {
    fn new() -> EventReply {
        EventReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<EventReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<EventReply>(
                    "EventReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EventReply {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EventReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EventReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ViewStateRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ViewStateRequest {}

impl ViewStateRequest {
    pub fn new() -> ViewStateRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ViewStateRequest {
        static mut instance: ::protobuf::lazy::Lazy<ViewStateRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ViewStateRequest,
        };
        unsafe {
            instance.get(ViewStateRequest::new)
        }
    }
}

impl ::protobuf::Message for ViewStateRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ViewStateRequest {
    fn new() -> ViewStateRequest {
        ViewStateRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ViewStateRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ViewStateRequest>(
                    "ViewStateRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ViewStateRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ViewStateRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ViewStateRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ViewStateReply {
    // message fields
    lines: ::protobuf::RepeatedField<Line>,
    pub cursorX: i32,
    pub cursorY: i32,
    pub syntax_tree: ::std::string::String,
    pub lexing_time_nanos: i64,
    pub parsing_time_nanos: i64,
    pub file: ::std::string::String,
    pub dirty: bool,
    pub reparse_len: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ViewStateReply {}

impl ViewStateReply {
    pub fn new() -> ViewStateReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ViewStateReply {
        static mut instance: ::protobuf::lazy::Lazy<ViewStateReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ViewStateReply,
        };
        unsafe {
            instance.get(ViewStateReply::new)
        }
    }

    // repeated .editor.Line lines = 1;

    pub fn clear_lines(&mut self) {
        self.lines.clear();
    }

    // Param is passed by value, moved
    pub fn set_lines(&mut self, v: ::protobuf::RepeatedField<Line>) {
        self.lines = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lines(&mut self) -> &mut ::protobuf::RepeatedField<Line> {
        &mut self.lines
    }

    // Take field
    pub fn take_lines(&mut self) -> ::protobuf::RepeatedField<Line> {
        ::std::mem::replace(&mut self.lines, ::protobuf::RepeatedField::new())
    }

    pub fn get_lines(&self) -> &[Line] {
        &self.lines
    }

    fn get_lines_for_reflect(&self) -> &::protobuf::RepeatedField<Line> {
        &self.lines
    }

    fn mut_lines_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Line> {
        &mut self.lines
    }

    // int32 cursorX = 2;

    pub fn clear_cursorX(&mut self) {
        self.cursorX = 0;
    }

    // Param is passed by value, moved
    pub fn set_cursorX(&mut self, v: i32) {
        self.cursorX = v;
    }

    pub fn get_cursorX(&self) -> i32 {
        self.cursorX
    }

    fn get_cursorX_for_reflect(&self) -> &i32 {
        &self.cursorX
    }

    fn mut_cursorX_for_reflect(&mut self) -> &mut i32 {
        &mut self.cursorX
    }

    // int32 cursorY = 3;

    pub fn clear_cursorY(&mut self) {
        self.cursorY = 0;
    }

    // Param is passed by value, moved
    pub fn set_cursorY(&mut self, v: i32) {
        self.cursorY = v;
    }

    pub fn get_cursorY(&self) -> i32 {
        self.cursorY
    }

    fn get_cursorY_for_reflect(&self) -> &i32 {
        &self.cursorY
    }

    fn mut_cursorY_for_reflect(&mut self) -> &mut i32 {
        &mut self.cursorY
    }

    // string syntax_tree = 4;

    pub fn clear_syntax_tree(&mut self) {
        self.syntax_tree.clear();
    }

    // Param is passed by value, moved
    pub fn set_syntax_tree(&mut self, v: ::std::string::String) {
        self.syntax_tree = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_syntax_tree(&mut self) -> &mut ::std::string::String {
        &mut self.syntax_tree
    }

    // Take field
    pub fn take_syntax_tree(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.syntax_tree, ::std::string::String::new())
    }

    pub fn get_syntax_tree(&self) -> &str {
        &self.syntax_tree
    }

    fn get_syntax_tree_for_reflect(&self) -> &::std::string::String {
        &self.syntax_tree
    }

    fn mut_syntax_tree_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.syntax_tree
    }

    // int64 lexing_time_nanos = 5;

    pub fn clear_lexing_time_nanos(&mut self) {
        self.lexing_time_nanos = 0;
    }

    // Param is passed by value, moved
    pub fn set_lexing_time_nanos(&mut self, v: i64) {
        self.lexing_time_nanos = v;
    }

    pub fn get_lexing_time_nanos(&self) -> i64 {
        self.lexing_time_nanos
    }

    fn get_lexing_time_nanos_for_reflect(&self) -> &i64 {
        &self.lexing_time_nanos
    }

    fn mut_lexing_time_nanos_for_reflect(&mut self) -> &mut i64 {
        &mut self.lexing_time_nanos
    }

    // int64 parsing_time_nanos = 6;

    pub fn clear_parsing_time_nanos(&mut self) {
        self.parsing_time_nanos = 0;
    }

    // Param is passed by value, moved
    pub fn set_parsing_time_nanos(&mut self, v: i64) {
        self.parsing_time_nanos = v;
    }

    pub fn get_parsing_time_nanos(&self) -> i64 {
        self.parsing_time_nanos
    }

    fn get_parsing_time_nanos_for_reflect(&self) -> &i64 {
        &self.parsing_time_nanos
    }

    fn mut_parsing_time_nanos_for_reflect(&mut self) -> &mut i64 {
        &mut self.parsing_time_nanos
    }

    // string file = 7;

    pub fn clear_file(&mut self) {
        self.file.clear();
    }

    // Param is passed by value, moved
    pub fn set_file(&mut self, v: ::std::string::String) {
        self.file = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file(&mut self) -> &mut ::std::string::String {
        &mut self.file
    }

    // Take field
    pub fn take_file(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.file, ::std::string::String::new())
    }

    pub fn get_file(&self) -> &str {
        &self.file
    }

    fn get_file_for_reflect(&self) -> &::std::string::String {
        &self.file
    }

    fn mut_file_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.file
    }

    // bool dirty = 8;

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    // Param is passed by value, moved
    pub fn set_dirty(&mut self, v: bool) {
        self.dirty = v;
    }

    pub fn get_dirty(&self) -> bool {
        self.dirty
    }

    fn get_dirty_for_reflect(&self) -> &bool {
        &self.dirty
    }

    fn mut_dirty_for_reflect(&mut self) -> &mut bool {
        &mut self.dirty
    }

    // int32 reparse_len = 9;

    pub fn clear_reparse_len(&mut self) {
        self.reparse_len = 0;
    }

    // Param is passed by value, moved
    pub fn set_reparse_len(&mut self, v: i32) {
        self.reparse_len = v;
    }

    pub fn get_reparse_len(&self) -> i32 {
        self.reparse_len
    }

    fn get_reparse_len_for_reflect(&self) -> &i32 {
        &self.reparse_len
    }

    fn mut_reparse_len_for_reflect(&mut self) -> &mut i32 {
        &mut self.reparse_len
    }
}

impl ::protobuf::Message for ViewStateReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.lines)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.cursorX = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.cursorY = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.syntax_tree)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.lexing_time_nanos = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.parsing_time_nanos = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.file)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.dirty = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.reparse_len = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.lines {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.cursorX != 0 {
            my_size += ::protobuf::rt::value_size(2, self.cursorX, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.cursorY != 0 {
            my_size += ::protobuf::rt::value_size(3, self.cursorY, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.syntax_tree.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.syntax_tree);
        };
        if self.lexing_time_nanos != 0 {
            my_size += ::protobuf::rt::value_size(5, self.lexing_time_nanos, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.parsing_time_nanos != 0 {
            my_size += ::protobuf::rt::value_size(6, self.parsing_time_nanos, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.file.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.file);
        };
        if self.dirty != false {
            my_size += 2;
        };
        if self.reparse_len != 0 {
            my_size += ::protobuf::rt::value_size(9, self.reparse_len, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.lines {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.cursorX != 0 {
            os.write_int32(2, self.cursorX)?;
        };
        if self.cursorY != 0 {
            os.write_int32(3, self.cursorY)?;
        };
        if !self.syntax_tree.is_empty() {
            os.write_string(4, &self.syntax_tree)?;
        };
        if self.lexing_time_nanos != 0 {
            os.write_int64(5, self.lexing_time_nanos)?;
        };
        if self.parsing_time_nanos != 0 {
            os.write_int64(6, self.parsing_time_nanos)?;
        };
        if !self.file.is_empty() {
            os.write_string(7, &self.file)?;
        };
        if self.dirty != false {
            os.write_bool(8, self.dirty)?;
        };
        if self.reparse_len != 0 {
            os.write_int32(9, self.reparse_len)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ViewStateReply {
    fn new() -> ViewStateReply {
        ViewStateReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<ViewStateReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Line>>(
                    "lines",
                    ViewStateReply::get_lines_for_reflect,
                    ViewStateReply::mut_lines_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "cursorX",
                    ViewStateReply::get_cursorX_for_reflect,
                    ViewStateReply::mut_cursorX_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "cursorY",
                    ViewStateReply::get_cursorY_for_reflect,
                    ViewStateReply::mut_cursorY_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "syntax_tree",
                    ViewStateReply::get_syntax_tree_for_reflect,
                    ViewStateReply::mut_syntax_tree_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "lexing_time_nanos",
                    ViewStateReply::get_lexing_time_nanos_for_reflect,
                    ViewStateReply::mut_lexing_time_nanos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "parsing_time_nanos",
                    ViewStateReply::get_parsing_time_nanos_for_reflect,
                    ViewStateReply::mut_parsing_time_nanos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "file",
                    ViewStateReply::get_file_for_reflect,
                    ViewStateReply::mut_file_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "dirty",
                    ViewStateReply::get_dirty_for_reflect,
                    ViewStateReply::mut_dirty_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "reparse_len",
                    ViewStateReply::get_reparse_len_for_reflect,
                    ViewStateReply::mut_reparse_len_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ViewStateReply>(
                    "ViewStateReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ViewStateReply {
    fn clear(&mut self) {
        self.clear_lines();
        self.clear_cursorX();
        self.clear_cursorY();
        self.clear_syntax_tree();
        self.clear_lexing_time_nanos();
        self.clear_parsing_time_nanos();
        self.clear_file();
        self.clear_dirty();
        self.clear_reparse_len();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ViewStateReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ViewStateReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Line {
    // message fields
    ranges: ::protobuf::RepeatedField<StyledText>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Line {}

impl Line {
    pub fn new() -> Line {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Line {
        static mut instance: ::protobuf::lazy::Lazy<Line> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Line,
        };
        unsafe {
            instance.get(Line::new)
        }
    }

    // repeated .editor.StyledText ranges = 1;

    pub fn clear_ranges(&mut self) {
        self.ranges.clear();
    }

    // Param is passed by value, moved
    pub fn set_ranges(&mut self, v: ::protobuf::RepeatedField<StyledText>) {
        self.ranges = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ranges(&mut self) -> &mut ::protobuf::RepeatedField<StyledText> {
        &mut self.ranges
    }

    // Take field
    pub fn take_ranges(&mut self) -> ::protobuf::RepeatedField<StyledText> {
        ::std::mem::replace(&mut self.ranges, ::protobuf::RepeatedField::new())
    }

    pub fn get_ranges(&self) -> &[StyledText] {
        &self.ranges
    }

    fn get_ranges_for_reflect(&self) -> &::protobuf::RepeatedField<StyledText> {
        &self.ranges
    }

    fn mut_ranges_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<StyledText> {
        &mut self.ranges
    }
}

impl ::protobuf::Message for Line {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ranges)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.ranges {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.ranges {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Line {
    fn new() -> Line {
        Line::new()
    }

    fn descriptor_static(_: ::std::option::Option<Line>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StyledText>>(
                    "ranges",
                    Line::get_ranges_for_reflect,
                    Line::mut_ranges_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Line>(
                    "Line",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Line {
    fn clear(&mut self) {
        self.clear_ranges();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Line {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Line {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StyledText {
    // message fields
    pub text: ::std::string::String,
    pub style: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StyledText {}

impl StyledText {
    pub fn new() -> StyledText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StyledText {
        static mut instance: ::protobuf::lazy::Lazy<StyledText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StyledText,
        };
        unsafe {
            instance.get(StyledText::new)
        }
    }

    // string text = 1;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        &mut self.text
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.text, ::std::string::String::new())
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    fn get_text_for_reflect(&self) -> &::std::string::String {
        &self.text
    }

    fn mut_text_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.text
    }

    // string style = 2;

    pub fn clear_style(&mut self) {
        self.style.clear();
    }

    // Param is passed by value, moved
    pub fn set_style(&mut self, v: ::std::string::String) {
        self.style = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_style(&mut self) -> &mut ::std::string::String {
        &mut self.style
    }

    // Take field
    pub fn take_style(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.style, ::std::string::String::new())
    }

    pub fn get_style(&self) -> &str {
        &self.style
    }

    fn get_style_for_reflect(&self) -> &::std::string::String {
        &self.style
    }

    fn mut_style_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.style
    }
}

impl ::protobuf::Message for StyledText {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.text)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.style)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.text.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.text);
        };
        if !self.style.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.style);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.text.is_empty() {
            os.write_string(1, &self.text)?;
        };
        if !self.style.is_empty() {
            os.write_string(2, &self.style)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StyledText {
    fn new() -> StyledText {
        StyledText::new()
    }

    fn descriptor_static(_: ::std::option::Option<StyledText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    StyledText::get_text_for_reflect,
                    StyledText::mut_text_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "style",
                    StyledText::get_style_for_reflect,
                    StyledText::mut_style_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StyledText>(
                    "StyledText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StyledText {
    fn clear(&mut self) {
        self.clear_text();
        self.clear_style();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StyledText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StyledText {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Direction {
    UP = 0,
    RIGHT = 1,
    DOWN = 2,
    LEFT = 3,
}

impl ::protobuf::ProtobufEnum for Direction {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Direction> {
        match value {
            0 => ::std::option::Option::Some(Direction::UP),
            1 => ::std::option::Option::Some(Direction::RIGHT),
            2 => ::std::option::Option::Some(Direction::DOWN),
            3 => ::std::option::Option::Some(Direction::LEFT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Direction] = &[
            Direction::UP,
            Direction::RIGHT,
            Direction::DOWN,
            Direction::LEFT,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Direction>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Direction", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Direction {
}

impl ::std::default::Default for Direction {
    fn default() -> Self {
        Direction::UP
    }
}

impl ::protobuf::reflect::ProtobufValue for Direction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Amount {
    CHAR = 0,
    PAGE = 1,
}

impl ::protobuf::ProtobufEnum for Amount {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Amount> {
        match value {
            0 => ::std::option::Option::Some(Amount::CHAR),
            1 => ::std::option::Option::Some(Amount::PAGE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Amount] = &[
            Amount::CHAR,
            Amount::PAGE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Amount>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Amount", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Amount {
}

impl ::std::default::Default for Amount {
    fn default() -> Self {
        Amount::CHAR
    }
}

impl ::protobuf::reflect::ProtobufValue for Amount {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x12, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x65, 0x64, 0x69, 0x74, 0x6f, 0x72, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x06, 0x65, 0x64, 0x69, 0x74, 0x6f, 0x72, 0x22, 0x80, 0x04, 0x0a,
    0x0a, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x30, 0x0a, 0x05, 0x72,
    0x65, 0x61, 0x64, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x65, 0x64, 0x69,
    0x74, 0x6f, 0x72, 0x2e, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x52,
    0x65, 0x61, 0x64, 0x79, 0x48, 0x00, 0x52, 0x05, 0x72, 0x65, 0x61, 0x64, 0x79, 0x12, 0x40, 0x0a,
    0x0b, 0x6d, 0x6f, 0x76, 0x65, 0x5f, 0x63, 0x75, 0x72, 0x73, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x65, 0x64, 0x69, 0x74, 0x6f, 0x72, 0x2e, 0x49, 0x6e, 0x70, 0x75,
    0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x4d, 0x6f, 0x76, 0x65, 0x43, 0x75, 0x72, 0x73, 0x6f,
    0x72, 0x48, 0x00, 0x52, 0x0a, 0x6d, 0x6f, 0x76, 0x65, 0x43, 0x75, 0x72, 0x73, 0x6f, 0x72, 0x12,
    0x40, 0x0a, 0x0b, 0x69, 0x6e, 0x73, 0x65, 0x72, 0x74, 0x5f, 0x74, 0x65, 0x78, 0x74, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x65, 0x64, 0x69, 0x74, 0x6f, 0x72, 0x2e, 0x49, 0x6e,
    0x70, 0x75, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x49, 0x6e, 0x73, 0x65, 0x72, 0x74, 0x54,
    0x65, 0x78, 0x74, 0x48, 0x00, 0x52, 0x0a, 0x69, 0x6e, 0x73, 0x65, 0x72, 0x74, 0x54, 0x65, 0x78,
    0x74, 0x12, 0x3a, 0x0a, 0x09, 0x6f, 0x70, 0x65, 0x6e, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x65, 0x64, 0x69, 0x74, 0x6f, 0x72, 0x2e, 0x49, 0x6e,
    0x70, 0x75, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x4f, 0x70, 0x65, 0x6e, 0x46, 0x69, 0x6c,
    0x65, 0x48, 0x00, 0x52, 0x08, 0x6f, 0x70, 0x65, 0x6e, 0x46, 0x69, 0x6c, 0x65, 0x12, 0x3a, 0x0a,
    0x09, 0x73, 0x61, 0x76, 0x65, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1b, 0x2e, 0x65, 0x64, 0x69, 0x74, 0x6f, 0x72, 0x2e, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x45,
    0x76, 0x65, 0x6e, 0x74, 0x2e, 0x53, 0x61, 0x76, 0x65, 0x46, 0x69, 0x6c, 0x65, 0x48, 0x00, 0x52,
    0x08, 0x73, 0x61, 0x76, 0x65, 0x46, 0x69, 0x6c, 0x65, 0x1a, 0x07, 0x0a, 0x05, 0x52, 0x65, 0x61,
    0x64, 0x79, 0x1a, 0x65, 0x0a, 0x0a, 0x4d, 0x6f, 0x76, 0x65, 0x43, 0x75, 0x72, 0x73, 0x6f, 0x72,
    0x12, 0x2f, 0x0a, 0x09, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x65, 0x64, 0x69, 0x74, 0x6f, 0x72, 0x2e, 0x44, 0x69, 0x72,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x09, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x26, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x0e, 0x2e, 0x65, 0x64, 0x69, 0x74, 0x6f, 0x72, 0x2e, 0x41, 0x6d, 0x6f, 0x75, 0x6e,
    0x74, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x1a, 0x20, 0x0a, 0x0a, 0x49, 0x6e, 0x73,
    0x65, 0x72, 0x74, 0x54, 0x65, 0x78, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x74, 0x65, 0x78, 0x74, 0x1a, 0x1e, 0x0a, 0x08, 0x4f,
    0x70, 0x65, 0x6e, 0x46, 0x69, 0x6c, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x1a, 0x0a, 0x0a, 0x08, 0x53,
    0x61, 0x76, 0x65, 0x46, 0x69, 0x6c, 0x65, 0x42, 0x06, 0x0a, 0x04, 0x6b, 0x69, 0x6e, 0x64, 0x22,
    0x0c, 0x0a, 0x0a, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x12, 0x0a,
    0x10, 0x56, 0x69, 0x65, 0x77, 0x53, 0x74, 0x61, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x22, 0xae, 0x02, 0x0a, 0x0e, 0x56, 0x69, 0x65, 0x77, 0x53, 0x74, 0x61, 0x74, 0x65, 0x52,
    0x65, 0x70, 0x6c, 0x79, 0x12, 0x22, 0x0a, 0x05, 0x6c, 0x69, 0x6e, 0x65, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x65, 0x64, 0x69, 0x74, 0x6f, 0x72, 0x2e, 0x4c, 0x69, 0x6e,
    0x65, 0x52, 0x05, 0x6c, 0x69, 0x6e, 0x65, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x75, 0x72, 0x73,
    0x6f, 0x72, 0x58, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x63, 0x75, 0x72, 0x73, 0x6f,
    0x72, 0x58, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x75, 0x72, 0x73, 0x6f, 0x72, 0x59, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x07, 0x63, 0x75, 0x72, 0x73, 0x6f, 0x72, 0x59, 0x12, 0x1f, 0x0a, 0x0b,
    0x73, 0x79, 0x6e, 0x74, 0x61, 0x78, 0x5f, 0x74, 0x72, 0x65, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x0a, 0x73, 0x79, 0x6e, 0x74, 0x61, 0x78, 0x54, 0x72, 0x65, 0x65, 0x12, 0x2a, 0x0a,
    0x11, 0x6c, 0x65, 0x78, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x6e, 0x61, 0x6e,
    0x6f, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0f, 0x6c, 0x65, 0x78, 0x69, 0x6e, 0x67,
    0x54, 0x69, 0x6d, 0x65, 0x4e, 0x61, 0x6e, 0x6f, 0x73, 0x12, 0x2c, 0x0a, 0x12, 0x70, 0x61, 0x72,
    0x73, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x6e, 0x61, 0x6e, 0x6f, 0x73, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x03, 0x52, 0x10, 0x70, 0x61, 0x72, 0x73, 0x69, 0x6e, 0x67, 0x54, 0x69,
    0x6d, 0x65, 0x4e, 0x61, 0x6e, 0x6f, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x66, 0x69, 0x6c, 0x65, 0x18,
    0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x66, 0x69, 0x6c, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x64,
    0x69, 0x72, 0x74, 0x79, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x52, 0x05, 0x64, 0x69, 0x72, 0x74,
    0x79, 0x12, 0x1f, 0x0a, 0x0b, 0x72, 0x65, 0x70, 0x61, 0x72, 0x73, 0x65, 0x5f, 0x6c, 0x65, 0x6e,
    0x18, 0x09, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x72, 0x65, 0x70, 0x61, 0x72, 0x73, 0x65, 0x4c,
    0x65, 0x6e, 0x22, 0x32, 0x0a, 0x04, 0x4c, 0x69, 0x6e, 0x65, 0x12, 0x2a, 0x0a, 0x06, 0x72, 0x61,
    0x6e, 0x67, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x65, 0x64, 0x69,
    0x74, 0x6f, 0x72, 0x2e, 0x53, 0x74, 0x79, 0x6c, 0x65, 0x64, 0x54, 0x65, 0x78, 0x74, 0x52, 0x06,
    0x72, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x22, 0x36, 0x0a, 0x0a, 0x53, 0x74, 0x79, 0x6c, 0x65, 0x64,
    0x54, 0x65, 0x78, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x04, 0x74, 0x65, 0x78, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x73, 0x74, 0x79, 0x6c,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x73, 0x74, 0x79, 0x6c, 0x65, 0x2a, 0x32,
    0x0a, 0x09, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x06, 0x0a, 0x02, 0x55,
    0x50, 0x10, 0x00, 0x12, 0x09, 0x0a, 0x05, 0x52, 0x49, 0x47, 0x48, 0x54, 0x10, 0x01, 0x12, 0x08,
    0x0a, 0x04, 0x44, 0x4f, 0x57, 0x4e, 0x10, 0x02, 0x12, 0x08, 0x0a, 0x04, 0x4c, 0x45, 0x46, 0x54,
    0x10, 0x03, 0x2a, 0x1c, 0x0a, 0x06, 0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x08, 0x0a, 0x04,
    0x43, 0x48, 0x41, 0x52, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x50, 0x41, 0x47, 0x45, 0x10, 0x01,
    0x32, 0x7c, 0x0a, 0x06, 0x45, 0x64, 0x69, 0x74, 0x6f, 0x72, 0x12, 0x31, 0x0a, 0x05, 0x65, 0x76,
    0x65, 0x6e, 0x74, 0x12, 0x12, 0x2e, 0x65, 0x64, 0x69, 0x74, 0x6f, 0x72, 0x2e, 0x49, 0x6e, 0x70,
    0x75, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x1a, 0x12, 0x2e, 0x65, 0x64, 0x69, 0x74, 0x6f, 0x72,
    0x2e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x00, 0x12, 0x3f, 0x0a,
    0x07, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x12, 0x18, 0x2e, 0x65, 0x64, 0x69, 0x74, 0x6f,
    0x72, 0x2e, 0x56, 0x69, 0x65, 0x77, 0x53, 0x74, 0x61, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x1a, 0x16, 0x2e, 0x65, 0x64, 0x69, 0x74, 0x6f, 0x72, 0x2e, 0x56, 0x69, 0x65, 0x77,
    0x53, 0x74, 0x61, 0x74, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x00, 0x30, 0x01, 0x42, 0x14,
    0x0a, 0x10, 0x72, 0x75, 0x2e, 0x6d, 0x61, 0x74, 0x6b, 0x6c, 0x61, 0x64, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x50, 0x01, 0x4a, 0xcd, 0x12, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x52, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x02, 0x00, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x02, 0x00, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x02, 0x07, 0x1a, 0x0a, 0x0d,
    0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x02, 0x07, 0x1a, 0x0a, 0x0e, 0x0a,
    0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x02, 0x07, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x02, 0x1d, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x03, 0x00, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x03,
    0x00, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x03, 0x07, 0x13,
    0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x03, 0x07, 0x13, 0x0a,
    0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x07, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x07, 0x12, 0x03, 0x03, 0x16, 0x28, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x05, 0x08, 0x0e, 0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x07,
    0x00, 0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x0e, 0x0a,
    0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x04, 0x08, 0x04, 0x09, 0x05, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x08, 0x0f, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x08, 0x24, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12,
    0x04, 0x0a, 0x04, 0x0b, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x0a, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0a, 0x11,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0a, 0x2c, 0x32, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x33, 0x41, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x0e, 0x00, 0x2a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x0e, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x00, 0x12, 0x04, 0x0f,
    0x04, 0x10, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x0c,
    0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x01, 0x12, 0x04, 0x12, 0x04, 0x15, 0x05, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x01, 0x01, 0x12, 0x03, 0x12, 0x0c, 0x16, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x12, 0x03, 0x13, 0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x13, 0x08, 0x12, 0x18, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x13, 0x08, 0x11, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x12, 0x1b, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x13, 0x1e, 0x1f, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x12, 0x03, 0x14, 0x08, 0x1a, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x14, 0x08, 0x13, 0x20, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x14, 0x08, 0x0e, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x0f, 0x15, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x18, 0x19, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x00, 0x03, 0x02, 0x12, 0x04, 0x17, 0x04, 0x19, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x03, 0x02, 0x01, 0x12, 0x03, 0x17, 0x0c, 0x16, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03,
    0x02, 0x02, 0x00, 0x12, 0x03, 0x18, 0x08, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x18, 0x08, 0x17, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x18, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x0f, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03,
    0x03, 0x12, 0x04, 0x1b, 0x04, 0x1d, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x03, 0x01,
    0x12, 0x03, 0x1b, 0x0c, 0x14, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x03, 0x02, 0x00, 0x12,
    0x03, 0x1c, 0x08, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x1c, 0x08, 0x1b, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x1c, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x1c, 0x0f, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x1c, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x04, 0x12, 0x04, 0x1f,
    0x04, 0x21, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x04, 0x01, 0x12, 0x03, 0x1f, 0x0c,
    0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x08, 0x00, 0x12, 0x04, 0x23, 0x04, 0x29, 0x05, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x08, 0x00, 0x01, 0x12, 0x03, 0x23, 0x0a, 0x0e, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x24, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x24, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x24, 0x0e, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x24, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x25, 0x08,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x25, 0x08, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x25, 0x13, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x25, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x26, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x26, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x26, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x26,
    0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x27, 0x08, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x27, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x27, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x27, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x04, 0x12, 0x03, 0x28, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12,
    0x03, 0x28, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x28,
    0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x28, 0x1d, 0x1e,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x2c, 0x00, 0x2d, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04,
    0x2f, 0x00, 0x34, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x05, 0x0e,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x30, 0x04, 0x0b, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x30, 0x04, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x30, 0x09, 0x0a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x31, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x31, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x31,
    0x0c, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x32, 0x04, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x32, 0x04, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x32, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x33, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x33, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12,
    0x03, 0x33, 0x0b, 0x0c, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x36, 0x00, 0x39, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x36, 0x05, 0x0b, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x37, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x37, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x37, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x38,
    0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x38, 0x04, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x38, 0x0b, 0x0c, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x3c, 0x00, 0x3d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x3c, 0x08, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x3f, 0x00,
    0x49, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x3f, 0x08, 0x16, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x40, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x40, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x40, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x40, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x40, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x41, 0x04, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x04, 0x41, 0x04, 0x40, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x41, 0x04, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x41, 0x0a, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x41, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x02, 0x12, 0x03, 0x42, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x42, 0x04, 0x41, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x42, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x42, 0x0a,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x42, 0x14, 0x15, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x43, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x04, 0x43, 0x04, 0x42, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x43, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x43, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x43, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x44,
    0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x04, 0x44, 0x04, 0x43,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x03, 0x44, 0x04, 0x09, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x44, 0x0a, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x44, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x05, 0x12, 0x03, 0x45, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05,
    0x04, 0x12, 0x04, 0x45, 0x04, 0x44, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x45, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x45, 0x0a, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x03, 0x12, 0x03, 0x45, 0x1f,
    0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x06, 0x12, 0x03, 0x46, 0x04, 0x14, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x04, 0x12, 0x04, 0x46, 0x04, 0x45, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x06, 0x05, 0x12, 0x03, 0x46, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x06, 0x01, 0x12, 0x03, 0x46, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x46, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x07, 0x12,
    0x03, 0x47, 0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x04, 0x12, 0x04, 0x47,
    0x04, 0x46, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x05, 0x12, 0x03, 0x47, 0x04,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x01, 0x12, 0x03, 0x47, 0x09, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x03, 0x12, 0x03, 0x47, 0x11, 0x12, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x08, 0x12, 0x03, 0x48, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x08, 0x04, 0x12, 0x04, 0x48, 0x04, 0x47, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x08, 0x05, 0x12, 0x03, 0x48, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x48, 0x0a, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x03, 0x12, 0x03,
    0x48, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x4b, 0x00, 0x4d, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x4b, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x00, 0x12, 0x03, 0x4c, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x4c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x4c, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4c,
    0x18, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4c, 0x21, 0x22,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x4f, 0x00, 0x52, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x05, 0x01, 0x12, 0x03, 0x4f, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00,
    0x12, 0x03, 0x50, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x50, 0x04, 0x4f, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x50,
    0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x50, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x50, 0x12, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x51, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x04, 0x12, 0x04, 0x51, 0x04, 0x50, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x51, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x51, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x51, 0x13, 0x14, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
