use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ApplicationinsightsApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_config_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_create: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cwe_monitor_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grouping_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ops_center_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ops_item_sns_topic_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    resource_group_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
}

struct ApplicationinsightsApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApplicationinsightsApplicationData>,
}

#[derive(Clone)]
pub struct ApplicationinsightsApplication(Rc<ApplicationinsightsApplication_>);

impl ApplicationinsightsApplication {
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

    #[doc = "Set the field `auto_config_enabled`.\n"]
    pub fn set_auto_config_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_config_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `auto_create`.\n"]
    pub fn set_auto_create(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_create = Some(v.into());
        self
    }

    #[doc = "Set the field `cwe_monitor_enabled`.\n"]
    pub fn set_cwe_monitor_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cwe_monitor_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `grouping_type`.\n"]
    pub fn set_grouping_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().grouping_type = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `ops_center_enabled`.\n"]
    pub fn set_ops_center_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ops_center_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `ops_item_sns_topic_arn`.\n"]
    pub fn set_ops_item_sns_topic_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ops_item_sns_topic_arn = Some(v.into());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_config_enabled` after provisioning.\n"]
    pub fn auto_config_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_config_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_create` after provisioning.\n"]
    pub fn auto_create(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_create", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cwe_monitor_enabled` after provisioning.\n"]
    pub fn cwe_monitor_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cwe_monitor_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grouping_type` after provisioning.\n"]
    pub fn grouping_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grouping_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ops_center_enabled` after provisioning.\n"]
    pub fn ops_center_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ops_center_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ops_item_sns_topic_arn` after provisioning.\n"]
    pub fn ops_item_sns_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ops_item_sns_topic_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_group_name` after provisioning.\n"]
    pub fn resource_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_group_name", self.extract_ref()))
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

impl Referable for ApplicationinsightsApplication {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ApplicationinsightsApplication { }

impl ToListMappable for ApplicationinsightsApplication {
    type O = ListRef<ApplicationinsightsApplicationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApplicationinsightsApplication_ {
    fn extract_resource_type(&self) -> String {
        "aws_applicationinsights_application".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApplicationinsightsApplication {
    pub tf_id: String,
    #[doc = ""]
    pub resource_group_name: PrimField<String>,
}

impl BuildApplicationinsightsApplication {
    pub fn build(self, stack: &mut Stack) -> ApplicationinsightsApplication {
        let out = ApplicationinsightsApplication(Rc::new(ApplicationinsightsApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApplicationinsightsApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_config_enabled: core::default::Default::default(),
                auto_create: core::default::Default::default(),
                cwe_monitor_enabled: core::default::Default::default(),
                grouping_type: core::default::Default::default(),
                id: core::default::Default::default(),
                ops_center_enabled: core::default::Default::default(),
                ops_item_sns_topic_arn: core::default::Default::default(),
                region: core::default::Default::default(),
                resource_group_name: self.resource_group_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApplicationinsightsApplicationRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApplicationinsightsApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl ApplicationinsightsApplicationRef {
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

    #[doc = "Get a reference to the value of field `auto_config_enabled` after provisioning.\n"]
    pub fn auto_config_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_config_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_create` after provisioning.\n"]
    pub fn auto_create(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_create", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cwe_monitor_enabled` after provisioning.\n"]
    pub fn cwe_monitor_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cwe_monitor_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grouping_type` after provisioning.\n"]
    pub fn grouping_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grouping_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ops_center_enabled` after provisioning.\n"]
    pub fn ops_center_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ops_center_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ops_item_sns_topic_arn` after provisioning.\n"]
    pub fn ops_item_sns_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ops_item_sns_topic_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_group_name` after provisioning.\n"]
    pub fn resource_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_group_name", self.extract_ref()))
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
