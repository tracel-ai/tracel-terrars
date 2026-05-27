use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataprocClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    graceful_decommission_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_config: Option<Vec<DataprocClusterClusterConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataprocClusterTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_cluster_config: Option<Vec<DataprocClusterVirtualClusterConfigEl>>,
    dynamic: DataprocClusterDynamic,
}
struct DataprocCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataprocClusterData>,
}
#[derive(Clone)]
pub struct DataprocCluster(Rc<DataprocCluster_>);
impl DataprocCluster {
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
    #[doc = "Set the field `graceful_decommission_timeout`.\nThe timeout duration which allows graceful decomissioning when you change the number of worker nodes directly through a terraform apply"]
    pub fn set_graceful_decommission_timeout(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().graceful_decommission_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe list of the labels (key/value pairs) configured on the resource and to be applied to instances in the cluster.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project in which the cluster will exist. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region in which the cluster and associated nodes will be created in. Defaults to global."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_config`.\n"]
    pub fn set_cluster_config(
        self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cluster_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cluster_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataprocClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `virtual_cluster_config`.\n"]
    pub fn set_virtual_cluster_config(
        self,
        v: impl Into<BlockAssignable<DataprocClusterVirtualClusterConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().virtual_cluster_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.virtual_cluster_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `graceful_decommission_timeout` after provisioning.\nThe timeout duration which allows graceful decomissioning when you change the number of worker nodes directly through a terraform apply"]
    pub fn graceful_decommission_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.graceful_decommission_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe list of the labels (key/value pairs) configured on the resource and to be applied to instances in the cluster.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster, unique within the project and zone."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the cluster will exist. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region in which the cluster and associated nodes will be created in. Defaults to global."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> ListRef<DataprocClusterClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocClusterTimeoutsElRef {
        DataprocClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `virtual_cluster_config` after provisioning.\n"]
    pub fn virtual_cluster_config(&self) -> ListRef<DataprocClusterVirtualClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_cluster_config", self.extract_ref()),
        )
    }
}
impl Referable for DataprocCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataprocCluster {}
impl ToListMappable for DataprocCluster {
    type O = ListRef<DataprocClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataprocCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_dataproc_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataprocCluster {
    pub tf_id: String,
    #[doc = "The name of the cluster, unique within the project and zone."]
    pub name: PrimField<String>,
}
impl BuildDataprocCluster {
    pub fn build(self, stack: &mut Stack) -> DataprocCluster {
        let out = DataprocCluster(Rc::new(DataprocCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataprocClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                graceful_decommission_timeout: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                cluster_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                virtual_cluster_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataprocClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataprocClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `graceful_decommission_timeout` after provisioning.\nThe timeout duration which allows graceful decomissioning when you change the number of worker nodes directly through a terraform apply"]
    pub fn graceful_decommission_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.graceful_decommission_timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe list of the labels (key/value pairs) configured on the resource and to be applied to instances in the cluster.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster, unique within the project and zone."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the cluster will exist. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region in which the cluster and associated nodes will be created in. Defaults to global."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> ListRef<DataprocClusterClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocClusterTimeoutsElRef {
        DataprocClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `virtual_cluster_config` after provisioning.\n"]
    pub fn virtual_cluster_config(&self) -> ListRef<DataprocClusterVirtualClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_cluster_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElAutoscalingConfigEl {
    policy_uri: PrimField<String>,
}
impl DataprocClusterClusterConfigElAutoscalingConfigEl {}
impl ToListMappable for DataprocClusterClusterConfigElAutoscalingConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElAutoscalingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElAutoscalingConfigEl {
    #[doc = "The autoscaling policy used by the cluster."]
    pub policy_uri: PrimField<String>,
}
impl BuildDataprocClusterClusterConfigElAutoscalingConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElAutoscalingConfigEl {
        DataprocClusterClusterConfigElAutoscalingConfigEl {
            policy_uri: self.policy_uri,
        }
    }
}
pub struct DataprocClusterClusterConfigElAutoscalingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElAutoscalingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElAutoscalingConfigElRef {
        DataprocClusterClusterConfigElAutoscalingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElAutoscalingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `policy_uri` after provisioning.\nThe autoscaling policy used by the cluster."]
    pub fn policy_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElAcceleratorsEl
{
    accelerator_count: PrimField<f64>,
    accelerator_type: PrimField<String>,
}
impl DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElAcceleratorsEl {}
impl ToListMappable for DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElAcceleratorsEl { type O = BlockAssignable < DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElAcceleratorsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElAcceleratorsEl
{
    #[doc = "The number of the accelerator cards of this type exposed to this instance. Often restricted to one of 1, 2, 4, or 8."]
    pub accelerator_count: PrimField<f64>,
    #[doc = "The short name of the accelerator type to expose to this instance. For example, nvidia-tesla-k80."]
    pub accelerator_type: PrimField<String>,
}
impl BuildDataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElAcceleratorsEl { pub fn build (self) -> DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElAcceleratorsEl { DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElAcceleratorsEl { accelerator_count : self . accelerator_count , accelerator_type : self . accelerator_type , } } }
pub struct DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElAcceleratorsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElAcceleratorsElRef { fn new (shared : StackShared , base : String) -> DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElAcceleratorsElRef { DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElAcceleratorsElRef { shared : shared , base : base . to_string () , } } }
impl
    DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElAcceleratorsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of the accelerator cards of this type exposed to this instance. Often restricted to one of 1, 2, 4, or 8."]
    pub fn accelerator_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\nThe short name of the accelerator type to expose to this instance. For example, nvidia-tesla-k80."]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDiskConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_provisioned_iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_provisioned_throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_interface: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_local_ssds: Option<PrimField<f64>>,
}
impl DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDiskConfigEl {
    #[doc = "Set the field `boot_disk_provisioned_iops`.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn set_boot_disk_provisioned_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk_provisioned_throughput`.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn set_boot_disk_provisioned_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk_size_gb`.\nSize of the primary disk attached to each node, specified in GB. The primary disk contains the boot volume and system libraries, and the smallest allowed disk size is 10GB. GCP will default to a predetermined computed value if not set (currently 500GB). Note: If SSDs are not attached, it also contains the HDFS data blocks and Hadoop working directories."]
    pub fn set_boot_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk_type`.\nThe disk type of the primary disk attached to each node. Such as \"pd-ssd\" or \"pd-standard\". Defaults to \"pd-standard\"."]
    pub fn set_boot_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.boot_disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `local_ssd_interface`.\nInterface type of local SSDs (default is \"scsi\"). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn set_local_ssd_interface(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_ssd_interface = Some(v.into());
        self
    }
    #[doc = "Set the field `num_local_ssds`.\nThe amount of local SSD disks that will be attached to each master cluster node. Defaults to 0."]
    pub fn set_num_local_ssds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_local_ssds = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDiskConfigEl
{
    type O = BlockAssignable<
        DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDiskConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDiskConfigEl
{}
impl
    BuildDataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDiskConfigEl
{
    pub fn build(
        self,
    ) -> DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDiskConfigEl
    {
        DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDiskConfigEl { boot_disk_provisioned_iops : core :: default :: Default :: default () , boot_disk_provisioned_throughput : core :: default :: Default :: default () , boot_disk_size_gb : core :: default :: Default :: default () , boot_disk_type : core :: default :: Default :: default () , local_ssd_interface : core :: default :: Default :: default () , num_local_ssds : core :: default :: Default :: default () , }
    }
}
pub struct DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDiskConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDiskConfigElRef { fn new (shared : StackShared , base : String) -> DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDiskConfigElRef { DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDiskConfigElRef { shared : shared , base : base . to_string () , } } }
impl
    DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDiskConfigElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `boot_disk_provisioned_iops` after provisioning.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn boot_disk_provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_provisioned_iops", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk_provisioned_throughput` after provisioning.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn boot_disk_provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_provisioned_throughput", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nSize of the primary disk attached to each node, specified in GB. The primary disk contains the boot volume and system libraries, and the smallest allowed disk size is 10GB. GCP will default to a predetermined computed value if not set (currently 500GB). Note: If SSDs are not attached, it also contains the HDFS data blocks and Hadoop working directories."]
    pub fn boot_disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk_type` after provisioning.\nThe disk type of the primary disk attached to each node. Such as \"pd-ssd\" or \"pd-standard\". Defaults to \"pd-standard\"."]
    pub fn boot_disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_ssd_interface` after provisioning.\nInterface type of local SSDs (default is \"scsi\"). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn local_ssd_interface(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_ssd_interface", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `num_local_ssds` after provisioning.\nThe amount of local SSD disks that will be attached to each master cluster node. Defaults to 0."]
    pub fn num_local_ssds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_local_ssds", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDynamic { accelerators : Option < DynamicBlock < DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElAcceleratorsEl >> , disk_config : Option < DynamicBlock < DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDiskConfigEl >> , }
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigEl { # [serde (skip_serializing_if = "Option::is_none")] machine_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] min_cpu_platform : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] num_instances : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] accelerators : Option < Vec < DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElAcceleratorsEl > > , # [serde (skip_serializing_if = "Option::is_none")] disk_config : Option < Vec < DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDiskConfigEl > > , dynamic : DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDynamic , }
impl DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigEl {
    #[doc = "Set the field `machine_type`.\nThe name of a Google Compute Engine machine type to create for the master"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `min_cpu_platform`.\nThe name of a minimum generation of CPU family for the auxiliary node group. If not specified, GCP will default to a predetermined computed value for each zone."]
    pub fn set_min_cpu_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_cpu_platform = Some(v.into());
        self
    }
    #[doc = "Set the field `num_instances`.\nSpecifies the number of auxiliary nodes to create. If not specified, GCP will default to a predetermined computed value."]
    pub fn set_num_instances(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        mut self,
        v : impl Into < BlockAssignable < DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElAcceleratorsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.accelerators = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerators = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `disk_config`.\n"]
    pub fn set_disk_config(
        mut self,
        v : impl Into < BlockAssignable < DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDiskConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.disk_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigEl
{
    type O = BlockAssignable<
        DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigEl {}
impl BuildDataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigEl {
    pub fn build(
        self,
    ) -> DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigEl {
        DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigEl {
            machine_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            num_instances: core::default::Default::default(),
            accelerators: core::default::Default::default(),
            disk_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElRef {
        DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_names` after provisioning.\nList of auxiliary node group instance names which have been assigned to the cluster."]
    pub fn instance_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe name of a Google Compute Engine machine type to create for the master"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `min_cpu_platform` after provisioning.\nThe name of a minimum generation of CPU family for the auxiliary node group. If not specified, GCP will default to a predetermined computed value for each zone."]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_cpu_platform", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `num_instances` after provisioning.\nSpecifies the number of auxiliary nodes to create. If not specified, GCP will default to a predetermined computed value."]
    pub fn num_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_config` after provisioning.\n"]    pub fn disk_config (& self) -> ListRef < DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElDiskConfigElRef >{
        ListRef::new(self.shared().clone(), format!("{}.disk_config", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElDynamic {
    node_group_config: Option<
        DynamicBlock<
            DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupEl {
    roles: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_group_config: Option<
        Vec<DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigEl>,
    >,
    dynamic: DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElDynamic,
}
impl DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupEl {
    #[doc = "Set the field `node_group_config`.\n"]
    pub fn set_node_group_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_group_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_group_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupEl {
    #[doc = "Node group roles."]
    pub roles: ListField<PrimField<String>>,
}
impl BuildDataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupEl {
    pub fn build(self) -> DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupEl {
        DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupEl {
            roles: self.roles,
            node_group_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElRef {
        DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe Node group resource name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `roles` after provisioning.\nNode group roles."]
    pub fn roles(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }
    #[doc = "Get a reference to the value of field `node_group_config` after provisioning.\n"]
    pub fn node_group_config(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElNodeGroupConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_group_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocClusterClusterConfigElAuxiliaryNodeGroupsElDynamic {
    node_group:
        Option<DynamicBlock<DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupEl>>,
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElAuxiliaryNodeGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_group: Option<Vec<DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupEl>>,
    dynamic: DataprocClusterClusterConfigElAuxiliaryNodeGroupsElDynamic,
}
impl DataprocClusterClusterConfigElAuxiliaryNodeGroupsEl {
    #[doc = "Set the field `node_group_id`.\nA node group ID. Generated if not specified. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of from 3 to 33 characters."]
    pub fn set_node_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_group_id = Some(v.into());
        self
    }
    #[doc = "Set the field `node_group`.\n"]
    pub fn set_node_group(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_group = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElAuxiliaryNodeGroupsEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElAuxiliaryNodeGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElAuxiliaryNodeGroupsEl {}
impl BuildDataprocClusterClusterConfigElAuxiliaryNodeGroupsEl {
    pub fn build(self) -> DataprocClusterClusterConfigElAuxiliaryNodeGroupsEl {
        DataprocClusterClusterConfigElAuxiliaryNodeGroupsEl {
            node_group_id: core::default::Default::default(),
            node_group: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElAuxiliaryNodeGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElAuxiliaryNodeGroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElAuxiliaryNodeGroupsElRef {
        DataprocClusterClusterConfigElAuxiliaryNodeGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElAuxiliaryNodeGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `node_group_id` after provisioning.\nA node group ID. Generated if not specified. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of from 3 to 33 characters."]
    pub fn node_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_group_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_group` after provisioning.\n"]
    pub fn node_group(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElAuxiliaryNodeGroupsElNodeGroupElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_group", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElDataprocMetricConfigElMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_overrides: Option<SetField<PrimField<String>>>,
    metric_source: PrimField<String>,
}
impl DataprocClusterClusterConfigElDataprocMetricConfigElMetricsEl {
    #[doc = "Set the field `metric_overrides`.\nSpecify one or more [available OSS metrics] (https://cloud.google.com/dataproc/docs/guides/monitoring#available_oss_metrics) to collect."]
    pub fn set_metric_overrides(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.metric_overrides = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElDataprocMetricConfigElMetricsEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElDataprocMetricConfigElMetricsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElDataprocMetricConfigElMetricsEl {
    #[doc = "A source for the collection of Dataproc OSS metrics (see [available OSS metrics] (https://cloud.google.com//dataproc/docs/guides/monitoring#available_oss_metrics))."]
    pub metric_source: PrimField<String>,
}
impl BuildDataprocClusterClusterConfigElDataprocMetricConfigElMetricsEl {
    pub fn build(self) -> DataprocClusterClusterConfigElDataprocMetricConfigElMetricsEl {
        DataprocClusterClusterConfigElDataprocMetricConfigElMetricsEl {
            metric_overrides: core::default::Default::default(),
            metric_source: self.metric_source,
        }
    }
}
pub struct DataprocClusterClusterConfigElDataprocMetricConfigElMetricsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElDataprocMetricConfigElMetricsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElDataprocMetricConfigElMetricsElRef {
        DataprocClusterClusterConfigElDataprocMetricConfigElMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElDataprocMetricConfigElMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metric_overrides` after provisioning.\nSpecify one or more [available OSS metrics] (https://cloud.google.com/dataproc/docs/guides/monitoring#available_oss_metrics) to collect."]
    pub fn metric_overrides(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.metric_overrides", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metric_source` after provisioning.\nA source for the collection of Dataproc OSS metrics (see [available OSS metrics] (https://cloud.google.com//dataproc/docs/guides/monitoring#available_oss_metrics))."]
    pub fn metric_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metric_source", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocClusterClusterConfigElDataprocMetricConfigElDynamic {
    metrics: Option<DynamicBlock<DataprocClusterClusterConfigElDataprocMetricConfigElMetricsEl>>,
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElDataprocMetricConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics: Option<Vec<DataprocClusterClusterConfigElDataprocMetricConfigElMetricsEl>>,
    dynamic: DataprocClusterClusterConfigElDataprocMetricConfigElDynamic,
}
impl DataprocClusterClusterConfigElDataprocMetricConfigEl {
    #[doc = "Set the field `metrics`.\n"]
    pub fn set_metrics(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElDataprocMetricConfigElMetricsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metrics = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metrics = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElDataprocMetricConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElDataprocMetricConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElDataprocMetricConfigEl {}
impl BuildDataprocClusterClusterConfigElDataprocMetricConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElDataprocMetricConfigEl {
        DataprocClusterClusterConfigElDataprocMetricConfigEl {
            metrics: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElDataprocMetricConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElDataprocMetricConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElDataprocMetricConfigElRef {
        DataprocClusterClusterConfigElDataprocMetricConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElDataprocMetricConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metrics` after provisioning.\n"]
    pub fn metrics(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElDataprocMetricConfigElMetricsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metrics", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElEncryptionConfigEl {
    kms_key_name: PrimField<String>,
}
impl DataprocClusterClusterConfigElEncryptionConfigEl {}
impl ToListMappable for DataprocClusterClusterConfigElEncryptionConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElEncryptionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElEncryptionConfigEl {
    #[doc = "The Cloud KMS key name to use for PD disk encryption for all instances in the cluster."]
    pub kms_key_name: PrimField<String>,
}
impl BuildDataprocClusterClusterConfigElEncryptionConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElEncryptionConfigEl {
        DataprocClusterClusterConfigElEncryptionConfigEl {
            kms_key_name: self.kms_key_name,
        }
    }
}
pub struct DataprocClusterClusterConfigElEncryptionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElEncryptionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElEncryptionConfigElRef {
        DataprocClusterClusterConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe Cloud KMS key name to use for PD disk encryption for all instances in the cluster."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElEndpointConfigEl {
    enable_http_port_access: PrimField<bool>,
}
impl DataprocClusterClusterConfigElEndpointConfigEl {}
impl ToListMappable for DataprocClusterClusterConfigElEndpointConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElEndpointConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElEndpointConfigEl {
    #[doc = "The flag to enable http access to specific ports on the cluster from external sources (aka Component Gateway). Defaults to false."]
    pub enable_http_port_access: PrimField<bool>,
}
impl BuildDataprocClusterClusterConfigElEndpointConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElEndpointConfigEl {
        DataprocClusterClusterConfigElEndpointConfigEl {
            enable_http_port_access: self.enable_http_port_access,
        }
    }
}
pub struct DataprocClusterClusterConfigElEndpointConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElEndpointConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocClusterClusterConfigElEndpointConfigElRef {
        DataprocClusterClusterConfigElEndpointConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElEndpointConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_http_port_access` after provisioning.\nThe flag to enable http access to specific ports on the cluster from external sources (aka Component Gateway). Defaults to false."]
    pub fn enable_http_port_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_http_port_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `http_ports` after provisioning.\nThe map of port descriptions to URLs. Will only be populated if enable_http_port_access is true."]
    pub fn http_ports(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.http_ports", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElGceClusterConfigElConfidentialInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_confidential_compute: Option<PrimField<bool>>,
}
impl DataprocClusterClusterConfigElGceClusterConfigElConfidentialInstanceConfigEl {
    #[doc = "Set the field `enable_confidential_compute`.\nDefines whether the instance should have confidential compute enabled."]
    pub fn set_enable_confidential_compute(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_confidential_compute = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataprocClusterClusterConfigElGceClusterConfigElConfidentialInstanceConfigEl
{
    type O = BlockAssignable<
        DataprocClusterClusterConfigElGceClusterConfigElConfidentialInstanceConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElGceClusterConfigElConfidentialInstanceConfigEl {}
impl BuildDataprocClusterClusterConfigElGceClusterConfigElConfidentialInstanceConfigEl {
    pub fn build(
        self,
    ) -> DataprocClusterClusterConfigElGceClusterConfigElConfidentialInstanceConfigEl {
        DataprocClusterClusterConfigElGceClusterConfigElConfidentialInstanceConfigEl {
            enable_confidential_compute: core::default::Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElGceClusterConfigElConfidentialInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElGceClusterConfigElConfidentialInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElGceClusterConfigElConfidentialInstanceConfigElRef {
        DataprocClusterClusterConfigElGceClusterConfigElConfidentialInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElGceClusterConfigElConfidentialInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_confidential_compute` after provisioning.\nDefines whether the instance should have confidential compute enabled."]
    pub fn enable_confidential_compute(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_confidential_compute", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElGceClusterConfigElNodeGroupAffinityEl {
    node_group_uri: PrimField<String>,
}
impl DataprocClusterClusterConfigElGceClusterConfigElNodeGroupAffinityEl {}
impl ToListMappable for DataprocClusterClusterConfigElGceClusterConfigElNodeGroupAffinityEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElGceClusterConfigElNodeGroupAffinityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElGceClusterConfigElNodeGroupAffinityEl {
    #[doc = "The URI of a sole-tenant that the cluster will be created on."]
    pub node_group_uri: PrimField<String>,
}
impl BuildDataprocClusterClusterConfigElGceClusterConfigElNodeGroupAffinityEl {
    pub fn build(self) -> DataprocClusterClusterConfigElGceClusterConfigElNodeGroupAffinityEl {
        DataprocClusterClusterConfigElGceClusterConfigElNodeGroupAffinityEl {
            node_group_uri: self.node_group_uri,
        }
    }
}
pub struct DataprocClusterClusterConfigElGceClusterConfigElNodeGroupAffinityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElGceClusterConfigElNodeGroupAffinityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElGceClusterConfigElNodeGroupAffinityElRef {
        DataprocClusterClusterConfigElGceClusterConfigElNodeGroupAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElGceClusterConfigElNodeGroupAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `node_group_uri` after provisioning.\nThe URI of a sole-tenant that the cluster will be created on."]
    pub fn node_group_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_group_uri", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElGceClusterConfigElReservationAffinityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    consume_reservation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}
impl DataprocClusterClusterConfigElGceClusterConfigElReservationAffinityEl {
    #[doc = "Set the field `consume_reservation_type`.\nType of reservation to consume."]
    pub fn set_consume_reservation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consume_reservation_type = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\nCorresponds to the label key of reservation resource."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\nCorresponds to the label values of reservation resource."]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElGceClusterConfigElReservationAffinityEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElGceClusterConfigElReservationAffinityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElGceClusterConfigElReservationAffinityEl {}
impl BuildDataprocClusterClusterConfigElGceClusterConfigElReservationAffinityEl {
    pub fn build(self) -> DataprocClusterClusterConfigElGceClusterConfigElReservationAffinityEl {
        DataprocClusterClusterConfigElGceClusterConfigElReservationAffinityEl {
            consume_reservation_type: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElGceClusterConfigElReservationAffinityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElGceClusterConfigElReservationAffinityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElGceClusterConfigElReservationAffinityElRef {
        DataprocClusterClusterConfigElGceClusterConfigElReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElGceClusterConfigElReservationAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `consume_reservation_type` after provisioning.\nType of reservation to consume."]
    pub fn consume_reservation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consume_reservation_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nCorresponds to the label key of reservation resource."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\nCorresponds to the label values of reservation resource."]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElGceClusterConfigElShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_vtpm: Option<PrimField<bool>>,
}
impl DataprocClusterClusterConfigElGceClusterConfigElShieldedInstanceConfigEl {
    #[doc = "Set the field `enable_integrity_monitoring`.\nDefines whether instances have integrity monitoring enabled."]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_secure_boot`.\nDefines whether instances have Secure Boot enabled."]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_vtpm`.\nDefines whether instances have the vTPM enabled."]
    pub fn set_enable_vtpm(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_vtpm = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElGceClusterConfigElShieldedInstanceConfigEl {
    type O =
        BlockAssignable<DataprocClusterClusterConfigElGceClusterConfigElShieldedInstanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElGceClusterConfigElShieldedInstanceConfigEl {}
impl BuildDataprocClusterClusterConfigElGceClusterConfigElShieldedInstanceConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElGceClusterConfigElShieldedInstanceConfigEl {
        DataprocClusterClusterConfigElGceClusterConfigElShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElGceClusterConfigElShieldedInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElGceClusterConfigElShieldedInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElGceClusterConfigElShieldedInstanceConfigElRef {
        DataprocClusterClusterConfigElGceClusterConfigElShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElGceClusterConfigElShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\nDefines whether instances have integrity monitoring enabled."]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_integrity_monitoring", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_secure_boot` after provisioning.\nDefines whether instances have Secure Boot enabled."]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_secure_boot", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_vtpm` after provisioning.\nDefines whether instances have the vTPM enabled."]
    pub fn enable_vtpm(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataprocClusterClusterConfigElGceClusterConfigElDynamic {
    confidential_instance_config: Option<
        DynamicBlock<DataprocClusterClusterConfigElGceClusterConfigElConfidentialInstanceConfigEl>,
    >,
    node_group_affinity:
        Option<DynamicBlock<DataprocClusterClusterConfigElGceClusterConfigElNodeGroupAffinityEl>>,
    reservation_affinity:
        Option<DynamicBlock<DataprocClusterClusterConfigElGceClusterConfigElReservationAffinityEl>>,
    shielded_instance_config: Option<
        DynamicBlock<DataprocClusterClusterConfigElGceClusterConfigElShieldedInstanceConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElGceClusterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ip_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_scopes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_instance_config:
        Option<Vec<DataprocClusterClusterConfigElGceClusterConfigElConfidentialInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_group_affinity:
        Option<Vec<DataprocClusterClusterConfigElGceClusterConfigElNodeGroupAffinityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation_affinity:
        Option<Vec<DataprocClusterClusterConfigElGceClusterConfigElReservationAffinityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shielded_instance_config:
        Option<Vec<DataprocClusterClusterConfigElGceClusterConfigElShieldedInstanceConfigEl>>,
    dynamic: DataprocClusterClusterConfigElGceClusterConfigElDynamic,
}
impl DataprocClusterClusterConfigElGceClusterConfigEl {
    #[doc = "Set the field `internal_ip_only`.\nBy default, clusters are not restricted to internal IP addresses, and will have ephemeral external IP addresses assigned to each instance. If set to true, all instances in the cluster will only have internal IP addresses. Note: Private Google Access (also known as privateIpGoogleAccess) must be enabled on the subnetwork that the cluster will be launched in."]
    pub fn set_internal_ip_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.internal_ip_only = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\nA map of the Compute Engine metadata entries to add to all instances"]
    pub fn set_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThe name or self_link of the Google Compute Engine network to the cluster will be part of. Conflicts with subnetwork. If neither is specified, this defaults to the \"default\" network."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_manager_tags`.\nA map of resource manager tags to add to all instances. Keys must be in the format tagKeys/{tag_key_id} and values in the format tagValues/{tag_value_id}."]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nThe service account to be used by the Node VMs. If not specified, the \"default\" service account is used."]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_scopes`.\nThe set of Google API scopes to be made available on all of the node VMs under the service_account specified. These can be either FQDNs, or scope aliases."]
    pub fn set_service_account_scopes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.service_account_scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nThe name or self_link of the Google Compute Engine subnetwork the cluster will be part of. Conflicts with network."]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nThe list of instance tags applied to instances in the cluster. Tags are used to identify valid sources or targets for network firewalls."]
    pub fn set_tags(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\nThe GCP zone where your data is stored and used (i.e. where the master and the worker nodes will be created in). If region is set to 'global' (default) then zone is mandatory, otherwise GCP is able to make use of Auto Zone Placement to determine this automatically for you. Note: This setting additionally determines and restricts which computing resources are available for use with other configs such as cluster_config.master_config.machine_type and cluster_config.worker_config.machine_type."]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
    #[doc = "Set the field `confidential_instance_config`.\n"]
    pub fn set_confidential_instance_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataprocClusterClusterConfigElGceClusterConfigElConfidentialInstanceConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.confidential_instance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.confidential_instance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_group_affinity`.\n"]
    pub fn set_node_group_affinity(
        mut self,
        v: impl Into<
            BlockAssignable<DataprocClusterClusterConfigElGceClusterConfigElNodeGroupAffinityEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_group_affinity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_group_affinity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        mut self,
        v: impl Into<
            BlockAssignable<DataprocClusterClusterConfigElGceClusterConfigElReservationAffinityEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.reservation_affinity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.reservation_affinity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataprocClusterClusterConfigElGceClusterConfigElShieldedInstanceConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.shielded_instance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.shielded_instance_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElGceClusterConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElGceClusterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElGceClusterConfigEl {}
impl BuildDataprocClusterClusterConfigElGceClusterConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElGceClusterConfigEl {
        DataprocClusterClusterConfigElGceClusterConfigEl {
            internal_ip_only: core::default::Default::default(),
            metadata: core::default::Default::default(),
            network: core::default::Default::default(),
            resource_manager_tags: core::default::Default::default(),
            service_account: core::default::Default::default(),
            service_account_scopes: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
            zone: core::default::Default::default(),
            confidential_instance_config: core::default::Default::default(),
            node_group_affinity: core::default::Default::default(),
            reservation_affinity: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElGceClusterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElGceClusterConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElGceClusterConfigElRef {
        DataprocClusterClusterConfigElGceClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElGceClusterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `internal_ip_only` after provisioning.\nBy default, clusters are not restricted to internal IP addresses, and will have ephemeral external IP addresses assigned to each instance. If set to true, all instances in the cluster will only have internal IP addresses. Note: Private Google Access (also known as privateIpGoogleAccess) must be enabled on the subnetwork that the cluster will be launched in."]
    pub fn internal_ip_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.internal_ip_only", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nA map of the Compute Engine metadata entries to add to all instances"]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe name or self_link of the Google Compute Engine network to the cluster will be part of. Conflicts with subnetwork. If neither is specified, this defaults to the \"default\" network."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\nA map of resource manager tags to add to all instances. Keys must be in the format tagKeys/{tag_key_id} and values in the format tagValues/{tag_value_id}."]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe service account to be used by the Node VMs. If not specified, the \"default\" service account is used."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_scopes` after provisioning.\nThe set of Google API scopes to be made available on all of the node VMs under the service_account specified. These can be either FQDNs, or scope aliases."]
    pub fn service_account_scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.service_account_scopes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe name or self_link of the Google Compute Engine subnetwork the cluster will be part of. Conflicts with network."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nThe list of instance tags applied to instances in the cluster. Tags are used to identify valid sources or targets for network firewalls."]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe GCP zone where your data is stored and used (i.e. where the master and the worker nodes will be created in). If region is set to 'global' (default) then zone is mandatory, otherwise GCP is able to make use of Auto Zone Placement to determine this automatically for you. Note: This setting additionally determines and restricts which computing resources are available for use with other configs such as cluster_config.master_config.machine_type and cluster_config.worker_config.machine_type."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
    #[doc = "Get a reference to the value of field `confidential_instance_config` after provisioning.\n"]
    pub fn confidential_instance_config(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElGceClusterConfigElConfidentialInstanceConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confidential_instance_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_group_affinity` after provisioning.\n"]
    pub fn node_group_affinity(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElGceClusterConfigElNodeGroupAffinityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_group_affinity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElGceClusterConfigElReservationAffinityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reservation_affinity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElGceClusterConfigElShieldedInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElInitializationActionEl {
    script: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_sec: Option<PrimField<f64>>,
}
impl DataprocClusterClusterConfigElInitializationActionEl {
    #[doc = "Set the field `timeout_sec`.\nThe maximum duration (in seconds) which script is allowed to take to execute its action. GCP will default to a predetermined computed value if not set (currently 300)."]
    pub fn set_timeout_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_sec = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElInitializationActionEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElInitializationActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElInitializationActionEl {
    #[doc = "The script to be executed during initialization of the cluster. The script must be a GCS file with a gs:// prefix."]
    pub script: PrimField<String>,
}
impl BuildDataprocClusterClusterConfigElInitializationActionEl {
    pub fn build(self) -> DataprocClusterClusterConfigElInitializationActionEl {
        DataprocClusterClusterConfigElInitializationActionEl {
            script: self.script,
            timeout_sec: core::default::Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElInitializationActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElInitializationActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElInitializationActionElRef {
        DataprocClusterClusterConfigElInitializationActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElInitializationActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `script` after provisioning.\nThe script to be executed during initialization of the cluster. The script must be a GCS file with a gs:// prefix."]
    pub fn script(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.script", self.base))
    }
    #[doc = "Get a reference to the value of field `timeout_sec` after provisioning.\nThe maximum duration (in seconds) which script is allowed to take to execute its action. GCP will default to a predetermined computed value if not set (currently 300)."]
    pub fn timeout_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout_sec", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElLifecycleConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_delete_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_stop_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_delete_ttl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_stop_ttl: Option<PrimField<String>>,
}
impl DataprocClusterClusterConfigElLifecycleConfigEl {
    #[doc = "Set the field `auto_delete_time`.\nThe time when cluster will be auto-deleted. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_auto_delete_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_delete_time = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_stop_time`.\nThe time when cluster will be auto-stopped. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_auto_stop_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_stop_time = Some(v.into());
        self
    }
    #[doc = "Set the field `idle_delete_ttl`.\nThe duration to keep the cluster alive while idling (no jobs running). After this TTL, the cluster will be deleted. Valid range: [300s, 1209600s]."]
    pub fn set_idle_delete_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.idle_delete_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `idle_stop_ttl`.\nThe duration to keep the cluster started while idling (no jobs running). After this TTL, the cluster will be stopped. Valid range: [10m, 14d]."]
    pub fn set_idle_stop_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.idle_stop_ttl = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElLifecycleConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElLifecycleConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElLifecycleConfigEl {}
impl BuildDataprocClusterClusterConfigElLifecycleConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElLifecycleConfigEl {
        DataprocClusterClusterConfigElLifecycleConfigEl {
            auto_delete_time: core::default::Default::default(),
            auto_stop_time: core::default::Default::default(),
            idle_delete_ttl: core::default::Default::default(),
            idle_stop_ttl: core::default::Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElLifecycleConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElLifecycleConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElLifecycleConfigElRef {
        DataprocClusterClusterConfigElLifecycleConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElLifecycleConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_delete_time` after provisioning.\nThe time when cluster will be auto-deleted. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn auto_delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_delete_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auto_stop_time` after provisioning.\nThe time when cluster will be auto-stopped. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn auto_stop_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_stop_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `idle_delete_ttl` after provisioning.\nThe duration to keep the cluster alive while idling (no jobs running). After this TTL, the cluster will be deleted. Valid range: [300s, 1209600s]."]
    pub fn idle_delete_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_delete_ttl", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `idle_start_time` after provisioning.\nTime when the cluster became idle (most recent job finished) and became eligible for deletion due to idleness."]
    pub fn idle_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_start_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `idle_stop_ttl` after provisioning.\nThe duration to keep the cluster started while idling (no jobs running). After this TTL, the cluster will be stopped. Valid range: [10m, 14d]."]
    pub fn idle_stop_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_stop_ttl", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElMasterConfigElAcceleratorsEl {
    accelerator_count: PrimField<f64>,
    accelerator_type: PrimField<String>,
}
impl DataprocClusterClusterConfigElMasterConfigElAcceleratorsEl {}
impl ToListMappable for DataprocClusterClusterConfigElMasterConfigElAcceleratorsEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElMasterConfigElAcceleratorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElMasterConfigElAcceleratorsEl {
    #[doc = "The number of the accelerator cards of this type exposed to this instance. Often restricted to one of 1, 2, 4, or 8."]
    pub accelerator_count: PrimField<f64>,
    #[doc = "The short name of the accelerator type to expose to this instance. For example, nvidia-tesla-k80."]
    pub accelerator_type: PrimField<String>,
}
impl BuildDataprocClusterClusterConfigElMasterConfigElAcceleratorsEl {
    pub fn build(self) -> DataprocClusterClusterConfigElMasterConfigElAcceleratorsEl {
        DataprocClusterClusterConfigElMasterConfigElAcceleratorsEl {
            accelerator_count: self.accelerator_count,
            accelerator_type: self.accelerator_type,
        }
    }
}
pub struct DataprocClusterClusterConfigElMasterConfigElAcceleratorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElMasterConfigElAcceleratorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElMasterConfigElAcceleratorsElRef {
        DataprocClusterClusterConfigElMasterConfigElAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElMasterConfigElAcceleratorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of the accelerator cards of this type exposed to this instance. Often restricted to one of 1, 2, 4, or 8."]
    pub fn accelerator_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\nThe short name of the accelerator type to expose to this instance. For example, nvidia-tesla-k80."]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElMasterConfigElDiskConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_provisioned_iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_provisioned_throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_interface: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_local_ssds: Option<PrimField<f64>>,
}
impl DataprocClusterClusterConfigElMasterConfigElDiskConfigEl {
    #[doc = "Set the field `boot_disk_provisioned_iops`.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn set_boot_disk_provisioned_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk_provisioned_throughput`.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn set_boot_disk_provisioned_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk_size_gb`.\nSize of the primary disk attached to each node, specified in GB. The primary disk contains the boot volume and system libraries, and the smallest allowed disk size is 10GB. GCP will default to a predetermined computed value if not set (currently 500GB). Note: If SSDs are not attached, it also contains the HDFS data blocks and Hadoop working directories."]
    pub fn set_boot_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk_type`.\nThe disk type of the primary disk attached to each node. Such as \"pd-ssd\" or \"pd-standard\". Defaults to \"pd-standard\"."]
    pub fn set_boot_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.boot_disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `local_ssd_interface`.\nInterface type of local SSDs (default is \"scsi\"). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn set_local_ssd_interface(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_ssd_interface = Some(v.into());
        self
    }
    #[doc = "Set the field `num_local_ssds`.\nThe amount of local SSD disks that will be attached to each master cluster node. Defaults to 0."]
    pub fn set_num_local_ssds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_local_ssds = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElMasterConfigElDiskConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElMasterConfigElDiskConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElMasterConfigElDiskConfigEl {}
impl BuildDataprocClusterClusterConfigElMasterConfigElDiskConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElMasterConfigElDiskConfigEl {
        DataprocClusterClusterConfigElMasterConfigElDiskConfigEl {
            boot_disk_provisioned_iops: core::default::Default::default(),
            boot_disk_provisioned_throughput: core::default::Default::default(),
            boot_disk_size_gb: core::default::Default::default(),
            boot_disk_type: core::default::Default::default(),
            local_ssd_interface: core::default::Default::default(),
            num_local_ssds: core::default::Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElMasterConfigElDiskConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElMasterConfigElDiskConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElMasterConfigElDiskConfigElRef {
        DataprocClusterClusterConfigElMasterConfigElDiskConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElMasterConfigElDiskConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `boot_disk_provisioned_iops` after provisioning.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn boot_disk_provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_provisioned_iops", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk_provisioned_throughput` after provisioning.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn boot_disk_provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_provisioned_throughput", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nSize of the primary disk attached to each node, specified in GB. The primary disk contains the boot volume and system libraries, and the smallest allowed disk size is 10GB. GCP will default to a predetermined computed value if not set (currently 500GB). Note: If SSDs are not attached, it also contains the HDFS data blocks and Hadoop working directories."]
    pub fn boot_disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk_type` after provisioning.\nThe disk type of the primary disk attached to each node. Such as \"pd-ssd\" or \"pd-standard\". Defaults to \"pd-standard\"."]
    pub fn boot_disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_ssd_interface` after provisioning.\nInterface type of local SSDs (default is \"scsi\"). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn local_ssd_interface(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_ssd_interface", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `num_local_ssds` after provisioning.\nThe amount of local SSD disks that will be attached to each master cluster node. Defaults to 0."]
    pub fn num_local_ssds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_local_ssds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_count: Option<PrimField<f64>>,
}
impl DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl { # [doc = "Set the field `machine_type`.\n"] pub fn set_machine_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . machine_type = Some (v . into ()) ; self } # [doc = "Set the field `vm_count`.\n"] pub fn set_vm_count (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . vm_count = Some (v . into ()) ; self } }
impl ToListMappable for DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl { type O = BlockAssignable < DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl
{}
impl BuildDataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl { pub fn build (self) -> DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl { DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl { machine_type : core :: default :: Default :: default () , vm_count : core :: default :: Default :: default () , } } }
pub struct DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef { fn new (shared : StackShared , base : String) -> DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef { DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef { shared : shared , base : base . to_string () , } } }
impl DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `machine_type` after provisioning.\n"] pub fn machine_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.machine_type" , self . base)) } # [doc = "Get a reference to the value of field `vm_count` after provisioning.\n"] pub fn vm_count (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.vm_count" , self . base)) } }
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rank: Option<PrimField<f64>>,
}
impl
    DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl
{
    #[doc = "Set the field `machine_types`.\nFull machine-type names, e.g. \"n1-standard-16\"."]
    pub fn set_machine_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.machine_types = Some(v.into());
        self
    }
    #[doc = "Set the field `rank`.\nPreference of this instance selection. Lower number means higher preference. Dataproc will first try to create a VM based on the machine-type with priority rank and fallback to next rank based on availability. Machine types and instance selections with the same priority have the same preference."]
    pub fn set_rank(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.rank = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl { type O = BlockAssignable < DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl
{}
impl BuildDataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl { pub fn build (self) -> DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl { DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl { machine_types : core :: default :: Default :: default () , rank : core :: default :: Default :: default () , } } }
pub struct DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef { fn new (shared : StackShared , base : String) -> DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef { DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef { shared : shared , base : base . to_string () , } } }
impl DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `machine_types` after provisioning.\nFull machine-type names, e.g. \"n1-standard-16\"."] pub fn machine_types (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.machine_types" , self . base)) } # [doc = "Get a reference to the value of field `rank` after provisioning.\nPreference of this instance selection. Lower number means higher preference. Dataproc will first try to create a VM based on the machine-type with priority rank and fallback to next rank based on availability. Machine types and instance selections with the same priority have the same preference."] pub fn rank (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.rank" , self . base)) } }
#[derive(Serialize, Default)]
struct DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElDynamic { instance_selection_list : Option < DynamicBlock < DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl >> , }
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyEl { # [serde (skip_serializing_if = "Option::is_none")] instance_selection_list : Option < Vec < DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl > > , dynamic : DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElDynamic , }
impl DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyEl {
    #[doc = "Set the field `instance_selection_list`.\n"]
    pub fn set_instance_selection_list(
        mut self,
        v : impl Into < BlockAssignable < DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instance_selection_list = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_selection_list = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyEl {
    type O =
        BlockAssignable<DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyEl {}
impl BuildDataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyEl {
    pub fn build(self) -> DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyEl {
        DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyEl {
            instance_selection_list: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElRef {
        DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_selection_results` after provisioning.\nA list of instance selection results in the group."]    pub fn instance_selection_results (& self) -> ListRef < DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_selection_results", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_selection_list` after provisioning.\n"]    pub fn instance_selection_list (& self) -> ListRef < DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_selection_list", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocClusterClusterConfigElMasterConfigElDynamic {
    accelerators: Option<DynamicBlock<DataprocClusterClusterConfigElMasterConfigElAcceleratorsEl>>,
    disk_config: Option<DynamicBlock<DataprocClusterClusterConfigElMasterConfigElDiskConfigEl>>,
    instance_flexibility_policy: Option<
        DynamicBlock<DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyEl>,
    >,
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElMasterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_cpu_platform: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_instances: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerators: Option<Vec<DataprocClusterClusterConfigElMasterConfigElAcceleratorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_config: Option<Vec<DataprocClusterClusterConfigElMasterConfigElDiskConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_flexibility_policy:
        Option<Vec<DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyEl>>,
    dynamic: DataprocClusterClusterConfigElMasterConfigElDynamic,
}
impl DataprocClusterClusterConfigElMasterConfigEl {
    #[doc = "Set the field `image_uri`.\nThe URI for the image to use for this master"]
    pub fn set_image_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nThe name of a Google Compute Engine machine type to create for the master"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `min_cpu_platform`.\nThe name of a minimum generation of CPU family for the master. If not specified, GCP will default to a predetermined computed value for each zone."]
    pub fn set_min_cpu_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_cpu_platform = Some(v.into());
        self
    }
    #[doc = "Set the field `num_instances`.\nSpecifies the number of master nodes to create. If not specified, GCP will default to a predetermined computed value."]
    pub fn set_num_instances(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElMasterConfigElAcceleratorsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.accelerators = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerators = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `disk_config`.\n"]
    pub fn set_disk_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElMasterConfigElDiskConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.disk_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `instance_flexibility_policy`.\n"]
    pub fn set_instance_flexibility_policy(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instance_flexibility_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_flexibility_policy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElMasterConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElMasterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElMasterConfigEl {}
impl BuildDataprocClusterClusterConfigElMasterConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElMasterConfigEl {
        DataprocClusterClusterConfigElMasterConfigEl {
            image_uri: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            num_instances: core::default::Default::default(),
            accelerators: core::default::Default::default(),
            disk_config: core::default::Default::default(),
            instance_flexibility_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElMasterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElMasterConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocClusterClusterConfigElMasterConfigElRef {
        DataprocClusterClusterConfigElMasterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElMasterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\nThe URI for the image to use for this master"]
    pub fn image_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `instance_names` after provisioning.\nList of master instance names which have been assigned to the cluster."]
    pub fn instance_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe name of a Google Compute Engine machine type to create for the master"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `min_cpu_platform` after provisioning.\nThe name of a minimum generation of CPU family for the master. If not specified, GCP will default to a predetermined computed value for each zone."]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_cpu_platform", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `num_instances` after provisioning.\nSpecifies the number of master nodes to create. If not specified, GCP will default to a predetermined computed value."]
    pub fn num_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_config` after provisioning.\n"]
    pub fn disk_config(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElMasterConfigElDiskConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.disk_config", self.base))
    }
    #[doc = "Get a reference to the value of field `instance_flexibility_policy` after provisioning.\n"]
    pub fn instance_flexibility_policy(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElMasterConfigElInstanceFlexibilityPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_flexibility_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElMetastoreConfigEl {
    dataproc_metastore_service: PrimField<String>,
}
impl DataprocClusterClusterConfigElMetastoreConfigEl {}
impl ToListMappable for DataprocClusterClusterConfigElMetastoreConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElMetastoreConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElMetastoreConfigEl {
    #[doc = "Resource name of an existing Dataproc Metastore service."]
    pub dataproc_metastore_service: PrimField<String>,
}
impl BuildDataprocClusterClusterConfigElMetastoreConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElMetastoreConfigEl {
        DataprocClusterClusterConfigElMetastoreConfigEl {
            dataproc_metastore_service: self.dataproc_metastore_service,
        }
    }
}
pub struct DataprocClusterClusterConfigElMetastoreConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElMetastoreConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElMetastoreConfigElRef {
        DataprocClusterClusterConfigElMetastoreConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElMetastoreConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataproc_metastore_service` after provisioning.\nResource name of an existing Dataproc Metastore service."]
    pub fn dataproc_metastore_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataproc_metastore_service", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElPreemptibleWorkerConfigElDiskConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_provisioned_iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_provisioned_throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_interface: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_local_ssds: Option<PrimField<f64>>,
}
impl DataprocClusterClusterConfigElPreemptibleWorkerConfigElDiskConfigEl {
    #[doc = "Set the field `boot_disk_provisioned_iops`.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn set_boot_disk_provisioned_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk_provisioned_throughput`.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn set_boot_disk_provisioned_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk_size_gb`.\nSize of the primary disk attached to each preemptible worker node, specified in GB. The smallest allowed disk size is 10GB. GCP will default to a predetermined computed value if not set (currently 500GB). Note: If SSDs are not attached, it also contains the HDFS data blocks and Hadoop working directories."]
    pub fn set_boot_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk_type`.\nThe disk type of the primary disk attached to each preemptible worker node. Such as \"pd-ssd\" or \"pd-standard\". Defaults to \"pd-standard\"."]
    pub fn set_boot_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.boot_disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `local_ssd_interface`.\nInterface type of local SSDs (default is \"scsi\"). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn set_local_ssd_interface(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_ssd_interface = Some(v.into());
        self
    }
    #[doc = "Set the field `num_local_ssds`.\nThe amount of local SSD disks that will be attached to each preemptible worker node. Defaults to 0."]
    pub fn set_num_local_ssds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_local_ssds = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElPreemptibleWorkerConfigElDiskConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElPreemptibleWorkerConfigElDiskConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElPreemptibleWorkerConfigElDiskConfigEl {}
impl BuildDataprocClusterClusterConfigElPreemptibleWorkerConfigElDiskConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElPreemptibleWorkerConfigElDiskConfigEl {
        DataprocClusterClusterConfigElPreemptibleWorkerConfigElDiskConfigEl {
            boot_disk_provisioned_iops: core::default::Default::default(),
            boot_disk_provisioned_throughput: core::default::Default::default(),
            boot_disk_size_gb: core::default::Default::default(),
            boot_disk_type: core::default::Default::default(),
            local_ssd_interface: core::default::Default::default(),
            num_local_ssds: core::default::Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElPreemptibleWorkerConfigElDiskConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElPreemptibleWorkerConfigElDiskConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElPreemptibleWorkerConfigElDiskConfigElRef {
        DataprocClusterClusterConfigElPreemptibleWorkerConfigElDiskConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElPreemptibleWorkerConfigElDiskConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `boot_disk_provisioned_iops` after provisioning.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn boot_disk_provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_provisioned_iops", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk_provisioned_throughput` after provisioning.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn boot_disk_provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_provisioned_throughput", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nSize of the primary disk attached to each preemptible worker node, specified in GB. The smallest allowed disk size is 10GB. GCP will default to a predetermined computed value if not set (currently 500GB). Note: If SSDs are not attached, it also contains the HDFS data blocks and Hadoop working directories."]
    pub fn boot_disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk_type` after provisioning.\nThe disk type of the primary disk attached to each preemptible worker node. Such as \"pd-ssd\" or \"pd-standard\". Defaults to \"pd-standard\"."]
    pub fn boot_disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_ssd_interface` after provisioning.\nInterface type of local SSDs (default is \"scsi\"). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn local_ssd_interface(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_ssd_interface", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `num_local_ssds` after provisioning.\nThe amount of local SSD disks that will be attached to each preemptible worker node. Defaults to 0."]
    pub fn num_local_ssds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_local_ssds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_count: Option<PrimField<f64>>,
}
impl DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl { # [doc = "Set the field `machine_type`.\n"] pub fn set_machine_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . machine_type = Some (v . into ()) ; self } # [doc = "Set the field `vm_count`.\n"] pub fn set_vm_count (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . vm_count = Some (v . into ()) ; self } }
impl ToListMappable for DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl { type O = BlockAssignable < DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl
{}
impl BuildDataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl { pub fn build (self) -> DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl { DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl { machine_type : core :: default :: Default :: default () , vm_count : core :: default :: Default :: default () , } } }
pub struct DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef { fn new (shared : StackShared , base : String) -> DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef { DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef { shared : shared , base : base . to_string () , } } }
impl DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `machine_type` after provisioning.\n"] pub fn machine_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.machine_type" , self . base)) } # [doc = "Get a reference to the value of field `vm_count` after provisioning.\n"] pub fn vm_count (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.vm_count" , self . base)) } }
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rank: Option<PrimField<f64>>,
}
impl DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl { # [doc = "Set the field `machine_types`.\nFull machine-type names, e.g. \"n1-standard-16\"."] pub fn set_machine_types (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . machine_types = Some (v . into ()) ; self } # [doc = "Set the field `rank`.\nPreference of this instance selection. Lower number means higher preference. Dataproc will first try to create a VM based on the machine-type with priority rank and fallback to next rank based on availability. Machine types and instance selections with the same priority have the same preference."] pub fn set_rank (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . rank = Some (v . into ()) ; self } }
impl ToListMappable for DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl { type O = BlockAssignable < DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl
{}
impl BuildDataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl { pub fn build (self) -> DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl { DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl { machine_types : core :: default :: Default :: default () , rank : core :: default :: Default :: default () , } } }
pub struct DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef { fn new (shared : StackShared , base : String) -> DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef { DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef { shared : shared , base : base . to_string () , } } }
impl DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `machine_types` after provisioning.\nFull machine-type names, e.g. \"n1-standard-16\"."] pub fn machine_types (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.machine_types" , self . base)) } # [doc = "Get a reference to the value of field `rank` after provisioning.\nPreference of this instance selection. Lower number means higher preference. Dataproc will first try to create a VM based on the machine-type with priority rank and fallback to next rank based on availability. Machine types and instance selections with the same priority have the same preference."] pub fn rank (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.rank" , self . base)) } }
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElProvisioningModelMixEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    standard_capacity_base: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standard_capacity_percent_above_base: Option<PrimField<f64>>,
}
impl DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElProvisioningModelMixEl { # [doc = "Set the field `standard_capacity_base`.\nThe base capacity that will always use Standard VMs to avoid risk of more preemption than the minimum capacity you need."] pub fn set_standard_capacity_base (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . standard_capacity_base = Some (v . into ()) ; self } # [doc = "Set the field `standard_capacity_percent_above_base`.\nThe percentage of target capacity that should use Standard VM. The remaining percentage will use Spot VMs."] pub fn set_standard_capacity_percent_above_base (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . standard_capacity_percent_above_base = Some (v . into ()) ; self } }
impl ToListMappable for DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElProvisioningModelMixEl { type O = BlockAssignable < DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElProvisioningModelMixEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElProvisioningModelMixEl
{}
impl BuildDataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElProvisioningModelMixEl { pub fn build (self) -> DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElProvisioningModelMixEl { DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElProvisioningModelMixEl { standard_capacity_base : core :: default :: Default :: default () , standard_capacity_percent_above_base : core :: default :: Default :: default () , } } }
pub struct DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElProvisioningModelMixElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElProvisioningModelMixElRef { fn new (shared : StackShared , base : String) -> DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElProvisioningModelMixElRef { DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElProvisioningModelMixElRef { shared : shared , base : base . to_string () , } } }
impl DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElProvisioningModelMixElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `standard_capacity_base` after provisioning.\nThe base capacity that will always use Standard VMs to avoid risk of more preemption than the minimum capacity you need."] pub fn standard_capacity_base (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.standard_capacity_base" , self . base)) } # [doc = "Get a reference to the value of field `standard_capacity_percent_above_base` after provisioning.\nThe percentage of target capacity that should use Standard VM. The remaining percentage will use Spot VMs."] pub fn standard_capacity_percent_above_base (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.standard_capacity_percent_above_base" , self . base)) } }
#[derive(Serialize, Default)]
struct DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElDynamic { instance_selection_list : Option < DynamicBlock < DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl >> , provisioning_model_mix : Option < DynamicBlock < DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElProvisioningModelMixEl >> , }
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyEl { # [serde (skip_serializing_if = "Option::is_none")] instance_selection_list : Option < Vec < DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl > > , # [serde (skip_serializing_if = "Option::is_none")] provisioning_model_mix : Option < Vec < DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElProvisioningModelMixEl > > , dynamic : DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElDynamic , }
impl DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyEl {
    #[doc = "Set the field `instance_selection_list`.\n"]
    pub fn set_instance_selection_list(
        mut self,
        v : impl Into < BlockAssignable < DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instance_selection_list = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_selection_list = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `provisioning_model_mix`.\n"]
    pub fn set_provisioning_model_mix(
        mut self,
        v : impl Into < BlockAssignable < DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElProvisioningModelMixEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.provisioning_model_mix = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.provisioning_model_mix = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyEl
{
    type O = BlockAssignable<
        DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyEl
{}
impl BuildDataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyEl {
    pub fn build(
        self,
    ) -> DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyEl {
        DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyEl {
            instance_selection_list: core::default::Default::default(),
            provisioning_model_mix: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElRef {
        DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_selection_results` after provisioning.\nA list of instance selection results in the group."]    pub fn instance_selection_results (& self) -> ListRef < DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_selection_results", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_selection_list` after provisioning.\n"]    pub fn instance_selection_list (& self) -> ListRef < DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_selection_list", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `provisioning_model_mix` after provisioning.\n"]    pub fn provisioning_model_mix (& self) -> ListRef < DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElProvisioningModelMixElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.provisioning_model_mix", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocClusterClusterConfigElPreemptibleWorkerConfigElDynamic {
    disk_config:
        Option<DynamicBlock<DataprocClusterClusterConfigElPreemptibleWorkerConfigElDiskConfigEl>>,
    instance_flexibility_policy: Option<
        DynamicBlock<
            DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElPreemptibleWorkerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    num_instances: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preemptibility: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_config: Option<Vec<DataprocClusterClusterConfigElPreemptibleWorkerConfigElDiskConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_flexibility_policy: Option<
        Vec<DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyEl>,
    >,
    dynamic: DataprocClusterClusterConfigElPreemptibleWorkerConfigElDynamic,
}
impl DataprocClusterClusterConfigElPreemptibleWorkerConfigEl {
    #[doc = "Set the field `num_instances`.\nSpecifies the number of preemptible nodes to create. Defaults to 0."]
    pub fn set_num_instances(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `preemptibility`.\nSpecifies the preemptibility of the secondary nodes. Defaults to PREEMPTIBLE."]
    pub fn set_preemptibility(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preemptibility = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_config`.\n"]
    pub fn set_disk_config(
        mut self,
        v: impl Into<
            BlockAssignable<DataprocClusterClusterConfigElPreemptibleWorkerConfigElDiskConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.disk_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `instance_flexibility_policy`.\n"]
    pub fn set_instance_flexibility_policy(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instance_flexibility_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_flexibility_policy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElPreemptibleWorkerConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElPreemptibleWorkerConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElPreemptibleWorkerConfigEl {}
impl BuildDataprocClusterClusterConfigElPreemptibleWorkerConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElPreemptibleWorkerConfigEl {
        DataprocClusterClusterConfigElPreemptibleWorkerConfigEl {
            num_instances: core::default::Default::default(),
            preemptibility: core::default::Default::default(),
            disk_config: core::default::Default::default(),
            instance_flexibility_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElPreemptibleWorkerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElPreemptibleWorkerConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElPreemptibleWorkerConfigElRef {
        DataprocClusterClusterConfigElPreemptibleWorkerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElPreemptibleWorkerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_names` after provisioning.\nList of preemptible instance names which have been assigned to the cluster."]
    pub fn instance_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `num_instances` after provisioning.\nSpecifies the number of preemptible nodes to create. Defaults to 0."]
    pub fn num_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `preemptibility` after provisioning.\nSpecifies the preemptibility of the secondary nodes. Defaults to PREEMPTIBLE."]
    pub fn preemptibility(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preemptibility", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_config` after provisioning.\n"]
    pub fn disk_config(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElPreemptibleWorkerConfigElDiskConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.disk_config", self.base))
    }
    #[doc = "Get a reference to the value of field `instance_flexibility_policy` after provisioning.\n"]
    pub fn instance_flexibility_policy(
        &self,
    ) -> ListRef<
        DataprocClusterClusterConfigElPreemptibleWorkerConfigElInstanceFlexibilityPolicyElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_flexibility_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElSecurityConfigElIdentityConfigEl {
    user_service_account_mapping: RecField<PrimField<String>>,
}
impl DataprocClusterClusterConfigElSecurityConfigElIdentityConfigEl {}
impl ToListMappable for DataprocClusterClusterConfigElSecurityConfigElIdentityConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElSecurityConfigElIdentityConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElSecurityConfigElIdentityConfigEl {
    #[doc = "User to service account mappings for multi-tenancy."]
    pub user_service_account_mapping: RecField<PrimField<String>>,
}
impl BuildDataprocClusterClusterConfigElSecurityConfigElIdentityConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElSecurityConfigElIdentityConfigEl {
        DataprocClusterClusterConfigElSecurityConfigElIdentityConfigEl {
            user_service_account_mapping: self.user_service_account_mapping,
        }
    }
}
pub struct DataprocClusterClusterConfigElSecurityConfigElIdentityConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElSecurityConfigElIdentityConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElSecurityConfigElIdentityConfigElRef {
        DataprocClusterClusterConfigElSecurityConfigElIdentityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElSecurityConfigElIdentityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `user_service_account_mapping` after provisioning.\nUser to service account mappings for multi-tenancy."]
    pub fn user_service_account_mapping(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.user_service_account_mapping", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElSecurityConfigElKerberosConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_realm_trust_admin_server: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_realm_trust_kdc: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_realm_trust_realm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_realm_trust_shared_password_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_kerberos: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kdc_db_key_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_password_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keystore_password_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keystore_uri: Option<PrimField<String>>,
    kms_key_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    realm: Option<PrimField<String>>,
    root_principal_password_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tgt_lifetime_hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    truststore_password_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    truststore_uri: Option<PrimField<String>>,
}
impl DataprocClusterClusterConfigElSecurityConfigElKerberosConfigEl {
    #[doc = "Set the field `cross_realm_trust_admin_server`.\nThe admin server (IP or hostname) for the remote trusted realm in a cross realm trust relationship."]
    pub fn set_cross_realm_trust_admin_server(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cross_realm_trust_admin_server = Some(v.into());
        self
    }
    #[doc = "Set the field `cross_realm_trust_kdc`.\nThe KDC (IP or hostname) for the remote trusted realm in a cross realm trust relationship."]
    pub fn set_cross_realm_trust_kdc(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cross_realm_trust_kdc = Some(v.into());
        self
    }
    #[doc = "Set the field `cross_realm_trust_realm`.\nThe remote realm the Dataproc on-cluster KDC will trust, should the user enable cross realm trust."]
    pub fn set_cross_realm_trust_realm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cross_realm_trust_realm = Some(v.into());
        self
    }
    #[doc = "Set the field `cross_realm_trust_shared_password_uri`.\nThe Cloud Storage URI of a KMS encrypted file containing the shared password between the on-cluster\nKerberos realm and the remote trusted realm, in a cross realm trust relationship."]
    pub fn set_cross_realm_trust_shared_password_uri(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.cross_realm_trust_shared_password_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_kerberos`.\nFlag to indicate whether to Kerberize the cluster."]
    pub fn set_enable_kerberos(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_kerberos = Some(v.into());
        self
    }
    #[doc = "Set the field `kdc_db_key_uri`.\nThe Cloud Storage URI of a KMS encrypted file containing the master key of the KDC database."]
    pub fn set_kdc_db_key_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kdc_db_key_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `key_password_uri`.\nThe Cloud Storage URI of a KMS encrypted file containing the password to the user provided key. For the self-signed certificate, this password is generated by Dataproc."]
    pub fn set_key_password_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_password_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `keystore_password_uri`.\nThe Cloud Storage URI of a KMS encrypted file containing\nthe password to the user provided keystore. For the self-signed certificate, this password is generated\nby Dataproc"]
    pub fn set_keystore_password_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.keystore_password_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `keystore_uri`.\nThe Cloud Storage URI of the keystore file used for SSL encryption. If not provided, Dataproc will provide a self-signed certificate."]
    pub fn set_keystore_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.keystore_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `realm`.\nThe name of the on-cluster Kerberos realm. If not specified, the uppercased domain of hostnames will be the realm."]
    pub fn set_realm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.realm = Some(v.into());
        self
    }
    #[doc = "Set the field `tgt_lifetime_hours`.\nThe lifetime of the ticket granting ticket, in hours."]
    pub fn set_tgt_lifetime_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.tgt_lifetime_hours = Some(v.into());
        self
    }
    #[doc = "Set the field `truststore_password_uri`.\nThe Cloud Storage URI of a KMS encrypted file containing the password to the user provided truststore. For the self-signed certificate, this password is generated by Dataproc."]
    pub fn set_truststore_password_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.truststore_password_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `truststore_uri`.\nThe Cloud Storage URI of the truststore file used for SSL encryption. If not provided, Dataproc will provide a self-signed certificate."]
    pub fn set_truststore_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.truststore_uri = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElSecurityConfigElKerberosConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElSecurityConfigElKerberosConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElSecurityConfigElKerberosConfigEl {
    #[doc = "The uri of the KMS key used to encrypt various sensitive files."]
    pub kms_key_uri: PrimField<String>,
    #[doc = "The cloud Storage URI of a KMS encrypted file containing the root principal password."]
    pub root_principal_password_uri: PrimField<String>,
}
impl BuildDataprocClusterClusterConfigElSecurityConfigElKerberosConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElSecurityConfigElKerberosConfigEl {
        DataprocClusterClusterConfigElSecurityConfigElKerberosConfigEl {
            cross_realm_trust_admin_server: core::default::Default::default(),
            cross_realm_trust_kdc: core::default::Default::default(),
            cross_realm_trust_realm: core::default::Default::default(),
            cross_realm_trust_shared_password_uri: core::default::Default::default(),
            enable_kerberos: core::default::Default::default(),
            kdc_db_key_uri: core::default::Default::default(),
            key_password_uri: core::default::Default::default(),
            keystore_password_uri: core::default::Default::default(),
            keystore_uri: core::default::Default::default(),
            kms_key_uri: self.kms_key_uri,
            realm: core::default::Default::default(),
            root_principal_password_uri: self.root_principal_password_uri,
            tgt_lifetime_hours: core::default::Default::default(),
            truststore_password_uri: core::default::Default::default(),
            truststore_uri: core::default::Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElSecurityConfigElKerberosConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElSecurityConfigElKerberosConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElSecurityConfigElKerberosConfigElRef {
        DataprocClusterClusterConfigElSecurityConfigElKerberosConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElSecurityConfigElKerberosConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cross_realm_trust_admin_server` after provisioning.\nThe admin server (IP or hostname) for the remote trusted realm in a cross realm trust relationship."]
    pub fn cross_realm_trust_admin_server(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cross_realm_trust_admin_server", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cross_realm_trust_kdc` after provisioning.\nThe KDC (IP or hostname) for the remote trusted realm in a cross realm trust relationship."]
    pub fn cross_realm_trust_kdc(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cross_realm_trust_kdc", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cross_realm_trust_realm` after provisioning.\nThe remote realm the Dataproc on-cluster KDC will trust, should the user enable cross realm trust."]
    pub fn cross_realm_trust_realm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cross_realm_trust_realm", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cross_realm_trust_shared_password_uri` after provisioning.\nThe Cloud Storage URI of a KMS encrypted file containing the shared password between the on-cluster\nKerberos realm and the remote trusted realm, in a cross realm trust relationship."]
    pub fn cross_realm_trust_shared_password_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cross_realm_trust_shared_password_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_kerberos` after provisioning.\nFlag to indicate whether to Kerberize the cluster."]
    pub fn enable_kerberos(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_kerberos", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kdc_db_key_uri` after provisioning.\nThe Cloud Storage URI of a KMS encrypted file containing the master key of the KDC database."]
    pub fn kdc_db_key_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kdc_db_key_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_password_uri` after provisioning.\nThe Cloud Storage URI of a KMS encrypted file containing the password to the user provided key. For the self-signed certificate, this password is generated by Dataproc."]
    pub fn key_password_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_password_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `keystore_password_uri` after provisioning.\nThe Cloud Storage URI of a KMS encrypted file containing\nthe password to the user provided keystore. For the self-signed certificate, this password is generated\nby Dataproc"]
    pub fn keystore_password_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.keystore_password_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `keystore_uri` after provisioning.\nThe Cloud Storage URI of the keystore file used for SSL encryption. If not provided, Dataproc will provide a self-signed certificate."]
    pub fn keystore_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.keystore_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key_uri` after provisioning.\nThe uri of the KMS key used to encrypt various sensitive files."]
    pub fn kms_key_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `realm` after provisioning.\nThe name of the on-cluster Kerberos realm. If not specified, the uppercased domain of hostnames will be the realm."]
    pub fn realm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.realm", self.base))
    }
    #[doc = "Get a reference to the value of field `root_principal_password_uri` after provisioning.\nThe cloud Storage URI of a KMS encrypted file containing the root principal password."]
    pub fn root_principal_password_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_principal_password_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tgt_lifetime_hours` after provisioning.\nThe lifetime of the ticket granting ticket, in hours."]
    pub fn tgt_lifetime_hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tgt_lifetime_hours", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `truststore_password_uri` after provisioning.\nThe Cloud Storage URI of a KMS encrypted file containing the password to the user provided truststore. For the self-signed certificate, this password is generated by Dataproc."]
    pub fn truststore_password_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.truststore_password_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `truststore_uri` after provisioning.\nThe Cloud Storage URI of the truststore file used for SSL encryption. If not provided, Dataproc will provide a self-signed certificate."]
    pub fn truststore_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.truststore_uri", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocClusterClusterConfigElSecurityConfigElDynamic {
    identity_config:
        Option<DynamicBlock<DataprocClusterClusterConfigElSecurityConfigElIdentityConfigEl>>,
    kerberos_config:
        Option<DynamicBlock<DataprocClusterClusterConfigElSecurityConfigElKerberosConfigEl>>,
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElSecurityConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_config: Option<Vec<DataprocClusterClusterConfigElSecurityConfigElIdentityConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kerberos_config: Option<Vec<DataprocClusterClusterConfigElSecurityConfigElKerberosConfigEl>>,
    dynamic: DataprocClusterClusterConfigElSecurityConfigElDynamic,
}
impl DataprocClusterClusterConfigElSecurityConfigEl {
    #[doc = "Set the field `identity_config`.\n"]
    pub fn set_identity_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElSecurityConfigElIdentityConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.identity_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.identity_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kerberos_config`.\n"]
    pub fn set_kerberos_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElSecurityConfigElKerberosConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kerberos_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kerberos_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElSecurityConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElSecurityConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElSecurityConfigEl {}
impl BuildDataprocClusterClusterConfigElSecurityConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElSecurityConfigEl {
        DataprocClusterClusterConfigElSecurityConfigEl {
            identity_config: core::default::Default::default(),
            kerberos_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElSecurityConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElSecurityConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocClusterClusterConfigElSecurityConfigElRef {
        DataprocClusterClusterConfigElSecurityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElSecurityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identity_config` after provisioning.\n"]
    pub fn identity_config(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElSecurityConfigElIdentityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kerberos_config` after provisioning.\n"]
    pub fn kerberos_config(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElSecurityConfigElKerberosConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kerberos_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElSoftwareConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_components: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_properties: Option<RecField<PrimField<String>>>,
}
impl DataprocClusterClusterConfigElSoftwareConfigEl {
    #[doc = "Set the field `image_version`.\nThe Cloud Dataproc image version to use for the cluster - this controls the sets of software versions installed onto the nodes when you create clusters. If not specified, defaults to the latest version."]
    pub fn set_image_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_version = Some(v.into());
        self
    }
    #[doc = "Set the field `optional_components`.\nThe set of optional components to activate on the cluster."]
    pub fn set_optional_components(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.optional_components = Some(v.into());
        self
    }
    #[doc = "Set the field `override_properties`.\nA list of override and additional properties (key/value pairs) used to modify various aspects of the common configuration files used when creating a cluster."]
    pub fn set_override_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.override_properties = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElSoftwareConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElSoftwareConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElSoftwareConfigEl {}
impl BuildDataprocClusterClusterConfigElSoftwareConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElSoftwareConfigEl {
        DataprocClusterClusterConfigElSoftwareConfigEl {
            image_version: core::default::Default::default(),
            optional_components: core::default::Default::default(),
            override_properties: core::default::Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElSoftwareConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElSoftwareConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocClusterClusterConfigElSoftwareConfigElRef {
        DataprocClusterClusterConfigElSoftwareConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElSoftwareConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `image_version` after provisioning.\nThe Cloud Dataproc image version to use for the cluster - this controls the sets of software versions installed onto the nodes when you create clusters. If not specified, defaults to the latest version."]
    pub fn image_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `optional_components` after provisioning.\nThe set of optional components to activate on the cluster."]
    pub fn optional_components(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.optional_components", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `override_properties` after provisioning.\nA list of override and additional properties (key/value pairs) used to modify various aspects of the common configuration files used when creating a cluster."]
    pub fn override_properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.override_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nA list of the properties used to set the daemon config files. This will include any values supplied by the user via cluster_config.software_config.override_properties"]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElWorkerConfigElAcceleratorsEl {
    accelerator_count: PrimField<f64>,
    accelerator_type: PrimField<String>,
}
impl DataprocClusterClusterConfigElWorkerConfigElAcceleratorsEl {}
impl ToListMappable for DataprocClusterClusterConfigElWorkerConfigElAcceleratorsEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElWorkerConfigElAcceleratorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElWorkerConfigElAcceleratorsEl {
    #[doc = "The number of the accelerator cards of this type exposed to this instance. Often restricted to one of 1, 2, 4, or 8."]
    pub accelerator_count: PrimField<f64>,
    #[doc = "The short name of the accelerator type to expose to this instance. For example, nvidia-tesla-k80."]
    pub accelerator_type: PrimField<String>,
}
impl BuildDataprocClusterClusterConfigElWorkerConfigElAcceleratorsEl {
    pub fn build(self) -> DataprocClusterClusterConfigElWorkerConfigElAcceleratorsEl {
        DataprocClusterClusterConfigElWorkerConfigElAcceleratorsEl {
            accelerator_count: self.accelerator_count,
            accelerator_type: self.accelerator_type,
        }
    }
}
pub struct DataprocClusterClusterConfigElWorkerConfigElAcceleratorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElWorkerConfigElAcceleratorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElWorkerConfigElAcceleratorsElRef {
        DataprocClusterClusterConfigElWorkerConfigElAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElWorkerConfigElAcceleratorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of the accelerator cards of this type exposed to this instance. Often restricted to one of 1, 2, 4, or 8."]
    pub fn accelerator_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\nThe short name of the accelerator type to expose to this instance. For example, nvidia-tesla-k80."]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElWorkerConfigElDiskConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_provisioned_iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_provisioned_throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_interface: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_local_ssds: Option<PrimField<f64>>,
}
impl DataprocClusterClusterConfigElWorkerConfigElDiskConfigEl {
    #[doc = "Set the field `boot_disk_provisioned_iops`.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn set_boot_disk_provisioned_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_iops = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk_provisioned_throughput`.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn set_boot_disk_provisioned_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_throughput = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk_size_gb`.\nSize of the primary disk attached to each node, specified in GB. The primary disk contains the boot volume and system libraries, and the smallest allowed disk size is 10GB. GCP will default to a predetermined computed value if not set (currently 500GB). Note: If SSDs are not attached, it also contains the HDFS data blocks and Hadoop working directories."]
    pub fn set_boot_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk_type`.\nThe disk type of the primary disk attached to each node. Such as \"pd-ssd\" or \"pd-standard\". Defaults to \"pd-standard\"."]
    pub fn set_boot_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.boot_disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `local_ssd_interface`.\nInterface type of local SSDs (default is \"scsi\"). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn set_local_ssd_interface(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_ssd_interface = Some(v.into());
        self
    }
    #[doc = "Set the field `num_local_ssds`.\nThe amount of local SSD disks that will be attached to each master cluster node. Defaults to 0."]
    pub fn set_num_local_ssds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_local_ssds = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElWorkerConfigElDiskConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElWorkerConfigElDiskConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElWorkerConfigElDiskConfigEl {}
impl BuildDataprocClusterClusterConfigElWorkerConfigElDiskConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElWorkerConfigElDiskConfigEl {
        DataprocClusterClusterConfigElWorkerConfigElDiskConfigEl {
            boot_disk_provisioned_iops: core::default::Default::default(),
            boot_disk_provisioned_throughput: core::default::Default::default(),
            boot_disk_size_gb: core::default::Default::default(),
            boot_disk_type: core::default::Default::default(),
            local_ssd_interface: core::default::Default::default(),
            num_local_ssds: core::default::Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElWorkerConfigElDiskConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElWorkerConfigElDiskConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElWorkerConfigElDiskConfigElRef {
        DataprocClusterClusterConfigElWorkerConfigElDiskConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElWorkerConfigElDiskConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `boot_disk_provisioned_iops` after provisioning.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn boot_disk_provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_provisioned_iops", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk_provisioned_throughput` after provisioning.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn boot_disk_provisioned_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_provisioned_throughput", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nSize of the primary disk attached to each node, specified in GB. The primary disk contains the boot volume and system libraries, and the smallest allowed disk size is 10GB. GCP will default to a predetermined computed value if not set (currently 500GB). Note: If SSDs are not attached, it also contains the HDFS data blocks and Hadoop working directories."]
    pub fn boot_disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk_type` after provisioning.\nThe disk type of the primary disk attached to each node. Such as \"pd-ssd\" or \"pd-standard\". Defaults to \"pd-standard\"."]
    pub fn boot_disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.boot_disk_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_ssd_interface` after provisioning.\nInterface type of local SSDs (default is \"scsi\"). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn local_ssd_interface(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_ssd_interface", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `num_local_ssds` after provisioning.\nThe amount of local SSD disks that will be attached to each master cluster node. Defaults to 0."]
    pub fn num_local_ssds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_local_ssds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_count: Option<PrimField<f64>>,
}
impl DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl { # [doc = "Set the field `machine_type`.\n"] pub fn set_machine_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . machine_type = Some (v . into ()) ; self } # [doc = "Set the field `vm_count`.\n"] pub fn set_vm_count (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . vm_count = Some (v . into ()) ; self } }
impl ToListMappable for DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl { type O = BlockAssignable < DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl
{}
impl BuildDataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl { pub fn build (self) -> DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl { DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsEl { machine_type : core :: default :: Default :: default () , vm_count : core :: default :: Default :: default () , } } }
pub struct DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef { fn new (shared : StackShared , base : String) -> DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef { DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef { shared : shared , base : base . to_string () , } } }
impl DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `machine_type` after provisioning.\n"] pub fn machine_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.machine_type" , self . base)) } # [doc = "Get a reference to the value of field `vm_count` after provisioning.\n"] pub fn vm_count (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.vm_count" , self . base)) } }
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rank: Option<PrimField<f64>>,
}
impl
    DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl
{
    #[doc = "Set the field `machine_types`.\nFull machine-type names, e.g. \"n1-standard-16\"."]
    pub fn set_machine_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.machine_types = Some(v.into());
        self
    }
    #[doc = "Set the field `rank`.\nPreference of this instance selection. Lower number means higher preference. Dataproc will first try to create a VM based on the machine-type with priority rank and fallback to next rank based on availability. Machine types and instance selections with the same priority have the same preference."]
    pub fn set_rank(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.rank = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl { type O = BlockAssignable < DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl
{}
impl BuildDataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl { pub fn build (self) -> DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl { DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl { machine_types : core :: default :: Default :: default () , rank : core :: default :: Default :: default () , } } }
pub struct DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef { fn new (shared : StackShared , base : String) -> DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef { DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef { shared : shared , base : base . to_string () , } } }
impl DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `machine_types` after provisioning.\nFull machine-type names, e.g. \"n1-standard-16\"."] pub fn machine_types (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.machine_types" , self . base)) } # [doc = "Get a reference to the value of field `rank` after provisioning.\nPreference of this instance selection. Lower number means higher preference. Dataproc will first try to create a VM based on the machine-type with priority rank and fallback to next rank based on availability. Machine types and instance selections with the same priority have the same preference."] pub fn rank (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.rank" , self . base)) } }
#[derive(Serialize, Default)]
struct DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElDynamic { instance_selection_list : Option < DynamicBlock < DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl >> , }
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyEl { # [serde (skip_serializing_if = "Option::is_none")] instance_selection_list : Option < Vec < DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl > > , dynamic : DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElDynamic , }
impl DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyEl {
    #[doc = "Set the field `instance_selection_list`.\n"]
    pub fn set_instance_selection_list(
        mut self,
        v : impl Into < BlockAssignable < DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instance_selection_list = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_selection_list = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyEl {
    type O =
        BlockAssignable<DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyEl {}
impl BuildDataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyEl {
    pub fn build(self) -> DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyEl {
        DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyEl {
            instance_selection_list: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElRef {
        DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_selection_results` after provisioning.\nA list of instance selection results in the group."]    pub fn instance_selection_results (& self) -> ListRef < DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionResultsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_selection_results", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_selection_list` after provisioning.\n"]    pub fn instance_selection_list (& self) -> ListRef < DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElInstanceSelectionListElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_selection_list", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocClusterClusterConfigElWorkerConfigElDynamic {
    accelerators: Option<DynamicBlock<DataprocClusterClusterConfigElWorkerConfigElAcceleratorsEl>>,
    disk_config: Option<DynamicBlock<DataprocClusterClusterConfigElWorkerConfigElDiskConfigEl>>,
    instance_flexibility_policy: Option<
        DynamicBlock<DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyEl>,
    >,
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigElWorkerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_cpu_platform: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_num_instances: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_instances: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerators: Option<Vec<DataprocClusterClusterConfigElWorkerConfigElAcceleratorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_config: Option<Vec<DataprocClusterClusterConfigElWorkerConfigElDiskConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_flexibility_policy:
        Option<Vec<DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyEl>>,
    dynamic: DataprocClusterClusterConfigElWorkerConfigElDynamic,
}
impl DataprocClusterClusterConfigElWorkerConfigEl {
    #[doc = "Set the field `image_uri`.\nThe URI for the image to use for this master/worker"]
    pub fn set_image_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nThe name of a Google Compute Engine machine type to create for the master/worker"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `min_cpu_platform`.\nThe name of a minimum generation of CPU family for the master/worker. If not specified, GCP will default to a predetermined computed value for each zone."]
    pub fn set_min_cpu_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_cpu_platform = Some(v.into());
        self
    }
    #[doc = "Set the field `min_num_instances`.\nThe minimum number of primary worker instances to create."]
    pub fn set_min_num_instances(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_num_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `num_instances`.\nSpecifies the number of worker nodes to create. If not specified, GCP will default to a predetermined computed value."]
    pub fn set_num_instances(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElWorkerConfigElAcceleratorsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.accelerators = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerators = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `disk_config`.\n"]
    pub fn set_disk_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElWorkerConfigElDiskConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.disk_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `instance_flexibility_policy`.\n"]
    pub fn set_instance_flexibility_policy(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instance_flexibility_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_flexibility_policy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigElWorkerConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigElWorkerConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigElWorkerConfigEl {}
impl BuildDataprocClusterClusterConfigElWorkerConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigElWorkerConfigEl {
        DataprocClusterClusterConfigElWorkerConfigEl {
            image_uri: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            min_num_instances: core::default::Default::default(),
            num_instances: core::default::Default::default(),
            accelerators: core::default::Default::default(),
            disk_config: core::default::Default::default(),
            instance_flexibility_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElWorkerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElWorkerConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocClusterClusterConfigElWorkerConfigElRef {
        DataprocClusterClusterConfigElWorkerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElWorkerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\nThe URI for the image to use for this master/worker"]
    pub fn image_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `instance_names` after provisioning.\nList of master/worker instance names which have been assigned to the cluster."]
    pub fn instance_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe name of a Google Compute Engine machine type to create for the master/worker"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `min_cpu_platform` after provisioning.\nThe name of a minimum generation of CPU family for the master/worker. If not specified, GCP will default to a predetermined computed value for each zone."]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_cpu_platform", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_num_instances` after provisioning.\nThe minimum number of primary worker instances to create."]
    pub fn min_num_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_num_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `num_instances` after provisioning.\nSpecifies the number of worker nodes to create. If not specified, GCP will default to a predetermined computed value."]
    pub fn num_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disk_config` after provisioning.\n"]
    pub fn disk_config(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElWorkerConfigElDiskConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.disk_config", self.base))
    }
    #[doc = "Get a reference to the value of field `instance_flexibility_policy` after provisioning.\n"]
    pub fn instance_flexibility_policy(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElWorkerConfigElInstanceFlexibilityPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_flexibility_policy", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocClusterClusterConfigElDynamic {
    autoscaling_config: Option<DynamicBlock<DataprocClusterClusterConfigElAutoscalingConfigEl>>,
    auxiliary_node_groups:
        Option<DynamicBlock<DataprocClusterClusterConfigElAuxiliaryNodeGroupsEl>>,
    dataproc_metric_config:
        Option<DynamicBlock<DataprocClusterClusterConfigElDataprocMetricConfigEl>>,
    encryption_config: Option<DynamicBlock<DataprocClusterClusterConfigElEncryptionConfigEl>>,
    endpoint_config: Option<DynamicBlock<DataprocClusterClusterConfigElEndpointConfigEl>>,
    gce_cluster_config: Option<DynamicBlock<DataprocClusterClusterConfigElGceClusterConfigEl>>,
    initialization_action:
        Option<DynamicBlock<DataprocClusterClusterConfigElInitializationActionEl>>,
    lifecycle_config: Option<DynamicBlock<DataprocClusterClusterConfigElLifecycleConfigEl>>,
    master_config: Option<DynamicBlock<DataprocClusterClusterConfigElMasterConfigEl>>,
    metastore_config: Option<DynamicBlock<DataprocClusterClusterConfigElMetastoreConfigEl>>,
    preemptible_worker_config:
        Option<DynamicBlock<DataprocClusterClusterConfigElPreemptibleWorkerConfigEl>>,
    security_config: Option<DynamicBlock<DataprocClusterClusterConfigElSecurityConfigEl>>,
    software_config: Option<DynamicBlock<DataprocClusterClusterConfigElSoftwareConfigEl>>,
    worker_config: Option<DynamicBlock<DataprocClusterClusterConfigElWorkerConfigEl>>,
}
#[derive(Serialize)]
pub struct DataprocClusterClusterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    staging_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temp_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_config: Option<Vec<DataprocClusterClusterConfigElAutoscalingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auxiliary_node_groups: Option<Vec<DataprocClusterClusterConfigElAuxiliaryNodeGroupsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataproc_metric_config: Option<Vec<DataprocClusterClusterConfigElDataprocMetricConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_config: Option<Vec<DataprocClusterClusterConfigElEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_config: Option<Vec<DataprocClusterClusterConfigElEndpointConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gce_cluster_config: Option<Vec<DataprocClusterClusterConfigElGceClusterConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initialization_action: Option<Vec<DataprocClusterClusterConfigElInitializationActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config: Option<Vec<DataprocClusterClusterConfigElLifecycleConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_config: Option<Vec<DataprocClusterClusterConfigElMasterConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metastore_config: Option<Vec<DataprocClusterClusterConfigElMetastoreConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preemptible_worker_config: Option<Vec<DataprocClusterClusterConfigElPreemptibleWorkerConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_config: Option<Vec<DataprocClusterClusterConfigElSecurityConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    software_config: Option<Vec<DataprocClusterClusterConfigElSoftwareConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_config: Option<Vec<DataprocClusterClusterConfigElWorkerConfigEl>>,
    dynamic: DataprocClusterClusterConfigElDynamic,
}
impl DataprocClusterClusterConfigEl {
    #[doc = "Set the field `cluster_tier`.\nSpecifies the tier of the cluster created."]
    pub fn set_cluster_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_tier = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_type`.\nThe type of the cluster."]
    pub fn set_cluster_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_type = Some(v.into());
        self
    }
    #[doc = "Set the field `engine`.\nSpecifies the engine of the cluster created."]
    pub fn set_engine(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.engine = Some(v.into());
        self
    }
    #[doc = "Set the field `staging_bucket`.\nThe Cloud Storage staging bucket used to stage files, such as Hadoop jars, between client machines and the cluster. Note: If you don't explicitly specify a staging_bucket then GCP will auto create / assign one for you. However, you are not guaranteed an auto generated bucket which is solely dedicated to your cluster; it may be shared with other clusters in the same region/zone also choosing to use the auto generation option."]
    pub fn set_staging_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.staging_bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `temp_bucket`.\nThe Cloud Storage temp bucket used to store ephemeral cluster and jobs data, such as Spark and MapReduce history files. Note: If you don't explicitly specify a temp_bucket then GCP will auto create / assign one for you."]
    pub fn set_temp_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.temp_bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_config`.\n"]
    pub fn set_autoscaling_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElAutoscalingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.autoscaling_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `auxiliary_node_groups`.\n"]
    pub fn set_auxiliary_node_groups(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElAuxiliaryNodeGroupsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auxiliary_node_groups = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auxiliary_node_groups = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dataproc_metric_config`.\n"]
    pub fn set_dataproc_metric_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElDataprocMetricConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dataproc_metric_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dataproc_metric_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElEncryptionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryption_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `endpoint_config`.\n"]
    pub fn set_endpoint_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElEndpointConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.endpoint_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.endpoint_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gce_cluster_config`.\n"]
    pub fn set_gce_cluster_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElGceClusterConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gce_cluster_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gce_cluster_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `initialization_action`.\n"]
    pub fn set_initialization_action(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElInitializationActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.initialization_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.initialization_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lifecycle_config`.\n"]
    pub fn set_lifecycle_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElLifecycleConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lifecycle_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lifecycle_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `master_config`.\n"]
    pub fn set_master_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElMasterConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.master_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.master_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `metastore_config`.\n"]
    pub fn set_metastore_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElMetastoreConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metastore_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metastore_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `preemptible_worker_config`.\n"]
    pub fn set_preemptible_worker_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElPreemptibleWorkerConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.preemptible_worker_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.preemptible_worker_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `security_config`.\n"]
    pub fn set_security_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElSecurityConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.security_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.security_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `software_config`.\n"]
    pub fn set_software_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElSoftwareConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.software_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.software_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `worker_config`.\n"]
    pub fn set_worker_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterClusterConfigElWorkerConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.worker_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.worker_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocClusterClusterConfigEl {
    type O = BlockAssignable<DataprocClusterClusterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterClusterConfigEl {}
impl BuildDataprocClusterClusterConfigEl {
    pub fn build(self) -> DataprocClusterClusterConfigEl {
        DataprocClusterClusterConfigEl {
            cluster_tier: core::default::Default::default(),
            cluster_type: core::default::Default::default(),
            engine: core::default::Default::default(),
            staging_bucket: core::default::Default::default(),
            temp_bucket: core::default::Default::default(),
            autoscaling_config: core::default::Default::default(),
            auxiliary_node_groups: core::default::Default::default(),
            dataproc_metric_config: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            endpoint_config: core::default::Default::default(),
            gce_cluster_config: core::default::Default::default(),
            initialization_action: core::default::Default::default(),
            lifecycle_config: core::default::Default::default(),
            master_config: core::default::Default::default(),
            metastore_config: core::default::Default::default(),
            preemptible_worker_config: core::default::Default::default(),
            security_config: core::default::Default::default(),
            software_config: core::default::Default::default(),
            worker_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocClusterClusterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterClusterConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocClusterClusterConfigElRef {
        DataprocClusterClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterClusterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n The name of the cloud storage bucket ultimately used to house the staging data for the cluster. If staging_bucket is specified, it will contain this value, otherwise it will be the auto generated name."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `cluster_tier` after provisioning.\nSpecifies the tier of the cluster created."]
    pub fn cluster_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_tier", self.base))
    }
    #[doc = "Get a reference to the value of field `cluster_type` after provisioning.\nThe type of the cluster."]
    pub fn cluster_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_type", self.base))
    }
    #[doc = "Get a reference to the value of field `engine` after provisioning.\nSpecifies the engine of the cluster created."]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.base))
    }
    #[doc = "Get a reference to the value of field `staging_bucket` after provisioning.\nThe Cloud Storage staging bucket used to stage files, such as Hadoop jars, between client machines and the cluster. Note: If you don't explicitly specify a staging_bucket then GCP will auto create / assign one for you. However, you are not guaranteed an auto generated bucket which is solely dedicated to your cluster; it may be shared with other clusters in the same region/zone also choosing to use the auto generation option."]
    pub fn staging_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.staging_bucket", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `temp_bucket` after provisioning.\nThe Cloud Storage temp bucket used to store ephemeral cluster and jobs data, such as Spark and MapReduce history files. Note: If you don't explicitly specify a temp_bucket then GCP will auto create / assign one for you."]
    pub fn temp_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.temp_bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `autoscaling_config` after provisioning.\n"]
    pub fn autoscaling_config(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElAutoscalingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auxiliary_node_groups` after provisioning.\n"]
    pub fn auxiliary_node_groups(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElAuxiliaryNodeGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auxiliary_node_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dataproc_metric_config` after provisioning.\n"]
    pub fn dataproc_metric_config(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElDataprocMetricConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dataproc_metric_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElEncryptionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_config` after provisioning.\n"]
    pub fn endpoint_config(&self) -> ListRef<DataprocClusterClusterConfigElEndpointConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gce_cluster_config` after provisioning.\n"]
    pub fn gce_cluster_config(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElGceClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gce_cluster_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `initialization_action` after provisioning.\n"]
    pub fn initialization_action(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElInitializationActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.initialization_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lifecycle_config` after provisioning.\n"]
    pub fn lifecycle_config(&self) -> ListRef<DataprocClusterClusterConfigElLifecycleConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lifecycle_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `master_config` after provisioning.\n"]
    pub fn master_config(&self) -> ListRef<DataprocClusterClusterConfigElMasterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.master_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metastore_config` after provisioning.\n"]
    pub fn metastore_config(&self) -> ListRef<DataprocClusterClusterConfigElMetastoreConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metastore_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `preemptible_worker_config` after provisioning.\n"]
    pub fn preemptible_worker_config(
        &self,
    ) -> ListRef<DataprocClusterClusterConfigElPreemptibleWorkerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preemptible_worker_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(&self) -> ListRef<DataprocClusterClusterConfigElSecurityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `software_config` after provisioning.\n"]
    pub fn software_config(&self) -> ListRef<DataprocClusterClusterConfigElSoftwareConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.software_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `worker_config` after provisioning.\n"]
    pub fn worker_config(&self) -> ListRef<DataprocClusterClusterConfigElWorkerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.worker_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataprocClusterTimeoutsEl {
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
impl ToListMappable for DataprocClusterTimeoutsEl {
    type O = BlockAssignable<DataprocClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterTimeoutsEl {}
impl BuildDataprocClusterTimeoutsEl {
    pub fn build(self) -> DataprocClusterTimeoutsEl {
        DataprocClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataprocClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataprocClusterTimeoutsElRef {
        DataprocClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterTimeoutsElRef {
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
#[derive(Serialize)]
pub struct DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElMetastoreConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataproc_metastore_service: Option<PrimField<String>>,
}
impl DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElMetastoreConfigEl {
    #[doc = "Set the field `dataproc_metastore_service`.\nThe Hive Metastore configuration for this workload."]
    pub fn set_dataproc_metastore_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataproc_metastore_service = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElMetastoreConfigEl
{
    type O = BlockAssignable<
        DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElMetastoreConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElMetastoreConfigEl {}
impl BuildDataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElMetastoreConfigEl {
    pub fn build(
        self,
    ) -> DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElMetastoreConfigEl {
        DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElMetastoreConfigEl {
            dataproc_metastore_service: core::default::Default::default(),
        }
    }
}
pub struct DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElMetastoreConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElMetastoreConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElMetastoreConfigElRef {
        DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElMetastoreConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElMetastoreConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataproc_metastore_service` after provisioning.\nThe Hive Metastore configuration for this workload."]
    pub fn dataproc_metastore_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataproc_metastore_service", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElSparkHistoryServerConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    dataproc_cluster: Option<PrimField<String>>,
}
impl DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElSparkHistoryServerConfigEl {
    #[doc = "Set the field `dataproc_cluster`.\nResource name of an existing Dataproc Cluster to act as a Spark History Server for the workload."]
    pub fn set_dataproc_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataproc_cluster = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElSparkHistoryServerConfigEl
{
    type O = BlockAssignable<
        DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElSparkHistoryServerConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElSparkHistoryServerConfigEl
{}
impl BuildDataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElSparkHistoryServerConfigEl {
    pub fn build(
        self,
    ) -> DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElSparkHistoryServerConfigEl
    {
        DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElSparkHistoryServerConfigEl {
            dataproc_cluster: core::default::Default::default(),
        }
    }
}
pub struct DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElSparkHistoryServerConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElSparkHistoryServerConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElSparkHistoryServerConfigElRef
    {
        DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElSparkHistoryServerConfigElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElSparkHistoryServerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataproc_cluster` after provisioning.\nResource name of an existing Dataproc Cluster to act as a Spark History Server for the workload."]
    pub fn dataproc_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataproc_cluster", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElDynamic { metastore_config : Option < DynamicBlock < DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElMetastoreConfigEl >> , spark_history_server_config : Option < DynamicBlock < DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElSparkHistoryServerConfigEl >> , }
#[derive(Serialize)]
pub struct DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigEl { # [serde (skip_serializing_if = "Option::is_none")] metastore_config : Option < Vec < DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElMetastoreConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] spark_history_server_config : Option < Vec < DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElSparkHistoryServerConfigEl > > , dynamic : DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElDynamic , }
impl DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigEl {
    #[doc = "Set the field `metastore_config`.\n"]
    pub fn set_metastore_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElMetastoreConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metastore_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metastore_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spark_history_server_config`.\n"]
    pub fn set_spark_history_server_config(
        mut self,
        v : impl Into < BlockAssignable < DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElSparkHistoryServerConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.spark_history_server_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.spark_history_server_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigEl {
    type O = BlockAssignable<DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigEl {}
impl BuildDataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigEl {
    pub fn build(self) -> DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigEl {
        DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigEl {
            metastore_config: core::default::Default::default(),
            spark_history_server_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElRef {
        DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metastore_config` after provisioning.\n"]
    pub fn metastore_config(
        &self,
    ) -> ListRef<DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElMetastoreConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metastore_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `spark_history_server_config` after provisioning.\n"]
    pub fn spark_history_server_config(
        &self,
    ) -> ListRef<
        DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElSparkHistoryServerConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_history_server_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElAutoscalingEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_node_count: Option<PrimField<f64>>,
}
impl DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElAutoscalingEl { # [doc = "Set the field `max_node_count`.\nThe maximum number of nodes in the node pool. Must be >= minNodeCount, and must be > 0."] pub fn set_max_node_count (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . max_node_count = Some (v . into ()) ; self } # [doc = "Set the field `min_node_count`.\nThe minimum number of nodes in the node pool. Must be >= 0 and <= maxNodeCount."] pub fn set_min_node_count (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . min_node_count = Some (v . into ()) ; self } }
impl ToListMappable for DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElAutoscalingEl { type O = BlockAssignable < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElAutoscalingEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElAutoscalingEl
{}
impl BuildDataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElAutoscalingEl { pub fn build (self) -> DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElAutoscalingEl { DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElAutoscalingEl { max_node_count : core :: default :: Default :: default () , min_node_count : core :: default :: Default :: default () , } } }
pub struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElAutoscalingElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElAutoscalingElRef { fn new (shared : StackShared , base : String) -> DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElAutoscalingElRef { DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElAutoscalingElRef { shared : shared , base : base . to_string () , } } }
impl DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElAutoscalingElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `max_node_count` after provisioning.\nThe maximum number of nodes in the node pool. Must be >= minNodeCount, and must be > 0."] pub fn max_node_count (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.max_node_count" , self . base)) } # [doc = "Get a reference to the value of field `min_node_count` after provisioning.\nThe minimum number of nodes in the node pool. Must be >= 0 and <= maxNodeCount."] pub fn min_node_count (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min_node_count" , self . base)) } }
#[derive(Serialize)]
pub struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_cpu_platform: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preemptible: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot: Option<PrimField<bool>>,
}
impl DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElConfigEl { # [doc = "Set the field `local_ssd_count`.\nThe minimum number of nodes in the node pool. Must be >= 0 and <= maxNodeCount."] pub fn set_local_ssd_count (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . local_ssd_count = Some (v . into ()) ; self } # [doc = "Set the field `machine_type`.\nThe name of a Compute Engine machine type."] pub fn set_machine_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . machine_type = Some (v . into ()) ; self } # [doc = "Set the field `min_cpu_platform`.\nMinimum CPU platform to be used by this instance. The instance may be scheduled on the specified or a newer CPU platform. Specify the friendly names of CPU platforms, such as \"Intel Haswell\" or \"Intel Sandy Bridge\"."] pub fn set_min_cpu_platform (mut self , v : impl Into < PrimField < String > >) -> Self { self . min_cpu_platform = Some (v . into ()) ; self } # [doc = "Set the field `preemptible`.\nWhether the nodes are created as preemptible VM instances. Preemptible nodes cannot be used in a node pool with the CONTROLLER role or in the DEFAULT node pool if the CONTROLLER role is not assigned (the DEFAULT node pool will assume the CONTROLLER role)."] pub fn set_preemptible (mut self , v : impl Into < PrimField < bool > >) -> Self { self . preemptible = Some (v . into ()) ; self } # [doc = "Set the field `spot`.\nSpot flag for enabling Spot VM, which is a rebrand of the existing preemptible flag."] pub fn set_spot (mut self , v : impl Into < PrimField < bool > >) -> Self { self . spot = Some (v . into ()) ; self } }
impl ToListMappable for DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElConfigEl { type O = BlockAssignable < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElConfigEl
{}
impl BuildDataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElConfigEl { pub fn build (self) -> DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElConfigEl { DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElConfigEl { local_ssd_count : core :: default :: Default :: default () , machine_type : core :: default :: Default :: default () , min_cpu_platform : core :: default :: Default :: default () , preemptible : core :: default :: Default :: default () , spot : core :: default :: Default :: default () , } } }
pub struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElConfigElRef { fn new (shared : StackShared , base : String) -> DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElConfigElRef { DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `local_ssd_count` after provisioning.\nThe minimum number of nodes in the node pool. Must be >= 0 and <= maxNodeCount."] pub fn local_ssd_count (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.local_ssd_count" , self . base)) } # [doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe name of a Compute Engine machine type."] pub fn machine_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.machine_type" , self . base)) } # [doc = "Get a reference to the value of field `min_cpu_platform` after provisioning.\nMinimum CPU platform to be used by this instance. The instance may be scheduled on the specified or a newer CPU platform. Specify the friendly names of CPU platforms, such as \"Intel Haswell\" or \"Intel Sandy Bridge\"."] pub fn min_cpu_platform (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.min_cpu_platform" , self . base)) } # [doc = "Get a reference to the value of field `preemptible` after provisioning.\nWhether the nodes are created as preemptible VM instances. Preemptible nodes cannot be used in a node pool with the CONTROLLER role or in the DEFAULT node pool if the CONTROLLER role is not assigned (the DEFAULT node pool will assume the CONTROLLER role)."] pub fn preemptible (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.preemptible" , self . base)) } # [doc = "Get a reference to the value of field `spot` after provisioning.\nSpot flag for enabling Spot VM, which is a rebrand of the existing preemptible flag."] pub fn spot (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.spot" , self . base)) } }
#[derive(Serialize, Default)]
struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElDynamic { autoscaling : Option < DynamicBlock < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElAutoscalingEl >> , config : Option < DynamicBlock < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElConfigEl >> , }
#[derive(Serialize)]
pub struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigEl { locations : SetField < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] autoscaling : Option < Vec < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElAutoscalingEl > > , # [serde (skip_serializing_if = "Option::is_none")] config : Option < Vec < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElConfigEl > > , dynamic : DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElDynamic , }
impl DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigEl { # [doc = "Set the field `autoscaling`.\n"] pub fn set_autoscaling (mut self , v : impl Into < BlockAssignable < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElAutoscalingEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . autoscaling = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . autoscaling = Some (d) ; } } self } # [doc = "Set the field `config`.\n"] pub fn set_config (mut self , v : impl Into < BlockAssignable < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElConfigEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . config = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . config = Some (d) ; } } self } }
impl ToListMappable for DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigEl { type O = BlockAssignable < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigEl
{
    #[doc = "The list of Compute Engine zones where node pool nodes associated with a Dataproc on GKE virtual cluster will be located."]
    pub locations: SetField<PrimField<String>>,
}
impl BuildDataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigEl { pub fn build (self) -> DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigEl { DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigEl { locations : self . locations , autoscaling : core :: default :: Default :: default () , config : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElRef { fn new (shared : StackShared , base : String) -> DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElRef { DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `locations` after provisioning.\nThe list of Compute Engine zones where node pool nodes associated with a Dataproc on GKE virtual cluster will be located."] pub fn locations (& self) -> SetRef < PrimExpr < String > > { SetRef :: new (self . shared () . clone () , format ! ("{}.locations" , self . base)) } # [doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"] pub fn autoscaling (& self) -> ListRef < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElAutoscalingElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.autoscaling" , self . base)) } # [doc = "Get a reference to the value of field `config` after provisioning.\n"] pub fn config (& self) -> ListRef < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.config" , self . base)) } }
#[derive(Serialize, Default)]
struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElDynamic { node_pool_config : Option < DynamicBlock < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigEl >> , }
#[derive(Serialize)]
pub struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetEl { node_pool : PrimField < String > , roles : SetField < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] node_pool_config : Option < Vec < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigEl > > , dynamic : DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElDynamic , }
impl
    DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetEl
{
    #[doc = "Set the field `node_pool_config`.\n"]
    pub fn set_node_pool_config(
        mut self,
        v : impl Into < BlockAssignable < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_pool_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_pool_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetEl { type O = BlockAssignable < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetEl
{
    #[doc = "The target GKE node pool. Format: 'projects/{project}/locations/{location}/clusters/{cluster}/nodePools/{nodePool}'"]
    pub node_pool: PrimField<String>,
    #[doc = "The roles associated with the GKE node pool."]
    pub roles: SetField<PrimField<String>>,
}
impl BuildDataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetEl { pub fn build (self) -> DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetEl { DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetEl { node_pool : self . node_pool , roles : self . roles , node_pool_config : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElRef { fn new (shared : StackShared , base : String) -> DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElRef { DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElRef { shared : shared , base : base . to_string () , } } }
impl DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `node_pool` after provisioning.\nThe target GKE node pool. Format: 'projects/{project}/locations/{location}/clusters/{cluster}/nodePools/{nodePool}'"] pub fn node_pool (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.node_pool" , self . base)) } # [doc = "Get a reference to the value of field `roles` after provisioning.\nThe roles associated with the GKE node pool."] pub fn roles (& self) -> SetRef < PrimExpr < String > > { SetRef :: new (self . shared () . clone () , format ! ("{}.roles" , self . base)) } # [doc = "Get a reference to the value of field `node_pool_config` after provisioning.\n"] pub fn node_pool_config (& self) -> ListRef < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElNodePoolConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.node_pool_config" , self . base)) } }
#[derive(Serialize, Default)]
struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElDynamic { node_pool_target : Option < DynamicBlock < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetEl >> , }
#[derive(Serialize)]
pub struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigEl { # [serde (skip_serializing_if = "Option::is_none")] gke_cluster_target : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] node_pool_target : Option < Vec < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetEl > > , dynamic : DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElDynamic , }
impl DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigEl {
    #[doc = "Set the field `gke_cluster_target`.\nA target GKE cluster to deploy to. It must be in the same project and region as the Dataproc cluster (the GKE cluster can be zonal or regional). Format: 'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub fn set_gke_cluster_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gke_cluster_target = Some(v.into());
        self
    }
    #[doc = "Set the field `node_pool_target`.\n"]
    pub fn set_node_pool_target(
        mut self,
        v : impl Into < BlockAssignable < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_pool_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_pool_target = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigEl
{
    type O = BlockAssignable<
        DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigEl {
}
impl BuildDataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigEl {
    pub fn build(
        self,
    ) -> DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigEl {
        DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigEl {
            gke_cluster_target: core::default::Default::default(),
            node_pool_target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElRef {
        DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gke_cluster_target` after provisioning.\nA target GKE cluster to deploy to. It must be in the same project and region as the Dataproc cluster (the GKE cluster can be zonal or regional). Format: 'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub fn gke_cluster_target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gke_cluster_target", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_pool_target` after provisioning.\n"]    pub fn node_pool_target (& self) -> ListRef < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElNodePoolTargetElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_pool_target", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElKubernetesSoftwareConfigEl
{
    component_version: RecField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
}
impl DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElKubernetesSoftwareConfigEl {
    #[doc = "Set the field `properties`.\nThe properties to set on daemon config files. Property keys are specified in prefix:property format, for example spark:spark.kubernetes.container.image."]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElKubernetesSoftwareConfigEl
{
    type O = BlockAssignable<
        DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElKubernetesSoftwareConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterVirtualClusterConfigElKubernetesClusterConfigElKubernetesSoftwareConfigEl
{
    #[doc = "The components that should be installed in this Dataproc cluster. The key must be a string from the KubernetesComponent enumeration. The value is the version of the software to be installed."]
    pub component_version: RecField<PrimField<String>>,
}
impl BuildDataprocClusterVirtualClusterConfigElKubernetesClusterConfigElKubernetesSoftwareConfigEl {
    pub fn build(
        self,
    ) -> DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElKubernetesSoftwareConfigEl
    {
        DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElKubernetesSoftwareConfigEl {
            component_version: self.component_version,
            properties: core::default::Default::default(),
        }
    }
}
pub struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElKubernetesSoftwareConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElKubernetesSoftwareConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElKubernetesSoftwareConfigElRef
    {
        DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElKubernetesSoftwareConfigElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElKubernetesSoftwareConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `component_version` after provisioning.\nThe components that should be installed in this Dataproc cluster. The key must be a string from the KubernetesComponent enumeration. The value is the version of the software to be installed."]
    pub fn component_version(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.component_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nThe properties to set on daemon config files. Property keys are specified in prefix:property format, for example spark:spark.kubernetes.container.image."]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElDynamic { gke_cluster_config : Option < DynamicBlock < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigEl >> , kubernetes_software_config : Option < DynamicBlock < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElKubernetesSoftwareConfigEl >> , }
#[derive(Serialize)]
pub struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigEl { # [serde (skip_serializing_if = "Option::is_none")] kubernetes_namespace : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] gke_cluster_config : Option < Vec < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] kubernetes_software_config : Option < Vec < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElKubernetesSoftwareConfigEl > > , dynamic : DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElDynamic , }
impl DataprocClusterVirtualClusterConfigElKubernetesClusterConfigEl {
    #[doc = "Set the field `kubernetes_namespace`.\nA namespace within the Kubernetes cluster to deploy into. If this namespace does not exist, it is created. If it exists, Dataproc verifies that another Dataproc VirtualCluster is not installed into it. If not specified, the name of the Dataproc Cluster is used."]
    pub fn set_kubernetes_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kubernetes_namespace = Some(v.into());
        self
    }
    #[doc = "Set the field `gke_cluster_config`.\n"]
    pub fn set_gke_cluster_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gke_cluster_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gke_cluster_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kubernetes_software_config`.\n"]
    pub fn set_kubernetes_software_config(
        mut self,
        v : impl Into < BlockAssignable < DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElKubernetesSoftwareConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kubernetes_software_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kubernetes_software_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocClusterVirtualClusterConfigElKubernetesClusterConfigEl {
    type O = BlockAssignable<DataprocClusterVirtualClusterConfigElKubernetesClusterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterVirtualClusterConfigElKubernetesClusterConfigEl {}
impl BuildDataprocClusterVirtualClusterConfigElKubernetesClusterConfigEl {
    pub fn build(self) -> DataprocClusterVirtualClusterConfigElKubernetesClusterConfigEl {
        DataprocClusterVirtualClusterConfigElKubernetesClusterConfigEl {
            kubernetes_namespace: core::default::Default::default(),
            gke_cluster_config: core::default::Default::default(),
            kubernetes_software_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElRef {
        DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kubernetes_namespace` after provisioning.\nA namespace within the Kubernetes cluster to deploy into. If this namespace does not exist, it is created. If it exists, Dataproc verifies that another Dataproc VirtualCluster is not installed into it. If not specified, the name of the Dataproc Cluster is used."]
    pub fn kubernetes_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kubernetes_namespace", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gke_cluster_config` after provisioning.\n"]
    pub fn gke_cluster_config(
        &self,
    ) -> ListRef<DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElGkeClusterConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gke_cluster_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kubernetes_software_config` after provisioning.\n"]
    pub fn kubernetes_software_config(
        &self,
    ) -> ListRef<
        DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElKubernetesSoftwareConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kubernetes_software_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocClusterVirtualClusterConfigElDynamic {
    auxiliary_services_config:
        Option<DynamicBlock<DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigEl>>,
    kubernetes_cluster_config:
        Option<DynamicBlock<DataprocClusterVirtualClusterConfigElKubernetesClusterConfigEl>>,
}
#[derive(Serialize)]
pub struct DataprocClusterVirtualClusterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    staging_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auxiliary_services_config:
        Option<Vec<DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kubernetes_cluster_config:
        Option<Vec<DataprocClusterVirtualClusterConfigElKubernetesClusterConfigEl>>,
    dynamic: DataprocClusterVirtualClusterConfigElDynamic,
}
impl DataprocClusterVirtualClusterConfigEl {
    #[doc = "Set the field `staging_bucket`.\nA Cloud Storage bucket used to stage job dependencies, config files, and job driver console output. If you do not specify a staging bucket, Cloud Dataproc will determine a Cloud Storage location (US, ASIA, or EU) for your cluster's staging bucket according to the Compute Engine zone where your cluster is deployed, and then create and manage this project-level, per-location bucket."]
    pub fn set_staging_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.staging_bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `auxiliary_services_config`.\n"]
    pub fn set_auxiliary_services_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auxiliary_services_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auxiliary_services_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kubernetes_cluster_config`.\n"]
    pub fn set_kubernetes_cluster_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocClusterVirtualClusterConfigElKubernetesClusterConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kubernetes_cluster_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kubernetes_cluster_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocClusterVirtualClusterConfigEl {
    type O = BlockAssignable<DataprocClusterVirtualClusterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocClusterVirtualClusterConfigEl {}
impl BuildDataprocClusterVirtualClusterConfigEl {
    pub fn build(self) -> DataprocClusterVirtualClusterConfigEl {
        DataprocClusterVirtualClusterConfigEl {
            staging_bucket: core::default::Default::default(),
            auxiliary_services_config: core::default::Default::default(),
            kubernetes_cluster_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocClusterVirtualClusterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocClusterVirtualClusterConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocClusterVirtualClusterConfigElRef {
        DataprocClusterVirtualClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocClusterVirtualClusterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `staging_bucket` after provisioning.\nA Cloud Storage bucket used to stage job dependencies, config files, and job driver console output. If you do not specify a staging bucket, Cloud Dataproc will determine a Cloud Storage location (US, ASIA, or EU) for your cluster's staging bucket according to the Compute Engine zone where your cluster is deployed, and then create and manage this project-level, per-location bucket."]
    pub fn staging_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.staging_bucket", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `auxiliary_services_config` after provisioning.\n"]
    pub fn auxiliary_services_config(
        &self,
    ) -> ListRef<DataprocClusterVirtualClusterConfigElAuxiliaryServicesConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auxiliary_services_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kubernetes_cluster_config` after provisioning.\n"]
    pub fn kubernetes_cluster_config(
        &self,
    ) -> ListRef<DataprocClusterVirtualClusterConfigElKubernetesClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kubernetes_cluster_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocClusterDynamic {
    cluster_config: Option<DynamicBlock<DataprocClusterClusterConfigEl>>,
    virtual_cluster_config: Option<DynamicBlock<DataprocClusterVirtualClusterConfigEl>>,
}
