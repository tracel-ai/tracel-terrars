use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct AppsyncChannelNamespaceData {
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
    code_handlers: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    handler_configs: Option<Vec<AppsyncChannelNamespaceHandlerConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publish_auth_mode: Option<Vec<AppsyncChannelNamespacePublishAuthModeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscribe_auth_mode: Option<Vec<AppsyncChannelNamespaceSubscribeAuthModeEl>>,
    dynamic: AppsyncChannelNamespaceDynamic,
}

struct AppsyncChannelNamespace_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppsyncChannelNamespaceData>,
}

#[derive(Clone)]
pub struct AppsyncChannelNamespace(Rc<AppsyncChannelNamespace_>);

impl AppsyncChannelNamespace {
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

    #[doc = "Set the field `code_handlers`.\n"]
    pub fn set_code_handlers(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().code_handlers = Some(v.into());
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

    #[doc = "Set the field `handler_configs`.\n"]
    pub fn set_handler_configs(
        self,
        v: impl Into<BlockAssignable<AppsyncChannelNamespaceHandlerConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().handler_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.handler_configs = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `publish_auth_mode`.\n"]
    pub fn set_publish_auth_mode(
        self,
        v: impl Into<BlockAssignable<AppsyncChannelNamespacePublishAuthModeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().publish_auth_mode = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.publish_auth_mode = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `subscribe_auth_mode`.\n"]
    pub fn set_subscribe_auth_mode(
        self,
        v: impl Into<BlockAssignable<AppsyncChannelNamespaceSubscribeAuthModeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subscribe_auth_mode = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subscribe_auth_mode = Some(d);
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

    #[doc = "Get a reference to the value of field `channel_namespace_arn` after provisioning.\n"]
    pub fn channel_namespace_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.channel_namespace_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `code_handlers` after provisioning.\n"]
    pub fn code_handlers(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.code_handlers", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `handler_configs` after provisioning.\n"]
    pub fn handler_configs(&self) -> ListRef<AppsyncChannelNamespaceHandlerConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.handler_configs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `publish_auth_mode` after provisioning.\n"]
    pub fn publish_auth_mode(&self) -> ListRef<AppsyncChannelNamespacePublishAuthModeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.publish_auth_mode", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subscribe_auth_mode` after provisioning.\n"]
    pub fn subscribe_auth_mode(&self) -> ListRef<AppsyncChannelNamespaceSubscribeAuthModeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subscribe_auth_mode", self.extract_ref()),
        )
    }
}

impl Referable for AppsyncChannelNamespace {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for AppsyncChannelNamespace {}

impl ToListMappable for AppsyncChannelNamespace {
    type O = ListRef<AppsyncChannelNamespaceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AppsyncChannelNamespace_ {
    fn extract_resource_type(&self) -> String {
        "aws_appsync_channel_namespace".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppsyncChannelNamespace {
    pub tf_id: String,
    #[doc = ""]
    pub api_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAppsyncChannelNamespace {
    pub fn build(self, stack: &mut Stack) -> AppsyncChannelNamespace {
        let out = AppsyncChannelNamespace(Rc::new(AppsyncChannelNamespace_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppsyncChannelNamespaceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_id: self.api_id,
                code_handlers: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                handler_configs: core::default::Default::default(),
                publish_auth_mode: core::default::Default::default(),
                subscribe_auth_mode: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppsyncChannelNamespaceRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncChannelNamespaceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl AppsyncChannelNamespaceRef {
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

    #[doc = "Get a reference to the value of field `channel_namespace_arn` after provisioning.\n"]
    pub fn channel_namespace_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.channel_namespace_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `code_handlers` after provisioning.\n"]
    pub fn code_handlers(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.code_handlers", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `handler_configs` after provisioning.\n"]
    pub fn handler_configs(&self) -> ListRef<AppsyncChannelNamespaceHandlerConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.handler_configs", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `publish_auth_mode` after provisioning.\n"]
    pub fn publish_auth_mode(&self) -> ListRef<AppsyncChannelNamespacePublishAuthModeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.publish_auth_mode", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subscribe_auth_mode` after provisioning.\n"]
    pub fn subscribe_auth_mode(&self) -> ListRef<AppsyncChannelNamespaceSubscribeAuthModeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subscribe_auth_mode", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElLambdaConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    invoke_type: Option<PrimField<String>>,
}

impl AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElLambdaConfigEl {
    #[doc = "Set the field `invoke_type`.\n"]
    pub fn set_invoke_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.invoke_type = Some(v.into());
        self
    }
}

impl ToListMappable
    for AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElLambdaConfigEl
{
    type O = BlockAssignable<
        AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElLambdaConfigEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElLambdaConfigEl {}

impl BuildAppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElLambdaConfigEl {
    pub fn build(
        self,
    ) -> AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElLambdaConfigEl {
        AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElLambdaConfigEl {
            invoke_type: core::default::Default::default(),
        }
    }
}

pub struct AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElLambdaConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElLambdaConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElLambdaConfigElRef {
        AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElLambdaConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElLambdaConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invoke_type` after provisioning.\n"]
    pub fn invoke_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.invoke_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElDynamic {
    lambda_config: Option<
        DynamicBlock<AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElLambdaConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationEl {
    data_source_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_config:
        Option<Vec<AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElLambdaConfigEl>>,
    dynamic: AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElDynamic,
}

impl AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationEl {
    #[doc = "Set the field `lambda_config`.\n"]
    pub fn set_lambda_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElLambdaConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_config = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationEl {
    type O = BlockAssignable<AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationEl {
    #[doc = ""]
    pub data_source_name: PrimField<String>,
}

impl BuildAppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationEl {
    pub fn build(self) -> AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationEl {
        AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationEl {
            data_source_name: self.data_source_name,
            lambda_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElRef {
        AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_source_name` after provisioning.\n"]
    pub fn data_source_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(
        &self,
    ) -> ListRef<AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElLambdaConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_config", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AppsyncChannelNamespaceHandlerConfigsElOnPublishElDynamic {
    integration:
        Option<DynamicBlock<AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationEl>>,
}

#[derive(Serialize)]
pub struct AppsyncChannelNamespaceHandlerConfigsElOnPublishEl {
    behavior: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integration: Option<Vec<AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationEl>>,
    dynamic: AppsyncChannelNamespaceHandlerConfigsElOnPublishElDynamic,
}

impl AppsyncChannelNamespaceHandlerConfigsElOnPublishEl {
    #[doc = "Set the field `integration`.\n"]
    pub fn set_integration(
        mut self,
        v: impl Into<BlockAssignable<AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.integration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.integration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppsyncChannelNamespaceHandlerConfigsElOnPublishEl {
    type O = BlockAssignable<AppsyncChannelNamespaceHandlerConfigsElOnPublishEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncChannelNamespaceHandlerConfigsElOnPublishEl {
    #[doc = ""]
    pub behavior: PrimField<String>,
}

impl BuildAppsyncChannelNamespaceHandlerConfigsElOnPublishEl {
    pub fn build(self) -> AppsyncChannelNamespaceHandlerConfigsElOnPublishEl {
        AppsyncChannelNamespaceHandlerConfigsElOnPublishEl {
            behavior: self.behavior,
            integration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppsyncChannelNamespaceHandlerConfigsElOnPublishElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncChannelNamespaceHandlerConfigsElOnPublishElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncChannelNamespaceHandlerConfigsElOnPublishElRef {
        AppsyncChannelNamespaceHandlerConfigsElOnPublishElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncChannelNamespaceHandlerConfigsElOnPublishElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `behavior` after provisioning.\n"]
    pub fn behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `integration` after provisioning.\n"]
    pub fn integration(
        &self,
    ) -> ListRef<AppsyncChannelNamespaceHandlerConfigsElOnPublishElIntegrationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.integration", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElLambdaConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    invoke_type: Option<PrimField<String>>,
}

impl AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElLambdaConfigEl {
    #[doc = "Set the field `invoke_type`.\n"]
    pub fn set_invoke_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.invoke_type = Some(v.into());
        self
    }
}

impl ToListMappable
    for AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElLambdaConfigEl
{
    type O = BlockAssignable<
        AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElLambdaConfigEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElLambdaConfigEl {}

impl BuildAppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElLambdaConfigEl {
    pub fn build(
        self,
    ) -> AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElLambdaConfigEl {
        AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElLambdaConfigEl {
            invoke_type: core::default::Default::default(),
        }
    }
}

pub struct AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElLambdaConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElLambdaConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElLambdaConfigElRef {
        AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElLambdaConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElLambdaConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invoke_type` after provisioning.\n"]
    pub fn invoke_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.invoke_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElDynamic {
    lambda_config: Option<
        DynamicBlock<
            AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElLambdaConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationEl {
    data_source_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_config: Option<
        Vec<AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElLambdaConfigEl>,
    >,
    dynamic: AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElDynamic,
}

impl AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationEl {
    #[doc = "Set the field `lambda_config`.\n"]
    pub fn set_lambda_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElLambdaConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_config = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationEl {
    type O = BlockAssignable<AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationEl {
    #[doc = ""]
    pub data_source_name: PrimField<String>,
}

impl BuildAppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationEl {
    pub fn build(self) -> AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationEl {
        AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationEl {
            data_source_name: self.data_source_name,
            lambda_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElRef {
        AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_source_name` after provisioning.\n"]
    pub fn data_source_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(
        &self,
    ) -> ListRef<AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElLambdaConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_config", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElDynamic {
    integration:
        Option<DynamicBlock<AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationEl>>,
}

#[derive(Serialize)]
pub struct AppsyncChannelNamespaceHandlerConfigsElOnSubscribeEl {
    behavior: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integration: Option<Vec<AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationEl>>,
    dynamic: AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElDynamic,
}

impl AppsyncChannelNamespaceHandlerConfigsElOnSubscribeEl {
    #[doc = "Set the field `integration`.\n"]
    pub fn set_integration(
        mut self,
        v: impl Into<BlockAssignable<AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.integration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.integration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppsyncChannelNamespaceHandlerConfigsElOnSubscribeEl {
    type O = BlockAssignable<AppsyncChannelNamespaceHandlerConfigsElOnSubscribeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncChannelNamespaceHandlerConfigsElOnSubscribeEl {
    #[doc = ""]
    pub behavior: PrimField<String>,
}

impl BuildAppsyncChannelNamespaceHandlerConfigsElOnSubscribeEl {
    pub fn build(self) -> AppsyncChannelNamespaceHandlerConfigsElOnSubscribeEl {
        AppsyncChannelNamespaceHandlerConfigsElOnSubscribeEl {
            behavior: self.behavior,
            integration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElRef {
        AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `behavior` after provisioning.\n"]
    pub fn behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `integration` after provisioning.\n"]
    pub fn integration(
        &self,
    ) -> ListRef<AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElIntegrationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.integration", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncChannelNamespaceHandlerConfigsElDynamic {
    on_publish: Option<DynamicBlock<AppsyncChannelNamespaceHandlerConfigsElOnPublishEl>>,
    on_subscribe: Option<DynamicBlock<AppsyncChannelNamespaceHandlerConfigsElOnSubscribeEl>>,
}

#[derive(Serialize)]
pub struct AppsyncChannelNamespaceHandlerConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    on_publish: Option<Vec<AppsyncChannelNamespaceHandlerConfigsElOnPublishEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_subscribe: Option<Vec<AppsyncChannelNamespaceHandlerConfigsElOnSubscribeEl>>,
    dynamic: AppsyncChannelNamespaceHandlerConfigsElDynamic,
}

impl AppsyncChannelNamespaceHandlerConfigsEl {
    #[doc = "Set the field `on_publish`.\n"]
    pub fn set_on_publish(
        mut self,
        v: impl Into<BlockAssignable<AppsyncChannelNamespaceHandlerConfigsElOnPublishEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.on_publish = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.on_publish = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `on_subscribe`.\n"]
    pub fn set_on_subscribe(
        mut self,
        v: impl Into<BlockAssignable<AppsyncChannelNamespaceHandlerConfigsElOnSubscribeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.on_subscribe = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.on_subscribe = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppsyncChannelNamespaceHandlerConfigsEl {
    type O = BlockAssignable<AppsyncChannelNamespaceHandlerConfigsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncChannelNamespaceHandlerConfigsEl {}

impl BuildAppsyncChannelNamespaceHandlerConfigsEl {
    pub fn build(self) -> AppsyncChannelNamespaceHandlerConfigsEl {
        AppsyncChannelNamespaceHandlerConfigsEl {
            on_publish: core::default::Default::default(),
            on_subscribe: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppsyncChannelNamespaceHandlerConfigsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncChannelNamespaceHandlerConfigsElRef {
    fn new(shared: StackShared, base: String) -> AppsyncChannelNamespaceHandlerConfigsElRef {
        AppsyncChannelNamespaceHandlerConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncChannelNamespaceHandlerConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_publish` after provisioning.\n"]
    pub fn on_publish(&self) -> ListRef<AppsyncChannelNamespaceHandlerConfigsElOnPublishElRef> {
        ListRef::new(self.shared().clone(), format!("{}.on_publish", self.base))
    }

    #[doc = "Get a reference to the value of field `on_subscribe` after provisioning.\n"]
    pub fn on_subscribe(&self) -> ListRef<AppsyncChannelNamespaceHandlerConfigsElOnSubscribeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.on_subscribe", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncChannelNamespacePublishAuthModeEl {
    auth_type: PrimField<String>,
}

impl AppsyncChannelNamespacePublishAuthModeEl {}

impl ToListMappable for AppsyncChannelNamespacePublishAuthModeEl {
    type O = BlockAssignable<AppsyncChannelNamespacePublishAuthModeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncChannelNamespacePublishAuthModeEl {
    #[doc = ""]
    pub auth_type: PrimField<String>,
}

impl BuildAppsyncChannelNamespacePublishAuthModeEl {
    pub fn build(self) -> AppsyncChannelNamespacePublishAuthModeEl {
        AppsyncChannelNamespacePublishAuthModeEl {
            auth_type: self.auth_type,
        }
    }
}

pub struct AppsyncChannelNamespacePublishAuthModeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncChannelNamespacePublishAuthModeElRef {
    fn new(shared: StackShared, base: String) -> AppsyncChannelNamespacePublishAuthModeElRef {
        AppsyncChannelNamespacePublishAuthModeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncChannelNamespacePublishAuthModeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncChannelNamespaceSubscribeAuthModeEl {
    auth_type: PrimField<String>,
}

impl AppsyncChannelNamespaceSubscribeAuthModeEl {}

impl ToListMappable for AppsyncChannelNamespaceSubscribeAuthModeEl {
    type O = BlockAssignable<AppsyncChannelNamespaceSubscribeAuthModeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncChannelNamespaceSubscribeAuthModeEl {
    #[doc = ""]
    pub auth_type: PrimField<String>,
}

impl BuildAppsyncChannelNamespaceSubscribeAuthModeEl {
    pub fn build(self) -> AppsyncChannelNamespaceSubscribeAuthModeEl {
        AppsyncChannelNamespaceSubscribeAuthModeEl {
            auth_type: self.auth_type,
        }
    }
}

pub struct AppsyncChannelNamespaceSubscribeAuthModeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppsyncChannelNamespaceSubscribeAuthModeElRef {
    fn new(shared: StackShared, base: String) -> AppsyncChannelNamespaceSubscribeAuthModeElRef {
        AppsyncChannelNamespaceSubscribeAuthModeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncChannelNamespaceSubscribeAuthModeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncChannelNamespaceDynamic {
    handler_configs: Option<DynamicBlock<AppsyncChannelNamespaceHandlerConfigsEl>>,
    publish_auth_mode: Option<DynamicBlock<AppsyncChannelNamespacePublishAuthModeEl>>,
    subscribe_auth_mode: Option<DynamicBlock<AppsyncChannelNamespaceSubscribeAuthModeEl>>,
}
