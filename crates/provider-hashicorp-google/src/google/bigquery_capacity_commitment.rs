use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BigqueryCapacityCommitmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_commitment_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_single_admin_project_per_org: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    plan: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    renewal_plan: Option<PrimField<String>>,
    slot_count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigqueryCapacityCommitmentTimeoutsEl>,
}
struct BigqueryCapacityCommitment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryCapacityCommitmentData>,
}
#[derive(Clone)]
pub struct BigqueryCapacityCommitment(Rc<BigqueryCapacityCommitment_>);
impl BigqueryCapacityCommitment {
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
    #[doc = "Set the field `capacity_commitment_id`.\nThe optional capacity commitment ID. Capacity commitment name will be generated automatically if this field is\nempty. This field must only contain lower case alphanumeric characters or dashes. The first and last character\ncannot be a dash. Max length is 64 characters. NOTE: this ID won't be kept if the capacity commitment is split\nor merged."]
    pub fn set_capacity_commitment_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().capacity_commitment_id = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `edition`.\nThe edition type. Valid values are STANDARD, ENTERPRISE, ENTERPRISE_PLUS"]
    pub fn set_edition(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().edition = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_single_admin_project_per_org`.\nIf true, fail the request if another project in the organization has a capacity commitment."]
    pub fn set_enforce_single_admin_project_per_org(self, v: impl Into<PrimField<String>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .enforce_single_admin_project_per_org = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe geographic location where the transfer config should reside.\nExamples: US, EU, asia-northeast1. The default value is US."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `renewal_plan`.\nThe plan this capacity commitment is converted to after commitmentEndTime passes. Once the plan is changed, committed period is extended according to commitment plan. Only applicable for some commitment plans."]
    pub fn set_renewal_plan(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().renewal_plan = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigqueryCapacityCommitmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `capacity_commitment_id` after provisioning.\nThe optional capacity commitment ID. Capacity commitment name will be generated automatically if this field is\nempty. This field must only contain lower case alphanumeric characters or dashes. The first and last character\ncannot be a dash. Max length is 64 characters. NOTE: this ID won't be kept if the capacity commitment is split\nor merged."]
    pub fn capacity_commitment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_commitment_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `commitment_end_time` after provisioning.\nThe start of the current commitment period. It is applicable only for ACTIVE capacity commitments."]
    pub fn commitment_end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.commitment_end_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `commitment_start_time` after provisioning.\nThe start of the current commitment period. It is applicable only for ACTIVE capacity commitments."]
    pub fn commitment_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.commitment_start_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edition` after provisioning.\nThe edition type. Valid values are STANDARD, ENTERPRISE, ENTERPRISE_PLUS"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_single_admin_project_per_org` after provisioning.\nIf true, fail the request if another project in the organization has a capacity commitment."]
    pub fn enforce_single_admin_project_per_org(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.enforce_single_admin_project_per_org",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the transfer config should reside.\nExamples: US, EU, asia-northeast1. The default value is US."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the capacity commitment, e.g., projects/myproject/locations/US/capacityCommitments/123"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `plan` after provisioning.\nCapacity commitment plan. Valid values are at https://cloud.google.com/bigquery/docs/reference/reservations/rpc/google.cloud.bigquery.reservation.v1#commitmentplan"]
    pub fn plan(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plan", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `renewal_plan` after provisioning.\nThe plan this capacity commitment is converted to after commitmentEndTime passes. Once the plan is changed, committed period is extended according to commitment plan. Only applicable for some commitment plans."]
    pub fn renewal_plan(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.renewal_plan", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `slot_count` after provisioning.\nNumber of slots in this commitment."]
    pub fn slot_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slot_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the commitment"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryCapacityCommitmentTimeoutsElRef {
        BigqueryCapacityCommitmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BigqueryCapacityCommitment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BigqueryCapacityCommitment {}
impl ToListMappable for BigqueryCapacityCommitment {
    type O = ListRef<BigqueryCapacityCommitmentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BigqueryCapacityCommitment_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_capacity_commitment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBigqueryCapacityCommitment {
    pub tf_id: String,
    #[doc = "Capacity commitment plan. Valid values are at https://cloud.google.com/bigquery/docs/reference/reservations/rpc/google.cloud.bigquery.reservation.v1#commitmentplan"]
    pub plan: PrimField<String>,
    #[doc = "Number of slots in this commitment."]
    pub slot_count: PrimField<f64>,
}
impl BuildBigqueryCapacityCommitment {
    pub fn build(self, stack: &mut Stack) -> BigqueryCapacityCommitment {
        let out = BigqueryCapacityCommitment(Rc::new(BigqueryCapacityCommitment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigqueryCapacityCommitmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                capacity_commitment_id: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                edition: core::default::Default::default(),
                enforce_single_admin_project_per_org: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                plan: self.plan,
                project: core::default::Default::default(),
                renewal_plan: core::default::Default::default(),
                slot_count: self.slot_count,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BigqueryCapacityCommitmentRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryCapacityCommitmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BigqueryCapacityCommitmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `capacity_commitment_id` after provisioning.\nThe optional capacity commitment ID. Capacity commitment name will be generated automatically if this field is\nempty. This field must only contain lower case alphanumeric characters or dashes. The first and last character\ncannot be a dash. Max length is 64 characters. NOTE: this ID won't be kept if the capacity commitment is split\nor merged."]
    pub fn capacity_commitment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_commitment_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `commitment_end_time` after provisioning.\nThe start of the current commitment period. It is applicable only for ACTIVE capacity commitments."]
    pub fn commitment_end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.commitment_end_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `commitment_start_time` after provisioning.\nThe start of the current commitment period. It is applicable only for ACTIVE capacity commitments."]
    pub fn commitment_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.commitment_start_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edition` after provisioning.\nThe edition type. Valid values are STANDARD, ENTERPRISE, ENTERPRISE_PLUS"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_single_admin_project_per_org` after provisioning.\nIf true, fail the request if another project in the organization has a capacity commitment."]
    pub fn enforce_single_admin_project_per_org(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.enforce_single_admin_project_per_org",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the transfer config should reside.\nExamples: US, EU, asia-northeast1. The default value is US."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the capacity commitment, e.g., projects/myproject/locations/US/capacityCommitments/123"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `plan` after provisioning.\nCapacity commitment plan. Valid values are at https://cloud.google.com/bigquery/docs/reference/reservations/rpc/google.cloud.bigquery.reservation.v1#commitmentplan"]
    pub fn plan(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.plan", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `renewal_plan` after provisioning.\nThe plan this capacity commitment is converted to after commitmentEndTime passes. Once the plan is changed, committed period is extended according to commitment plan. Only applicable for some commitment plans."]
    pub fn renewal_plan(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.renewal_plan", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `slot_count` after provisioning.\nNumber of slots in this commitment."]
    pub fn slot_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.slot_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the commitment"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryCapacityCommitmentTimeoutsElRef {
        BigqueryCapacityCommitmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryCapacityCommitmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BigqueryCapacityCommitmentTimeoutsEl {
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
impl ToListMappable for BigqueryCapacityCommitmentTimeoutsEl {
    type O = BlockAssignable<BigqueryCapacityCommitmentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryCapacityCommitmentTimeoutsEl {}
impl BuildBigqueryCapacityCommitmentTimeoutsEl {
    pub fn build(self) -> BigqueryCapacityCommitmentTimeoutsEl {
        BigqueryCapacityCommitmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BigqueryCapacityCommitmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryCapacityCommitmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryCapacityCommitmentTimeoutsElRef {
        BigqueryCapacityCommitmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryCapacityCommitmentTimeoutsElRef {
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
