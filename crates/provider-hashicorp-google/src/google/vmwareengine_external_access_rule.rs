use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VmwareengineExternalAccessRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    destination_ports: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    ip_protocol: PrimField<String>,
    name: PrimField<String>,
    parent: PrimField<String>,
    priority: PrimField<f64>,
    source_ports: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_ip_ranges: Option<Vec<VmwareengineExternalAccessRuleDestinationIpRangesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_ip_ranges: Option<Vec<VmwareengineExternalAccessRuleSourceIpRangesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VmwareengineExternalAccessRuleTimeoutsEl>,
    dynamic: VmwareengineExternalAccessRuleDynamic,
}
struct VmwareengineExternalAccessRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VmwareengineExternalAccessRuleData>,
}
#[derive(Clone)]
pub struct VmwareengineExternalAccessRule(Rc<VmwareengineExternalAccessRule_>);
impl VmwareengineExternalAccessRule {
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
    #[doc = "Set the field `description`.\nUser-provided description for the external access rule."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `destination_ip_ranges`.\n"]
    pub fn set_destination_ip_ranges(
        self,
        v: impl Into<BlockAssignable<VmwareengineExternalAccessRuleDestinationIpRangesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_ip_ranges = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_ip_ranges = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_ip_ranges`.\n"]
    pub fn set_source_ip_ranges(
        self,
        v: impl Into<BlockAssignable<VmwareengineExternalAccessRuleSourceIpRangesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_ip_ranges = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_ip_ranges = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VmwareengineExternalAccessRuleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe action that the external access rule performs. Possible values: [\"ALLOW\", \"DENY\"]"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description for the external access rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_ports` after provisioning.\nA list of destination ports to which the external access rule applies."]
    pub fn destination_ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_ports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_protocol` after provisioning.\nThe IP protocol to which the external access rule applies."]
    pub fn ip_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the external access rule."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the network policy.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/networkPolicies/my-policy"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nExternal access rule priority, which determines the external access rule to use when multiple rules apply."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_ports` after provisioning.\nA list of source ports to which the external access rule applies."]
    pub fn source_ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_ports", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `destination_ip_ranges` after provisioning.\n"]
    pub fn destination_ip_ranges(
        &self,
    ) -> ListRef<VmwareengineExternalAccessRuleDestinationIpRangesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_ip_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_ip_ranges` after provisioning.\n"]
    pub fn source_ip_ranges(&self) -> ListRef<VmwareengineExternalAccessRuleSourceIpRangesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_ip_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineExternalAccessRuleTimeoutsElRef {
        VmwareengineExternalAccessRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VmwareengineExternalAccessRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VmwareengineExternalAccessRule {}
impl ToListMappable for VmwareengineExternalAccessRule {
    type O = ListRef<VmwareengineExternalAccessRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VmwareengineExternalAccessRule_ {
    fn extract_resource_type(&self) -> String {
        "google_vmwareengine_external_access_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVmwareengineExternalAccessRule {
    pub tf_id: String,
    #[doc = "The action that the external access rule performs. Possible values: [\"ALLOW\", \"DENY\"]"]
    pub action: PrimField<String>,
    #[doc = "A list of destination ports to which the external access rule applies."]
    pub destination_ports: ListField<PrimField<String>>,
    #[doc = "The IP protocol to which the external access rule applies."]
    pub ip_protocol: PrimField<String>,
    #[doc = "The ID of the external access rule."]
    pub name: PrimField<String>,
    #[doc = "The resource name of the network policy.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/networkPolicies/my-policy"]
    pub parent: PrimField<String>,
    #[doc = "External access rule priority, which determines the external access rule to use when multiple rules apply."]
    pub priority: PrimField<f64>,
    #[doc = "A list of source ports to which the external access rule applies."]
    pub source_ports: ListField<PrimField<String>>,
}
impl BuildVmwareengineExternalAccessRule {
    pub fn build(self, stack: &mut Stack) -> VmwareengineExternalAccessRule {
        let out = VmwareengineExternalAccessRule(Rc::new(VmwareengineExternalAccessRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VmwareengineExternalAccessRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                action: self.action,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                destination_ports: self.destination_ports,
                id: core::default::Default::default(),
                ip_protocol: self.ip_protocol,
                name: self.name,
                parent: self.parent,
                priority: self.priority,
                source_ports: self.source_ports,
                destination_ip_ranges: core::default::Default::default(),
                source_ip_ranges: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VmwareengineExternalAccessRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineExternalAccessRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VmwareengineExternalAccessRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe action that the external access rule performs. Possible values: [\"ALLOW\", \"DENY\"]"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-provided description for the external access rule."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_ports` after provisioning.\nA list of destination ports to which the external access rule applies."]
    pub fn destination_ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_ports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_protocol` after provisioning.\nThe IP protocol to which the external access rule applies."]
    pub fn ip_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the external access rule."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the network policy.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/networkPolicies/my-policy"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nExternal access rule priority, which determines the external access rule to use when multiple rules apply."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_ports` after provisioning.\nA list of source ports to which the external access rule applies."]
    pub fn source_ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_ports", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `destination_ip_ranges` after provisioning.\n"]
    pub fn destination_ip_ranges(
        &self,
    ) -> ListRef<VmwareengineExternalAccessRuleDestinationIpRangesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_ip_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_ip_ranges` after provisioning.\n"]
    pub fn source_ip_ranges(&self) -> ListRef<VmwareengineExternalAccessRuleSourceIpRangesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_ip_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineExternalAccessRuleTimeoutsElRef {
        VmwareengineExternalAccessRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VmwareengineExternalAccessRuleDestinationIpRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    external_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_range: Option<PrimField<String>>,
}
impl VmwareengineExternalAccessRuleDestinationIpRangesEl {
    #[doc = "Set the field `external_address`.\nThe name of an 'ExternalAddress' resource."]
    pub fn set_external_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_address = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address_range`.\nAn IP address range in the CIDR format."]
    pub fn set_ip_address_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address_range = Some(v.into());
        self
    }
}
impl ToListMappable for VmwareengineExternalAccessRuleDestinationIpRangesEl {
    type O = BlockAssignable<VmwareengineExternalAccessRuleDestinationIpRangesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineExternalAccessRuleDestinationIpRangesEl {}
impl BuildVmwareengineExternalAccessRuleDestinationIpRangesEl {
    pub fn build(self) -> VmwareengineExternalAccessRuleDestinationIpRangesEl {
        VmwareengineExternalAccessRuleDestinationIpRangesEl {
            external_address: core::default::Default::default(),
            ip_address_range: core::default::Default::default(),
        }
    }
}
pub struct VmwareengineExternalAccessRuleDestinationIpRangesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineExternalAccessRuleDestinationIpRangesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VmwareengineExternalAccessRuleDestinationIpRangesElRef {
        VmwareengineExternalAccessRuleDestinationIpRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineExternalAccessRuleDestinationIpRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `external_address` after provisioning.\nThe name of an 'ExternalAddress' resource."]
    pub fn external_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_address", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address_range` after provisioning.\nAn IP address range in the CIDR format."]
    pub fn ip_address_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_range", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VmwareengineExternalAccessRuleSourceIpRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_range: Option<PrimField<String>>,
}
impl VmwareengineExternalAccessRuleSourceIpRangesEl {
    #[doc = "Set the field `ip_address`.\nA single IP address."]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address_range`.\nAn IP address range in the CIDR format."]
    pub fn set_ip_address_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address_range = Some(v.into());
        self
    }
}
impl ToListMappable for VmwareengineExternalAccessRuleSourceIpRangesEl {
    type O = BlockAssignable<VmwareengineExternalAccessRuleSourceIpRangesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineExternalAccessRuleSourceIpRangesEl {}
impl BuildVmwareengineExternalAccessRuleSourceIpRangesEl {
    pub fn build(self) -> VmwareengineExternalAccessRuleSourceIpRangesEl {
        VmwareengineExternalAccessRuleSourceIpRangesEl {
            ip_address: core::default::Default::default(),
            ip_address_range: core::default::Default::default(),
        }
    }
}
pub struct VmwareengineExternalAccessRuleSourceIpRangesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineExternalAccessRuleSourceIpRangesElRef {
    fn new(shared: StackShared, base: String) -> VmwareengineExternalAccessRuleSourceIpRangesElRef {
        VmwareengineExternalAccessRuleSourceIpRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineExternalAccessRuleSourceIpRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nA single IP address."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_address_range` after provisioning.\nAn IP address range in the CIDR format."]
    pub fn ip_address_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_range", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VmwareengineExternalAccessRuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VmwareengineExternalAccessRuleTimeoutsEl {
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
impl ToListMappable for VmwareengineExternalAccessRuleTimeoutsEl {
    type O = BlockAssignable<VmwareengineExternalAccessRuleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVmwareengineExternalAccessRuleTimeoutsEl {}
impl BuildVmwareengineExternalAccessRuleTimeoutsEl {
    pub fn build(self) -> VmwareengineExternalAccessRuleTimeoutsEl {
        VmwareengineExternalAccessRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VmwareengineExternalAccessRuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VmwareengineExternalAccessRuleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VmwareengineExternalAccessRuleTimeoutsElRef {
        VmwareengineExternalAccessRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VmwareengineExternalAccessRuleTimeoutsElRef {
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
struct VmwareengineExternalAccessRuleDynamic {
    destination_ip_ranges:
        Option<DynamicBlock<VmwareengineExternalAccessRuleDestinationIpRangesEl>>,
    source_ip_ranges: Option<DynamicBlock<VmwareengineExternalAccessRuleSourceIpRangesEl>>,
}
