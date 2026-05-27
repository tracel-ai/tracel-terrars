use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeReservationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_at_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_reservation_required: Option<PrimField<bool>>,
    zone: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_after_duration: Option<Vec<ComputeReservationDeleteAfterDurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation_sharing_policy: Option<Vec<ComputeReservationReservationSharingPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_settings: Option<Vec<ComputeReservationShareSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_reservation: Option<Vec<ComputeReservationSpecificReservationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeReservationTimeoutsEl>,
    dynamic: ComputeReservationDynamic,
}
struct ComputeReservation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeReservationData>,
}
#[derive(Clone)]
pub struct ComputeReservation(Rc<ComputeReservation_>);
impl ComputeReservation {
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
    #[doc = "Set the field `delete_at_time`.\nAbsolute time in future when the reservation will be auto-deleted by Compute Engine. Timestamp is represented in RFC3339 text format.\nCannot be used with delete_after_duration."]
    pub fn set_delete_at_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().delete_at_time = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `specific_reservation_required`.\nWhen set to true, only VMs that target this reservation by name can\nconsume this reservation. Otherwise, it can be consumed by VMs with\naffinity for any reservation. Defaults to false."]
    pub fn set_specific_reservation_required(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().specific_reservation_required = Some(v.into());
        self
    }
    #[doc = "Set the field `delete_after_duration`.\n"]
    pub fn set_delete_after_duration(
        self,
        v: impl Into<BlockAssignable<ComputeReservationDeleteAfterDurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().delete_after_duration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.delete_after_duration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `reservation_sharing_policy`.\n"]
    pub fn set_reservation_sharing_policy(
        self,
        v: impl Into<BlockAssignable<ComputeReservationReservationSharingPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reservation_sharing_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reservation_sharing_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `share_settings`.\n"]
    pub fn set_share_settings(
        self,
        v: impl Into<BlockAssignable<ComputeReservationShareSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().share_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.share_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `specific_reservation`.\n"]
    pub fn set_specific_reservation(
        self,
        v: impl Into<BlockAssignable<ComputeReservationSpecificReservationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().specific_reservation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.specific_reservation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeReservationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `block_names` after provisioning.\nList of all reservation block names in the parent reservation."]
    pub fn block_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.block_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `commitment` after provisioning.\nFull or partial URL to a parent commitment. This field displays for\nreservations that are tied to a commitment."]
    pub fn commitment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.commitment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_at_time` after provisioning.\nAbsolute time in future when the reservation will be auto-deleted by Compute Engine. Timestamp is represented in RFC3339 text format.\nCannot be used with delete_after_duration."]
    pub fn delete_at_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_at_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nType of the resource. Always compute#reservations for reservations."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_commitments` after provisioning.\nFull or partial URL to parent commitments. This field displays for reservations that are tied to multiple commitments."]
    pub fn linked_commitments(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linked_commitments", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `reservation_block_count` after provisioning.\nThe number of reservation blocks associated with this reservation."]
    pub fn reservation_block_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reservation_block_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_status` after provisioning.\nStatus information for Reservation resource."]
    pub fn resource_status(&self) -> ListRef<ComputeReservationResourceStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `satisfies_pzs` after provisioning.\nReserved for future use."]
    pub fn satisfies_pzs(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.satisfies_pzs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `specific_reservation_required` after provisioning.\nWhen set to true, only VMs that target this reservation by name can\nconsume this reservation. Otherwise, it can be consumed by VMs with\naffinity for any reservation. Defaults to false."]
    pub fn specific_reservation_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.specific_reservation_required", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the reservation."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone where the reservation is made."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_after_duration` after provisioning.\n"]
    pub fn delete_after_duration(&self) -> ListRef<ComputeReservationDeleteAfterDurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delete_after_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_sharing_policy` after provisioning.\n"]
    pub fn reservation_sharing_policy(
        &self,
    ) -> ListRef<ComputeReservationReservationSharingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_sharing_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `share_settings` after provisioning.\n"]
    pub fn share_settings(&self) -> ListRef<ComputeReservationShareSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.share_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `specific_reservation` after provisioning.\n"]
    pub fn specific_reservation(&self) -> ListRef<ComputeReservationSpecificReservationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.specific_reservation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeReservationTimeoutsElRef {
        ComputeReservationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeReservation {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeReservation {}
impl ToListMappable for ComputeReservation {
    type O = ListRef<ComputeReservationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeReservation_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_reservation".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeReservation {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc = "The zone where the reservation is made."]
    pub zone: PrimField<String>,
}
impl BuildComputeReservation {
    pub fn build(self, stack: &mut Stack) -> ComputeReservation {
        let out = ComputeReservation(Rc::new(ComputeReservation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeReservationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                delete_at_time: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                specific_reservation_required: core::default::Default::default(),
                zone: self.zone,
                delete_after_duration: core::default::Default::default(),
                reservation_sharing_policy: core::default::Default::default(),
                share_settings: core::default::Default::default(),
                specific_reservation: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeReservationRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeReservationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeReservationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `block_names` after provisioning.\nList of all reservation block names in the parent reservation."]
    pub fn block_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.block_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `commitment` after provisioning.\nFull or partial URL to a parent commitment. This field displays for\nreservations that are tied to a commitment."]
    pub fn commitment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.commitment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_at_time` after provisioning.\nAbsolute time in future when the reservation will be auto-deleted by Compute Engine. Timestamp is represented in RFC3339 text format.\nCannot be used with delete_after_duration."]
    pub fn delete_at_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_at_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nType of the resource. Always compute#reservations for reservations."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_commitments` after provisioning.\nFull or partial URL to parent commitments. This field displays for reservations that are tied to multiple commitments."]
    pub fn linked_commitments(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linked_commitments", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `reservation_block_count` after provisioning.\nThe number of reservation blocks associated with this reservation."]
    pub fn reservation_block_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reservation_block_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_status` after provisioning.\nStatus information for Reservation resource."]
    pub fn resource_status(&self) -> ListRef<ComputeReservationResourceStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `satisfies_pzs` after provisioning.\nReserved for future use."]
    pub fn satisfies_pzs(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.satisfies_pzs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `specific_reservation_required` after provisioning.\nWhen set to true, only VMs that target this reservation by name can\nconsume this reservation. Otherwise, it can be consumed by VMs with\naffinity for any reservation. Defaults to false."]
    pub fn specific_reservation_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.specific_reservation_required", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the reservation."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone where the reservation is made."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_after_duration` after provisioning.\n"]
    pub fn delete_after_duration(&self) -> ListRef<ComputeReservationDeleteAfterDurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delete_after_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_sharing_policy` after provisioning.\n"]
    pub fn reservation_sharing_policy(
        &self,
    ) -> ListRef<ComputeReservationReservationSharingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_sharing_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `share_settings` after provisioning.\n"]
    pub fn share_settings(&self) -> ListRef<ComputeReservationShareSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.share_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `specific_reservation` after provisioning.\n"]
    pub fn specific_reservation(&self) -> ListRef<ComputeReservationSpecificReservationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.specific_reservation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeReservationTimeoutsElRef {
        ComputeReservationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeReservationResourceStatusElHealthInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    degraded_block_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    healthy_block_count: Option<PrimField<f64>>,
}
impl ComputeReservationResourceStatusElHealthInfoEl {
    #[doc = "Set the field `degraded_block_count`.\n"]
    pub fn set_degraded_block_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.degraded_block_count = Some(v.into());
        self
    }
    #[doc = "Set the field `health_status`.\n"]
    pub fn set_health_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.health_status = Some(v.into());
        self
    }
    #[doc = "Set the field `healthy_block_count`.\n"]
    pub fn set_healthy_block_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.healthy_block_count = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeReservationResourceStatusElHealthInfoEl {
    type O = BlockAssignable<ComputeReservationResourceStatusElHealthInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeReservationResourceStatusElHealthInfoEl {}
impl BuildComputeReservationResourceStatusElHealthInfoEl {
    pub fn build(self) -> ComputeReservationResourceStatusElHealthInfoEl {
        ComputeReservationResourceStatusElHealthInfoEl {
            degraded_block_count: core::default::Default::default(),
            health_status: core::default::Default::default(),
            healthy_block_count: core::default::Default::default(),
        }
    }
}
pub struct ComputeReservationResourceStatusElHealthInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeReservationResourceStatusElHealthInfoElRef {
    fn new(shared: StackShared, base: String) -> ComputeReservationResourceStatusElHealthInfoElRef {
        ComputeReservationResourceStatusElHealthInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeReservationResourceStatusElHealthInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `degraded_block_count` after provisioning.\n"]
    pub fn degraded_block_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.degraded_block_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `health_status` after provisioning.\n"]
    pub fn health_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_status", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `healthy_block_count` after provisioning.\n"]
    pub fn healthy_block_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.healthy_block_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    can_reschedule: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latest_window_start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_on_shutdown: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_reasons: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_status: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    window_end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    window_start_time: Option<PrimField<String>>,
}
impl ComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl {
    #[doc = "Set the field `can_reschedule`.\n"]
    pub fn set_can_reschedule(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.can_reschedule = Some(v.into());
        self
    }
    #[doc = "Set the field `latest_window_start_time`.\n"]
    pub fn set_latest_window_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.latest_window_start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_on_shutdown`.\n"]
    pub fn set_maintenance_on_shutdown(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.maintenance_on_shutdown = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_reasons`.\n"]
    pub fn set_maintenance_reasons(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.maintenance_reasons = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_status`.\n"]
    pub fn set_maintenance_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.maintenance_status = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `window_end_time`.\n"]
    pub fn set_window_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.window_end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `window_start_time`.\n"]
    pub fn set_window_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.window_start_time = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl
{
    type O = BlockAssignable<
        ComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl
{}
impl BuildComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl {
    pub fn build(
        self,
    ) -> ComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl {
        ComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl {
            can_reschedule: core::default::Default::default(),
            latest_window_start_time: core::default::Default::default(),
            maintenance_on_shutdown: core::default::Default::default(),
            maintenance_reasons: core::default::Default::default(),
            maintenance_status: core::default::Default::default(),
            type_: core::default::Default::default(),
            window_end_time: core::default::Default::default(),
            window_start_time: core::default::Default::default(),
        }
    }
}
pub struct ComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceElRef
    {
        ComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `can_reschedule` after provisioning.\n"]
    pub fn can_reschedule(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.can_reschedule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `latest_window_start_time` after provisioning.\n"]
    pub fn latest_window_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_window_start_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_on_shutdown` after provisioning.\n"]
    pub fn maintenance_on_shutdown(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_on_shutdown", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_reasons` after provisioning.\n"]
    pub fn maintenance_reasons(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_reasons", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_status` after provisioning.\n"]
    pub fn maintenance_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_status", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `window_end_time` after provisioning.\n"]
    pub fn window_end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.window_end_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `window_start_time` after provisioning.\n"]
    pub fn window_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.window_start_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeReservationResourceStatusElReservationMaintenanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_maintenance_ongoing_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_maintenance_pending_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_ongoing_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_pending_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduling_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subblock_infra_maintenance_ongoing_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subblock_infra_maintenance_pending_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upcoming_group_maintenance: Option<
        ListField<
            ComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl,
        >,
    >,
}
impl ComputeReservationResourceStatusElReservationMaintenanceEl {
    #[doc = "Set the field `instance_maintenance_ongoing_count`.\n"]
    pub fn set_instance_maintenance_ongoing_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_maintenance_ongoing_count = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_maintenance_pending_count`.\n"]
    pub fn set_instance_maintenance_pending_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_maintenance_pending_count = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_ongoing_count`.\n"]
    pub fn set_maintenance_ongoing_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maintenance_ongoing_count = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_pending_count`.\n"]
    pub fn set_maintenance_pending_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maintenance_pending_count = Some(v.into());
        self
    }
    #[doc = "Set the field `scheduling_type`.\n"]
    pub fn set_scheduling_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scheduling_type = Some(v.into());
        self
    }
    #[doc = "Set the field `subblock_infra_maintenance_ongoing_count`.\n"]
    pub fn set_subblock_infra_maintenance_ongoing_count(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.subblock_infra_maintenance_ongoing_count = Some(v.into());
        self
    }
    #[doc = "Set the field `subblock_infra_maintenance_pending_count`.\n"]
    pub fn set_subblock_infra_maintenance_pending_count(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.subblock_infra_maintenance_pending_count = Some(v.into());
        self
    }
    #[doc = "Set the field `upcoming_group_maintenance`.\n"]
    pub fn set_upcoming_group_maintenance(
        mut self,
        v : impl Into < ListField < ComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl > >,
    ) -> Self {
        self.upcoming_group_maintenance = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeReservationResourceStatusElReservationMaintenanceEl {
    type O = BlockAssignable<ComputeReservationResourceStatusElReservationMaintenanceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeReservationResourceStatusElReservationMaintenanceEl {}
impl BuildComputeReservationResourceStatusElReservationMaintenanceEl {
    pub fn build(self) -> ComputeReservationResourceStatusElReservationMaintenanceEl {
        ComputeReservationResourceStatusElReservationMaintenanceEl {
            instance_maintenance_ongoing_count: core::default::Default::default(),
            instance_maintenance_pending_count: core::default::Default::default(),
            maintenance_ongoing_count: core::default::Default::default(),
            maintenance_pending_count: core::default::Default::default(),
            scheduling_type: core::default::Default::default(),
            subblock_infra_maintenance_ongoing_count: core::default::Default::default(),
            subblock_infra_maintenance_pending_count: core::default::Default::default(),
            upcoming_group_maintenance: core::default::Default::default(),
        }
    }
}
pub struct ComputeReservationResourceStatusElReservationMaintenanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeReservationResourceStatusElReservationMaintenanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeReservationResourceStatusElReservationMaintenanceElRef {
        ComputeReservationResourceStatusElReservationMaintenanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeReservationResourceStatusElReservationMaintenanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_maintenance_ongoing_count` after provisioning.\n"]
    pub fn instance_maintenance_ongoing_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_maintenance_ongoing_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_maintenance_pending_count` after provisioning.\n"]
    pub fn instance_maintenance_pending_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_maintenance_pending_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_ongoing_count` after provisioning.\n"]
    pub fn maintenance_ongoing_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_ongoing_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_pending_count` after provisioning.\n"]
    pub fn maintenance_pending_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_pending_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scheduling_type` after provisioning.\n"]
    pub fn scheduling_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduling_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subblock_infra_maintenance_ongoing_count` after provisioning.\n"]
    pub fn subblock_infra_maintenance_ongoing_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subblock_infra_maintenance_ongoing_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subblock_infra_maintenance_pending_count` after provisioning.\n"]
    pub fn subblock_infra_maintenance_pending_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subblock_infra_maintenance_pending_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `upcoming_group_maintenance` after provisioning.\n"]
    pub fn upcoming_group_maintenance(
        &self,
    ) -> ListRef<
        ComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upcoming_group_maintenance", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeReservationResourceStatusElSpecificSkuAllocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_instance_template_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    utilizations: Option<RecField<PrimField<String>>>,
}
impl ComputeReservationResourceStatusElSpecificSkuAllocationEl {
    #[doc = "Set the field `source_instance_template_id`.\n"]
    pub fn set_source_instance_template_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_instance_template_id = Some(v.into());
        self
    }
    #[doc = "Set the field `utilizations`.\n"]
    pub fn set_utilizations(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.utilizations = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeReservationResourceStatusElSpecificSkuAllocationEl {
    type O = BlockAssignable<ComputeReservationResourceStatusElSpecificSkuAllocationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeReservationResourceStatusElSpecificSkuAllocationEl {}
impl BuildComputeReservationResourceStatusElSpecificSkuAllocationEl {
    pub fn build(self) -> ComputeReservationResourceStatusElSpecificSkuAllocationEl {
        ComputeReservationResourceStatusElSpecificSkuAllocationEl {
            source_instance_template_id: core::default::Default::default(),
            utilizations: core::default::Default::default(),
        }
    }
}
pub struct ComputeReservationResourceStatusElSpecificSkuAllocationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeReservationResourceStatusElSpecificSkuAllocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeReservationResourceStatusElSpecificSkuAllocationElRef {
        ComputeReservationResourceStatusElSpecificSkuAllocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeReservationResourceStatusElSpecificSkuAllocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `source_instance_template_id` after provisioning.\n"]
    pub fn source_instance_template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_instance_template_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `utilizations` after provisioning.\n"]
    pub fn utilizations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.utilizations", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeReservationResourceStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    health_info: Option<ListField<ComputeReservationResourceStatusElHealthInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation_block_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation_maintenance:
        Option<ListField<ComputeReservationResourceStatusElReservationMaintenanceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_sku_allocation:
        Option<ListField<ComputeReservationResourceStatusElSpecificSkuAllocationEl>>,
}
impl ComputeReservationResourceStatusEl {
    #[doc = "Set the field `health_info`.\n"]
    pub fn set_health_info(
        mut self,
        v: impl Into<ListField<ComputeReservationResourceStatusElHealthInfoEl>>,
    ) -> Self {
        self.health_info = Some(v.into());
        self
    }
    #[doc = "Set the field `reservation_block_count`.\n"]
    pub fn set_reservation_block_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.reservation_block_count = Some(v.into());
        self
    }
    #[doc = "Set the field `reservation_maintenance`.\n"]
    pub fn set_reservation_maintenance(
        mut self,
        v: impl Into<ListField<ComputeReservationResourceStatusElReservationMaintenanceEl>>,
    ) -> Self {
        self.reservation_maintenance = Some(v.into());
        self
    }
    #[doc = "Set the field `specific_sku_allocation`.\n"]
    pub fn set_specific_sku_allocation(
        mut self,
        v: impl Into<ListField<ComputeReservationResourceStatusElSpecificSkuAllocationEl>>,
    ) -> Self {
        self.specific_sku_allocation = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeReservationResourceStatusEl {
    type O = BlockAssignable<ComputeReservationResourceStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeReservationResourceStatusEl {}
impl BuildComputeReservationResourceStatusEl {
    pub fn build(self) -> ComputeReservationResourceStatusEl {
        ComputeReservationResourceStatusEl {
            health_info: core::default::Default::default(),
            reservation_block_count: core::default::Default::default(),
            reservation_maintenance: core::default::Default::default(),
            specific_sku_allocation: core::default::Default::default(),
        }
    }
}
pub struct ComputeReservationResourceStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeReservationResourceStatusElRef {
    fn new(shared: StackShared, base: String) -> ComputeReservationResourceStatusElRef {
        ComputeReservationResourceStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeReservationResourceStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `health_info` after provisioning.\n"]
    pub fn health_info(&self) -> ListRef<ComputeReservationResourceStatusElHealthInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.health_info", self.base))
    }
    #[doc = "Get a reference to the value of field `reservation_block_count` after provisioning.\n"]
    pub fn reservation_block_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reservation_block_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_maintenance` after provisioning.\n"]
    pub fn reservation_maintenance(
        &self,
    ) -> ListRef<ComputeReservationResourceStatusElReservationMaintenanceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_maintenance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `specific_sku_allocation` after provisioning.\n"]
    pub fn specific_sku_allocation(
        &self,
    ) -> ListRef<ComputeReservationResourceStatusElSpecificSkuAllocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.specific_sku_allocation", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeReservationDeleteAfterDurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<String>>,
}
impl ComputeReservationDeleteAfterDurationEl {
    #[doc = "Set the field `nanos`.\nNumber of nanoseconds for the auto-delete duration."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\nNumber of seconds for the auto-delete duration."]
    pub fn set_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeReservationDeleteAfterDurationEl {
    type O = BlockAssignable<ComputeReservationDeleteAfterDurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeReservationDeleteAfterDurationEl {}
impl BuildComputeReservationDeleteAfterDurationEl {
    pub fn build(self) -> ComputeReservationDeleteAfterDurationEl {
        ComputeReservationDeleteAfterDurationEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct ComputeReservationDeleteAfterDurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeReservationDeleteAfterDurationElRef {
    fn new(shared: StackShared, base: String) -> ComputeReservationDeleteAfterDurationElRef {
        ComputeReservationDeleteAfterDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeReservationDeleteAfterDurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nNumber of nanoseconds for the auto-delete duration."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nNumber of seconds for the auto-delete duration."]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeReservationReservationSharingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_share_type: Option<PrimField<String>>,
}
impl ComputeReservationReservationSharingPolicyEl {
    #[doc = "Set the field `service_share_type`.\nSharing config for all Google Cloud services. Possible values: [\"ALLOW_ALL\", \"DISALLOW_ALL\"]"]
    pub fn set_service_share_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_share_type = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeReservationReservationSharingPolicyEl {
    type O = BlockAssignable<ComputeReservationReservationSharingPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeReservationReservationSharingPolicyEl {}
impl BuildComputeReservationReservationSharingPolicyEl {
    pub fn build(self) -> ComputeReservationReservationSharingPolicyEl {
        ComputeReservationReservationSharingPolicyEl {
            service_share_type: core::default::Default::default(),
        }
    }
}
pub struct ComputeReservationReservationSharingPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeReservationReservationSharingPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeReservationReservationSharingPolicyElRef {
        ComputeReservationReservationSharingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeReservationReservationSharingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_share_type` after provisioning.\nSharing config for all Google Cloud services. Possible values: [\"ALLOW_ALL\", \"DISALLOW_ALL\"]"]
    pub fn service_share_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_share_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeReservationShareSettingsElProjectMapEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
}
impl ComputeReservationShareSettingsElProjectMapEl {
    #[doc = "Set the field `project_id`.\nThe project id/number, should be same as the key of this project config in the project map."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeReservationShareSettingsElProjectMapEl {
    type O = BlockAssignable<ComputeReservationShareSettingsElProjectMapEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeReservationShareSettingsElProjectMapEl {
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildComputeReservationShareSettingsElProjectMapEl {
    pub fn build(self) -> ComputeReservationShareSettingsElProjectMapEl {
        ComputeReservationShareSettingsElProjectMapEl {
            id: self.id,
            project_id: core::default::Default::default(),
        }
    }
}
pub struct ComputeReservationShareSettingsElProjectMapElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeReservationShareSettingsElProjectMapElRef {
    fn new(shared: StackShared, base: String) -> ComputeReservationShareSettingsElProjectMapElRef {
        ComputeReservationShareSettingsElProjectMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeReservationShareSettingsElProjectMapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe project id/number, should be same as the key of this project config in the project map."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeReservationShareSettingsElDynamic {
    project_map: Option<DynamicBlock<ComputeReservationShareSettingsElProjectMapEl>>,
}
#[derive(Serialize)]
pub struct ComputeReservationShareSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    share_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_map: Option<Vec<ComputeReservationShareSettingsElProjectMapEl>>,
    dynamic: ComputeReservationShareSettingsElDynamic,
}
impl ComputeReservationShareSettingsEl {
    #[doc = "Set the field `share_type`.\nType of sharing for this shared-reservation Possible values: [\"LOCAL\", \"SPECIFIC_PROJECTS\"]"]
    pub fn set_share_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.share_type = Some(v.into());
        self
    }
    #[doc = "Set the field `project_map`.\n"]
    pub fn set_project_map(
        mut self,
        v: impl Into<BlockAssignable<ComputeReservationShareSettingsElProjectMapEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.project_map = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.project_map = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeReservationShareSettingsEl {
    type O = BlockAssignable<ComputeReservationShareSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeReservationShareSettingsEl {}
impl BuildComputeReservationShareSettingsEl {
    pub fn build(self) -> ComputeReservationShareSettingsEl {
        ComputeReservationShareSettingsEl {
            share_type: core::default::Default::default(),
            project_map: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeReservationShareSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeReservationShareSettingsElRef {
    fn new(shared: StackShared, base: String) -> ComputeReservationShareSettingsElRef {
        ComputeReservationShareSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeReservationShareSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `share_type` after provisioning.\nType of sharing for this shared-reservation Possible values: [\"LOCAL\", \"SPECIFIC_PROJECTS\"]"]
    pub fn share_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.share_type", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
    accelerator_count: PrimField<f64>,
    accelerator_type: PrimField<String>,
}
impl ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {}
impl ToListMappable
    for ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl
{
    type O = BlockAssignable<
        ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
    #[doc = "The number of the guest accelerator cards exposed to\nthis instance."]
    pub accelerator_count: PrimField<f64>,
    #[doc = "The full or partial URL of the accelerator type to\nattach to this instance. For example:\n'projects/my-project/zones/us-central1-c/acceleratorTypes/nvidia-tesla-p100'\n\nIf you are creating an instance template, specify only the accelerator name."]
    pub accelerator_type: PrimField<String>,
}
impl BuildComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
    pub fn build(
        self,
    ) -> ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
        ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
            accelerator_count: self.accelerator_count,
            accelerator_type: self.accelerator_type,
        }
    }
}
pub struct ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
        ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of the guest accelerator cards exposed to\nthis instance."]
    pub fn accelerator_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\nThe full or partial URL of the accelerator type to\nattach to this instance. For example:\n'projects/my-project/zones/us-central1-c/acceleratorTypes/nvidia-tesla-p100'\n\nIf you are creating an instance template, specify only the accelerator name."]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    disk_size_gb: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interface: Option<PrimField<String>>,
}
impl ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    #[doc = "Set the field `interface`.\nThe disk interface to use for attaching this disk. Default value: \"SCSI\" Possible values: [\"SCSI\", \"NVME\"]"]
    pub fn set_interface(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interface = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    type O =
        BlockAssignable<ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    #[doc = "The size of the disk in base-2 GB."]
    pub disk_size_gb: PrimField<f64>,
}
impl BuildComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    pub fn build(self) -> ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
        ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
            disk_size_gb: self.disk_size_gb,
            interface: core::default::Default::default(),
        }
    }
}
pub struct ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
        ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\nThe size of the disk in base-2 GB."]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `interface` after provisioning.\nThe disk interface to use for attaching this disk. Default value: \"SCSI\" Possible values: [\"SCSI\", \"NVME\"]"]
    pub fn interface(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interface", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeReservationSpecificReservationElInstancePropertiesElDynamic {
    guest_accelerators: Option<
        DynamicBlock<
            ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl,
        >,
    >,
    local_ssds: Option<
        DynamicBlock<ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl>,
    >,
}
#[derive(Serialize)]
pub struct ComputeReservationSpecificReservationElInstancePropertiesEl {
    machine_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_cpu_platform: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_accelerators:
        Option<Vec<ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssds: Option<Vec<ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl>>,
    dynamic: ComputeReservationSpecificReservationElInstancePropertiesElDynamic,
}
impl ComputeReservationSpecificReservationElInstancePropertiesEl {
    #[doc = "Set the field `min_cpu_platform`.\nThe minimum CPU platform for the reservation. For example,\n'\"Intel Skylake\"'. See\nthe CPU platform availability reference](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform#availablezones)\nfor information on available CPU platforms."]
    pub fn set_min_cpu_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_cpu_platform = Some(v.into());
        self
    }
    #[doc = "Set the field `guest_accelerators`.\n"]
    pub fn set_guest_accelerators(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.guest_accelerators = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.guest_accelerators = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `local_ssds`.\n"]
    pub fn set_local_ssds(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.local_ssds = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.local_ssds = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeReservationSpecificReservationElInstancePropertiesEl {
    type O = BlockAssignable<ComputeReservationSpecificReservationElInstancePropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeReservationSpecificReservationElInstancePropertiesEl {
    #[doc = "The name of the machine type to reserve."]
    pub machine_type: PrimField<String>,
}
impl BuildComputeReservationSpecificReservationElInstancePropertiesEl {
    pub fn build(self) -> ComputeReservationSpecificReservationElInstancePropertiesEl {
        ComputeReservationSpecificReservationElInstancePropertiesEl {
            machine_type: self.machine_type,
            min_cpu_platform: core::default::Default::default(),
            guest_accelerators: core::default::Default::default(),
            local_ssds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeReservationSpecificReservationElInstancePropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeReservationSpecificReservationElInstancePropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeReservationSpecificReservationElInstancePropertiesElRef {
        ComputeReservationSpecificReservationElInstancePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeReservationSpecificReservationElInstancePropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location_hint` after provisioning.\nAn opaque location hint used to place the allocation close to other resources. This field is for use by internal tools that use the public API."]
    pub fn location_hint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_hint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe name of the machine type to reserve."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `min_cpu_platform` after provisioning.\nThe minimum CPU platform for the reservation. For example,\n'\"Intel Skylake\"'. See\nthe CPU platform availability reference](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform#availablezones)\nfor information on available CPU platforms."]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_cpu_platform", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `guest_accelerators` after provisioning.\n"]
    pub fn guest_accelerators(
        &self,
    ) -> ListRef<ComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guest_accelerators", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_ssds` after provisioning.\n"]
    pub fn local_ssds(
        &self,
    ) -> ListRef<ComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.local_ssds", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeReservationSpecificReservationElDynamic {
    instance_properties:
        Option<DynamicBlock<ComputeReservationSpecificReservationElInstancePropertiesEl>>,
}
#[derive(Serialize)]
pub struct ComputeReservationSpecificReservationEl {
    count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_instance_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_properties: Option<Vec<ComputeReservationSpecificReservationElInstancePropertiesEl>>,
    dynamic: ComputeReservationSpecificReservationElDynamic,
}
impl ComputeReservationSpecificReservationEl {
    #[doc = "Set the field `source_instance_template`.\nSpecifies the instance template to create the reservation. If you use this field, you must exclude the\ninstanceProperties field."]
    pub fn set_source_instance_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_instance_template = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_properties`.\n"]
    pub fn set_instance_properties(
        mut self,
        v: impl Into<BlockAssignable<ComputeReservationSpecificReservationElInstancePropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instance_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_properties = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeReservationSpecificReservationEl {
    type O = BlockAssignable<ComputeReservationSpecificReservationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeReservationSpecificReservationEl {
    #[doc = "The number of resources that are allocated."]
    pub count: PrimField<f64>,
}
impl BuildComputeReservationSpecificReservationEl {
    pub fn build(self) -> ComputeReservationSpecificReservationEl {
        ComputeReservationSpecificReservationEl {
            count: self.count,
            source_instance_template: core::default::Default::default(),
            instance_properties: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeReservationSpecificReservationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeReservationSpecificReservationElRef {
    fn new(shared: StackShared, base: String) -> ComputeReservationSpecificReservationElRef {
        ComputeReservationSpecificReservationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeReservationSpecificReservationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `assured_count` after provisioning.\nIndicates how many instances are actually usable currently."]
    pub fn assured_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assured_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\nThe number of resources that are allocated."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `in_use_count` after provisioning.\nHow many instances are in use."]
    pub fn in_use_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.in_use_count", self.base))
    }
    #[doc = "Get a reference to the value of field `source_instance_template` after provisioning.\nSpecifies the instance template to create the reservation. If you use this field, you must exclude the\ninstanceProperties field."]
    pub fn source_instance_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_instance_template", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_properties` after provisioning.\n"]
    pub fn instance_properties(
        &self,
    ) -> ListRef<ComputeReservationSpecificReservationElInstancePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_properties", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeReservationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeReservationTimeoutsEl {
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
impl ToListMappable for ComputeReservationTimeoutsEl {
    type O = BlockAssignable<ComputeReservationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeReservationTimeoutsEl {}
impl BuildComputeReservationTimeoutsEl {
    pub fn build(self) -> ComputeReservationTimeoutsEl {
        ComputeReservationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeReservationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeReservationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeReservationTimeoutsElRef {
        ComputeReservationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeReservationTimeoutsElRef {
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
struct ComputeReservationDynamic {
    delete_after_duration: Option<DynamicBlock<ComputeReservationDeleteAfterDurationEl>>,
    reservation_sharing_policy: Option<DynamicBlock<ComputeReservationReservationSharingPolicyEl>>,
    share_settings: Option<DynamicBlock<ComputeReservationShareSettingsEl>>,
    specific_reservation: Option<DynamicBlock<ComputeReservationSpecificReservationEl>>,
}
