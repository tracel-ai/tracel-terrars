use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VmwareengineNetworkPolicyData {
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
    edge_services_cidr: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    vmware_engine_network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_ip: Option<Vec<VmwareengineNetworkPolicyExternalIpEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internet_access: Option<Vec<VmwareengineNetworkPolicyInternetAccessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VmwareengineNetworkPolicyTimeoutsEl>,
    dynamic: VmwareengineNetworkPolicyDynamic,
}
struct VmwareengineNetworkPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VmwareengineNetworkPolicyData>,
}
#[derive(Clone)]
pub struct VmwareengineNetworkPolicy(Rc<VmwareengineNetworkPolicy_>);
impl VmwareengineNetworkPolicy {
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
    #[doc = "Set the field `description`.\nUser-provided description for this network policy."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `external_ip`.\n"]
    pub fn set_external_ip(
        self,
        v: impl Into<BlockAssignable<VmwareengineNetworkPolicyExternalIpEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().external_ip = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.external_ip = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `internet_access`.\n"]
    pub fn set_internet_access(
        self,
        v: impl Into<BlockAssignable<VmwareengineNetworkPolicyInternetAccessEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().internet_access = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.internet_access = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VmwareengineNetworkPolicyTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description for this network policy."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edge_services_cidr` after provisioning.\nIP address range in CIDR notation used to create internet access and external IP access.\nAn RFC 1918 CIDR block, with a \"/26\" prefix, is required. The range cannot overlap with any\nprefixes either in the consumer VPC network or in use by the private clouds attached to that VPC network."]
    pub fn edge_services_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edge_services_cidr", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe resource name of the location (region) to create the new network policy in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-central1"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the Network Policy."]
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
    #[doc = "Get a reference to the value of field `vmware_engine_network` after provisioning.\nThe relative resource name of the VMware Engine network. Specify the name in the following form:\nprojects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId} where {project}\ncan either be a project number or a project ID."]
    pub fn vmware_engine_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vmware_engine_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vmware_engine_network_canonical` after provisioning.\nThe canonical name of the VMware Engine network in the form:\nprojects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}"]
    pub fn vmware_engine_network_canonical(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vmware_engine_network_canonical", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_ip` after provisioning.\n"]
    pub fn external_ip(&self) -> ListRef<VmwareengineNetworkPolicyExternalIpElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `internet_access` after provisioning.\n"]
    pub fn internet_access(&self) -> ListRef<VmwareengineNetworkPolicyInternetAccessElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.internet_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineNetworkPolicyTimeoutsElRef {
        VmwareengineNetworkPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VmwareengineNetworkPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VmwareengineNetworkPolicy {}
impl ToListMappable for VmwareengineNetworkPolicy {
    type O = ListRef<VmwareengineNetworkPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VmwareengineNetworkPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_vmwareengine_network_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVmwareengineNetworkPolicy {
    pub tf_id: String,
    #[doc = "IP address range in CIDR notation used to create internet access and external IP access.\nAn RFC 1918 CIDR block, with a \"/26\" prefix, is required. The range cannot overlap with any\nprefixes either in the consumer VPC network or in use by the private clouds attached to that VPC network."]
    pub edge_services_cidr: PrimField<String>,
    #[doc = "The resource name of the location (region) to create the new network policy in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-central1"]
    pub location: PrimField<String>,
    #[doc = "The ID of the Network Policy."]
    pub name: PrimField<String>,
    #[doc = "The relative resource name of the VMware Engine network. Specify the name in the following form:\nprojects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId} where {project}\ncan either be a project number or a project ID."]
    pub vmware_engine_network: PrimField<String>,
}
impl BuildVmwareengineNetworkPolicy {
    pub fn build(self, stack: &mut Stack) -> VmwareengineNetworkPolicy {
        let out = VmwareengineNetworkPolicy(Rc::new(VmwareengineNetworkPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VmwareengineNetworkPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                edge_services_cidr: self.edge_services_cidr,
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                vmware_engine_network: self.vmware_engine_network,
                external_ip: core::default::Default::default(),
                internet_access: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VmwareengineNetworkPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineNetworkPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VmwareengineNetworkPolicyRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description for this network policy."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edge_services_cidr` after provisioning.\nIP address range in CIDR notation used to create internet access and external IP access.\nAn RFC 1918 CIDR block, with a \"/26\" prefix, is required. The range cannot overlap with any\nprefixes either in the consumer VPC network or in use by the private clouds attached to that VPC network."]
    pub fn edge_services_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edge_services_cidr", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe resource name of the location (region) to create the new network policy in.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-central1"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the Network Policy."]
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
    #[doc = "Get a reference to the value of field `vmware_engine_network` after provisioning.\nThe relative resource name of the VMware Engine network. Specify the name in the following form:\nprojects/{project}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId} where {project}\ncan either be a project number or a project ID."]
    pub fn vmware_engine_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vmware_engine_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vmware_engine_network_canonical` after provisioning.\nThe canonical name of the VMware Engine network in the form:\nprojects/{project_number}/locations/{location}/vmwareEngineNetworks/{vmwareEngineNetworkId}"]
    pub fn vmware_engine_network_canonical(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vmware_engine_network_canonical", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_ip` after provisioning.\n"]
    pub fn external_ip(&self) -> ListRef<VmwareengineNetworkPolicyExternalIpElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `internet_access` after provisioning.\n"]
    pub fn internet_access(&self) -> ListRef<VmwareengineNetworkPolicyInternetAccessElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.internet_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineNetworkPolicyTimeoutsElRef {
        VmwareengineNetworkPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VmwareengineNetworkPolicyExternalIpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl VmwareengineNetworkPolicyExternalIpEl {
    #[doc = "Set the field `enabled`.\nTrue if the service is enabled; false otherwise."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for VmwareengineNetworkPolicyExternalIpEl {
    type O = BlockAssignable<VmwareengineNetworkPolicyExternalIpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineNetworkPolicyExternalIpEl {}
impl BuildVmwareengineNetworkPolicyExternalIpEl {
    pub fn build(self) -> VmwareengineNetworkPolicyExternalIpEl {
        VmwareengineNetworkPolicyExternalIpEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct VmwareengineNetworkPolicyExternalIpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineNetworkPolicyExternalIpElRef {
    fn new(shared: StackShared, base: String) -> VmwareengineNetworkPolicyExternalIpElRef {
        VmwareengineNetworkPolicyExternalIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineNetworkPolicyExternalIpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nTrue if the service is enabled; false otherwise."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the service. New values may be added to this enum when appropriate."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct VmwareengineNetworkPolicyInternetAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl VmwareengineNetworkPolicyInternetAccessEl {
    #[doc = "Set the field `enabled`.\nTrue if the service is enabled; false otherwise."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for VmwareengineNetworkPolicyInternetAccessEl {
    type O = BlockAssignable<VmwareengineNetworkPolicyInternetAccessEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineNetworkPolicyInternetAccessEl {}
impl BuildVmwareengineNetworkPolicyInternetAccessEl {
    pub fn build(self) -> VmwareengineNetworkPolicyInternetAccessEl {
        VmwareengineNetworkPolicyInternetAccessEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct VmwareengineNetworkPolicyInternetAccessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineNetworkPolicyInternetAccessElRef {
    fn new(shared: StackShared, base: String) -> VmwareengineNetworkPolicyInternetAccessElRef {
        VmwareengineNetworkPolicyInternetAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineNetworkPolicyInternetAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nTrue if the service is enabled; false otherwise."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the service. New values may be added to this enum when appropriate."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct VmwareengineNetworkPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VmwareengineNetworkPolicyTimeoutsEl {
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
impl ToListMappable for VmwareengineNetworkPolicyTimeoutsEl {
    type O = BlockAssignable<VmwareengineNetworkPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineNetworkPolicyTimeoutsEl {}
impl BuildVmwareengineNetworkPolicyTimeoutsEl {
    pub fn build(self) -> VmwareengineNetworkPolicyTimeoutsEl {
        VmwareengineNetworkPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VmwareengineNetworkPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineNetworkPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VmwareengineNetworkPolicyTimeoutsElRef {
        VmwareengineNetworkPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineNetworkPolicyTimeoutsElRef {
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
struct VmwareengineNetworkPolicyDynamic {
    external_ip: Option<DynamicBlock<VmwareengineNetworkPolicyExternalIpEl>>,
    internet_access: Option<DynamicBlock<VmwareengineNetworkPolicyInternetAccessEl>>,
}
