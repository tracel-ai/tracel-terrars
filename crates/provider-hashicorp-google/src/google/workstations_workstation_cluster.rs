use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct WorkstationsWorkstationClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    subnetwork: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workstation_authorization_url: Option<PrimField<String>>,
    workstation_cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workstation_launch_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_config: Option<Vec<WorkstationsWorkstationClusterDomainConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_cluster_config: Option<Vec<WorkstationsWorkstationClusterPrivateClusterConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<WorkstationsWorkstationClusterTimeoutsEl>,
    dynamic: WorkstationsWorkstationClusterDynamic,
}
struct WorkstationsWorkstationCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WorkstationsWorkstationClusterData>,
}
#[derive(Clone)]
pub struct WorkstationsWorkstationCluster(Rc<WorkstationsWorkstationCluster_>);
impl WorkstationsWorkstationCluster {
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
    #[doc = "Set the field `annotations`.\nClient-specified annotations. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nHuman-readable name for this resource."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nClient-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe location where the workstation cluster should reside."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nResource manager tags bound to this resource.\nFor example:\n\"123/environment\": \"production\",\n\"123/costCenter\": \"marketing\""]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `workstation_authorization_url`.\nSpecifies the redirect URL for unauthorized requests received by workstation VMs in this cluster.\nRedirects to this endpoint will send a base64 encoded 'state' query param containing the target workstation name and original request hostname. The endpoint is responsible for retrieving a token using 'GenerateAccessToken' and redirecting back to the original hostname with the token."]
    pub fn set_workstation_authorization_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().workstation_authorization_url = Some(v.into());
        self
    }
    #[doc = "Set the field `workstation_launch_url`.\nSpecifies the launch URL for workstations in this cluster. Requests sent to unstarted workstations will be redirected to this URL.\nRequests redirected to the launch endpoint will be sent with a 'workstation' query parameter containing the full workstation resource. The launch endpoint is responsible for starting the workstation, polling it until it reaches 'STATE_RUNNING', and then issuing a redirect to the workstation's host URL."]
    pub fn set_workstation_launch_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().workstation_launch_url = Some(v.into());
        self
    }
    #[doc = "Set the field `domain_config`.\n"]
    pub fn set_domain_config(
        self,
        v: impl Into<BlockAssignable<WorkstationsWorkstationClusterDomainConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().domain_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.domain_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `private_cluster_config`.\n"]
    pub fn set_private_cluster_config(
        self,
        v: impl Into<BlockAssignable<WorkstationsWorkstationClusterPrivateClusterConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_cluster_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_cluster_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<WorkstationsWorkstationClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nClient-specified annotations. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\nStatus conditions describing the current resource state."]
    pub fn conditions(&self) -> ListRef<WorkstationsWorkstationClusterConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_ip` after provisioning.\nThe private IP address of the control plane for this workstation cluster.\nWorkstation VMs need access to this IP address to work with the service, so make sure that your firewall rules allow egress from the workstation VMs to this address."]
    pub fn control_plane_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_plane_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime when this resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `degraded` after provisioning.\nWhether this resource is in degraded mode, in which case it may require user action to restore full functionality.\nDetails can be found in the conditions field."]
    pub fn degraded(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.degraded", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable name for this resource."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nChecksum computed by the server.\nMay be sent on update and delete requests to ensure that the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nClient-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location where the workstation cluster should reside."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe relative resource name of the VPC network on which the instance can be accessed.\nIt is specified in the following form: \"projects/{projectNumber}/global/networks/{network_id}\"."]
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
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nName of the Compute Engine subnetwork in which instances associated with this cluster will be created.\nMust be part of the subnetwork specified for this cluster."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nResource manager tags bound to this resource.\nFor example:\n\"123/environment\": \"production\",\n\"123/costCenter\": \"marketing\""]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `workstation_authorization_url` after provisioning.\nSpecifies the redirect URL for unauthorized requests received by workstation VMs in this cluster.\nRedirects to this endpoint will send a base64 encoded 'state' query param containing the target workstation name and original request hostname. The endpoint is responsible for retrieving a token using 'GenerateAccessToken' and redirecting back to the original hostname with the token."]
    pub fn workstation_authorization_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workstation_authorization_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workstation_cluster_id` after provisioning.\nID to use for the workstation cluster."]
    pub fn workstation_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workstation_cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workstation_launch_url` after provisioning.\nSpecifies the launch URL for workstations in this cluster. Requests sent to unstarted workstations will be redirected to this URL.\nRequests redirected to the launch endpoint will be sent with a 'workstation' query parameter containing the full workstation resource. The launch endpoint is responsible for starting the workstation, polling it until it reaches 'STATE_RUNNING', and then issuing a redirect to the workstation's host URL."]
    pub fn workstation_launch_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workstation_launch_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_config` after provisioning.\n"]
    pub fn domain_config(&self) -> ListRef<WorkstationsWorkstationClusterDomainConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.domain_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_cluster_config` after provisioning.\n"]
    pub fn private_cluster_config(
        &self,
    ) -> ListRef<WorkstationsWorkstationClusterPrivateClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_cluster_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WorkstationsWorkstationClusterTimeoutsElRef {
        WorkstationsWorkstationClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for WorkstationsWorkstationCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for WorkstationsWorkstationCluster {}
impl ToListMappable for WorkstationsWorkstationCluster {
    type O = ListRef<WorkstationsWorkstationClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for WorkstationsWorkstationCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_workstations_workstation_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildWorkstationsWorkstationCluster {
    pub tf_id: String,
    #[doc = "The relative resource name of the VPC network on which the instance can be accessed.\nIt is specified in the following form: \"projects/{projectNumber}/global/networks/{network_id}\"."]
    pub network: PrimField<String>,
    #[doc = "Name of the Compute Engine subnetwork in which instances associated with this cluster will be created.\nMust be part of the subnetwork specified for this cluster."]
    pub subnetwork: PrimField<String>,
    #[doc = "ID to use for the workstation cluster."]
    pub workstation_cluster_id: PrimField<String>,
}
impl BuildWorkstationsWorkstationCluster {
    pub fn build(self, stack: &mut Stack) -> WorkstationsWorkstationCluster {
        let out = WorkstationsWorkstationCluster(Rc::new(WorkstationsWorkstationCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WorkstationsWorkstationClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                network: self.network,
                project: core::default::Default::default(),
                subnetwork: self.subnetwork,
                tags: core::default::Default::default(),
                workstation_authorization_url: core::default::Default::default(),
                workstation_cluster_id: self.workstation_cluster_id,
                workstation_launch_url: core::default::Default::default(),
                domain_config: core::default::Default::default(),
                private_cluster_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct WorkstationsWorkstationClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl WorkstationsWorkstationClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nClient-specified annotations. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\nStatus conditions describing the current resource state."]
    pub fn conditions(&self) -> ListRef<WorkstationsWorkstationClusterConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane_ip` after provisioning.\nThe private IP address of the control plane for this workstation cluster.\nWorkstation VMs need access to this IP address to work with the service, so make sure that your firewall rules allow egress from the workstation VMs to this address."]
    pub fn control_plane_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_plane_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime when this resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `degraded` after provisioning.\nWhether this resource is in degraded mode, in which case it may require user action to restore full functionality.\nDetails can be found in the conditions field."]
    pub fn degraded(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.degraded", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable name for this resource."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nChecksum computed by the server.\nMay be sent on update and delete requests to ensure that the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nClient-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location where the workstation cluster should reside."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe relative resource name of the VPC network on which the instance can be accessed.\nIt is specified in the following form: \"projects/{projectNumber}/global/networks/{network_id}\"."]
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
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nName of the Compute Engine subnetwork in which instances associated with this cluster will be created.\nMust be part of the subnetwork specified for this cluster."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nResource manager tags bound to this resource.\nFor example:\n\"123/environment\": \"production\",\n\"123/costCenter\": \"marketing\""]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `workstation_authorization_url` after provisioning.\nSpecifies the redirect URL for unauthorized requests received by workstation VMs in this cluster.\nRedirects to this endpoint will send a base64 encoded 'state' query param containing the target workstation name and original request hostname. The endpoint is responsible for retrieving a token using 'GenerateAccessToken' and redirecting back to the original hostname with the token."]
    pub fn workstation_authorization_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workstation_authorization_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workstation_cluster_id` after provisioning.\nID to use for the workstation cluster."]
    pub fn workstation_cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workstation_cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workstation_launch_url` after provisioning.\nSpecifies the launch URL for workstations in this cluster. Requests sent to unstarted workstations will be redirected to this URL.\nRequests redirected to the launch endpoint will be sent with a 'workstation' query parameter containing the full workstation resource. The launch endpoint is responsible for starting the workstation, polling it until it reaches 'STATE_RUNNING', and then issuing a redirect to the workstation's host URL."]
    pub fn workstation_launch_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workstation_launch_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_config` after provisioning.\n"]
    pub fn domain_config(&self) -> ListRef<WorkstationsWorkstationClusterDomainConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.domain_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_cluster_config` after provisioning.\n"]
    pub fn private_cluster_config(
        &self,
    ) -> ListRef<WorkstationsWorkstationClusterPrivateClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_cluster_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WorkstationsWorkstationClusterTimeoutsElRef {
        WorkstationsWorkstationClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationClusterConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<ListField<RecField<PrimField<String>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl WorkstationsWorkstationClusterConditionsEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl Into<ListField<RecField<PrimField<String>>>>) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for WorkstationsWorkstationClusterConditionsEl {
    type O = BlockAssignable<WorkstationsWorkstationClusterConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationClusterConditionsEl {}
impl BuildWorkstationsWorkstationClusterConditionsEl {
    pub fn build(self) -> WorkstationsWorkstationClusterConditionsEl {
        WorkstationsWorkstationClusterConditionsEl {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct WorkstationsWorkstationClusterConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationClusterConditionsElRef {
    fn new(shared: StackShared, base: String) -> WorkstationsWorkstationClusterConditionsElRef {
        WorkstationsWorkstationClusterConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationClusterConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<RecRef<PrimExpr<String>>> {
        ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationClusterDomainConfigEl {
    domain: PrimField<String>,
}
impl WorkstationsWorkstationClusterDomainConfigEl {}
impl ToListMappable for WorkstationsWorkstationClusterDomainConfigEl {
    type O = BlockAssignable<WorkstationsWorkstationClusterDomainConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationClusterDomainConfigEl {
    #[doc = "Domain used by Workstations for HTTP ingress."]
    pub domain: PrimField<String>,
}
impl BuildWorkstationsWorkstationClusterDomainConfigEl {
    pub fn build(self) -> WorkstationsWorkstationClusterDomainConfigEl {
        WorkstationsWorkstationClusterDomainConfigEl {
            domain: self.domain,
        }
    }
}
pub struct WorkstationsWorkstationClusterDomainConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationClusterDomainConfigElRef {
    fn new(shared: StackShared, base: String) -> WorkstationsWorkstationClusterDomainConfigElRef {
        WorkstationsWorkstationClusterDomainConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationClusterDomainConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nDomain used by Workstations for HTTP ingress."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationClusterPrivateClusterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_projects: Option<ListField<PrimField<String>>>,
    enable_private_endpoint: PrimField<bool>,
}
impl WorkstationsWorkstationClusterPrivateClusterConfigEl {
    #[doc = "Set the field `allowed_projects`.\nAdditional project IDs that are allowed to attach to the workstation cluster's service attachment.\nBy default, the workstation cluster's project and the VPC host project (if different) are allowed."]
    pub fn set_allowed_projects(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_projects = Some(v.into());
        self
    }
}
impl ToListMappable for WorkstationsWorkstationClusterPrivateClusterConfigEl {
    type O = BlockAssignable<WorkstationsWorkstationClusterPrivateClusterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationClusterPrivateClusterConfigEl {
    #[doc = "Whether Workstations endpoint is private."]
    pub enable_private_endpoint: PrimField<bool>,
}
impl BuildWorkstationsWorkstationClusterPrivateClusterConfigEl {
    pub fn build(self) -> WorkstationsWorkstationClusterPrivateClusterConfigEl {
        WorkstationsWorkstationClusterPrivateClusterConfigEl {
            allowed_projects: core::default::Default::default(),
            enable_private_endpoint: self.enable_private_endpoint,
        }
    }
}
pub struct WorkstationsWorkstationClusterPrivateClusterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationClusterPrivateClusterConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkstationsWorkstationClusterPrivateClusterConfigElRef {
        WorkstationsWorkstationClusterPrivateClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationClusterPrivateClusterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_projects` after provisioning.\nAdditional project IDs that are allowed to attach to the workstation cluster's service attachment.\nBy default, the workstation cluster's project and the VPC host project (if different) are allowed."]
    pub fn allowed_projects(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_projects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_hostname` after provisioning.\nHostname for the workstation cluster.\nThis field will be populated only when private endpoint is enabled.\nTo access workstations in the cluster, create a new DNS zone mapping this domain name to an internal IP address and a forwarding rule mapping that address to the service attachment."]
    pub fn cluster_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_hostname", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_private_endpoint` after provisioning.\nWhether Workstations endpoint is private."]
    pub fn enable_private_endpoint(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_attachment_uri` after provisioning.\nService attachment URI for the workstation cluster.\nThe service attachment is created when private endpoint is enabled.\nTo access workstations in the cluster, configure access to the managed service using (Private Service Connect)[https://cloud.google.com/vpc/docs/configure-private-service-connect-services]."]
    pub fn service_attachment_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_attachment_uri", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct WorkstationsWorkstationClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl WorkstationsWorkstationClusterTimeoutsEl {
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
impl ToListMappable for WorkstationsWorkstationClusterTimeoutsEl {
    type O = BlockAssignable<WorkstationsWorkstationClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkstationsWorkstationClusterTimeoutsEl {}
impl BuildWorkstationsWorkstationClusterTimeoutsEl {
    pub fn build(self) -> WorkstationsWorkstationClusterTimeoutsEl {
        WorkstationsWorkstationClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct WorkstationsWorkstationClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkstationsWorkstationClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> WorkstationsWorkstationClusterTimeoutsElRef {
        WorkstationsWorkstationClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkstationsWorkstationClusterTimeoutsElRef {
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
struct WorkstationsWorkstationClusterDynamic {
    domain_config: Option<DynamicBlock<WorkstationsWorkstationClusterDomainConfigEl>>,
    private_cluster_config:
        Option<DynamicBlock<WorkstationsWorkstationClusterPrivateClusterConfigEl>>,
}
