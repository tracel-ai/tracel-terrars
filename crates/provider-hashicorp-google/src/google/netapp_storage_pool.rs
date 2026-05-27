use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetappStoragePoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_directory: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_auto_tiering: Option<PrimField<bool>>,
    capacity_gib: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_performance_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_hot_tier_auto_resize: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hot_tier_size_gib: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ldap_enabled: Option<PrimField<bool>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    name: PrimField<String>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qos_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_type: Option<PrimField<String>>,
    service_level: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_iops: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_throughput_mibps: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetappStoragePoolTimeoutsEl>,
}
struct NetappStoragePool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetappStoragePoolData>,
}
#[derive(Clone)]
pub struct NetappStoragePool(Rc<NetappStoragePool_>);
impl NetappStoragePool {
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
    #[doc = "Set the field `active_directory`.\nSpecifies the Active Directory policy to be used. Format: 'projects/{{project}}/locations/{{location}}/activeDirectories/{{name}}'.\nThe policy needs to be in the same location as the storage pool."]
    pub fn set_active_directory(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().active_directory = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_auto_tiering`.\nOptional. True if the storage pool supports Auto Tiering enabled volumes. Default is false.\nAuto-tiering can be enabled after storage pool creation but it can't be disabled once enabled."]
    pub fn set_allow_auto_tiering(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_auto_tiering = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_performance_enabled`.\nOptional. True if using Independent Scaling of capacity and performance (Hyperdisk). Default is false."]
    pub fn set_custom_performance_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().custom_performance_enabled = Some(v.into());
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
    #[doc = "Set the field `enable_hot_tier_auto_resize`.\nFlag indicating that the hot-tier threshold will be auto-increased by 10% of the hot-tier when it hits 100%. Default is true.\nThe increment will kick in only if the new size after increment is still less than or equal to storage pool size."]
    pub fn set_enable_hot_tier_auto_resize(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_hot_tier_auto_resize = Some(v.into());
        self
    }
    #[doc = "Set the field `hot_tier_size_gib`.\nTotal hot tier capacity for the Storage Pool. It is applicable only to Flex service level.\nIt should be less than the minimum storage pool size and cannot be more than the current storage pool size. It cannot be decreased once set."]
    pub fn set_hot_tier_size_gib(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().hot_tier_size_gib = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_config`.\nSpecifies the CMEK policy to be used for volume encryption. Format: 'projects/{{project}}/locations/{{location}}/kmsConfigs/{{name}}'.\nThe policy needs to be in the same location as the storage pool."]
    pub fn set_kms_config(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_config = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `ldap_enabled`.\nWhen enabled, the volumes uses Active Directory as LDAP name service for UID/GID lookups. Required to enable extended group support for NFSv3,\nusing security identifiers for NFSv4.1 or principal names for kerberized NFSv4.1."]
    pub fn set_ldap_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ldap_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\nMode of the storage pool.\nThe operational mode of the storage pool. ONTAP mode enables operations\nvia ONTAP Mode APIs, while DEFAULT mode enables operations via NetApp Volumes APIs.\nIf not specified during creation, the mode defaults to DEFAULT. Possible values: [\"MODE_UNSPECIFIED\", \"DEFAULT\", \"ONTAP\"]"]
    pub fn set_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mode = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `qos_type`.\nQoS (Quality of Service) type of the storage pool.\nPossible values are: AUTO, MANUAL. Possible values: [\"QOS_TYPE_UNSPECIFIED\", \"AUTO\", \"MANUAL\"]"]
    pub fn set_qos_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().qos_type = Some(v.into());
        self
    }
    #[doc = "Set the field `replica_zone`.\nSpecifies the replica zone for regional Flex pools. 'zone' and 'replica_zone' values can be swapped to initiate a\n[zone switch](https://cloud.google.com/netapp/volumes/docs/configure-and-use/storage-pools/edit-or-delete-storage-pool#switch_active_and_replica_zones)."]
    pub fn set_replica_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().replica_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `scale_type`.\nThe scale type of the storage pool. Defaults to 'SCALE_TYPE_DEFAULT' if not specified. Possible values: [\"SCALE_TYPE_UNSPECIFIED\", \"SCALE_TYPE_DEFAULT\", \"SCALE_TYPE_SCALEOUT\"]"]
    pub fn set_scale_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().scale_type = Some(v.into());
        self
    }
    #[doc = "Set the field `total_iops`.\nOptional. Custom Performance Total IOPS of the pool If not provided, it will be calculated based on the totalThroughputMibps"]
    pub fn set_total_iops(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().total_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `total_throughput_mibps`.\nOptional. Custom Performance Total Throughput of the pool (in MiB/s)."]
    pub fn set_total_throughput_mibps(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().total_throughput_mibps = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nType of the storage pool.\nThis field is used to control whether the pool supports FILE based volumes only or UNIFIED (both FILE and BLOCK) volumes.\nIf not specified during creation, it defaults to FILE. Possible values: [\"STORAGE_POOL_TYPE_UNSPECIFIED\", \"FILE\", \"UNIFIED\"]"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nSpecifies the active zone for regional Flex pools. 'zone' and 'replica_zone' values can be swapped to initiate a\n[zone switch](https://cloud.google.com/netapp/volumes/docs/configure-and-use/storage-pools/edit-or-delete-storage-pool#switch_active_and_replica_zones).\nIf you want to create a zonal Flex pool, specify a zone name for 'location' and omit 'zone'."]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetappStoragePoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `active_directory` after provisioning.\nSpecifies the Active Directory policy to be used. Format: 'projects/{{project}}/locations/{{location}}/activeDirectories/{{name}}'.\nThe policy needs to be in the same location as the storage pool."]
    pub fn active_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active_directory", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow_auto_tiering` after provisioning.\nOptional. True if the storage pool supports Auto Tiering enabled volumes. Default is false.\nAuto-tiering can be enabled after storage pool creation but it can't be disabled once enabled."]
    pub fn allow_auto_tiering(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_auto_tiering", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `available_throughput_mibps` after provisioning.\nAvailable throughput of the storage pool (in MiB/s)."]
    pub fn available_throughput_mibps(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_throughput_mibps", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `capacity_gib` after provisioning.\nCapacity of the storage pool (in GiB)."]
    pub fn capacity_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cold_tier_size_used_gib` after provisioning.\nTotal cold tier data rounded down to the nearest GiB used by the storage pool."]
    pub fn cold_tier_size_used_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cold_tier_size_used_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_performance_enabled` after provisioning.\nOptional. True if using Independent Scaling of capacity and performance (Hyperdisk). Default is false."]
    pub fn custom_performance_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_performance_enabled", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_hot_tier_auto_resize` after provisioning.\nFlag indicating that the hot-tier threshold will be auto-increased by 10% of the hot-tier when it hits 100%. Default is true.\nThe increment will kick in only if the new size after increment is still less than or equal to storage pool size."]
    pub fn enable_hot_tier_auto_resize(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_hot_tier_auto_resize", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\nReports if volumes in the pool are encrypted using a Google-managed encryption key or CMEK."]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hot_tier_size_gib` after provisioning.\nTotal hot tier capacity for the Storage Pool. It is applicable only to Flex service level.\nIt should be less than the minimum storage pool size and cannot be more than the current storage pool size. It cannot be decreased once set."]
    pub fn hot_tier_size_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hot_tier_size_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hot_tier_size_used_gib` after provisioning.\nTotal hot tier data rounded down to the nearest GiB used by the storage pool."]
    pub fn hot_tier_size_used_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hot_tier_size_used_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_config` after provisioning.\nSpecifies the CMEK policy to be used for volume encryption. Format: 'projects/{{project}}/locations/{{location}}/kmsConfigs/{{name}}'.\nThe policy needs to be in the same location as the storage pool."]
    pub fn kms_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ldap_enabled` after provisioning.\nWhen enabled, the volumes uses Active Directory as LDAP name service for UID/GID lookups. Required to enable extended group support for NFSv3,\nusing security identifiers for NFSv4.1 or principal names for kerberized NFSv4.1."]
    pub fn ldap_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ldap_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nName of the location. For zonal Flex pools specify a zone name, in all other cases a region name."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nMode of the storage pool.\nThe operational mode of the storage pool. ONTAP mode enables operations\nvia ONTAP Mode APIs, while DEFAULT mode enables operations via NetApp Volumes APIs.\nIf not specified during creation, the mode defaults to DEFAULT. Possible values: [\"MODE_UNSPECIFIED\", \"DEFAULT\", \"ONTAP\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the storage pool. Needs to be unique per location/region."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nVPC network name with format: 'projects/{{project}}/global/networks/{{network}}'"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `qos_type` after provisioning.\nQoS (Quality of Service) type of the storage pool.\nPossible values are: AUTO, MANUAL. Possible values: [\"QOS_TYPE_UNSPECIFIED\", \"AUTO\", \"MANUAL\"]"]
    pub fn qos_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qos_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_zone` after provisioning.\nSpecifies the replica zone for regional Flex pools. 'zone' and 'replica_zone' values can be swapped to initiate a\n[zone switch](https://cloud.google.com/netapp/volumes/docs/configure-and-use/storage-pools/edit-or-delete-storage-pool#switch_active_and_replica_zones)."]
    pub fn replica_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scale_type` after provisioning.\nThe scale type of the storage pool. Defaults to 'SCALE_TYPE_DEFAULT' if not specified. Possible values: [\"SCALE_TYPE_UNSPECIFIED\", \"SCALE_TYPE_DEFAULT\", \"SCALE_TYPE_SCALEOUT\"]"]
    pub fn scale_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scale_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_level` after provisioning.\nService level of the storage pool. Possible values: [\"PREMIUM\", \"EXTREME\", \"STANDARD\", \"FLEX\"]"]
    pub fn service_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_iops` after provisioning.\nOptional. Custom Performance Total IOPS of the pool If not provided, it will be calculated based on the totalThroughputMibps"]
    pub fn total_iops(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_iops", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_throughput_mibps` after provisioning.\nOptional. Custom Performance Total Throughput of the pool (in MiB/s)."]
    pub fn total_throughput_mibps(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_throughput_mibps", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of the storage pool.\nThis field is used to control whether the pool supports FILE based volumes only or UNIFIED (both FILE and BLOCK) volumes.\nIf not specified during creation, it defaults to FILE. Possible values: [\"STORAGE_POOL_TYPE_UNSPECIFIED\", \"FILE\", \"UNIFIED\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `volume_capacity_gib` after provisioning.\nSize allocated to volumes in the storage pool (in GiB)."]
    pub fn volume_capacity_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_capacity_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `volume_count` after provisioning.\nNumber of volume in the storage pool."]
    pub fn volume_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nSpecifies the active zone for regional Flex pools. 'zone' and 'replica_zone' values can be swapped to initiate a\n[zone switch](https://cloud.google.com/netapp/volumes/docs/configure-and-use/storage-pools/edit-or-delete-storage-pool#switch_active_and_replica_zones).\nIf you want to create a zonal Flex pool, specify a zone name for 'location' and omit 'zone'."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetappStoragePoolTimeoutsElRef {
        NetappStoragePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetappStoragePool {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetappStoragePool {}
impl ToListMappable for NetappStoragePool {
    type O = ListRef<NetappStoragePoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetappStoragePool_ {
    fn extract_resource_type(&self) -> String {
        "google_netapp_storage_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetappStoragePool {
    pub tf_id: String,
    #[doc = "Capacity of the storage pool (in GiB)."]
    pub capacity_gib: PrimField<String>,
    #[doc = "Name of the location. For zonal Flex pools specify a zone name, in all other cases a region name."]
    pub location: PrimField<String>,
    #[doc = "The resource name of the storage pool. Needs to be unique per location/region."]
    pub name: PrimField<String>,
    #[doc = "VPC network name with format: 'projects/{{project}}/global/networks/{{network}}'"]
    pub network: PrimField<String>,
    #[doc = "Service level of the storage pool. Possible values: [\"PREMIUM\", \"EXTREME\", \"STANDARD\", \"FLEX\"]"]
    pub service_level: PrimField<String>,
}
impl BuildNetappStoragePool {
    pub fn build(self, stack: &mut Stack) -> NetappStoragePool {
        let out = NetappStoragePool(Rc::new(NetappStoragePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetappStoragePoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                active_directory: core::default::Default::default(),
                allow_auto_tiering: core::default::Default::default(),
                capacity_gib: self.capacity_gib,
                custom_performance_enabled: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                enable_hot_tier_auto_resize: core::default::Default::default(),
                hot_tier_size_gib: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_config: core::default::Default::default(),
                labels: core::default::Default::default(),
                ldap_enabled: core::default::Default::default(),
                location: self.location,
                mode: core::default::Default::default(),
                name: self.name,
                network: self.network,
                project: core::default::Default::default(),
                qos_type: core::default::Default::default(),
                replica_zone: core::default::Default::default(),
                scale_type: core::default::Default::default(),
                service_level: self.service_level,
                total_iops: core::default::Default::default(),
                total_throughput_mibps: core::default::Default::default(),
                type_: core::default::Default::default(),
                zone: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetappStoragePoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappStoragePoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetappStoragePoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `active_directory` after provisioning.\nSpecifies the Active Directory policy to be used. Format: 'projects/{{project}}/locations/{{location}}/activeDirectories/{{name}}'.\nThe policy needs to be in the same location as the storage pool."]
    pub fn active_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active_directory", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow_auto_tiering` after provisioning.\nOptional. True if the storage pool supports Auto Tiering enabled volumes. Default is false.\nAuto-tiering can be enabled after storage pool creation but it can't be disabled once enabled."]
    pub fn allow_auto_tiering(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_auto_tiering", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `available_throughput_mibps` after provisioning.\nAvailable throughput of the storage pool (in MiB/s)."]
    pub fn available_throughput_mibps(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_throughput_mibps", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `capacity_gib` after provisioning.\nCapacity of the storage pool (in GiB)."]
    pub fn capacity_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cold_tier_size_used_gib` after provisioning.\nTotal cold tier data rounded down to the nearest GiB used by the storage pool."]
    pub fn cold_tier_size_used_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cold_tier_size_used_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_performance_enabled` after provisioning.\nOptional. True if using Independent Scaling of capacity and performance (Hyperdisk). Default is false."]
    pub fn custom_performance_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_performance_enabled", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_hot_tier_auto_resize` after provisioning.\nFlag indicating that the hot-tier threshold will be auto-increased by 10% of the hot-tier when it hits 100%. Default is true.\nThe increment will kick in only if the new size after increment is still less than or equal to storage pool size."]
    pub fn enable_hot_tier_auto_resize(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_hot_tier_auto_resize", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\nReports if volumes in the pool are encrypted using a Google-managed encryption key or CMEK."]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hot_tier_size_gib` after provisioning.\nTotal hot tier capacity for the Storage Pool. It is applicable only to Flex service level.\nIt should be less than the minimum storage pool size and cannot be more than the current storage pool size. It cannot be decreased once set."]
    pub fn hot_tier_size_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hot_tier_size_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hot_tier_size_used_gib` after provisioning.\nTotal hot tier data rounded down to the nearest GiB used by the storage pool."]
    pub fn hot_tier_size_used_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hot_tier_size_used_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_config` after provisioning.\nSpecifies the CMEK policy to be used for volume encryption. Format: 'projects/{{project}}/locations/{{location}}/kmsConfigs/{{name}}'.\nThe policy needs to be in the same location as the storage pool."]
    pub fn kms_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs. Example: '{ \"owner\": \"Bob\", \"department\": \"finance\", \"purpose\": \"testing\" }'.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ldap_enabled` after provisioning.\nWhen enabled, the volumes uses Active Directory as LDAP name service for UID/GID lookups. Required to enable extended group support for NFSv3,\nusing security identifiers for NFSv4.1 or principal names for kerberized NFSv4.1."]
    pub fn ldap_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ldap_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nName of the location. For zonal Flex pools specify a zone name, in all other cases a region name."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nMode of the storage pool.\nThe operational mode of the storage pool. ONTAP mode enables operations\nvia ONTAP Mode APIs, while DEFAULT mode enables operations via NetApp Volumes APIs.\nIf not specified during creation, the mode defaults to DEFAULT. Possible values: [\"MODE_UNSPECIFIED\", \"DEFAULT\", \"ONTAP\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the storage pool. Needs to be unique per location/region."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nVPC network name with format: 'projects/{{project}}/global/networks/{{network}}'"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `qos_type` after provisioning.\nQoS (Quality of Service) type of the storage pool.\nPossible values are: AUTO, MANUAL. Possible values: [\"QOS_TYPE_UNSPECIFIED\", \"AUTO\", \"MANUAL\"]"]
    pub fn qos_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qos_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replica_zone` after provisioning.\nSpecifies the replica zone for regional Flex pools. 'zone' and 'replica_zone' values can be swapped to initiate a\n[zone switch](https://cloud.google.com/netapp/volumes/docs/configure-and-use/storage-pools/edit-or-delete-storage-pool#switch_active_and_replica_zones)."]
    pub fn replica_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scale_type` after provisioning.\nThe scale type of the storage pool. Defaults to 'SCALE_TYPE_DEFAULT' if not specified. Possible values: [\"SCALE_TYPE_UNSPECIFIED\", \"SCALE_TYPE_DEFAULT\", \"SCALE_TYPE_SCALEOUT\"]"]
    pub fn scale_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scale_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_level` after provisioning.\nService level of the storage pool. Possible values: [\"PREMIUM\", \"EXTREME\", \"STANDARD\", \"FLEX\"]"]
    pub fn service_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_iops` after provisioning.\nOptional. Custom Performance Total IOPS of the pool If not provided, it will be calculated based on the totalThroughputMibps"]
    pub fn total_iops(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_iops", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_throughput_mibps` after provisioning.\nOptional. Custom Performance Total Throughput of the pool (in MiB/s)."]
    pub fn total_throughput_mibps(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_throughput_mibps", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of the storage pool.\nThis field is used to control whether the pool supports FILE based volumes only or UNIFIED (both FILE and BLOCK) volumes.\nIf not specified during creation, it defaults to FILE. Possible values: [\"STORAGE_POOL_TYPE_UNSPECIFIED\", \"FILE\", \"UNIFIED\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `volume_capacity_gib` after provisioning.\nSize allocated to volumes in the storage pool (in GiB)."]
    pub fn volume_capacity_gib(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_capacity_gib", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `volume_count` after provisioning.\nNumber of volume in the storage pool."]
    pub fn volume_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nSpecifies the active zone for regional Flex pools. 'zone' and 'replica_zone' values can be swapped to initiate a\n[zone switch](https://cloud.google.com/netapp/volumes/docs/configure-and-use/storage-pools/edit-or-delete-storage-pool#switch_active_and_replica_zones).\nIf you want to create a zonal Flex pool, specify a zone name for 'location' and omit 'zone'."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetappStoragePoolTimeoutsElRef {
        NetappStoragePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetappStoragePoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetappStoragePoolTimeoutsEl {
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
impl ToListMappable for NetappStoragePoolTimeoutsEl {
    type O = BlockAssignable<NetappStoragePoolTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetappStoragePoolTimeoutsEl {}
impl BuildNetappStoragePoolTimeoutsEl {
    pub fn build(self) -> NetappStoragePoolTimeoutsEl {
        NetappStoragePoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetappStoragePoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetappStoragePoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetappStoragePoolTimeoutsElRef {
        NetappStoragePoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetappStoragePoolTimeoutsElRef {
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
