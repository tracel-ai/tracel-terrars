use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DnsPolicyData {
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
    enable_inbound_forwarding: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alternative_name_server_config: Option<Vec<DnsPolicyAlternativeNameServerConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns64_config: Option<Vec<DnsPolicyDns64ConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    networks: Option<Vec<DnsPolicyNetworksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DnsPolicyTimeoutsEl>,
    dynamic: DnsPolicyDynamic,
}
struct DnsPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DnsPolicyData>,
}
#[derive(Clone)]
pub struct DnsPolicy(Rc<DnsPolicy_>);
impl DnsPolicy {
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
    #[doc = "Set the field `description`.\nA textual description field. Defaults to 'Managed by Terraform'."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_inbound_forwarding`.\nAllows networks bound to this policy to receive DNS queries sent\nby VMs or applications over VPN connections. When enabled, a\nvirtual IP address will be allocated from each of the sub-networks\nthat are bound to this policy."]
    pub fn set_enable_inbound_forwarding(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_inbound_forwarding = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_logging`.\nControls whether logging is enabled for the networks bound to this policy.\nDefaults to no logging if not set."]
    pub fn set_enable_logging(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_logging = Some(v.into());
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
    #[doc = "Set the field `alternative_name_server_config`.\n"]
    pub fn set_alternative_name_server_config(
        self,
        v: impl Into<BlockAssignable<DnsPolicyAlternativeNameServerConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alternative_name_server_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .alternative_name_server_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dns64_config`.\n"]
    pub fn set_dns64_config(self, v: impl Into<BlockAssignable<DnsPolicyDns64ConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dns64_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dns64_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `networks`.\n"]
    pub fn set_networks(self, v: impl Into<BlockAssignable<DnsPolicyNetworksEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().networks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.networks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DnsPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA textual description field. Defaults to 'Managed by Terraform'."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_inbound_forwarding` after provisioning.\nAllows networks bound to this policy to receive DNS queries sent\nby VMs or applications over VPN connections. When enabled, a\nvirtual IP address will be allocated from each of the sub-networks\nthat are bound to this policy."]
    pub fn enable_inbound_forwarding(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_inbound_forwarding", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_logging` after provisioning.\nControls whether logging is enabled for the networks bound to this policy.\nDefaults to no logging if not set."]
    pub fn enable_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUser assigned name for this policy."]
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
    #[doc = "Get a reference to the value of field `alternative_name_server_config` after provisioning.\n"]
    pub fn alternative_name_server_config(
        &self,
    ) -> ListRef<DnsPolicyAlternativeNameServerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.alternative_name_server_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns64_config` after provisioning.\n"]
    pub fn dns64_config(&self) -> ListRef<DnsPolicyDns64ConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns64_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsPolicyTimeoutsElRef {
        DnsPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DnsPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DnsPolicy {}
impl ToListMappable for DnsPolicy {
    type O = ListRef<DnsPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DnsPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_dns_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDnsPolicy {
    pub tf_id: String,
    #[doc = "User assigned name for this policy."]
    pub name: PrimField<String>,
}
impl BuildDnsPolicy {
    pub fn build(self, stack: &mut Stack) -> DnsPolicy {
        let out = DnsPolicy(Rc::new(DnsPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DnsPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                enable_inbound_forwarding: core::default::Default::default(),
                enable_logging: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                alternative_name_server_config: core::default::Default::default(),
                dns64_config: core::default::Default::default(),
                networks: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DnsPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DnsPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA textual description field. Defaults to 'Managed by Terraform'."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_inbound_forwarding` after provisioning.\nAllows networks bound to this policy to receive DNS queries sent\nby VMs or applications over VPN connections. When enabled, a\nvirtual IP address will be allocated from each of the sub-networks\nthat are bound to this policy."]
    pub fn enable_inbound_forwarding(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_inbound_forwarding", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_logging` after provisioning.\nControls whether logging is enabled for the networks bound to this policy.\nDefaults to no logging if not set."]
    pub fn enable_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUser assigned name for this policy."]
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
    #[doc = "Get a reference to the value of field `alternative_name_server_config` after provisioning.\n"]
    pub fn alternative_name_server_config(
        &self,
    ) -> ListRef<DnsPolicyAlternativeNameServerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.alternative_name_server_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns64_config` after provisioning.\n"]
    pub fn dns64_config(&self) -> ListRef<DnsPolicyDns64ConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns64_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsPolicyTimeoutsElRef {
        DnsPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DnsPolicyAlternativeNameServerConfigElTargetNameServersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarding_path: Option<PrimField<String>>,
    ipv4_address: PrimField<String>,
}
impl DnsPolicyAlternativeNameServerConfigElTargetNameServersEl {
    #[doc = "Set the field `forwarding_path`.\nForwarding path for this TargetNameServer. If unset or 'default' Cloud DNS will make forwarding\ndecision based on address ranges, i.e. RFC1918 addresses go to the VPC, Non-RFC1918 addresses go\nto the Internet. When set to 'private', Cloud DNS will always send queries through VPC for this target Possible values: [\"default\", \"private\"]"]
    pub fn set_forwarding_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.forwarding_path = Some(v.into());
        self
    }
}
impl ToListMappable for DnsPolicyAlternativeNameServerConfigElTargetNameServersEl {
    type O = BlockAssignable<DnsPolicyAlternativeNameServerConfigElTargetNameServersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsPolicyAlternativeNameServerConfigElTargetNameServersEl {
    #[doc = "IPv4 address to forward to."]
    pub ipv4_address: PrimField<String>,
}
impl BuildDnsPolicyAlternativeNameServerConfigElTargetNameServersEl {
    pub fn build(self) -> DnsPolicyAlternativeNameServerConfigElTargetNameServersEl {
        DnsPolicyAlternativeNameServerConfigElTargetNameServersEl {
            forwarding_path: core::default::Default::default(),
            ipv4_address: self.ipv4_address,
        }
    }
}
pub struct DnsPolicyAlternativeNameServerConfigElTargetNameServersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsPolicyAlternativeNameServerConfigElTargetNameServersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DnsPolicyAlternativeNameServerConfigElTargetNameServersElRef {
        DnsPolicyAlternativeNameServerConfigElTargetNameServersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsPolicyAlternativeNameServerConfigElTargetNameServersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `forwarding_path` after provisioning.\nForwarding path for this TargetNameServer. If unset or 'default' Cloud DNS will make forwarding\ndecision based on address ranges, i.e. RFC1918 addresses go to the VPC, Non-RFC1918 addresses go\nto the Internet. When set to 'private', Cloud DNS will always send queries through VPC for this target Possible values: [\"default\", \"private\"]"]
    pub fn forwarding_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.forwarding_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ipv4_address` after provisioning.\nIPv4 address to forward to."]
    pub fn ipv4_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.base))
    }
}
#[derive(Serialize, Default)]
struct DnsPolicyAlternativeNameServerConfigElDynamic {
    target_name_servers:
        Option<DynamicBlock<DnsPolicyAlternativeNameServerConfigElTargetNameServersEl>>,
}
#[derive(Serialize)]
pub struct DnsPolicyAlternativeNameServerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_name_servers: Option<Vec<DnsPolicyAlternativeNameServerConfigElTargetNameServersEl>>,
    dynamic: DnsPolicyAlternativeNameServerConfigElDynamic,
}
impl DnsPolicyAlternativeNameServerConfigEl {
    #[doc = "Set the field `target_name_servers`.\n"]
    pub fn set_target_name_servers(
        mut self,
        v: impl Into<BlockAssignable<DnsPolicyAlternativeNameServerConfigElTargetNameServersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_name_servers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_name_servers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DnsPolicyAlternativeNameServerConfigEl {
    type O = BlockAssignable<DnsPolicyAlternativeNameServerConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsPolicyAlternativeNameServerConfigEl {}
impl BuildDnsPolicyAlternativeNameServerConfigEl {
    pub fn build(self) -> DnsPolicyAlternativeNameServerConfigEl {
        DnsPolicyAlternativeNameServerConfigEl {
            target_name_servers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DnsPolicyAlternativeNameServerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsPolicyAlternativeNameServerConfigElRef {
    fn new(shared: StackShared, base: String) -> DnsPolicyAlternativeNameServerConfigElRef {
        DnsPolicyAlternativeNameServerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsPolicyAlternativeNameServerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DnsPolicyDns64ConfigElScopeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all_queries: Option<PrimField<bool>>,
}
impl DnsPolicyDns64ConfigElScopeEl {
    #[doc = "Set the field `all_queries`.\nControls whether DNS64 is enabled globally at the network level."]
    pub fn set_all_queries(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all_queries = Some(v.into());
        self
    }
}
impl ToListMappable for DnsPolicyDns64ConfigElScopeEl {
    type O = BlockAssignable<DnsPolicyDns64ConfigElScopeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsPolicyDns64ConfigElScopeEl {}
impl BuildDnsPolicyDns64ConfigElScopeEl {
    pub fn build(self) -> DnsPolicyDns64ConfigElScopeEl {
        DnsPolicyDns64ConfigElScopeEl {
            all_queries: core::default::Default::default(),
        }
    }
}
pub struct DnsPolicyDns64ConfigElScopeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsPolicyDns64ConfigElScopeElRef {
    fn new(shared: StackShared, base: String) -> DnsPolicyDns64ConfigElScopeElRef {
        DnsPolicyDns64ConfigElScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsPolicyDns64ConfigElScopeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all_queries` after provisioning.\nControls whether DNS64 is enabled globally at the network level."]
    pub fn all_queries(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.all_queries", self.base))
    }
}
#[derive(Serialize, Default)]
struct DnsPolicyDns64ConfigElDynamic {
    scope: Option<DynamicBlock<DnsPolicyDns64ConfigElScopeEl>>,
}
#[derive(Serialize)]
pub struct DnsPolicyDns64ConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<Vec<DnsPolicyDns64ConfigElScopeEl>>,
    dynamic: DnsPolicyDns64ConfigElDynamic,
}
impl DnsPolicyDns64ConfigEl {
    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(
        mut self,
        v: impl Into<BlockAssignable<DnsPolicyDns64ConfigElScopeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scope = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scope = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DnsPolicyDns64ConfigEl {
    type O = BlockAssignable<DnsPolicyDns64ConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsPolicyDns64ConfigEl {}
impl BuildDnsPolicyDns64ConfigEl {
    pub fn build(self) -> DnsPolicyDns64ConfigEl {
        DnsPolicyDns64ConfigEl {
            scope: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DnsPolicyDns64ConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsPolicyDns64ConfigElRef {
    fn new(shared: StackShared, base: String) -> DnsPolicyDns64ConfigElRef {
        DnsPolicyDns64ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsPolicyDns64ConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> ListRef<DnsPolicyDns64ConfigElScopeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}
#[derive(Serialize)]
pub struct DnsPolicyNetworksEl {
    network_url: PrimField<String>,
}
impl DnsPolicyNetworksEl {}
impl ToListMappable for DnsPolicyNetworksEl {
    type O = BlockAssignable<DnsPolicyNetworksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsPolicyNetworksEl {
    #[doc = "The id or fully qualified URL of the VPC network to forward queries to.\nThis should be formatted like 'projects/{project}/global/networks/{network}' or\n'https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}'"]
    pub network_url: PrimField<String>,
}
impl BuildDnsPolicyNetworksEl {
    pub fn build(self) -> DnsPolicyNetworksEl {
        DnsPolicyNetworksEl {
            network_url: self.network_url,
        }
    }
}
pub struct DnsPolicyNetworksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsPolicyNetworksElRef {
    fn new(shared: StackShared, base: String) -> DnsPolicyNetworksElRef {
        DnsPolicyNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsPolicyNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network_url` after provisioning.\nThe id or fully qualified URL of the VPC network to forward queries to.\nThis should be formatted like 'projects/{project}/global/networks/{network}' or\n'https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}'"]
    pub fn network_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }
}
#[derive(Serialize)]
pub struct DnsPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DnsPolicyTimeoutsEl {
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
impl ToListMappable for DnsPolicyTimeoutsEl {
    type O = BlockAssignable<DnsPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsPolicyTimeoutsEl {}
impl BuildDnsPolicyTimeoutsEl {
    pub fn build(self) -> DnsPolicyTimeoutsEl {
        DnsPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DnsPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DnsPolicyTimeoutsElRef {
        DnsPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsPolicyTimeoutsElRef {
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
struct DnsPolicyDynamic {
    alternative_name_server_config: Option<DynamicBlock<DnsPolicyAlternativeNameServerConfigEl>>,
    dns64_config: Option<DynamicBlock<DnsPolicyDns64ConfigEl>>,
    networks: Option<DynamicBlock<DnsPolicyNetworksEl>>,
}
