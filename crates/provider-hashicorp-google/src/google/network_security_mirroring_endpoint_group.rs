use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkSecurityMirroringEndpointGroupData {
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
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirroring_deployment_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirroring_deployment_groups: Option<ListField<PrimField<String>>>,
    mirroring_endpoint_group_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkSecurityMirroringEndpointGroupTimeoutsEl>,
}
struct NetworkSecurityMirroringEndpointGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkSecurityMirroringEndpointGroupData>,
}
#[derive(Clone)]
pub struct NetworkSecurityMirroringEndpointGroup(Rc<NetworkSecurityMirroringEndpointGroup_>);
impl NetworkSecurityMirroringEndpointGroup {
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
    #[doc = "Set the field `description`.\nUser-provided description of the endpoint group.\nUsed as additional context for the endpoint group."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels are key/value pairs that help to organize and filter resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `mirroring_deployment_group`.\nThe deployment group that this DIRECT endpoint group is connected to, for example:\n'projects/123456789/locations/global/mirroringDeploymentGroups/my-dg'.\nSee https://google.aip.dev/124."]
    pub fn set_mirroring_deployment_group(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mirroring_deployment_group = Some(v.into());
        self
    }
    #[doc = "Set the field `mirroring_deployment_groups`.\nA list of the deployment groups that this BROKER endpoint group is\nconnected to, for example:\n'projects/123456789/locations/global/mirroringDeploymentGroups/my-dg'.\nSee https://google.aip.dev/124."]
    pub fn set_mirroring_deployment_groups(
        self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().mirroring_deployment_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThe type of the endpoint group.\nIf left unspecified, defaults to DIRECT.\nPossible values:\nDIRECT\nBROKER"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<NetworkSecurityMirroringEndpointGroupTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `associations` after provisioning.\nList of associations to this endpoint group."]
    pub fn associations(&self) -> SetRef<NetworkSecurityMirroringEndpointGroupAssociationsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.associations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connected_deployment_groups` after provisioning.\nList of details about the connected deployment groups to this endpoint\ngroup."]
    pub fn connected_deployment_groups(
        &self,
    ) -> SetRef<NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.connected_deployment_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created.\nSee https://google.aip.dev/148#timestamps."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the endpoint group.\nUsed as additional context for the endpoint group."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels are key/value pairs that help to organize and filter resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe cloud location of the endpoint group, currently restricted to 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mirroring_deployment_group` after provisioning.\nThe deployment group that this DIRECT endpoint group is connected to, for example:\n'projects/123456789/locations/global/mirroringDeploymentGroups/my-dg'.\nSee https://google.aip.dev/124."]
    pub fn mirroring_deployment_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mirroring_deployment_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mirroring_deployment_groups` after provisioning.\nA list of the deployment groups that this BROKER endpoint group is\nconnected to, for example:\n'projects/123456789/locations/global/mirroringDeploymentGroups/my-dg'.\nSee https://google.aip.dev/124."]
    pub fn mirroring_deployment_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mirroring_deployment_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mirroring_endpoint_group_id` after provisioning.\nThe ID to use for the endpoint group, which will become the final component\nof the endpoint group's resource name."]
    pub fn mirroring_endpoint_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mirroring_endpoint_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of this endpoint group, for example:\n'projects/123456789/locations/global/mirroringEndpointGroups/my-eg'.\nSee https://google.aip.dev/122 for more details."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nThe current state of the resource does not match the user's intended state,\nand the system is working to reconcile them. This is part of the normal\noperation (e.g. adding a new association to the group).\nSee https://google.aip.dev/128."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the endpoint group.\nSee https://google.aip.dev/216.\nPossible values:\nSTATE_UNSPECIFIED\nACTIVE\nCLOSED\nCREATING\nDELETING\nOUT_OF_SYNC\nDELETE_FAILED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the endpoint group.\nIf left unspecified, defaults to DIRECT.\nPossible values:\nDIRECT\nBROKER"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was most recently updated.\nSee https://google.aip.dev/148#timestamps."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecurityMirroringEndpointGroupTimeoutsElRef {
        NetworkSecurityMirroringEndpointGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkSecurityMirroringEndpointGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkSecurityMirroringEndpointGroup {}
impl ToListMappable for NetworkSecurityMirroringEndpointGroup {
    type O = ListRef<NetworkSecurityMirroringEndpointGroupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkSecurityMirroringEndpointGroup_ {
    fn extract_resource_type(&self) -> String {
        "google_network_security_mirroring_endpoint_group".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkSecurityMirroringEndpointGroup {
    pub tf_id: String,
    #[doc = "The cloud location of the endpoint group, currently restricted to 'global'."]
    pub location: PrimField<String>,
    #[doc = "The ID to use for the endpoint group, which will become the final component\nof the endpoint group's resource name."]
    pub mirroring_endpoint_group_id: PrimField<String>,
}
impl BuildNetworkSecurityMirroringEndpointGroup {
    pub fn build(self, stack: &mut Stack) -> NetworkSecurityMirroringEndpointGroup {
        let out = NetworkSecurityMirroringEndpointGroup(Rc::new(
            NetworkSecurityMirroringEndpointGroup_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(NetworkSecurityMirroringEndpointGroupData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    description: core::default::Default::default(),
                    id: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    location: self.location,
                    mirroring_deployment_group: core::default::Default::default(),
                    mirroring_deployment_groups: core::default::Default::default(),
                    mirroring_endpoint_group_id: self.mirroring_endpoint_group_id,
                    project: core::default::Default::default(),
                    type_: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkSecurityMirroringEndpointGroupRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityMirroringEndpointGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkSecurityMirroringEndpointGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `associations` after provisioning.\nList of associations to this endpoint group."]
    pub fn associations(&self) -> SetRef<NetworkSecurityMirroringEndpointGroupAssociationsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.associations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connected_deployment_groups` after provisioning.\nList of details about the connected deployment groups to this endpoint\ngroup."]
    pub fn connected_deployment_groups(
        &self,
    ) -> SetRef<NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.connected_deployment_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created.\nSee https://google.aip.dev/148#timestamps."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the endpoint group.\nUsed as additional context for the endpoint group."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels are key/value pairs that help to organize and filter resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe cloud location of the endpoint group, currently restricted to 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mirroring_deployment_group` after provisioning.\nThe deployment group that this DIRECT endpoint group is connected to, for example:\n'projects/123456789/locations/global/mirroringDeploymentGroups/my-dg'.\nSee https://google.aip.dev/124."]
    pub fn mirroring_deployment_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mirroring_deployment_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mirroring_deployment_groups` after provisioning.\nA list of the deployment groups that this BROKER endpoint group is\nconnected to, for example:\n'projects/123456789/locations/global/mirroringDeploymentGroups/my-dg'.\nSee https://google.aip.dev/124."]
    pub fn mirroring_deployment_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mirroring_deployment_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mirroring_endpoint_group_id` after provisioning.\nThe ID to use for the endpoint group, which will become the final component\nof the endpoint group's resource name."]
    pub fn mirroring_endpoint_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mirroring_endpoint_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of this endpoint group, for example:\n'projects/123456789/locations/global/mirroringEndpointGroups/my-eg'.\nSee https://google.aip.dev/122 for more details."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nThe current state of the resource does not match the user's intended state,\nand the system is working to reconcile them. This is part of the normal\noperation (e.g. adding a new association to the group).\nSee https://google.aip.dev/128."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the endpoint group.\nSee https://google.aip.dev/216.\nPossible values:\nSTATE_UNSPECIFIED\nACTIVE\nCLOSED\nCREATING\nDELETING\nOUT_OF_SYNC\nDELETE_FAILED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the endpoint group.\nIf left unspecified, defaults to DIRECT.\nPossible values:\nDIRECT\nBROKER"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was most recently updated.\nSee https://google.aip.dev/148#timestamps."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecurityMirroringEndpointGroupTimeoutsElRef {
        NetworkSecurityMirroringEndpointGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityMirroringEndpointGroupAssociationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl NetworkSecurityMirroringEndpointGroupAssociationsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkSecurityMirroringEndpointGroupAssociationsEl {
    type O = BlockAssignable<NetworkSecurityMirroringEndpointGroupAssociationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityMirroringEndpointGroupAssociationsEl {}
impl BuildNetworkSecurityMirroringEndpointGroupAssociationsEl {
    pub fn build(self) -> NetworkSecurityMirroringEndpointGroupAssociationsEl {
        NetworkSecurityMirroringEndpointGroupAssociationsEl {
            name: core::default::Default::default(),
            network: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityMirroringEndpointGroupAssociationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityMirroringEndpointGroupAssociationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityMirroringEndpointGroupAssociationsElRef {
        NetworkSecurityMirroringEndpointGroupAssociationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityMirroringEndpointGroupAssociationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElLocationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElLocationsEl {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable
    for NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElLocationsEl
{
    type O = BlockAssignable<
        NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElLocationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElLocationsEl {}
impl BuildNetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElLocationsEl {
    pub fn build(
        self,
    ) -> NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElLocationsEl {
        NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElLocationsEl {
            location: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElLocationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElLocationsElRef {
        NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    locations: Option<
        SetField<NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElLocationsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsEl {
    #[doc = "Set the field `locations`.\n"]
    pub fn set_locations(
        mut self,
        v: impl Into<
            SetField<NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElLocationsEl>,
        >,
    ) -> Self {
        self.locations = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsEl {
    type O = BlockAssignable<NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsEl {}
impl BuildNetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsEl {
    pub fn build(self) -> NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsEl {
        NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsEl {
            locations: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElRef {
        NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(
        &self,
    ) -> SetRef<NetworkSecurityMirroringEndpointGroupConnectedDeploymentGroupsElLocationsElRef>
    {
        SetRef::new(self.shared().clone(), format!("{}.locations", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityMirroringEndpointGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkSecurityMirroringEndpointGroupTimeoutsEl {
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
impl ToListMappable for NetworkSecurityMirroringEndpointGroupTimeoutsEl {
    type O = BlockAssignable<NetworkSecurityMirroringEndpointGroupTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityMirroringEndpointGroupTimeoutsEl {}
impl BuildNetworkSecurityMirroringEndpointGroupTimeoutsEl {
    pub fn build(self) -> NetworkSecurityMirroringEndpointGroupTimeoutsEl {
        NetworkSecurityMirroringEndpointGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityMirroringEndpointGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityMirroringEndpointGroupTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityMirroringEndpointGroupTimeoutsElRef {
        NetworkSecurityMirroringEndpointGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityMirroringEndpointGroupTimeoutsElRef {
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
