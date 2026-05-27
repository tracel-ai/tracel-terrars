use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeReservationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    zone: PrimField<String>,
}
struct DataComputeReservation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeReservationData>,
}
#[derive(Clone)]
pub struct DataComputeReservation(Rc<DataComputeReservation_>);
impl DataComputeReservation {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
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
    #[doc = "Get a reference to the value of field `delete_after_duration` after provisioning.\nDuration after which the reservation will be auto-deleted by Compute Engine. Cannot be used with delete_at_time."]
    pub fn delete_after_duration(&self) -> ListRef<DataComputeReservationDeleteAfterDurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delete_after_duration", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `reservation_sharing_policy` after provisioning.\nSharing policy for reservations with Google Cloud managed services."]
    pub fn reservation_sharing_policy(
        &self,
    ) -> ListRef<DataComputeReservationReservationSharingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_sharing_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_status` after provisioning.\nStatus information for Reservation resource."]
    pub fn resource_status(&self) -> ListRef<DataComputeReservationResourceStatusElRef> {
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
    #[doc = "Get a reference to the value of field `share_settings` after provisioning.\nThe share setting for reservations."]
    pub fn share_settings(&self) -> ListRef<DataComputeReservationShareSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.share_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `specific_reservation` after provisioning.\nReservation for instances with specific machine shapes."]
    pub fn specific_reservation(&self) -> ListRef<DataComputeReservationSpecificReservationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.specific_reservation", self.extract_ref()),
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
}
impl Referable for DataComputeReservation {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeReservation {}
impl ToListMappable for DataComputeReservation {
    type O = ListRef<DataComputeReservationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeReservation_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_reservation".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeReservation {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc = "The zone where the reservation is made."]
    pub zone: PrimField<String>,
}
impl BuildDataComputeReservation {
    pub fn build(self, stack: &mut Stack) -> DataComputeReservation {
        let out = DataComputeReservation(Rc::new(DataComputeReservation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeReservationData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                name: self.name,
                project: core::default::Default::default(),
                zone: self.zone,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeReservationRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeReservationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeReservationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
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
    #[doc = "Get a reference to the value of field `delete_after_duration` after provisioning.\nDuration after which the reservation will be auto-deleted by Compute Engine. Cannot be used with delete_at_time."]
    pub fn delete_after_duration(&self) -> ListRef<DataComputeReservationDeleteAfterDurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delete_after_duration", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `reservation_sharing_policy` after provisioning.\nSharing policy for reservations with Google Cloud managed services."]
    pub fn reservation_sharing_policy(
        &self,
    ) -> ListRef<DataComputeReservationReservationSharingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_sharing_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_status` after provisioning.\nStatus information for Reservation resource."]
    pub fn resource_status(&self) -> ListRef<DataComputeReservationResourceStatusElRef> {
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
    #[doc = "Get a reference to the value of field `share_settings` after provisioning.\nThe share setting for reservations."]
    pub fn share_settings(&self) -> ListRef<DataComputeReservationShareSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.share_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `specific_reservation` after provisioning.\nReservation for instances with specific machine shapes."]
    pub fn specific_reservation(&self) -> ListRef<DataComputeReservationSpecificReservationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.specific_reservation", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct DataComputeReservationDeleteAfterDurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<String>>,
}
impl DataComputeReservationDeleteAfterDurationEl {
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeReservationDeleteAfterDurationEl {
    type O = BlockAssignable<DataComputeReservationDeleteAfterDurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeReservationDeleteAfterDurationEl {}
impl BuildDataComputeReservationDeleteAfterDurationEl {
    pub fn build(self) -> DataComputeReservationDeleteAfterDurationEl {
        DataComputeReservationDeleteAfterDurationEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataComputeReservationDeleteAfterDurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeReservationDeleteAfterDurationElRef {
    fn new(shared: StackShared, base: String) -> DataComputeReservationDeleteAfterDurationElRef {
        DataComputeReservationDeleteAfterDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeReservationDeleteAfterDurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeReservationReservationSharingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_share_type: Option<PrimField<String>>,
}
impl DataComputeReservationReservationSharingPolicyEl {
    #[doc = "Set the field `service_share_type`.\n"]
    pub fn set_service_share_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_share_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeReservationReservationSharingPolicyEl {
    type O = BlockAssignable<DataComputeReservationReservationSharingPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeReservationReservationSharingPolicyEl {}
impl BuildDataComputeReservationReservationSharingPolicyEl {
    pub fn build(self) -> DataComputeReservationReservationSharingPolicyEl {
        DataComputeReservationReservationSharingPolicyEl {
            service_share_type: core::default::Default::default(),
        }
    }
}
pub struct DataComputeReservationReservationSharingPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeReservationReservationSharingPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeReservationReservationSharingPolicyElRef {
        DataComputeReservationReservationSharingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeReservationReservationSharingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_share_type` after provisioning.\n"]
    pub fn service_share_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_share_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeReservationResourceStatusElHealthInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    degraded_block_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    healthy_block_count: Option<PrimField<f64>>,
}
impl DataComputeReservationResourceStatusElHealthInfoEl {
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
impl ToListMappable for DataComputeReservationResourceStatusElHealthInfoEl {
    type O = BlockAssignable<DataComputeReservationResourceStatusElHealthInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeReservationResourceStatusElHealthInfoEl {}
impl BuildDataComputeReservationResourceStatusElHealthInfoEl {
    pub fn build(self) -> DataComputeReservationResourceStatusElHealthInfoEl {
        DataComputeReservationResourceStatusElHealthInfoEl {
            degraded_block_count: core::default::Default::default(),
            health_status: core::default::Default::default(),
            healthy_block_count: core::default::Default::default(),
        }
    }
}
pub struct DataComputeReservationResourceStatusElHealthInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeReservationResourceStatusElHealthInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeReservationResourceStatusElHealthInfoElRef {
        DataComputeReservationResourceStatusElHealthInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeReservationResourceStatusElHealthInfoElRef {
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
pub struct DataComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl
{
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
impl DataComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl {
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
    for DataComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl
{
    type O = BlockAssignable<
        DataComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl
{}
impl BuildDataComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl {
    pub fn build(
        self,
    ) -> DataComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl
    {
        DataComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl {
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
pub struct DataComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceElRef
    {
        DataComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceElRef {
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
pub struct DataComputeReservationResourceStatusElReservationMaintenanceEl { # [serde (skip_serializing_if = "Option::is_none")] instance_maintenance_ongoing_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] instance_maintenance_pending_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] maintenance_ongoing_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] maintenance_pending_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] scheduling_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] subblock_infra_maintenance_ongoing_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] subblock_infra_maintenance_pending_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] upcoming_group_maintenance : Option < ListField < DataComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl > > , }
impl DataComputeReservationResourceStatusElReservationMaintenanceEl {
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
        v : impl Into < ListField < DataComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceEl > >,
    ) -> Self {
        self.upcoming_group_maintenance = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeReservationResourceStatusElReservationMaintenanceEl {
    type O = BlockAssignable<DataComputeReservationResourceStatusElReservationMaintenanceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeReservationResourceStatusElReservationMaintenanceEl {}
impl BuildDataComputeReservationResourceStatusElReservationMaintenanceEl {
    pub fn build(self) -> DataComputeReservationResourceStatusElReservationMaintenanceEl {
        DataComputeReservationResourceStatusElReservationMaintenanceEl {
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
pub struct DataComputeReservationResourceStatusElReservationMaintenanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeReservationResourceStatusElReservationMaintenanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeReservationResourceStatusElReservationMaintenanceElRef {
        DataComputeReservationResourceStatusElReservationMaintenanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeReservationResourceStatusElReservationMaintenanceElRef {
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
        DataComputeReservationResourceStatusElReservationMaintenanceElUpcomingGroupMaintenanceElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upcoming_group_maintenance", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeReservationResourceStatusElSpecificSkuAllocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_instance_template_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    utilizations: Option<RecField<PrimField<String>>>,
}
impl DataComputeReservationResourceStatusElSpecificSkuAllocationEl {
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
impl ToListMappable for DataComputeReservationResourceStatusElSpecificSkuAllocationEl {
    type O = BlockAssignable<DataComputeReservationResourceStatusElSpecificSkuAllocationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeReservationResourceStatusElSpecificSkuAllocationEl {}
impl BuildDataComputeReservationResourceStatusElSpecificSkuAllocationEl {
    pub fn build(self) -> DataComputeReservationResourceStatusElSpecificSkuAllocationEl {
        DataComputeReservationResourceStatusElSpecificSkuAllocationEl {
            source_instance_template_id: core::default::Default::default(),
            utilizations: core::default::Default::default(),
        }
    }
}
pub struct DataComputeReservationResourceStatusElSpecificSkuAllocationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeReservationResourceStatusElSpecificSkuAllocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeReservationResourceStatusElSpecificSkuAllocationElRef {
        DataComputeReservationResourceStatusElSpecificSkuAllocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeReservationResourceStatusElSpecificSkuAllocationElRef {
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
pub struct DataComputeReservationResourceStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    health_info: Option<ListField<DataComputeReservationResourceStatusElHealthInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation_block_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation_maintenance:
        Option<ListField<DataComputeReservationResourceStatusElReservationMaintenanceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_sku_allocation:
        Option<ListField<DataComputeReservationResourceStatusElSpecificSkuAllocationEl>>,
}
impl DataComputeReservationResourceStatusEl {
    #[doc = "Set the field `health_info`.\n"]
    pub fn set_health_info(
        mut self,
        v: impl Into<ListField<DataComputeReservationResourceStatusElHealthInfoEl>>,
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
        v: impl Into<ListField<DataComputeReservationResourceStatusElReservationMaintenanceEl>>,
    ) -> Self {
        self.reservation_maintenance = Some(v.into());
        self
    }
    #[doc = "Set the field `specific_sku_allocation`.\n"]
    pub fn set_specific_sku_allocation(
        mut self,
        v: impl Into<ListField<DataComputeReservationResourceStatusElSpecificSkuAllocationEl>>,
    ) -> Self {
        self.specific_sku_allocation = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeReservationResourceStatusEl {
    type O = BlockAssignable<DataComputeReservationResourceStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeReservationResourceStatusEl {}
impl BuildDataComputeReservationResourceStatusEl {
    pub fn build(self) -> DataComputeReservationResourceStatusEl {
        DataComputeReservationResourceStatusEl {
            health_info: core::default::Default::default(),
            reservation_block_count: core::default::Default::default(),
            reservation_maintenance: core::default::Default::default(),
            specific_sku_allocation: core::default::Default::default(),
        }
    }
}
pub struct DataComputeReservationResourceStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeReservationResourceStatusElRef {
    fn new(shared: StackShared, base: String) -> DataComputeReservationResourceStatusElRef {
        DataComputeReservationResourceStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeReservationResourceStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `health_info` after provisioning.\n"]
    pub fn health_info(&self) -> ListRef<DataComputeReservationResourceStatusElHealthInfoElRef> {
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
    ) -> ListRef<DataComputeReservationResourceStatusElReservationMaintenanceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_maintenance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `specific_sku_allocation` after provisioning.\n"]
    pub fn specific_sku_allocation(
        &self,
    ) -> ListRef<DataComputeReservationResourceStatusElSpecificSkuAllocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.specific_sku_allocation", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeReservationShareSettingsElProjectMapEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
}
impl DataComputeReservationShareSettingsElProjectMapEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeReservationShareSettingsElProjectMapEl {
    type O = BlockAssignable<DataComputeReservationShareSettingsElProjectMapEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeReservationShareSettingsElProjectMapEl {}
impl BuildDataComputeReservationShareSettingsElProjectMapEl {
    pub fn build(self) -> DataComputeReservationShareSettingsElProjectMapEl {
        DataComputeReservationShareSettingsElProjectMapEl {
            id: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}
pub struct DataComputeReservationShareSettingsElProjectMapElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeReservationShareSettingsElProjectMapElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeReservationShareSettingsElProjectMapElRef {
        DataComputeReservationShareSettingsElProjectMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeReservationShareSettingsElProjectMapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeReservationShareSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    project_map: Option<SetField<DataComputeReservationShareSettingsElProjectMapEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_type: Option<PrimField<String>>,
}
impl DataComputeReservationShareSettingsEl {
    #[doc = "Set the field `project_map`.\n"]
    pub fn set_project_map(
        mut self,
        v: impl Into<SetField<DataComputeReservationShareSettingsElProjectMapEl>>,
    ) -> Self {
        self.project_map = Some(v.into());
        self
    }
    #[doc = "Set the field `share_type`.\n"]
    pub fn set_share_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.share_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeReservationShareSettingsEl {
    type O = BlockAssignable<DataComputeReservationShareSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeReservationShareSettingsEl {}
impl BuildDataComputeReservationShareSettingsEl {
    pub fn build(self) -> DataComputeReservationShareSettingsEl {
        DataComputeReservationShareSettingsEl {
            project_map: core::default::Default::default(),
            share_type: core::default::Default::default(),
        }
    }
}
pub struct DataComputeReservationShareSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeReservationShareSettingsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeReservationShareSettingsElRef {
        DataComputeReservationShareSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeReservationShareSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `project_map` after provisioning.\n"]
    pub fn project_map(&self) -> SetRef<DataComputeReservationShareSettingsElProjectMapElRef> {
        SetRef::new(self.shared().clone(), format!("{}.project_map", self.base))
    }
    #[doc = "Get a reference to the value of field `share_type` after provisioning.\n"]
    pub fn share_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.share_type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_type: Option<PrimField<String>>,
}
impl DataComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
    #[doc = "Set the field `accelerator_count`.\n"]
    pub fn set_accelerator_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.accelerator_count = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerator_type`.\n"]
    pub fn set_accelerator_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator_type = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl
{
    type O = BlockAssignable<
        DataComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl
{}
impl BuildDataComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
    pub fn build(
        self,
    ) -> DataComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
        DataComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
        }
    }
}
pub struct DataComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
        DataComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\n"]
    pub fn accelerator_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\n"]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interface: Option<PrimField<String>>,
}
impl DataComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    #[doc = "Set the field `disk_size_gb`.\n"]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `interface`.\n"]
    pub fn set_interface(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interface = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    type O =
        BlockAssignable<DataComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {}
impl BuildDataComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
    pub fn build(
        self,
    ) -> DataComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
        DataComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl {
            disk_size_gb: core::default::Default::default(),
            interface: core::default::Default::default(),
        }
    }
}
pub struct DataComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
        DataComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\n"]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `interface` after provisioning.\n"]
    pub fn interface(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interface", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeReservationSpecificReservationElInstancePropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_accelerators: Option<
        ListField<
            DataComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssds: Option<
        ListField<DataComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_hint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_cpu_platform: Option<PrimField<String>>,
}
impl DataComputeReservationSpecificReservationElInstancePropertiesEl {
    #[doc = "Set the field `guest_accelerators`.\n"]
    pub fn set_guest_accelerators(
        mut self,
        v: impl Into<
            ListField<
                DataComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsEl,
            >,
        >,
    ) -> Self {
        self.guest_accelerators = Some(v.into());
        self
    }
    #[doc = "Set the field `local_ssds`.\n"]
    pub fn set_local_ssds(
        mut self,
        v: impl Into<
            ListField<DataComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsEl>,
        >,
    ) -> Self {
        self.local_ssds = Some(v.into());
        self
    }
    #[doc = "Set the field `location_hint`.\n"]
    pub fn set_location_hint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location_hint = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `min_cpu_platform`.\n"]
    pub fn set_min_cpu_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_cpu_platform = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeReservationSpecificReservationElInstancePropertiesEl {
    type O = BlockAssignable<DataComputeReservationSpecificReservationElInstancePropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeReservationSpecificReservationElInstancePropertiesEl {}
impl BuildDataComputeReservationSpecificReservationElInstancePropertiesEl {
    pub fn build(self) -> DataComputeReservationSpecificReservationElInstancePropertiesEl {
        DataComputeReservationSpecificReservationElInstancePropertiesEl {
            guest_accelerators: core::default::Default::default(),
            local_ssds: core::default::Default::default(),
            location_hint: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
        }
    }
}
pub struct DataComputeReservationSpecificReservationElInstancePropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeReservationSpecificReservationElInstancePropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeReservationSpecificReservationElInstancePropertiesElRef {
        DataComputeReservationSpecificReservationElInstancePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeReservationSpecificReservationElInstancePropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `guest_accelerators` after provisioning.\n"]
    pub fn guest_accelerators(
        &self,
    ) -> ListRef<
        DataComputeReservationSpecificReservationElInstancePropertiesElGuestAcceleratorsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guest_accelerators", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_ssds` after provisioning.\n"]
    pub fn local_ssds(
        &self,
    ) -> ListRef<DataComputeReservationSpecificReservationElInstancePropertiesElLocalSsdsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.local_ssds", self.base))
    }
    #[doc = "Get a reference to the value of field `location_hint` after provisioning.\n"]
    pub fn location_hint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_hint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `min_cpu_platform` after provisioning.\n"]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_cpu_platform", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeReservationSpecificReservationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    assured_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_use_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_properties:
        Option<ListField<DataComputeReservationSpecificReservationElInstancePropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_instance_template: Option<PrimField<String>>,
}
impl DataComputeReservationSpecificReservationEl {
    #[doc = "Set the field `assured_count`.\n"]
    pub fn set_assured_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.assured_count = Some(v.into());
        self
    }
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `in_use_count`.\n"]
    pub fn set_in_use_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.in_use_count = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_properties`.\n"]
    pub fn set_instance_properties(
        mut self,
        v: impl Into<ListField<DataComputeReservationSpecificReservationElInstancePropertiesEl>>,
    ) -> Self {
        self.instance_properties = Some(v.into());
        self
    }
    #[doc = "Set the field `source_instance_template`.\n"]
    pub fn set_source_instance_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_instance_template = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeReservationSpecificReservationEl {
    type O = BlockAssignable<DataComputeReservationSpecificReservationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeReservationSpecificReservationEl {}
impl BuildDataComputeReservationSpecificReservationEl {
    pub fn build(self) -> DataComputeReservationSpecificReservationEl {
        DataComputeReservationSpecificReservationEl {
            assured_count: core::default::Default::default(),
            count: core::default::Default::default(),
            in_use_count: core::default::Default::default(),
            instance_properties: core::default::Default::default(),
            source_instance_template: core::default::Default::default(),
        }
    }
}
pub struct DataComputeReservationSpecificReservationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeReservationSpecificReservationElRef {
    fn new(shared: StackShared, base: String) -> DataComputeReservationSpecificReservationElRef {
        DataComputeReservationSpecificReservationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeReservationSpecificReservationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `assured_count` after provisioning.\n"]
    pub fn assured_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assured_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `in_use_count` after provisioning.\n"]
    pub fn in_use_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.in_use_count", self.base))
    }
    #[doc = "Get a reference to the value of field `instance_properties` after provisioning.\n"]
    pub fn instance_properties(
        &self,
    ) -> ListRef<DataComputeReservationSpecificReservationElInstancePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_instance_template` after provisioning.\n"]
    pub fn source_instance_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_instance_template", self.base),
        )
    }
}
