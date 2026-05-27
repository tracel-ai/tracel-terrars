use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct EdgecontainerVpnConnectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_high_availability: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nat_gateway_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    router: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EdgecontainerVpnConnectionTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_project: Option<Vec<EdgecontainerVpnConnectionVpcProjectEl>>,
    dynamic: EdgecontainerVpnConnectionDynamic,
}
struct EdgecontainerVpnConnection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EdgecontainerVpnConnectionData>,
}
#[derive(Clone)]
pub struct EdgecontainerVpnConnection(Rc<EdgecontainerVpnConnection_>);
impl EdgecontainerVpnConnection {
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
    #[doc = "Set the field `enable_high_availability`.\nWhether this VPN connection has HA enabled on cluster side. If enabled, when creating VPN connection we will attempt to use 2 ANG floating IPs."]
    pub fn set_enable_high_availability(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_high_availability = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels associated with this resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `nat_gateway_ip`.\nNAT gateway IP, or WAN IP address. If a customer has multiple NAT IPs, the customer needs to configure NAT such that only one external IP maps to the GMEC Anthos cluster.\nThis is empty if NAT is not used."]
    pub fn set_nat_gateway_ip(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().nat_gateway_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `router`.\nThe VPN connection Cloud Router name."]
    pub fn set_router(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().router = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc`.\nThe network ID of VPC to connect to."]
    pub fn set_vpc(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpc = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EdgecontainerVpnConnectionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_project`.\n"]
    pub fn set_vpc_project(
        self,
        v: impl Into<BlockAssignable<EdgecontainerVpnConnectionVpcProjectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_project = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_project = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe canonical Cluster name to connect to. It is in the form of projects/{project}/locations/{location}/clusters/{cluster}."]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the VPN connection was created."]
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
    #[doc = "Get a reference to the value of field `details` after provisioning.\nA nested object resource."]
    pub fn details(&self) -> ListRef<EdgecontainerVpnConnectionDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_high_availability` after provisioning.\nWhether this VPN connection has HA enabled on cluster side. If enabled, when creating VPN connection we will attempt to use 2 ANG floating IPs."]
    pub fn enable_high_availability(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_high_availability", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels associated with this resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nGoogle Cloud Platform location."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of VPN connection"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nat_gateway_ip` after provisioning.\nNAT gateway IP, or WAN IP address. If a customer has multiple NAT IPs, the customer needs to configure NAT such that only one external IP maps to the GMEC Anthos cluster.\nThis is empty if NAT is not used."]
    pub fn nat_gateway_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nat_gateway_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `router` after provisioning.\nThe VPN connection Cloud Router name."]
    pub fn router(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.router", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the VPN connection was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc` after provisioning.\nThe network ID of VPC to connect to."]
    pub fn vpc(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EdgecontainerVpnConnectionTimeoutsElRef {
        EdgecontainerVpnConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_project` after provisioning.\n"]
    pub fn vpc_project(&self) -> ListRef<EdgecontainerVpnConnectionVpcProjectElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_project", self.extract_ref()),
        )
    }
}
impl Referable for EdgecontainerVpnConnection {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for EdgecontainerVpnConnection {}
impl ToListMappable for EdgecontainerVpnConnection {
    type O = ListRef<EdgecontainerVpnConnectionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for EdgecontainerVpnConnection_ {
    fn extract_resource_type(&self) -> String {
        "google_edgecontainer_vpn_connection".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildEdgecontainerVpnConnection {
    pub tf_id: String,
    #[doc = "The canonical Cluster name to connect to. It is in the form of projects/{project}/locations/{location}/clusters/{cluster}."]
    pub cluster: PrimField<String>,
    #[doc = "Google Cloud Platform location."]
    pub location: PrimField<String>,
    #[doc = "The resource name of VPN connection"]
    pub name: PrimField<String>,
}
impl BuildEdgecontainerVpnConnection {
    pub fn build(self, stack: &mut Stack) -> EdgecontainerVpnConnection {
        let out = EdgecontainerVpnConnection(Rc::new(EdgecontainerVpnConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EdgecontainerVpnConnectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cluster: self.cluster,
                deletion_policy: core::default::Default::default(),
                enable_high_availability: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                nat_gateway_ip: core::default::Default::default(),
                project: core::default::Default::default(),
                router: core::default::Default::default(),
                vpc: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_project: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct EdgecontainerVpnConnectionRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerVpnConnectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl EdgecontainerVpnConnectionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nThe canonical Cluster name to connect to. It is in the form of projects/{project}/locations/{location}/clusters/{cluster}."]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time when the VPN connection was created."]
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
    #[doc = "Get a reference to the value of field `details` after provisioning.\nA nested object resource."]
    pub fn details(&self) -> ListRef<EdgecontainerVpnConnectionDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_high_availability` after provisioning.\nWhether this VPN connection has HA enabled on cluster side. If enabled, when creating VPN connection we will attempt to use 2 ANG floating IPs."]
    pub fn enable_high_availability(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_high_availability", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels associated with this resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nGoogle Cloud Platform location."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of VPN connection"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nat_gateway_ip` after provisioning.\nNAT gateway IP, or WAN IP address. If a customer has multiple NAT IPs, the customer needs to configure NAT such that only one external IP maps to the GMEC Anthos cluster.\nThis is empty if NAT is not used."]
    pub fn nat_gateway_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.nat_gateway_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `router` after provisioning.\nThe VPN connection Cloud Router name."]
    pub fn router(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.router", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe time when the VPN connection was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc` after provisioning.\nThe network ID of VPC to connect to."]
    pub fn vpc(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EdgecontainerVpnConnectionTimeoutsElRef {
        EdgecontainerVpnConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_project` after provisioning.\n"]
    pub fn vpc_project(&self) -> ListRef<EdgecontainerVpnConnectionVpcProjectElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_project", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct EdgecontainerVpnConnectionDetailsElCloudRouterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl EdgecontainerVpnConnectionDetailsElCloudRouterEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for EdgecontainerVpnConnectionDetailsElCloudRouterEl {
    type O = BlockAssignable<EdgecontainerVpnConnectionDetailsElCloudRouterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerVpnConnectionDetailsElCloudRouterEl {}
impl BuildEdgecontainerVpnConnectionDetailsElCloudRouterEl {
    pub fn build(self) -> EdgecontainerVpnConnectionDetailsElCloudRouterEl {
        EdgecontainerVpnConnectionDetailsElCloudRouterEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct EdgecontainerVpnConnectionDetailsElCloudRouterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerVpnConnectionDetailsElCloudRouterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EdgecontainerVpnConnectionDetailsElCloudRouterElRef {
        EdgecontainerVpnConnectionDetailsElCloudRouterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerVpnConnectionDetailsElCloudRouterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct EdgecontainerVpnConnectionDetailsElCloudVpnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway: Option<PrimField<String>>,
}
impl EdgecontainerVpnConnectionDetailsElCloudVpnsEl {
    #[doc = "Set the field `gateway`.\n"]
    pub fn set_gateway(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gateway = Some(v.into());
        self
    }
}
impl ToListMappable for EdgecontainerVpnConnectionDetailsElCloudVpnsEl {
    type O = BlockAssignable<EdgecontainerVpnConnectionDetailsElCloudVpnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerVpnConnectionDetailsElCloudVpnsEl {}
impl BuildEdgecontainerVpnConnectionDetailsElCloudVpnsEl {
    pub fn build(self) -> EdgecontainerVpnConnectionDetailsElCloudVpnsEl {
        EdgecontainerVpnConnectionDetailsElCloudVpnsEl {
            gateway: core::default::Default::default(),
        }
    }
}
pub struct EdgecontainerVpnConnectionDetailsElCloudVpnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerVpnConnectionDetailsElCloudVpnsElRef {
    fn new(shared: StackShared, base: String) -> EdgecontainerVpnConnectionDetailsElCloudVpnsElRef {
        EdgecontainerVpnConnectionDetailsElCloudVpnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerVpnConnectionDetailsElCloudVpnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gateway` after provisioning.\n"]
    pub fn gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gateway", self.base))
    }
}
#[derive(Serialize)]
pub struct EdgecontainerVpnConnectionDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_router: Option<ListField<EdgecontainerVpnConnectionDetailsElCloudRouterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_vpns: Option<ListField<EdgecontainerVpnConnectionDetailsElCloudVpnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl EdgecontainerVpnConnectionDetailsEl {
    #[doc = "Set the field `cloud_router`.\n"]
    pub fn set_cloud_router(
        mut self,
        v: impl Into<ListField<EdgecontainerVpnConnectionDetailsElCloudRouterEl>>,
    ) -> Self {
        self.cloud_router = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_vpns`.\n"]
    pub fn set_cloud_vpns(
        mut self,
        v: impl Into<ListField<EdgecontainerVpnConnectionDetailsElCloudVpnsEl>>,
    ) -> Self {
        self.cloud_vpns = Some(v.into());
        self
    }
    #[doc = "Set the field `error`.\n"]
    pub fn set_error(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for EdgecontainerVpnConnectionDetailsEl {
    type O = BlockAssignable<EdgecontainerVpnConnectionDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerVpnConnectionDetailsEl {}
impl BuildEdgecontainerVpnConnectionDetailsEl {
    pub fn build(self) -> EdgecontainerVpnConnectionDetailsEl {
        EdgecontainerVpnConnectionDetailsEl {
            cloud_router: core::default::Default::default(),
            cloud_vpns: core::default::Default::default(),
            error: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct EdgecontainerVpnConnectionDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerVpnConnectionDetailsElRef {
    fn new(shared: StackShared, base: String) -> EdgecontainerVpnConnectionDetailsElRef {
        EdgecontainerVpnConnectionDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerVpnConnectionDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_router` after provisioning.\n"]
    pub fn cloud_router(&self) -> ListRef<EdgecontainerVpnConnectionDetailsElCloudRouterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloud_router", self.base))
    }
    #[doc = "Get a reference to the value of field `cloud_vpns` after provisioning.\n"]
    pub fn cloud_vpns(&self) -> ListRef<EdgecontainerVpnConnectionDetailsElCloudVpnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloud_vpns", self.base))
    }
    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct EdgecontainerVpnConnectionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl EdgecontainerVpnConnectionTimeoutsEl {
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
impl ToListMappable for EdgecontainerVpnConnectionTimeoutsEl {
    type O = BlockAssignable<EdgecontainerVpnConnectionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerVpnConnectionTimeoutsEl {}
impl BuildEdgecontainerVpnConnectionTimeoutsEl {
    pub fn build(self) -> EdgecontainerVpnConnectionTimeoutsEl {
        EdgecontainerVpnConnectionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct EdgecontainerVpnConnectionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerVpnConnectionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EdgecontainerVpnConnectionTimeoutsElRef {
        EdgecontainerVpnConnectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerVpnConnectionTimeoutsElRef {
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
pub struct EdgecontainerVpnConnectionVpcProjectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
}
impl EdgecontainerVpnConnectionVpcProjectEl {
    #[doc = "Set the field `project_id`.\nThe project of the VPC to connect to. If not specified, it is the same as the cluster project."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}
impl ToListMappable for EdgecontainerVpnConnectionVpcProjectEl {
    type O = BlockAssignable<EdgecontainerVpnConnectionVpcProjectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildEdgecontainerVpnConnectionVpcProjectEl {}
impl BuildEdgecontainerVpnConnectionVpcProjectEl {
    pub fn build(self) -> EdgecontainerVpnConnectionVpcProjectEl {
        EdgecontainerVpnConnectionVpcProjectEl {
            project_id: core::default::Default::default(),
        }
    }
}
pub struct EdgecontainerVpnConnectionVpcProjectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for EdgecontainerVpnConnectionVpcProjectElRef {
    fn new(shared: StackShared, base: String) -> EdgecontainerVpnConnectionVpcProjectElRef {
        EdgecontainerVpnConnectionVpcProjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl EdgecontainerVpnConnectionVpcProjectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe project of the VPC to connect to. If not specified, it is the same as the cluster project."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct EdgecontainerVpnConnectionDynamic {
    vpc_project: Option<DynamicBlock<EdgecontainerVpnConnectionVpcProjectEl>>,
}
