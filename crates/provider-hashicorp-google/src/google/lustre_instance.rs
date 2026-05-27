use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct LustreInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    capacity_gib: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    filesystem: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_support_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_unit_storage_throughput: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_rules_options: Option<Vec<LustreInstanceAccessRulesOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynamic_tier_options: Option<Vec<LustreInstanceDynamicTierOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_policy: Option<Vec<LustreInstanceMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<LustreInstanceTimeoutsEl>,
    dynamic: LustreInstanceDynamic,
}
struct LustreInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LustreInstanceData>,
}
#[derive(Clone)]
pub struct LustreInstance(Rc<LustreInstance_>);
impl LustreInstance {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA user-readable description of the instance."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `gke_support_enabled`.\nIndicates whether you want to enable support for GKE clients. By default,\nGKE clients are not supported."]
    pub fn set_gke_support_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().gke_support_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key`.\nThe Cloud KMS key name to use for data encryption.\nIf not set, the instance will use Google-managed encryption keys.\nIf set, the instance will use customer-managed encryption keys.\nThe key must be in the same region as the instance.\nThe key format is:\nprojects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{key}"]
    pub fn set_kms_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels as key value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `per_unit_storage_throughput`.\nThe throughput of the instance in MBps per TiB. Valid values are 125, 250,\n500, 1000.\nSee [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor more information.\n\nIf the instance is using the Dynamic tier, this field must not be set or\nmust be set to zero."]
    pub fn set_per_unit_storage_throughput(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().per_unit_storage_throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `placement_policy`.\nThe placement policy name for the instance in the format of\nprojects/{project}/locations/{location}/resourcePolicies/{resource_policy}"]
    pub fn set_placement_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().placement_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `access_rules_options`.\n"]
    pub fn set_access_rules_options(
        self,
        v: impl Into<BlockAssignable<LustreInstanceAccessRulesOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_rules_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_rules_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dynamic_tier_options`.\n"]
    pub fn set_dynamic_tier_options(
        self,
        v: impl Into<BlockAssignable<LustreInstanceDynamicTierOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dynamic_tier_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dynamic_tier_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `maintenance_policy`.\n"]
    pub fn set_maintenance_policy(
        self,
        v: impl Into<BlockAssignable<LustreInstanceMaintenancePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<LustreInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `capacity_gib` after provisioning.\nThe storage capacity of the instance in gibibytes (GiB). Allowed values\nare from '9000' to '7632000', depending on the 'perUnitStorageThroughput'.\nSee [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor specific minimums, maximums, and step sizes for each performance tier."]
    pub fn capacity_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the instance was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA user-readable description of the instance."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filesystem` after provisioning.\nThe filesystem name for this instance. This name is used by client-side\ntools, including when mounting the instance. Must be eight characters or\nless and can only contain letters and numbers."]
    pub fn filesystem(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filesystem", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gke_support_enabled` after provisioning.\nIndicates whether you want to enable support for GKE clients. By default,\nGKE clients are not supported."]
    pub fn gke_support_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gke_support_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe name of the Managed Lustre instance.\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter."]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS key name to use for data encryption.\nIf not set, the instance will use Google-managed encryption keys.\nIf set, the instance will use customer-managed encryption keys.\nThe key must be in the same region as the instance.\nThe key format is:\nprojects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{key}"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mount_point` after provisioning.\nMount point of the instance in the format 'IP_ADDRESS@tcp:/FILESYSTEM'."]
    pub fn mount_point(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mount_point", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe full name of the VPC network to which the instance is connected.\nMust be in the format\n'projects/{project_id}/global/networks/{network_name}'."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `per_unit_storage_throughput` after provisioning.\nThe throughput of the instance in MBps per TiB. Valid values are 125, 250,\n500, 1000.\nSee [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor more information.\n\nIf the instance is using the Dynamic tier, this field must not be set or\nmust be set to zero."]
    pub fn per_unit_storage_throughput(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.per_unit_storage_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `placement_policy` after provisioning.\nThe placement policy name for the instance in the format of\nprojects/{project}/locations/{location}/resourcePolicies/{resource_policy}"]
    pub fn placement_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.placement_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the instance.\nPossible values:\nACTIVE\nCREATING\nDELETING\nUPGRADING\nREPAIRING\nSTOPPED\nUPDATING\nSUSPENDED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_reason` after provisioning.\nThe reason why the instance is in a certain state (e.g. SUSPENDED)."]
    pub fn state_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique ID of the resource.\nThis is unrelated to the access rules which allow specifying the root\nsquash uid."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `upcoming_maintenance_schedule` after provisioning.\nRepresents a scheduled maintenance event."]
    pub fn upcoming_maintenance_schedule(
        &self,
    ) -> ListRef<LustreInstanceUpcomingMaintenanceScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upcoming_maintenance_schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the instance was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_rules_options` after provisioning.\n"]
    pub fn access_rules_options(&self) -> ListRef<LustreInstanceAccessRulesOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_rules_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dynamic_tier_options` after provisioning.\n"]
    pub fn dynamic_tier_options(&self) -> ListRef<LustreInstanceDynamicTierOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dynamic_tier_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> ListRef<LustreInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LustreInstanceTimeoutsElRef {
        LustreInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for LustreInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for LustreInstance {}
impl ToListMappable for LustreInstance {
    type O = ListRef<LustreInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for LustreInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_lustre_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLustreInstance {
    pub tf_id: String,
    #[doc = "The storage capacity of the instance in gibibytes (GiB). Allowed values\nare from '9000' to '7632000', depending on the 'perUnitStorageThroughput'.\nSee [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor specific minimums, maximums, and step sizes for each performance tier."]
    pub capacity_gib: PrimField<String>,
    #[doc = "The filesystem name for this instance. This name is used by client-side\ntools, including when mounting the instance. Must be eight characters or\nless and can only contain letters and numbers."]
    pub filesystem: PrimField<String>,
    #[doc = "The name of the Managed Lustre instance.\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter."]
    pub instance_id: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "The full name of the VPC network to which the instance is connected.\nMust be in the format\n'projects/{project_id}/global/networks/{network_name}'."]
    pub network: PrimField<String>,
}
impl BuildLustreInstance {
    pub fn build(self, stack: &mut Stack) -> LustreInstance {
        let out = LustreInstance(Rc::new(LustreInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LustreInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                capacity_gib: self.capacity_gib,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                filesystem: self.filesystem,
                gke_support_enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                kms_key: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                network: self.network,
                per_unit_storage_throughput: core::default::Default::default(),
                placement_policy: core::default::Default::default(),
                project: core::default::Default::default(),
                access_rules_options: core::default::Default::default(),
                dynamic_tier_options: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct LustreInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for LustreInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl LustreInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `capacity_gib` after provisioning.\nThe storage capacity of the instance in gibibytes (GiB). Allowed values\nare from '9000' to '7632000', depending on the 'perUnitStorageThroughput'.\nSee [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor specific minimums, maximums, and step sizes for each performance tier."]
    pub fn capacity_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the instance was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA user-readable description of the instance."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filesystem` after provisioning.\nThe filesystem name for this instance. This name is used by client-side\ntools, including when mounting the instance. Must be eight characters or\nless and can only contain letters and numbers."]
    pub fn filesystem(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filesystem", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gke_support_enabled` after provisioning.\nIndicates whether you want to enable support for GKE clients. By default,\nGKE clients are not supported."]
    pub fn gke_support_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gke_support_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe name of the Managed Lustre instance.\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter."]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS key name to use for data encryption.\nIf not set, the instance will use Google-managed encryption keys.\nIf set, the instance will use customer-managed encryption keys.\nThe key must be in the same region as the instance.\nThe key format is:\nprojects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{key}"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mount_point` after provisioning.\nMount point of the instance in the format 'IP_ADDRESS@tcp:/FILESYSTEM'."]
    pub fn mount_point(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mount_point", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe full name of the VPC network to which the instance is connected.\nMust be in the format\n'projects/{project_id}/global/networks/{network_name}'."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `per_unit_storage_throughput` after provisioning.\nThe throughput of the instance in MBps per TiB. Valid values are 125, 250,\n500, 1000.\nSee [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor more information.\n\nIf the instance is using the Dynamic tier, this field must not be set or\nmust be set to zero."]
    pub fn per_unit_storage_throughput(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.per_unit_storage_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `placement_policy` after provisioning.\nThe placement policy name for the instance in the format of\nprojects/{project}/locations/{location}/resourcePolicies/{resource_policy}"]
    pub fn placement_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.placement_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the instance.\nPossible values:\nACTIVE\nCREATING\nDELETING\nUPGRADING\nREPAIRING\nSTOPPED\nUPDATING\nSUSPENDED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_reason` after provisioning.\nThe reason why the instance is in a certain state (e.g. SUSPENDED)."]
    pub fn state_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique ID of the resource.\nThis is unrelated to the access rules which allow specifying the root\nsquash uid."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `upcoming_maintenance_schedule` after provisioning.\nRepresents a scheduled maintenance event."]
    pub fn upcoming_maintenance_schedule(
        &self,
    ) -> ListRef<LustreInstanceUpcomingMaintenanceScheduleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upcoming_maintenance_schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the instance was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_rules_options` after provisioning.\n"]
    pub fn access_rules_options(&self) -> ListRef<LustreInstanceAccessRulesOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_rules_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dynamic_tier_options` after provisioning.\n"]
    pub fn dynamic_tier_options(&self) -> ListRef<LustreInstanceDynamicTierOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dynamic_tier_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> ListRef<LustreInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LustreInstanceTimeoutsElRef {
        LustreInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct LustreInstanceUpcomingMaintenanceScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl LustreInstanceUpcomingMaintenanceScheduleEl {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for LustreInstanceUpcomingMaintenanceScheduleEl {
    type O = BlockAssignable<LustreInstanceUpcomingMaintenanceScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLustreInstanceUpcomingMaintenanceScheduleEl {}
impl BuildLustreInstanceUpcomingMaintenanceScheduleEl {
    pub fn build(self) -> LustreInstanceUpcomingMaintenanceScheduleEl {
        LustreInstanceUpcomingMaintenanceScheduleEl {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct LustreInstanceUpcomingMaintenanceScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LustreInstanceUpcomingMaintenanceScheduleElRef {
    fn new(shared: StackShared, base: String) -> LustreInstanceUpcomingMaintenanceScheduleElRef {
        LustreInstanceUpcomingMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LustreInstanceUpcomingMaintenanceScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct LustreInstanceAccessRulesOptionsElAccessRulesEl {
    ip_address_ranges: ListField<PrimField<String>>,
    name: PrimField<String>,
    squash_mode: PrimField<String>,
}
impl LustreInstanceAccessRulesOptionsElAccessRulesEl {}
impl ToListMappable for LustreInstanceAccessRulesOptionsElAccessRulesEl {
    type O = BlockAssignable<LustreInstanceAccessRulesOptionsElAccessRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLustreInstanceAccessRulesOptionsElAccessRulesEl {
    #[doc = "The IP address ranges to which to apply this access rule. Accepts\nnon-overlapping CIDR ranges (e.g., '192.168.1.0/24') and IP addresses\n(e.g., '192.168.1.0')."]
    pub ip_address_ranges: ListField<PrimField<String>>,
    #[doc = "The name of the access rule policy group.\nMust be 16 characters or less and include only alphanumeric characters\nor '_'."]
    pub name: PrimField<String>,
    #[doc = "Squash mode for the access rule.\nPossible values:\nNO_SQUASH\nROOT_SQUASH"]
    pub squash_mode: PrimField<String>,
}
impl BuildLustreInstanceAccessRulesOptionsElAccessRulesEl {
    pub fn build(self) -> LustreInstanceAccessRulesOptionsElAccessRulesEl {
        LustreInstanceAccessRulesOptionsElAccessRulesEl {
            ip_address_ranges: self.ip_address_ranges,
            name: self.name,
            squash_mode: self.squash_mode,
        }
    }
}
pub struct LustreInstanceAccessRulesOptionsElAccessRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LustreInstanceAccessRulesOptionsElAccessRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LustreInstanceAccessRulesOptionsElAccessRulesElRef {
        LustreInstanceAccessRulesOptionsElAccessRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LustreInstanceAccessRulesOptionsElAccessRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_address_ranges` after provisioning.\nThe IP address ranges to which to apply this access rule. Accepts\nnon-overlapping CIDR ranges (e.g., '192.168.1.0/24') and IP addresses\n(e.g., '192.168.1.0')."]
    pub fn ip_address_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_address_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the access rule policy group.\nMust be 16 characters or less and include only alphanumeric characters\nor '_'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `squash_mode` after provisioning.\nSquash mode for the access rule.\nPossible values:\nNO_SQUASH\nROOT_SQUASH"]
    pub fn squash_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_mode", self.base))
    }
}
#[derive(Serialize, Default)]
struct LustreInstanceAccessRulesOptionsElDynamic {
    access_rules: Option<DynamicBlock<LustreInstanceAccessRulesOptionsElAccessRulesEl>>,
}
#[derive(Serialize)]
pub struct LustreInstanceAccessRulesOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_squash_gid: Option<PrimField<f64>>,
    default_squash_mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_squash_uid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_rules: Option<Vec<LustreInstanceAccessRulesOptionsElAccessRulesEl>>,
    dynamic: LustreInstanceAccessRulesOptionsElDynamic,
}
impl LustreInstanceAccessRulesOptionsEl {
    #[doc = "Set the field `default_squash_gid`.\nThe user squash GID for the default access rule.\nThis user squash GID applies to all root users connecting from clients\nthat are not matched by any of the access rules. If not set, the default\nis 0 (no GID squash)."]
    pub fn set_default_squash_gid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_squash_gid = Some(v.into());
        self
    }
    #[doc = "Set the field `default_squash_uid`.\nThe user squash UID for the default access rule.\nThis user squash UID applies to all root users connecting from clients\nthat are not matched by any of the access rules. If not set, the default\nis 0 (no UID squash)."]
    pub fn set_default_squash_uid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_squash_uid = Some(v.into());
        self
    }
    #[doc = "Set the field `access_rules`.\n"]
    pub fn set_access_rules(
        mut self,
        v: impl Into<BlockAssignable<LustreInstanceAccessRulesOptionsElAccessRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_rules = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LustreInstanceAccessRulesOptionsEl {
    type O = BlockAssignable<LustreInstanceAccessRulesOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLustreInstanceAccessRulesOptionsEl {
    #[doc = "The squash mode for the default access rule.\nPossible values:\nNO_SQUASH\nROOT_SQUASH"]
    pub default_squash_mode: PrimField<String>,
}
impl BuildLustreInstanceAccessRulesOptionsEl {
    pub fn build(self) -> LustreInstanceAccessRulesOptionsEl {
        LustreInstanceAccessRulesOptionsEl {
            default_squash_gid: core::default::Default::default(),
            default_squash_mode: self.default_squash_mode,
            default_squash_uid: core::default::Default::default(),
            access_rules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LustreInstanceAccessRulesOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LustreInstanceAccessRulesOptionsElRef {
    fn new(shared: StackShared, base: String) -> LustreInstanceAccessRulesOptionsElRef {
        LustreInstanceAccessRulesOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LustreInstanceAccessRulesOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_squash_gid` after provisioning.\nThe user squash GID for the default access rule.\nThis user squash GID applies to all root users connecting from clients\nthat are not matched by any of the access rules. If not set, the default\nis 0 (no GID squash)."]
    pub fn default_squash_gid(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_squash_gid", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_squash_mode` after provisioning.\nThe squash mode for the default access rule.\nPossible values:\nNO_SQUASH\nROOT_SQUASH"]
    pub fn default_squash_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_squash_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_squash_uid` after provisioning.\nThe user squash UID for the default access rule.\nThis user squash UID applies to all root users connecting from clients\nthat are not matched by any of the access rules. If not set, the default\nis 0 (no UID squash)."]
    pub fn default_squash_uid(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_squash_uid", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `access_rules` after provisioning.\n"]
    pub fn access_rules(&self) -> ListRef<LustreInstanceAccessRulesOptionsElAccessRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_rules", self.base))
    }
}
#[derive(Serialize)]
pub struct LustreInstanceDynamicTierOptionsEl {
    mode: PrimField<String>,
}
impl LustreInstanceDynamicTierOptionsEl {}
impl ToListMappable for LustreInstanceDynamicTierOptionsEl {
    type O = BlockAssignable<LustreInstanceDynamicTierOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLustreInstanceDynamicTierOptionsEl {
    #[doc = "The dynamic tier mode of the instance.\nPossible values:\nDISABLED\nDEFAULT_CACHE"]
    pub mode: PrimField<String>,
}
impl BuildLustreInstanceDynamicTierOptionsEl {
    pub fn build(self) -> LustreInstanceDynamicTierOptionsEl {
        LustreInstanceDynamicTierOptionsEl { mode: self.mode }
    }
}
pub struct LustreInstanceDynamicTierOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LustreInstanceDynamicTierOptionsElRef {
    fn new(shared: StackShared, base: String) -> LustreInstanceDynamicTierOptionsElRef {
        LustreInstanceDynamicTierOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LustreInstanceDynamicTierOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nThe dynamic tier mode of the instance.\nPossible values:\nDISABLED\nDEFAULT_CACHE"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<PrimField<f64>>,
}
impl LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl {
    #[doc = "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't\nsignificant."]
    pub fn set_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.day = Some(v.into());
        self
    }
    #[doc = "Set the field `month`.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn set_month(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.month = Some(v.into());
        self
    }
    #[doc = "Set the field `year`.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn set_year(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.year = Some(v.into());
        self
    }
}
impl ToListMappable for LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl {
    type O =
        BlockAssignable<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl {}
impl BuildLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl {
    pub fn build(self) -> LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl {
        LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}
pub struct LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateElRef {
        LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't\nsignificant."]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }
    #[doc = "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}
#[derive(Serialize)]
pub struct LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<PrimField<f64>>,
}
impl LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl {
    #[doc = "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't\nsignificant."]
    pub fn set_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.day = Some(v.into());
        self
    }
    #[doc = "Set the field `month`.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn set_month(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.month = Some(v.into());
        self
    }
    #[doc = "Set the field `year`.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn set_year(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.year = Some(v.into());
        self
    }
}
impl ToListMappable for LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl {
    type O =
        BlockAssignable<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl {}
impl BuildLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl {
    pub fn build(self) -> LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl {
        LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}
pub struct LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateElRef {
        LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't\nsignificant."]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }
    #[doc = "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}
#[derive(Serialize)]
pub struct LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl {
    #[doc = "Set the field `hours`.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and\ntypically must be less than or equal to 23. An API may choose to allow the\nvalue \"24:00:00\" for scenarios like business closing time."]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\nMinutes of an hour. Must be greater than or equal to 0 and less than or\nequal to 59."]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `nanos`.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0\nand less than or equal to 999,999,999."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\nSeconds of a minute. Must be greater than or equal to 0 and typically must\nbe less than or equal to 59. An API may allow the value 60 if it allows\nleap-seconds."]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl {
    type O = BlockAssignable<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl {}
impl BuildLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl {
    pub fn build(self) -> LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl {
        LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeElRef {
        LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and\ntypically must be less than or equal to 23. An API may choose to allow the\nvalue \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\nMinutes of an hour. Must be greater than or equal to 0 and less than or\nequal to 59."]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0\nand less than or equal to 999,999,999."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSeconds of a minute. Must be greater than or equal to 0 and typically must\nbe less than or equal to 59. An API may allow the value 60 if it allows\nleap-seconds."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize, Default)]
struct LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElDynamic {
    end_date: Option<
        DynamicBlock<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl>,
    >,
    start_date: Option<
        DynamicBlock<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl>,
    >,
    time: Option<DynamicBlock<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl>>,
}
#[derive(Serialize)]
pub struct LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<Vec<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date:
        Option<Vec<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<Vec<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl>>,
    dynamic: LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElDynamic,
}
impl LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl {
    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(
        mut self,
        v: impl Into<
            BlockAssignable<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.end_date = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.end_date = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(
        mut self,
        v: impl Into<
            BlockAssignable<
                LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.start_date = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.start_date = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `time`.\n"]
    pub fn set_time(
        mut self,
        v: impl Into<
            BlockAssignable<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.time = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl {
    type O = BlockAssignable<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl {}
impl BuildLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl {
    pub fn build(self) -> LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl {
        LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl {
            end_date: core::default::Default::default(),
            start_date: core::default::Default::default(),
            time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElRef {
        LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(
        &self,
    ) -> ListRef<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.end_date", self.base))
    }
    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(
        &self,
    ) -> ListRef<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.start_date", self.base))
    }
    #[doc = "Get a reference to the value of field `time` after provisioning.\n"]
    pub fn time(
        &self,
    ) -> ListRef<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time", self.base))
    }
}
#[derive(Serialize)]
pub struct LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl {
    #[doc = "Set the field `hours`.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and\ntypically must be less than or equal to 23. An API may choose to allow the\nvalue \"24:00:00\" for scenarios like business closing time."]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\nMinutes of an hour. Must be greater than or equal to 0 and less than or\nequal to 59."]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `nanos`.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0\nand less than or equal to 999,999,999."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\nSeconds of a minute. Must be greater than or equal to 0 and typically must\nbe less than or equal to 59. An API may allow the value 60 if it allows\nleap-seconds."]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl {
    type O =
        BlockAssignable<LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl {}
impl BuildLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl {
    pub fn build(self) -> LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl {
        LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeElRef {
        LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and\ntypically must be less than or equal to 23. An API may choose to allow the\nvalue \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\nMinutes of an hour. Must be greater than or equal to 0 and less than or\nequal to 59."]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0\nand less than or equal to 999,999,999."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSeconds of a minute. Must be greater than or equal to 0 and typically must\nbe less than or equal to 59. An API may allow the value 60 if it allows\nleap-seconds."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize, Default)]
struct LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElDynamic {
    start_time: Option<
        DynamicBlock<LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl>,
    >,
}
#[derive(Serialize)]
pub struct LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl {
    day_of_week: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<Vec<LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl>>,
    dynamic: LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElDynamic,
}
impl LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl Into<
            BlockAssignable<LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.start_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl {
    type O = BlockAssignable<LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl {
    #[doc = "Possible values:\nMONDAY\nTUESDAY\nWEDNESDAY\nTHURSDAY\nFRIDAY\nSATURDAY\nSUNDAY"]
    pub day_of_week: PrimField<String>,
}
impl BuildLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl {
    pub fn build(self) -> LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl {
        LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl {
            day_of_week: self.day_of_week,
            start_time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElRef {
        LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\nPossible values:\nMONDAY\nTUESDAY\nWEDNESDAY\nTHURSDAY\nFRIDAY\nSATURDAY\nSUNDAY"]
    pub fn day_of_week(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> ListRef<LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct LustreInstanceMaintenancePolicyElDynamic {
    maintenance_exclusion_window:
        Option<DynamicBlock<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl>>,
    weekly_maintenance_windows:
        Option<DynamicBlock<LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl>>,
}
#[derive(Serialize)]
pub struct LustreInstanceMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_exclusion_window:
        Option<Vec<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_maintenance_windows:
        Option<Vec<LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl>>,
    dynamic: LustreInstanceMaintenancePolicyElDynamic,
}
impl LustreInstanceMaintenancePolicyEl {
    #[doc = "Set the field `maintenance_exclusion_window`.\n"]
    pub fn set_maintenance_exclusion_window(
        mut self,
        v: impl Into<BlockAssignable<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.maintenance_exclusion_window = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_exclusion_window = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `weekly_maintenance_windows`.\n"]
    pub fn set_weekly_maintenance_windows(
        mut self,
        v: impl Into<BlockAssignable<LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weekly_maintenance_windows = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weekly_maintenance_windows = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LustreInstanceMaintenancePolicyEl {
    type O = BlockAssignable<LustreInstanceMaintenancePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLustreInstanceMaintenancePolicyEl {}
impl BuildLustreInstanceMaintenancePolicyEl {
    pub fn build(self) -> LustreInstanceMaintenancePolicyEl {
        LustreInstanceMaintenancePolicyEl {
            maintenance_exclusion_window: core::default::Default::default(),
            weekly_maintenance_windows: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LustreInstanceMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LustreInstanceMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> LustreInstanceMaintenancePolicyElRef {
        LustreInstanceMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LustreInstanceMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `maintenance_exclusion_window` after provisioning.\n"]
    pub fn maintenance_exclusion_window(
        &self,
    ) -> ListRef<LustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_exclusion_window", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weekly_maintenance_windows` after provisioning.\n"]
    pub fn weekly_maintenance_windows(
        &self,
    ) -> ListRef<LustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_windows", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LustreInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl LustreInstanceTimeoutsEl {
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
impl ToListMappable for LustreInstanceTimeoutsEl {
    type O = BlockAssignable<LustreInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLustreInstanceTimeoutsEl {}
impl BuildLustreInstanceTimeoutsEl {
    pub fn build(self) -> LustreInstanceTimeoutsEl {
        LustreInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct LustreInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LustreInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> LustreInstanceTimeoutsElRef {
        LustreInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LustreInstanceTimeoutsElRef {
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
struct LustreInstanceDynamic {
    access_rules_options: Option<DynamicBlock<LustreInstanceAccessRulesOptionsEl>>,
    dynamic_tier_options: Option<DynamicBlock<LustreInstanceDynamicTierOptionsEl>>,
    maintenance_policy: Option<DynamicBlock<LustreInstanceMaintenancePolicyEl>>,
}
