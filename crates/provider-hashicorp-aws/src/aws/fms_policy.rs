use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FmsPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_all_policy_resources: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_unused_fm_managed_resources: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    exclude_resource_tags: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remediation_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_set_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_tag_logical_operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type_list: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_map: Option<Vec<FmsPolicyExcludeMapEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_map: Option<Vec<FmsPolicyIncludeMapEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_service_policy_data: Option<Vec<FmsPolicySecurityServicePolicyDataEl>>,
    dynamic: FmsPolicyDynamic,
}

struct FmsPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FmsPolicyData>,
}

#[derive(Clone)]
pub struct FmsPolicy(Rc<FmsPolicy_>);

impl FmsPolicy {
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

    #[doc = "Set the field `delete_all_policy_resources`.\n"]
    pub fn set_delete_all_policy_resources(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_all_policy_resources = Some(v.into());
        self
    }

    #[doc = "Set the field `delete_unused_fm_managed_resources`.\n"]
    pub fn set_delete_unused_fm_managed_resources(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_unused_fm_managed_resources = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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

    #[doc = "Set the field `remediation_enabled`.\n"]
    pub fn set_remediation_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().remediation_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_set_ids`.\n"]
    pub fn set_resource_set_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resource_set_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_tag_logical_operator`.\n"]
    pub fn set_resource_tag_logical_operator(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_tag_logical_operator = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resource_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_type = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type_list`.\n"]
    pub fn set_resource_type_list(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resource_type_list = Some(v.into());
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

    #[doc = "Set the field `exclude_map`.\n"]
    pub fn set_exclude_map(self, v: impl Into<BlockAssignable<FmsPolicyExcludeMapEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exclude_map = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.exclude_map = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `include_map`.\n"]
    pub fn set_include_map(self, v: impl Into<BlockAssignable<FmsPolicyIncludeMapEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().include_map = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.include_map = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `security_service_policy_data`.\n"]
    pub fn set_security_service_policy_data(
        self,
        v: impl Into<BlockAssignable<FmsPolicySecurityServicePolicyDataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().security_service_policy_data = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.security_service_policy_data = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_all_policy_resources` after provisioning.\n"]
    pub fn delete_all_policy_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_all_policy_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_unused_fm_managed_resources` after provisioning.\n"]
    pub fn delete_unused_fm_managed_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_unused_fm_managed_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_resource_tags` after provisioning.\n"]
    pub fn exclude_resource_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_update_token` after provisioning.\n"]
    pub fn policy_update_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_update_token", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remediation_enabled` after provisioning.\n"]
    pub fn remediation_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.remediation_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_set_ids` after provisioning.\n"]
    pub fn resource_set_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resource_set_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_tag_logical_operator` after provisioning.\n"]
    pub fn resource_tag_logical_operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_tag_logical_operator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type_list` after provisioning.\n"]
    pub fn resource_type_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resource_type_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_map` after provisioning.\n"]
    pub fn exclude_map(&self) -> ListRef<FmsPolicyExcludeMapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclude_map", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_map` after provisioning.\n"]
    pub fn include_map(&self) -> ListRef<FmsPolicyIncludeMapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.include_map", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_service_policy_data` after provisioning.\n"]
    pub fn security_service_policy_data(&self) -> ListRef<FmsPolicySecurityServicePolicyDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_service_policy_data", self.extract_ref()))
    }
}

impl Referable for FmsPolicy {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for FmsPolicy { }

impl ToListMappable for FmsPolicy {
    type O = ListRef<FmsPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FmsPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_fms_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFmsPolicy {
    pub tf_id: String,
    #[doc = ""]
    pub exclude_resource_tags: PrimField<bool>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildFmsPolicy {
    pub fn build(self, stack: &mut Stack) -> FmsPolicy {
        let out = FmsPolicy(Rc::new(FmsPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FmsPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                delete_all_policy_resources: core::default::Default::default(),
                delete_unused_fm_managed_resources: core::default::Default::default(),
                description: core::default::Default::default(),
                exclude_resource_tags: self.exclude_resource_tags,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                remediation_enabled: core::default::Default::default(),
                resource_set_ids: core::default::Default::default(),
                resource_tag_logical_operator: core::default::Default::default(),
                resource_tags: core::default::Default::default(),
                resource_type: core::default::Default::default(),
                resource_type_list: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                exclude_map: core::default::Default::default(),
                include_map: core::default::Default::default(),
                security_service_policy_data: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FmsPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl FmsPolicyRef {
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

    #[doc = "Get a reference to the value of field `delete_all_policy_resources` after provisioning.\n"]
    pub fn delete_all_policy_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_all_policy_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_unused_fm_managed_resources` after provisioning.\n"]
    pub fn delete_unused_fm_managed_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_unused_fm_managed_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_resource_tags` after provisioning.\n"]
    pub fn exclude_resource_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_update_token` after provisioning.\n"]
    pub fn policy_update_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_update_token", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remediation_enabled` after provisioning.\n"]
    pub fn remediation_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.remediation_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_set_ids` after provisioning.\n"]
    pub fn resource_set_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resource_set_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_tag_logical_operator` after provisioning.\n"]
    pub fn resource_tag_logical_operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_tag_logical_operator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type_list` after provisioning.\n"]
    pub fn resource_type_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resource_type_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_map` after provisioning.\n"]
    pub fn exclude_map(&self) -> ListRef<FmsPolicyExcludeMapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclude_map", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_map` after provisioning.\n"]
    pub fn include_map(&self) -> ListRef<FmsPolicyIncludeMapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.include_map", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_service_policy_data` after provisioning.\n"]
    pub fn security_service_policy_data(&self) -> ListRef<FmsPolicySecurityServicePolicyDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_service_policy_data", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FmsPolicyExcludeMapEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orgunit: Option<SetField<PrimField<String>>>,
}

impl FmsPolicyExcludeMapEl {
    #[doc = "Set the field `account`.\n"]
    pub fn set_account(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.account = Some(v.into());
        self
    }

    #[doc = "Set the field `orgunit`.\n"]
    pub fn set_orgunit(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.orgunit = Some(v.into());
        self
    }
}

impl ToListMappable for FmsPolicyExcludeMapEl {
    type O = BlockAssignable<FmsPolicyExcludeMapEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicyExcludeMapEl {}

impl BuildFmsPolicyExcludeMapEl {
    pub fn build(self) -> FmsPolicyExcludeMapEl {
        FmsPolicyExcludeMapEl {
            account: core::default::Default::default(),
            orgunit: core::default::Default::default(),
        }
    }
}

pub struct FmsPolicyExcludeMapElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicyExcludeMapElRef {
    fn new(shared: StackShared, base: String) -> FmsPolicyExcludeMapElRef {
        FmsPolicyExcludeMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicyExcludeMapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.account", self.base))
    }

    #[doc = "Get a reference to the value of field `orgunit` after provisioning.\n"]
    pub fn orgunit(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.orgunit", self.base))
    }
}

#[derive(Serialize)]
pub struct FmsPolicyIncludeMapEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orgunit: Option<SetField<PrimField<String>>>,
}

impl FmsPolicyIncludeMapEl {
    #[doc = "Set the field `account`.\n"]
    pub fn set_account(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.account = Some(v.into());
        self
    }

    #[doc = "Set the field `orgunit`.\n"]
    pub fn set_orgunit(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.orgunit = Some(v.into());
        self
    }
}

impl ToListMappable for FmsPolicyIncludeMapEl {
    type O = BlockAssignable<FmsPolicyIncludeMapEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicyIncludeMapEl {}

impl BuildFmsPolicyIncludeMapEl {
    pub fn build(self) -> FmsPolicyIncludeMapEl {
        FmsPolicyIncludeMapEl {
            account: core::default::Default::default(),
            orgunit: core::default::Default::default(),
        }
    }
}

pub struct FmsPolicyIncludeMapElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicyIncludeMapElRef {
    fn new(shared: StackShared, base: String) -> FmsPolicyIncludeMapElRef {
        FmsPolicyIncludeMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicyIncludeMapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.account", self.base))
    }

    #[doc = "Get a reference to the value of field `orgunit` after provisioning.\n"]
    pub fn orgunit(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.orgunit", self.base))
    }
}

#[derive(Serialize)]
pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElIcmpTypeCodeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<f64>>,
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElIcmpTypeCodeEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElIcmpTypeCodeEl {
    type O =
        BlockAssignable<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElIcmpTypeCodeEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElIcmpTypeCodeEl {}

impl BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElIcmpTypeCodeEl {
    pub fn build(
        self,
    ) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElIcmpTypeCodeEl {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElIcmpTypeCodeEl {
            code: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElIcmpTypeCodeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElIcmpTypeCodeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElIcmpTypeCodeElRef {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElIcmpTypeCodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElIcmpTypeCodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElPortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElPortRangeEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElPortRangeEl {
    type O =
        BlockAssignable<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElPortRangeEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElPortRangeEl {}

impl BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElPortRangeEl {
    pub fn build(
        self,
    ) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElPortRangeEl {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElPortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElPortRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElPortRangeElRef {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize, Default)]
struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElDynamic {
    icmp_type_code: Option<
        DynamicBlock<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElIcmpTypeCodeEl,
        >,
    >,
    port_range: Option<
        DynamicBlock<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElPortRangeEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
    egress: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr_block: Option<PrimField<String>>,
    protocol: PrimField<String>,
    rule_action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icmp_type_code: Option<
        Vec<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElIcmpTypeCodeEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<
        Vec<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElPortRangeEl,
        >,
    >,
    dynamic: FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElDynamic,
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryEl {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr_block = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_cidr_block = Some(v.into());
        self
    }

    #[doc = "Set the field `icmp_type_code`.\n"]
    pub fn set_icmp_type_code(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElIcmpTypeCodeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.icmp_type_code = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.icmp_type_code = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElPortRangeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.port_range = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.port_range = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryEl {
    type O =
        BlockAssignable<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryEl {
    #[doc = ""]
    pub egress: PrimField<bool>,
    #[doc = ""]
    pub protocol: PrimField<String>,
    #[doc = ""]
    pub rule_action: PrimField<String>,
}

impl BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryEl {
    pub fn build(
        self,
    ) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryEl {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryEl {
            cidr_block: core::default::Default::default(),
            egress: self.egress,
            ipv6_cidr_block: core::default::Default::default(),
            protocol: self.protocol,
            rule_action: self.rule_action,
            icmp_type_code: core::default::Default::default(),
            port_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElRef {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.egress", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_action` after provisioning.\n"]
    pub fn rule_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_action", self.base))
    }

    #[doc = "Get a reference to the value of field `icmp_type_code` after provisioning.\n"]
    pub fn icmp_type_code(
        &self,
    ) -> ListRef<
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElIcmpTypeCodeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.icmp_type_code", self.base))
    }

    #[doc = "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(
        &self,
    ) -> ListRef<
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryElPortRangeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.port_range", self.base))
    }
}

#[derive(Serialize)]
pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElIcmpTypeCodeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<f64>>,
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElIcmpTypeCodeEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElIcmpTypeCodeEl {
    type O =
        BlockAssignable<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElIcmpTypeCodeEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElIcmpTypeCodeEl {}

impl BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElIcmpTypeCodeEl {
    pub fn build(
        self,
    ) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElIcmpTypeCodeEl {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElIcmpTypeCodeEl {
            code: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElIcmpTypeCodeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElIcmpTypeCodeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElIcmpTypeCodeElRef {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElIcmpTypeCodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElIcmpTypeCodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElPortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<PrimField<f64>>,
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElPortRangeEl {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from = Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to = Some(v.into());
        self
    }
}

impl ToListMappable for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElPortRangeEl {
    type O =
        BlockAssignable<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElPortRangeEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElPortRangeEl {}

impl BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElPortRangeEl {
    pub fn build(
        self,
    ) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElPortRangeEl {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElPortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElPortRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElPortRangeElRef {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize, Default)]
struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElDynamic {
    icmp_type_code: Option<
        DynamicBlock<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElIcmpTypeCodeEl,
        >,
    >,
    port_range: Option<
        DynamicBlock<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElPortRangeEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
    egress: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr_block: Option<PrimField<String>>,
    protocol: PrimField<String>,
    rule_action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icmp_type_code: Option<
        Vec<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElIcmpTypeCodeEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<
        Vec<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElPortRangeEl,
        >,
    >,
    dynamic: FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElDynamic,
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryEl {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr_block = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_cidr_block = Some(v.into());
        self
    }

    #[doc = "Set the field `icmp_type_code`.\n"]
    pub fn set_icmp_type_code(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElIcmpTypeCodeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.icmp_type_code = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.icmp_type_code = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElPortRangeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.port_range = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.port_range = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryEl {
    type O =
        BlockAssignable<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryEl {
    #[doc = ""]
    pub egress: PrimField<bool>,
    #[doc = ""]
    pub protocol: PrimField<String>,
    #[doc = ""]
    pub rule_action: PrimField<String>,
}

impl BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryEl {
    pub fn build(
        self,
    ) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryEl {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryEl {
            cidr_block: core::default::Default::default(),
            egress: self.egress,
            ipv6_cidr_block: core::default::Default::default(),
            protocol: self.protocol,
            rule_action: self.rule_action,
            icmp_type_code: core::default::Default::default(),
            port_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElRef {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.egress", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_action` after provisioning.\n"]
    pub fn rule_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_action", self.base))
    }

    #[doc = "Get a reference to the value of field `icmp_type_code` after provisioning.\n"]
    pub fn icmp_type_code(
        &self,
    ) -> ListRef<
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElIcmpTypeCodeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.icmp_type_code", self.base))
    }

    #[doc = "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(
        &self,
    ) -> ListRef<
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryElPortRangeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.port_range", self.base))
    }
}

#[derive(Serialize, Default)]
struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElDynamic {
    first_entry: Option<
        DynamicBlock<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryEl,
        >,
    >,
    last_entry: Option<
        DynamicBlock<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetEl {
    force_remediate_for_first_entries: PrimField<bool>,
    force_remediate_for_last_entries: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_entry: Option<
        Vec<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_entry: Option<
        Vec<FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryEl>,
    >,
    dynamic: FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElDynamic,
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetEl {
    #[doc = "Set the field `first_entry`.\n"]
    pub fn set_first_entry(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElFirstEntryEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.first_entry = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.first_entry = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `last_entry`.\n"]
    pub fn set_last_entry(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElLastEntryEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.last_entry = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.last_entry = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetEl {
    type O =
        BlockAssignable<
            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetEl {
    #[doc = ""]
    pub force_remediate_for_first_entries: PrimField<bool>,
    #[doc = ""]
    pub force_remediate_for_last_entries: PrimField<bool>,
}

impl BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetEl {
    pub fn build(
        self,
    ) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetEl {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetEl {
            force_remediate_for_first_entries: self.force_remediate_for_first_entries,
            force_remediate_for_last_entries: self.force_remediate_for_last_entries,
            first_entry: core::default::Default::default(),
            last_entry: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElRef {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `force_remediate_for_first_entries` after provisioning.\n"]
    pub fn force_remediate_for_first_entries(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_remediate_for_first_entries", self.base))
    }

    #[doc = "Get a reference to the value of field `force_remediate_for_last_entries` after provisioning.\n"]
    pub fn force_remediate_for_last_entries(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_remediate_for_last_entries", self.base))
    }
}

#[derive(Serialize, Default)]
struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElDynamic {
    network_acl_entry_set: Option<
        DynamicBlock<FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetEl>,
    >,
}

#[derive(Serialize)]
pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network_acl_entry_set: Option<
        Vec<FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetEl>,
    >,
    dynamic: FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElDynamic,
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyEl {
    #[doc = "Set the field `network_acl_entry_set`.\n"]
    pub fn set_network_acl_entry_set(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_acl_entry_set = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_acl_entry_set = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyEl {
    type O = BlockAssignable<FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyEl {}

impl BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyEl {
    pub fn build(self) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyEl {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyEl {
            network_acl_entry_set: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElRef {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_acl_entry_set` after provisioning.\n"]
    pub fn network_acl_entry_set(
        &self,
    ) -> ListRef<FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElNetworkAclEntrySetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_acl_entry_set", self.base))
    }
}

#[derive(Serialize)]
pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkFirewallPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    firewall_deployment_model: Option<PrimField<String>>,
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkFirewallPolicyEl {
    #[doc = "Set the field `firewall_deployment_model`.\n"]
    pub fn set_firewall_deployment_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.firewall_deployment_model = Some(v.into());
        self
    }
}

impl ToListMappable for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkFirewallPolicyEl {
    type O = BlockAssignable<FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkFirewallPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkFirewallPolicyEl {}

impl BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkFirewallPolicyEl {
    pub fn build(self) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkFirewallPolicyEl {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkFirewallPolicyEl {
            firewall_deployment_model: core::default::Default::default(),
        }
    }
}

pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkFirewallPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkFirewallPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkFirewallPolicyElRef {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkFirewallPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkFirewallPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `firewall_deployment_model` after provisioning.\n"]
    pub fn firewall_deployment_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_deployment_model", self.base))
    }
}

#[derive(Serialize)]
pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElThirdPartyFirewallPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    firewall_deployment_model: Option<PrimField<String>>,
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElThirdPartyFirewallPolicyEl {
    #[doc = "Set the field `firewall_deployment_model`.\n"]
    pub fn set_firewall_deployment_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.firewall_deployment_model = Some(v.into());
        self
    }
}

impl ToListMappable for FmsPolicySecurityServicePolicyDataElPolicyOptionElThirdPartyFirewallPolicyEl {
    type O = BlockAssignable<FmsPolicySecurityServicePolicyDataElPolicyOptionElThirdPartyFirewallPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElThirdPartyFirewallPolicyEl {}

impl BuildFmsPolicySecurityServicePolicyDataElPolicyOptionElThirdPartyFirewallPolicyEl {
    pub fn build(self) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElThirdPartyFirewallPolicyEl {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElThirdPartyFirewallPolicyEl {
            firewall_deployment_model: core::default::Default::default(),
        }
    }
}

pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElThirdPartyFirewallPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicySecurityServicePolicyDataElPolicyOptionElThirdPartyFirewallPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElThirdPartyFirewallPolicyElRef {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElThirdPartyFirewallPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElThirdPartyFirewallPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `firewall_deployment_model` after provisioning.\n"]
    pub fn firewall_deployment_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_deployment_model", self.base))
    }
}

#[derive(Serialize, Default)]
struct FmsPolicySecurityServicePolicyDataElPolicyOptionElDynamic {
    network_acl_common_policy: Option<
        DynamicBlock<FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyEl>,
    >,
    network_firewall_policy: Option<
        DynamicBlock<FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkFirewallPolicyEl>,
    >,
    third_party_firewall_policy: Option<
        DynamicBlock<FmsPolicySecurityServicePolicyDataElPolicyOptionElThirdPartyFirewallPolicyEl>,
    >,
}

#[derive(Serialize)]
pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network_acl_common_policy: Option<Vec<FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_firewall_policy: Option<Vec<FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkFirewallPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    third_party_firewall_policy: Option<
        Vec<FmsPolicySecurityServicePolicyDataElPolicyOptionElThirdPartyFirewallPolicyEl>,
    >,
    dynamic: FmsPolicySecurityServicePolicyDataElPolicyOptionElDynamic,
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionEl {
    #[doc = "Set the field `network_acl_common_policy`.\n"]
    pub fn set_network_acl_common_policy(
        mut self,
        v: impl Into<BlockAssignable<FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_acl_common_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_acl_common_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_firewall_policy`.\n"]
    pub fn set_network_firewall_policy(
        mut self,
        v: impl Into<BlockAssignable<FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkFirewallPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_firewall_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_firewall_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `third_party_firewall_policy`.\n"]
    pub fn set_third_party_firewall_policy(
        mut self,
        v: impl Into<BlockAssignable<FmsPolicySecurityServicePolicyDataElPolicyOptionElThirdPartyFirewallPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.third_party_firewall_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.third_party_firewall_policy = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for FmsPolicySecurityServicePolicyDataElPolicyOptionEl {
    type O = BlockAssignable<FmsPolicySecurityServicePolicyDataElPolicyOptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicySecurityServicePolicyDataElPolicyOptionEl {}

impl BuildFmsPolicySecurityServicePolicyDataElPolicyOptionEl {
    pub fn build(self) -> FmsPolicySecurityServicePolicyDataElPolicyOptionEl {
        FmsPolicySecurityServicePolicyDataElPolicyOptionEl {
            network_acl_common_policy: core::default::Default::default(),
            network_firewall_policy: core::default::Default::default(),
            third_party_firewall_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct FmsPolicySecurityServicePolicyDataElPolicyOptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicySecurityServicePolicyDataElPolicyOptionElRef {
    fn new(shared: StackShared, base: String) -> FmsPolicySecurityServicePolicyDataElPolicyOptionElRef {
        FmsPolicySecurityServicePolicyDataElPolicyOptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicySecurityServicePolicyDataElPolicyOptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_acl_common_policy` after provisioning.\n"]
    pub fn network_acl_common_policy(
        &self,
    ) -> ListRef<FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkAclCommonPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_acl_common_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `network_firewall_policy` after provisioning.\n"]
    pub fn network_firewall_policy(
        &self,
    ) -> ListRef<FmsPolicySecurityServicePolicyDataElPolicyOptionElNetworkFirewallPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_firewall_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `third_party_firewall_policy` after provisioning.\n"]
    pub fn third_party_firewall_policy(
        &self,
    ) -> ListRef<FmsPolicySecurityServicePolicyDataElPolicyOptionElThirdPartyFirewallPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.third_party_firewall_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct FmsPolicySecurityServicePolicyDataElDynamic {
    policy_option: Option<DynamicBlock<FmsPolicySecurityServicePolicyDataElPolicyOptionEl>>,
}

#[derive(Serialize)]
pub struct FmsPolicySecurityServicePolicyDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_service_data: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_option: Option<Vec<FmsPolicySecurityServicePolicyDataElPolicyOptionEl>>,
    dynamic: FmsPolicySecurityServicePolicyDataElDynamic,
}

impl FmsPolicySecurityServicePolicyDataEl {
    #[doc = "Set the field `managed_service_data`.\n"]
    pub fn set_managed_service_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.managed_service_data = Some(v.into());
        self
    }

    #[doc = "Set the field `policy_option`.\n"]
    pub fn set_policy_option(
        mut self,
        v: impl Into<BlockAssignable<FmsPolicySecurityServicePolicyDataElPolicyOptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.policy_option = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_option = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for FmsPolicySecurityServicePolicyDataEl {
    type O = BlockAssignable<FmsPolicySecurityServicePolicyDataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicySecurityServicePolicyDataEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildFmsPolicySecurityServicePolicyDataEl {
    pub fn build(self) -> FmsPolicySecurityServicePolicyDataEl {
        FmsPolicySecurityServicePolicyDataEl {
            managed_service_data: core::default::Default::default(),
            type_: self.type_,
            policy_option: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct FmsPolicySecurityServicePolicyDataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicySecurityServicePolicyDataElRef {
    fn new(shared: StackShared, base: String) -> FmsPolicySecurityServicePolicyDataElRef {
        FmsPolicySecurityServicePolicyDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicySecurityServicePolicyDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `managed_service_data` after provisioning.\n"]
    pub fn managed_service_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.managed_service_data", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_option` after provisioning.\n"]
    pub fn policy_option(&self) -> ListRef<FmsPolicySecurityServicePolicyDataElPolicyOptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.policy_option", self.base))
    }
}

#[derive(Serialize, Default)]
struct FmsPolicyDynamic {
    exclude_map: Option<DynamicBlock<FmsPolicyExcludeMapEl>>,
    include_map: Option<DynamicBlock<FmsPolicyIncludeMapEl>>,
    security_service_policy_data: Option<DynamicBlock<FmsPolicySecurityServicePolicyDataEl>>,
}
