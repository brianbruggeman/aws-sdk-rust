// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_add_application_cloud_watch_logging_option_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddApplicationCloudWatchLoggingOptionInput,
) {
    if let Some(var_1) = &input.application_name {
        object.key("ApplicationName").string(var_1);
    }
    if let Some(var_2) = &input.current_application_version_id {
        object.key("CurrentApplicationVersionId").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.cloud_watch_logging_option {
        let mut object_4 = object.key("CloudWatchLoggingOption").start_object();
        crate::json_ser::serialize_structure_cloud_watch_logging_option(&mut object_4, var_3);
        object_4.finish();
    }
}

pub fn serialize_structure_add_application_input_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddApplicationInputInput,
) {
    if let Some(var_5) = &input.application_name {
        object.key("ApplicationName").string(var_5);
    }
    if let Some(var_6) = &input.current_application_version_id {
        object.key("CurrentApplicationVersionId").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.input {
        let mut object_8 = object.key("Input").start_object();
        crate::json_ser::serialize_structure_input(&mut object_8, var_7);
        object_8.finish();
    }
}

pub fn serialize_structure_add_application_input_processing_configuration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddApplicationInputProcessingConfigurationInput,
) {
    if let Some(var_9) = &input.application_name {
        object.key("ApplicationName").string(var_9);
    }
    if let Some(var_10) = &input.current_application_version_id {
        object.key("CurrentApplicationVersionId").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_10).into()),
        );
    }
    if let Some(var_11) = &input.input_id {
        object.key("InputId").string(var_11);
    }
    if let Some(var_12) = &input.input_processing_configuration {
        let mut object_13 = object.key("InputProcessingConfiguration").start_object();
        crate::json_ser::serialize_structure_input_processing_configuration(&mut object_13, var_12);
        object_13.finish();
    }
}

pub fn serialize_structure_add_application_output_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddApplicationOutputInput,
) {
    if let Some(var_14) = &input.application_name {
        object.key("ApplicationName").string(var_14);
    }
    if let Some(var_15) = &input.current_application_version_id {
        object.key("CurrentApplicationVersionId").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_15).into()),
        );
    }
    if let Some(var_16) = &input.output {
        let mut object_17 = object.key("Output").start_object();
        crate::json_ser::serialize_structure_output(&mut object_17, var_16);
        object_17.finish();
    }
}

pub fn serialize_structure_add_application_reference_data_source_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddApplicationReferenceDataSourceInput,
) {
    if let Some(var_18) = &input.application_name {
        object.key("ApplicationName").string(var_18);
    }
    if let Some(var_19) = &input.current_application_version_id {
        object.key("CurrentApplicationVersionId").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_19).into()),
        );
    }
    if let Some(var_20) = &input.reference_data_source {
        let mut object_21 = object.key("ReferenceDataSource").start_object();
        crate::json_ser::serialize_structure_reference_data_source(&mut object_21, var_20);
        object_21.finish();
    }
}

pub fn serialize_structure_create_application_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateApplicationInput,
) {
    if let Some(var_22) = &input.application_name {
        object.key("ApplicationName").string(var_22);
    }
    if let Some(var_23) = &input.application_description {
        object.key("ApplicationDescription").string(var_23);
    }
    if let Some(var_24) = &input.inputs {
        let mut array_25 = object.key("Inputs").start_array();
        for item_26 in var_24 {
            {
                let mut object_27 = array_25.value().start_object();
                crate::json_ser::serialize_structure_input(&mut object_27, item_26);
                object_27.finish();
            }
        }
        array_25.finish();
    }
    if let Some(var_28) = &input.outputs {
        let mut array_29 = object.key("Outputs").start_array();
        for item_30 in var_28 {
            {
                let mut object_31 = array_29.value().start_object();
                crate::json_ser::serialize_structure_output(&mut object_31, item_30);
                object_31.finish();
            }
        }
        array_29.finish();
    }
    if let Some(var_32) = &input.cloud_watch_logging_options {
        let mut array_33 = object.key("CloudWatchLoggingOptions").start_array();
        for item_34 in var_32 {
            {
                let mut object_35 = array_33.value().start_object();
                crate::json_ser::serialize_structure_cloud_watch_logging_option(
                    &mut object_35,
                    item_34,
                );
                object_35.finish();
            }
        }
        array_33.finish();
    }
    if let Some(var_36) = &input.application_code {
        object.key("ApplicationCode").string(var_36);
    }
    if let Some(var_37) = &input.tags {
        let mut array_38 = object.key("Tags").start_array();
        for item_39 in var_37 {
            {
                let mut object_40 = array_38.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_40, item_39);
                object_40.finish();
            }
        }
        array_38.finish();
    }
}

pub fn serialize_structure_delete_application_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteApplicationInput,
) {
    if let Some(var_41) = &input.application_name {
        object.key("ApplicationName").string(var_41);
    }
    if let Some(var_42) = &input.create_timestamp {
        object
            .key("CreateTimestamp")
            .instant(var_42, smithy_types::instant::Format::EpochSeconds);
    }
}

pub fn serialize_structure_delete_application_cloud_watch_logging_option_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteApplicationCloudWatchLoggingOptionInput,
) {
    if let Some(var_43) = &input.application_name {
        object.key("ApplicationName").string(var_43);
    }
    if let Some(var_44) = &input.current_application_version_id {
        object.key("CurrentApplicationVersionId").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_44).into()),
        );
    }
    if let Some(var_45) = &input.cloud_watch_logging_option_id {
        object.key("CloudWatchLoggingOptionId").string(var_45);
    }
}

pub fn serialize_structure_delete_application_input_processing_configuration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteApplicationInputProcessingConfigurationInput,
) {
    if let Some(var_46) = &input.application_name {
        object.key("ApplicationName").string(var_46);
    }
    if let Some(var_47) = &input.current_application_version_id {
        object.key("CurrentApplicationVersionId").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_47).into()),
        );
    }
    if let Some(var_48) = &input.input_id {
        object.key("InputId").string(var_48);
    }
}

pub fn serialize_structure_delete_application_reference_data_source_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteApplicationReferenceDataSourceInput,
) {
    if let Some(var_49) = &input.application_name {
        object.key("ApplicationName").string(var_49);
    }
    if let Some(var_50) = &input.current_application_version_id {
        object.key("CurrentApplicationVersionId").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_50).into()),
        );
    }
    if let Some(var_51) = &input.reference_id {
        object.key("ReferenceId").string(var_51);
    }
}

pub fn serialize_structure_describe_application_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeApplicationInput,
) {
    if let Some(var_52) = &input.application_name {
        object.key("ApplicationName").string(var_52);
    }
}

pub fn serialize_structure_discover_input_schema_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DiscoverInputSchemaInput,
) {
    if let Some(var_53) = &input.resource_arn {
        object.key("ResourceARN").string(var_53);
    }
    if let Some(var_54) = &input.role_arn {
        object.key("RoleARN").string(var_54);
    }
    if let Some(var_55) = &input.input_starting_position_configuration {
        let mut object_56 = object
            .key("InputStartingPositionConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_input_starting_position_configuration(
            &mut object_56,
            var_55,
        );
        object_56.finish();
    }
    if let Some(var_57) = &input.s3_configuration {
        let mut object_58 = object.key("S3Configuration").start_object();
        crate::json_ser::serialize_structure_s3_configuration(&mut object_58, var_57);
        object_58.finish();
    }
    if let Some(var_59) = &input.input_processing_configuration {
        let mut object_60 = object.key("InputProcessingConfiguration").start_object();
        crate::json_ser::serialize_structure_input_processing_configuration(&mut object_60, var_59);
        object_60.finish();
    }
}

pub fn serialize_structure_list_applications_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListApplicationsInput,
) {
    if let Some(var_61) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_61).into()),
        );
    }
    if let Some(var_62) = &input.exclusive_start_application_name {
        object.key("ExclusiveStartApplicationName").string(var_62);
    }
}

pub fn serialize_structure_list_tags_for_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) {
    if let Some(var_63) = &input.resource_arn {
        object.key("ResourceARN").string(var_63);
    }
}

pub fn serialize_structure_start_application_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartApplicationInput,
) {
    if let Some(var_64) = &input.application_name {
        object.key("ApplicationName").string(var_64);
    }
    if let Some(var_65) = &input.input_configurations {
        let mut array_66 = object.key("InputConfigurations").start_array();
        for item_67 in var_65 {
            {
                let mut object_68 = array_66.value().start_object();
                crate::json_ser::serialize_structure_input_configuration(&mut object_68, item_67);
                object_68.finish();
            }
        }
        array_66.finish();
    }
}

pub fn serialize_structure_stop_application_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopApplicationInput,
) {
    if let Some(var_69) = &input.application_name {
        object.key("ApplicationName").string(var_69);
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_70) = &input.resource_arn {
        object.key("ResourceARN").string(var_70);
    }
    if let Some(var_71) = &input.tags {
        let mut array_72 = object.key("Tags").start_array();
        for item_73 in var_71 {
            {
                let mut object_74 = array_72.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_74, item_73);
                object_74.finish();
            }
        }
        array_72.finish();
    }
}

pub fn serialize_structure_untag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) {
    if let Some(var_75) = &input.resource_arn {
        object.key("ResourceARN").string(var_75);
    }
    if let Some(var_76) = &input.tag_keys {
        let mut array_77 = object.key("TagKeys").start_array();
        for item_78 in var_76 {
            {
                array_77.value().string(item_78);
            }
        }
        array_77.finish();
    }
}

pub fn serialize_structure_update_application_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateApplicationInput,
) {
    if let Some(var_79) = &input.application_name {
        object.key("ApplicationName").string(var_79);
    }
    if let Some(var_80) = &input.current_application_version_id {
        object.key("CurrentApplicationVersionId").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_80).into()),
        );
    }
    if let Some(var_81) = &input.application_update {
        let mut object_82 = object.key("ApplicationUpdate").start_object();
        crate::json_ser::serialize_structure_application_update(&mut object_82, var_81);
        object_82.finish();
    }
}

pub fn serialize_structure_cloud_watch_logging_option(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CloudWatchLoggingOption,
) {
    if let Some(var_83) = &input.log_stream_arn {
        object.key("LogStreamARN").string(var_83);
    }
    if let Some(var_84) = &input.role_arn {
        object.key("RoleARN").string(var_84);
    }
}

pub fn serialize_structure_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Input,
) {
    if let Some(var_85) = &input.name_prefix {
        object.key("NamePrefix").string(var_85);
    }
    if let Some(var_86) = &input.input_processing_configuration {
        let mut object_87 = object.key("InputProcessingConfiguration").start_object();
        crate::json_ser::serialize_structure_input_processing_configuration(&mut object_87, var_86);
        object_87.finish();
    }
    if let Some(var_88) = &input.kinesis_streams_input {
        let mut object_89 = object.key("KinesisStreamsInput").start_object();
        crate::json_ser::serialize_structure_kinesis_streams_input(&mut object_89, var_88);
        object_89.finish();
    }
    if let Some(var_90) = &input.kinesis_firehose_input {
        let mut object_91 = object.key("KinesisFirehoseInput").start_object();
        crate::json_ser::serialize_structure_kinesis_firehose_input(&mut object_91, var_90);
        object_91.finish();
    }
    if let Some(var_92) = &input.input_parallelism {
        let mut object_93 = object.key("InputParallelism").start_object();
        crate::json_ser::serialize_structure_input_parallelism(&mut object_93, var_92);
        object_93.finish();
    }
    if let Some(var_94) = &input.input_schema {
        let mut object_95 = object.key("InputSchema").start_object();
        crate::json_ser::serialize_structure_source_schema(&mut object_95, var_94);
        object_95.finish();
    }
}

pub fn serialize_structure_input_processing_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InputProcessingConfiguration,
) {
    if let Some(var_96) = &input.input_lambda_processor {
        let mut object_97 = object.key("InputLambdaProcessor").start_object();
        crate::json_ser::serialize_structure_input_lambda_processor(&mut object_97, var_96);
        object_97.finish();
    }
}

pub fn serialize_structure_output(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Output,
) {
    if let Some(var_98) = &input.name {
        object.key("Name").string(var_98);
    }
    if let Some(var_99) = &input.kinesis_streams_output {
        let mut object_100 = object.key("KinesisStreamsOutput").start_object();
        crate::json_ser::serialize_structure_kinesis_streams_output(&mut object_100, var_99);
        object_100.finish();
    }
    if let Some(var_101) = &input.kinesis_firehose_output {
        let mut object_102 = object.key("KinesisFirehoseOutput").start_object();
        crate::json_ser::serialize_structure_kinesis_firehose_output(&mut object_102, var_101);
        object_102.finish();
    }
    if let Some(var_103) = &input.lambda_output {
        let mut object_104 = object.key("LambdaOutput").start_object();
        crate::json_ser::serialize_structure_lambda_output(&mut object_104, var_103);
        object_104.finish();
    }
    if let Some(var_105) = &input.destination_schema {
        let mut object_106 = object.key("DestinationSchema").start_object();
        crate::json_ser::serialize_structure_destination_schema(&mut object_106, var_105);
        object_106.finish();
    }
}

pub fn serialize_structure_reference_data_source(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ReferenceDataSource,
) {
    if let Some(var_107) = &input.table_name {
        object.key("TableName").string(var_107);
    }
    if let Some(var_108) = &input.s3_reference_data_source {
        let mut object_109 = object.key("S3ReferenceDataSource").start_object();
        crate::json_ser::serialize_structure_s3_reference_data_source(&mut object_109, var_108);
        object_109.finish();
    }
    if let Some(var_110) = &input.reference_schema {
        let mut object_111 = object.key("ReferenceSchema").start_object();
        crate::json_ser::serialize_structure_source_schema(&mut object_111, var_110);
        object_111.finish();
    }
}

pub fn serialize_structure_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_112) = &input.key {
        object.key("Key").string(var_112);
    }
    if let Some(var_113) = &input.value {
        object.key("Value").string(var_113);
    }
}

pub fn serialize_structure_input_starting_position_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InputStartingPositionConfiguration,
) {
    if let Some(var_114) = &input.input_starting_position {
        object.key("InputStartingPosition").string(var_114.as_str());
    }
}

pub fn serialize_structure_s3_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3Configuration,
) {
    if let Some(var_115) = &input.role_arn {
        object.key("RoleARN").string(var_115);
    }
    if let Some(var_116) = &input.bucket_arn {
        object.key("BucketARN").string(var_116);
    }
    if let Some(var_117) = &input.file_key {
        object.key("FileKey").string(var_117);
    }
}

pub fn serialize_structure_input_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InputConfiguration,
) {
    if let Some(var_118) = &input.id {
        object.key("Id").string(var_118);
    }
    if let Some(var_119) = &input.input_starting_position_configuration {
        let mut object_120 = object
            .key("InputStartingPositionConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_input_starting_position_configuration(
            &mut object_120,
            var_119,
        );
        object_120.finish();
    }
}

pub fn serialize_structure_application_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ApplicationUpdate,
) {
    if let Some(var_121) = &input.input_updates {
        let mut array_122 = object.key("InputUpdates").start_array();
        for item_123 in var_121 {
            {
                let mut object_124 = array_122.value().start_object();
                crate::json_ser::serialize_structure_input_update(&mut object_124, item_123);
                object_124.finish();
            }
        }
        array_122.finish();
    }
    if let Some(var_125) = &input.application_code_update {
        object.key("ApplicationCodeUpdate").string(var_125);
    }
    if let Some(var_126) = &input.output_updates {
        let mut array_127 = object.key("OutputUpdates").start_array();
        for item_128 in var_126 {
            {
                let mut object_129 = array_127.value().start_object();
                crate::json_ser::serialize_structure_output_update(&mut object_129, item_128);
                object_129.finish();
            }
        }
        array_127.finish();
    }
    if let Some(var_130) = &input.reference_data_source_updates {
        let mut array_131 = object.key("ReferenceDataSourceUpdates").start_array();
        for item_132 in var_130 {
            {
                let mut object_133 = array_131.value().start_object();
                crate::json_ser::serialize_structure_reference_data_source_update(
                    &mut object_133,
                    item_132,
                );
                object_133.finish();
            }
        }
        array_131.finish();
    }
    if let Some(var_134) = &input.cloud_watch_logging_option_updates {
        let mut array_135 = object.key("CloudWatchLoggingOptionUpdates").start_array();
        for item_136 in var_134 {
            {
                let mut object_137 = array_135.value().start_object();
                crate::json_ser::serialize_structure_cloud_watch_logging_option_update(
                    &mut object_137,
                    item_136,
                );
                object_137.finish();
            }
        }
        array_135.finish();
    }
}

pub fn serialize_structure_kinesis_streams_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KinesisStreamsInput,
) {
    if let Some(var_138) = &input.resource_arn {
        object.key("ResourceARN").string(var_138);
    }
    if let Some(var_139) = &input.role_arn {
        object.key("RoleARN").string(var_139);
    }
}

pub fn serialize_structure_kinesis_firehose_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KinesisFirehoseInput,
) {
    if let Some(var_140) = &input.resource_arn {
        object.key("ResourceARN").string(var_140);
    }
    if let Some(var_141) = &input.role_arn {
        object.key("RoleARN").string(var_141);
    }
}

pub fn serialize_structure_input_parallelism(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InputParallelism,
) {
    if let Some(var_142) = &input.count {
        object.key("Count").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_142).into()),
        );
    }
}

pub fn serialize_structure_source_schema(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SourceSchema,
) {
    if let Some(var_143) = &input.record_format {
        let mut object_144 = object.key("RecordFormat").start_object();
        crate::json_ser::serialize_structure_record_format(&mut object_144, var_143);
        object_144.finish();
    }
    if let Some(var_145) = &input.record_encoding {
        object.key("RecordEncoding").string(var_145);
    }
    if let Some(var_146) = &input.record_columns {
        let mut array_147 = object.key("RecordColumns").start_array();
        for item_148 in var_146 {
            {
                let mut object_149 = array_147.value().start_object();
                crate::json_ser::serialize_structure_record_column(&mut object_149, item_148);
                object_149.finish();
            }
        }
        array_147.finish();
    }
}

pub fn serialize_structure_input_lambda_processor(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InputLambdaProcessor,
) {
    if let Some(var_150) = &input.resource_arn {
        object.key("ResourceARN").string(var_150);
    }
    if let Some(var_151) = &input.role_arn {
        object.key("RoleARN").string(var_151);
    }
}

pub fn serialize_structure_kinesis_streams_output(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KinesisStreamsOutput,
) {
    if let Some(var_152) = &input.resource_arn {
        object.key("ResourceARN").string(var_152);
    }
    if let Some(var_153) = &input.role_arn {
        object.key("RoleARN").string(var_153);
    }
}

pub fn serialize_structure_kinesis_firehose_output(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KinesisFirehoseOutput,
) {
    if let Some(var_154) = &input.resource_arn {
        object.key("ResourceARN").string(var_154);
    }
    if let Some(var_155) = &input.role_arn {
        object.key("RoleARN").string(var_155);
    }
}

pub fn serialize_structure_lambda_output(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaOutput,
) {
    if let Some(var_156) = &input.resource_arn {
        object.key("ResourceARN").string(var_156);
    }
    if let Some(var_157) = &input.role_arn {
        object.key("RoleARN").string(var_157);
    }
}

pub fn serialize_structure_destination_schema(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DestinationSchema,
) {
    if let Some(var_158) = &input.record_format_type {
        object.key("RecordFormatType").string(var_158.as_str());
    }
}

pub fn serialize_structure_s3_reference_data_source(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3ReferenceDataSource,
) {
    if let Some(var_159) = &input.bucket_arn {
        object.key("BucketARN").string(var_159);
    }
    if let Some(var_160) = &input.file_key {
        object.key("FileKey").string(var_160);
    }
    if let Some(var_161) = &input.reference_role_arn {
        object.key("ReferenceRoleARN").string(var_161);
    }
}

pub fn serialize_structure_input_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InputUpdate,
) {
    if let Some(var_162) = &input.input_id {
        object.key("InputId").string(var_162);
    }
    if let Some(var_163) = &input.name_prefix_update {
        object.key("NamePrefixUpdate").string(var_163);
    }
    if let Some(var_164) = &input.input_processing_configuration_update {
        let mut object_165 = object
            .key("InputProcessingConfigurationUpdate")
            .start_object();
        crate::json_ser::serialize_structure_input_processing_configuration_update(
            &mut object_165,
            var_164,
        );
        object_165.finish();
    }
    if let Some(var_166) = &input.kinesis_streams_input_update {
        let mut object_167 = object.key("KinesisStreamsInputUpdate").start_object();
        crate::json_ser::serialize_structure_kinesis_streams_input_update(&mut object_167, var_166);
        object_167.finish();
    }
    if let Some(var_168) = &input.kinesis_firehose_input_update {
        let mut object_169 = object.key("KinesisFirehoseInputUpdate").start_object();
        crate::json_ser::serialize_structure_kinesis_firehose_input_update(
            &mut object_169,
            var_168,
        );
        object_169.finish();
    }
    if let Some(var_170) = &input.input_schema_update {
        let mut object_171 = object.key("InputSchemaUpdate").start_object();
        crate::json_ser::serialize_structure_input_schema_update(&mut object_171, var_170);
        object_171.finish();
    }
    if let Some(var_172) = &input.input_parallelism_update {
        let mut object_173 = object.key("InputParallelismUpdate").start_object();
        crate::json_ser::serialize_structure_input_parallelism_update(&mut object_173, var_172);
        object_173.finish();
    }
}

pub fn serialize_structure_output_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OutputUpdate,
) {
    if let Some(var_174) = &input.output_id {
        object.key("OutputId").string(var_174);
    }
    if let Some(var_175) = &input.name_update {
        object.key("NameUpdate").string(var_175);
    }
    if let Some(var_176) = &input.kinesis_streams_output_update {
        let mut object_177 = object.key("KinesisStreamsOutputUpdate").start_object();
        crate::json_ser::serialize_structure_kinesis_streams_output_update(
            &mut object_177,
            var_176,
        );
        object_177.finish();
    }
    if let Some(var_178) = &input.kinesis_firehose_output_update {
        let mut object_179 = object.key("KinesisFirehoseOutputUpdate").start_object();
        crate::json_ser::serialize_structure_kinesis_firehose_output_update(
            &mut object_179,
            var_178,
        );
        object_179.finish();
    }
    if let Some(var_180) = &input.lambda_output_update {
        let mut object_181 = object.key("LambdaOutputUpdate").start_object();
        crate::json_ser::serialize_structure_lambda_output_update(&mut object_181, var_180);
        object_181.finish();
    }
    if let Some(var_182) = &input.destination_schema_update {
        let mut object_183 = object.key("DestinationSchemaUpdate").start_object();
        crate::json_ser::serialize_structure_destination_schema(&mut object_183, var_182);
        object_183.finish();
    }
}

pub fn serialize_structure_reference_data_source_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ReferenceDataSourceUpdate,
) {
    if let Some(var_184) = &input.reference_id {
        object.key("ReferenceId").string(var_184);
    }
    if let Some(var_185) = &input.table_name_update {
        object.key("TableNameUpdate").string(var_185);
    }
    if let Some(var_186) = &input.s3_reference_data_source_update {
        let mut object_187 = object.key("S3ReferenceDataSourceUpdate").start_object();
        crate::json_ser::serialize_structure_s3_reference_data_source_update(
            &mut object_187,
            var_186,
        );
        object_187.finish();
    }
    if let Some(var_188) = &input.reference_schema_update {
        let mut object_189 = object.key("ReferenceSchemaUpdate").start_object();
        crate::json_ser::serialize_structure_source_schema(&mut object_189, var_188);
        object_189.finish();
    }
}

pub fn serialize_structure_cloud_watch_logging_option_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CloudWatchLoggingOptionUpdate,
) {
    if let Some(var_190) = &input.cloud_watch_logging_option_id {
        object.key("CloudWatchLoggingOptionId").string(var_190);
    }
    if let Some(var_191) = &input.log_stream_arn_update {
        object.key("LogStreamARNUpdate").string(var_191);
    }
    if let Some(var_192) = &input.role_arn_update {
        object.key("RoleARNUpdate").string(var_192);
    }
}

pub fn serialize_structure_record_format(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RecordFormat,
) {
    if let Some(var_193) = &input.record_format_type {
        object.key("RecordFormatType").string(var_193.as_str());
    }
    if let Some(var_194) = &input.mapping_parameters {
        let mut object_195 = object.key("MappingParameters").start_object();
        crate::json_ser::serialize_structure_mapping_parameters(&mut object_195, var_194);
        object_195.finish();
    }
}

pub fn serialize_structure_record_column(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RecordColumn,
) {
    if let Some(var_196) = &input.name {
        object.key("Name").string(var_196);
    }
    if let Some(var_197) = &input.mapping {
        object.key("Mapping").string(var_197);
    }
    if let Some(var_198) = &input.sql_type {
        object.key("SqlType").string(var_198);
    }
}

pub fn serialize_structure_input_processing_configuration_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InputProcessingConfigurationUpdate,
) {
    if let Some(var_199) = &input.input_lambda_processor_update {
        let mut object_200 = object.key("InputLambdaProcessorUpdate").start_object();
        crate::json_ser::serialize_structure_input_lambda_processor_update(
            &mut object_200,
            var_199,
        );
        object_200.finish();
    }
}

pub fn serialize_structure_kinesis_streams_input_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KinesisStreamsInputUpdate,
) {
    if let Some(var_201) = &input.resource_arn_update {
        object.key("ResourceARNUpdate").string(var_201);
    }
    if let Some(var_202) = &input.role_arn_update {
        object.key("RoleARNUpdate").string(var_202);
    }
}

pub fn serialize_structure_kinesis_firehose_input_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KinesisFirehoseInputUpdate,
) {
    if let Some(var_203) = &input.resource_arn_update {
        object.key("ResourceARNUpdate").string(var_203);
    }
    if let Some(var_204) = &input.role_arn_update {
        object.key("RoleARNUpdate").string(var_204);
    }
}

pub fn serialize_structure_input_schema_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InputSchemaUpdate,
) {
    if let Some(var_205) = &input.record_format_update {
        let mut object_206 = object.key("RecordFormatUpdate").start_object();
        crate::json_ser::serialize_structure_record_format(&mut object_206, var_205);
        object_206.finish();
    }
    if let Some(var_207) = &input.record_encoding_update {
        object.key("RecordEncodingUpdate").string(var_207);
    }
    if let Some(var_208) = &input.record_column_updates {
        let mut array_209 = object.key("RecordColumnUpdates").start_array();
        for item_210 in var_208 {
            {
                let mut object_211 = array_209.value().start_object();
                crate::json_ser::serialize_structure_record_column(&mut object_211, item_210);
                object_211.finish();
            }
        }
        array_209.finish();
    }
}

pub fn serialize_structure_input_parallelism_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InputParallelismUpdate,
) {
    if let Some(var_212) = &input.count_update {
        object.key("CountUpdate").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_212).into()),
        );
    }
}

pub fn serialize_structure_kinesis_streams_output_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KinesisStreamsOutputUpdate,
) {
    if let Some(var_213) = &input.resource_arn_update {
        object.key("ResourceARNUpdate").string(var_213);
    }
    if let Some(var_214) = &input.role_arn_update {
        object.key("RoleARNUpdate").string(var_214);
    }
}

pub fn serialize_structure_kinesis_firehose_output_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KinesisFirehoseOutputUpdate,
) {
    if let Some(var_215) = &input.resource_arn_update {
        object.key("ResourceARNUpdate").string(var_215);
    }
    if let Some(var_216) = &input.role_arn_update {
        object.key("RoleARNUpdate").string(var_216);
    }
}

pub fn serialize_structure_lambda_output_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaOutputUpdate,
) {
    if let Some(var_217) = &input.resource_arn_update {
        object.key("ResourceARNUpdate").string(var_217);
    }
    if let Some(var_218) = &input.role_arn_update {
        object.key("RoleARNUpdate").string(var_218);
    }
}

pub fn serialize_structure_s3_reference_data_source_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3ReferenceDataSourceUpdate,
) {
    if let Some(var_219) = &input.bucket_arn_update {
        object.key("BucketARNUpdate").string(var_219);
    }
    if let Some(var_220) = &input.file_key_update {
        object.key("FileKeyUpdate").string(var_220);
    }
    if let Some(var_221) = &input.reference_role_arn_update {
        object.key("ReferenceRoleARNUpdate").string(var_221);
    }
}

pub fn serialize_structure_mapping_parameters(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MappingParameters,
) {
    if let Some(var_222) = &input.json_mapping_parameters {
        let mut object_223 = object.key("JSONMappingParameters").start_object();
        crate::json_ser::serialize_structure_json_mapping_parameters(&mut object_223, var_222);
        object_223.finish();
    }
    if let Some(var_224) = &input.csv_mapping_parameters {
        let mut object_225 = object.key("CSVMappingParameters").start_object();
        crate::json_ser::serialize_structure_csv_mapping_parameters(&mut object_225, var_224);
        object_225.finish();
    }
}

pub fn serialize_structure_input_lambda_processor_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InputLambdaProcessorUpdate,
) {
    if let Some(var_226) = &input.resource_arn_update {
        object.key("ResourceARNUpdate").string(var_226);
    }
    if let Some(var_227) = &input.role_arn_update {
        object.key("RoleARNUpdate").string(var_227);
    }
}

pub fn serialize_structure_json_mapping_parameters(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::JsonMappingParameters,
) {
    if let Some(var_228) = &input.record_row_path {
        object.key("RecordRowPath").string(var_228);
    }
}

pub fn serialize_structure_csv_mapping_parameters(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CsvMappingParameters,
) {
    if let Some(var_229) = &input.record_row_delimiter {
        object.key("RecordRowDelimiter").string(var_229);
    }
    if let Some(var_230) = &input.record_column_delimiter {
        object.key("RecordColumnDelimiter").string(var_230);
    }
}