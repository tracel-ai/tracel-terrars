use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SsoadminApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    application_provider_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    instance_arn: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    portal_options: Option<Vec<SsoadminApplicationPortalOptionsEl>>,
    dynamic: SsoadminApplicationDynamic,
}

struct SsoadminApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SsoadminApplicationData>,
}

#[derive(Clone)]
pub struct SsoadminApplication(Rc<SsoadminApplication_>);

impl SsoadminApplication {
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

    #[doc = "Set the field `client_token`.\n"]
    pub fn set_client_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client_token = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `portal_options`.\n"]
    pub fn set_portal_options(self, v: impl Into<BlockAssignable<SsoadminApplicationPortalOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().portal_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.portal_options = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `application_account` after provisioning.\n"]
    pub fn application_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_provider_arn` after provisioning.\n"]
    pub fn application_provider_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_provider_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_token` after provisioning.\n"]
    pub fn client_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `portal_options` after provisioning.\n"]
    pub fn portal_options(&self) -> ListRef<SsoadminApplicationPortalOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.portal_options", self.extract_ref()))
    }
}

impl Referable for SsoadminApplication {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SsoadminApplication { }

impl ToListMappable for SsoadminApplication {
    type O = ListRef<SsoadminApplicationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SsoadminApplication_ {
    fn extract_resource_type(&self) -> String {
        "aws_ssoadmin_application".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsoadminApplication {
    pub tf_id: String,
    #[doc = ""]
    pub application_provider_arn: PrimField<String>,
    #[doc = ""]
    pub instance_arn: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildSsoadminApplication {
    pub fn build(self, stack: &mut Stack) -> SsoadminApplication {
        let out = SsoadminApplication(Rc::new(SsoadminApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SsoadminApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                application_provider_arn: self.application_provider_arn,
                client_token: core::default::Default::default(),
                description: core::default::Default::default(),
                instance_arn: self.instance_arn,
                name: self.name,
                region: core::default::Default::default(),
                status: core::default::Default::default(),
                tags: core::default::Default::default(),
                portal_options: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsoadminApplicationRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsoadminApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SsoadminApplicationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_account` after provisioning.\n"]
    pub fn application_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_provider_arn` after provisioning.\n"]
    pub fn application_provider_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_provider_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_token` after provisioning.\n"]
    pub fn client_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `portal_options` after provisioning.\n"]
    pub fn portal_options(&self) -> ListRef<SsoadminApplicationPortalOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.portal_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SsoadminApplicationPortalOptionsElSignInOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_url: Option<PrimField<String>>,
    origin: PrimField<String>,
}

impl SsoadminApplicationPortalOptionsElSignInOptionsEl {
    #[doc = "Set the field `application_url`.\n"]
    pub fn set_application_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.application_url = Some(v.into());
        self
    }
}

impl ToListMappable for SsoadminApplicationPortalOptionsElSignInOptionsEl {
    type O = BlockAssignable<SsoadminApplicationPortalOptionsElSignInOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsoadminApplicationPortalOptionsElSignInOptionsEl {
    #[doc = ""]
    pub origin: PrimField<String>,
}

impl BuildSsoadminApplicationPortalOptionsElSignInOptionsEl {
    pub fn build(self) -> SsoadminApplicationPortalOptionsElSignInOptionsEl {
        SsoadminApplicationPortalOptionsElSignInOptionsEl {
            application_url: core::default::Default::default(),
            origin: self.origin,
        }
    }
}

pub struct SsoadminApplicationPortalOptionsElSignInOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsoadminApplicationPortalOptionsElSignInOptionsElRef {
    fn new(shared: StackShared, base: String) -> SsoadminApplicationPortalOptionsElSignInOptionsElRef {
        SsoadminApplicationPortalOptionsElSignInOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsoadminApplicationPortalOptionsElSignInOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_url` after provisioning.\n"]
    pub fn application_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_url", self.base))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsoadminApplicationPortalOptionsElDynamic {
    sign_in_options: Option<DynamicBlock<SsoadminApplicationPortalOptionsElSignInOptionsEl>>,
}

#[derive(Serialize)]
pub struct SsoadminApplicationPortalOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sign_in_options: Option<Vec<SsoadminApplicationPortalOptionsElSignInOptionsEl>>,
    dynamic: SsoadminApplicationPortalOptionsElDynamic,
}

impl SsoadminApplicationPortalOptionsEl {
    #[doc = "Set the field `visibility`.\n"]
    pub fn set_visibility(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.visibility = Some(v.into());
        self
    }

    #[doc = "Set the field `sign_in_options`.\n"]
    pub fn set_sign_in_options(
        mut self,
        v: impl Into<BlockAssignable<SsoadminApplicationPortalOptionsElSignInOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sign_in_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sign_in_options = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SsoadminApplicationPortalOptionsEl {
    type O = BlockAssignable<SsoadminApplicationPortalOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsoadminApplicationPortalOptionsEl {}

impl BuildSsoadminApplicationPortalOptionsEl {
    pub fn build(self) -> SsoadminApplicationPortalOptionsEl {
        SsoadminApplicationPortalOptionsEl {
            visibility: core::default::Default::default(),
            sign_in_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SsoadminApplicationPortalOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsoadminApplicationPortalOptionsElRef {
    fn new(shared: StackShared, base: String) -> SsoadminApplicationPortalOptionsElRef {
        SsoadminApplicationPortalOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsoadminApplicationPortalOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.base))
    }

    #[doc = "Get a reference to the value of field `sign_in_options` after provisioning.\n"]
    pub fn sign_in_options(&self) -> ListRef<SsoadminApplicationPortalOptionsElSignInOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sign_in_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsoadminApplicationDynamic {
    portal_options: Option<DynamicBlock<SsoadminApplicationPortalOptionsEl>>,
}
