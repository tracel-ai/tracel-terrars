use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkSecurityFirewallEndpointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_settings: Option<Vec<NetworkSecurityFirewallEndpointEndpointSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkSecurityFirewallEndpointTimeoutsEl>,
    dynamic: NetworkSecurityFirewallEndpointDynamic,
}
struct NetworkSecurityFirewallEndpoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkSecurityFirewallEndpointData>,
}
#[derive(Clone)]
pub struct NetworkSecurityFirewallEndpoint(Rc<NetworkSecurityFirewallEndpoint_>);
impl NetworkSecurityFirewallEndpoint {
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
    #[doc = "Set the field `billing_project_id`.\nProject to charge for the deployed firewall endpoint.\nThis field is required for organization-scoped endpoints.\nFor project-scoped endpoints, it is optional but must match the\nendpoint's project if specified."]
    pub fn set_billing_project_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().billing_project_id = Some(v.into());
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
    #[doc = "Set the field `labels`.\nA map of key/value label pairs to assign to the resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoint_settings`.\n"]
    pub fn set_endpoint_settings(
        self,
        v: impl Into<BlockAssignable<NetworkSecurityFirewallEndpointEndpointSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkSecurityFirewallEndpointTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `associated_networks` after provisioning.\nList of networks that are associated with this endpoint in the local zone.\nThis is a projection of the FirewallEndpointAssociations pointing at this\nendpoint. A network will only appear in this list after traffic routing is\nfully configured. Format: projects/{project}/global/networks/{name}."]
    pub fn associated_networks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.associated_networks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `billing_project_id` after provisioning.\nProject to charge for the deployed firewall endpoint.\nThis field is required for organization-scoped endpoints.\nFor project-scoped endpoints, it is optional but must match the\nendpoint's project if specified."]
    pub fn billing_project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_project_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the firewall endpoint was created in UTC."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA map of key/value label pairs to assign to the resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location (zone) of the firewall endpoint."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the firewall endpoint resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe name of the parent this firewall endpoint belongs to.\nFormat: 'organizations/{organization_id}' or 'projects/{project_id}'."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nWhether reconciling is in progress, recommended per https://google.aip.dev/128."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL of this resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the endpoint."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the firewall endpoint was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_settings` after provisioning.\n"]
    pub fn endpoint_settings(
        &self,
    ) -> ListRef<NetworkSecurityFirewallEndpointEndpointSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecurityFirewallEndpointTimeoutsElRef {
        NetworkSecurityFirewallEndpointTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkSecurityFirewallEndpoint {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkSecurityFirewallEndpoint {}
impl ToListMappable for NetworkSecurityFirewallEndpoint {
    type O = ListRef<NetworkSecurityFirewallEndpointRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkSecurityFirewallEndpoint_ {
    fn extract_resource_type(&self) -> String {
        "google_network_security_firewall_endpoint".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkSecurityFirewallEndpoint {
    pub tf_id: String,
    #[doc = "The location (zone) of the firewall endpoint."]
    pub location: PrimField<String>,
    #[doc = "The name of the firewall endpoint resource."]
    pub name: PrimField<String>,
    #[doc = "The name of the parent this firewall endpoint belongs to.\nFormat: 'organizations/{organization_id}' or 'projects/{project_id}'."]
    pub parent: PrimField<String>,
}
impl BuildNetworkSecurityFirewallEndpoint {
    pub fn build(self, stack: &mut Stack) -> NetworkSecurityFirewallEndpoint {
        let out = NetworkSecurityFirewallEndpoint(Rc::new(NetworkSecurityFirewallEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkSecurityFirewallEndpointData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                billing_project_id: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                parent: self.parent,
                endpoint_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkSecurityFirewallEndpointRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityFirewallEndpointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkSecurityFirewallEndpointRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `associated_networks` after provisioning.\nList of networks that are associated with this endpoint in the local zone.\nThis is a projection of the FirewallEndpointAssociations pointing at this\nendpoint. A network will only appear in this list after traffic routing is\nfully configured. Format: projects/{project}/global/networks/{name}."]
    pub fn associated_networks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.associated_networks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `billing_project_id` after provisioning.\nProject to charge for the deployed firewall endpoint.\nThis field is required for organization-scoped endpoints.\nFor project-scoped endpoints, it is optional but must match the\nendpoint's project if specified."]
    pub fn billing_project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_project_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the firewall endpoint was created in UTC."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA map of key/value label pairs to assign to the resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location (zone) of the firewall endpoint."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the firewall endpoint resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe name of the parent this firewall endpoint belongs to.\nFormat: 'organizations/{organization_id}' or 'projects/{project_id}'."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nWhether reconciling is in progress, recommended per https://google.aip.dev/128."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL of this resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the endpoint."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the firewall endpoint was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_settings` after provisioning.\n"]
    pub fn endpoint_settings(
        &self,
    ) -> ListRef<NetworkSecurityFirewallEndpointEndpointSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecurityFirewallEndpointTimeoutsElRef {
        NetworkSecurityFirewallEndpointTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityFirewallEndpointEndpointSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    jumbo_frames_enabled: Option<PrimField<bool>>,
}
impl NetworkSecurityFirewallEndpointEndpointSettingsEl {
    #[doc = "Set the field `jumbo_frames_enabled`.\nIndicates whether Jumbo Frames are enabled for the firewall endpoint."]
    pub fn set_jumbo_frames_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.jumbo_frames_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkSecurityFirewallEndpointEndpointSettingsEl {
    type O = BlockAssignable<NetworkSecurityFirewallEndpointEndpointSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityFirewallEndpointEndpointSettingsEl {}
impl BuildNetworkSecurityFirewallEndpointEndpointSettingsEl {
    pub fn build(self) -> NetworkSecurityFirewallEndpointEndpointSettingsEl {
        NetworkSecurityFirewallEndpointEndpointSettingsEl {
            jumbo_frames_enabled: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityFirewallEndpointEndpointSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityFirewallEndpointEndpointSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkSecurityFirewallEndpointEndpointSettingsElRef {
        NetworkSecurityFirewallEndpointEndpointSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityFirewallEndpointEndpointSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `jumbo_frames_enabled` after provisioning.\nIndicates whether Jumbo Frames are enabled for the firewall endpoint."]
    pub fn jumbo_frames_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.jumbo_frames_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkSecurityFirewallEndpointTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkSecurityFirewallEndpointTimeoutsEl {
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
impl ToListMappable for NetworkSecurityFirewallEndpointTimeoutsEl {
    type O = BlockAssignable<NetworkSecurityFirewallEndpointTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkSecurityFirewallEndpointTimeoutsEl {}
impl BuildNetworkSecurityFirewallEndpointTimeoutsEl {
    pub fn build(self) -> NetworkSecurityFirewallEndpointTimeoutsEl {
        NetworkSecurityFirewallEndpointTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkSecurityFirewallEndpointTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkSecurityFirewallEndpointTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkSecurityFirewallEndpointTimeoutsElRef {
        NetworkSecurityFirewallEndpointTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkSecurityFirewallEndpointTimeoutsElRef {
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
struct NetworkSecurityFirewallEndpointDynamic {
    endpoint_settings: Option<DynamicBlock<NetworkSecurityFirewallEndpointEndpointSettingsEl>>,
}
