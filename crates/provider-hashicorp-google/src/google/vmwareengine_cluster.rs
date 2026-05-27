use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VmwareengineClusterData {
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
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_settings: Option<Vec<VmwareengineClusterAutoscalingSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datastore_mount_config: Option<Vec<VmwareengineClusterDatastoreMountConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_type_configs: Option<Vec<VmwareengineClusterNodeTypeConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VmwareengineClusterTimeoutsEl>,
    dynamic: VmwareengineClusterDynamic,
}
struct VmwareengineCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VmwareengineClusterData>,
}
#[derive(Clone)]
pub struct VmwareengineCluster(Rc<VmwareengineCluster_>);
impl VmwareengineCluster {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_settings`.\n"]
    pub fn set_autoscaling_settings(
        self,
        v: impl Into<BlockAssignable<VmwareengineClusterAutoscalingSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autoscaling_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autoscaling_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `datastore_mount_config`.\n"]
    pub fn set_datastore_mount_config(
        self,
        v: impl Into<BlockAssignable<VmwareengineClusterDatastoreMountConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().datastore_mount_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.datastore_mount_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_type_configs`.\n"]
    pub fn set_node_type_configs(
        self,
        v: impl Into<BlockAssignable<VmwareengineClusterNodeTypeConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_type_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_type_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VmwareengineClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
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
    #[doc = "Get a reference to the value of field `autoscaling_settings` after provisioning.\n"]
    pub fn autoscaling_settings(&self) -> ListRef<VmwareengineClusterAutoscalingSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `datastore_mount_config` after provisioning.\n"]
    pub fn datastore_mount_config(&self) -> ListRef<VmwareengineClusterDatastoreMountConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.datastore_mount_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineClusterTimeoutsElRef {
        VmwareengineClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VmwareengineCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VmwareengineCluster {}
impl ToListMappable for VmwareengineCluster {
    type O = ListRef<VmwareengineClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VmwareengineCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_vmwareengine_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVmwareengineCluster {
    pub tf_id: String,
    #[doc = "The ID of the Cluster."]
    pub name: PrimField<String>,
    #[doc = "The resource name of the private cloud to create a new cluster in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub parent: PrimField<String>,
}
impl BuildVmwareengineCluster {
    pub fn build(self, stack: &mut Stack) -> VmwareengineCluster {
        let out = VmwareengineCluster(Rc::new(VmwareengineCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VmwareengineClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parent: self.parent,
                autoscaling_settings: core::default::Default::default(),
                datastore_mount_config: core::default::Default::default(),
                node_type_configs: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VmwareengineClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VmwareengineClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
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
    #[doc = "Get a reference to the value of field `autoscaling_settings` after provisioning.\n"]
    pub fn autoscaling_settings(&self) -> ListRef<VmwareengineClusterAutoscalingSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `datastore_mount_config` after provisioning.\n"]
    pub fn datastore_mount_config(&self) -> ListRef<VmwareengineClusterDatastoreMountConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.datastore_mount_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineClusterTimeoutsElRef {
        VmwareengineClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl {
    scale_in: PrimField<f64>,
    scale_out: PrimField<f64>,
}
impl VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl {}
impl ToListMappable
    for VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl
{
    type O = BlockAssignable<
        VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl
{
    #[doc = "The utilization triggering the scale-in operation in percent."]
    pub scale_in: PrimField<f64>,
    #[doc = "The utilization triggering the scale-out operation in percent."]
    pub scale_out: PrimField<f64>,
}
impl BuildVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl {
    pub fn build(
        self,
    ) -> VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl
    {
        VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl {
            scale_in: self.scale_in,
            scale_out: self.scale_out,
        }
    }
}
pub struct VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsElRef
    {
        VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scale_in` after provisioning.\nThe utilization triggering the scale-in operation in percent."]
    pub fn scale_in(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }
    #[doc = "Get a reference to the value of field `scale_out` after provisioning.\nThe utilization triggering the scale-out operation in percent."]
    pub fn scale_out(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}
#[derive(Serialize)]
pub struct VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl {
    scale_in: PrimField<f64>,
    scale_out: PrimField<f64>,
}
impl VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl {}
impl ToListMappable
    for VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl
{
    type O = BlockAssignable<
        VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl {
    #[doc = "The utilization triggering the scale-in operation in percent."]
    pub scale_in: PrimField<f64>,
    #[doc = "The utilization triggering the scale-out operation in percent."]
    pub scale_out: PrimField<f64>,
}
impl BuildVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl {
    pub fn build(
        self,
    ) -> VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl {
        VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl {
            scale_in: self.scale_in,
            scale_out: self.scale_out,
        }
    }
}
pub struct VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsElRef {
        VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scale_in` after provisioning.\nThe utilization triggering the scale-in operation in percent."]
    pub fn scale_in(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }
    #[doc = "Get a reference to the value of field `scale_out` after provisioning.\nThe utilization triggering the scale-out operation in percent."]
    pub fn scale_out(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}
#[derive(Serialize)]
pub struct VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl {
    scale_in: PrimField<f64>,
    scale_out: PrimField<f64>,
}
impl VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl {}
impl ToListMappable
    for VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl
{
    type O = BlockAssignable<
        VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl {
    #[doc = "The utilization triggering the scale-in operation in percent."]
    pub scale_in: PrimField<f64>,
    #[doc = "The utilization triggering the scale-out operation in percent."]
    pub scale_out: PrimField<f64>,
}
impl BuildVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl {
    pub fn build(
        self,
    ) -> VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl {
        VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl {
            scale_in: self.scale_in,
            scale_out: self.scale_out,
        }
    }
}
pub struct VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsElRef {
        VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scale_in` after provisioning.\nThe utilization triggering the scale-in operation in percent."]
    pub fn scale_in(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_in", self.base))
    }
    #[doc = "Get a reference to the value of field `scale_out` after provisioning.\nThe utilization triggering the scale-out operation in percent."]
    pub fn scale_out(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_out", self.base))
    }
}
#[derive(Serialize, Default)]
struct VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElDynamic {
    consumed_memory_thresholds: Option<
        DynamicBlock<
            VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl,
        >,
    >,
    cpu_thresholds: Option<
        DynamicBlock<VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl>,
    >,
    storage_thresholds: Option<
        DynamicBlock<
            VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl {
    autoscale_policy_id: PrimField<String>,
    node_type_id: PrimField<String>,
    scale_out_size: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumed_memory_thresholds: Option<
        Vec<
            VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_thresholds:
        Option<Vec<VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_thresholds: Option<
        Vec<VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl>,
    >,
    dynamic: VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElDynamic,
}
impl VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl {
    #[doc = "Set the field `consumed_memory_thresholds`.\n"]
    pub fn set_consumed_memory_thresholds(
        mut self,
        v : impl Into < BlockAssignable < VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.consumed_memory_thresholds = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.consumed_memory_thresholds = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cpu_thresholds`.\n"]
    pub fn set_cpu_thresholds(
        mut self,
        v: impl Into<
            BlockAssignable<
                VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cpu_thresholds = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cpu_thresholds = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `storage_thresholds`.\n"]
    pub fn set_storage_thresholds(
        mut self,
        v: impl Into<
            BlockAssignable<
                VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.storage_thresholds = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_thresholds = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl {
    type O = BlockAssignable<VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl {
    #[doc = ""]
    pub autoscale_policy_id: PrimField<String>,
    #[doc = "The canonical identifier of the node type to add or remove."]
    pub node_type_id: PrimField<String>,
    #[doc = "Number of nodes to add to a cluster during a scale-out operation.\nMust be divisible by 2 for stretched clusters."]
    pub scale_out_size: PrimField<f64>,
}
impl BuildVmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl {
    pub fn build(self) -> VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl {
        VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl {
            autoscale_policy_id: self.autoscale_policy_id,
            node_type_id: self.node_type_id,
            scale_out_size: self.scale_out_size,
            consumed_memory_thresholds: core::default::Default::default(),
            cpu_thresholds: core::default::Default::default(),
            storage_thresholds: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElRef {
        VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElRef {
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
    #[doc = "Get a reference to the value of field `node_type_id` after provisioning.\nThe canonical identifier of the node type to add or remove."]
    pub fn node_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }
    #[doc = "Get a reference to the value of field `scale_out_size` after provisioning.\nNumber of nodes to add to a cluster during a scale-out operation.\nMust be divisible by 2 for stretched clusters."]
    pub fn scale_out_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scale_out_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `consumed_memory_thresholds` after provisioning.\n"]
    pub fn consumed_memory_thresholds(
        &self,
    ) -> ListRef<
        VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElConsumedMemoryThresholdsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consumed_memory_thresholds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_thresholds` after provisioning.\n"]
    pub fn cpu_thresholds(
        &self,
    ) -> ListRef<VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElCpuThresholdsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cpu_thresholds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_thresholds` after provisioning.\n"]
    pub fn storage_thresholds(
        &self,
    ) -> ListRef<VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesElStorageThresholdsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_thresholds", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct VmwareengineClusterAutoscalingSettingsElDynamic {
    autoscaling_policies:
        Option<DynamicBlock<VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl>>,
}
#[derive(Serialize)]
pub struct VmwareengineClusterAutoscalingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cool_down_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_cluster_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_cluster_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_policies:
        Option<Vec<VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl>>,
    dynamic: VmwareengineClusterAutoscalingSettingsElDynamic,
}
impl VmwareengineClusterAutoscalingSettingsEl {
    #[doc = "Set the field `cool_down_period`.\nThe minimum duration between consecutive autoscale operations.\nIt starts once addition or removal of nodes is fully completed.\nMinimum cool down period is 30m.\nCool down period must be in whole minutes (for example, 30m, 31m, 50m).\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn set_cool_down_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cool_down_period = Some(v.into());
        self
    }
    #[doc = "Set the field `max_cluster_node_count`.\nMaximum number of nodes of any type in a cluster.\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn set_max_cluster_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_cluster_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `min_cluster_node_count`.\nMinimum number of nodes of any type in a cluster.\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn set_min_cluster_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_cluster_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `autoscaling_policies`.\n"]
    pub fn set_autoscaling_policies(
        mut self,
        v: impl Into<BlockAssignable<VmwareengineClusterAutoscalingSettingsElAutoscalingPoliciesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.autoscaling_policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_policies = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VmwareengineClusterAutoscalingSettingsEl {
    type O = BlockAssignable<VmwareengineClusterAutoscalingSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineClusterAutoscalingSettingsEl {}
impl BuildVmwareengineClusterAutoscalingSettingsEl {
    pub fn build(self) -> VmwareengineClusterAutoscalingSettingsEl {
        VmwareengineClusterAutoscalingSettingsEl {
            cool_down_period: core::default::Default::default(),
            max_cluster_node_count: core::default::Default::default(),
            min_cluster_node_count: core::default::Default::default(),
            autoscaling_policies: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VmwareengineClusterAutoscalingSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineClusterAutoscalingSettingsElRef {
    fn new(shared: StackShared, base: String) -> VmwareengineClusterAutoscalingSettingsElRef {
        VmwareengineClusterAutoscalingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineClusterAutoscalingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cool_down_period` after provisioning.\nThe minimum duration between consecutive autoscale operations.\nIt starts once addition or removal of nodes is fully completed.\nMinimum cool down period is 30m.\nCool down period must be in whole minutes (for example, 30m, 31m, 50m).\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn cool_down_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cool_down_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_cluster_node_count` after provisioning.\nMaximum number of nodes of any type in a cluster.\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn max_cluster_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_cluster_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_cluster_node_count` after provisioning.\nMinimum number of nodes of any type in a cluster.\nMandatory for successful addition of autoscaling settings in cluster."]
    pub fn min_cluster_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_cluster_node_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mtu: Option<PrimField<f64>>,
    subnet: PrimField<String>,
}
impl VmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl {
    #[doc = "Set the field `connection_count`.\nOptional. The number of connections of the NFS volume.\nSupported from vsphere 8.0u1. Possible values are 1-4.\nDefault value is 4."]
    pub fn set_connection_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.connection_count = Some(v.into());
        self
    }
    #[doc = "Set the field `mtu`.\nOptional. The Maximal Transmission Unit (MTU) of the datastore.\nMTU value can range from 1330-9000. If not set, system sets\ndefault MTU size to 1500."]
    pub fn set_mtu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.mtu = Some(v.into());
        self
    }
}
impl ToListMappable for VmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl {
    type O = BlockAssignable<VmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl {
    #[doc = "The resource name of the subnet\nResource names are schemeless URIs that follow the conventions in\nhttps://cloud.google.com/apis/design/resource_names.\ne.g. projects/my-project/locations/us-central1/subnets/my-subnet"]
    pub subnet: PrimField<String>,
}
impl BuildVmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl {
    pub fn build(self) -> VmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl {
        VmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl {
            connection_count: core::default::Default::default(),
            mtu: core::default::Default::default(),
            subnet: self.subnet,
        }
    }
}
pub struct VmwareengineClusterDatastoreMountConfigElDatastoreNetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineClusterDatastoreMountConfigElDatastoreNetworkElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VmwareengineClusterDatastoreMountConfigElDatastoreNetworkElRef {
        VmwareengineClusterDatastoreMountConfigElDatastoreNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineClusterDatastoreMountConfigElDatastoreNetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_count` after provisioning.\nOptional. The number of connections of the NFS volume.\nSupported from vsphere 8.0u1. Possible values are 1-4.\nDefault value is 4."]
    pub fn connection_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mtu` after provisioning.\nOptional. The Maximal Transmission Unit (MTU) of the datastore.\nMTU value can range from 1330-9000. If not set, system sets\ndefault MTU size to 1500."]
    pub fn mtu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mtu", self.base))
    }
    #[doc = "Get a reference to the value of field `network_peering` after provisioning.\nThe resource name of the network peering, used to access the\nfile share by clients on private cloud. Resource names are schemeless\nURIs that follow the conventions in\nhttps://cloud.google.com/apis/design/resource_names.\ne.g.\nprojects/my-project/locations/us-central1/networkPeerings/my-network-peering"]
    pub fn network_peering(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_peering", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnet` after provisioning.\nThe resource name of the subnet\nResource names are schemeless URIs that follow the conventions in\nhttps://cloud.google.com/apis/design/resource_names.\ne.g. projects/my-project/locations/us-central1/subnets/my-subnet"]
    pub fn subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet", self.base))
    }
}
#[derive(Serialize, Default)]
struct VmwareengineClusterDatastoreMountConfigElDynamic {
    datastore_network:
        Option<DynamicBlock<VmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl>>,
}
#[derive(Serialize)]
pub struct VmwareengineClusterDatastoreMountConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_mode: Option<PrimField<String>>,
    datastore: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_colocation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datastore_network: Option<Vec<VmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl>>,
    dynamic: VmwareengineClusterDatastoreMountConfigElDynamic,
}
impl VmwareengineClusterDatastoreMountConfigEl {
    #[doc = "Set the field `access_mode`.\nOptional. NFS is accessed by hosts in either read or read_write mode\nDefault value used will be READ_WRITE\nPossible values:\nREAD_ONLY\nREAD_WRITE"]
    pub fn set_access_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_colocation`.\nOptional. If set to true, the colocation requirement will be ignored.\nIf set to false, the colocation requirement will be enforced.\nColocation requirement is the requirement that the cluster must be in the\nsame region/zone of datastore."]
    pub fn set_ignore_colocation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_colocation = Some(v.into());
        self
    }
    #[doc = "Set the field `nfs_version`.\nOptional. The NFS protocol supported by the NFS volume.\nDefault value used will be NFS_V3\nPossible values:\nNFS_V3"]
    pub fn set_nfs_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nfs_version = Some(v.into());
        self
    }
    #[doc = "Set the field `datastore_network`.\n"]
    pub fn set_datastore_network(
        mut self,
        v: impl Into<BlockAssignable<VmwareengineClusterDatastoreMountConfigElDatastoreNetworkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.datastore_network = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.datastore_network = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VmwareengineClusterDatastoreMountConfigEl {
    type O = BlockAssignable<VmwareengineClusterDatastoreMountConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineClusterDatastoreMountConfigEl {
    #[doc = "The resource name of the datastore to unmount.\nThe datastore requested to be mounted should be in same region/zone as the\ncluster.\nResource names are schemeless URIs that follow the conventions in\nhttps://cloud.google.com/apis/design/resource_names.\nFor example:\n'projects/my-project/locations/us-central1/datastores/my-datastore'"]
    pub datastore: PrimField<String>,
}
impl BuildVmwareengineClusterDatastoreMountConfigEl {
    pub fn build(self) -> VmwareengineClusterDatastoreMountConfigEl {
        VmwareengineClusterDatastoreMountConfigEl {
            access_mode: core::default::Default::default(),
            datastore: self.datastore,
            ignore_colocation: core::default::Default::default(),
            nfs_version: core::default::Default::default(),
            datastore_network: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VmwareengineClusterDatastoreMountConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineClusterDatastoreMountConfigElRef {
    fn new(shared: StackShared, base: String) -> VmwareengineClusterDatastoreMountConfigElRef {
        VmwareengineClusterDatastoreMountConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineClusterDatastoreMountConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_mode` after provisioning.\nOptional. NFS is accessed by hosts in either read or read_write mode\nDefault value used will be READ_WRITE\nPossible values:\nREAD_ONLY\nREAD_WRITE"]
    pub fn access_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `datastore` after provisioning.\nThe resource name of the datastore to unmount.\nThe datastore requested to be mounted should be in same region/zone as the\ncluster.\nResource names are schemeless URIs that follow the conventions in\nhttps://cloud.google.com/apis/design/resource_names.\nFor example:\n'projects/my-project/locations/us-central1/datastores/my-datastore'"]
    pub fn datastore(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.datastore", self.base))
    }
    #[doc = "Get a reference to the value of field `file_share` after provisioning.\nFile share name."]
    pub fn file_share(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_share", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_colocation` after provisioning.\nOptional. If set to true, the colocation requirement will be ignored.\nIf set to false, the colocation requirement will be enforced.\nColocation requirement is the requirement that the cluster must be in the\nsame region/zone of datastore."]
    pub fn ignore_colocation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_colocation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `nfs_version` after provisioning.\nOptional. The NFS protocol supported by the NFS volume.\nDefault value used will be NFS_V3\nPossible values:\nNFS_V3"]
    pub fn nfs_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nfs_version", self.base))
    }
    #[doc = "Get a reference to the value of field `servers` after provisioning.\nServer IP addresses of the NFS volume.\nFor NFS 3, you can only provide a single\nserver IP address or DNS names."]
    pub fn servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.servers", self.base))
    }
    #[doc = "Get a reference to the value of field `datastore_network` after provisioning.\n"]
    pub fn datastore_network(
        &self,
    ) -> ListRef<VmwareengineClusterDatastoreMountConfigElDatastoreNetworkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.datastore_network", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VmwareengineClusterNodeTypeConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_core_count: Option<PrimField<f64>>,
    node_count: PrimField<f64>,
    node_type_id: PrimField<String>,
}
impl VmwareengineClusterNodeTypeConfigsEl {
    #[doc = "Set the field `custom_core_count`.\nCustomized number of cores available to each node of the type.\nThis number must always be one of 'nodeType.availableCustomCoreCounts'.\nIf zero is provided max value from 'nodeType.availableCustomCoreCounts' will be used.\nOnce the customer is created then corecount cannot be changed."]
    pub fn set_custom_core_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.custom_core_count = Some(v.into());
        self
    }
}
impl ToListMappable for VmwareengineClusterNodeTypeConfigsEl {
    type O = BlockAssignable<VmwareengineClusterNodeTypeConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineClusterNodeTypeConfigsEl {
    #[doc = "The number of nodes of this type in the cluster."]
    pub node_count: PrimField<f64>,
    #[doc = ""]
    pub node_type_id: PrimField<String>,
}
impl BuildVmwareengineClusterNodeTypeConfigsEl {
    pub fn build(self) -> VmwareengineClusterNodeTypeConfigsEl {
        VmwareengineClusterNodeTypeConfigsEl {
            custom_core_count: core::default::Default::default(),
            node_count: self.node_count,
            node_type_id: self.node_type_id,
        }
    }
}
pub struct VmwareengineClusterNodeTypeConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineClusterNodeTypeConfigsElRef {
    fn new(shared: StackShared, base: String) -> VmwareengineClusterNodeTypeConfigsElRef {
        VmwareengineClusterNodeTypeConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineClusterNodeTypeConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_core_count` after provisioning.\nCustomized number of cores available to each node of the type.\nThis number must always be one of 'nodeType.availableCustomCoreCounts'.\nIf zero is provided max value from 'nodeType.availableCustomCoreCounts' will be used.\nOnce the customer is created then corecount cannot be changed."]
    pub fn custom_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_core_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes of this type in the cluster."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }
    #[doc = "Get a reference to the value of field `node_type_id` after provisioning.\n"]
    pub fn node_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type_id", self.base))
    }
}
#[derive(Serialize)]
pub struct VmwareengineClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VmwareengineClusterTimeoutsEl {
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
impl ToListMappable for VmwareengineClusterTimeoutsEl {
    type O = BlockAssignable<VmwareengineClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineClusterTimeoutsEl {}
impl BuildVmwareengineClusterTimeoutsEl {
    pub fn build(self) -> VmwareengineClusterTimeoutsEl {
        VmwareengineClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VmwareengineClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VmwareengineClusterTimeoutsElRef {
        VmwareengineClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineClusterTimeoutsElRef {
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
struct VmwareengineClusterDynamic {
    autoscaling_settings: Option<DynamicBlock<VmwareengineClusterAutoscalingSettingsEl>>,
    datastore_mount_config: Option<DynamicBlock<VmwareengineClusterDatastoreMountConfigEl>>,
    node_type_configs: Option<DynamicBlock<VmwareengineClusterNodeTypeConfigsEl>>,
}
