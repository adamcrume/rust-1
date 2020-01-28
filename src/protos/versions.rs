// This file is generated by rust-protobuf 2.8.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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
//! Generated file from `tensorflow/core/framework/versions.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_0;

#[derive(PartialEq,Clone,Default)]
pub struct VersionDef {
    // message fields
    pub producer: i32,
    pub min_consumer: i32,
    pub bad_consumers: ::std::vec::Vec<i32>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a VersionDef {
    fn default() -> &'a VersionDef {
        <VersionDef as ::protobuf::Message>::default_instance()
    }
}

impl VersionDef {
    pub fn new() -> VersionDef {
        ::std::default::Default::default()
    }

    // int32 producer = 1;


    pub fn get_producer(&self) -> i32 {
        self.producer
    }
    pub fn clear_producer(&mut self) {
        self.producer = 0;
    }

    // Param is passed by value, moved
    pub fn set_producer(&mut self, v: i32) {
        self.producer = v;
    }

    // int32 min_consumer = 2;


    pub fn get_min_consumer(&self) -> i32 {
        self.min_consumer
    }
    pub fn clear_min_consumer(&mut self) {
        self.min_consumer = 0;
    }

    // Param is passed by value, moved
    pub fn set_min_consumer(&mut self, v: i32) {
        self.min_consumer = v;
    }

    // repeated int32 bad_consumers = 3;


    pub fn get_bad_consumers(&self) -> &[i32] {
        &self.bad_consumers
    }
    pub fn clear_bad_consumers(&mut self) {
        self.bad_consumers.clear();
    }

    // Param is passed by value, moved
    pub fn set_bad_consumers(&mut self, v: ::std::vec::Vec<i32>) {
        self.bad_consumers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bad_consumers(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.bad_consumers
    }

    // Take field
    pub fn take_bad_consumers(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.bad_consumers, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for VersionDef {
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
                    }
                    let tmp = is.read_int32()?;
                    self.producer = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.min_consumer = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.bad_consumers)?;
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
        if self.producer != 0 {
            my_size += ::protobuf::rt::value_size(1, self.producer, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.min_consumer != 0 {
            my_size += ::protobuf::rt::value_size(2, self.min_consumer, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.bad_consumers {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.producer != 0 {
            os.write_int32(1, self.producer)?;
        }
        if self.min_consumer != 0 {
            os.write_int32(2, self.min_consumer)?;
        }
        for v in &self.bad_consumers {
            os.write_int32(3, *v)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> VersionDef {
        VersionDef::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "producer",
                    |m: &VersionDef| { &m.producer },
                    |m: &mut VersionDef| { &mut m.producer },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "min_consumer",
                    |m: &VersionDef| { &m.min_consumer },
                    |m: &mut VersionDef| { &mut m.min_consumer },
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "bad_consumers",
                    |m: &VersionDef| { &m.bad_consumers },
                    |m: &mut VersionDef| { &mut m.bad_consumers },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VersionDef>(
                    "VersionDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static VersionDef {
        static mut instance: ::protobuf::lazy::Lazy<VersionDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VersionDef,
        };
        unsafe {
            instance.get(VersionDef::new)
        }
    }
}

impl ::protobuf::Clear for VersionDef {
    fn clear(&mut self) {
        self.producer = 0;
        self.min_consumer = 0;
        self.bad_consumers.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VersionDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VersionDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(tensorflow/core/framework/versions.proto\x12\ntensorflow\"p\n\nVersio\
    nDef\x12\x1a\n\x08producer\x18\x01\x20\x01(\x05R\x08producer\x12!\n\x0cm\
    in_consumer\x18\x02\x20\x01(\x05R\x0bminConsumer\x12#\n\rbad_consumers\
    \x18\x03\x20\x03(\x05R\x0cbadConsumersBn\n\x18org.tensorflow.frameworkB\
    \x0eVersionsProtosP\x01Z=github.com/tensorflow/tensorflow/tensorflow/go/\
    core/framework\xf8\x01\x01b\x06proto3\
";

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
