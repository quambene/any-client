use prost_types::{
    descriptor_proto::{ExtensionRange, ReservedRange},
    enum_descriptor_proto::EnumReservedRange,
    source_code_info::Location,
    uninterpreted_option::NamePart,
    DescriptorProto, EnumDescriptorProto, EnumOptions, EnumValueDescriptorProto, EnumValueOptions,
    ExtensionRangeOptions, FieldDescriptorProto, FieldOptions, FileDescriptorProto, FileOptions,
    MessageOptions, MethodDescriptorProto, MethodOptions, OneofDescriptorProto, OneofOptions,
    ServiceDescriptorProto, ServiceOptions, SourceCodeInfo, UninterpretedOption,
};

pub fn convert_file_descriptor_proto(
    file_descriptor_proto: protobuf::descriptor::FileDescriptorProto,
) -> FileDescriptorProto {
    FileDescriptorProto {
        name: file_descriptor_proto.name,
        package: file_descriptor_proto.package,
        dependency: file_descriptor_proto.dependency,
        public_dependency: file_descriptor_proto.public_dependency,
        weak_dependency: file_descriptor_proto.weak_dependency,
        message_type: file_descriptor_proto
            .message_type
            .into_iter()
            .map(convert_descriptor_proto)
            .collect(),
        enum_type: file_descriptor_proto
            .enum_type
            .into_iter()
            .map(convert_enum_descriptor_proto)
            .collect(),
        service: file_descriptor_proto
            .service
            .into_iter()
            .map(convert_service_descriptor_proto)
            .collect(),
        extension: file_descriptor_proto
            .extension
            .into_iter()
            .map(convert_field_descriptor_proto)
            .collect(),
        options: file_descriptor_proto.options.0.map(convert_file_options),
        source_code_info: file_descriptor_proto
            .source_code_info
            .0
            .map(convert_source_code_info),
        syntax: file_descriptor_proto.syntax,
    }
}

fn convert_descriptor_proto(
    descriptor_proto: protobuf::descriptor::DescriptorProto,
) -> DescriptorProto {
    DescriptorProto {
        name: descriptor_proto.name,
        field: descriptor_proto
            .field
            .into_iter()
            .map(convert_field_descriptor_proto)
            .collect(),
        extension: descriptor_proto
            .extension
            .into_iter()
            .map(convert_field_descriptor_proto)
            .collect(),
        nested_type: descriptor_proto
            .nested_type
            .into_iter()
            .map(convert_descriptor_proto)
            .collect(),
        enum_type: descriptor_proto
            .enum_type
            .into_iter()
            .map(convert_enum_descriptor_proto)
            .collect(),
        extension_range: descriptor_proto
            .extension_range
            .into_iter()
            .map(convert_extension_range)
            .collect(),
        oneof_decl: descriptor_proto
            .oneof_decl
            .into_iter()
            .map(convert_oneof_descriptor_proto)
            .collect(),
        options: descriptor_proto.options.0.map(convert_message_options),
        reserved_range: descriptor_proto
            .reserved_range
            .into_iter()
            .map(convert_reserved_range)
            .collect(),
        reserved_name: descriptor_proto.reserved_name,
    }
}

fn convert_reserved_range(
    reserved_range: protobuf::descriptor::descriptor_proto::ReservedRange,
) -> ReservedRange {
    ReservedRange {
        start: reserved_range.start,
        end: reserved_range.end,
    }
}

fn convert_oneof_descriptor_proto(
    oneof_descriptor_proto: protobuf::descriptor::OneofDescriptorProto,
) -> OneofDescriptorProto {
    OneofDescriptorProto {
        name: oneof_descriptor_proto.name,
        options: oneof_descriptor_proto.options.0.map(convert_oneof_options),
    }
}

fn convert_oneof_options(oneof_options: Box<protobuf::descriptor::OneofOptions>) -> OneofOptions {
    OneofOptions {
        uninterpreted_option: oneof_options
            .uninterpreted_option
            .into_iter()
            .map(convert_uninterpreted_option)
            .collect(),
    }
}

fn convert_extension_range(
    extension_range: protobuf::descriptor::descriptor_proto::ExtensionRange,
) -> ExtensionRange {
    ExtensionRange {
        start: extension_range.start,
        end: extension_range.end,
        options: extension_range
            .options
            .0
            .map(convert_extension_range_options),
    }
}

fn convert_extension_range_options(
    extension_range_options: Box<protobuf::descriptor::ExtensionRangeOptions>,
) -> ExtensionRangeOptions {
    ExtensionRangeOptions {
        uninterpreted_option: extension_range_options
            .uninterpreted_option
            .into_iter()
            .map(convert_uninterpreted_option)
            .collect(),
    }
}

fn convert_enum_descriptor_proto(
    enum_descriptor_proto: protobuf::descriptor::EnumDescriptorProto,
) -> EnumDescriptorProto {
    EnumDescriptorProto {
        name: enum_descriptor_proto.name,
        value: enum_descriptor_proto
            .value
            .into_iter()
            .map(convert_enum_value_descriptor_proto)
            .collect(),
        options: enum_descriptor_proto.options.0.map(convert_enum_options),
        reserved_range: enum_descriptor_proto
            .reserved_range
            .into_iter()
            .map(convert_enum_reserved_range)
            .collect(),
        reserved_name: enum_descriptor_proto.reserved_name,
    }
}

fn convert_enum_value_descriptor_proto(
    enum_value_descriptor_proto: protobuf::descriptor::EnumValueDescriptorProto,
) -> EnumValueDescriptorProto {
    EnumValueDescriptorProto {
        name: enum_value_descriptor_proto.name,
        number: enum_value_descriptor_proto.number,
        options: enum_value_descriptor_proto
            .options
            .0
            .map(convert_enum_value_options),
    }
}

fn convert_enum_value_options(
    enum_value_options: Box<protobuf::descriptor::EnumValueOptions>,
) -> EnumValueOptions {
    EnumValueOptions {
        deprecated: enum_value_options.deprecated,
        uninterpreted_option: enum_value_options
            .uninterpreted_option
            .into_iter()
            .map(convert_uninterpreted_option)
            .collect(),
    }
}

fn convert_enum_options(enum_options: Box<protobuf::descriptor::EnumOptions>) -> EnumOptions {
    EnumOptions {
        allow_alias: enum_options.allow_alias,
        deprecated: enum_options.deprecated,
        uninterpreted_option: enum_options
            .uninterpreted_option
            .into_iter()
            .map(convert_uninterpreted_option)
            .collect(),
    }
}

fn convert_enum_reserved_range(
    enum_reserved_range: protobuf::descriptor::enum_descriptor_proto::EnumReservedRange,
) -> EnumReservedRange {
    EnumReservedRange {
        start: enum_reserved_range.start,
        end: enum_reserved_range.end,
    }
}

fn convert_service_descriptor_proto(
    service_descriptor_proto: protobuf::descriptor::ServiceDescriptorProto,
) -> ServiceDescriptorProto {
    ServiceDescriptorProto {
        name: service_descriptor_proto.name,
        method: service_descriptor_proto
            .method
            .into_iter()
            .map(convert_method_descriptor_proto)
            .collect(),
        options: service_descriptor_proto
            .options
            .0
            .map(convert_service_options),
    }
}

fn convert_field_descriptor_proto(
    field_descriptor_proto: protobuf::descriptor::FieldDescriptorProto,
) -> FieldDescriptorProto {
    FieldDescriptorProto {
        name: field_descriptor_proto.name,
        number: field_descriptor_proto.number,
        label: field_descriptor_proto.label.map(|el| el.value()),
        r#type: field_descriptor_proto.type_.map(|el| el.value()),
        type_name: field_descriptor_proto.type_name,
        extendee: field_descriptor_proto.extendee,
        default_value: field_descriptor_proto.default_value,
        oneof_index: field_descriptor_proto.oneof_index,
        json_name: field_descriptor_proto.json_name,
        options: field_descriptor_proto.options.0.map(convert_field_options),
        proto3_optional: field_descriptor_proto.proto3_optional,
    }
}

fn convert_message_options(
    message_options: Box<protobuf::descriptor::MessageOptions>,
) -> MessageOptions {
    MessageOptions {
        message_set_wire_format: message_options.message_set_wire_format,
        no_standard_descriptor_accessor: message_options.no_standard_descriptor_accessor,
        deprecated: message_options.deprecated,
        map_entry: message_options.map_entry,
        uninterpreted_option: message_options
            .uninterpreted_option
            .into_iter()
            .map(convert_uninterpreted_option)
            .collect(),
    }
}

fn convert_uninterpreted_option(
    uninterpreted_option: protobuf::descriptor::UninterpretedOption,
) -> UninterpretedOption {
    UninterpretedOption {
        name: uninterpreted_option
            .name
            .into_iter()
            .map(convert_name_part)
            .collect(),
        identifier_value: uninterpreted_option.identifier_value,
        positive_int_value: uninterpreted_option.positive_int_value,
        negative_int_value: uninterpreted_option.negative_int_value,
        double_value: uninterpreted_option.double_value,
        string_value: uninterpreted_option.string_value,
        aggregate_value: uninterpreted_option.aggregate_value,
    }
}

fn convert_name_part(name_part: protobuf::descriptor::uninterpreted_option::NamePart) -> NamePart {
    NamePart {
        name_part: name_part.name_part.unwrap_or_default(),
        is_extension: name_part.is_extension.unwrap_or_default(),
    }
}

fn convert_file_options(file_options: Box<protobuf::descriptor::FileOptions>) -> FileOptions {
    FileOptions {
        java_package: file_options.java_package,
        java_outer_classname: file_options.java_outer_classname,
        java_multiple_files: file_options.java_multiple_files,
        java_generate_equals_and_hash: file_options.java_generate_equals_and_hash,
        java_string_check_utf8: file_options.java_string_check_utf8,
        optimize_for: file_options.optimize_for.map(|el| el.value()),
        go_package: file_options.go_package,
        cc_generic_services: file_options.cc_generic_services,
        java_generic_services: file_options.java_generic_services,
        py_generic_services: file_options.py_generic_services,
        php_generic_services: file_options.php_generic_services,
        deprecated: file_options.deprecated,
        cc_enable_arenas: file_options.cc_enable_arenas,
        objc_class_prefix: file_options.objc_class_prefix,
        csharp_namespace: file_options.csharp_namespace,
        swift_prefix: file_options.swift_prefix,
        php_class_prefix: file_options.php_class_prefix,
        php_namespace: file_options.php_namespace,
        php_metadata_namespace: file_options.php_metadata_namespace,
        ruby_package: file_options.ruby_package,
        uninterpreted_option: file_options
            .uninterpreted_option
            .into_iter()
            .map(convert_uninterpreted_option)
            .collect(),
    }
}

fn convert_source_code_info(
    source_code_info: Box<protobuf::descriptor::SourceCodeInfo>,
) -> SourceCodeInfo {
    SourceCodeInfo {
        location: source_code_info
            .location
            .into_iter()
            .map(convert_location)
            .collect(),
    }
}

fn convert_location(location: protobuf::descriptor::source_code_info::Location) -> Location {
    Location {
        path: location.path,
        span: location.span,
        leading_comments: location.leading_comments,
        trailing_comments: location.trailing_comments,
        leading_detached_comments: location.leading_detached_comments,
    }
}

fn convert_field_options(field_options: Box<protobuf::descriptor::FieldOptions>) -> FieldOptions {
    FieldOptions {
        ctype: field_options.ctype.map(|el| el.value()),
        packed: field_options.packed,
        jstype: field_options.jstype.map(|el| el.value()),
        lazy: field_options.lazy,
        deprecated: field_options.deprecated,
        weak: field_options.weak,
        uninterpreted_option: field_options
            .uninterpreted_option
            .into_iter()
            .map(convert_uninterpreted_option)
            .collect(),
    }
}

fn convert_service_options(
    service_options: Box<protobuf::descriptor::ServiceOptions>,
) -> ServiceOptions {
    ServiceOptions {
        deprecated: service_options.deprecated,
        uninterpreted_option: service_options
            .uninterpreted_option
            .into_iter()
            .map(convert_uninterpreted_option)
            .collect(),
    }
}

fn convert_method_descriptor_proto(
    method_descriptor_proto: protobuf::descriptor::MethodDescriptorProto,
) -> MethodDescriptorProto {
    MethodDescriptorProto {
        name: method_descriptor_proto.name,
        input_type: method_descriptor_proto.input_type,
        output_type: method_descriptor_proto.output_type,
        options: method_descriptor_proto
            .options
            .0
            .map(convert_method_options),
        client_streaming: method_descriptor_proto.client_streaming,
        server_streaming: method_descriptor_proto.server_streaming,
    }
}

fn convert_method_options(
    method_options: Box<protobuf::descriptor::MethodOptions>,
) -> MethodOptions {
    MethodOptions {
        deprecated: method_options.deprecated,
        idempotency_level: method_options.idempotency_level.map(|el| el.value()),
        uninterpreted_option: method_options
            .uninterpreted_option
            .into_iter()
            .map(convert_uninterpreted_option)
            .collect(),
    }
}
