use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeStoragePoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_provisioning_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performance_provisioning_type: Option<PrimField<String>>,
    pool_provisioned_capacity_gb: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool_provisioned_iops: Option<PrimField<String>>,
    pool_provisioned_throughput: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    storage_pool_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Vec<ComputeStoragePoolParamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeStoragePoolTimeoutsEl>,
    dynamic: ComputeStoragePoolDynamic,
}
struct ComputeStoragePool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeStoragePoolData>,
}
#[derive(Clone)]
pub struct ComputeStoragePool(Rc<ComputeStoragePool_>);
impl ComputeStoragePool {
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
    #[doc = "Set the field `capacity_provisioning_type`.\nProvisioning type of the byte capacity of the pool. Possible values: [\"STANDARD\", \"ADVANCED\"]"]
    pub fn set_capacity_provisioning_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().capacity_provisioning_type = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the StoragePool.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the StoragePool will fail.\nWhen the field is set to false, deleting the StoragePool is allowed."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA description of this resource. Provide this property when you create the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels to apply to this storage pool. These can be later modified by the setLabels method.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `performance_provisioning_type`.\nProvisioning type of the performance-related parameters of the pool, such as throughput and IOPS. Possible values: [\"STANDARD\", \"ADVANCED\"]"]
    pub fn set_performance_provisioning_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().performance_provisioning_type = Some(v.into());
        self
    }
    #[doc = "Set the field `pool_provisioned_iops`.\nProvisioned IOPS of the storage pool.\nOnly relevant if the storage pool type is 'hyperdisk-balanced'."]
    pub fn set_pool_provisioned_iops(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().pool_provisioned_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nA reference to the zone where the storage pool resides."]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }
    #[doc = "Set the field `params`.\n"]
    pub fn set_params(self, v: impl Into<BlockAssignable<ComputeStoragePoolParamsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().params = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.params = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeStoragePoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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
    pub fn resource_status(&self) -> ListRef<ComputeStoragePoolResourceStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus information for the storage pool resource."]
    pub fn status(&self) -> ListRef<ComputeStoragePoolStatusElRef> {
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
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeStoragePoolParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeStoragePoolTimeoutsElRef {
        ComputeStoragePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeStoragePool {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeStoragePool {}
impl ToListMappable for ComputeStoragePool {
    type O = ListRef<ComputeStoragePoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeStoragePool_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_storage_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeStoragePool {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is created.\nThe name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter,\nand all following characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc = "Size, in GiB, of the storage pool. For more information about the size limits,\nsee https://cloud.google.com/compute/docs/disks/storage-pools."]
    pub pool_provisioned_capacity_gb: PrimField<String>,
    #[doc = "Provisioned throughput, in MB/s, of the storage pool.\nOnly relevant if the storage pool type is 'hyperdisk-balanced' or 'hyperdisk-throughput'."]
    pub pool_provisioned_throughput: PrimField<String>,
    #[doc = "Type of the storage pool. For example, the\nfollowing are valid values:\n\n* 'https://www.googleapis.com/compute/v1/projects/{project_id}/zones/{zone}/storagePoolTypes/hyperdisk-balanced'\n* 'hyperdisk-throughput'"]
    pub storage_pool_type: PrimField<String>,
}
impl BuildComputeStoragePool {
    pub fn build(self, stack: &mut Stack) -> ComputeStoragePool {
        let out = ComputeStoragePool(Rc::new(ComputeStoragePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeStoragePoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                capacity_provisioning_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                description: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                performance_provisioning_type: core::default::Default::default(),
                pool_provisioned_capacity_gb: self.pool_provisioned_capacity_gb,
                pool_provisioned_iops: core::default::Default::default(),
                pool_provisioned_throughput: self.pool_provisioned_throughput,
                project: core::default::Default::default(),
                storage_pool_type: self.storage_pool_type,
                zone: core::default::Default::default(),
                params: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeStoragePoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeStoragePoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeStoragePoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    pub fn resource_status(&self) -> ListRef<ComputeStoragePoolResourceStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus information for the storage pool resource."]
    pub fn status(&self) -> ListRef<ComputeStoragePoolStatusElRef> {
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
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeStoragePoolParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeStoragePoolTimeoutsElRef {
        ComputeStoragePoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeStoragePoolResourceStatusEl {
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
impl ComputeStoragePoolResourceStatusEl {
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
impl ToListMappable for ComputeStoragePoolResourceStatusEl {
    type O = BlockAssignable<ComputeStoragePoolResourceStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeStoragePoolResourceStatusEl {}
impl BuildComputeStoragePoolResourceStatusEl {
    pub fn build(self) -> ComputeStoragePoolResourceStatusEl {
        ComputeStoragePoolResourceStatusEl {
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
pub struct ComputeStoragePoolResourceStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeStoragePoolResourceStatusElRef {
    fn new(shared: StackShared, base: String) -> ComputeStoragePoolResourceStatusElRef {
        ComputeStoragePoolResourceStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeStoragePoolResourceStatusElRef {
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
pub struct ComputeStoragePoolStatusEl {
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
impl ComputeStoragePoolStatusEl {
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
impl ToListMappable for ComputeStoragePoolStatusEl {
    type O = BlockAssignable<ComputeStoragePoolStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeStoragePoolStatusEl {}
impl BuildComputeStoragePoolStatusEl {
    pub fn build(self) -> ComputeStoragePoolStatusEl {
        ComputeStoragePoolStatusEl {
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
pub struct ComputeStoragePoolStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeStoragePoolStatusElRef {
    fn new(shared: StackShared, base: String) -> ComputeStoragePoolStatusElRef {
        ComputeStoragePoolStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeStoragePoolStatusElRef {
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
pub struct ComputeStoragePoolParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl ComputeStoragePoolParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the storage pool. Tag keys and values have the\nsame definition as resource manager tags. Keys and values can be either in numeric format,\nsuch as tagKeys/{tag_key_id} and tagValues/{tag_value_id} or in namespaced format such as\n{org_id|projectId}/{tag_key_short_name} and {tag_value_short_name}. The field is ignored when empty.\nThe field is immutable and causes resource replacement when mutated. This field is only\nset at create time and modifying this field after creation will trigger recreation.\nTo apply tags to an existing resource, see the google_tags_tag_binding resource."]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeStoragePoolParamsEl {
    type O = BlockAssignable<ComputeStoragePoolParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeStoragePoolParamsEl {}
impl BuildComputeStoragePoolParamsEl {
    pub fn build(self) -> ComputeStoragePoolParamsEl {
        ComputeStoragePoolParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct ComputeStoragePoolParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeStoragePoolParamsElRef {
    fn new(shared: StackShared, base: String) -> ComputeStoragePoolParamsElRef {
        ComputeStoragePoolParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeStoragePoolParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the storage pool. Tag keys and values have the\nsame definition as resource manager tags. Keys and values can be either in numeric format,\nsuch as tagKeys/{tag_key_id} and tagValues/{tag_value_id} or in namespaced format such as\n{org_id|projectId}/{tag_key_short_name} and {tag_value_short_name}. The field is ignored when empty.\nThe field is immutable and causes resource replacement when mutated. This field is only\nset at create time and modifying this field after creation will trigger recreation.\nTo apply tags to an existing resource, see the google_tags_tag_binding resource."]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeStoragePoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeStoragePoolTimeoutsEl {
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
impl ToListMappable for ComputeStoragePoolTimeoutsEl {
    type O = BlockAssignable<ComputeStoragePoolTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeStoragePoolTimeoutsEl {}
impl BuildComputeStoragePoolTimeoutsEl {
    pub fn build(self) -> ComputeStoragePoolTimeoutsEl {
        ComputeStoragePoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeStoragePoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeStoragePoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeStoragePoolTimeoutsElRef {
        ComputeStoragePoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeStoragePoolTimeoutsElRef {
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
struct ComputeStoragePoolDynamic {
    params: Option<DynamicBlock<ComputeStoragePoolParamsEl>>,
}
