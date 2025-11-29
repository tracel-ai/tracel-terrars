use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WorkspaceswebBrowserSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_encryption_context: Option<RecField<PrimField<String>>>,
    browser_policy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct WorkspaceswebBrowserSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WorkspaceswebBrowserSettingsData>,
}

#[derive(Clone)]
pub struct WorkspaceswebBrowserSettings(Rc<WorkspaceswebBrowserSettings_>);

impl WorkspaceswebBrowserSettings {
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

    #[doc = "Set the field `additional_encryption_context`.\n"]
    pub fn set_additional_encryption_context(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().additional_encryption_context = Some(v.into());
        self
    }

    #[doc = "Set the field `customer_managed_key`.\n"]
    pub fn set_customer_managed_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_managed_key = Some(v.into());
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

    #[doc = "Get a reference to the value of field `additional_encryption_context` after provisioning.\n"]
    pub fn additional_encryption_context(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.additional_encryption_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associated_portal_arns` after provisioning.\n"]
    pub fn associated_portal_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.associated_portal_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `browser_policy` after provisioning.\n"]
    pub fn browser_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.browser_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `browser_settings_arn` after provisioning.\n"]
    pub fn browser_settings_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.browser_settings_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_managed_key` after provisioning.\n"]
    pub fn customer_managed_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_managed_key", self.extract_ref()))
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
}

impl Referable for WorkspaceswebBrowserSettings {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for WorkspaceswebBrowserSettings { }

impl ToListMappable for WorkspaceswebBrowserSettings {
    type O = ListRef<WorkspaceswebBrowserSettingsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for WorkspaceswebBrowserSettings_ {
    fn extract_resource_type(&self) -> String {
        "aws_workspacesweb_browser_settings".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkspaceswebBrowserSettings {
    pub tf_id: String,
    #[doc = ""]
    pub browser_policy: PrimField<String>,
}

impl BuildWorkspaceswebBrowserSettings {
    pub fn build(self, stack: &mut Stack) -> WorkspaceswebBrowserSettings {
        let out = WorkspaceswebBrowserSettings(Rc::new(WorkspaceswebBrowserSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WorkspaceswebBrowserSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                additional_encryption_context: core::default::Default::default(),
                browser_policy: self.browser_policy,
                customer_managed_key: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkspaceswebBrowserSettingsRef {
    shared: StackShared,
    base: String,
}

impl Ref for WorkspaceswebBrowserSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl WorkspaceswebBrowserSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_encryption_context` after provisioning.\n"]
    pub fn additional_encryption_context(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.additional_encryption_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associated_portal_arns` after provisioning.\n"]
    pub fn associated_portal_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.associated_portal_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `browser_policy` after provisioning.\n"]
    pub fn browser_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.browser_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `browser_settings_arn` after provisioning.\n"]
    pub fn browser_settings_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.browser_settings_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_managed_key` after provisioning.\n"]
    pub fn customer_managed_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_managed_key", self.extract_ref()))
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
}
