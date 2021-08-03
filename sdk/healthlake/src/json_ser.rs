// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_create_fhir_datastore_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateFhirDatastoreInput,
) {
    if let Some(var_1) = &input.datastore_name {
        object.key("DatastoreName").string(var_1);
    }
    if let Some(var_2) = &input.datastore_type_version {
        object.key("DatastoreTypeVersion").string(var_2.as_str());
    }
    if let Some(var_3) = &input.preload_data_config {
        let mut object_4 = object.key("PreloadDataConfig").start_object();
        crate::json_ser::serialize_structure_preload_data_config(&mut object_4, var_3);
        object_4.finish();
    }
    if let Some(var_5) = &input.client_token {
        object.key("ClientToken").string(var_5);
    }
}

pub fn serialize_structure_delete_fhir_datastore_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteFhirDatastoreInput,
) {
    if let Some(var_6) = &input.datastore_id {
        object.key("DatastoreId").string(var_6);
    }
}

pub fn serialize_structure_describe_fhir_datastore_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeFhirDatastoreInput,
) {
    if let Some(var_7) = &input.datastore_id {
        object.key("DatastoreId").string(var_7);
    }
}

pub fn serialize_structure_describe_fhir_export_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeFhirExportJobInput,
) {
    if let Some(var_8) = &input.datastore_id {
        object.key("DatastoreId").string(var_8);
    }
    if let Some(var_9) = &input.job_id {
        object.key("JobId").string(var_9);
    }
}

pub fn serialize_structure_describe_fhir_import_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeFhirImportJobInput,
) {
    if let Some(var_10) = &input.datastore_id {
        object.key("DatastoreId").string(var_10);
    }
    if let Some(var_11) = &input.job_id {
        object.key("JobId").string(var_11);
    }
}

pub fn serialize_structure_list_fhir_datastores_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListFhirDatastoresInput,
) {
    if let Some(var_12) = &input.filter {
        let mut object_13 = object.key("Filter").start_object();
        crate::json_ser::serialize_structure_datastore_filter(&mut object_13, var_12);
        object_13.finish();
    }
    if let Some(var_14) = &input.next_token {
        object.key("NextToken").string(var_14);
    }
    if let Some(var_15) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_15).into()),
        );
    }
}

pub fn serialize_structure_start_fhir_export_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartFhirExportJobInput,
) {
    if let Some(var_16) = &input.job_name {
        object.key("JobName").string(var_16);
    }
    if let Some(var_17) = &input.output_data_config {
        let mut object_18 = object.key("OutputDataConfig").start_object();
        crate::json_ser::serialize_union_output_data_config(&mut object_18, var_17);
        object_18.finish();
    }
    if let Some(var_19) = &input.datastore_id {
        object.key("DatastoreId").string(var_19);
    }
    if let Some(var_20) = &input.data_access_role_arn {
        object.key("DataAccessRoleArn").string(var_20);
    }
    if let Some(var_21) = &input.client_token {
        object.key("ClientToken").string(var_21);
    }
}

pub fn serialize_structure_start_fhir_import_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartFhirImportJobInput,
) {
    if let Some(var_22) = &input.job_name {
        object.key("JobName").string(var_22);
    }
    if let Some(var_23) = &input.input_data_config {
        let mut object_24 = object.key("InputDataConfig").start_object();
        crate::json_ser::serialize_union_input_data_config(&mut object_24, var_23);
        object_24.finish();
    }
    if let Some(var_25) = &input.datastore_id {
        object.key("DatastoreId").string(var_25);
    }
    if let Some(var_26) = &input.data_access_role_arn {
        object.key("DataAccessRoleArn").string(var_26);
    }
    if let Some(var_27) = &input.client_token {
        object.key("ClientToken").string(var_27);
    }
}

pub fn serialize_structure_preload_data_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PreloadDataConfig,
) {
    if let Some(var_28) = &input.preload_data_type {
        object.key("PreloadDataType").string(var_28.as_str());
    }
}

pub fn serialize_structure_datastore_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DatastoreFilter,
) {
    if let Some(var_29) = &input.datastore_name {
        object.key("DatastoreName").string(var_29);
    }
    if let Some(var_30) = &input.datastore_status {
        object.key("DatastoreStatus").string(var_30.as_str());
    }
    if let Some(var_31) = &input.created_before {
        object
            .key("CreatedBefore")
            .instant(var_31, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_32) = &input.created_after {
        object
            .key("CreatedAfter")
            .instant(var_32, smithy_types::instant::Format::EpochSeconds);
    }
}

pub fn serialize_union_output_data_config(
    object_18: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OutputDataConfig,
) {
    match input {
        crate::model::OutputDataConfig::S3Uri(inner) => {
            object_18.key("S3Uri").string(inner);
        }
    }
}

pub fn serialize_union_input_data_config(
    object_24: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InputDataConfig,
) {
    match input {
        crate::model::InputDataConfig::S3Uri(inner) => {
            object_24.key("S3Uri").string(inner);
        }
    }
}