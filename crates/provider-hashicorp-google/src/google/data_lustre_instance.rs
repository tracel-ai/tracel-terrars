use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataLustreInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
struct DataLustreInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLustreInstanceData>,
}
#[derive(Clone)]
pub struct DataLustreInstance(Rc<DataLustreInstance_>);
impl DataLustreInstance {
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
    #[doc = "Set the field `zone`.\nZone of Lustre instance"]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `access_rules_options` after provisioning.\nIP-based access rules for the Managed Lustre instance. These options\ndefine the root user squash configuration."]
    pub fn access_rules_options(&self) -> ListRef<DataLustreInstanceAccessRulesOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_rules_options", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `dynamic_tier_options` after provisioning.\nDynamic tier options for a Managed Lustre instance."]
    pub fn dynamic_tier_options(&self) -> ListRef<DataLustreInstanceDynamicTierOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dynamic_tier_options", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\nDefines a maintenance policy for a resource."]
    pub fn maintenance_policy(&self) -> ListRef<DataLustreInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
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
    ) -> ListRef<DataLustreInstanceUpcomingMaintenanceScheduleElRef> {
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
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nZone of Lustre instance"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
}
impl Referable for DataLustreInstance {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataLustreInstance {}
impl ToListMappable for DataLustreInstance {
    type O = ListRef<DataLustreInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataLustreInstance_ {
    fn extract_datasource_type(&self) -> String {
        "google_lustre_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataLustreInstance {
    pub tf_id: String,
    #[doc = "The name of the Managed Lustre instance.\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter."]
    pub instance_id: PrimField<String>,
}
impl BuildDataLustreInstance {
    pub fn build(self, stack: &mut Stack) -> DataLustreInstance {
        let out = DataLustreInstance(Rc::new(DataLustreInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLustreInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                project: core::default::Default::default(),
                zone: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataLustreInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLustreInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataLustreInstanceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `access_rules_options` after provisioning.\nIP-based access rules for the Managed Lustre instance. These options\ndefine the root user squash configuration."]
    pub fn access_rules_options(&self) -> ListRef<DataLustreInstanceAccessRulesOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_rules_options", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `dynamic_tier_options` after provisioning.\nDynamic tier options for a Managed Lustre instance."]
    pub fn dynamic_tier_options(&self) -> ListRef<DataLustreInstanceDynamicTierOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dynamic_tier_options", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\nDefines a maintenance policy for a resource."]
    pub fn maintenance_policy(&self) -> ListRef<DataLustreInstanceMaintenancePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_policy", self.extract_ref()),
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
    ) -> ListRef<DataLustreInstanceUpcomingMaintenanceScheduleElRef> {
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
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nZone of Lustre instance"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataLustreInstanceAccessRulesOptionsElAccessRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    squash_mode: Option<PrimField<String>>,
}
impl DataLustreInstanceAccessRulesOptionsElAccessRulesEl {
    #[doc = "Set the field `ip_address_ranges`.\n"]
    pub fn set_ip_address_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_address_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `squash_mode`.\n"]
    pub fn set_squash_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.squash_mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataLustreInstanceAccessRulesOptionsElAccessRulesEl {
    type O = BlockAssignable<DataLustreInstanceAccessRulesOptionsElAccessRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLustreInstanceAccessRulesOptionsElAccessRulesEl {}
impl BuildDataLustreInstanceAccessRulesOptionsElAccessRulesEl {
    pub fn build(self) -> DataLustreInstanceAccessRulesOptionsElAccessRulesEl {
        DataLustreInstanceAccessRulesOptionsElAccessRulesEl {
            ip_address_ranges: core::default::Default::default(),
            name: core::default::Default::default(),
            squash_mode: core::default::Default::default(),
        }
    }
}
pub struct DataLustreInstanceAccessRulesOptionsElAccessRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLustreInstanceAccessRulesOptionsElAccessRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLustreInstanceAccessRulesOptionsElAccessRulesElRef {
        DataLustreInstanceAccessRulesOptionsElAccessRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLustreInstanceAccessRulesOptionsElAccessRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_address_ranges` after provisioning.\n"]
    pub fn ip_address_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_address_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `squash_mode` after provisioning.\n"]
    pub fn squash_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_mode", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLustreInstanceAccessRulesOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_rules: Option<ListField<DataLustreInstanceAccessRulesOptionsElAccessRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_squash_gid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_squash_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_squash_uid: Option<PrimField<f64>>,
}
impl DataLustreInstanceAccessRulesOptionsEl {
    #[doc = "Set the field `access_rules`.\n"]
    pub fn set_access_rules(
        mut self,
        v: impl Into<ListField<DataLustreInstanceAccessRulesOptionsElAccessRulesEl>>,
    ) -> Self {
        self.access_rules = Some(v.into());
        self
    }
    #[doc = "Set the field `default_squash_gid`.\n"]
    pub fn set_default_squash_gid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_squash_gid = Some(v.into());
        self
    }
    #[doc = "Set the field `default_squash_mode`.\n"]
    pub fn set_default_squash_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_squash_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `default_squash_uid`.\n"]
    pub fn set_default_squash_uid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_squash_uid = Some(v.into());
        self
    }
}
impl ToListMappable for DataLustreInstanceAccessRulesOptionsEl {
    type O = BlockAssignable<DataLustreInstanceAccessRulesOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLustreInstanceAccessRulesOptionsEl {}
impl BuildDataLustreInstanceAccessRulesOptionsEl {
    pub fn build(self) -> DataLustreInstanceAccessRulesOptionsEl {
        DataLustreInstanceAccessRulesOptionsEl {
            access_rules: core::default::Default::default(),
            default_squash_gid: core::default::Default::default(),
            default_squash_mode: core::default::Default::default(),
            default_squash_uid: core::default::Default::default(),
        }
    }
}
pub struct DataLustreInstanceAccessRulesOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLustreInstanceAccessRulesOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataLustreInstanceAccessRulesOptionsElRef {
        DataLustreInstanceAccessRulesOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLustreInstanceAccessRulesOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_rules` after provisioning.\n"]
    pub fn access_rules(&self) -> ListRef<DataLustreInstanceAccessRulesOptionsElAccessRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_rules", self.base))
    }
    #[doc = "Get a reference to the value of field `default_squash_gid` after provisioning.\n"]
    pub fn default_squash_gid(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_squash_gid", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_squash_mode` after provisioning.\n"]
    pub fn default_squash_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_squash_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_squash_uid` after provisioning.\n"]
    pub fn default_squash_uid(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_squash_uid", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLustreInstanceDynamicTierOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}
impl DataLustreInstanceDynamicTierOptionsEl {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataLustreInstanceDynamicTierOptionsEl {
    type O = BlockAssignable<DataLustreInstanceDynamicTierOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLustreInstanceDynamicTierOptionsEl {}
impl BuildDataLustreInstanceDynamicTierOptionsEl {
    pub fn build(self) -> DataLustreInstanceDynamicTierOptionsEl {
        DataLustreInstanceDynamicTierOptionsEl {
            mode: core::default::Default::default(),
        }
    }
}
pub struct DataLustreInstanceDynamicTierOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLustreInstanceDynamicTierOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataLustreInstanceDynamicTierOptionsElRef {
        DataLustreInstanceDynamicTierOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLustreInstanceDynamicTierOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<PrimField<f64>>,
}
impl DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl {
    #[doc = "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.day = Some(v.into());
        self
    }
    #[doc = "Set the field `month`.\n"]
    pub fn set_month(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.month = Some(v.into());
        self
    }
    #[doc = "Set the field `year`.\n"]
    pub fn set_year(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.year = Some(v.into());
        self
    }
}
impl ToListMappable for DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl {
    type O =
        BlockAssignable<DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl {}
impl BuildDataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl {
    pub fn build(
        self,
    ) -> DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl {
        DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}
pub struct DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateElRef {
        DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `month` after provisioning.\n"]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }
    #[doc = "Get a reference to the value of field `year` after provisioning.\n"]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    month: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<PrimField<f64>>,
}
impl DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl {
    #[doc = "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.day = Some(v.into());
        self
    }
    #[doc = "Set the field `month`.\n"]
    pub fn set_month(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.month = Some(v.into());
        self
    }
    #[doc = "Set the field `year`.\n"]
    pub fn set_year(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.year = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl
{
    type O = BlockAssignable<
        DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl {}
impl BuildDataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl {
    pub fn build(
        self,
    ) -> DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl {
        DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}
pub struct DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateElRef {
        DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
    #[doc = "Get a reference to the value of field `month` after provisioning.\n"]
    pub fn month(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }
    #[doc = "Get a reference to the value of field `year` after provisioning.\n"]
    pub fn year(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl {
    type O =
        BlockAssignable<DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl {}
impl BuildDataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl {
    pub fn build(self) -> DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl {
        DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeElRef {
        DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<
        ListField<DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<
        ListField<DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    time:
        Option<ListField<DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl>>,
}
impl DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl {
    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(
        mut self,
        v: impl Into<
            ListField<DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateEl>,
        >,
    ) -> Self {
        self.end_date = Some(v.into());
        self
    }
    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(
        mut self,
        v: impl Into<
            ListField<DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateEl>,
        >,
    ) -> Self {
        self.start_date = Some(v.into());
        self
    }
    #[doc = "Set the field `time`.\n"]
    pub fn set_time(
        mut self,
        v: impl Into<ListField<DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeEl>>,
    ) -> Self {
        self.time = Some(v.into());
        self
    }
}
impl ToListMappable for DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl {
    type O = BlockAssignable<DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl {}
impl BuildDataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl {
    pub fn build(self) -> DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl {
        DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl {
            end_date: core::default::Default::default(),
            start_date: core::default::Default::default(),
            time: core::default::Default::default(),
        }
    }
}
pub struct DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElRef {
        DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(
        &self,
    ) -> ListRef<DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElEndDateElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.end_date", self.base))
    }
    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(
        &self,
    ) -> ListRef<DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElStartDateElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.start_date", self.base))
    }
    #[doc = "Get a reference to the value of field `time` after provisioning.\n"]
    pub fn time(
        &self,
    ) -> ListRef<DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }
    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl {
    type O =
        BlockAssignable<DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl {}
impl BuildDataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl {
    pub fn build(
        self,
    ) -> DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl {
        DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeElRef {
        DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }
    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day_of_week: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<
        ListField<DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl>,
    >,
}
impl DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl {
    #[doc = "Set the field `day_of_week`.\n"]
    pub fn set_day_of_week(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.day_of_week = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl Into<
            ListField<DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeEl>,
        >,
    ) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl {
    type O = BlockAssignable<DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl {}
impl BuildDataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl {
    pub fn build(self) -> DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl {
        DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl {
            day_of_week: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElRef {
        DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"]
    pub fn day_of_week(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> ListRef<DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElStartTimeElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLustreInstanceMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_exclusion_window:
        Option<ListField<DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_maintenance_windows:
        Option<ListField<DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl>>,
}
impl DataLustreInstanceMaintenancePolicyEl {
    #[doc = "Set the field `maintenance_exclusion_window`.\n"]
    pub fn set_maintenance_exclusion_window(
        mut self,
        v: impl Into<ListField<DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowEl>>,
    ) -> Self {
        self.maintenance_exclusion_window = Some(v.into());
        self
    }
    #[doc = "Set the field `weekly_maintenance_windows`.\n"]
    pub fn set_weekly_maintenance_windows(
        mut self,
        v: impl Into<ListField<DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsEl>>,
    ) -> Self {
        self.weekly_maintenance_windows = Some(v.into());
        self
    }
}
impl ToListMappable for DataLustreInstanceMaintenancePolicyEl {
    type O = BlockAssignable<DataLustreInstanceMaintenancePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLustreInstanceMaintenancePolicyEl {}
impl BuildDataLustreInstanceMaintenancePolicyEl {
    pub fn build(self) -> DataLustreInstanceMaintenancePolicyEl {
        DataLustreInstanceMaintenancePolicyEl {
            maintenance_exclusion_window: core::default::Default::default(),
            weekly_maintenance_windows: core::default::Default::default(),
        }
    }
}
pub struct DataLustreInstanceMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLustreInstanceMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> DataLustreInstanceMaintenancePolicyElRef {
        DataLustreInstanceMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLustreInstanceMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `maintenance_exclusion_window` after provisioning.\n"]
    pub fn maintenance_exclusion_window(
        &self,
    ) -> ListRef<DataLustreInstanceMaintenancePolicyElMaintenanceExclusionWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_exclusion_window", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weekly_maintenance_windows` after provisioning.\n"]
    pub fn weekly_maintenance_windows(
        &self,
    ) -> ListRef<DataLustreInstanceMaintenancePolicyElWeeklyMaintenanceWindowsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weekly_maintenance_windows", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLustreInstanceUpcomingMaintenanceScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl DataLustreInstanceUpcomingMaintenanceScheduleEl {
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
impl ToListMappable for DataLustreInstanceUpcomingMaintenanceScheduleEl {
    type O = BlockAssignable<DataLustreInstanceUpcomingMaintenanceScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLustreInstanceUpcomingMaintenanceScheduleEl {}
impl BuildDataLustreInstanceUpcomingMaintenanceScheduleEl {
    pub fn build(self) -> DataLustreInstanceUpcomingMaintenanceScheduleEl {
        DataLustreInstanceUpcomingMaintenanceScheduleEl {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataLustreInstanceUpcomingMaintenanceScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLustreInstanceUpcomingMaintenanceScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLustreInstanceUpcomingMaintenanceScheduleElRef {
        DataLustreInstanceUpcomingMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLustreInstanceUpcomingMaintenanceScheduleElRef {
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
