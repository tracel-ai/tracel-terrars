use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ChronicleRuleDeploymentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alerting: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    archived: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance: PrimField<String>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    rule: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_frequency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ChronicleRuleDeploymentTimeoutsEl>,
}
struct ChronicleRuleDeployment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ChronicleRuleDeploymentData>,
}
#[derive(Clone)]
pub struct ChronicleRuleDeployment(Rc<ChronicleRuleDeployment_>);
impl ChronicleRuleDeployment {
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
    #[doc = "Set the field `alerting`.\nWhether detections resulting from this deployment should be considered\nalerts."]
    pub fn set_alerting(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().alerting = Some(v.into());
        self
    }
    #[doc = "Set the field `archived`.\nThe archive state of the rule deployment.\nCannot be set to true unless enabled is set to false i.e.\narchiving requires a two-step process: first, disable the rule by\nsetting 'enabled' to false, then set 'archive' to true.\nIf set to true, alerting will automatically be set to false.\nIf currently set to true, enabled, alerting, and run_frequency cannot be\nupdated."]
    pub fn set_archived(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().archived = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\nWhether the rule is currently deployed continuously against incoming data."]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `run_frequency`.\nThe run frequency of the rule deployment.\nPossible values:\nLIVE\nHOURLY\nDAILY"]
    pub fn set_run_frequency(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().run_frequency = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ChronicleRuleDeploymentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `alerting` after provisioning.\nWhether detections resulting from this deployment should be considered\nalerts."]
    pub fn alerting(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alerting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `archive_time` after provisioning.\nOutput only. The timestamp when the rule deployment archive state was last set to true. If the rule deployment's current archive state is not set to true, the field will be empty."]
    pub fn archive_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.archive_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `archived` after provisioning.\nThe archive state of the rule deployment.\nCannot be set to true unless enabled is set to false i.e.\narchiving requires a two-step process: first, disable the rule by\nsetting 'enabled' to false, then set 'archive' to true.\nIf set to true, alerting will automatically be set to false.\nIf currently set to true, enabled, alerting, and run_frequency cannot be\nupdated."]
    pub fn archived(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.archived", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `consumer_rules` after provisioning.\nOutput only. The names of the associated/chained consumer rules. Rules are considered\nconsumers of this rule if their rule text explicitly filters on this rule's ruleid.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}"]
    pub fn consumer_rules(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consumer_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the rule is currently deployed continuously against incoming data."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_state` after provisioning.\nThe execution state of the rule deployment.\nPossible values:\nDEFAULT\nLIMITED\nPAUSED"]
    pub fn execution_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_alert_status_change_time` after provisioning.\nOutput only. The timestamp when the rule deployment alert state was lastly changed. This is filled regardless of the current alert state.E.g. if the current alert status is false, this timestamp will be the timestamp when the alert status was changed to false."]
    pub fn last_alert_status_change_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_alert_status_change_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the rule deployment.\nNote that RuleDeployment is a child of the overall Rule, not any individual\nrevision, so the resource ID segment for the Rule resource must not\nreference a specific revision.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}/deployment"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `producer_rules` after provisioning.\nOutput only. The names of the associated/chained producer rules. Rules are considered\nproducers for this rule if this rule explicitly filters on their ruleid.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}"]
    pub fn producer_rules(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.producer_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\nThe Rule ID of the rule."]
    pub fn rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `run_frequency` after provisioning.\nThe run frequency of the rule deployment.\nPossible values:\nLIVE\nHOURLY\nDAILY"]
    pub fn run_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_frequency", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleRuleDeploymentTimeoutsElRef {
        ChronicleRuleDeploymentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ChronicleRuleDeployment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ChronicleRuleDeployment {}
impl ToListMappable for ChronicleRuleDeployment {
    type O = ListRef<ChronicleRuleDeploymentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ChronicleRuleDeployment_ {
    fn extract_resource_type(&self) -> String {
        "google_chronicle_rule_deployment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildChronicleRuleDeployment {
    pub tf_id: String,
    #[doc = "The unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub instance: PrimField<String>,
    #[doc = "The location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub location: PrimField<String>,
    #[doc = "The Rule ID of the rule."]
    pub rule: PrimField<String>,
}
impl BuildChronicleRuleDeployment {
    pub fn build(self, stack: &mut Stack) -> ChronicleRuleDeployment {
        let out = ChronicleRuleDeployment(Rc::new(ChronicleRuleDeployment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ChronicleRuleDeploymentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                alerting: core::default::Default::default(),
                archived: core::default::Default::default(),
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                project: core::default::Default::default(),
                rule: self.rule,
                run_frequency: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ChronicleRuleDeploymentRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleRuleDeploymentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ChronicleRuleDeploymentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alerting` after provisioning.\nWhether detections resulting from this deployment should be considered\nalerts."]
    pub fn alerting(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alerting", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `archive_time` after provisioning.\nOutput only. The timestamp when the rule deployment archive state was last set to true. If the rule deployment's current archive state is not set to true, the field will be empty."]
    pub fn archive_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.archive_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `archived` after provisioning.\nThe archive state of the rule deployment.\nCannot be set to true unless enabled is set to false i.e.\narchiving requires a two-step process: first, disable the rule by\nsetting 'enabled' to false, then set 'archive' to true.\nIf set to true, alerting will automatically be set to false.\nIf currently set to true, enabled, alerting, and run_frequency cannot be\nupdated."]
    pub fn archived(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.archived", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `consumer_rules` after provisioning.\nOutput only. The names of the associated/chained consumer rules. Rules are considered\nconsumers of this rule if their rule text explicitly filters on this rule's ruleid.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}"]
    pub fn consumer_rules(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consumer_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the rule is currently deployed continuously against incoming data."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_state` after provisioning.\nThe execution state of the rule deployment.\nPossible values:\nDEFAULT\nLIMITED\nPAUSED"]
    pub fn execution_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_alert_status_change_time` after provisioning.\nOutput only. The timestamp when the rule deployment alert state was lastly changed. This is filled regardless of the current alert state.E.g. if the current alert status is false, this timestamp will be the timestamp when the alert status was changed to false."]
    pub fn last_alert_status_change_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_alert_status_change_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the rule deployment.\nNote that RuleDeployment is a child of the overall Rule, not any individual\nrevision, so the resource ID segment for the Rule resource must not\nreference a specific revision.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}/deployment"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `producer_rules` after provisioning.\nOutput only. The names of the associated/chained producer rules. Rules are considered\nproducers for this rule if this rule explicitly filters on their ruleid.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}"]
    pub fn producer_rules(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.producer_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\nThe Rule ID of the rule."]
    pub fn rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `run_frequency` after provisioning.\nThe run frequency of the rule deployment.\nPossible values:\nLIVE\nHOURLY\nDAILY"]
    pub fn run_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_frequency", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleRuleDeploymentTimeoutsElRef {
        ChronicleRuleDeploymentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleRuleDeploymentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ChronicleRuleDeploymentTimeoutsEl {
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
impl ToListMappable for ChronicleRuleDeploymentTimeoutsEl {
    type O = BlockAssignable<ChronicleRuleDeploymentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleRuleDeploymentTimeoutsEl {}
impl BuildChronicleRuleDeploymentTimeoutsEl {
    pub fn build(self) -> ChronicleRuleDeploymentTimeoutsEl {
        ChronicleRuleDeploymentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ChronicleRuleDeploymentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleRuleDeploymentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ChronicleRuleDeploymentTimeoutsElRef {
        ChronicleRuleDeploymentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleRuleDeploymentTimeoutsElRef {
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
