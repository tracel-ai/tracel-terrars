use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodepipelineWebhookData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    authentication: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    target_action: PrimField<String>,
    target_pipeline: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_configuration: Option<Vec<CodepipelineWebhookAuthenticationConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<CodepipelineWebhookFilterEl>>,
    dynamic: CodepipelineWebhookDynamic,
}

struct CodepipelineWebhook_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CodepipelineWebhookData>,
}

#[derive(Clone)]
pub struct CodepipelineWebhook(Rc<CodepipelineWebhook_>);

impl CodepipelineWebhook {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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

    #[doc = "Set the field `authentication_configuration`.\n"]
    pub fn set_authentication_configuration(
        self,
        v: impl Into<BlockAssignable<CodepipelineWebhookAuthenticationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authentication_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authentication_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<CodepipelineWebhookFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_action` after provisioning.\n"]
    pub fn target_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_pipeline` after provisioning.\n"]
    pub fn target_pipeline(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_pipeline", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_configuration` after provisioning.\n"]
    pub fn authentication_configuration(&self) -> ListRef<CodepipelineWebhookAuthenticationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authentication_configuration", self.extract_ref()))
    }
}

impl Referable for CodepipelineWebhook {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CodepipelineWebhook { }

impl ToListMappable for CodepipelineWebhook {
    type O = ListRef<CodepipelineWebhookRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CodepipelineWebhook_ {
    fn extract_resource_type(&self) -> String {
        "aws_codepipeline_webhook".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodepipelineWebhook {
    pub tf_id: String,
    #[doc = ""]
    pub authentication: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub target_action: PrimField<String>,
    #[doc = ""]
    pub target_pipeline: PrimField<String>,
}

impl BuildCodepipelineWebhook {
    pub fn build(self, stack: &mut Stack) -> CodepipelineWebhook {
        let out = CodepipelineWebhook(Rc::new(CodepipelineWebhook_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CodepipelineWebhookData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                authentication: self.authentication,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                target_action: self.target_action,
                target_pipeline: self.target_pipeline,
                authentication_configuration: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodepipelineWebhookRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineWebhookRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CodepipelineWebhookRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_action` after provisioning.\n"]
    pub fn target_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_pipeline` after provisioning.\n"]
    pub fn target_pipeline(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_pipeline", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_configuration` after provisioning.\n"]
    pub fn authentication_configuration(&self) -> ListRef<CodepipelineWebhookAuthenticationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.authentication_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CodepipelineWebhookAuthenticationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_ip_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_token: Option<PrimField<String>>,
}

impl CodepipelineWebhookAuthenticationConfigurationEl {
    #[doc = "Set the field `allowed_ip_range`.\n"]
    pub fn set_allowed_ip_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allowed_ip_range = Some(v.into());
        self
    }

    #[doc = "Set the field `secret_token`.\n"]
    pub fn set_secret_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_token = Some(v.into());
        self
    }
}

impl ToListMappable for CodepipelineWebhookAuthenticationConfigurationEl {
    type O = BlockAssignable<CodepipelineWebhookAuthenticationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineWebhookAuthenticationConfigurationEl {}

impl BuildCodepipelineWebhookAuthenticationConfigurationEl {
    pub fn build(self) -> CodepipelineWebhookAuthenticationConfigurationEl {
        CodepipelineWebhookAuthenticationConfigurationEl {
            allowed_ip_range: core::default::Default::default(),
            secret_token: core::default::Default::default(),
        }
    }
}

pub struct CodepipelineWebhookAuthenticationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineWebhookAuthenticationConfigurationElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineWebhookAuthenticationConfigurationElRef {
        CodepipelineWebhookAuthenticationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineWebhookAuthenticationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_ip_range` after provisioning.\n"]
    pub fn allowed_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.allowed_ip_range", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_token` after provisioning.\n"]
    pub fn secret_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_token", self.base))
    }
}

#[derive(Serialize)]
pub struct CodepipelineWebhookFilterEl {
    json_path: PrimField<String>,
    match_equals: PrimField<String>,
}

impl CodepipelineWebhookFilterEl { }

impl ToListMappable for CodepipelineWebhookFilterEl {
    type O = BlockAssignable<CodepipelineWebhookFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodepipelineWebhookFilterEl {
    #[doc = ""]
    pub json_path: PrimField<String>,
    #[doc = ""]
    pub match_equals: PrimField<String>,
}

impl BuildCodepipelineWebhookFilterEl {
    pub fn build(self) -> CodepipelineWebhookFilterEl {
        CodepipelineWebhookFilterEl {
            json_path: self.json_path,
            match_equals: self.match_equals,
        }
    }
}

pub struct CodepipelineWebhookFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodepipelineWebhookFilterElRef {
    fn new(shared: StackShared, base: String) -> CodepipelineWebhookFilterElRef {
        CodepipelineWebhookFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodepipelineWebhookFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json_path` after provisioning.\n"]
    pub fn json_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.json_path", self.base))
    }

    #[doc = "Get a reference to the value of field `match_equals` after provisioning.\n"]
    pub fn match_equals(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.match_equals", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodepipelineWebhookDynamic {
    authentication_configuration: Option<DynamicBlock<CodepipelineWebhookAuthenticationConfigurationEl>>,
    filter: Option<DynamicBlock<CodepipelineWebhookFilterEl>>,
}
