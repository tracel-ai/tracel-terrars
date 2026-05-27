use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkSecurityInterceptEndpointGroupAssociationData {
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
    intercept_endpoint_group: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intercept_endpoint_group_association_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkSecurityInterceptEndpointGroupAssociationTimeoutsEl>,
}
struct NetworkSecurityInterceptEndpointGroupAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkSecurityInterceptEndpointGroupAssociationData>,
}
#[derive(Clone)]
pub struct NetworkSecurityInterceptEndpointGroupAssociation(
    Rc<NetworkSecurityInterceptEndpointGroupAssociation_>,
);
impl NetworkSecurityInterceptEndpointGroupAssociation {
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
    #[doc = "Set the field `intercept_endpoint_group_association_id`.\nThe ID to use for the new association, which will become the final\ncomponent of the endpoint group's resource name. If not provided, the\nserver will generate a unique ID."]
    pub fn set_intercept_endpoint_group_association_id(
        self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .intercept_endpoint_group_association_id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels are key/value pairs that help to organize and filter resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<NetworkSecurityInterceptEndpointGroupAssociationTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
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
    #[doc = "Get a reference to the value of field `intercept_endpoint_group` after provisioning.\nThe endpoint group that this association is connected to, for example:\n'projects/123456789/locations/global/interceptEndpointGroups/my-eg'.\nSee https://google.aip.dev/124."]
    pub fn intercept_endpoint_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.intercept_endpoint_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `intercept_endpoint_group_association_id` after provisioning.\nThe ID to use for the new association, which will become the final\ncomponent of the endpoint group's resource name. If not provided, the\nserver will generate a unique ID."]
    pub fn intercept_endpoint_group_association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.intercept_endpoint_group_association_id",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels are key/value pairs that help to organize and filter resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe cloud location of the association, currently restricted to 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\nThe list of locations where the association is configured. This information\nis retrieved from the linked endpoint group."]
    pub fn locations(
        &self,
    ) -> SetRef<NetworkSecurityInterceptEndpointGroupAssociationLocationsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `locations_details` after provisioning.\nThe list of locations where the association is present. This information\nis retrieved from the linked endpoint group, and not configured as part\nof the association itself."]
    pub fn locations_details(
        &self,
    ) -> ListRef<NetworkSecurityInterceptEndpointGroupAssociationLocationsDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.locations_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of this endpoint group association, for example:\n'projects/123456789/locations/global/interceptEndpointGroupAssociations/my-eg-association'.\nSee https://google.aip.dev/122 for more details."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe VPC network that is associated. for example:\n'projects/123456789/global/networks/my-network'.\nSee https://google.aip.dev/124."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nThe current state of the resource does not match the user's intended state,\nand the system is working to reconcile them. This part of the normal\noperation (e.g. adding a new location to the target deployment group).\nSee https://google.aip.dev/128."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the endpoint group association.\nPossible values:\nSTATE_UNSPECIFIED\nACTIVE\nCREATING\nDELETING\nCLOSED\nOUT_OF_SYNC\nDELETE_FAILED"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was most recently updated.\nSee https://google.aip.dev/148#timestamps."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecurityInterceptEndpointGroupAssociationTimeoutsElRef {
        NetworkSecurityInterceptEndpointGroupAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkSecurityInterceptEndpointGroupAssociation {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkSecurityInterceptEndpointGroupAssociation {}
impl ToListMappable for NetworkSecurityInterceptEndpointGroupAssociation {
    type O = ListRef<NetworkSecurityInterceptEndpointGroupAssociationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkSecurityInterceptEndpointGroupAssociation_ {
    fn extract_resource_type(&self) -> String {
        "google_network_security_intercept_endpoint_group_association".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkSecurityInterceptEndpointGroupAssociation {
    pub tf_id: String,
    #[doc = "The endpoint group that this association is connected to, for example:\n'projects/123456789/locations/global/interceptEndpointGroups/my-eg'.\nSee https://google.aip.dev/124."]
    pub intercept_endpoint_group: PrimField<String>,
    #[doc = "The cloud location of the association, currently restricted to 'global'."]
    pub location: PrimField<String>,
    #[doc = "The VPC network that is associated. for example:\n'projects/123456789/global/networks/my-network'.\nSee https://google.aip.dev/124."]
    pub network: PrimField<String>,
}
impl BuildNetworkSecurityInterceptEndpointGroupAssociation {
    pub fn build(self, stack: &mut Stack) -> NetworkSecurityInterceptEndpointGroupAssociation {
        let out = NetworkSecurityInterceptEndpointGroupAssociation(Rc::new(
            NetworkSecurityInterceptEndpointGroupAssociation_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(NetworkSecurityInterceptEndpointGroupAssociationData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    id: core::default::Default::default(),
                    intercept_endpoint_group: self.intercept_endpoint_group,
                    intercept_endpoint_group_association_id: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    location: self.location,
                    network: self.network,
                    project: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkSecurityInterceptEndpointGroupAssociationRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityInterceptEndpointGroupAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkSecurityInterceptEndpointGroupAssociationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `intercept_endpoint_group` after provisioning.\nThe endpoint group that this association is connected to, for example:\n'projects/123456789/locations/global/interceptEndpointGroups/my-eg'.\nSee https://google.aip.dev/124."]
    pub fn intercept_endpoint_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.intercept_endpoint_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `intercept_endpoint_group_association_id` after provisioning.\nThe ID to use for the new association, which will become the final\ncomponent of the endpoint group's resource name. If not provided, the\nserver will generate a unique ID."]
    pub fn intercept_endpoint_group_association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.intercept_endpoint_group_association_id",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels are key/value pairs that help to organize and filter resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe cloud location of the association, currently restricted to 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\nThe list of locations where the association is configured. This information\nis retrieved from the linked endpoint group."]
    pub fn locations(
        &self,
    ) -> SetRef<NetworkSecurityInterceptEndpointGroupAssociationLocationsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `locations_details` after provisioning.\nThe list of locations where the association is present. This information\nis retrieved from the linked endpoint group, and not configured as part\nof the association itself."]
    pub fn locations_details(
        &self,
    ) -> ListRef<NetworkSecurityInterceptEndpointGroupAssociationLocationsDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.locations_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of this endpoint group association, for example:\n'projects/123456789/locations/global/interceptEndpointGroupAssociations/my-eg-association'.\nSee https://google.aip.dev/122 for more details."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe VPC network that is associated. for example:\n'projects/123456789/global/networks/my-network'.\nSee https://google.aip.dev/124."]
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
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nThe current state of the resource does not match the user's intended state,\nand the system is working to reconcile them. This part of the normal\noperation (e.g. adding a new location to the target deployment group).\nSee https://google.aip.dev/128."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the endpoint group association.\nPossible values:\nSTATE_UNSPECIFIED\nACTIVE\nCREATING\nDELETING\nCLOSED\nOUT_OF_SYNC\nDELETE_FAILED"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was most recently updated.\nSee https://google.aip.dev/148#timestamps."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecurityInterceptEndpointGroupAssociationTimeoutsElRef {
        NetworkSecurityInterceptEndpointGroupAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityInterceptEndpointGroupAssociationLocationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl NetworkSecurityInterceptEndpointGroupAssociationLocationsEl {
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
impl ToListMappable for NetworkSecurityInterceptEndpointGroupAssociationLocationsEl {
    type O = BlockAssignable<NetworkSecurityInterceptEndpointGroupAssociationLocationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityInterceptEndpointGroupAssociationLocationsEl {}
impl BuildNetworkSecurityInterceptEndpointGroupAssociationLocationsEl {
    pub fn build(self) -> NetworkSecurityInterceptEndpointGroupAssociationLocationsEl {
        NetworkSecurityInterceptEndpointGroupAssociationLocationsEl {
            location: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityInterceptEndpointGroupAssociationLocationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityInterceptEndpointGroupAssociationLocationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityInterceptEndpointGroupAssociationLocationsElRef {
        NetworkSecurityInterceptEndpointGroupAssociationLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityInterceptEndpointGroupAssociationLocationsElRef {
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
pub struct NetworkSecurityInterceptEndpointGroupAssociationLocationsDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl NetworkSecurityInterceptEndpointGroupAssociationLocationsDetailsEl {
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
impl ToListMappable for NetworkSecurityInterceptEndpointGroupAssociationLocationsDetailsEl {
    type O = BlockAssignable<NetworkSecurityInterceptEndpointGroupAssociationLocationsDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityInterceptEndpointGroupAssociationLocationsDetailsEl {}
impl BuildNetworkSecurityInterceptEndpointGroupAssociationLocationsDetailsEl {
    pub fn build(self) -> NetworkSecurityInterceptEndpointGroupAssociationLocationsDetailsEl {
        NetworkSecurityInterceptEndpointGroupAssociationLocationsDetailsEl {
            location: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityInterceptEndpointGroupAssociationLocationsDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityInterceptEndpointGroupAssociationLocationsDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityInterceptEndpointGroupAssociationLocationsDetailsElRef {
        NetworkSecurityInterceptEndpointGroupAssociationLocationsDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityInterceptEndpointGroupAssociationLocationsDetailsElRef {
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
pub struct NetworkSecurityInterceptEndpointGroupAssociationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkSecurityInterceptEndpointGroupAssociationTimeoutsEl {
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
impl ToListMappable for NetworkSecurityInterceptEndpointGroupAssociationTimeoutsEl {
    type O = BlockAssignable<NetworkSecurityInterceptEndpointGroupAssociationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityInterceptEndpointGroupAssociationTimeoutsEl {}
impl BuildNetworkSecurityInterceptEndpointGroupAssociationTimeoutsEl {
    pub fn build(self) -> NetworkSecurityInterceptEndpointGroupAssociationTimeoutsEl {
        NetworkSecurityInterceptEndpointGroupAssociationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityInterceptEndpointGroupAssociationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityInterceptEndpointGroupAssociationTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityInterceptEndpointGroupAssociationTimeoutsElRef {
        NetworkSecurityInterceptEndpointGroupAssociationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityInterceptEndpointGroupAssociationTimeoutsElRef {
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
