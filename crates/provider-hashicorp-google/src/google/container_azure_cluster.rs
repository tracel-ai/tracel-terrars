use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ContainerAzureClusterData {
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
    azure_region: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    resource_group_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization: Option<Vec<ContainerAzureClusterAuthorizationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    azure_services_authentication: Option<Vec<ContainerAzureClusterAzureServicesAuthenticationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane: Option<Vec<ContainerAzureClusterControlPlaneEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fleet: Option<Vec<ContainerAzureClusterFleetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    networking: Option<Vec<ContainerAzureClusterNetworkingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ContainerAzureClusterTimeoutsEl>,
    dynamic: ContainerAzureClusterDynamic,
}
struct ContainerAzureCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ContainerAzureClusterData>,
}
#[derive(Clone)]
pub struct ContainerAzureCluster(Rc<ContainerAzureCluster_>);
impl ContainerAzureCluster {
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
    #[doc = "Set the field `annotations`.\nOptional. Annotations on the cluster. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `client`.\nName of the AzureClient. The `AzureClient` resource must reside on the same GCP project and region as the `AzureCluster`. `AzureClient` names are formatted as `projects/<project-number>/locations/<region>/azureClients/<client-id>`. See Resource Names (https:cloud.google.com/apis/design/resource_names) for more details on Google Cloud resource names."]
    pub fn set_client(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nOptional. A human readable description of this cluster. Cannot be longer than 255 UTF-8 encoded bytes."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `authorization`.\n"]
    pub fn set_authorization(
        self,
        v: impl Into<BlockAssignable<ContainerAzureClusterAuthorizationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authorization = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authorization = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `azure_services_authentication`.\n"]
    pub fn set_azure_services_authentication(
        self,
        v: impl Into<BlockAssignable<ContainerAzureClusterAzureServicesAuthenticationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().azure_services_authentication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .azure_services_authentication = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `control_plane`.\n"]
    pub fn set_control_plane(
        self,
        v: impl Into<BlockAssignable<ContainerAzureClusterControlPlaneEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().control_plane = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.control_plane = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `fleet`.\n"]
    pub fn set_fleet(self, v: impl Into<BlockAssignable<ContainerAzureClusterFleetEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fleet = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fleet = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `networking`.\n"]
    pub fn set_networking(
        self,
        v: impl Into<BlockAssignable<ContainerAzureClusterNetworkingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().networking = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.networking = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ContainerAzureClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the cluster. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `azure_region` after provisioning.\nThe Azure region where the cluster runs. Each Google Cloud region supports a subset of nearby Azure regions. You can call to list all supported Azure regions within a given Google Cloud region."]
    pub fn azure_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.azure_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client` after provisioning.\nName of the AzureClient. The `AzureClient` resource must reside on the same GCP project and region as the `AzureCluster`. `AzureClient` names are formatted as `projects/<project-number>/locations/<region>/azureClients/<client-id>`. See Resource Names (https:cloud.google.com/apis/design/resource_names) for more details on Google Cloud resource names."]
    pub fn client(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this cluster was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. A human readable description of this cluster. Cannot be longer than 255 UTF-8 encoded bytes."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nOutput only. The endpoint of the cluster's API server."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nAllows clients to perform consistent read-modify-writes through optimistic concurrency control. May be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. If set, there are currently changes in flight to the cluster."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_group_id` after provisioning.\nThe ARM ID of the resource group where the cluster resources are deployed. For example: `/subscriptions/*/resourceGroups/*`"]
    pub fn resource_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the cluster. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. A globally unique identifier for the cluster."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which this cluster was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_config` after provisioning.\nOutput only. Workload Identity settings."]
    pub fn workload_identity_config(
        &self,
    ) -> ListRef<ContainerAzureClusterWorkloadIdentityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_identity_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> ListRef<ContainerAzureClusterAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `azure_services_authentication` after provisioning.\n"]
    pub fn azure_services_authentication(
        &self,
    ) -> ListRef<ContainerAzureClusterAzureServicesAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.azure_services_authentication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> ListRef<ContainerAzureClusterControlPlaneElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> ListRef<ContainerAzureClusterFleetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `networking` after provisioning.\n"]
    pub fn networking(&self) -> ListRef<ContainerAzureClusterNetworkingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.networking", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAzureClusterTimeoutsElRef {
        ContainerAzureClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ContainerAzureCluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ContainerAzureCluster {}
impl ToListMappable for ContainerAzureCluster {
    type O = ListRef<ContainerAzureClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ContainerAzureCluster_ {
    fn extract_resource_type(&self) -> String {
        "google_container_azure_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildContainerAzureCluster {
    pub tf_id: String,
    #[doc = "The Azure region where the cluster runs. Each Google Cloud region supports a subset of nearby Azure regions. You can call to list all supported Azure regions within a given Google Cloud region."]
    pub azure_region: PrimField<String>,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "The name of this resource."]
    pub name: PrimField<String>,
    #[doc = "The ARM ID of the resource group where the cluster resources are deployed. For example: `/subscriptions/*/resourceGroups/*`"]
    pub resource_group_id: PrimField<String>,
}
impl BuildContainerAzureCluster {
    pub fn build(self, stack: &mut Stack) -> ContainerAzureCluster {
        let out = ContainerAzureCluster(Rc::new(ContainerAzureCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ContainerAzureClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                azure_region: self.azure_region,
                client: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                resource_group_id: self.resource_group_id,
                authorization: core::default::Default::default(),
                azure_services_authentication: core::default::Default::default(),
                control_plane: core::default::Default::default(),
                fleet: core::default::Default::default(),
                networking: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ContainerAzureClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ContainerAzureClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nOptional. Annotations on the cluster. This field has the same restrictions as Kubernetes annotations. The total size of all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `azure_region` after provisioning.\nThe Azure region where the cluster runs. Each Google Cloud region supports a subset of nearby Azure regions. You can call to list all supported Azure regions within a given Google Cloud region."]
    pub fn azure_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.azure_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client` after provisioning.\nName of the AzureClient. The `AzureClient` resource must reside on the same GCP project and region as the `AzureCluster`. `AzureClient` names are formatted as `projects/<project-number>/locations/<region>/azureClients/<client-id>`. See Resource Names (https:cloud.google.com/apis/design/resource_names) for more details on Google Cloud resource names."]
    pub fn client(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this cluster was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. A human readable description of this cluster. Cannot be longer than 255 UTF-8 encoded bytes."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nOutput only. The endpoint of the cluster's API server."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nAllows clients to perform consistent read-modify-writes through optimistic concurrency control. May be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. If set, there are currently changes in flight to the cluster."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_group_id` after provisioning.\nThe ARM ID of the resource group where the cluster resources are deployed. For example: `/subscriptions/*/resourceGroups/*`"]
    pub fn resource_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the cluster. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. A globally unique identifier for the cluster."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which this cluster was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_config` after provisioning.\nOutput only. Workload Identity settings."]
    pub fn workload_identity_config(
        &self,
    ) -> ListRef<ContainerAzureClusterWorkloadIdentityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_identity_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorization` after provisioning.\n"]
    pub fn authorization(&self) -> ListRef<ContainerAzureClusterAuthorizationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `azure_services_authentication` after provisioning.\n"]
    pub fn azure_services_authentication(
        &self,
    ) -> ListRef<ContainerAzureClusterAzureServicesAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.azure_services_authentication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_plane` after provisioning.\n"]
    pub fn control_plane(&self) -> ListRef<ContainerAzureClusterControlPlaneElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_plane", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fleet` after provisioning.\n"]
    pub fn fleet(&self) -> ListRef<ContainerAzureClusterFleetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fleet", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `networking` after provisioning.\n"]
    pub fn networking(&self) -> ListRef<ContainerAzureClusterNetworkingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.networking", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAzureClusterTimeoutsElRef {
        ContainerAzureClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAzureClusterWorkloadIdentityConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workload_pool: Option<PrimField<String>>,
}
impl ContainerAzureClusterWorkloadIdentityConfigEl {
    #[doc = "Set the field `identity_provider`.\n"]
    pub fn set_identity_provider(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_provider = Some(v.into());
        self
    }
    #[doc = "Set the field `issuer_uri`.\n"]
    pub fn set_issuer_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `workload_pool`.\n"]
    pub fn set_workload_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.workload_pool = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAzureClusterWorkloadIdentityConfigEl {
    type O = BlockAssignable<ContainerAzureClusterWorkloadIdentityConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureClusterWorkloadIdentityConfigEl {}
impl BuildContainerAzureClusterWorkloadIdentityConfigEl {
    pub fn build(self) -> ContainerAzureClusterWorkloadIdentityConfigEl {
        ContainerAzureClusterWorkloadIdentityConfigEl {
            identity_provider: core::default::Default::default(),
            issuer_uri: core::default::Default::default(),
            workload_pool: core::default::Default::default(),
        }
    }
}
pub struct ContainerAzureClusterWorkloadIdentityConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureClusterWorkloadIdentityConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerAzureClusterWorkloadIdentityConfigElRef {
        ContainerAzureClusterWorkloadIdentityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureClusterWorkloadIdentityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identity_provider` after provisioning.\n"]
    pub fn identity_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_provider", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `issuer_uri` after provisioning.\n"]
    pub fn issuer_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `workload_pool` after provisioning.\n"]
    pub fn workload_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_pool", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAzureClusterAuthorizationElAdminGroupsEl {
    group: PrimField<String>,
}
impl ContainerAzureClusterAuthorizationElAdminGroupsEl {}
impl ToListMappable for ContainerAzureClusterAuthorizationElAdminGroupsEl {
    type O = BlockAssignable<ContainerAzureClusterAuthorizationElAdminGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureClusterAuthorizationElAdminGroupsEl {
    #[doc = "The name of the group, e.g. `my-group@domain.com`."]
    pub group: PrimField<String>,
}
impl BuildContainerAzureClusterAuthorizationElAdminGroupsEl {
    pub fn build(self) -> ContainerAzureClusterAuthorizationElAdminGroupsEl {
        ContainerAzureClusterAuthorizationElAdminGroupsEl { group: self.group }
    }
}
pub struct ContainerAzureClusterAuthorizationElAdminGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureClusterAuthorizationElAdminGroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerAzureClusterAuthorizationElAdminGroupsElRef {
        ContainerAzureClusterAuthorizationElAdminGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureClusterAuthorizationElAdminGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `group` after provisioning.\nThe name of the group, e.g. `my-group@domain.com`."]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAzureClusterAuthorizationElAdminUsersEl {
    username: PrimField<String>,
}
impl ContainerAzureClusterAuthorizationElAdminUsersEl {}
impl ToListMappable for ContainerAzureClusterAuthorizationElAdminUsersEl {
    type O = BlockAssignable<ContainerAzureClusterAuthorizationElAdminUsersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureClusterAuthorizationElAdminUsersEl {
    #[doc = "The name of the user, e.g. `my-gcp-id@gmail.com`."]
    pub username: PrimField<String>,
}
impl BuildContainerAzureClusterAuthorizationElAdminUsersEl {
    pub fn build(self) -> ContainerAzureClusterAuthorizationElAdminUsersEl {
        ContainerAzureClusterAuthorizationElAdminUsersEl {
            username: self.username,
        }
    }
}
pub struct ContainerAzureClusterAuthorizationElAdminUsersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureClusterAuthorizationElAdminUsersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerAzureClusterAuthorizationElAdminUsersElRef {
        ContainerAzureClusterAuthorizationElAdminUsersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureClusterAuthorizationElAdminUsersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe name of the user, e.g. `my-gcp-id@gmail.com`."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize, Default)]
struct ContainerAzureClusterAuthorizationElDynamic {
    admin_groups: Option<DynamicBlock<ContainerAzureClusterAuthorizationElAdminGroupsEl>>,
    admin_users: Option<DynamicBlock<ContainerAzureClusterAuthorizationElAdminUsersEl>>,
}
#[derive(Serialize)]
pub struct ContainerAzureClusterAuthorizationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_groups: Option<Vec<ContainerAzureClusterAuthorizationElAdminGroupsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_users: Option<Vec<ContainerAzureClusterAuthorizationElAdminUsersEl>>,
    dynamic: ContainerAzureClusterAuthorizationElDynamic,
}
impl ContainerAzureClusterAuthorizationEl {
    #[doc = "Set the field `admin_groups`.\n"]
    pub fn set_admin_groups(
        mut self,
        v: impl Into<BlockAssignable<ContainerAzureClusterAuthorizationElAdminGroupsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.admin_groups = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.admin_groups = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `admin_users`.\n"]
    pub fn set_admin_users(
        mut self,
        v: impl Into<BlockAssignable<ContainerAzureClusterAuthorizationElAdminUsersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.admin_users = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.admin_users = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerAzureClusterAuthorizationEl {
    type O = BlockAssignable<ContainerAzureClusterAuthorizationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureClusterAuthorizationEl {}
impl BuildContainerAzureClusterAuthorizationEl {
    pub fn build(self) -> ContainerAzureClusterAuthorizationEl {
        ContainerAzureClusterAuthorizationEl {
            admin_groups: core::default::Default::default(),
            admin_users: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerAzureClusterAuthorizationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureClusterAuthorizationElRef {
    fn new(shared: StackShared, base: String) -> ContainerAzureClusterAuthorizationElRef {
        ContainerAzureClusterAuthorizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureClusterAuthorizationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_groups` after provisioning.\n"]
    pub fn admin_groups(&self) -> ListRef<ContainerAzureClusterAuthorizationElAdminGroupsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.admin_groups", self.base))
    }
    #[doc = "Get a reference to the value of field `admin_users` after provisioning.\n"]
    pub fn admin_users(&self) -> ListRef<ContainerAzureClusterAuthorizationElAdminUsersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.admin_users", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAzureClusterAzureServicesAuthenticationEl {
    application_id: PrimField<String>,
    tenant_id: PrimField<String>,
}
impl ContainerAzureClusterAzureServicesAuthenticationEl {}
impl ToListMappable for ContainerAzureClusterAzureServicesAuthenticationEl {
    type O = BlockAssignable<ContainerAzureClusterAzureServicesAuthenticationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureClusterAzureServicesAuthenticationEl {
    #[doc = "The Azure Active Directory Application ID for Authentication configuration."]
    pub application_id: PrimField<String>,
    #[doc = "The Azure Active Directory Tenant ID for Authentication configuration."]
    pub tenant_id: PrimField<String>,
}
impl BuildContainerAzureClusterAzureServicesAuthenticationEl {
    pub fn build(self) -> ContainerAzureClusterAzureServicesAuthenticationEl {
        ContainerAzureClusterAzureServicesAuthenticationEl {
            application_id: self.application_id,
            tenant_id: self.tenant_id,
        }
    }
}
pub struct ContainerAzureClusterAzureServicesAuthenticationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureClusterAzureServicesAuthenticationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerAzureClusterAzureServicesAuthenticationElRef {
        ContainerAzureClusterAzureServicesAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureClusterAzureServicesAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `application_id` after provisioning.\nThe Azure Active Directory Application ID for Authentication configuration."]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\nThe Azure Active Directory Tenant ID for Authentication configuration."]
    pub fn tenant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAzureClusterControlPlaneElDatabaseEncryptionEl {
    key_id: PrimField<String>,
}
impl ContainerAzureClusterControlPlaneElDatabaseEncryptionEl {}
impl ToListMappable for ContainerAzureClusterControlPlaneElDatabaseEncryptionEl {
    type O = BlockAssignable<ContainerAzureClusterControlPlaneElDatabaseEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureClusterControlPlaneElDatabaseEncryptionEl {
    #[doc = "The ARM ID of the Azure Key Vault key to encrypt / decrypt data. For example: `/subscriptions/<subscription-id>/resourceGroups/<resource-group-id>/providers/Microsoft.KeyVault/vaults/<key-vault-id>/keys/<key-name>` Encryption will always take the latest version of the key and hence specific version is not supported."]
    pub key_id: PrimField<String>,
}
impl BuildContainerAzureClusterControlPlaneElDatabaseEncryptionEl {
    pub fn build(self) -> ContainerAzureClusterControlPlaneElDatabaseEncryptionEl {
        ContainerAzureClusterControlPlaneElDatabaseEncryptionEl {
            key_id: self.key_id,
        }
    }
}
pub struct ContainerAzureClusterControlPlaneElDatabaseEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureClusterControlPlaneElDatabaseEncryptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerAzureClusterControlPlaneElDatabaseEncryptionElRef {
        ContainerAzureClusterControlPlaneElDatabaseEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureClusterControlPlaneElDatabaseEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key_id` after provisioning.\nThe ARM ID of the Azure Key Vault key to encrypt / decrypt data. For example: `/subscriptions/<subscription-id>/resourceGroups/<resource-group-id>/providers/Microsoft.KeyVault/vaults/<key-vault-id>/keys/<key-name>` Encryption will always take the latest version of the key and hence specific version is not supported."]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAzureClusterControlPlaneElMainVolumeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    size_gib: Option<PrimField<f64>>,
}
impl ContainerAzureClusterControlPlaneElMainVolumeEl {
    #[doc = "Set the field `size_gib`.\nOptional. The size of the disk, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource."]
    pub fn set_size_gib(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size_gib = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAzureClusterControlPlaneElMainVolumeEl {
    type O = BlockAssignable<ContainerAzureClusterControlPlaneElMainVolumeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureClusterControlPlaneElMainVolumeEl {}
impl BuildContainerAzureClusterControlPlaneElMainVolumeEl {
    pub fn build(self) -> ContainerAzureClusterControlPlaneElMainVolumeEl {
        ContainerAzureClusterControlPlaneElMainVolumeEl {
            size_gib: core::default::Default::default(),
        }
    }
}
pub struct ContainerAzureClusterControlPlaneElMainVolumeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureClusterControlPlaneElMainVolumeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerAzureClusterControlPlaneElMainVolumeElRef {
        ContainerAzureClusterControlPlaneElMainVolumeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureClusterControlPlaneElMainVolumeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `size_gib` after provisioning.\nOptional. The size of the disk, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource."]
    pub fn size_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gib", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAzureClusterControlPlaneElProxyConfigEl {
    resource_group_id: PrimField<String>,
    secret_id: PrimField<String>,
}
impl ContainerAzureClusterControlPlaneElProxyConfigEl {}
impl ToListMappable for ContainerAzureClusterControlPlaneElProxyConfigEl {
    type O = BlockAssignable<ContainerAzureClusterControlPlaneElProxyConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureClusterControlPlaneElProxyConfigEl {
    #[doc = "The ARM ID the of the resource group containing proxy keyvault. Resource group ids are formatted as `/subscriptions/<subscription-id>/resourceGroups/<resource-group-name>`"]
    pub resource_group_id: PrimField<String>,
    #[doc = "The URL the of the proxy setting secret with its version. Secret ids are formatted as `https:<key-vault-name>.vault.azure.net/secrets/<secret-name>/<secret-version>`."]
    pub secret_id: PrimField<String>,
}
impl BuildContainerAzureClusterControlPlaneElProxyConfigEl {
    pub fn build(self) -> ContainerAzureClusterControlPlaneElProxyConfigEl {
        ContainerAzureClusterControlPlaneElProxyConfigEl {
            resource_group_id: self.resource_group_id,
            secret_id: self.secret_id,
        }
    }
}
pub struct ContainerAzureClusterControlPlaneElProxyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureClusterControlPlaneElProxyConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerAzureClusterControlPlaneElProxyConfigElRef {
        ContainerAzureClusterControlPlaneElProxyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureClusterControlPlaneElProxyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_group_id` after provisioning.\nThe ARM ID the of the resource group containing proxy keyvault. Resource group ids are formatted as `/subscriptions/<subscription-id>/resourceGroups/<resource-group-name>`"]
    pub fn resource_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_group_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\nThe URL the of the proxy setting secret with its version. Secret ids are formatted as `https:<key-vault-name>.vault.azure.net/secrets/<secret-name>/<secret-version>`."]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAzureClusterControlPlaneElReplicaPlacementsEl {
    azure_availability_zone: PrimField<String>,
    subnet_id: PrimField<String>,
}
impl ContainerAzureClusterControlPlaneElReplicaPlacementsEl {}
impl ToListMappable for ContainerAzureClusterControlPlaneElReplicaPlacementsEl {
    type O = BlockAssignable<ContainerAzureClusterControlPlaneElReplicaPlacementsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureClusterControlPlaneElReplicaPlacementsEl {
    #[doc = "For a given replica, the Azure availability zone where to provision the control plane VM and the ETCD disk."]
    pub azure_availability_zone: PrimField<String>,
    #[doc = "For a given replica, the ARM ID of the subnet where the control plane VM is deployed. Make sure it's a subnet under the virtual network in the cluster configuration."]
    pub subnet_id: PrimField<String>,
}
impl BuildContainerAzureClusterControlPlaneElReplicaPlacementsEl {
    pub fn build(self) -> ContainerAzureClusterControlPlaneElReplicaPlacementsEl {
        ContainerAzureClusterControlPlaneElReplicaPlacementsEl {
            azure_availability_zone: self.azure_availability_zone,
            subnet_id: self.subnet_id,
        }
    }
}
pub struct ContainerAzureClusterControlPlaneElReplicaPlacementsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureClusterControlPlaneElReplicaPlacementsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerAzureClusterControlPlaneElReplicaPlacementsElRef {
        ContainerAzureClusterControlPlaneElReplicaPlacementsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureClusterControlPlaneElReplicaPlacementsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `azure_availability_zone` after provisioning.\nFor a given replica, the Azure availability zone where to provision the control plane VM and the ETCD disk."]
    pub fn azure_availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.azure_availability_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\nFor a given replica, the ARM ID of the subnet where the control plane VM is deployed. Make sure it's a subnet under the virtual network in the cluster configuration."]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAzureClusterControlPlaneElRootVolumeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    size_gib: Option<PrimField<f64>>,
}
impl ContainerAzureClusterControlPlaneElRootVolumeEl {
    #[doc = "Set the field `size_gib`.\nOptional. The size of the disk, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource."]
    pub fn set_size_gib(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size_gib = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAzureClusterControlPlaneElRootVolumeEl {
    type O = BlockAssignable<ContainerAzureClusterControlPlaneElRootVolumeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureClusterControlPlaneElRootVolumeEl {}
impl BuildContainerAzureClusterControlPlaneElRootVolumeEl {
    pub fn build(self) -> ContainerAzureClusterControlPlaneElRootVolumeEl {
        ContainerAzureClusterControlPlaneElRootVolumeEl {
            size_gib: core::default::Default::default(),
        }
    }
}
pub struct ContainerAzureClusterControlPlaneElRootVolumeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureClusterControlPlaneElRootVolumeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ContainerAzureClusterControlPlaneElRootVolumeElRef {
        ContainerAzureClusterControlPlaneElRootVolumeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureClusterControlPlaneElRootVolumeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `size_gib` after provisioning.\nOptional. The size of the disk, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource."]
    pub fn size_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gib", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAzureClusterControlPlaneElSshConfigEl {
    authorized_key: PrimField<String>,
}
impl ContainerAzureClusterControlPlaneElSshConfigEl {}
impl ToListMappable for ContainerAzureClusterControlPlaneElSshConfigEl {
    type O = BlockAssignable<ContainerAzureClusterControlPlaneElSshConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureClusterControlPlaneElSshConfigEl {
    #[doc = "The SSH public key data for VMs managed by Anthos. This accepts the authorized_keys file format used in OpenSSH according to the sshd(8) manual page."]
    pub authorized_key: PrimField<String>,
}
impl BuildContainerAzureClusterControlPlaneElSshConfigEl {
    pub fn build(self) -> ContainerAzureClusterControlPlaneElSshConfigEl {
        ContainerAzureClusterControlPlaneElSshConfigEl {
            authorized_key: self.authorized_key,
        }
    }
}
pub struct ContainerAzureClusterControlPlaneElSshConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureClusterControlPlaneElSshConfigElRef {
    fn new(shared: StackShared, base: String) -> ContainerAzureClusterControlPlaneElSshConfigElRef {
        ContainerAzureClusterControlPlaneElSshConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureClusterControlPlaneElSshConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authorized_key` after provisioning.\nThe SSH public key data for VMs managed by Anthos. This accepts the authorized_keys file format used in OpenSSH according to the sshd(8) manual page."]
    pub fn authorized_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorized_key", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ContainerAzureClusterControlPlaneElDynamic {
    database_encryption:
        Option<DynamicBlock<ContainerAzureClusterControlPlaneElDatabaseEncryptionEl>>,
    main_volume: Option<DynamicBlock<ContainerAzureClusterControlPlaneElMainVolumeEl>>,
    proxy_config: Option<DynamicBlock<ContainerAzureClusterControlPlaneElProxyConfigEl>>,
    replica_placements:
        Option<DynamicBlock<ContainerAzureClusterControlPlaneElReplicaPlacementsEl>>,
    root_volume: Option<DynamicBlock<ContainerAzureClusterControlPlaneElRootVolumeEl>>,
    ssh_config: Option<DynamicBlock<ContainerAzureClusterControlPlaneElSshConfigEl>>,
}
#[derive(Serialize)]
pub struct ContainerAzureClusterControlPlaneEl {
    subnet_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_encryption: Option<Vec<ContainerAzureClusterControlPlaneElDatabaseEncryptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    main_volume: Option<Vec<ContainerAzureClusterControlPlaneElMainVolumeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_config: Option<Vec<ContainerAzureClusterControlPlaneElProxyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_placements: Option<Vec<ContainerAzureClusterControlPlaneElReplicaPlacementsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_volume: Option<Vec<ContainerAzureClusterControlPlaneElRootVolumeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssh_config: Option<Vec<ContainerAzureClusterControlPlaneElSshConfigEl>>,
    dynamic: ContainerAzureClusterControlPlaneElDynamic,
}
impl ContainerAzureClusterControlPlaneEl {
    #[doc = "Set the field `tags`.\nOptional. A set of tags to apply to all underlying control plane Azure resources."]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `vm_size`.\nOptional. The Azure VM size name. Example: `Standard_DS2_v2`. For available VM sizes, see https://docs.microsoft.com/en-us/azure/virtual-machines/vm-naming-conventions. When unspecified, it defaults to `Standard_DS2_v2`."]
    pub fn set_vm_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vm_size = Some(v.into());
        self
    }
    #[doc = "Set the field `database_encryption`.\n"]
    pub fn set_database_encryption(
        mut self,
        v: impl Into<BlockAssignable<ContainerAzureClusterControlPlaneElDatabaseEncryptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.database_encryption = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.database_encryption = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `main_volume`.\n"]
    pub fn set_main_volume(
        mut self,
        v: impl Into<BlockAssignable<ContainerAzureClusterControlPlaneElMainVolumeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.main_volume = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.main_volume = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `proxy_config`.\n"]
    pub fn set_proxy_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerAzureClusterControlPlaneElProxyConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.proxy_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.proxy_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `replica_placements`.\n"]
    pub fn set_replica_placements(
        mut self,
        v: impl Into<BlockAssignable<ContainerAzureClusterControlPlaneElReplicaPlacementsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.replica_placements = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.replica_placements = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `root_volume`.\n"]
    pub fn set_root_volume(
        mut self,
        v: impl Into<BlockAssignable<ContainerAzureClusterControlPlaneElRootVolumeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.root_volume = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.root_volume = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ssh_config`.\n"]
    pub fn set_ssh_config(
        mut self,
        v: impl Into<BlockAssignable<ContainerAzureClusterControlPlaneElSshConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ssh_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ssh_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ContainerAzureClusterControlPlaneEl {
    type O = BlockAssignable<ContainerAzureClusterControlPlaneEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureClusterControlPlaneEl {
    #[doc = "The ARM ID of the subnet where the control plane VMs are deployed. Example: `/subscriptions//resourceGroups//providers/Microsoft.Network/virtualNetworks//subnets/default`."]
    pub subnet_id: PrimField<String>,
    #[doc = "The Kubernetes version to run on control plane replicas (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling GetAzureServerConfig."]
    pub version: PrimField<String>,
}
impl BuildContainerAzureClusterControlPlaneEl {
    pub fn build(self) -> ContainerAzureClusterControlPlaneEl {
        ContainerAzureClusterControlPlaneEl {
            subnet_id: self.subnet_id,
            tags: core::default::Default::default(),
            version: self.version,
            vm_size: core::default::Default::default(),
            database_encryption: core::default::Default::default(),
            main_volume: core::default::Default::default(),
            proxy_config: core::default::Default::default(),
            replica_placements: core::default::Default::default(),
            root_volume: core::default::Default::default(),
            ssh_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ContainerAzureClusterControlPlaneElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureClusterControlPlaneElRef {
    fn new(shared: StackShared, base: String) -> ContainerAzureClusterControlPlaneElRef {
        ContainerAzureClusterControlPlaneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureClusterControlPlaneElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\nThe ARM ID of the subnet where the control plane VMs are deployed. Example: `/subscriptions//resourceGroups//providers/Microsoft.Network/virtualNetworks//subnets/default`."]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nOptional. A set of tags to apply to all underlying control plane Azure resources."]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Kubernetes version to run on control plane replicas (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling GetAzureServerConfig."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
    #[doc = "Get a reference to the value of field `vm_size` after provisioning.\nOptional. The Azure VM size name. Example: `Standard_DS2_v2`. For available VM sizes, see https://docs.microsoft.com/en-us/azure/virtual-machines/vm-naming-conventions. When unspecified, it defaults to `Standard_DS2_v2`."]
    pub fn vm_size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vm_size", self.base))
    }
    #[doc = "Get a reference to the value of field `database_encryption` after provisioning.\n"]
    pub fn database_encryption(
        &self,
    ) -> ListRef<ContainerAzureClusterControlPlaneElDatabaseEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.database_encryption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `main_volume` after provisioning.\n"]
    pub fn main_volume(&self) -> ListRef<ContainerAzureClusterControlPlaneElMainVolumeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.main_volume", self.base))
    }
    #[doc = "Get a reference to the value of field `proxy_config` after provisioning.\n"]
    pub fn proxy_config(&self) -> ListRef<ContainerAzureClusterControlPlaneElProxyConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.proxy_config", self.base))
    }
    #[doc = "Get a reference to the value of field `replica_placements` after provisioning.\n"]
    pub fn replica_placements(
        &self,
    ) -> ListRef<ContainerAzureClusterControlPlaneElReplicaPlacementsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replica_placements", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `root_volume` after provisioning.\n"]
    pub fn root_volume(&self) -> ListRef<ContainerAzureClusterControlPlaneElRootVolumeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.root_volume", self.base))
    }
    #[doc = "Get a reference to the value of field `ssh_config` after provisioning.\n"]
    pub fn ssh_config(&self) -> ListRef<ContainerAzureClusterControlPlaneElSshConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssh_config", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAzureClusterFleetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
impl ContainerAzureClusterFleetEl {
    #[doc = "Set the field `project`.\nThe number of the Fleet host project where this cluster will be registered."]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
}
impl ToListMappable for ContainerAzureClusterFleetEl {
    type O = BlockAssignable<ContainerAzureClusterFleetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureClusterFleetEl {}
impl BuildContainerAzureClusterFleetEl {
    pub fn build(self) -> ContainerAzureClusterFleetEl {
        ContainerAzureClusterFleetEl {
            project: core::default::Default::default(),
        }
    }
}
pub struct ContainerAzureClusterFleetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureClusterFleetElRef {
    fn new(shared: StackShared, base: String) -> ContainerAzureClusterFleetElRef {
        ContainerAzureClusterFleetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureClusterFleetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `membership` after provisioning.\nThe name of the managed Hub Membership resource associated to this cluster. Membership names are formatted as projects/<project-number>/locations/global/membership/<cluster-id>."]
    pub fn membership(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe number of the Fleet host project where this cluster will be registered."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}
#[derive(Serialize)]
pub struct ContainerAzureClusterNetworkingEl {
    pod_address_cidr_blocks: ListField<PrimField<String>>,
    service_address_cidr_blocks: ListField<PrimField<String>>,
    virtual_network_id: PrimField<String>,
}
impl ContainerAzureClusterNetworkingEl {}
impl ToListMappable for ContainerAzureClusterNetworkingEl {
    type O = BlockAssignable<ContainerAzureClusterNetworkingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureClusterNetworkingEl {
    #[doc = "The IP address range of the pods in this cluster, in CIDR notation (e.g. `10.96.0.0/14`). All pods in the cluster get assigned a unique RFC1918 IPv4 address from these ranges. Only a single range is supported. This field cannot be changed after creation."]
    pub pod_address_cidr_blocks: ListField<PrimField<String>>,
    #[doc = "The IP address range for services in this cluster, in CIDR notation (e.g. `10.96.0.0/14`). All services in the cluster get assigned a unique RFC1918 IPv4 address from these ranges. Only a single range is supported. This field cannot be changed after creating a cluster."]
    pub service_address_cidr_blocks: ListField<PrimField<String>>,
    #[doc = "The Azure Resource Manager (ARM) ID of the VNet associated with your cluster. All components in the cluster (i.e. control plane and node pools) run on a single VNet. Example: `/subscriptions/*/resourceGroups/*/providers/Microsoft.Network/virtualNetworks/*` This field cannot be changed after creation."]
    pub virtual_network_id: PrimField<String>,
}
impl BuildContainerAzureClusterNetworkingEl {
    pub fn build(self) -> ContainerAzureClusterNetworkingEl {
        ContainerAzureClusterNetworkingEl {
            pod_address_cidr_blocks: self.pod_address_cidr_blocks,
            service_address_cidr_blocks: self.service_address_cidr_blocks,
            virtual_network_id: self.virtual_network_id,
        }
    }
}
pub struct ContainerAzureClusterNetworkingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureClusterNetworkingElRef {
    fn new(shared: StackShared, base: String) -> ContainerAzureClusterNetworkingElRef {
        ContainerAzureClusterNetworkingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureClusterNetworkingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pod_address_cidr_blocks` after provisioning.\nThe IP address range of the pods in this cluster, in CIDR notation (e.g. `10.96.0.0/14`). All pods in the cluster get assigned a unique RFC1918 IPv4 address from these ranges. Only a single range is supported. This field cannot be changed after creation."]
    pub fn pod_address_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pod_address_cidr_blocks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_address_cidr_blocks` after provisioning.\nThe IP address range for services in this cluster, in CIDR notation (e.g. `10.96.0.0/14`). All services in the cluster get assigned a unique RFC1918 IPv4 address from these ranges. Only a single range is supported. This field cannot be changed after creating a cluster."]
    pub fn service_address_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_address_cidr_blocks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `virtual_network_id` after provisioning.\nThe Azure Resource Manager (ARM) ID of the VNet associated with your cluster. All components in the cluster (i.e. control plane and node pools) run on a single VNet. Example: `/subscriptions/*/resourceGroups/*/providers/Microsoft.Network/virtualNetworks/*` This field cannot be changed after creation."]
    pub fn virtual_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.virtual_network_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ContainerAzureClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ContainerAzureClusterTimeoutsEl {
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
impl ToListMappable for ContainerAzureClusterTimeoutsEl {
    type O = BlockAssignable<ContainerAzureClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildContainerAzureClusterTimeoutsEl {}
impl BuildContainerAzureClusterTimeoutsEl {
    pub fn build(self) -> ContainerAzureClusterTimeoutsEl {
        ContainerAzureClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ContainerAzureClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ContainerAzureClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ContainerAzureClusterTimeoutsElRef {
        ContainerAzureClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ContainerAzureClusterTimeoutsElRef {
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
struct ContainerAzureClusterDynamic {
    authorization: Option<DynamicBlock<ContainerAzureClusterAuthorizationEl>>,
    azure_services_authentication:
        Option<DynamicBlock<ContainerAzureClusterAzureServicesAuthenticationEl>>,
    control_plane: Option<DynamicBlock<ContainerAzureClusterControlPlaneEl>>,
    fleet: Option<DynamicBlock<ContainerAzureClusterFleetEl>>,
    networking: Option<DynamicBlock<ContainerAzureClusterNetworkingEl>>,
}
