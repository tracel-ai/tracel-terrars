use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct AppsyncResolverData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source: Option<PrimField<String>>,
    field: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_batch_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_template: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caching_config: Option<Vec<AppsyncResolverCachingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_config: Option<Vec<AppsyncResolverPipelineConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime: Option<Vec<AppsyncResolverRuntimeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_config: Option<Vec<AppsyncResolverSyncConfigEl>>,
    dynamic: AppsyncResolverDynamic,
}

struct AppsyncResolver_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppsyncResolverData>,
}

#[derive(Clone)]
pub struct AppsyncResolver(Rc<AppsyncResolver_>);

impl AppsyncResolver {
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

    #[doc = "Set the field `data_source`.\n"]
    pub fn set_data_source(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_source = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `kind`.\n"]
    pub fn set_kind(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kind = Some(v.into());
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

    #[doc = "Set the field `request_template`.\n"]
    pub fn set_request_template(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().request_template = Some(v.into());
        self
    }

    #[doc = "Set the field `response_template`.\n"]
    pub fn set_response_template(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().response_template = Some(v.into());
        self
    }

    #[doc = "Set the field `caching_config`.\n"]
    pub fn set_caching_config(
        self,
        v: impl Into<BlockAssignable<AppsyncResolverCachingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().caching_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.caching_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `pipeline_config`.\n"]
    pub fn set_pipeline_config(
        self,
        v: impl Into<BlockAssignable<AppsyncResolverPipelineConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pipeline_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pipeline_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `runtime`.\n"]
    pub fn set_runtime(self, v: impl Into<BlockAssignable<AppsyncResolverRuntimeEl>>) -> Self {
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
        v: impl Into<BlockAssignable<AppsyncResolverSyncConfigEl>>,
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

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.field", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kind` after provisioning.\n"]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_batch_size` after provisioning.\n"]
    pub fn max_batch_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_batch_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `request_template` after provisioning.\n"]
    pub fn request_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `response_template` after provisioning.\n"]
    pub fn response_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `caching_config` after provisioning.\n"]
    pub fn caching_config(&self) -> ListRef<AppsyncResolverCachingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.caching_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pipeline_config` after provisioning.\n"]
    pub fn pipeline_config(&self) -> ListRef<AppsyncResolverPipelineConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pipeline_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> ListRef<AppsyncResolverRuntimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sync_config` after provisioning.\n"]
    pub fn sync_config(&self) -> ListRef<AppsyncResolverSyncConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sync_config", self.extract_ref()),
        )
    }
}

impl Referable for AppsyncResolver {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for AppsyncResolver {}

impl ToListMappable for AppsyncResolver {
    type O = ListRef<AppsyncResolverRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AppsyncResolver_ {
    fn extract_resource_type(&self) -> String {
        "aws_appsync_resolver".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppsyncResolver {
    pub tf_id: String,
    #[doc = ""]
    pub api_id: PrimField<String>,
    #[doc = ""]
    pub field: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildAppsyncResolver {
    pub fn build(self, stack: &mut Stack) -> AppsyncResolver {
        let out = AppsyncResolver(Rc::new(AppsyncResolver_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppsyncResolverData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_id: self.api_id,
                code: core::default::Default::default(),
                data_source: core::default::Default::default(),
                field: self.field,
                id: core::default::Default::default(),
                kind: core::default::Default::default(),
                max_batch_size: core::default::Default::default(),
                region: core::default::Default::default(),
                request_template: core::default::Default::default(),
                response_template: core::default::Default::default(),
                type_: self.type_,
                caching_config: core::default::Default::default(),
                pipeline_config: core::default::Default::default(),
                runtime: core::default::Default::default(),
                sync_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppsyncResolverRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncResolverRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl AppsyncResolverRef {
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

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.field", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kind` after provisioning.\n"]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_batch_size` after provisioning.\n"]
    pub fn max_batch_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_batch_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `request_template` after provisioning.\n"]
    pub fn request_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `response_template` after provisioning.\n"]
    pub fn response_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `caching_config` after provisioning.\n"]
    pub fn caching_config(&self) -> ListRef<AppsyncResolverCachingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.caching_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pipeline_config` after provisioning.\n"]
    pub fn pipeline_config(&self) -> ListRef<AppsyncResolverPipelineConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pipeline_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> ListRef<AppsyncResolverRuntimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sync_config` after provisioning.\n"]
    pub fn sync_config(&self) -> ListRef<AppsyncResolverSyncConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sync_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AppsyncResolverCachingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    caching_keys: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<f64>>,
}

impl AppsyncResolverCachingConfigEl {
    #[doc = "Set the field `caching_keys`.\n"]
    pub fn set_caching_keys(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.caching_keys = Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ttl = Some(v.into());
        self
    }
}

impl ToListMappable for AppsyncResolverCachingConfigEl {
    type O = BlockAssignable<AppsyncResolverCachingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncResolverCachingConfigEl {}

impl BuildAppsyncResolverCachingConfigEl {
    pub fn build(self) -> AppsyncResolverCachingConfigEl {
        AppsyncResolverCachingConfigEl {
            caching_keys: core::default::Default::default(),
            ttl: core::default::Default::default(),
        }
    }
}

pub struct AppsyncResolverCachingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncResolverCachingConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncResolverCachingConfigElRef {
        AppsyncResolverCachingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncResolverCachingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `caching_keys` after provisioning.\n"]
    pub fn caching_keys(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.caching_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncResolverPipelineConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    functions: Option<ListField<PrimField<String>>>,
}

impl AppsyncResolverPipelineConfigEl {
    #[doc = "Set the field `functions`.\n"]
    pub fn set_functions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.functions = Some(v.into());
        self
    }
}

impl ToListMappable for AppsyncResolverPipelineConfigEl {
    type O = BlockAssignable<AppsyncResolverPipelineConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncResolverPipelineConfigEl {}

impl BuildAppsyncResolverPipelineConfigEl {
    pub fn build(self) -> AppsyncResolverPipelineConfigEl {
        AppsyncResolverPipelineConfigEl {
            functions: core::default::Default::default(),
        }
    }
}

pub struct AppsyncResolverPipelineConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncResolverPipelineConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncResolverPipelineConfigElRef {
        AppsyncResolverPipelineConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncResolverPipelineConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `functions` after provisioning.\n"]
    pub fn functions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.functions", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncResolverRuntimeEl {
    name: PrimField<String>,
    runtime_version: PrimField<String>,
}

impl AppsyncResolverRuntimeEl {}

impl ToListMappable for AppsyncResolverRuntimeEl {
    type O = BlockAssignable<AppsyncResolverRuntimeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncResolverRuntimeEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub runtime_version: PrimField<String>,
}

impl BuildAppsyncResolverRuntimeEl {
    pub fn build(self) -> AppsyncResolverRuntimeEl {
        AppsyncResolverRuntimeEl {
            name: self.name,
            runtime_version: self.runtime_version,
        }
    }
}

pub struct AppsyncResolverRuntimeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncResolverRuntimeElRef {
    fn new(shared: StackShared, base: String) -> AppsyncResolverRuntimeElRef {
        AppsyncResolverRuntimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncResolverRuntimeElRef {
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
pub struct AppsyncResolverSyncConfigElLambdaConflictHandlerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_conflict_handler_arn: Option<PrimField<String>>,
}

impl AppsyncResolverSyncConfigElLambdaConflictHandlerConfigEl {
    #[doc = "Set the field `lambda_conflict_handler_arn`.\n"]
    pub fn set_lambda_conflict_handler_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lambda_conflict_handler_arn = Some(v.into());
        self
    }
}

impl ToListMappable for AppsyncResolverSyncConfigElLambdaConflictHandlerConfigEl {
    type O = BlockAssignable<AppsyncResolverSyncConfigElLambdaConflictHandlerConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncResolverSyncConfigElLambdaConflictHandlerConfigEl {}

impl BuildAppsyncResolverSyncConfigElLambdaConflictHandlerConfigEl {
    pub fn build(self) -> AppsyncResolverSyncConfigElLambdaConflictHandlerConfigEl {
        AppsyncResolverSyncConfigElLambdaConflictHandlerConfigEl {
            lambda_conflict_handler_arn: core::default::Default::default(),
        }
    }
}

pub struct AppsyncResolverSyncConfigElLambdaConflictHandlerConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncResolverSyncConfigElLambdaConflictHandlerConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncResolverSyncConfigElLambdaConflictHandlerConfigElRef {
        AppsyncResolverSyncConfigElLambdaConflictHandlerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncResolverSyncConfigElLambdaConflictHandlerConfigElRef {
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
struct AppsyncResolverSyncConfigElDynamic {
    lambda_conflict_handler_config:
        Option<DynamicBlock<AppsyncResolverSyncConfigElLambdaConflictHandlerConfigEl>>,
}

#[derive(Serialize)]
pub struct AppsyncResolverSyncConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conflict_detection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conflict_handler: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_conflict_handler_config:
        Option<Vec<AppsyncResolverSyncConfigElLambdaConflictHandlerConfigEl>>,
    dynamic: AppsyncResolverSyncConfigElDynamic,
}

impl AppsyncResolverSyncConfigEl {
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
        v: impl Into<BlockAssignable<AppsyncResolverSyncConfigElLambdaConflictHandlerConfigEl>>,
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

impl ToListMappable for AppsyncResolverSyncConfigEl {
    type O = BlockAssignable<AppsyncResolverSyncConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncResolverSyncConfigEl {}

impl BuildAppsyncResolverSyncConfigEl {
    pub fn build(self) -> AppsyncResolverSyncConfigEl {
        AppsyncResolverSyncConfigEl {
            conflict_detection: core::default::Default::default(),
            conflict_handler: core::default::Default::default(),
            lambda_conflict_handler_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppsyncResolverSyncConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncResolverSyncConfigElRef {
    fn new(shared: StackShared, base: String) -> AppsyncResolverSyncConfigElRef {
        AppsyncResolverSyncConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncResolverSyncConfigElRef {
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
    ) -> ListRef<AppsyncResolverSyncConfigElLambdaConflictHandlerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_conflict_handler_config", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AppsyncResolverDynamic {
    caching_config: Option<DynamicBlock<AppsyncResolverCachingConfigEl>>,
    pipeline_config: Option<DynamicBlock<AppsyncResolverPipelineConfigEl>>,
    runtime: Option<DynamicBlock<AppsyncResolverRuntimeEl>>,
    sync_config: Option<DynamicBlock<AppsyncResolverSyncConfigEl>>,
}
