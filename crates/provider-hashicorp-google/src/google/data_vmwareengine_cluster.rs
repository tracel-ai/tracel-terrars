use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataVmwareengineClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    parent: PrimField<String>,
}
struct DataVmwareengineCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVmwareengineClusterData>,
}
#[derive(Clone)]
pub struct DataVmwareengineCluster(Rc<DataVmwareengineCluster_>);
impl DataVmwareengineCluster {
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
    #[doc = "Get a reference to the value of field `autoscaling_settings` after provisioning.\nConfiguration of the autoscaling applied to this cluster"]
    pub fn autoscaling_settings(&self) -> ListRef<DataVmwareengineClusterAutoscalingSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `datastore_mount_config` after provisioning.\nOptional. Configuration to mount a datastore.\nMount can be done along with cluster create or during cluster update\nSince service subnet is not configured with ip range on mgmt cluster creation, mount on management cluster is done as update only\nfor unmount remove 'datastore_mount_config' config from the update of cluster resource"]
    pub fn datastore_mount_config(
        &self,
    ) -> ListRef<DataVmwareengineClusterDatastoreMountConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.datastore_mount_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `management` after provisioning.\nTrue if the cluster is a management cluster; false otherwise.\nThere can only be one management cluster in a private cloud and it has to be the first one."]
    pub fn management(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.management", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the Cluster."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_type_configs` after provisioning.\nThe map of cluster node types in this cluster,\nwhere the key is canonical identifier of the node type (corresponds to the NodeType)."]
    pub fn node_type_configs(&self) -> SetRef<DataVmwareengineClusterNodeTypeConfigsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.node_type_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the private cloud to create a new cluster in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the Cluster."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nLast updated time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
impl Referable for DataVmwareengineCluster {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataVmwareengineCluster {}
impl ToListMappable for DataVmwareengineCluster {
    type O = ListRef<DataVmwareengineClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataVmwareengineCluster_ {
    fn extract_datasource_type(&self) -> String {
        "google_vmwareengine_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataVmwareengineCluster {
    pub tf_id: String,
    #[doc = "The ID of the Cluster."]
    pub name: PrimField<String>,
    #[doc = "The resource name of the private cloud to create a new cluster in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub parent: PrimField<String>,
}
impl BuildDataVmwareengineCluster {
    pub fn build(self, stack: &mut Stack) -> DataVmwareengineCluster {
        let out = DataVmwareengineCluster(Rc::new(DataVmwareengineCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVmwareengineClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                parent: self.parent,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataVmwareengineClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataVmwareengineClusterRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `autoscaling_settings` after provisioning.\nConfiguration of the autoscaling applied to this cluster"]
    pub fn autoscaling_settings(&self) -> ListRef<DataVmwareengineClusterAutoscalingSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `datastore_mount_config` after provisioning.\nOptional. Configuration to mount a datastore.\nMount can be done along with cluster create or during cluster update\nSince service subnet is not configured with ip range on mgmt cluster creation, mount on management cluster is done as update only\nfor unmount remove 'datastore_mount_config' config from the update of cluster resource"]
    pub fn datastore_mount_config(
        &self,
    ) -> ListRef<DataVmwareengineClusterDatastoreMountConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.datastore_mount_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `management` after provisioning.\nTrue if the cluster is a management cluster; false otherwise.\nThere can only be one management cluster in a private cloud and it has to be the first one."]
    pub fn management(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.management", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the Cluster."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_type_configs` after provisioning.\nThe map of cluster node types in this cluster,\nwhere the key is canonical identifier of the node type (corresponds to the NodeType)."]
    pub fn node_type_configs(&self) -> SetRef<DataVmwareengineClusterNodeTypeConfigsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.node_type_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the private cloud to create a new cluster in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the Cluster."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nLast updated time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_in: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_out: Option<PrimField<f64>>,
}
impl DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl {
    #[doc = "Set the field `scale_in`.\n"]
    pub fn set_scale_in(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scale_in = Some(v.into());
        self
    }
    #[doc = "Set the field `scale_out`.\n"]
    pub fn set_scale_out(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scale_out = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl
{
    type O = BlockAssignable<
        DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl
{}
impl
    BuildDataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl
{
    pub fn build(
        self,
    ) -> DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl
    {
        DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl { scale_in : core :: default :: Default :: default () , scale_out : core :: default :: Default :: default () , }
    }
}
pub struct DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsElRef { fn new (shared : StackShared , base : String) -> DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsElRef { DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsElRef { shared : shared , base : base . to_string () , } } }
impl
    DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scale_in` after provisioning.\n"]
    pub fn scale_in(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }
    #[doc = "Get a reference to the value of field `scale_out` after provisioning.\n"]
    pub fn scale_out(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_in: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_out: Option<PrimField<f64>>,
}
impl DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl {
    #[doc = "Set the field `scale_in`.\n"]
    pub fn set_scale_in(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scale_in = Some(v.into());
        self
    }
    #[doc = "Set the field `scale_out`.\n"]
    pub fn set_scale_out(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scale_out = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl
{
    type O = BlockAssignable<
        DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl {
}
impl BuildDataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl {
    pub fn build(
        self,
    ) -> DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl {
        DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl {
            scale_in: core::default::Default::default(),
            scale_out: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsElRef {
        DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scale_in` after provisioning.\n"]
    pub fn scale_in(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }
    #[doc = "Get a reference to the value of field `scale_out` after provisioning.\n"]
    pub fn scale_out(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_in: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_out: Option<PrimField<f64>>,
}
impl DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl {
    #[doc = "Set the field `scale_in`.\n"]
    pub fn set_scale_in(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scale_in = Some(v.into());
        self
    }
    #[doc = "Set the field `scale_out`.\n"]
    pub fn set_scale_out(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scale_out = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl
{
    type O = BlockAssignable<
        DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl
{}
impl BuildDataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl {
    pub fn build(
        self,
    ) -> DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl {
        DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl {
            scale_in: core::default::Default::default(),
            scale_out: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsElRef
    {
        DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scale_in` after provisioning.\n"]
    pub fn scale_in(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }
    #[doc = "Get a reference to the value of field `scale_out` after provisioning.\n"]
    pub fn scale_out(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl { # [serde (skip_serializing_if = "Option::is_none")] autoscale_policy_id : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] consumed_memory_thresholds : Option < ListField < DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl > > , # [serde (skip_serializing_if = "Option::is_none")] cpu_thresholds : Option < ListField < DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl > > , # [serde (skip_serializing_if = "Option::is_none")] node_type_id : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] scale_out_size : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] storage_thresholds : Option < ListField < DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl > > , }
impl DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl {
    #[doc = "Set the field `autoscale_policy_id`.\n"]
    pub fn set_autoscale_policy_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.autoscale_policy_id = Some(v.into());
        self
    }
    #[doc = "Set the field `consumed_memory_thresholds`.\n"]
    pub fn set_consumed_memory_thresholds(
        mut self,
        v : impl Into < ListField < DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl > >,
    ) -> Self {
        self.consumed_memory_thresholds = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_thresholds`.\n"]
    pub fn set_cpu_thresholds(
        mut self,
        v: impl Into<
            ListField<
                DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl,
            >,
        >,
    ) -> Self {
        self.cpu_thresholds = Some(v.into());
        self
    }
    #[doc = "Set the field `node_type_id`.\n"]
    pub fn set_node_type_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_type_id = Some(v.into());
        self
    }
    #[doc = "Set the field `scale_out_size`.\n"]
    pub fn set_scale_out_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scale_out_size = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_thresholds`.\n"]
    pub fn set_storage_thresholds(
        mut self,
        v : impl Into < ListField < DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl > >,
    ) -> Self {
        self.storage_thresholds = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl {
    type O = BlockAssignable<DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl {}
impl BuildDataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl {
    pub fn build(self) -> DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl {
        DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl {
            autoscale_policy_id: core::default::Default::default(),
            consumed_memory_thresholds: core::default::Default::default(),
            cpu_thresholds: core::default::Default::default(),
            node_type_id: core::default::Default::default(),
            scale_out_size: core::default::Default::default(),
            storage_thresholds: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElRef {
        DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `autoscale_policy_id` after provisioning.\n"]
    pub fn autoscale_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.autoscale_policy_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `consumed_memory_thresholds` after provisioning.\n"]    pub fn consumed_memory_thresholds (& self) -> ListRef < DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.consumed_memory_thresholds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_thresholds` after provisioning.\n"]
    pub fn cpu_thresholds(
        &self,
    ) -> ListRef<DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cpu_thresholds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_type_id` after provisioning.\n"]
    pub fn node_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }
    #[doc = "Get a reference to the value of field `scale_out_size` after provisioning.\n"]
    pub fn scale_out_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scale_out_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_thresholds` after provisioning.\n"]
    pub fn storage_thresholds(
        &self,
    ) -> ListRef<
        DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_thresholds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineClusterAutoscalingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_policies:
        Option<SetField<DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cool_down_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_cluster_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_cluster_node_count: Option<PrimField<f64>>,
}
impl DataVmwareengineClusterAutoscalingSettingsEl {
    #[doc = "Set the field `autoscaling_policies`.\n"]
    pub fn set_autoscaling_policies(
        mut self,
        v: impl Into<SetField<DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl>>,
    ) -> Self {
        self.autoscaling_policies = Some(v.into());
        self
    }
    #[doc = "Set the field `cool_down_period`.\n"]
    pub fn set_cool_down_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cool_down_period = Some(v.into());
        self
    }
    #[doc = "Set the field `max_cluster_node_count`.\n"]
    pub fn set_max_cluster_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_cluster_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `min_cluster_node_count`.\n"]
    pub fn set_min_cluster_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_cluster_node_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineClusterAutoscalingSettingsEl {
    type O = BlockAssignable<DataVmwareengineClusterAutoscalingSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineClusterAutoscalingSettingsEl {}
impl BuildDataVmwareengineClusterAutoscalingSettingsEl {
    pub fn build(self) -> DataVmwareengineClusterAutoscalingSettingsEl {
        DataVmwareengineClusterAutoscalingSettingsEl {
            autoscaling_policies: core::default::Default::default(),
            cool_down_period: core::default::Default::default(),
            max_cluster_node_count: core::default::Default::default(),
            min_cluster_node_count: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineClusterAutoscalingSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineClusterAutoscalingSettingsElRef {
    fn new(shared: StackShared, base: String) -> DataVmwareengineClusterAutoscalingSettingsElRef {
        DataVmwareengineClusterAutoscalingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineClusterAutoscalingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `autoscaling_policies` after provisioning.\n"]
    pub fn autoscaling_policies(
        &self,
    ) -> SetRef<DataVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_policies", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cool_down_period` after provisioning.\n"]
    pub fn cool_down_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cool_down_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_cluster_node_count` after provisioning.\n"]
    pub fn max_cluster_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_cluster_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_cluster_node_count` after provisioning.\n"]
    pub fn min_cluster_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_cluster_node_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mtu: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_peering: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet: Option<PrimField<String>>,
}
impl DataVmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl {
    #[doc = "Set the field `connection_count`.\n"]
    pub fn set_connection_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.connection_count = Some(v.into());
        self
    }
    #[doc = "Set the field `mtu`.\n"]
    pub fn set_mtu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.mtu = Some(v.into());
        self
    }
    #[doc = "Set the field `network_peering`.\n"]
    pub fn set_network_peering(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_peering = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet`.\n"]
    pub fn set_subnet(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl {
    type O = BlockAssignable<DataVmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl {}
impl BuildDataVmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl {
    pub fn build(self) -> DataVmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl {
        DataVmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl {
            connection_count: core::default::Default::default(),
            mtu: core::default::Default::default(),
            network_peering: core::default::Default::default(),
            subnet: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineClusterDatastoreMountConfigElDatastoreNetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineClusterDatastoreMountConfigElDatastoreNetworkElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVmwareengineClusterDatastoreMountConfigElDatastoreNetworkElRef {
        DataVmwareengineClusterDatastoreMountConfigElDatastoreNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineClusterDatastoreMountConfigElDatastoreNetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_count` after provisioning.\n"]
    pub fn connection_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mtu` after provisioning.\n"]
    pub fn mtu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mtu", self.base))
    }
    #[doc = "Get a reference to the value of field `network_peering` after provisioning.\n"]
    pub fn network_peering(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_peering", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineClusterDatastoreMountConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datastore: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datastore_network:
        Option<ListField<DataVmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_share: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_colocation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    servers: Option<ListField<PrimField<String>>>,
}
impl DataVmwareengineClusterDatastoreMountConfigEl {
    #[doc = "Set the field `access_mode`.\n"]
    pub fn set_access_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `datastore`.\n"]
    pub fn set_datastore(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.datastore = Some(v.into());
        self
    }
    #[doc = "Set the field `datastore_network`.\n"]
    pub fn set_datastore_network(
        mut self,
        v: impl Into<ListField<DataVmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl>>,
    ) -> Self {
        self.datastore_network = Some(v.into());
        self
    }
    #[doc = "Set the field `file_share`.\n"]
    pub fn set_file_share(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_share = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_colocation`.\n"]
    pub fn set_ignore_colocation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_colocation = Some(v.into());
        self
    }
    #[doc = "Set the field `nfs_version`.\n"]
    pub fn set_nfs_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nfs_version = Some(v.into());
        self
    }
    #[doc = "Set the field `servers`.\n"]
    pub fn set_servers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.servers = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineClusterDatastoreMountConfigEl {
    type O = BlockAssignable<DataVmwareengineClusterDatastoreMountConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineClusterDatastoreMountConfigEl {}
impl BuildDataVmwareengineClusterDatastoreMountConfigEl {
    pub fn build(self) -> DataVmwareengineClusterDatastoreMountConfigEl {
        DataVmwareengineClusterDatastoreMountConfigEl {
            access_mode: core::default::Default::default(),
            datastore: core::default::Default::default(),
            datastore_network: core::default::Default::default(),
            file_share: core::default::Default::default(),
            ignore_colocation: core::default::Default::default(),
            nfs_version: core::default::Default::default(),
            servers: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineClusterDatastoreMountConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineClusterDatastoreMountConfigElRef {
    fn new(shared: StackShared, base: String) -> DataVmwareengineClusterDatastoreMountConfigElRef {
        DataVmwareengineClusterDatastoreMountConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineClusterDatastoreMountConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_mode` after provisioning.\n"]
    pub fn access_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `datastore` after provisioning.\n"]
    pub fn datastore(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.datastore", self.base))
    }
    #[doc = "Get a reference to the value of field `datastore_network` after provisioning.\n"]
    pub fn datastore_network(
        &self,
    ) -> ListRef<DataVmwareengineClusterDatastoreMountConfigElDatastoreNetworkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.datastore_network", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `file_share` after provisioning.\n"]
    pub fn file_share(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_share", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_colocation` after provisioning.\n"]
    pub fn ignore_colocation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_colocation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nfs_version` after provisioning.\n"]
    pub fn nfs_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nfs_version", self.base))
    }
    #[doc = "Get a reference to the value of field `servers` after provisioning.\n"]
    pub fn servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.servers", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineClusterNodeTypeConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_core_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_type_id: Option<PrimField<String>>,
}
impl DataVmwareengineClusterNodeTypeConfigsEl {
    #[doc = "Set the field `custom_core_count`.\n"]
    pub fn set_custom_core_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.custom_core_count = Some(v.into());
        self
    }
    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `node_type_id`.\n"]
    pub fn set_node_type_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_type_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineClusterNodeTypeConfigsEl {
    type O = BlockAssignable<DataVmwareengineClusterNodeTypeConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineClusterNodeTypeConfigsEl {}
impl BuildDataVmwareengineClusterNodeTypeConfigsEl {
    pub fn build(self) -> DataVmwareengineClusterNodeTypeConfigsEl {
        DataVmwareengineClusterNodeTypeConfigsEl {
            custom_core_count: core::default::Default::default(),
            node_count: core::default::Default::default(),
            node_type_id: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineClusterNodeTypeConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineClusterNodeTypeConfigsElRef {
    fn new(shared: StackShared, base: String) -> DataVmwareengineClusterNodeTypeConfigsElRef {
        DataVmwareengineClusterNodeTypeConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineClusterNodeTypeConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_core_count` after provisioning.\n"]
    pub fn custom_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_core_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }
    #[doc = "Get a reference to the value of field `node_type_id` after provisioning.\n"]
    pub fn node_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }
}
