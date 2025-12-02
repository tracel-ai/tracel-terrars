use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataLambdaFunctionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    function_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qualifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
struct DataLambdaFunction_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLambdaFunctionData>,
}
#[derive(Clone)]
pub struct DataLambdaFunction(Rc<DataLambdaFunction_>);
impl DataLambdaFunction {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `qualifier`.\n"]
    pub fn set_qualifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().qualifier = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `architectures` after provisioning.\n"]
    pub fn architectures(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.architectures", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `code_sha256` after provisioning.\n"]
    pub fn code_sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.code_sha256", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `code_signing_config_arn` after provisioning.\n"]
    pub fn code_signing_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.code_signing_config_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> ListRef<DataLambdaFunctionDeadLetterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dead_letter_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> ListRef<DataLambdaFunctionEnvironmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> ListRef<DataLambdaFunctionEphemeralStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ephemeral_storage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(&self) -> ListRef<DataLambdaFunctionFileSystemConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.file_system_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `handler` after provisioning.\n"]
    pub fn handler(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.handler", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\n"]
    pub fn image_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `invoke_arn` after provisioning.\n"]
    pub fn invoke_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.invoke_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `layers` after provisioning.\n"]
    pub fn layers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.layers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<DataLambdaFunctionLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `qualified_arn` after provisioning.\n"]
    pub fn qualified_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qualified_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `qualified_invoke_arn` after provisioning.\n"]
    pub fn qualified_invoke_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qualified_invoke_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qualifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reserved_concurrent_executions` after provisioning.\n"]
    pub fn reserved_concurrent_executions(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reserved_concurrent_executions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `signing_job_arn` after provisioning.\n"]
    pub fn signing_job_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signing_job_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `signing_profile_version_arn` after provisioning.\n"]
    pub fn signing_profile_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signing_profile_version_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_code_hash` after provisioning.\n"]
    pub fn source_code_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_code_hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_code_size` after provisioning.\n"]
    pub fn source_code_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_code_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_kms_key_arn` after provisioning.\n"]
    pub fn source_kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_kms_key_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tracing_config` after provisioning.\n"]
    pub fn tracing_config(&self) -> ListRef<DataLambdaFunctionTracingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tracing_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<DataLambdaFunctionVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
impl Referable for DataLambdaFunction {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataLambdaFunction {}
impl ToListMappable for DataLambdaFunction {
    type O = ListRef<DataLambdaFunctionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataLambdaFunction_ {
    fn extract_datasource_type(&self) -> String {
        "aws_lambda_function".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataLambdaFunction {
    pub tf_id: String,
    #[doc = ""]
    pub function_name: PrimField<String>,
}
impl BuildDataLambdaFunction {
    pub fn build(self, stack: &mut Stack) -> DataLambdaFunction {
        let out = DataLambdaFunction(Rc::new(DataLambdaFunction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLambdaFunctionData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                function_name: self.function_name,
                id: core::default::Default::default(),
                qualifier: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataLambdaFunctionRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLambdaFunctionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataLambdaFunctionRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `architectures` after provisioning.\n"]
    pub fn architectures(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.architectures", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `code_sha256` after provisioning.\n"]
    pub fn code_sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.code_sha256", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `code_signing_config_arn` after provisioning.\n"]
    pub fn code_signing_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.code_signing_config_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> ListRef<DataLambdaFunctionDeadLetterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dead_letter_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> ListRef<DataLambdaFunctionEnvironmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> ListRef<DataLambdaFunctionEphemeralStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ephemeral_storage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(&self) -> ListRef<DataLambdaFunctionFileSystemConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.file_system_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `handler` after provisioning.\n"]
    pub fn handler(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.handler", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\n"]
    pub fn image_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `invoke_arn` after provisioning.\n"]
    pub fn invoke_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.invoke_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `layers` after provisioning.\n"]
    pub fn layers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.layers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<DataLambdaFunctionLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `qualified_arn` after provisioning.\n"]
    pub fn qualified_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qualified_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `qualified_invoke_arn` after provisioning.\n"]
    pub fn qualified_invoke_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qualified_invoke_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qualifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reserved_concurrent_executions` after provisioning.\n"]
    pub fn reserved_concurrent_executions(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reserved_concurrent_executions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `signing_job_arn` after provisioning.\n"]
    pub fn signing_job_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signing_job_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `signing_profile_version_arn` after provisioning.\n"]
    pub fn signing_profile_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signing_profile_version_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_code_hash` after provisioning.\n"]
    pub fn source_code_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_code_hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_code_size` after provisioning.\n"]
    pub fn source_code_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_code_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_kms_key_arn` after provisioning.\n"]
    pub fn source_kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_kms_key_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tracing_config` after provisioning.\n"]
    pub fn tracing_config(&self) -> ListRef<DataLambdaFunctionTracingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tracing_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<DataLambdaFunctionVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataLambdaFunctionDeadLetterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_arn: Option<PrimField<String>>,
}
impl DataLambdaFunctionDeadLetterConfigEl {
    #[doc = "Set the field `target_arn`.\n"]
    pub fn set_target_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_arn = Some(v.into());
        self
    }
}
impl ToListMappable for DataLambdaFunctionDeadLetterConfigEl {
    type O = BlockAssignable<DataLambdaFunctionDeadLetterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLambdaFunctionDeadLetterConfigEl {}
impl BuildDataLambdaFunctionDeadLetterConfigEl {
    pub fn build(self) -> DataLambdaFunctionDeadLetterConfigEl {
        DataLambdaFunctionDeadLetterConfigEl {
            target_arn: core::default::Default::default(),
        }
    }
}
pub struct DataLambdaFunctionDeadLetterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLambdaFunctionDeadLetterConfigElRef {
    fn new(shared: StackShared, base: String) -> DataLambdaFunctionDeadLetterConfigElRef {
        DataLambdaFunctionDeadLetterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLambdaFunctionDeadLetterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_arn` after provisioning.\n"]
    pub fn target_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLambdaFunctionEnvironmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<RecField<PrimField<String>>>,
}
impl DataLambdaFunctionEnvironmentEl {
    #[doc = "Set the field `variables`.\n"]
    pub fn set_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.variables = Some(v.into());
        self
    }
}
impl ToListMappable for DataLambdaFunctionEnvironmentEl {
    type O = BlockAssignable<DataLambdaFunctionEnvironmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLambdaFunctionEnvironmentEl {}
impl BuildDataLambdaFunctionEnvironmentEl {
    pub fn build(self) -> DataLambdaFunctionEnvironmentEl {
        DataLambdaFunctionEnvironmentEl {
            variables: core::default::Default::default(),
        }
    }
}
pub struct DataLambdaFunctionEnvironmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLambdaFunctionEnvironmentElRef {
    fn new(shared: StackShared, base: String) -> DataLambdaFunctionEnvironmentElRef {
        DataLambdaFunctionEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLambdaFunctionEnvironmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `variables` after provisioning.\n"]
    pub fn variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.variables", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLambdaFunctionEphemeralStorageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
}
impl DataLambdaFunctionEphemeralStorageEl {
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }
}
impl ToListMappable for DataLambdaFunctionEphemeralStorageEl {
    type O = BlockAssignable<DataLambdaFunctionEphemeralStorageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLambdaFunctionEphemeralStorageEl {}
impl BuildDataLambdaFunctionEphemeralStorageEl {
    pub fn build(self) -> DataLambdaFunctionEphemeralStorageEl {
        DataLambdaFunctionEphemeralStorageEl {
            size: core::default::Default::default(),
        }
    }
}
pub struct DataLambdaFunctionEphemeralStorageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLambdaFunctionEphemeralStorageElRef {
    fn new(shared: StackShared, base: String) -> DataLambdaFunctionEphemeralStorageElRef {
        DataLambdaFunctionEphemeralStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLambdaFunctionEphemeralStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLambdaFunctionFileSystemConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_mount_path: Option<PrimField<String>>,
}
impl DataLambdaFunctionFileSystemConfigEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }
    #[doc = "Set the field `local_mount_path`.\n"]
    pub fn set_local_mount_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_mount_path = Some(v.into());
        self
    }
}
impl ToListMappable for DataLambdaFunctionFileSystemConfigEl {
    type O = BlockAssignable<DataLambdaFunctionFileSystemConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLambdaFunctionFileSystemConfigEl {}
impl BuildDataLambdaFunctionFileSystemConfigEl {
    pub fn build(self) -> DataLambdaFunctionFileSystemConfigEl {
        DataLambdaFunctionFileSystemConfigEl {
            arn: core::default::Default::default(),
            local_mount_path: core::default::Default::default(),
        }
    }
}
pub struct DataLambdaFunctionFileSystemConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLambdaFunctionFileSystemConfigElRef {
    fn new(shared: StackShared, base: String) -> DataLambdaFunctionFileSystemConfigElRef {
        DataLambdaFunctionFileSystemConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLambdaFunctionFileSystemConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
    #[doc = "Get a reference to the value of field `local_mount_path` after provisioning.\n"]
    pub fn local_mount_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_mount_path", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLambdaFunctionLoggingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_log_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_log_level: Option<PrimField<String>>,
}
impl DataLambdaFunctionLoggingConfigEl {
    #[doc = "Set the field `application_log_level`.\n"]
    pub fn set_application_log_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.application_log_level = Some(v.into());
        self
    }
    #[doc = "Set the field `log_format`.\n"]
    pub fn set_log_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_format = Some(v.into());
        self
    }
    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group = Some(v.into());
        self
    }
    #[doc = "Set the field `system_log_level`.\n"]
    pub fn set_system_log_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.system_log_level = Some(v.into());
        self
    }
}
impl ToListMappable for DataLambdaFunctionLoggingConfigEl {
    type O = BlockAssignable<DataLambdaFunctionLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLambdaFunctionLoggingConfigEl {}
impl BuildDataLambdaFunctionLoggingConfigEl {
    pub fn build(self) -> DataLambdaFunctionLoggingConfigEl {
        DataLambdaFunctionLoggingConfigEl {
            application_log_level: core::default::Default::default(),
            log_format: core::default::Default::default(),
            log_group: core::default::Default::default(),
            system_log_level: core::default::Default::default(),
        }
    }
}
pub struct DataLambdaFunctionLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLambdaFunctionLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> DataLambdaFunctionLoggingConfigElRef {
        DataLambdaFunctionLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLambdaFunctionLoggingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `application_log_level` after provisioning.\n"]
    pub fn application_log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_log_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `log_format` after provisioning.\n"]
    pub fn log_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_format", self.base))
    }
    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }
    #[doc = "Get a reference to the value of field `system_log_level` after provisioning.\n"]
    pub fn system_log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.system_log_level", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLambdaFunctionTracingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl DataLambdaFunctionTracingConfigEl {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataLambdaFunctionTracingConfigEl {
    type O = BlockAssignable<DataLambdaFunctionTracingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLambdaFunctionTracingConfigEl {}
impl BuildDataLambdaFunctionTracingConfigEl {
    pub fn build(self) -> DataLambdaFunctionTracingConfigEl {
        DataLambdaFunctionTracingConfigEl {
            mode: core::default::Default::default(),
        }
    }
}
pub struct DataLambdaFunctionTracingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLambdaFunctionTracingConfigElRef {
    fn new(shared: StackShared, base: String) -> DataLambdaFunctionTracingConfigElRef {
        DataLambdaFunctionTracingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLambdaFunctionTracingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLambdaFunctionVpcConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_allowed_for_dual_stack: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
}
impl DataLambdaFunctionVpcConfigEl {
    #[doc = "Set the field `ipv6_allowed_for_dual_stack`.\n"]
    pub fn set_ipv6_allowed_for_dual_stack(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ipv6_allowed_for_dual_stack = Some(v.into());
        self
    }
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnet_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataLambdaFunctionVpcConfigEl {
    type O = BlockAssignable<DataLambdaFunctionVpcConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLambdaFunctionVpcConfigEl {}
impl BuildDataLambdaFunctionVpcConfigEl {
    pub fn build(self) -> DataLambdaFunctionVpcConfigEl {
        DataLambdaFunctionVpcConfigEl {
            ipv6_allowed_for_dual_stack: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}
pub struct DataLambdaFunctionVpcConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLambdaFunctionVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> DataLambdaFunctionVpcConfigElRef {
        DataLambdaFunctionVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLambdaFunctionVpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ipv6_allowed_for_dual_stack` after provisioning.\n"]
    pub fn ipv6_allowed_for_dual_stack(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_allowed_for_dual_stack", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}
