use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ContactCenterInsightsAutoLabelingRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_labeling_rule_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label_key_type: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<ContactCenterInsightsAutoLabelingRuleConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ContactCenterInsightsAutoLabelingRuleTimeoutsEl>,
    dynamic: ContactCenterInsightsAutoLabelingRuleDynamic,
}
struct ContactCenterInsightsAutoLabelingRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ContactCenterInsightsAutoLabelingRuleData>,
}
#[derive(Clone)]
pub struct ContactCenterInsightsAutoLabelingRule(Rc<ContactCenterInsightsAutoLabelingRule_>);
impl ContactCenterInsightsAutoLabelingRule {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `active`.\nWhether the rule is active."]
    pub fn set_active(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().active = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_labeling_rule_id`.\nA unique ID for the new AutoLabelingRule. This ID will become the final\ncomponent of the AutoLabelingRule's resource name. If no ID is specified,\na server-generated ID will be used.\n\nThis value should be 4-64 characters and must match the regular\nexpression '^[A-Za-z0-9]{4,64}$'."]
    pub fn set_auto_labeling_rule_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().auto_labeling_rule_id = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe description of the rule."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nDisplay Name of the auto labeling rule."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `label_key`.\nThe label key."]
    pub fn set_label_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().label_key = Some(v.into());
        self
    }
    #[doc = "Set the field `label_key_type`.\nThe type of the label key. Possible values: [\"LABEL_KEY_TYPE_UNSPECIFIED\", \"LABEL_KEY_TYPE_CUSTOM\"]"]
    pub fn set_label_key_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().label_key_type = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        self,
        v: impl Into<BlockAssignable<ContactCenterInsightsAutoLabelingRuleConditionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.conditions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<ContactCenterInsightsAutoLabelingRuleTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `active` after provisioning.\nWhether the rule is active."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_labeling_rule_id` after provisioning.\nA unique ID for the new AutoLabelingRule. This ID will become the final\ncomponent of the AutoLabelingRule's resource name. If no ID is specified,\na server-generated ID will be used.\n\nThis value should be 4-64 characters and must match the regular\nexpression '^[A-Za-z0-9]{4,64}$'."]
    pub fn auto_labeling_rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_labeling_rule_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which this rule was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay Name of the auto labeling rule."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `label_key` after provisioning.\nThe label key."]
    pub fn label_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `label_key_type` after provisioning.\nThe type of the label key. Possible values: [\"LABEL_KEY_TYPE_UNSPECIFIED\", \"LABEL_KEY_TYPE_CUSTOM\"]"]
    pub fn label_key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_key_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the auto labeling rule.\nFormat:\nprojects/{project}/locations/{location}/autoLabelingRules/{auto_labeling_rule}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe most recent time at which this rule was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<ContactCenterInsightsAutoLabelingRuleConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContactCenterInsightsAutoLabelingRuleTimeoutsElRef {
        ContactCenterInsightsAutoLabelingRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ContactCenterInsightsAutoLabelingRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ContactCenterInsightsAutoLabelingRule {}
impl ToListMappable for ContactCenterInsightsAutoLabelingRule {
    type O = ListRef<ContactCenterInsightsAutoLabelingRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ContactCenterInsightsAutoLabelingRule_ {
    fn extract_resource_type(&self) -> String {
        "google_contact_center_insights_auto_labeling_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildContactCenterInsightsAutoLabelingRule {
    pub tf_id: String,
    #[doc = "Location of the resource."]
    pub location: PrimField<String>,
}
impl BuildContactCenterInsightsAutoLabelingRule {
    pub fn build(self, stack: &mut Stack) -> ContactCenterInsightsAutoLabelingRule {
        let out = ContactCenterInsightsAutoLabelingRule(Rc::new(
            ContactCenterInsightsAutoLabelingRule_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(ContactCenterInsightsAutoLabelingRuleData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    active: core::default::Default::default(),
                    auto_labeling_rule_id: core::default::Default::default(),
                    deletion_policy: core::default::Default::default(),
                    description: core::default::Default::default(),
                    display_name: core::default::Default::default(),
                    id: core::default::Default::default(),
                    label_key: core::default::Default::default(),
                    label_key_type: core::default::Default::default(),
                    location: self.location,
                    project: core::default::Default::default(),
                    conditions: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ContactCenterInsightsAutoLabelingRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsAutoLabelingRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ContactCenterInsightsAutoLabelingRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `active` after provisioning.\nWhether the rule is active."]
    pub fn active(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_labeling_rule_id` after provisioning.\nA unique ID for the new AutoLabelingRule. This ID will become the final\ncomponent of the AutoLabelingRule's resource name. If no ID is specified,\na server-generated ID will be used.\n\nThis value should be 4-64 characters and must match the regular\nexpression '^[A-Za-z0-9]{4,64}$'."]
    pub fn auto_labeling_rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_labeling_rule_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which this rule was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay Name of the auto labeling rule."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `label_key` after provisioning.\nThe label key."]
    pub fn label_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `label_key_type` after provisioning.\nThe type of the label key. Possible values: [\"LABEL_KEY_TYPE_UNSPECIFIED\", \"LABEL_KEY_TYPE_CUSTOM\"]"]
    pub fn label_key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_key_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the resource."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the auto labeling rule.\nFormat:\nprojects/{project}/locations/{location}/autoLabelingRules/{auto_labeling_rule}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe most recent time at which this rule was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<ContactCenterInsightsAutoLabelingRuleConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContactCenterInsightsAutoLabelingRuleTimeoutsElRef {
        ContactCenterInsightsAutoLabelingRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ContactCenterInsightsAutoLabelingRuleConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ContactCenterInsightsAutoLabelingRuleConditionsEl {
    #[doc = "Set the field `condition`.\nA optional CEL expression to be evaluated as a boolean value.\nOnce evaluated as true, then we will proceed with the value evaluation.\nAn empty condition will be auto evaluated as true."]
    pub fn set_condition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.condition = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nCEL expression to be evaluated as the value."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for ContactCenterInsightsAutoLabelingRuleConditionsEl {
    type O = BlockAssignable<ContactCenterInsightsAutoLabelingRuleConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsAutoLabelingRuleConditionsEl {}
impl BuildContactCenterInsightsAutoLabelingRuleConditionsEl {
    pub fn build(self) -> ContactCenterInsightsAutoLabelingRuleConditionsEl {
        ContactCenterInsightsAutoLabelingRuleConditionsEl {
            condition: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct ContactCenterInsightsAutoLabelingRuleConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsAutoLabelingRuleConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContactCenterInsightsAutoLabelingRuleConditionsElRef {
        ContactCenterInsightsAutoLabelingRuleConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsAutoLabelingRuleConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\nA optional CEL expression to be evaluated as a boolean value.\nOnce evaluated as true, then we will proceed with the value evaluation.\nAn empty condition will be auto evaluated as true."]
    pub fn condition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nCEL expression to be evaluated as the value."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ContactCenterInsightsAutoLabelingRuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ContactCenterInsightsAutoLabelingRuleTimeoutsEl {
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
impl ToListMappable for ContactCenterInsightsAutoLabelingRuleTimeoutsEl {
    type O = BlockAssignable<ContactCenterInsightsAutoLabelingRuleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContactCenterInsightsAutoLabelingRuleTimeoutsEl {}
impl BuildContactCenterInsightsAutoLabelingRuleTimeoutsEl {
    pub fn build(self) -> ContactCenterInsightsAutoLabelingRuleTimeoutsEl {
        ContactCenterInsightsAutoLabelingRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ContactCenterInsightsAutoLabelingRuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContactCenterInsightsAutoLabelingRuleTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContactCenterInsightsAutoLabelingRuleTimeoutsElRef {
        ContactCenterInsightsAutoLabelingRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContactCenterInsightsAutoLabelingRuleTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct ContactCenterInsightsAutoLabelingRuleDynamic {
    conditions: Option<DynamicBlock<ContactCenterInsightsAutoLabelingRuleConditionsEl>>,
}
