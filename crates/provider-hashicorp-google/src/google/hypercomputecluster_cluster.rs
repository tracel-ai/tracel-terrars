use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct HypercomputeclusterClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_resources: Option<Vec<HypercomputeclusterClusterComputeResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_resources: Option<Vec<HypercomputeclusterClusterNetworkResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orchestrator: Option<Vec<HypercomputeclusterClusterOrchestratorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_resources: Option<Vec<HypercomputeclusterClusterStorageResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<HypercomputeclusterClusterTimeoutsEl>,
    dynamic: HypercomputeclusterClusterDynamic,
}
struct HypercomputeclusterCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<HypercomputeclusterClusterData>,
}
#[derive(Clone)]
pub struct HypercomputeclusterCluster(Rc<HypercomputeclusterCluster_>);
impl HypercomputeclusterCluster {
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
    #[doc = "Set the field `description`.\nUser-provided description of the cluster."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n[Labels](https://cloud.google.com/compute/docs/labeling-resources) applied\nto the cluster. Labels can be used to organize clusters and to filter them\nin queries.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `compute_resources`.\n"]
    pub fn set_compute_resources(
        self,
        v: impl Into<BlockAssignable<HypercomputeclusterClusterComputeResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.compute_resources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_resources`.\n"]
    pub fn set_network_resources(
        self,
        v: impl Into<BlockAssignable<HypercomputeclusterClusterNetworkResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_resources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `orchestrator`.\n"]
    pub fn set_orchestrator(
        self,
        v: impl Into<BlockAssignable<HypercomputeclusterClusterOrchestratorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().orchestrator = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.orchestrator = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `storage_resources`.\n"]
    pub fn set_storage_resources(
        self,
        v: impl Into<BlockAssignable<HypercomputeclusterClusterStorageResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_resources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<HypercomputeclusterClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\nID of the cluster to create. Must start with a lowercase letter,\nuse only lowercase letters and numbers, and be at most 10 characters long."]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime that the cluster was originally created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the cluster."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n[Labels](https://cloud.google.com/compute/docs/labeling-resources) applied\nto the cluster. Labels can be used to organize clusters and to filter them\nin queries.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. [Relative resource name](https://google.aip.dev/122) of the cluster, in the\nformat 'projects/{project}/locations/{location}/clusters/{cluster}'."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIndicates whether changes to the cluster are currently in flight. If this\nis 'true', then the current state might not match the cluster's intended\nstate."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime that the cluster was most recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `orchestrator` after provisioning.\n"]
    pub fn orchestrator(&self) -> ListRef<HypercomputeclusterClusterOrchestratorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.orchestrator", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HypercomputeclusterClusterTimeoutsElRef {
        HypercomputeclusterClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for HypercomputeclusterCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for HypercomputeclusterCluster {}
impl ToListMappable for HypercomputeclusterCluster {
    type O = ListRef<HypercomputeclusterClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for HypercomputeclusterCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_hypercomputecluster_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildHypercomputeclusterCluster {
    pub tf_id: String,
    #[doc = "ID of the cluster to create. Must start with a lowercase letter,\nuse only lowercase letters and numbers, and be at most 10 characters long."]
    pub cluster_id: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
}
impl BuildHypercomputeclusterCluster {
    pub fn build(self, stack: &mut Stack) -> HypercomputeclusterCluster {
        let out = HypercomputeclusterCluster(Rc::new(HypercomputeclusterCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(HypercomputeclusterClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cluster_id: self.cluster_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                compute_resources: core::default::Default::default(),
                network_resources: core::default::Default::default(),
                orchestrator: core::default::Default::default(),
                storage_resources: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct HypercomputeclusterClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl HypercomputeclusterClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\nID of the cluster to create. Must start with a lowercase letter,\nuse only lowercase letters and numbers, and be at most 10 characters long."]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime that the cluster was originally created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the cluster."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n[Labels](https://cloud.google.com/compute/docs/labeling-resources) applied\nto the cluster. Labels can be used to organize clusters and to filter them\nin queries.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. [Relative resource name](https://google.aip.dev/122) of the cluster, in the\nformat 'projects/{project}/locations/{location}/clusters/{cluster}'."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIndicates whether changes to the cluster are currently in flight. If this\nis 'true', then the current state might not match the cluster's intended\nstate."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime that the cluster was most recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `orchestrator` after provisioning.\n"]
    pub fn orchestrator(&self) -> ListRef<HypercomputeclusterClusterOrchestratorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.orchestrator", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HypercomputeclusterClusterTimeoutsElRef {
        HypercomputeclusterClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterComputeResourcesElConfigElNewFlexStartInstancesEl {
    machine_type: PrimField<String>,
    max_duration: PrimField<String>,
    zone: PrimField<String>,
}
impl HypercomputeclusterClusterComputeResourcesElConfigElNewFlexStartInstancesEl {}
impl ToListMappable
    for HypercomputeclusterClusterComputeResourcesElConfigElNewFlexStartInstancesEl
{
    type O = BlockAssignable<
        HypercomputeclusterClusterComputeResourcesElConfigElNewFlexStartInstancesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterComputeResourcesElConfigElNewFlexStartInstancesEl {
    #[doc = "Name of the Compute Engine [machine\ntype](https://cloud.google.com/compute/docs/machine-resource) to use, e.g.\n'n2-standard-2'."]
    pub machine_type: PrimField<String>,
    #[doc = "Specifies the time limit for created instances. Instances will be\nterminated at the end of this duration."]
    pub max_duration: PrimField<String>,
    #[doc = "Name of the zone in which VM instances should run, e.g., 'us-central1-a'.\nMust be in the same region as the cluster, and must match the zone of any\nother resources specified in the cluster."]
    pub zone: PrimField<String>,
}
impl BuildHypercomputeclusterClusterComputeResourcesElConfigElNewFlexStartInstancesEl {
    pub fn build(
        self,
    ) -> HypercomputeclusterClusterComputeResourcesElConfigElNewFlexStartInstancesEl {
        HypercomputeclusterClusterComputeResourcesElConfigElNewFlexStartInstancesEl {
            machine_type: self.machine_type,
            max_duration: self.max_duration,
            zone: self.zone,
        }
    }
}
pub struct HypercomputeclusterClusterComputeResourcesElConfigElNewFlexStartInstancesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterComputeResourcesElConfigElNewFlexStartInstancesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterComputeResourcesElConfigElNewFlexStartInstancesElRef {
        HypercomputeclusterClusterComputeResourcesElConfigElNewFlexStartInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterComputeResourcesElConfigElNewFlexStartInstancesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nName of the Compute Engine [machine\ntype](https://cloud.google.com/compute/docs/machine-resource) to use, e.g.\n'n2-standard-2'."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `max_duration` after provisioning.\nSpecifies the time limit for created instances. Instances will be\nterminated at the end of this duration."]
    pub fn max_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_duration", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nName of the zone in which VM instances should run, e.g., 'us-central1-a'.\nMust be in the same region as the cluster, and must match the zone of any\nother resources specified in the cluster."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterComputeResourcesElConfigElNewOnDemandInstancesEl {
    machine_type: PrimField<String>,
    zone: PrimField<String>,
}
impl HypercomputeclusterClusterComputeResourcesElConfigElNewOnDemandInstancesEl {}
impl ToListMappable for HypercomputeclusterClusterComputeResourcesElConfigElNewOnDemandInstancesEl {
    type O =
        BlockAssignable<HypercomputeclusterClusterComputeResourcesElConfigElNewOnDemandInstancesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterComputeResourcesElConfigElNewOnDemandInstancesEl {
    #[doc = "Name of the Compute Engine [machine\ntype](https://cloud.google.com/compute/docs/machine-resource) to use, e.g.\n'n2-standard-2'."]
    pub machine_type: PrimField<String>,
    #[doc = "Name of the zone in which VM instances should run, e.g., 'us-central1-a'.\nMust be in the same region as the cluster, and must match the zone of any\nother resources specified in the cluster."]
    pub zone: PrimField<String>,
}
impl BuildHypercomputeclusterClusterComputeResourcesElConfigElNewOnDemandInstancesEl {
    pub fn build(
        self,
    ) -> HypercomputeclusterClusterComputeResourcesElConfigElNewOnDemandInstancesEl {
        HypercomputeclusterClusterComputeResourcesElConfigElNewOnDemandInstancesEl {
            machine_type: self.machine_type,
            zone: self.zone,
        }
    }
}
pub struct HypercomputeclusterClusterComputeResourcesElConfigElNewOnDemandInstancesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterComputeResourcesElConfigElNewOnDemandInstancesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterComputeResourcesElConfigElNewOnDemandInstancesElRef {
        HypercomputeclusterClusterComputeResourcesElConfigElNewOnDemandInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterComputeResourcesElConfigElNewOnDemandInstancesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nName of the Compute Engine [machine\ntype](https://cloud.google.com/compute/docs/machine-resource) to use, e.g.\n'n2-standard-2'."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nName of the zone in which VM instances should run, e.g., 'us-central1-a'.\nMust be in the same region as the cluster, and must match the zone of any\nother resources specified in the cluster."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterComputeResourcesElConfigElNewReservedInstancesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation: Option<PrimField<String>>,
}
impl HypercomputeclusterClusterComputeResourcesElConfigElNewReservedInstancesEl {
    #[doc = "Set the field `reservation`.\nName of the reservation from which VM instances should be created, in the\nformat 'projects/{project}/zones/{zone}/reservations/{reservation}'."]
    pub fn set_reservation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reservation = Some(v.into());
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterComputeResourcesElConfigElNewReservedInstancesEl {
    type O =
        BlockAssignable<HypercomputeclusterClusterComputeResourcesElConfigElNewReservedInstancesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterComputeResourcesElConfigElNewReservedInstancesEl {}
impl BuildHypercomputeclusterClusterComputeResourcesElConfigElNewReservedInstancesEl {
    pub fn build(
        self,
    ) -> HypercomputeclusterClusterComputeResourcesElConfigElNewReservedInstancesEl {
        HypercomputeclusterClusterComputeResourcesElConfigElNewReservedInstancesEl {
            reservation: core::default::Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterComputeResourcesElConfigElNewReservedInstancesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterComputeResourcesElConfigElNewReservedInstancesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterComputeResourcesElConfigElNewReservedInstancesElRef {
        HypercomputeclusterClusterComputeResourcesElConfigElNewReservedInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterComputeResourcesElConfigElNewReservedInstancesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `reservation` after provisioning.\nName of the reservation from which VM instances should be created, in the\nformat 'projects/{project}/zones/{zone}/reservations/{reservation}'."]
    pub fn reservation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reservation", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterComputeResourcesElConfigElNewSpotInstancesEl {
    machine_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    termination_action: Option<PrimField<String>>,
    zone: PrimField<String>,
}
impl HypercomputeclusterClusterComputeResourcesElConfigElNewSpotInstancesEl {
    #[doc = "Set the field `termination_action`.\nSpecifies the termination action of the instance\nPossible values:\nSTOP\nDELETE"]
    pub fn set_termination_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.termination_action = Some(v.into());
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterComputeResourcesElConfigElNewSpotInstancesEl {
    type O =
        BlockAssignable<HypercomputeclusterClusterComputeResourcesElConfigElNewSpotInstancesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterComputeResourcesElConfigElNewSpotInstancesEl {
    #[doc = "Name of the Compute Engine [machine\ntype](https://cloud.google.com/compute/docs/machine-resource) to use, e.g.\n'n2-standard-2'."]
    pub machine_type: PrimField<String>,
    #[doc = "Name of the zone in which VM instances should run, e.g., 'us-central1-a'.\nMust be in the same region as the cluster, and must match the zone of any\nother resources specified in the cluster."]
    pub zone: PrimField<String>,
}
impl BuildHypercomputeclusterClusterComputeResourcesElConfigElNewSpotInstancesEl {
    pub fn build(self) -> HypercomputeclusterClusterComputeResourcesElConfigElNewSpotInstancesEl {
        HypercomputeclusterClusterComputeResourcesElConfigElNewSpotInstancesEl {
            machine_type: self.machine_type,
            termination_action: core::default::Default::default(),
            zone: self.zone,
        }
    }
}
pub struct HypercomputeclusterClusterComputeResourcesElConfigElNewSpotInstancesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterComputeResourcesElConfigElNewSpotInstancesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterComputeResourcesElConfigElNewSpotInstancesElRef {
        HypercomputeclusterClusterComputeResourcesElConfigElNewSpotInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterComputeResourcesElConfigElNewSpotInstancesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nName of the Compute Engine [machine\ntype](https://cloud.google.com/compute/docs/machine-resource) to use, e.g.\n'n2-standard-2'."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `termination_action` after provisioning.\nSpecifies the termination action of the instance\nPossible values:\nSTOP\nDELETE"]
    pub fn termination_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.termination_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nName of the zone in which VM instances should run, e.g., 'us-central1-a'.\nMust be in the same region as the cluster, and must match the zone of any\nother resources specified in the cluster."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize, Default)]
struct HypercomputeclusterClusterComputeResourcesElConfigElDynamic {
    new_flex_start_instances: Option<
        DynamicBlock<HypercomputeclusterClusterComputeResourcesElConfigElNewFlexStartInstancesEl>,
    >,
    new_on_demand_instances: Option<
        DynamicBlock<HypercomputeclusterClusterComputeResourcesElConfigElNewOnDemandInstancesEl>,
    >,
    new_reserved_instances: Option<
        DynamicBlock<HypercomputeclusterClusterComputeResourcesElConfigElNewReservedInstancesEl>,
    >,
    new_spot_instances: Option<
        DynamicBlock<HypercomputeclusterClusterComputeResourcesElConfigElNewSpotInstancesEl>,
    >,
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterComputeResourcesElConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    new_flex_start_instances:
        Option<Vec<HypercomputeclusterClusterComputeResourcesElConfigElNewFlexStartInstancesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_on_demand_instances:
        Option<Vec<HypercomputeclusterClusterComputeResourcesElConfigElNewOnDemandInstancesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_reserved_instances:
        Option<Vec<HypercomputeclusterClusterComputeResourcesElConfigElNewReservedInstancesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_spot_instances:
        Option<Vec<HypercomputeclusterClusterComputeResourcesElConfigElNewSpotInstancesEl>>,
    dynamic: HypercomputeclusterClusterComputeResourcesElConfigElDynamic,
}
impl HypercomputeclusterClusterComputeResourcesElConfigEl {
    #[doc = "Set the field `new_flex_start_instances`.\n"]
    pub fn set_new_flex_start_instances(
        mut self,
        v: impl Into<
            BlockAssignable<
                HypercomputeclusterClusterComputeResourcesElConfigElNewFlexStartInstancesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.new_flex_start_instances = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.new_flex_start_instances = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `new_on_demand_instances`.\n"]
    pub fn set_new_on_demand_instances(
        mut self,
        v: impl Into<
            BlockAssignable<
                HypercomputeclusterClusterComputeResourcesElConfigElNewOnDemandInstancesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.new_on_demand_instances = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.new_on_demand_instances = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `new_reserved_instances`.\n"]
    pub fn set_new_reserved_instances(
        mut self,
        v: impl Into<
            BlockAssignable<
                HypercomputeclusterClusterComputeResourcesElConfigElNewReservedInstancesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.new_reserved_instances = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.new_reserved_instances = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `new_spot_instances`.\n"]
    pub fn set_new_spot_instances(
        mut self,
        v: impl Into<
            BlockAssignable<HypercomputeclusterClusterComputeResourcesElConfigElNewSpotInstancesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.new_spot_instances = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.new_spot_instances = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterComputeResourcesElConfigEl {
    type O = BlockAssignable<HypercomputeclusterClusterComputeResourcesElConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterComputeResourcesElConfigEl {}
impl BuildHypercomputeclusterClusterComputeResourcesElConfigEl {
    pub fn build(self) -> HypercomputeclusterClusterComputeResourcesElConfigEl {
        HypercomputeclusterClusterComputeResourcesElConfigEl {
            new_flex_start_instances: core::default::Default::default(),
            new_on_demand_instances: core::default::Default::default(),
            new_reserved_instances: core::default::Default::default(),
            new_spot_instances: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterComputeResourcesElConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterComputeResourcesElConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterComputeResourcesElConfigElRef {
        HypercomputeclusterClusterComputeResourcesElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterComputeResourcesElConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `new_flex_start_instances` after provisioning.\n"]
    pub fn new_flex_start_instances(
        &self,
    ) -> ListRef<HypercomputeclusterClusterComputeResourcesElConfigElNewFlexStartInstancesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.new_flex_start_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `new_on_demand_instances` after provisioning.\n"]
    pub fn new_on_demand_instances(
        &self,
    ) -> ListRef<HypercomputeclusterClusterComputeResourcesElConfigElNewOnDemandInstancesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.new_on_demand_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `new_reserved_instances` after provisioning.\n"]
    pub fn new_reserved_instances(
        &self,
    ) -> ListRef<HypercomputeclusterClusterComputeResourcesElConfigElNewReservedInstancesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.new_reserved_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `new_spot_instances` after provisioning.\n"]
    pub fn new_spot_instances(
        &self,
    ) -> ListRef<HypercomputeclusterClusterComputeResourcesElConfigElNewSpotInstancesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.new_spot_instances", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct HypercomputeclusterClusterComputeResourcesElDynamic {
    config: Option<DynamicBlock<HypercomputeclusterClusterComputeResourcesElConfigEl>>,
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterComputeResourcesEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<HypercomputeclusterClusterComputeResourcesElConfigEl>>,
    dynamic: HypercomputeclusterClusterComputeResourcesElDynamic,
}
impl HypercomputeclusterClusterComputeResourcesEl {
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl Into<BlockAssignable<HypercomputeclusterClusterComputeResourcesElConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterComputeResourcesEl {
    type O = BlockAssignable<HypercomputeclusterClusterComputeResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterComputeResourcesEl {
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildHypercomputeclusterClusterComputeResourcesEl {
    pub fn build(self) -> HypercomputeclusterClusterComputeResourcesEl {
        HypercomputeclusterClusterComputeResourcesEl {
            id: self.id,
            config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterComputeResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterComputeResourcesElRef {
    fn new(shared: StackShared, base: String) -> HypercomputeclusterClusterComputeResourcesElRef {
        HypercomputeclusterClusterComputeResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterComputeResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<HypercomputeclusterClusterComputeResourcesElConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterNetworkResourcesElNetworkEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
}
impl HypercomputeclusterClusterNetworkResourcesElNetworkEl {
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterNetworkResourcesElNetworkEl {
    type O = BlockAssignable<HypercomputeclusterClusterNetworkResourcesElNetworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterNetworkResourcesElNetworkEl {}
impl BuildHypercomputeclusterClusterNetworkResourcesElNetworkEl {
    pub fn build(self) -> HypercomputeclusterClusterNetworkResourcesElNetworkEl {
        HypercomputeclusterClusterNetworkResourcesElNetworkEl {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterNetworkResourcesElNetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterNetworkResourcesElNetworkElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterNetworkResourcesElNetworkElRef {
        HypercomputeclusterClusterNetworkResourcesElNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterNetworkResourcesElNetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterNetworkResourcesElConfigElExistingNetworkEl {
    network: PrimField<String>,
    subnetwork: PrimField<String>,
}
impl HypercomputeclusterClusterNetworkResourcesElConfigElExistingNetworkEl {}
impl ToListMappable for HypercomputeclusterClusterNetworkResourcesElConfigElExistingNetworkEl {
    type O = BlockAssignable<HypercomputeclusterClusterNetworkResourcesElConfigElExistingNetworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterNetworkResourcesElConfigElExistingNetworkEl {
    #[doc = "Name of the network to import, in the format\n'projects/{project}/global/networks/{network}'."]
    pub network: PrimField<String>,
    #[doc = "Particular subnetwork to use, in the format\n'projects/{project}/regions/{region}/subnetworks/{subnetwork}'."]
    pub subnetwork: PrimField<String>,
}
impl BuildHypercomputeclusterClusterNetworkResourcesElConfigElExistingNetworkEl {
    pub fn build(self) -> HypercomputeclusterClusterNetworkResourcesElConfigElExistingNetworkEl {
        HypercomputeclusterClusterNetworkResourcesElConfigElExistingNetworkEl {
            network: self.network,
            subnetwork: self.subnetwork,
        }
    }
}
pub struct HypercomputeclusterClusterNetworkResourcesElConfigElExistingNetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterNetworkResourcesElConfigElExistingNetworkElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterNetworkResourcesElConfigElExistingNetworkElRef {
        HypercomputeclusterClusterNetworkResourcesElConfigElExistingNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterNetworkResourcesElConfigElExistingNetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nName of the network to import, in the format\n'projects/{project}/global/networks/{network}'."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nParticular subnetwork to use, in the format\n'projects/{project}/regions/{region}/subnetworks/{subnetwork}'."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterNetworkResourcesElConfigElNewNetworkEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    network: PrimField<String>,
}
impl HypercomputeclusterClusterNetworkResourcesElConfigElNewNetworkEl {
    #[doc = "Set the field `description`.\nDescription of the network. Maximum of 2048 characters."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterNetworkResourcesElConfigElNewNetworkEl {
    type O = BlockAssignable<HypercomputeclusterClusterNetworkResourcesElConfigElNewNetworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterNetworkResourcesElConfigElNewNetworkEl {
    #[doc = "Name of the network to create, in the format\n'projects/{project}/global/networks/{network}'."]
    pub network: PrimField<String>,
}
impl BuildHypercomputeclusterClusterNetworkResourcesElConfigElNewNetworkEl {
    pub fn build(self) -> HypercomputeclusterClusterNetworkResourcesElConfigElNewNetworkEl {
        HypercomputeclusterClusterNetworkResourcesElConfigElNewNetworkEl {
            description: core::default::Default::default(),
            network: self.network,
        }
    }
}
pub struct HypercomputeclusterClusterNetworkResourcesElConfigElNewNetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterNetworkResourcesElConfigElNewNetworkElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterNetworkResourcesElConfigElNewNetworkElRef {
        HypercomputeclusterClusterNetworkResourcesElConfigElNewNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterNetworkResourcesElConfigElNewNetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the network. Maximum of 2048 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nName of the network to create, in the format\n'projects/{project}/global/networks/{network}'."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}
#[derive(Serialize, Default)]
struct HypercomputeclusterClusterNetworkResourcesElConfigElDynamic {
    existing_network:
        Option<DynamicBlock<HypercomputeclusterClusterNetworkResourcesElConfigElExistingNetworkEl>>,
    new_network:
        Option<DynamicBlock<HypercomputeclusterClusterNetworkResourcesElConfigElNewNetworkEl>>,
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterNetworkResourcesElConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    existing_network:
        Option<Vec<HypercomputeclusterClusterNetworkResourcesElConfigElExistingNetworkEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_network: Option<Vec<HypercomputeclusterClusterNetworkResourcesElConfigElNewNetworkEl>>,
    dynamic: HypercomputeclusterClusterNetworkResourcesElConfigElDynamic,
}
impl HypercomputeclusterClusterNetworkResourcesElConfigEl {
    #[doc = "Set the field `existing_network`.\n"]
    pub fn set_existing_network(
        mut self,
        v: impl Into<
            BlockAssignable<HypercomputeclusterClusterNetworkResourcesElConfigElExistingNetworkEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.existing_network = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.existing_network = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `new_network`.\n"]
    pub fn set_new_network(
        mut self,
        v: impl Into<BlockAssignable<HypercomputeclusterClusterNetworkResourcesElConfigElNewNetworkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.new_network = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.new_network = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterNetworkResourcesElConfigEl {
    type O = BlockAssignable<HypercomputeclusterClusterNetworkResourcesElConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterNetworkResourcesElConfigEl {}
impl BuildHypercomputeclusterClusterNetworkResourcesElConfigEl {
    pub fn build(self) -> HypercomputeclusterClusterNetworkResourcesElConfigEl {
        HypercomputeclusterClusterNetworkResourcesElConfigEl {
            existing_network: core::default::Default::default(),
            new_network: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterNetworkResourcesElConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterNetworkResourcesElConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterNetworkResourcesElConfigElRef {
        HypercomputeclusterClusterNetworkResourcesElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterNetworkResourcesElConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `existing_network` after provisioning.\n"]
    pub fn existing_network(
        &self,
    ) -> ListRef<HypercomputeclusterClusterNetworkResourcesElConfigElExistingNetworkElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.existing_network", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `new_network` after provisioning.\n"]
    pub fn new_network(
        &self,
    ) -> ListRef<HypercomputeclusterClusterNetworkResourcesElConfigElNewNetworkElRef> {
        ListRef::new(self.shared().clone(), format!("{}.new_network", self.base))
    }
}
#[derive(Serialize, Default)]
struct HypercomputeclusterClusterNetworkResourcesElDynamic {
    config: Option<DynamicBlock<HypercomputeclusterClusterNetworkResourcesElConfigEl>>,
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterNetworkResourcesEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<HypercomputeclusterClusterNetworkResourcesElConfigEl>>,
    dynamic: HypercomputeclusterClusterNetworkResourcesElDynamic,
}
impl HypercomputeclusterClusterNetworkResourcesEl {
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl Into<BlockAssignable<HypercomputeclusterClusterNetworkResourcesElConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterNetworkResourcesEl {
    type O = BlockAssignable<HypercomputeclusterClusterNetworkResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterNetworkResourcesEl {
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildHypercomputeclusterClusterNetworkResourcesEl {
    pub fn build(self) -> HypercomputeclusterClusterNetworkResourcesEl {
        HypercomputeclusterClusterNetworkResourcesEl {
            id: self.id,
            config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterNetworkResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterNetworkResourcesElRef {
    fn new(shared: StackShared, base: String) -> HypercomputeclusterClusterNetworkResourcesElRef {
        HypercomputeclusterClusterNetworkResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterNetworkResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nA reference to a [VPC network](https://cloud.google.com/vpc/docs/vpc) in\nGoogle Compute Engine."]
    pub fn network(&self) -> ListRef<HypercomputeclusterClusterNetworkResourcesElNetworkElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<HypercomputeclusterClusterNetworkResourcesElConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElInstancesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
}
impl HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElInstancesEl {
    #[doc = "Set the field `instance`.\n"]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElInstancesEl {
    type O =
        BlockAssignable<HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElInstancesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElInstancesEl {}
impl BuildHypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElInstancesEl {
    pub fn build(self) -> HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElInstancesEl {
        HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElInstancesEl {
            instance: core::default::Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElInstancesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElInstancesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElInstancesElRef {
        HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElInstancesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElInstancesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElBootDiskEl {
    size_gb: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElBootDiskEl {}
impl ToListMappable for HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElBootDiskEl {
    type O = BlockAssignable<HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElBootDiskEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElBootDiskEl {
    #[doc = "Size of the disk in gigabytes. Must be at least 10GB."]
    pub size_gb: PrimField<String>,
    #[doc = "[Persistent disk\ntype](https://cloud.google.com/compute/docs/disks#disk-types), in the\nformat 'projects/{project}/zones/{zone}/diskTypes/{disk_type}'."]
    pub type_: PrimField<String>,
}
impl BuildHypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElBootDiskEl {
    pub fn build(self) -> HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElBootDiskEl {
        HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElBootDiskEl {
            size_gb: self.size_gb,
            type_: self.type_,
        }
    }
}
pub struct HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElBootDiskElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElBootDiskElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElBootDiskElRef {
        HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElBootDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElBootDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\nSize of the disk in gigabytes. Must be at least 10GB."]
    pub fn size_gb(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n[Persistent disk\ntype](https://cloud.google.com/compute/docs/disks#disk-types), in the\nformat 'projects/{project}/zones/{zone}/diskTypes/{disk_type}'."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElStorageConfigsEl {
    id: PrimField<String>,
    local_mount: PrimField<String>,
}
impl HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElStorageConfigsEl {}
impl ToListMappable
    for HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElStorageConfigsEl
{
    type O = BlockAssignable<
        HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElStorageConfigsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElStorageConfigsEl {
    #[doc = "ID of the storage resource to mount, which must match a key in the\ncluster's [storage_resources](Cluster.storage_resources)."]
    pub id: PrimField<String>,
    #[doc = "A directory inside the VM instance's file system where the storage resource\nshould be mounted (e.g., '/mnt/share')."]
    pub local_mount: PrimField<String>,
}
impl BuildHypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElStorageConfigsEl {
    pub fn build(
        self,
    ) -> HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElStorageConfigsEl {
        HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElStorageConfigsEl {
            id: self.id,
            local_mount: self.local_mount,
        }
    }
}
pub struct HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElStorageConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElStorageConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElStorageConfigsElRef {
        HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElStorageConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElStorageConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the storage resource to mount, which must match a key in the\ncluster's [storage_resources](Cluster.storage_resources)."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `local_mount` after provisioning.\nA directory inside the VM instance's file system where the storage resource\nshould be mounted (e.g., '/mnt/share')."]
    pub fn local_mount(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_mount", self.base))
    }
}
#[derive(Serialize, Default)]
struct HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElDynamic {
    boot_disk:
        Option<DynamicBlock<HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElBootDiskEl>>,
    storage_configs: Option<
        DynamicBlock<HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElStorageConfigsEl>,
    >,
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesEl {
    count: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_os_login: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_public_ips: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    machine_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    startup_script: Option<PrimField<String>>,
    zone: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk: Option<Vec<HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElBootDiskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_configs:
        Option<Vec<HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElStorageConfigsEl>>,
    dynamic: HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElDynamic,
}
impl HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesEl {
    #[doc = "Set the field `enable_os_login`.\nWhether [OS Login](https://cloud.google.com/compute/docs/oslogin) should be\nenabled on login node instances."]
    pub fn set_enable_os_login(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_os_login = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_public_ips`.\nWhether login node instances should be assigned [external IP\naddresses](https://cloud.google.com/compute/docs/ip-addresses#externaladdresses)."]
    pub fn set_enable_public_ips(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_public_ips = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n[Labels](https://cloud.google.com/compute/docs/labeling-resources) that\nshould be applied to each login node instance."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `startup_script`.\n[Startup\nscript](https://cloud.google.com/compute/docs/instances/startup-scripts/linux)\nto be run on each login node instance. Max 256KB.\nThe script must complete within the system-defined default timeout of 5\nminutes. For tasks that require more time, consider running them in the\nbackground using methods such as '&' or 'nohup'."]
    pub fn set_startup_script(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.startup_script = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk`.\n"]
    pub fn set_boot_disk(
        mut self,
        v: impl Into<
            BlockAssignable<HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElBootDiskEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.boot_disk = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.boot_disk = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `storage_configs`.\n"]
    pub fn set_storage_configs(
        mut self,
        v: impl Into<
            BlockAssignable<
                HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElStorageConfigsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.storage_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesEl {
    type O = BlockAssignable<HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterOrchestratorElSlurmElLoginNodesEl {
    #[doc = "Number of login node instances to create."]
    pub count: PrimField<String>,
    #[doc = "Name of the Compute Engine [machine\ntype](https://cloud.google.com/compute/docs/machine-resource) to use for\nlogin nodes, e.g. 'n2-standard-2'."]
    pub machine_type: PrimField<String>,
    #[doc = "Name of the zone in which login nodes should run, e.g., 'us-central1-a'.\nMust be in the same region as the cluster, and must match the zone of any\nother resources specified in the cluster."]
    pub zone: PrimField<String>,
}
impl BuildHypercomputeclusterClusterOrchestratorElSlurmElLoginNodesEl {
    pub fn build(self) -> HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesEl {
        HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesEl {
            count: self.count,
            enable_os_login: core::default::Default::default(),
            enable_public_ips: core::default::Default::default(),
            labels: core::default::Default::default(),
            machine_type: self.machine_type,
            startup_script: core::default::Default::default(),
            zone: self.zone,
            boot_disk: core::default::Default::default(),
            storage_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElRef {
        HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\nNumber of login node instances to create."]
    pub fn count(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `enable_os_login` after provisioning.\nWhether [OS Login](https://cloud.google.com/compute/docs/oslogin) should be\nenabled on login node instances."]
    pub fn enable_os_login(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_os_login", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_public_ips` after provisioning.\nWhether login node instances should be assigned [external IP\naddresses](https://cloud.google.com/compute/docs/ip-addresses#externaladdresses)."]
    pub fn enable_public_ips(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_public_ips", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instances` after provisioning.\nInformation about the login node instances that were created in Compute\nEngine."]
    pub fn instances(
        &self,
    ) -> ListRef<HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElInstancesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n[Labels](https://cloud.google.com/compute/docs/labeling-resources) that\nshould be applied to each login node instance."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nName of the Compute Engine [machine\ntype](https://cloud.google.com/compute/docs/machine-resource) to use for\nlogin nodes, e.g. 'n2-standard-2'."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `startup_script` after provisioning.\n[Startup\nscript](https://cloud.google.com/compute/docs/instances/startup-scripts/linux)\nto be run on each login node instance. Max 256KB.\nThe script must complete within the system-defined default timeout of 5\nminutes. For tasks that require more time, consider running them in the\nbackground using methods such as '&' or 'nohup'."]
    pub fn startup_script(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.startup_script", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nName of the zone in which login nodes should run, e.g., 'us-central1-a'.\nMust be in the same region as the cluster, and must match the zone of any\nother resources specified in the cluster."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
    #[doc = "Get a reference to the value of field `boot_disk` after provisioning.\n"]
    pub fn boot_disk(
        &self,
    ) -> ListRef<HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElBootDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_configs` after provisioning.\n"]
    pub fn storage_configs(
        &self,
    ) -> ListRef<HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElStorageConfigsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElBootDiskEl {
    size_gb: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElBootDiskEl {}
impl ToListMappable
    for HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElBootDiskEl
{
    type O = BlockAssignable<
        HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElBootDiskEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElBootDiskEl
{
    #[doc = "Size of the disk in gigabytes. Must be at least 10GB."]
    pub size_gb: PrimField<String>,
    #[doc = "[Persistent disk\ntype](https://cloud.google.com/compute/docs/disks#disk-types), in the\nformat 'projects/{project}/zones/{zone}/diskTypes/{disk_type}'."]
    pub type_: PrimField<String>,
}
impl BuildHypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElBootDiskEl {
    pub fn build(
        self,
    ) -> HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElBootDiskEl {
        HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElBootDiskEl {
            size_gb: self.size_gb,
            type_: self.type_,
        }
    }
}
pub struct HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElBootDiskElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElBootDiskElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElBootDiskElRef
    {
        HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElBootDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElBootDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\nSize of the disk in gigabytes. Must be at least 10GB."]
    pub fn size_gb(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n[Persistent disk\ntype](https://cloud.google.com/compute/docs/disks#disk-types), in the\nformat 'projects/{project}/zones/{zone}/diskTypes/{disk_type}'."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElDynamic {
    boot_disk: Option<
        DynamicBlock<
            HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElBootDiskEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    startup_script: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot_disk: Option<
        Vec<HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElBootDiskEl>,
    >,
    dynamic: HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElDynamic,
}
impl HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceEl {
    #[doc = "Set the field `labels`.\n[Labels](https://cloud.google.com/compute/docs/labeling-resources) that\nshould be applied to each VM instance in the nodeset."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `startup_script`.\n[Startup\nscript](https://cloud.google.com/compute/docs/instances/startup-scripts/linux)\nto be run on each VM instance in the nodeset. Max 256KB."]
    pub fn set_startup_script(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.startup_script = Some(v.into());
        self
    }
    #[doc = "Set the field `boot_disk`.\n"]
    pub fn set_boot_disk(
        mut self,
        v : impl Into < BlockAssignable < HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElBootDiskEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.boot_disk = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.boot_disk = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceEl {
    type O =
        BlockAssignable<HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceEl {}
impl BuildHypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceEl {
    pub fn build(
        self,
    ) -> HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceEl {
        HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceEl {
            labels: core::default::Default::default(),
            startup_script: core::default::Default::default(),
            boot_disk: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElRef {
        HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n[Labels](https://cloud.google.com/compute/docs/labeling-resources) that\nshould be applied to each VM instance in the nodeset."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `startup_script` after provisioning.\n[Startup\nscript](https://cloud.google.com/compute/docs/instances/startup-scripts/linux)\nto be run on each VM instance in the nodeset. Max 256KB."]
    pub fn startup_script(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.startup_script", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boot_disk` after provisioning.\n"]
    pub fn boot_disk(
        &self,
    ) -> ListRef<
        HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElBootDiskElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElStorageConfigsEl {
    id: PrimField<String>,
    local_mount: PrimField<String>,
}
impl HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElStorageConfigsEl {}
impl ToListMappable for HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElStorageConfigsEl {
    type O =
        BlockAssignable<HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElStorageConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElStorageConfigsEl {
    #[doc = "ID of the storage resource to mount, which must match a key in the\ncluster's [storage_resources](Cluster.storage_resources)."]
    pub id: PrimField<String>,
    #[doc = "A directory inside the VM instance's file system where the storage resource\nshould be mounted (e.g., '/mnt/share')."]
    pub local_mount: PrimField<String>,
}
impl BuildHypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElStorageConfigsEl {
    pub fn build(
        self,
    ) -> HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElStorageConfigsEl {
        HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElStorageConfigsEl {
            id: self.id,
            local_mount: self.local_mount,
        }
    }
}
pub struct HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElStorageConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElStorageConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElStorageConfigsElRef {
        HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElStorageConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElStorageConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the storage resource to mount, which must match a key in the\ncluster's [storage_resources](Cluster.storage_resources)."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `local_mount` after provisioning.\nA directory inside the VM instance's file system where the storage resource\nshould be mounted (e.g., '/mnt/share')."]
    pub fn local_mount(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_mount", self.base))
    }
}
#[derive(Serialize, Default)]
struct HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElDynamic {
    compute_instance: Option<
        DynamicBlock<HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceEl>,
    >,
    storage_configs: Option<
        DynamicBlock<HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElStorageConfigsEl>,
    >,
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_id: Option<PrimField<String>>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_dynamic_node_count: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    static_node_count: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_instance:
        Option<Vec<HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_configs:
        Option<Vec<HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElStorageConfigsEl>>,
    dynamic: HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElDynamic,
}
impl HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsEl {
    #[doc = "Set the field `compute_id`.\nID of the compute resource on which this nodeset will run. Must match a key\nin the cluster's [compute_resources](Cluster.compute_resources)."]
    pub fn set_compute_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compute_id = Some(v.into());
        self
    }
    #[doc = "Set the field `max_dynamic_node_count`.\nControls how many additional nodes a cluster can bring online to handle\nworkloads. Set this value to enable dynamic node creation and limit the\nnumber of additional nodes the cluster can bring online. Leave empty if you\ndo not want the cluster to create nodes dynamically, and instead rely only\non static nodes."]
    pub fn set_max_dynamic_node_count(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_dynamic_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `static_node_count`.\nNumber of nodes to be statically created for this nodeset. The cluster will\nattempt to ensure that at least this many nodes exist at all times."]
    pub fn set_static_node_count(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.static_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `compute_instance`.\n"]
    pub fn set_compute_instance(
        mut self,
        v: impl Into<
            BlockAssignable<
                HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.compute_instance = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.compute_instance = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `storage_configs`.\n"]
    pub fn set_storage_configs(
        mut self,
        v: impl Into<
            BlockAssignable<
                HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElStorageConfigsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.storage_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsEl {
    type O = BlockAssignable<HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterOrchestratorElSlurmElNodeSetsEl {
    #[doc = "Identifier for the nodeset, which allows it to be referenced by partitions.\nMust conform to\n[RFC-1034](https://datatracker.ietf.org/doc/html/rfc1034) (lower-case,\nalphanumeric, and at most 63 characters)."]
    pub id: PrimField<String>,
}
impl BuildHypercomputeclusterClusterOrchestratorElSlurmElNodeSetsEl {
    pub fn build(self) -> HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsEl {
        HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsEl {
            compute_id: core::default::Default::default(),
            id: self.id,
            max_dynamic_node_count: core::default::Default::default(),
            static_node_count: core::default::Default::default(),
            compute_instance: core::default::Default::default(),
            storage_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElRef {
        HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `compute_id` after provisioning.\nID of the compute resource on which this nodeset will run. Must match a key\nin the cluster's [compute_resources](Cluster.compute_resources)."]
    pub fn compute_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compute_id", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nIdentifier for the nodeset, which allows it to be referenced by partitions.\nMust conform to\n[RFC-1034](https://datatracker.ietf.org/doc/html/rfc1034) (lower-case,\nalphanumeric, and at most 63 characters)."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `max_dynamic_node_count` after provisioning.\nControls how many additional nodes a cluster can bring online to handle\nworkloads. Set this value to enable dynamic node creation and limit the\nnumber of additional nodes the cluster can bring online. Leave empty if you\ndo not want the cluster to create nodes dynamically, and instead rely only\non static nodes."]
    pub fn max_dynamic_node_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_dynamic_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `static_node_count` after provisioning.\nNumber of nodes to be statically created for this nodeset. The cluster will\nattempt to ensure that at least this many nodes exist at all times."]
    pub fn static_node_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.static_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `compute_instance` after provisioning.\n"]
    pub fn compute_instance(
        &self,
    ) -> ListRef<HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElComputeInstanceElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_configs` after provisioning.\n"]
    pub fn storage_configs(
        &self,
    ) -> ListRef<HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElStorageConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterOrchestratorElSlurmElPartitionsEl {
    id: PrimField<String>,
    node_set_ids: ListField<PrimField<String>>,
}
impl HypercomputeclusterClusterOrchestratorElSlurmElPartitionsEl {}
impl ToListMappable for HypercomputeclusterClusterOrchestratorElSlurmElPartitionsEl {
    type O = BlockAssignable<HypercomputeclusterClusterOrchestratorElSlurmElPartitionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterOrchestratorElSlurmElPartitionsEl {
    #[doc = "ID of the partition, which is how users will identify it. Must conform to\n[RFC-1034](https://datatracker.ietf.org/doc/html/rfc1034) (lower-case,\nalphanumeric, and at most 63 characters)."]
    pub id: PrimField<String>,
    #[doc = "IDs of the nodesets that make up this partition. Values must match\nSlurmNodeSet.id."]
    pub node_set_ids: ListField<PrimField<String>>,
}
impl BuildHypercomputeclusterClusterOrchestratorElSlurmElPartitionsEl {
    pub fn build(self) -> HypercomputeclusterClusterOrchestratorElSlurmElPartitionsEl {
        HypercomputeclusterClusterOrchestratorElSlurmElPartitionsEl {
            id: self.id,
            node_set_ids: self.node_set_ids,
        }
    }
}
pub struct HypercomputeclusterClusterOrchestratorElSlurmElPartitionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterOrchestratorElSlurmElPartitionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterOrchestratorElSlurmElPartitionsElRef {
        HypercomputeclusterClusterOrchestratorElSlurmElPartitionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterOrchestratorElSlurmElPartitionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the partition, which is how users will identify it. Must conform to\n[RFC-1034](https://datatracker.ietf.org/doc/html/rfc1034) (lower-case,\nalphanumeric, and at most 63 characters)."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `node_set_ids` after provisioning.\nIDs of the nodesets that make up this partition. Values must match\nSlurmNodeSet.id."]
    pub fn node_set_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.node_set_ids", self.base))
    }
}
#[derive(Serialize, Default)]
struct HypercomputeclusterClusterOrchestratorElSlurmElDynamic {
    login_nodes: Option<DynamicBlock<HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesEl>>,
    node_sets: Option<DynamicBlock<HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsEl>>,
    partitions: Option<DynamicBlock<HypercomputeclusterClusterOrchestratorElSlurmElPartitionsEl>>,
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterOrchestratorElSlurmEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_partition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    epilog_bash_scripts: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prolog_bash_scripts: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_nodes: Option<Vec<HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_sets: Option<Vec<HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partitions: Option<Vec<HypercomputeclusterClusterOrchestratorElSlurmElPartitionsEl>>,
    dynamic: HypercomputeclusterClusterOrchestratorElSlurmElDynamic,
}
impl HypercomputeclusterClusterOrchestratorElSlurmEl {
    #[doc = "Set the field `default_partition`.\nDefault partition to use for submitted jobs that do not explicitly specify\na partition. Required if and only if there is more than one partition, in\nwhich case it must match the id of one of the partitions."]
    pub fn set_default_partition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_partition = Some(v.into());
        self
    }
    #[doc = "Set the field `epilog_bash_scripts`.\nSlurm [epilog scripts](https://slurm.schedmd.com/prolog_epilog.html), which\nwill be executed by compute nodes whenever a node finishes running a job.\nValues must not be empty."]
    pub fn set_epilog_bash_scripts(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.epilog_bash_scripts = Some(v.into());
        self
    }
    #[doc = "Set the field `prolog_bash_scripts`.\nSlurm [prolog scripts](https://slurm.schedmd.com/prolog_epilog.html), which\nwill be executed by compute nodes before a node begins running a new job.\nValues must not be empty."]
    pub fn set_prolog_bash_scripts(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.prolog_bash_scripts = Some(v.into());
        self
    }
    #[doc = "Set the field `login_nodes`.\n"]
    pub fn set_login_nodes(
        mut self,
        v: impl Into<BlockAssignable<HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.login_nodes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.login_nodes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_sets`.\n"]
    pub fn set_node_sets(
        mut self,
        v: impl Into<BlockAssignable<HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_sets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_sets = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `partitions`.\n"]
    pub fn set_partitions(
        mut self,
        v: impl Into<BlockAssignable<HypercomputeclusterClusterOrchestratorElSlurmElPartitionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.partitions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.partitions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterOrchestratorElSlurmEl {
    type O = BlockAssignable<HypercomputeclusterClusterOrchestratorElSlurmEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterOrchestratorElSlurmEl {}
impl BuildHypercomputeclusterClusterOrchestratorElSlurmEl {
    pub fn build(self) -> HypercomputeclusterClusterOrchestratorElSlurmEl {
        HypercomputeclusterClusterOrchestratorElSlurmEl {
            default_partition: core::default::Default::default(),
            epilog_bash_scripts: core::default::Default::default(),
            prolog_bash_scripts: core::default::Default::default(),
            login_nodes: core::default::Default::default(),
            node_sets: core::default::Default::default(),
            partitions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterOrchestratorElSlurmElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterOrchestratorElSlurmElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterOrchestratorElSlurmElRef {
        HypercomputeclusterClusterOrchestratorElSlurmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterOrchestratorElSlurmElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_partition` after provisioning.\nDefault partition to use for submitted jobs that do not explicitly specify\na partition. Required if and only if there is more than one partition, in\nwhich case it must match the id of one of the partitions."]
    pub fn default_partition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_partition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `epilog_bash_scripts` after provisioning.\nSlurm [epilog scripts](https://slurm.schedmd.com/prolog_epilog.html), which\nwill be executed by compute nodes whenever a node finishes running a job.\nValues must not be empty."]
    pub fn epilog_bash_scripts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.epilog_bash_scripts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prolog_bash_scripts` after provisioning.\nSlurm [prolog scripts](https://slurm.schedmd.com/prolog_epilog.html), which\nwill be executed by compute nodes before a node begins running a new job.\nValues must not be empty."]
    pub fn prolog_bash_scripts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.prolog_bash_scripts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `login_nodes` after provisioning.\n"]
    pub fn login_nodes(
        &self,
    ) -> ListRef<HypercomputeclusterClusterOrchestratorElSlurmElLoginNodesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.login_nodes", self.base))
    }
    #[doc = "Get a reference to the value of field `node_sets` after provisioning.\n"]
    pub fn node_sets(
        &self,
    ) -> ListRef<HypercomputeclusterClusterOrchestratorElSlurmElNodeSetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_sets", self.base))
    }
    #[doc = "Get a reference to the value of field `partitions` after provisioning.\n"]
    pub fn partitions(
        &self,
    ) -> ListRef<HypercomputeclusterClusterOrchestratorElSlurmElPartitionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.partitions", self.base))
    }
}
#[derive(Serialize, Default)]
struct HypercomputeclusterClusterOrchestratorElDynamic {
    slurm: Option<DynamicBlock<HypercomputeclusterClusterOrchestratorElSlurmEl>>,
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterOrchestratorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    slurm: Option<Vec<HypercomputeclusterClusterOrchestratorElSlurmEl>>,
    dynamic: HypercomputeclusterClusterOrchestratorElDynamic,
}
impl HypercomputeclusterClusterOrchestratorEl {
    #[doc = "Set the field `slurm`.\n"]
    pub fn set_slurm(
        mut self,
        v: impl Into<BlockAssignable<HypercomputeclusterClusterOrchestratorElSlurmEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.slurm = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.slurm = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterOrchestratorEl {
    type O = BlockAssignable<HypercomputeclusterClusterOrchestratorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterOrchestratorEl {}
impl BuildHypercomputeclusterClusterOrchestratorEl {
    pub fn build(self) -> HypercomputeclusterClusterOrchestratorEl {
        HypercomputeclusterClusterOrchestratorEl {
            slurm: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterOrchestratorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterOrchestratorElRef {
    fn new(shared: StackShared, base: String) -> HypercomputeclusterClusterOrchestratorElRef {
        HypercomputeclusterClusterOrchestratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterOrchestratorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `slurm` after provisioning.\n"]
    pub fn slurm(&self) -> ListRef<HypercomputeclusterClusterOrchestratorElSlurmElRef> {
        ListRef::new(self.shared().clone(), format!("{}.slurm", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterStorageResourcesElBucketEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
}
impl HypercomputeclusterClusterStorageResourcesElBucketEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterStorageResourcesElBucketEl {
    type O = BlockAssignable<HypercomputeclusterClusterStorageResourcesElBucketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterStorageResourcesElBucketEl {}
impl BuildHypercomputeclusterClusterStorageResourcesElBucketEl {
    pub fn build(self) -> HypercomputeclusterClusterStorageResourcesElBucketEl {
        HypercomputeclusterClusterStorageResourcesElBucketEl {
            bucket: core::default::Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterStorageResourcesElBucketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterStorageResourcesElBucketElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterStorageResourcesElBucketElRef {
        HypercomputeclusterClusterStorageResourcesElBucketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterStorageResourcesElBucketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterStorageResourcesElFilestoreEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filestore: Option<PrimField<String>>,
}
impl HypercomputeclusterClusterStorageResourcesElFilestoreEl {
    #[doc = "Set the field `filestore`.\n"]
    pub fn set_filestore(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filestore = Some(v.into());
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterStorageResourcesElFilestoreEl {
    type O = BlockAssignable<HypercomputeclusterClusterStorageResourcesElFilestoreEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterStorageResourcesElFilestoreEl {}
impl BuildHypercomputeclusterClusterStorageResourcesElFilestoreEl {
    pub fn build(self) -> HypercomputeclusterClusterStorageResourcesElFilestoreEl {
        HypercomputeclusterClusterStorageResourcesElFilestoreEl {
            filestore: core::default::Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterStorageResourcesElFilestoreElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterStorageResourcesElFilestoreElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterStorageResourcesElFilestoreElRef {
        HypercomputeclusterClusterStorageResourcesElFilestoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterStorageResourcesElFilestoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filestore` after provisioning.\n"]
    pub fn filestore(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filestore", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterStorageResourcesElLustreEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lustre: Option<PrimField<String>>,
}
impl HypercomputeclusterClusterStorageResourcesElLustreEl {
    #[doc = "Set the field `lustre`.\n"]
    pub fn set_lustre(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lustre = Some(v.into());
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterStorageResourcesElLustreEl {
    type O = BlockAssignable<HypercomputeclusterClusterStorageResourcesElLustreEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterStorageResourcesElLustreEl {}
impl BuildHypercomputeclusterClusterStorageResourcesElLustreEl {
    pub fn build(self) -> HypercomputeclusterClusterStorageResourcesElLustreEl {
        HypercomputeclusterClusterStorageResourcesElLustreEl {
            lustre: core::default::Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterStorageResourcesElLustreElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterStorageResourcesElLustreElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterStorageResourcesElLustreElRef {
        HypercomputeclusterClusterStorageResourcesElLustreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterStorageResourcesElLustreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `lustre` after provisioning.\n"]
    pub fn lustre(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lustre", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterStorageResourcesElConfigElExistingBucketEl {
    bucket: PrimField<String>,
}
impl HypercomputeclusterClusterStorageResourcesElConfigElExistingBucketEl {}
impl ToListMappable for HypercomputeclusterClusterStorageResourcesElConfigElExistingBucketEl {
    type O = BlockAssignable<HypercomputeclusterClusterStorageResourcesElConfigElExistingBucketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterStorageResourcesElConfigElExistingBucketEl {
    #[doc = "Name of the Cloud Storage bucket to import."]
    pub bucket: PrimField<String>,
}
impl BuildHypercomputeclusterClusterStorageResourcesElConfigElExistingBucketEl {
    pub fn build(self) -> HypercomputeclusterClusterStorageResourcesElConfigElExistingBucketEl {
        HypercomputeclusterClusterStorageResourcesElConfigElExistingBucketEl {
            bucket: self.bucket,
        }
    }
}
pub struct HypercomputeclusterClusterStorageResourcesElConfigElExistingBucketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterStorageResourcesElConfigElExistingBucketElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterStorageResourcesElConfigElExistingBucketElRef {
        HypercomputeclusterClusterStorageResourcesElConfigElExistingBucketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterStorageResourcesElConfigElExistingBucketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nName of the Cloud Storage bucket to import."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterStorageResourcesElConfigElExistingFilestoreEl {
    filestore: PrimField<String>,
}
impl HypercomputeclusterClusterStorageResourcesElConfigElExistingFilestoreEl {}
impl ToListMappable for HypercomputeclusterClusterStorageResourcesElConfigElExistingFilestoreEl {
    type O =
        BlockAssignable<HypercomputeclusterClusterStorageResourcesElConfigElExistingFilestoreEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterStorageResourcesElConfigElExistingFilestoreEl {
    #[doc = "Name of the Filestore instance to import, in the format\n'projects/{project}/locations/{location}/instances/{instance}'"]
    pub filestore: PrimField<String>,
}
impl BuildHypercomputeclusterClusterStorageResourcesElConfigElExistingFilestoreEl {
    pub fn build(self) -> HypercomputeclusterClusterStorageResourcesElConfigElExistingFilestoreEl {
        HypercomputeclusterClusterStorageResourcesElConfigElExistingFilestoreEl {
            filestore: self.filestore,
        }
    }
}
pub struct HypercomputeclusterClusterStorageResourcesElConfigElExistingFilestoreElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterStorageResourcesElConfigElExistingFilestoreElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterStorageResourcesElConfigElExistingFilestoreElRef {
        HypercomputeclusterClusterStorageResourcesElConfigElExistingFilestoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterStorageResourcesElConfigElExistingFilestoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filestore` after provisioning.\nName of the Filestore instance to import, in the format\n'projects/{project}/locations/{location}/instances/{instance}'"]
    pub fn filestore(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filestore", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterStorageResourcesElConfigElExistingLustreEl {
    lustre: PrimField<String>,
}
impl HypercomputeclusterClusterStorageResourcesElConfigElExistingLustreEl {}
impl ToListMappable for HypercomputeclusterClusterStorageResourcesElConfigElExistingLustreEl {
    type O = BlockAssignable<HypercomputeclusterClusterStorageResourcesElConfigElExistingLustreEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterStorageResourcesElConfigElExistingLustreEl {
    #[doc = "Name of the Managed Lustre instance to import, in the format\n'projects/{project}/locations/{location}/instances/{instance}'"]
    pub lustre: PrimField<String>,
}
impl BuildHypercomputeclusterClusterStorageResourcesElConfigElExistingLustreEl {
    pub fn build(self) -> HypercomputeclusterClusterStorageResourcesElConfigElExistingLustreEl {
        HypercomputeclusterClusterStorageResourcesElConfigElExistingLustreEl {
            lustre: self.lustre,
        }
    }
}
pub struct HypercomputeclusterClusterStorageResourcesElConfigElExistingLustreElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterStorageResourcesElConfigElExistingLustreElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterStorageResourcesElConfigElExistingLustreElRef {
        HypercomputeclusterClusterStorageResourcesElConfigElExistingLustreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterStorageResourcesElConfigElExistingLustreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `lustre` after provisioning.\nName of the Managed Lustre instance to import, in the format\n'projects/{project}/locations/{location}/instances/{instance}'"]
    pub fn lustre(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lustre", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElAutoclassEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terminal_storage_class: Option<PrimField<String>>,
}
impl HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElAutoclassEl {
    #[doc = "Set the field `terminal_storage_class`.\nTerminal storage class of the autoclass bucket\nPossible values:\nNEARLINE\nARCHIVE"]
    pub fn set_terminal_storage_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.terminal_storage_class = Some(v.into());
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElAutoclassEl {
    type O =
        BlockAssignable<HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElAutoclassEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterStorageResourcesElConfigElNewBucketElAutoclassEl {
    #[doc = "Enables Auto-class feature."]
    pub enabled: PrimField<bool>,
}
impl BuildHypercomputeclusterClusterStorageResourcesElConfigElNewBucketElAutoclassEl {
    pub fn build(
        self,
    ) -> HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElAutoclassEl {
        HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElAutoclassEl {
            enabled: self.enabled,
            terminal_storage_class: core::default::Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElAutoclassElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElAutoclassElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElAutoclassElRef {
        HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElAutoclassElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElAutoclassElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nEnables Auto-class feature."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `terminal_storage_class` after provisioning.\nTerminal storage class of the autoclass bucket\nPossible values:\nNEARLINE\nARCHIVE"]
    pub fn terminal_storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminal_storage_class", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElHierarchicalNamespaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElHierarchicalNamespaceEl {
    #[doc = "Set the field `enabled`.\nEnables hierarchical namespace setup for the bucket."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable
    for HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElHierarchicalNamespaceEl
{
    type O = BlockAssignable<
        HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElHierarchicalNamespaceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterStorageResourcesElConfigElNewBucketElHierarchicalNamespaceEl
{}
impl BuildHypercomputeclusterClusterStorageResourcesElConfigElNewBucketElHierarchicalNamespaceEl {
    pub fn build(
        self,
    ) -> HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElHierarchicalNamespaceEl
    {
        HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElHierarchicalNamespaceEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElHierarchicalNamespaceElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElHierarchicalNamespaceElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElHierarchicalNamespaceElRef
    {
        HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElHierarchicalNamespaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElHierarchicalNamespaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nEnables hierarchical namespace setup for the bucket."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElDynamic {
    autoclass: Option<
        DynamicBlock<HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElAutoclassEl>,
    >,
    hierarchical_namespace: Option<
        DynamicBlock<
            HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElHierarchicalNamespaceEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterStorageResourcesElConfigElNewBucketEl {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoclass:
        Option<Vec<HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElAutoclassEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hierarchical_namespace: Option<
        Vec<HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElHierarchicalNamespaceEl>,
    >,
    dynamic: HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElDynamic,
}
impl HypercomputeclusterClusterStorageResourcesElConfigElNewBucketEl {
    #[doc = "Set the field `storage_class`.\nIf set, uses the provided storage class as the bucket's default storage\nclass.\nPossible values:\nSTANDARD\nNEARLINE\nCOLDLINE\nARCHIVE"]
    pub fn set_storage_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_class = Some(v.into());
        self
    }
    #[doc = "Set the field `autoclass`.\n"]
    pub fn set_autoclass(
        mut self,
        v: impl Into<
            BlockAssignable<
                HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElAutoclassEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.autoclass = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.autoclass = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hierarchical_namespace`.\n"]
    pub fn set_hierarchical_namespace(
        mut self,
        v : impl Into < BlockAssignable < HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElHierarchicalNamespaceEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hierarchical_namespace = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hierarchical_namespace = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterStorageResourcesElConfigElNewBucketEl {
    type O = BlockAssignable<HypercomputeclusterClusterStorageResourcesElConfigElNewBucketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterStorageResourcesElConfigElNewBucketEl {
    #[doc = "Name of the Cloud Storage bucket to create."]
    pub bucket: PrimField<String>,
}
impl BuildHypercomputeclusterClusterStorageResourcesElConfigElNewBucketEl {
    pub fn build(self) -> HypercomputeclusterClusterStorageResourcesElConfigElNewBucketEl {
        HypercomputeclusterClusterStorageResourcesElConfigElNewBucketEl {
            bucket: self.bucket,
            storage_class: core::default::Default::default(),
            autoclass: core::default::Default::default(),
            hierarchical_namespace: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElRef {
        HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nName of the Cloud Storage bucket to create."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\nIf set, uses the provided storage class as the bucket's default storage\nclass.\nPossible values:\nSTANDARD\nNEARLINE\nCOLDLINE\nARCHIVE"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_class", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `autoclass` after provisioning.\n"]
    pub fn autoclass(
        &self,
    ) -> ListRef<HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElAutoclassElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.autoclass", self.base))
    }
    #[doc = "Get a reference to the value of field `hierarchical_namespace` after provisioning.\n"]
    pub fn hierarchical_namespace(
        &self,
    ) -> ListRef<
        HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElHierarchicalNamespaceElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hierarchical_namespace", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElFileSharesEl {
    capacity_gb: PrimField<String>,
    file_share: PrimField<String>,
}
impl HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElFileSharesEl {}
impl ToListMappable
    for HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElFileSharesEl
{
    type O = BlockAssignable<
        HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElFileSharesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElFileSharesEl {
    #[doc = "Size of the filestore in GB. Must be between 1024 and 102400, and must meet\nscalability requirements described at\nhttps://cloud.google.com/filestore/docs/service-tiers."]
    pub capacity_gb: PrimField<String>,
    #[doc = "Filestore share location"]
    pub file_share: PrimField<String>,
}
impl BuildHypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElFileSharesEl {
    pub fn build(
        self,
    ) -> HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElFileSharesEl {
        HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElFileSharesEl {
            capacity_gb: self.capacity_gb,
            file_share: self.file_share,
        }
    }
}
pub struct HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElFileSharesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElFileSharesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElFileSharesElRef {
        HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElFileSharesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElFileSharesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `capacity_gb` after provisioning.\nSize of the filestore in GB. Must be between 1024 and 102400, and must meet\nscalability requirements described at\nhttps://cloud.google.com/filestore/docs/service-tiers."]
    pub fn capacity_gb(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `file_share` after provisioning.\nFilestore share location"]
    pub fn file_share(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_share", self.base))
    }
}
#[derive(Serialize, Default)]
struct HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElDynamic {
    file_shares: Option<
        DynamicBlock<
            HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElFileSharesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    filestore: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    tier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_shares:
        Option<Vec<HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElFileSharesEl>>,
    dynamic: HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElDynamic,
}
impl HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreEl {
    #[doc = "Set the field `description`.\nDescription of the instance. Maximum of 2048 characters."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\nAccess protocol to use for all file shares in the instance. Defaults to NFS\nV3 if not set.\nPossible values:\nNFSV3\nNFSV41 Possible values: [\"PROTOCOL_UNSPECIFIED\", \"NFSV3\", \"NFSV41\"]"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `file_shares`.\n"]
    pub fn set_file_shares(
        mut self,
        v: impl Into<
            BlockAssignable<
                HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElFileSharesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file_shares = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file_shares = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreEl {
    type O = BlockAssignable<HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreEl {
    #[doc = "Name of the Filestore instance to create, in the format\n'projects/{project}/locations/{location}/instances/{instance}'"]
    pub filestore: PrimField<String>,
    #[doc = "Service tier to use for the instance.\nPossible values:\nZONAL\nREGIONAL Possible values: [\"TIER_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub tier: PrimField<String>,
}
impl BuildHypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreEl {
    pub fn build(self) -> HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreEl {
        HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreEl {
            description: core::default::Default::default(),
            filestore: self.filestore,
            protocol: core::default::Default::default(),
            tier: self.tier,
            file_shares: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElRef {
        HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the instance. Maximum of 2048 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `filestore` after provisioning.\nName of the Filestore instance to create, in the format\n'projects/{project}/locations/{location}/instances/{instance}'"]
    pub fn filestore(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filestore", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nAccess protocol to use for all file shares in the instance. Defaults to NFS\nV3 if not set.\nPossible values:\nNFSV3\nNFSV41 Possible values: [\"PROTOCOL_UNSPECIFIED\", \"NFSV3\", \"NFSV41\"]"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `tier` after provisioning.\nService tier to use for the instance.\nPossible values:\nZONAL\nREGIONAL Possible values: [\"TIER_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tier", self.base))
    }
    #[doc = "Get a reference to the value of field `file_shares` after provisioning.\n"]
    pub fn file_shares(
        &self,
    ) -> ListRef<HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElFileSharesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.file_shares", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterStorageResourcesElConfigElNewLustreEl {
    capacity_gb: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    filesystem: PrimField<String>,
    lustre: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_unit_storage_throughput: Option<PrimField<String>>,
}
impl HypercomputeclusterClusterStorageResourcesElConfigElNewLustreEl {
    #[doc = "Set the field `description`.\nDescription of the Managed Lustre instance. Maximum of 2048 characters."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `per_unit_storage_throughput`.\nThroughput of the instance in MB/s/TiB. Valid values are 125, 250,\n500, 1000. See [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor more information."]
    pub fn set_per_unit_storage_throughput(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.per_unit_storage_throughput = Some(v.into());
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterStorageResourcesElConfigElNewLustreEl {
    type O = BlockAssignable<HypercomputeclusterClusterStorageResourcesElConfigElNewLustreEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterStorageResourcesElConfigElNewLustreEl {
    #[doc = "Storage capacity of the instance in gibibytes (GiB). Allowed values are\nbetween 18000 and 7632000."]
    pub capacity_gb: PrimField<String>,
    #[doc = "Filesystem name for this instance. This name is used by client-side tools,\nincluding when mounting the instance. Must be 8 characters or less and can\nonly contain letters and numbers."]
    pub filesystem: PrimField<String>,
    #[doc = "Name of the Managed Lustre instance to create, in the format\n'projects/{project}/locations/{location}/instances/{instance}'"]
    pub lustre: PrimField<String>,
}
impl BuildHypercomputeclusterClusterStorageResourcesElConfigElNewLustreEl {
    pub fn build(self) -> HypercomputeclusterClusterStorageResourcesElConfigElNewLustreEl {
        HypercomputeclusterClusterStorageResourcesElConfigElNewLustreEl {
            capacity_gb: self.capacity_gb,
            description: core::default::Default::default(),
            filesystem: self.filesystem,
            lustre: self.lustre,
            per_unit_storage_throughput: core::default::Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterStorageResourcesElConfigElNewLustreElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterStorageResourcesElConfigElNewLustreElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterStorageResourcesElConfigElNewLustreElRef {
        HypercomputeclusterClusterStorageResourcesElConfigElNewLustreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterStorageResourcesElConfigElNewLustreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `capacity_gb` after provisioning.\nStorage capacity of the instance in gibibytes (GiB). Allowed values are\nbetween 18000 and 7632000."]
    pub fn capacity_gb(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the Managed Lustre instance. Maximum of 2048 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `filesystem` after provisioning.\nFilesystem name for this instance. This name is used by client-side tools,\nincluding when mounting the instance. Must be 8 characters or less and can\nonly contain letters and numbers."]
    pub fn filesystem(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filesystem", self.base))
    }
    #[doc = "Get a reference to the value of field `lustre` after provisioning.\nName of the Managed Lustre instance to create, in the format\n'projects/{project}/locations/{location}/instances/{instance}'"]
    pub fn lustre(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lustre", self.base))
    }
    #[doc = "Get a reference to the value of field `per_unit_storage_throughput` after provisioning.\nThroughput of the instance in MB/s/TiB. Valid values are 125, 250,\n500, 1000. See [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor more information."]
    pub fn per_unit_storage_throughput(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.per_unit_storage_throughput", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct HypercomputeclusterClusterStorageResourcesElConfigElDynamic {
    existing_bucket:
        Option<DynamicBlock<HypercomputeclusterClusterStorageResourcesElConfigElExistingBucketEl>>,
    existing_filestore: Option<
        DynamicBlock<HypercomputeclusterClusterStorageResourcesElConfigElExistingFilestoreEl>,
    >,
    existing_lustre:
        Option<DynamicBlock<HypercomputeclusterClusterStorageResourcesElConfigElExistingLustreEl>>,
    new_bucket:
        Option<DynamicBlock<HypercomputeclusterClusterStorageResourcesElConfigElNewBucketEl>>,
    new_filestore:
        Option<DynamicBlock<HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreEl>>,
    new_lustre:
        Option<DynamicBlock<HypercomputeclusterClusterStorageResourcesElConfigElNewLustreEl>>,
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterStorageResourcesElConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    existing_bucket:
        Option<Vec<HypercomputeclusterClusterStorageResourcesElConfigElExistingBucketEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    existing_filestore:
        Option<Vec<HypercomputeclusterClusterStorageResourcesElConfigElExistingFilestoreEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    existing_lustre:
        Option<Vec<HypercomputeclusterClusterStorageResourcesElConfigElExistingLustreEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_bucket: Option<Vec<HypercomputeclusterClusterStorageResourcesElConfigElNewBucketEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_filestore: Option<Vec<HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_lustre: Option<Vec<HypercomputeclusterClusterStorageResourcesElConfigElNewLustreEl>>,
    dynamic: HypercomputeclusterClusterStorageResourcesElConfigElDynamic,
}
impl HypercomputeclusterClusterStorageResourcesElConfigEl {
    #[doc = "Set the field `existing_bucket`.\n"]
    pub fn set_existing_bucket(
        mut self,
        v: impl Into<
            BlockAssignable<HypercomputeclusterClusterStorageResourcesElConfigElExistingBucketEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.existing_bucket = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.existing_bucket = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `existing_filestore`.\n"]
    pub fn set_existing_filestore(
        mut self,
        v: impl Into<
            BlockAssignable<
                HypercomputeclusterClusterStorageResourcesElConfigElExistingFilestoreEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.existing_filestore = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.existing_filestore = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `existing_lustre`.\n"]
    pub fn set_existing_lustre(
        mut self,
        v: impl Into<
            BlockAssignable<HypercomputeclusterClusterStorageResourcesElConfigElExistingLustreEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.existing_lustre = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.existing_lustre = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `new_bucket`.\n"]
    pub fn set_new_bucket(
        mut self,
        v: impl Into<BlockAssignable<HypercomputeclusterClusterStorageResourcesElConfigElNewBucketEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.new_bucket = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.new_bucket = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `new_filestore`.\n"]
    pub fn set_new_filestore(
        mut self,
        v: impl Into<
            BlockAssignable<HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.new_filestore = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.new_filestore = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `new_lustre`.\n"]
    pub fn set_new_lustre(
        mut self,
        v: impl Into<BlockAssignable<HypercomputeclusterClusterStorageResourcesElConfigElNewLustreEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.new_lustre = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.new_lustre = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterStorageResourcesElConfigEl {
    type O = BlockAssignable<HypercomputeclusterClusterStorageResourcesElConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterStorageResourcesElConfigEl {}
impl BuildHypercomputeclusterClusterStorageResourcesElConfigEl {
    pub fn build(self) -> HypercomputeclusterClusterStorageResourcesElConfigEl {
        HypercomputeclusterClusterStorageResourcesElConfigEl {
            existing_bucket: core::default::Default::default(),
            existing_filestore: core::default::Default::default(),
            existing_lustre: core::default::Default::default(),
            new_bucket: core::default::Default::default(),
            new_filestore: core::default::Default::default(),
            new_lustre: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterStorageResourcesElConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterStorageResourcesElConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HypercomputeclusterClusterStorageResourcesElConfigElRef {
        HypercomputeclusterClusterStorageResourcesElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterStorageResourcesElConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `existing_bucket` after provisioning.\n"]
    pub fn existing_bucket(
        &self,
    ) -> ListRef<HypercomputeclusterClusterStorageResourcesElConfigElExistingBucketElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.existing_bucket", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `existing_filestore` after provisioning.\n"]
    pub fn existing_filestore(
        &self,
    ) -> ListRef<HypercomputeclusterClusterStorageResourcesElConfigElExistingFilestoreElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.existing_filestore", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `existing_lustre` after provisioning.\n"]
    pub fn existing_lustre(
        &self,
    ) -> ListRef<HypercomputeclusterClusterStorageResourcesElConfigElExistingLustreElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.existing_lustre", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `new_bucket` after provisioning.\n"]
    pub fn new_bucket(
        &self,
    ) -> ListRef<HypercomputeclusterClusterStorageResourcesElConfigElNewBucketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.new_bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `new_filestore` after provisioning.\n"]
    pub fn new_filestore(
        &self,
    ) -> ListRef<HypercomputeclusterClusterStorageResourcesElConfigElNewFilestoreElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.new_filestore", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `new_lustre` after provisioning.\n"]
    pub fn new_lustre(
        &self,
    ) -> ListRef<HypercomputeclusterClusterStorageResourcesElConfigElNewLustreElRef> {
        ListRef::new(self.shared().clone(), format!("{}.new_lustre", self.base))
    }
}
#[derive(Serialize, Default)]
struct HypercomputeclusterClusterStorageResourcesElDynamic {
    config: Option<DynamicBlock<HypercomputeclusterClusterStorageResourcesElConfigEl>>,
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterStorageResourcesEl {
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<HypercomputeclusterClusterStorageResourcesElConfigEl>>,
    dynamic: HypercomputeclusterClusterStorageResourcesElDynamic,
}
impl HypercomputeclusterClusterStorageResourcesEl {
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl Into<BlockAssignable<HypercomputeclusterClusterStorageResourcesElConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HypercomputeclusterClusterStorageResourcesEl {
    type O = BlockAssignable<HypercomputeclusterClusterStorageResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterStorageResourcesEl {
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildHypercomputeclusterClusterStorageResourcesEl {
    pub fn build(self) -> HypercomputeclusterClusterStorageResourcesEl {
        HypercomputeclusterClusterStorageResourcesEl {
            id: self.id,
            config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterStorageResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterStorageResourcesElRef {
    fn new(shared: StackShared, base: String) -> HypercomputeclusterClusterStorageResourcesElRef {
        HypercomputeclusterClusterStorageResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterStorageResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nA reference to a [Google Cloud Storage](https://cloud.google.com/storage)\nbucket."]
    pub fn bucket(&self) -> ListRef<HypercomputeclusterClusterStorageResourcesElBucketElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `filestore` after provisioning.\nA reference to a [Filestore](https://cloud.google.com/filestore) instance."]
    pub fn filestore(&self) -> ListRef<HypercomputeclusterClusterStorageResourcesElFilestoreElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filestore", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `lustre` after provisioning.\nA reference to a [Managed\nLustre](https://cloud.google.com/products/managed-lustre) instance."]
    pub fn lustre(&self) -> ListRef<HypercomputeclusterClusterStorageResourcesElLustreElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lustre", self.base))
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<HypercomputeclusterClusterStorageResourcesElConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}
#[derive(Serialize)]
pub struct HypercomputeclusterClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl HypercomputeclusterClusterTimeoutsEl {
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
impl ToListMappable for HypercomputeclusterClusterTimeoutsEl {
    type O = BlockAssignable<HypercomputeclusterClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHypercomputeclusterClusterTimeoutsEl {}
impl BuildHypercomputeclusterClusterTimeoutsEl {
    pub fn build(self) -> HypercomputeclusterClusterTimeoutsEl {
        HypercomputeclusterClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct HypercomputeclusterClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HypercomputeclusterClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> HypercomputeclusterClusterTimeoutsElRef {
        HypercomputeclusterClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HypercomputeclusterClusterTimeoutsElRef {
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
struct HypercomputeclusterClusterDynamic {
    compute_resources: Option<DynamicBlock<HypercomputeclusterClusterComputeResourcesEl>>,
    network_resources: Option<DynamicBlock<HypercomputeclusterClusterNetworkResourcesEl>>,
    orchestrator: Option<DynamicBlock<HypercomputeclusterClusterOrchestratorEl>>,
    storage_resources: Option<DynamicBlock<HypercomputeclusterClusterStorageResourcesEl>>,
}
