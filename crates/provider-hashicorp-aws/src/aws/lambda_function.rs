use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct LambdaFunctionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    architectures: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_signing_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<PrimField<String>>,
    function_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    handler: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    layers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publish: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace_security_groups_on_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replacement_security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reserved_concurrent_executions: Option<PrimField<f64>>,
    role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_object_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_code_hash: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_letter_config: Option<Vec<LambdaFunctionDeadLetterConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<Vec<LambdaFunctionEnvironmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_storage: Option<Vec<LambdaFunctionEphemeralStorageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_config: Option<Vec<LambdaFunctionFileSystemConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_config: Option<Vec<LambdaFunctionImageConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_config: Option<Vec<LambdaFunctionLoggingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snap_start: Option<Vec<LambdaFunctionSnapStartEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<LambdaFunctionTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracing_config: Option<Vec<LambdaFunctionTracingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_config: Option<Vec<LambdaFunctionVpcConfigEl>>,
    dynamic: LambdaFunctionDynamic,
}
struct LambdaFunction_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LambdaFunctionData>,
}
#[derive(Clone)]
pub struct LambdaFunction(Rc<LambdaFunction_>);
impl LambdaFunction {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }
    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }
    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }
    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => true,
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    }
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }
    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(r.extract_ref());
        self
    }
    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(attr.to_string());
        self
    }
    #[doc = "Set the field `architectures`.\n"]
    pub fn set_architectures(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().architectures = Some(v.into());
        self
    }
    #[doc = "Set the field `code_signing_config_arn`.\n"]
    pub fn set_code_signing_config_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().code_signing_config_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `filename`.\n"]
    pub fn set_filename(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filename = Some(v.into());
        self
    }
    #[doc = "Set the field `handler`.\n"]
    pub fn set_handler(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().handler = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `image_uri`.\n"]
    pub fn set_image_uri(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().image_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `layers`.\n"]
    pub fn set_layers(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().layers = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_size`.\n"]
    pub fn set_memory_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().memory_size = Some(v.into());
        self
    }
    #[doc = "Set the field `package_type`.\n"]
    pub fn set_package_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().package_type = Some(v.into());
        self
    }
    #[doc = "Set the field `publish`.\n"]
    pub fn set_publish(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().publish = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `replace_security_groups_on_destroy`.\n"]
    pub fn set_replace_security_groups_on_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().replace_security_groups_on_destroy = Some(v.into());
        self
    }
    #[doc = "Set the field `replacement_security_group_ids`.\n"]
    pub fn set_replacement_security_group_ids(
        self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().replacement_security_group_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `reserved_concurrent_executions`.\n"]
    pub fn set_reserved_concurrent_executions(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().reserved_concurrent_executions = Some(v.into());
        self
    }
    #[doc = "Set the field `runtime`.\n"]
    pub fn set_runtime(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().runtime = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_bucket`.\n"]
    pub fn set_s3_bucket(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_key`.\n"]
    pub fn set_s3_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_key = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_object_version`.\n"]
    pub fn set_s3_object_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_object_version = Some(v.into());
        self
    }
    #[doc = "Set the field `skip_destroy`.\n"]
    pub fn set_skip_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_destroy = Some(v.into());
        self
    }
    #[doc = "Set the field `source_code_hash`.\n"]
    pub fn set_source_code_hash(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_code_hash = Some(v.into());
        self
    }
    #[doc = "Set the field `source_kms_key_arn`.\n"]
    pub fn set_source_kms_key_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_kms_key_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `dead_letter_config`.\n"]
    pub fn set_dead_letter_config(
        self,
        v: impl Into<BlockAssignable<LambdaFunctionDeadLetterConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dead_letter_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dead_letter_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        self,
        v: impl Into<BlockAssignable<LambdaFunctionEnvironmentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().environment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.environment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ephemeral_storage`.\n"]
    pub fn set_ephemeral_storage(
        self,
        v: impl Into<BlockAssignable<LambdaFunctionEphemeralStorageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ephemeral_storage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ephemeral_storage = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `file_system_config`.\n"]
    pub fn set_file_system_config(
        self,
        v: impl Into<BlockAssignable<LambdaFunctionFileSystemConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().file_system_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.file_system_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `image_config`.\n"]
    pub fn set_image_config(
        self,
        v: impl Into<BlockAssignable<LambdaFunctionImageConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        self,
        v: impl Into<BlockAssignable<LambdaFunctionLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `snap_start`.\n"]
    pub fn set_snap_start(self, v: impl Into<BlockAssignable<LambdaFunctionSnapStartEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().snap_start = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.snap_start = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<LambdaFunctionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `tracing_config`.\n"]
    pub fn set_tracing_config(
        self,
        v: impl Into<BlockAssignable<LambdaFunctionTracingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tracing_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tracing_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(self, v: impl Into<BlockAssignable<LambdaFunctionVpcConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = Some(d);
            }
        }
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filename` after provisioning.\n"]
    pub fn filename(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filename", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `package_type` after provisioning.\n"]
    pub fn package_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `publish` after provisioning.\n"]
    pub fn publish(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publish", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replace_security_groups_on_destroy` after provisioning.\n"]
    pub fn replace_security_groups_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replace_security_groups_on_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replacement_security_group_ids` after provisioning.\n"]
    pub fn replacement_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.replacement_security_group_ids", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_key` after provisioning.\n"]
    pub fn s3_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_object_version` after provisioning.\n"]
    pub fn s3_object_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_object_version", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `skip_destroy` after provisioning.\n"]
    pub fn skip_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_destroy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> ListRef<LambdaFunctionDeadLetterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dead_letter_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> ListRef<LambdaFunctionEnvironmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> ListRef<LambdaFunctionEphemeralStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ephemeral_storage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(&self) -> ListRef<LambdaFunctionFileSystemConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.file_system_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `image_config` after provisioning.\n"]
    pub fn image_config(&self) -> ListRef<LambdaFunctionImageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<LambdaFunctionLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snap_start` after provisioning.\n"]
    pub fn snap_start(&self) -> ListRef<LambdaFunctionSnapStartElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snap_start", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdaFunctionTimeoutsElRef {
        LambdaFunctionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tracing_config` after provisioning.\n"]
    pub fn tracing_config(&self) -> ListRef<LambdaFunctionTracingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tracing_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<LambdaFunctionVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
impl Referable for LambdaFunction {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for LambdaFunction {}
impl ToListMappable for LambdaFunction {
    type O = ListRef<LambdaFunctionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for LambdaFunction_ {
    fn extract_resource_type(&self) -> String {
        "aws_lambda_function".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLambdaFunction {
    pub tf_id: String,
    #[doc = ""]
    pub function_name: PrimField<String>,
    #[doc = ""]
    pub role: PrimField<String>,
}
impl BuildLambdaFunction {
    pub fn build(self, stack: &mut Stack) -> LambdaFunction {
        let out = LambdaFunction(Rc::new(LambdaFunction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LambdaFunctionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                architectures: core::default::Default::default(),
                code_signing_config_arn: core::default::Default::default(),
                description: core::default::Default::default(),
                filename: core::default::Default::default(),
                function_name: self.function_name,
                handler: core::default::Default::default(),
                id: core::default::Default::default(),
                image_uri: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                layers: core::default::Default::default(),
                memory_size: core::default::Default::default(),
                package_type: core::default::Default::default(),
                publish: core::default::Default::default(),
                region: core::default::Default::default(),
                replace_security_groups_on_destroy: core::default::Default::default(),
                replacement_security_group_ids: core::default::Default::default(),
                reserved_concurrent_executions: core::default::Default::default(),
                role: self.role,
                runtime: core::default::Default::default(),
                s3_bucket: core::default::Default::default(),
                s3_key: core::default::Default::default(),
                s3_object_version: core::default::Default::default(),
                skip_destroy: core::default::Default::default(),
                source_code_hash: core::default::Default::default(),
                source_kms_key_arn: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeout: core::default::Default::default(),
                dead_letter_config: core::default::Default::default(),
                environment: core::default::Default::default(),
                ephemeral_storage: core::default::Default::default(),
                file_system_config: core::default::Default::default(),
                image_config: core::default::Default::default(),
                logging_config: core::default::Default::default(),
                snap_start: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                tracing_config: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct LambdaFunctionRef {
    shared: StackShared,
    base: String,
}
impl Ref for LambdaFunctionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl LambdaFunctionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filename` after provisioning.\n"]
    pub fn filename(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filename", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `package_type` after provisioning.\n"]
    pub fn package_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `publish` after provisioning.\n"]
    pub fn publish(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publish", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replace_security_groups_on_destroy` after provisioning.\n"]
    pub fn replace_security_groups_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replace_security_groups_on_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replacement_security_group_ids` after provisioning.\n"]
    pub fn replacement_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.replacement_security_group_ids", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_key` after provisioning.\n"]
    pub fn s3_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_object_version` after provisioning.\n"]
    pub fn s3_object_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_object_version", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `skip_destroy` after provisioning.\n"]
    pub fn skip_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_destroy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> ListRef<LambdaFunctionDeadLetterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dead_letter_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> ListRef<LambdaFunctionEnvironmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> ListRef<LambdaFunctionEphemeralStorageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ephemeral_storage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(&self) -> ListRef<LambdaFunctionFileSystemConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.file_system_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `image_config` after provisioning.\n"]
    pub fn image_config(&self) -> ListRef<LambdaFunctionImageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> ListRef<LambdaFunctionLoggingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snap_start` after provisioning.\n"]
    pub fn snap_start(&self) -> ListRef<LambdaFunctionSnapStartElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snap_start", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdaFunctionTimeoutsElRef {
        LambdaFunctionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tracing_config` after provisioning.\n"]
    pub fn tracing_config(&self) -> ListRef<LambdaFunctionTracingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tracing_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<LambdaFunctionVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct LambdaFunctionDeadLetterConfigEl {
    target_arn: PrimField<String>,
}
impl LambdaFunctionDeadLetterConfigEl {}
impl ToListMappable for LambdaFunctionDeadLetterConfigEl {
    type O = BlockAssignable<LambdaFunctionDeadLetterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLambdaFunctionDeadLetterConfigEl {
    #[doc = ""]
    pub target_arn: PrimField<String>,
}
impl BuildLambdaFunctionDeadLetterConfigEl {
    pub fn build(self) -> LambdaFunctionDeadLetterConfigEl {
        LambdaFunctionDeadLetterConfigEl {
            target_arn: self.target_arn,
        }
    }
}
pub struct LambdaFunctionDeadLetterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LambdaFunctionDeadLetterConfigElRef {
    fn new(shared: StackShared, base: String) -> LambdaFunctionDeadLetterConfigElRef {
        LambdaFunctionDeadLetterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LambdaFunctionDeadLetterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_arn` after provisioning.\n"]
    pub fn target_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct LambdaFunctionEnvironmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<RecField<PrimField<String>>>,
}
impl LambdaFunctionEnvironmentEl {
    #[doc = "Set the field `variables`.\n"]
    pub fn set_variables(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.variables = Some(v.into());
        self
    }
}
impl ToListMappable for LambdaFunctionEnvironmentEl {
    type O = BlockAssignable<LambdaFunctionEnvironmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLambdaFunctionEnvironmentEl {}
impl BuildLambdaFunctionEnvironmentEl {
    pub fn build(self) -> LambdaFunctionEnvironmentEl {
        LambdaFunctionEnvironmentEl {
            variables: core::default::Default::default(),
        }
    }
}
pub struct LambdaFunctionEnvironmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LambdaFunctionEnvironmentElRef {
    fn new(shared: StackShared, base: String) -> LambdaFunctionEnvironmentElRef {
        LambdaFunctionEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LambdaFunctionEnvironmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `variables` after provisioning.\n"]
    pub fn variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.variables", self.base))
    }
}
#[derive(Serialize)]
pub struct LambdaFunctionEphemeralStorageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
}
impl LambdaFunctionEphemeralStorageEl {
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }
}
impl ToListMappable for LambdaFunctionEphemeralStorageEl {
    type O = BlockAssignable<LambdaFunctionEphemeralStorageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLambdaFunctionEphemeralStorageEl {}
impl BuildLambdaFunctionEphemeralStorageEl {
    pub fn build(self) -> LambdaFunctionEphemeralStorageEl {
        LambdaFunctionEphemeralStorageEl {
            size: core::default::Default::default(),
        }
    }
}
pub struct LambdaFunctionEphemeralStorageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LambdaFunctionEphemeralStorageElRef {
    fn new(shared: StackShared, base: String) -> LambdaFunctionEphemeralStorageElRef {
        LambdaFunctionEphemeralStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LambdaFunctionEphemeralStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
}
#[derive(Serialize)]
pub struct LambdaFunctionFileSystemConfigEl {
    arn: PrimField<String>,
    local_mount_path: PrimField<String>,
}
impl LambdaFunctionFileSystemConfigEl {}
impl ToListMappable for LambdaFunctionFileSystemConfigEl {
    type O = BlockAssignable<LambdaFunctionFileSystemConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLambdaFunctionFileSystemConfigEl {
    #[doc = ""]
    pub arn: PrimField<String>,
    #[doc = ""]
    pub local_mount_path: PrimField<String>,
}
impl BuildLambdaFunctionFileSystemConfigEl {
    pub fn build(self) -> LambdaFunctionFileSystemConfigEl {
        LambdaFunctionFileSystemConfigEl {
            arn: self.arn,
            local_mount_path: self.local_mount_path,
        }
    }
}
pub struct LambdaFunctionFileSystemConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LambdaFunctionFileSystemConfigElRef {
    fn new(shared: StackShared, base: String) -> LambdaFunctionFileSystemConfigElRef {
        LambdaFunctionFileSystemConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LambdaFunctionFileSystemConfigElRef {
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
pub struct LambdaFunctionImageConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_point: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_directory: Option<PrimField<String>>,
}
impl LambdaFunctionImageConfigEl {
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }
    #[doc = "Set the field `entry_point`.\n"]
    pub fn set_entry_point(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.entry_point = Some(v.into());
        self
    }
    #[doc = "Set the field `working_directory`.\n"]
    pub fn set_working_directory(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.working_directory = Some(v.into());
        self
    }
}
impl ToListMappable for LambdaFunctionImageConfigEl {
    type O = BlockAssignable<LambdaFunctionImageConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLambdaFunctionImageConfigEl {}
impl BuildLambdaFunctionImageConfigEl {
    pub fn build(self) -> LambdaFunctionImageConfigEl {
        LambdaFunctionImageConfigEl {
            command: core::default::Default::default(),
            entry_point: core::default::Default::default(),
            working_directory: core::default::Default::default(),
        }
    }
}
pub struct LambdaFunctionImageConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LambdaFunctionImageConfigElRef {
    fn new(shared: StackShared, base: String) -> LambdaFunctionImageConfigElRef {
        LambdaFunctionImageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LambdaFunctionImageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
    #[doc = "Get a reference to the value of field `entry_point` after provisioning.\n"]
    pub fn entry_point(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.entry_point", self.base))
    }
    #[doc = "Get a reference to the value of field `working_directory` after provisioning.\n"]
    pub fn working_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.working_directory", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LambdaFunctionLoggingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_log_level: Option<PrimField<String>>,
    log_format: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_log_level: Option<PrimField<String>>,
}
impl LambdaFunctionLoggingConfigEl {
    #[doc = "Set the field `application_log_level`.\n"]
    pub fn set_application_log_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.application_log_level = Some(v.into());
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
impl ToListMappable for LambdaFunctionLoggingConfigEl {
    type O = BlockAssignable<LambdaFunctionLoggingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLambdaFunctionLoggingConfigEl {
    #[doc = ""]
    pub log_format: PrimField<String>,
}
impl BuildLambdaFunctionLoggingConfigEl {
    pub fn build(self) -> LambdaFunctionLoggingConfigEl {
        LambdaFunctionLoggingConfigEl {
            application_log_level: core::default::Default::default(),
            log_format: self.log_format,
            log_group: core::default::Default::default(),
            system_log_level: core::default::Default::default(),
        }
    }
}
pub struct LambdaFunctionLoggingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LambdaFunctionLoggingConfigElRef {
    fn new(shared: StackShared, base: String) -> LambdaFunctionLoggingConfigElRef {
        LambdaFunctionLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LambdaFunctionLoggingConfigElRef {
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
pub struct LambdaFunctionSnapStartEl {
    apply_on: PrimField<String>,
}
impl LambdaFunctionSnapStartEl {}
impl ToListMappable for LambdaFunctionSnapStartEl {
    type O = BlockAssignable<LambdaFunctionSnapStartEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLambdaFunctionSnapStartEl {
    #[doc = ""]
    pub apply_on: PrimField<String>,
}
impl BuildLambdaFunctionSnapStartEl {
    pub fn build(self) -> LambdaFunctionSnapStartEl {
        LambdaFunctionSnapStartEl {
            apply_on: self.apply_on,
        }
    }
}
pub struct LambdaFunctionSnapStartElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LambdaFunctionSnapStartElRef {
    fn new(shared: StackShared, base: String) -> LambdaFunctionSnapStartElRef {
        LambdaFunctionSnapStartElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LambdaFunctionSnapStartElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `apply_on` after provisioning.\n"]
    pub fn apply_on(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.apply_on", self.base))
    }
    #[doc = "Get a reference to the value of field `optimization_status` after provisioning.\n"]
    pub fn optimization_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.optimization_status", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LambdaFunctionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl LambdaFunctionTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for LambdaFunctionTimeoutsEl {
    type O = BlockAssignable<LambdaFunctionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLambdaFunctionTimeoutsEl {}
impl BuildLambdaFunctionTimeoutsEl {
    pub fn build(self) -> LambdaFunctionTimeoutsEl {
        LambdaFunctionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct LambdaFunctionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LambdaFunctionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> LambdaFunctionTimeoutsElRef {
        LambdaFunctionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LambdaFunctionTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize)]
pub struct LambdaFunctionTracingConfigEl {
    mode: PrimField<String>,
}
impl LambdaFunctionTracingConfigEl {}
impl ToListMappable for LambdaFunctionTracingConfigEl {
    type O = BlockAssignable<LambdaFunctionTracingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLambdaFunctionTracingConfigEl {
    #[doc = ""]
    pub mode: PrimField<String>,
}
impl BuildLambdaFunctionTracingConfigEl {
    pub fn build(self) -> LambdaFunctionTracingConfigEl {
        LambdaFunctionTracingConfigEl { mode: self.mode }
    }
}
pub struct LambdaFunctionTracingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LambdaFunctionTracingConfigElRef {
    fn new(shared: StackShared, base: String) -> LambdaFunctionTracingConfigElRef {
        LambdaFunctionTracingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LambdaFunctionTracingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct LambdaFunctionVpcConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_allowed_for_dual_stack: Option<PrimField<bool>>,
    security_group_ids: SetField<PrimField<String>>,
    subnet_ids: SetField<PrimField<String>>,
}
impl LambdaFunctionVpcConfigEl {
    #[doc = "Set the field `ipv6_allowed_for_dual_stack`.\n"]
    pub fn set_ipv6_allowed_for_dual_stack(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ipv6_allowed_for_dual_stack = Some(v.into());
        self
    }
}
impl ToListMappable for LambdaFunctionVpcConfigEl {
    type O = BlockAssignable<LambdaFunctionVpcConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLambdaFunctionVpcConfigEl {
    #[doc = ""]
    pub security_group_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
}
impl BuildLambdaFunctionVpcConfigEl {
    pub fn build(self) -> LambdaFunctionVpcConfigEl {
        LambdaFunctionVpcConfigEl {
            ipv6_allowed_for_dual_stack: core::default::Default::default(),
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
        }
    }
}
pub struct LambdaFunctionVpcConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LambdaFunctionVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> LambdaFunctionVpcConfigElRef {
        LambdaFunctionVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LambdaFunctionVpcConfigElRef {
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
#[derive(Serialize, Default)]
struct LambdaFunctionDynamic {
    dead_letter_config: Option<DynamicBlock<LambdaFunctionDeadLetterConfigEl>>,
    environment: Option<DynamicBlock<LambdaFunctionEnvironmentEl>>,
    ephemeral_storage: Option<DynamicBlock<LambdaFunctionEphemeralStorageEl>>,
    file_system_config: Option<DynamicBlock<LambdaFunctionFileSystemConfigEl>>,
    image_config: Option<DynamicBlock<LambdaFunctionImageConfigEl>>,
    logging_config: Option<DynamicBlock<LambdaFunctionLoggingConfigEl>>,
    snap_start: Option<DynamicBlock<LambdaFunctionSnapStartEl>>,
    tracing_config: Option<DynamicBlock<LambdaFunctionTracingConfigEl>>,
    vpc_config: Option<DynamicBlock<LambdaFunctionVpcConfigEl>>,
}
