use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct AppsyncFunctionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    api_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
    data_source: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_batch_size: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_mapping_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_mapping_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime: Option<Vec<AppsyncFunctionRuntimeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_config: Option<Vec<AppsyncFunctionSyncConfigEl>>,
    dynamic: AppsyncFunctionDynamic,
}

struct AppsyncFunction_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppsyncFunctionData>,
}

#[derive(Clone)]
pub struct AppsyncFunction(Rc<AppsyncFunction_>);

impl AppsyncFunction {
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

    #[doc = "Set the field `code`.\n"]
    pub fn set_code(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().code = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `function_version`.\n"]
    pub fn set_function_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().function_version = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `max_batch_size`.\n"]
    pub fn set_max_batch_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_batch_size = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `request_mapping_template`.\n"]
    pub fn set_request_mapping_template(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().request_mapping_template = Some(v.into());
        self
    }

    #[doc = "Set the field `response_mapping_template`.\n"]
    pub fn set_response_mapping_template(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().response_mapping_template = Some(v.into());
        self
    }

    #[doc = "Set the field `runtime`.\n"]
    pub fn set_runtime(self, v: impl Into<BlockAssignable<AppsyncFunctionRuntimeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().runtime = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.runtime = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `sync_config`.\n"]
    pub fn set_sync_config(
        self,
        v: impl Into<BlockAssignable<AppsyncFunctionSyncConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sync_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sync_config = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `function_id` after provisioning.\n"]
    pub fn function_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `function_version` after provisioning.\n"]
    pub fn function_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_batch_size` after provisioning.\n"]
    pub fn max_batch_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_batch_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `request_mapping_template` after provisioning.\n"]
    pub fn request_mapping_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_mapping_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `response_mapping_template` after provisioning.\n"]
    pub fn response_mapping_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_mapping_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> ListRef<AppsyncFunctionRuntimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sync_config` after provisioning.\n"]
    pub fn sync_config(&self) -> ListRef<AppsyncFunctionSyncConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sync_config", self.extract_ref()),
        )
    }
}

impl Referable for AppsyncFunction {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for AppsyncFunction {}

impl ToListMappable for AppsyncFunction {
    type O = ListRef<AppsyncFunctionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AppsyncFunction_ {
    fn extract_resource_type(&self) -> String {
        "aws_appsync_function".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppsyncFunction {
    pub tf_id: String,
    #[doc = ""]
    pub api_id: PrimField<String>,
    #[doc = ""]
    pub data_source: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAppsyncFunction {
    pub fn build(self, stack: &mut Stack) -> AppsyncFunction {
        let out = AppsyncFunction(Rc::new(AppsyncFunction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppsyncFunctionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_id: self.api_id,
                code: core::default::Default::default(),
                data_source: self.data_source,
                description: core::default::Default::default(),
                function_version: core::default::Default::default(),
                id: core::default::Default::default(),
                max_batch_size: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                request_mapping_template: core::default::Default::default(),
                response_mapping_template: core::default::Default::default(),
                runtime: core::default::Default::default(),
                sync_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppsyncFunctionRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncFunctionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl AppsyncFunctionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `function_id` after provisioning.\n"]
    pub fn function_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `function_version` after provisioning.\n"]
    pub fn function_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_batch_size` after provisioning.\n"]
    pub fn max_batch_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_batch_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `request_mapping_template` after provisioning.\n"]
    pub fn request_mapping_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_mapping_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `response_mapping_template` after provisioning.\n"]
    pub fn response_mapping_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_mapping_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> ListRef<AppsyncFunctionRuntimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sync_config` after provisioning.\n"]
    pub fn sync_config(&self) -> ListRef<AppsyncFunctionSyncConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sync_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AppsyncFunctionRuntimeEl {
    name: PrimField<String>,
    runtime_version: PrimField<String>,
}

impl AppsyncFunctionRuntimeEl {}

impl ToListMappable for AppsyncFunctionRuntimeEl {
    type O = BlockAssignable<AppsyncFunctionRuntimeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncFunctionRuntimeEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub runtime_version: PrimField<String>,
}

impl BuildAppsyncFunctionRuntimeEl {
    pub fn build(self) -> AppsyncFunctionRuntimeEl {
        AppsyncFunctionRuntimeEl {
            name: self.name,
            runtime_version: self.runtime_version,
        }
    }
}

pub struct AppsyncFunctionRuntimeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncFunctionRuntimeElRef {
    fn new(shared: StackShared, base: String) -> AppsyncFunctionRuntimeElRef {
        AppsyncFunctionRuntimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncFunctionRuntimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime_version` after provisioning.\n"]
    pub fn runtime_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime_version", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AppsyncFunctionSyncConfigElLambdaConflictHandlerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_conflict_handler_arn: Option<PrimField<String>>,
}

impl AppsyncFunctionSyncConfigElLambdaConflictHandlerConfigEl {
    #[doc = "Set the field `lambda_conflict_handler_arn`.\n"]
    pub fn set_lambda_conflict_handler_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lambda_conflict_handler_arn = Some(v.into());
        self
    }
}

impl ToListMappable for AppsyncFunctionSyncConfigElLambdaConflictHandlerConfigEl {
    type O = BlockAssignable<AppsyncFunctionSyncConfigElLambdaConflictHandlerConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncFunctionSyncConfigElLambdaConflictHandlerConfigEl {}

impl BuildAppsyncFunctionSyncConfigElLambdaConflictHandlerConfigEl {
    pub fn build(self) -> AppsyncFunctionSyncConfigElLambdaConflictHandlerConfigEl {
        AppsyncFunctionSyncConfigElLambdaConflictHandlerConfigEl {
            lambda_conflict_handler_arn: core::default::Default::default(),
        }
    }
}

pub struct AppsyncFunctionSyncConfigElLambdaConflictHandlerConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncFunctionSyncConfigElLambdaConflictHandlerConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncFunctionSyncConfigElLambdaConflictHandlerConfigElRef {
        AppsyncFunctionSyncConfigElLambdaConflictHandlerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncFunctionSyncConfigElLambdaConflictHandlerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_conflict_handler_arn` after provisioning.\n"]
    pub fn lambda_conflict_handler_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lambda_conflict_handler_arn", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AppsyncFunctionSyncConfigElDynamic {
    lambda_conflict_handler_config:
        Option<DynamicBlock<AppsyncFunctionSyncConfigElLambdaConflictHandlerConfigEl>>,
}

#[derive(Serialize)]
pub struct AppsyncFunctionSyncConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conflict_detection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conflict_handler: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_conflict_handler_config:
        Option<Vec<AppsyncFunctionSyncConfigElLambdaConflictHandlerConfigEl>>,
    dynamic: AppsyncFunctionSyncConfigElDynamic,
}

impl AppsyncFunctionSyncConfigEl {
    #[doc = "Set the field `conflict_detection`.\n"]
    pub fn set_conflict_detection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.conflict_detection = Some(v.into());
        self
    }

    #[doc = "Set the field `conflict_handler`.\n"]
    pub fn set_conflict_handler(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.conflict_handler = Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_conflict_handler_config`.\n"]
    pub fn set_lambda_conflict_handler_config(
        mut self,
        v: impl Into<BlockAssignable<AppsyncFunctionSyncConfigElLambdaConflictHandlerConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda_conflict_handler_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_conflict_handler_config = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppsyncFunctionSyncConfigEl {
    type O = BlockAssignable<AppsyncFunctionSyncConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncFunctionSyncConfigEl {}

impl BuildAppsyncFunctionSyncConfigEl {
    pub fn build(self) -> AppsyncFunctionSyncConfigEl {
        AppsyncFunctionSyncConfigEl {
            conflict_detection: core::default::Default::default(),
            conflict_handler: core::default::Default::default(),
            lambda_conflict_handler_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppsyncFunctionSyncConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncFunctionSyncConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncFunctionSyncConfigElRef {
        AppsyncFunctionSyncConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncFunctionSyncConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conflict_detection` after provisioning.\n"]
    pub fn conflict_detection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.conflict_detection", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `conflict_handler` after provisioning.\n"]
    pub fn conflict_handler(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.conflict_handler", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_conflict_handler_config` after provisioning.\n"]
    pub fn lambda_conflict_handler_config(
        &self,
    ) -> ListRef<AppsyncFunctionSyncConfigElLambdaConflictHandlerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_conflict_handler_config", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AppsyncFunctionDynamic {
    runtime: Option<DynamicBlock<AppsyncFunctionRuntimeEl>>,
    sync_config: Option<DynamicBlock<AppsyncFunctionSyncConfigEl>>,
}
