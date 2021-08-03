// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <note>
/// <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p>
/// </note>
/// <p>Adds a CloudWatch log stream to monitor application configuration errors. For more
/// information about using CloudWatch log streams with Amazon Kinesis Analytics
/// applications, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html">Working with Amazon
/// CloudWatch Logs</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AddApplicationCloudWatchLoggingOption {
    _private: (),
}
impl AddApplicationCloudWatchLoggingOption {
    /// Creates a new builder-style object to manufacture [`AddApplicationCloudWatchLoggingOptionInput`](crate::input::AddApplicationCloudWatchLoggingOptionInput)
    pub fn builder() -> crate::input::add_application_cloud_watch_logging_option_input::Builder {
        crate::input::add_application_cloud_watch_logging_option_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for AddApplicationCloudWatchLoggingOption {
    type Output = std::result::Result<
        crate::output::AddApplicationCloudWatchLoggingOptionOutput,
        crate::error::AddApplicationCloudWatchLoggingOptionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_add_application_cloud_watch_logging_option_error(response)
        } else {
            crate::operation_deser::parse_add_application_cloud_watch_logging_option_response(
                response,
            )
        }
    }
}

/// <note>
/// <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p>
/// </note>
/// <p>
/// Adds a streaming source to your Amazon Kinesis application.
/// For conceptual information,
/// see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>.
/// </p>
/// <p>You can add a streaming source either when you create an application or you can use
/// this operation to add a streaming source after you create an application. For more information, see
/// <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_CreateApplication.html">CreateApplication</a>.</p>
/// <p>Any configuration update, including adding a streaming source using this operation,  
/// results in a new version of the application. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation  
/// to find the current application version.
/// </p>
/// <p>This operation requires permissions to perform the
/// <code>kinesisanalytics:AddApplicationInput</code> action.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AddApplicationInput {
    _private: (),
}
impl AddApplicationInput {
    /// Creates a new builder-style object to manufacture [`AddApplicationInputInput`](crate::input::AddApplicationInputInput)
    pub fn builder() -> crate::input::add_application_input_input::Builder {
        crate::input::add_application_input_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for AddApplicationInput {
    type Output = std::result::Result<
        crate::output::AddApplicationInputOutput,
        crate::error::AddApplicationInputError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_add_application_input_error(response)
        } else {
            crate::operation_deser::parse_add_application_input_response(response)
        }
    }
}

/// <note>
/// <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p>
/// </note>
/// <p>Adds an <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_InputProcessingConfiguration.html">InputProcessingConfiguration</a> to an application. An input processor preprocesses records on the input stream
/// before the application's SQL code executes. Currently, the only input processor available is
/// <a href="https://docs.aws.amazon.com/lambda/">AWS Lambda</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AddApplicationInputProcessingConfiguration {
    _private: (),
}
impl AddApplicationInputProcessingConfiguration {
    /// Creates a new builder-style object to manufacture [`AddApplicationInputProcessingConfigurationInput`](crate::input::AddApplicationInputProcessingConfigurationInput)
    pub fn builder() -> crate::input::add_application_input_processing_configuration_input::Builder
    {
        crate::input::add_application_input_processing_configuration_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for AddApplicationInputProcessingConfiguration {
    type Output = std::result::Result<
        crate::output::AddApplicationInputProcessingConfigurationOutput,
        crate::error::AddApplicationInputProcessingConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_add_application_input_processing_configuration_error(
                response,
            )
        } else {
            crate::operation_deser::parse_add_application_input_processing_configuration_response(
                response,
            )
        }
    }
}

/// <note>
/// <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p>
/// </note>
/// <p>Adds an external destination to your Amazon Kinesis Analytics application.</p>
/// <p>If you want Amazon Kinesis Analytics to deliver data from an in-application stream
/// within your application to an external destination (such as an Amazon Kinesis stream, an
/// Amazon Kinesis Firehose delivery stream, or an AWS Lambda function), you add the
/// relevant configuration to your application using this operation. You can configure one
/// or more outputs for your application. Each output configuration maps an in-application
/// stream and an external destination.</p>
/// <p> You can use one of the output configurations to deliver data from your
/// in-application error stream to an external destination so that you can analyze the
/// errors. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html">Understanding Application
/// Output (Destination)</a>. </p>
/// <p> Any configuration update, including adding a streaming source using this
/// operation, results in a new version of the application. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to find the current application
/// version.</p>
/// <p>For the limits on the number of application inputs and outputs
/// you can configure, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html">Limits</a>.</p>
/// <p>This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationOutput</code> action.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AddApplicationOutput {
    _private: (),
}
impl AddApplicationOutput {
    /// Creates a new builder-style object to manufacture [`AddApplicationOutputInput`](crate::input::AddApplicationOutputInput)
    pub fn builder() -> crate::input::add_application_output_input::Builder {
        crate::input::add_application_output_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for AddApplicationOutput {
    type Output = std::result::Result<
        crate::output::AddApplicationOutputOutput,
        crate::error::AddApplicationOutputError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_add_application_output_error(response)
        } else {
            crate::operation_deser::parse_add_application_output_response(response)
        }
    }
}

/// <note>
/// <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p>
/// </note>
/// <p>Adds a reference data source to an existing application.</p>
/// <p>Amazon Kinesis Analytics reads reference data (that is, an Amazon S3 object) and creates an in-application table within your application. In the request, you provide the source (S3 bucket name and object key name), name of the in-application table to create, and the necessary mapping information that describes how data in Amazon S3 object maps to columns in the resulting in-application table.</p>
/// <p>
/// For conceptual information,
/// see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>.
/// For the limits on data sources you can add to your application, see
/// <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html">Limits</a>.
/// </p>
/// <p>
/// This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationOutput</code> action.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AddApplicationReferenceDataSource {
    _private: (),
}
impl AddApplicationReferenceDataSource {
    /// Creates a new builder-style object to manufacture [`AddApplicationReferenceDataSourceInput`](crate::input::AddApplicationReferenceDataSourceInput)
    pub fn builder() -> crate::input::add_application_reference_data_source_input::Builder {
        crate::input::add_application_reference_data_source_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for AddApplicationReferenceDataSource {
    type Output = std::result::Result<
        crate::output::AddApplicationReferenceDataSourceOutput,
        crate::error::AddApplicationReferenceDataSourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_add_application_reference_data_source_error(response)
        } else {
            crate::operation_deser::parse_add_application_reference_data_source_response(response)
        }
    }
}

/// <note>
/// <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p>
/// </note>
/// <p>
/// Creates an Amazon Kinesis Analytics application.
/// You can configure each application with one streaming source as input,
/// application code to process the input, and up to
/// three destinations where
/// you want Amazon Kinesis Analytics to write the output data from your application.
/// For an overview, see
/// <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works.html">How it Works</a>.
/// </p>
/// <p>In the input configuration, you map the streaming source to an in-application stream, which you can think of as a constantly updating table. In the mapping, you must provide a schema for the in-application stream and map each data column in the in-application stream to a
/// data element in the streaming source.</p>
/// <p>Your application code is one or more SQL statements that read input data, transform it, and generate output. Your application code can create one or more SQL artifacts like SQL streams or pumps.</p>
/// <p>In the output configuration, you can configure the application to write data from in-application streams created in your applications to up to three destinations.</p>
/// <p>
/// To read data from your source stream or write data to destination streams, Amazon Kinesis Analytics
/// needs your permissions. You grant these permissions by creating IAM roles. This operation requires permissions to perform the
/// <code>kinesisanalytics:CreateApplication</code> action.
/// </p>
/// <p>
/// For introductory exercises to create an Amazon Kinesis Analytics application, see
/// <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/getting-started.html">Getting Started</a>.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateApplication {
    _private: (),
}
impl CreateApplication {
    /// Creates a new builder-style object to manufacture [`CreateApplicationInput`](crate::input::CreateApplicationInput)
    pub fn builder() -> crate::input::create_application_input::Builder {
        crate::input::create_application_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateApplication {
    type Output = std::result::Result<
        crate::output::CreateApplicationOutput,
        crate::error::CreateApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_application_error(response)
        } else {
            crate::operation_deser::parse_create_application_response(response)
        }
    }
}

/// <note>
/// <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p>
/// </note>
/// <p>Deletes the specified application. Amazon Kinesis Analytics halts application execution and deletes the application, including any application artifacts (such as in-application streams, reference table, and application code).</p>
/// <p>This operation requires permissions to perform the <code>kinesisanalytics:DeleteApplication</code> action.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteApplication {
    _private: (),
}
impl DeleteApplication {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationInput`](crate::input::DeleteApplicationInput)
    pub fn builder() -> crate::input::delete_application_input::Builder {
        crate::input::delete_application_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteApplication {
    type Output = std::result::Result<
        crate::output::DeleteApplicationOutput,
        crate::error::DeleteApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_application_error(response)
        } else {
            crate::operation_deser::parse_delete_application_response(response)
        }
    }
}

/// <note>
/// <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p>
/// </note>
/// <p>Deletes a CloudWatch log stream from an application. For more information about
/// using CloudWatch log streams with Amazon Kinesis Analytics applications, see
/// <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html">Working with Amazon CloudWatch Logs</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteApplicationCloudWatchLoggingOption {
    _private: (),
}
impl DeleteApplicationCloudWatchLoggingOption {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationCloudWatchLoggingOptionInput`](crate::input::DeleteApplicationCloudWatchLoggingOptionInput)
    pub fn builder() -> crate::input::delete_application_cloud_watch_logging_option_input::Builder {
        crate::input::delete_application_cloud_watch_logging_option_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteApplicationCloudWatchLoggingOption {
    type Output = std::result::Result<
        crate::output::DeleteApplicationCloudWatchLoggingOptionOutput,
        crate::error::DeleteApplicationCloudWatchLoggingOptionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_application_cloud_watch_logging_option_error(
                response,
            )
        } else {
            crate::operation_deser::parse_delete_application_cloud_watch_logging_option_response(
                response,
            )
        }
    }
}

/// <note>
/// <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p>
/// </note>
/// <p>Deletes an <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_InputProcessingConfiguration.html">InputProcessingConfiguration</a> from an input.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteApplicationInputProcessingConfiguration {
    _private: (),
}
impl DeleteApplicationInputProcessingConfiguration {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationInputProcessingConfigurationInput`](crate::input::DeleteApplicationInputProcessingConfigurationInput)
    pub fn builder(
    ) -> crate::input::delete_application_input_processing_configuration_input::Builder {
        crate::input::delete_application_input_processing_configuration_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteApplicationInputProcessingConfiguration {
    type Output = std::result::Result<
        crate::output::DeleteApplicationInputProcessingConfigurationOutput,
        crate::error::DeleteApplicationInputProcessingConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_application_input_processing_configuration_error(
                response,
            )
        } else {
            crate::operation_deser::parse_delete_application_input_processing_configuration_response(
                response,
            )
        }
    }
}

/// <note>
/// <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p>
/// </note>
/// <p>Deletes a reference data source configuration from the specified application configuration.</p>
/// <p>If the application is running, Amazon Kinesis Analytics immediately removes the in-application table
/// that you created using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_AddApplicationReferenceDataSource.html">AddApplicationReferenceDataSource</a> operation.  </p>
/// <p>This operation requires permissions to perform the <code>kinesisanalytics.DeleteApplicationReferenceDataSource</code>
/// action.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteApplicationReferenceDataSource {
    _private: (),
}
impl DeleteApplicationReferenceDataSource {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationReferenceDataSourceInput`](crate::input::DeleteApplicationReferenceDataSourceInput)
    pub fn builder() -> crate::input::delete_application_reference_data_source_input::Builder {
        crate::input::delete_application_reference_data_source_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteApplicationReferenceDataSource {
    type Output = std::result::Result<
        crate::output::DeleteApplicationReferenceDataSourceOutput,
        crate::error::DeleteApplicationReferenceDataSourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_application_reference_data_source_error(response)
        } else {
            crate::operation_deser::parse_delete_application_reference_data_source_response(
                response,
            )
        }
    }
}

/// <note>
/// <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p>
/// </note>
/// <p>Returns information about a specific Amazon Kinesis Analytics application.</p>
/// <p>If you want to retrieve a list of all applications in your account,
/// use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_ListApplications.html">ListApplications</a> operation.</p>
/// <p>This operation requires permissions to perform the <code>kinesisanalytics:DescribeApplication</code>
/// action. You can use <code>DescribeApplication</code> to get the current application versionId, which you need to call other
/// operations such as <code>Update</code>.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeApplication {
    _private: (),
}
impl DescribeApplication {
    /// Creates a new builder-style object to manufacture [`DescribeApplicationInput`](crate::input::DescribeApplicationInput)
    pub fn builder() -> crate::input::describe_application_input::Builder {
        crate::input::describe_application_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeApplication {
    type Output = std::result::Result<
        crate::output::DescribeApplicationOutput,
        crate::error::DescribeApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_application_error(response)
        } else {
            crate::operation_deser::parse_describe_application_response(response)
        }
    }
}

/// <note>
/// <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p>
/// </note>
/// <p>Infers a schema by evaluating sample records on the specified streaming source (Amazon Kinesis stream or Amazon Kinesis Firehose delivery stream) or S3 object. In the response, the operation returns the inferred schema and also the sample records that the operation used to infer the schema.</p>
/// <p>
/// You can use the inferred schema when configuring a streaming source
/// for your application. For conceptual information,
/// see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>.
/// Note that when you create an application using the Amazon Kinesis Analytics console,
/// the console uses this operation to infer a schema and show it in the console user interface.
/// </p>
/// <p>
/// This operation requires permissions to perform the
/// <code>kinesisanalytics:DiscoverInputSchema</code> action.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DiscoverInputSchema {
    _private: (),
}
impl DiscoverInputSchema {
    /// Creates a new builder-style object to manufacture [`DiscoverInputSchemaInput`](crate::input::DiscoverInputSchemaInput)
    pub fn builder() -> crate::input::discover_input_schema_input::Builder {
        crate::input::discover_input_schema_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DiscoverInputSchema {
    type Output = std::result::Result<
        crate::output::DiscoverInputSchemaOutput,
        crate::error::DiscoverInputSchemaError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_discover_input_schema_error(response)
        } else {
            crate::operation_deser::parse_discover_input_schema_response(response)
        }
    }
}

/// <note>
/// <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p>
/// </note>
/// <p>Returns a list of Amazon Kinesis Analytics applications in your account.
/// For each application, the response includes the application name,
/// Amazon Resource Name (ARN), and status.
/// If the response returns the <code>HasMoreApplications</code> value as true,  
/// you can send another request by adding the
/// <code>ExclusiveStartApplicationName</code> in the request body, and
/// set the value of this to the last application name from
/// the previous response.
/// </p>
/// <p>If you want detailed information about a specific application, use
/// <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a>.</p>
/// <p>This operation requires permissions to perform the
/// <code>kinesisanalytics:ListApplications</code> action.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListApplications {
    _private: (),
}
impl ListApplications {
    /// Creates a new builder-style object to manufacture [`ListApplicationsInput`](crate::input::ListApplicationsInput)
    pub fn builder() -> crate::input::list_applications_input::Builder {
        crate::input::list_applications_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListApplications {
    type Output = std::result::Result<
        crate::output::ListApplicationsOutput,
        crate::error::ListApplicationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_applications_error(response)
        } else {
            crate::operation_deser::parse_list_applications_response(response)
        }
    }
}

/// <p>Retrieves the list of key-value tags assigned to the application. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-tagging.html">Using Tagging</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput)
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// <note>
/// <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p>
/// </note>
/// <p>Starts the specified Amazon Kinesis Analytics application. After creating an application, you must exclusively call this operation to start your application.</p>
/// <p>After the application starts, it begins consuming the input data, processes it, and writes the output to the configured destination.</p>
/// <p>
/// The application status must be <code>READY</code> for you to start an application. You can
/// get the application status in the console or using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation.</p>
/// <p>After you start the application, you can stop the application from processing
/// the input by calling the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_StopApplication.html">StopApplication</a> operation.</p>
/// <p>This operation requires permissions to perform the
/// <code>kinesisanalytics:StartApplication</code> action.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartApplication {
    _private: (),
}
impl StartApplication {
    /// Creates a new builder-style object to manufacture [`StartApplicationInput`](crate::input::StartApplicationInput)
    pub fn builder() -> crate::input::start_application_input::Builder {
        crate::input::start_application_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StartApplication {
    type Output = std::result::Result<
        crate::output::StartApplicationOutput,
        crate::error::StartApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_application_error(response)
        } else {
            crate::operation_deser::parse_start_application_response(response)
        }
    }
}

/// <note>
/// <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p>
/// </note>
/// <p>Stops the application from processing input data. You can stop
/// an application only if it is in the running state.
/// You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to find the application state.
/// After the application is stopped,
/// Amazon Kinesis Analytics stops reading data from the input, the
/// application stops processing data, and there is no output written to the destination. </p>
/// <p>This operation requires permissions to perform the
/// <code>kinesisanalytics:StopApplication</code> action.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StopApplication {
    _private: (),
}
impl StopApplication {
    /// Creates a new builder-style object to manufacture [`StopApplicationInput`](crate::input::StopApplicationInput)
    pub fn builder() -> crate::input::stop_application_input::Builder {
        crate::input::stop_application_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StopApplication {
    type Output = std::result::Result<
        crate::output::StopApplicationOutput,
        crate::error::StopApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_application_error(response)
        } else {
            crate::operation_deser::parse_stop_application_response(response)
        }
    }
}

/// <p>Adds one or more key-value tags to a Kinesis Analytics application. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50.
/// For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-tagging.html">Using Tagging</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput)
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// <p>Removes one or more tags from a Kinesis Analytics application. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-tagging.html">Using Tagging</a>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput)
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// <note>
/// <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p>
/// </note>
/// <p>Updates an existing Amazon Kinesis Analytics application. Using this API,
/// you can update application code, input configuration, and
/// output configuration. </p>
/// <p>Note that Amazon Kinesis Analytics updates the <code>CurrentApplicationVersionId</code>
/// each time you update your application. </p>
/// <p>This operation requires permission for the
/// <code>kinesisanalytics:UpdateApplication</code> action.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateApplication {
    _private: (),
}
impl UpdateApplication {
    /// Creates a new builder-style object to manufacture [`UpdateApplicationInput`](crate::input::UpdateApplicationInput)
    pub fn builder() -> crate::input::update_application_input::Builder {
        crate::input::update_application_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateApplication {
    type Output = std::result::Result<
        crate::output::UpdateApplicationOutput,
        crate::error::UpdateApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_application_error(response)
        } else {
            crate::operation_deser::parse_update_application_response(response)
        }
    }
}