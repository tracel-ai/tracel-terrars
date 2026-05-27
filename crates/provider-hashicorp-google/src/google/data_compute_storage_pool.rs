use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeStoragePoolData {
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
struct DataComputeStoragePool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeStoragePoolData>,
}
#[derive(Clone)]
pub struct DataComputeStoragePool(Rc<DataComputeStoragePool_>);
impl DataComputeStoragePool {
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
    #[doc = "Get a reference to the value of field `capacity_provisioning_type` after provisioning.\nProvisioning type of the byte capacity of the pool. Possible values: [\"STANDARD\", \"ADVANCED\"]"]
    pub fn capacity_provisioning_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_provisioning_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the StoragePool.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the StoragePool will fail.\nWhen the field is set to false, deleting the StoragePool is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of this resource. Provide this property when you create the resource."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nType of the resource."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource.\nUsed internally during updates."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this storage pool. These can be later modified by the setLabels method.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created.\nThe name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter,\nand all following characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\nAdditional params passed with the request, but not persisted as part of resource payload"]
    pub fn params(&self) -> ListRef<DataComputeStoragePoolParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_provisioning_type` after provisioning.\nProvisioning type of the performance-related parameters of the pool, such as throughput and IOPS. Possible values: [\"STANDARD\", \"ADVANCED\"]"]
    pub fn performance_provisioning_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_provisioning_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pool_provisioned_capacity_gb` after provisioning.\nSize, in GiB, of the storage pool. For more information about the size limits,\nsee https://cloud.google.com/compute/docs/disks/storage-pools."]
    pub fn pool_provisioned_capacity_gb(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_provisioned_capacity_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pool_provisioned_iops` after provisioning.\nProvisioned IOPS of the storage pool.\nOnly relevant if the storage pool type is 'hyperdisk-balanced'."]
    pub fn pool_provisioned_iops(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_provisioned_iops", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pool_provisioned_throughput` after provisioning.\nProvisioned throughput, in MB/s, of the storage pool.\nOnly relevant if the storage pool type is 'hyperdisk-balanced' or 'hyperdisk-throughput'."]
    pub fn pool_provisioned_throughput(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_provisioned_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_status` after provisioning.\nStatus information for the storage pool resource."]
    pub fn resource_status(&self) -> ListRef<DataComputeStoragePoolResourceStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus information for the storage pool resource."]
    pub fn status(&self) -> ListRef<DataComputeStoragePoolStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_pool_type` after provisioning.\nType of the storage pool. For example, the\nfollowing are valid values:\n\n* 'https://www.googleapis.com/compute/v1/projects/{project_id}/zones/{zone}/storagePoolTypes/hyperdisk-balanced'\n* 'hyperdisk-throughput'"]
    pub fn storage_pool_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_pool_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nA reference to the zone where the storage pool resides."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeStoragePool {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeStoragePool {}
impl ToListMappable for DataComputeStoragePool {
    type O = ListRef<DataComputeStoragePoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeStoragePool_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_storage_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeStoragePool {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is created.\nThe name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter,\nand all following characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc = "A reference to the zone where the storage pool resides."]
    pub zone: PrimField<String>,
}
impl BuildDataComputeStoragePool {
    pub fn build(self, stack: &mut Stack) -> DataComputeStoragePool {
        let out = DataComputeStoragePool(Rc::new(DataComputeStoragePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeStoragePoolData {
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
pub struct DataComputeStoragePoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeStoragePoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeStoragePoolRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `capacity_provisioning_type` after provisioning.\nProvisioning type of the byte capacity of the pool. Possible values: [\"STANDARD\", \"ADVANCED\"]"]
    pub fn capacity_provisioning_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_provisioning_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the StoragePool.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the StoragePool will fail.\nWhen the field is set to false, deleting the StoragePool is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of this resource. Provide this property when you create the resource."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nType of the resource."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource.\nUsed internally during updates."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this storage pool. These can be later modified by the setLabels method.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created.\nThe name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter,\nand all following characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\nAdditional params passed with the request, but not persisted as part of resource payload"]
    pub fn params(&self) -> ListRef<DataComputeStoragePoolParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `performance_provisioning_type` after provisioning.\nProvisioning type of the performance-related parameters of the pool, such as throughput and IOPS. Possible values: [\"STANDARD\", \"ADVANCED\"]"]
    pub fn performance_provisioning_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.performance_provisioning_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pool_provisioned_capacity_gb` after provisioning.\nSize, in GiB, of the storage pool. For more information about the size limits,\nsee https://cloud.google.com/compute/docs/disks/storage-pools."]
    pub fn pool_provisioned_capacity_gb(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_provisioned_capacity_gb", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pool_provisioned_iops` after provisioning.\nProvisioned IOPS of the storage pool.\nOnly relevant if the storage pool type is 'hyperdisk-balanced'."]
    pub fn pool_provisioned_iops(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_provisioned_iops", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pool_provisioned_throughput` after provisioning.\nProvisioned throughput, in MB/s, of the storage pool.\nOnly relevant if the storage pool type is 'hyperdisk-balanced' or 'hyperdisk-throughput'."]
    pub fn pool_provisioned_throughput(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_provisioned_throughput", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_status` after provisioning.\nStatus information for the storage pool resource."]
    pub fn resource_status(&self) -> ListRef<DataComputeStoragePoolResourceStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus information for the storage pool resource."]
    pub fn status(&self) -> ListRef<DataComputeStoragePoolStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_pool_type` after provisioning.\nType of the storage pool. For example, the\nfollowing are valid values:\n\n* 'https://www.googleapis.com/compute/v1/projects/{project_id}/zones/{zone}/storagePoolTypes/hyperdisk-balanced'\n* 'hyperdisk-throughput'"]
    pub fn storage_pool_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_pool_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nA reference to the zone where the storage pool resides."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeStoragePoolParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl DataComputeStoragePoolParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeStoragePoolParamsEl {
    type O = BlockAssignable<DataComputeStoragePoolParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeStoragePoolParamsEl {}
impl BuildDataComputeStoragePoolParamsEl {
    pub fn build(self) -> DataComputeStoragePoolParamsEl {
        DataComputeStoragePoolParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct DataComputeStoragePoolParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeStoragePoolParamsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeStoragePoolParamsElRef {
        DataComputeStoragePoolParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeStoragePoolParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\n"]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeStoragePoolResourceStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_count: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_resize_timestamp: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_total_provisioned_disk_capacity_gb: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool_used_capacity_bytes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool_used_iops: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool_used_throughput: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool_user_written_bytes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_provisioned_disk_capacity_gb: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_provisioned_disk_iops: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_provisioned_disk_throughput: Option<PrimField<String>>,
}
impl DataComputeStoragePoolResourceStatusEl {
    #[doc = "Set the field `disk_count`.\n"]
    pub fn set_disk_count(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_count = Some(v.into());
        self
    }
    #[doc = "Set the field `last_resize_timestamp`.\n"]
    pub fn set_last_resize_timestamp(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_resize_timestamp = Some(v.into());
        self
    }
    #[doc = "Set the field `max_total_provisioned_disk_capacity_gb`.\n"]
    pub fn set_max_total_provisioned_disk_capacity_gb(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.max_total_provisioned_disk_capacity_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `pool_used_capacity_bytes`.\n"]
    pub fn set_pool_used_capacity_bytes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pool_used_capacity_bytes = Some(v.into());
        self
    }
    #[doc = "Set the field `pool_used_iops`.\n"]
    pub fn set_pool_used_iops(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pool_used_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `pool_used_throughput`.\n"]
    pub fn set_pool_used_throughput(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pool_used_throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `pool_user_written_bytes`.\n"]
    pub fn set_pool_user_written_bytes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pool_user_written_bytes = Some(v.into());
        self
    }
    #[doc = "Set the field `total_provisioned_disk_capacity_gb`.\n"]
    pub fn set_total_provisioned_disk_capacity_gb(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.total_provisioned_disk_capacity_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `total_provisioned_disk_iops`.\n"]
    pub fn set_total_provisioned_disk_iops(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.total_provisioned_disk_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `total_provisioned_disk_throughput`.\n"]
    pub fn set_total_provisioned_disk_throughput(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.total_provisioned_disk_throughput = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeStoragePoolResourceStatusEl {
    type O = BlockAssignable<DataComputeStoragePoolResourceStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeStoragePoolResourceStatusEl {}
impl BuildDataComputeStoragePoolResourceStatusEl {
    pub fn build(self) -> DataComputeStoragePoolResourceStatusEl {
        DataComputeStoragePoolResourceStatusEl {
            disk_count: core::default::Default::default(),
            last_resize_timestamp: core::default::Default::default(),
            max_total_provisioned_disk_capacity_gb: core::default::Default::default(),
            pool_used_capacity_bytes: core::default::Default::default(),
            pool_used_iops: core::default::Default::default(),
            pool_used_throughput: core::default::Default::default(),
            pool_user_written_bytes: core::default::Default::default(),
            total_provisioned_disk_capacity_gb: core::default::Default::default(),
            total_provisioned_disk_iops: core::default::Default::default(),
            total_provisioned_disk_throughput: core::default::Default::default(),
        }
    }
}
pub struct DataComputeStoragePoolResourceStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeStoragePoolResourceStatusElRef {
    fn new(shared: StackShared, base: String) -> DataComputeStoragePoolResourceStatusElRef {
        DataComputeStoragePoolResourceStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeStoragePoolResourceStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_count` after provisioning.\n"]
    pub fn disk_count(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_count", self.base))
    }
    #[doc = "Get a reference to the value of field `last_resize_timestamp` after provisioning.\n"]
    pub fn last_resize_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_resize_timestamp", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_total_provisioned_disk_capacity_gb` after provisioning.\n"]
    pub fn max_total_provisioned_disk_capacity_gb(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_total_provisioned_disk_capacity_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pool_used_capacity_bytes` after provisioning.\n"]
    pub fn pool_used_capacity_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_used_capacity_bytes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pool_used_iops` after provisioning.\n"]
    pub fn pool_used_iops(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_used_iops", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pool_used_throughput` after provisioning.\n"]
    pub fn pool_used_throughput(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_used_throughput", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pool_user_written_bytes` after provisioning.\n"]
    pub fn pool_user_written_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_user_written_bytes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_provisioned_disk_capacity_gb` after provisioning.\n"]
    pub fn total_provisioned_disk_capacity_gb(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_provisioned_disk_capacity_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_provisioned_disk_iops` after provisioning.\n"]
    pub fn total_provisioned_disk_iops(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_provisioned_disk_iops", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_provisioned_disk_throughput` after provisioning.\n"]
    pub fn total_provisioned_disk_throughput(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_provisioned_disk_throughput", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeStoragePoolStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_count: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_resize_timestamp: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_total_provisioned_disk_capacity_gb: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool_used_capacity_bytes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool_used_iops: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool_used_throughput: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool_user_written_bytes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_provisioned_disk_capacity_gb: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_provisioned_disk_iops: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_provisioned_disk_throughput: Option<PrimField<String>>,
}
impl DataComputeStoragePoolStatusEl {
    #[doc = "Set the field `disk_count`.\n"]
    pub fn set_disk_count(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_count = Some(v.into());
        self
    }
    #[doc = "Set the field `last_resize_timestamp`.\n"]
    pub fn set_last_resize_timestamp(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_resize_timestamp = Some(v.into());
        self
    }
    #[doc = "Set the field `max_total_provisioned_disk_capacity_gb`.\n"]
    pub fn set_max_total_provisioned_disk_capacity_gb(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.max_total_provisioned_disk_capacity_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `pool_used_capacity_bytes`.\n"]
    pub fn set_pool_used_capacity_bytes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pool_used_capacity_bytes = Some(v.into());
        self
    }
    #[doc = "Set the field `pool_used_iops`.\n"]
    pub fn set_pool_used_iops(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pool_used_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `pool_used_throughput`.\n"]
    pub fn set_pool_used_throughput(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pool_used_throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `pool_user_written_bytes`.\n"]
    pub fn set_pool_user_written_bytes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.pool_user_written_bytes = Some(v.into());
        self
    }
    #[doc = "Set the field `total_provisioned_disk_capacity_gb`.\n"]
    pub fn set_total_provisioned_disk_capacity_gb(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.total_provisioned_disk_capacity_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `total_provisioned_disk_iops`.\n"]
    pub fn set_total_provisioned_disk_iops(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.total_provisioned_disk_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `total_provisioned_disk_throughput`.\n"]
    pub fn set_total_provisioned_disk_throughput(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.total_provisioned_disk_throughput = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeStoragePoolStatusEl {
    type O = BlockAssignable<DataComputeStoragePoolStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeStoragePoolStatusEl {}
impl BuildDataComputeStoragePoolStatusEl {
    pub fn build(self) -> DataComputeStoragePoolStatusEl {
        DataComputeStoragePoolStatusEl {
            disk_count: core::default::Default::default(),
            last_resize_timestamp: core::default::Default::default(),
            max_total_provisioned_disk_capacity_gb: core::default::Default::default(),
            pool_used_capacity_bytes: core::default::Default::default(),
            pool_used_iops: core::default::Default::default(),
            pool_used_throughput: core::default::Default::default(),
            pool_user_written_bytes: core::default::Default::default(),
            total_provisioned_disk_capacity_gb: core::default::Default::default(),
            total_provisioned_disk_iops: core::default::Default::default(),
            total_provisioned_disk_throughput: core::default::Default::default(),
        }
    }
}
pub struct DataComputeStoragePoolStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeStoragePoolStatusElRef {
    fn new(shared: StackShared, base: String) -> DataComputeStoragePoolStatusElRef {
        DataComputeStoragePoolStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeStoragePoolStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_count` after provisioning.\n"]
    pub fn disk_count(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_count", self.base))
    }
    #[doc = "Get a reference to the value of field `last_resize_timestamp` after provisioning.\n"]
    pub fn last_resize_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_resize_timestamp", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_total_provisioned_disk_capacity_gb` after provisioning.\n"]
    pub fn max_total_provisioned_disk_capacity_gb(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_total_provisioned_disk_capacity_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pool_used_capacity_bytes` after provisioning.\n"]
    pub fn pool_used_capacity_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_used_capacity_bytes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pool_used_iops` after provisioning.\n"]
    pub fn pool_used_iops(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_used_iops", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pool_used_throughput` after provisioning.\n"]
    pub fn pool_used_throughput(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_used_throughput", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pool_user_written_bytes` after provisioning.\n"]
    pub fn pool_user_written_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_user_written_bytes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_provisioned_disk_capacity_gb` after provisioning.\n"]
    pub fn total_provisioned_disk_capacity_gb(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_provisioned_disk_capacity_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_provisioned_disk_iops` after provisioning.\n"]
    pub fn total_provisioned_disk_iops(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_provisioned_disk_iops", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_provisioned_disk_throughput` after provisioning.\n"]
    pub fn total_provisioned_disk_throughput(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_provisioned_disk_throughput", self.base),
        )
    }
}
