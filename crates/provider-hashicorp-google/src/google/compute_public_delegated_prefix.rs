use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputePublicDelegatedPrefixData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allocatable_prefix_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    ip_cidr_range: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_live_migration: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    name: PrimField<String>,
    parent_prefix: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    region: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputePublicDelegatedPrefixTimeoutsEl>,
}
struct ComputePublicDelegatedPrefix_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputePublicDelegatedPrefixData>,
}
#[derive(Clone)]
pub struct ComputePublicDelegatedPrefix(Rc<ComputePublicDelegatedPrefix_>);
impl ComputePublicDelegatedPrefix {
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
    #[doc = "Set the field `allocatable_prefix_length`.\nThe allocatable prefix length supported by this public delegated prefix. This field is optional and cannot be set for prefixes in DELEGATION mode. It cannot be set for IPv4 prefixes either, and it always defaults to 32."]
    pub fn set_allocatable_prefix_length(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().allocatable_prefix_length = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `is_live_migration`.\nIf true, the prefix will be live migrated."]
    pub fn set_is_live_migration(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_live_migration = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\nSpecifies the mode of this IPv6 PDP. MODE must be one of:\n  * DELEGATION\n  * EXTERNAL_IPV6_FORWARDING_RULE_CREATION\n  * EXTERNAL_IPV6_SUBNETWORK_CREATION\n  * INTERNAL_IPV6_SUBNETWORK_CREATION Possible values: [\"DELEGATION\", \"EXTERNAL_IPV6_FORWARDING_RULE_CREATION\", \"EXTERNAL_IPV6_SUBNETWORK_CREATION\", \"INTERNAL_IPV6_SUBNETWORK_CREATION\"]"]
    pub fn set_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mode = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputePublicDelegatedPrefixTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `allocatable_prefix_length` after provisioning.\nThe allocatable prefix length supported by this public delegated prefix. This field is optional and cannot be set for prefixes in DELEGATION mode. It cannot be set for IPv4 prefixes either, and it always defaults to 32."]
    pub fn allocatable_prefix_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocatable_prefix_length", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_enhanced_ipv4_allocation` after provisioning.\nWhether this PublicDelegatedPrefix supports enhanced IPv4 allocations.\nApplicable for IPv4 PDPs only."]
    pub fn enable_enhanced_ipv4_allocation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_enhanced_ipv4_allocation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe IP address range, in CIDR format, represented by this public delegated prefix."]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_access_type` after provisioning.\nThe internet access type for IPv6 Public Delegated Prefixes. Inherited\nfrom parent prefix and can be one of following:\n  * EXTERNAL: The prefix will be announced to the internet. All children\n  PDPs will have access type as EXTERNAL.\n  * INTERNAL: The prefix won’t be announced to the internet. Prefix will\n  be used privately within Google Cloud. All children PDPs will have\n  access type as INTERNAL."]
    pub fn ipv6_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_access_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_live_migration` after provisioning.\nIf true, the prefix will be live migrated."]
    pub fn is_live_migration(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_live_migration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nSpecifies the mode of this IPv6 PDP. MODE must be one of:\n  * DELEGATION\n  * EXTERNAL_IPV6_FORWARDING_RULE_CREATION\n  * EXTERNAL_IPV6_SUBNETWORK_CREATION\n  * INTERNAL_IPV6_SUBNETWORK_CREATION Possible values: [\"DELEGATION\", \"EXTERNAL_IPV6_FORWARDING_RULE_CREATION\", \"EXTERNAL_IPV6_SUBNETWORK_CREATION\", \"INTERNAL_IPV6_SUBNETWORK_CREATION\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent_prefix` after provisioning.\nThe URL of parent prefix. Either PublicAdvertisedPrefix or PublicDelegatedPrefix."]
    pub fn parent_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_delegated_sub_prefixs` after provisioning.\nList of sub public delegated fixes for BYO IP functionality.\nEach item in this array represents a sub prefix that can be\nused to create addresses or further allocations."]
    pub fn public_delegated_sub_prefixs(
        &self,
    ) -> ListRef<ComputePublicDelegatedPrefixPublicDelegatedSubPrefixsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_delegated_sub_prefixs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nA region where the prefix will reside."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputePublicDelegatedPrefixTimeoutsElRef {
        ComputePublicDelegatedPrefixTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputePublicDelegatedPrefix {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputePublicDelegatedPrefix {}
impl ToListMappable for ComputePublicDelegatedPrefix {
    type O = ListRef<ComputePublicDelegatedPrefixRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputePublicDelegatedPrefix_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_public_delegated_prefix".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputePublicDelegatedPrefix {
    pub tf_id: String,
    #[doc = "The IP address range, in CIDR format, represented by this public delegated prefix."]
    pub ip_cidr_range: PrimField<String>,
    #[doc = "Name of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc = "The URL of parent prefix. Either PublicAdvertisedPrefix or PublicDelegatedPrefix."]
    pub parent_prefix: PrimField<String>,
    #[doc = "A region where the prefix will reside."]
    pub region: PrimField<String>,
}
impl BuildComputePublicDelegatedPrefix {
    pub fn build(self, stack: &mut Stack) -> ComputePublicDelegatedPrefix {
        let out = ComputePublicDelegatedPrefix(Rc::new(ComputePublicDelegatedPrefix_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputePublicDelegatedPrefixData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allocatable_prefix_length: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_cidr_range: self.ip_cidr_range,
                is_live_migration: core::default::Default::default(),
                mode: core::default::Default::default(),
                name: self.name,
                parent_prefix: self.parent_prefix,
                project: core::default::Default::default(),
                region: self.region,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputePublicDelegatedPrefixRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputePublicDelegatedPrefixRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputePublicDelegatedPrefixRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allocatable_prefix_length` after provisioning.\nThe allocatable prefix length supported by this public delegated prefix. This field is optional and cannot be set for prefixes in DELEGATION mode. It cannot be set for IPv4 prefixes either, and it always defaults to 32."]
    pub fn allocatable_prefix_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocatable_prefix_length", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_enhanced_ipv4_allocation` after provisioning.\nWhether this PublicDelegatedPrefix supports enhanced IPv4 allocations.\nApplicable for IPv4 PDPs only."]
    pub fn enable_enhanced_ipv4_allocation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_enhanced_ipv4_allocation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe IP address range, in CIDR format, represented by this public delegated prefix."]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_access_type` after provisioning.\nThe internet access type for IPv6 Public Delegated Prefixes. Inherited\nfrom parent prefix and can be one of following:\n  * EXTERNAL: The prefix will be announced to the internet. All children\n  PDPs will have access type as EXTERNAL.\n  * INTERNAL: The prefix won’t be announced to the internet. Prefix will\n  be used privately within Google Cloud. All children PDPs will have\n  access type as INTERNAL."]
    pub fn ipv6_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_access_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `is_live_migration` after provisioning.\nIf true, the prefix will be live migrated."]
    pub fn is_live_migration(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_live_migration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nSpecifies the mode of this IPv6 PDP. MODE must be one of:\n  * DELEGATION\n  * EXTERNAL_IPV6_FORWARDING_RULE_CREATION\n  * EXTERNAL_IPV6_SUBNETWORK_CREATION\n  * INTERNAL_IPV6_SUBNETWORK_CREATION Possible values: [\"DELEGATION\", \"EXTERNAL_IPV6_FORWARDING_RULE_CREATION\", \"EXTERNAL_IPV6_SUBNETWORK_CREATION\", \"INTERNAL_IPV6_SUBNETWORK_CREATION\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent_prefix` after provisioning.\nThe URL of parent prefix. Either PublicAdvertisedPrefix or PublicDelegatedPrefix."]
    pub fn parent_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_delegated_sub_prefixs` after provisioning.\nList of sub public delegated fixes for BYO IP functionality.\nEach item in this array represents a sub prefix that can be\nused to create addresses or further allocations."]
    pub fn public_delegated_sub_prefixs(
        &self,
    ) -> ListRef<ComputePublicDelegatedPrefixPublicDelegatedSubPrefixsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_delegated_sub_prefixs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nA region where the prefix will reside."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputePublicDelegatedPrefixTimeoutsElRef {
        ComputePublicDelegatedPrefixTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputePublicDelegatedPrefixPublicDelegatedSubPrefixsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allocatable_prefix_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delegatee_project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_enhanced_ipv4_allocation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_cidr_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_access_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_address: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}
impl ComputePublicDelegatedPrefixPublicDelegatedSubPrefixsEl {
    #[doc = "Set the field `allocatable_prefix_length`.\n"]
    pub fn set_allocatable_prefix_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.allocatable_prefix_length = Some(v.into());
        self
    }
    #[doc = "Set the field `delegatee_project`.\n"]
    pub fn set_delegatee_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delegatee_project = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_enhanced_ipv4_allocation`.\n"]
    pub fn set_enable_enhanced_ipv4_allocation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_enhanced_ipv4_allocation = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_cidr_range`.\n"]
    pub fn set_ip_cidr_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_cidr_range = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv6_access_type`.\n"]
    pub fn set_ipv6_access_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_access_type = Some(v.into());
        self
    }
    #[doc = "Set the field `is_address`.\n"]
    pub fn set_is_address(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_address = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for ComputePublicDelegatedPrefixPublicDelegatedSubPrefixsEl {
    type O = BlockAssignable<ComputePublicDelegatedPrefixPublicDelegatedSubPrefixsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputePublicDelegatedPrefixPublicDelegatedSubPrefixsEl {}
impl BuildComputePublicDelegatedPrefixPublicDelegatedSubPrefixsEl {
    pub fn build(self) -> ComputePublicDelegatedPrefixPublicDelegatedSubPrefixsEl {
        ComputePublicDelegatedPrefixPublicDelegatedSubPrefixsEl {
            allocatable_prefix_length: core::default::Default::default(),
            delegatee_project: core::default::Default::default(),
            description: core::default::Default::default(),
            enable_enhanced_ipv4_allocation: core::default::Default::default(),
            ip_cidr_range: core::default::Default::default(),
            ipv6_access_type: core::default::Default::default(),
            is_address: core::default::Default::default(),
            mode: core::default::Default::default(),
            name: core::default::Default::default(),
            region: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct ComputePublicDelegatedPrefixPublicDelegatedSubPrefixsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputePublicDelegatedPrefixPublicDelegatedSubPrefixsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputePublicDelegatedPrefixPublicDelegatedSubPrefixsElRef {
        ComputePublicDelegatedPrefixPublicDelegatedSubPrefixsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputePublicDelegatedPrefixPublicDelegatedSubPrefixsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allocatable_prefix_length` after provisioning.\n"]
    pub fn allocatable_prefix_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocatable_prefix_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `delegatee_project` after provisioning.\n"]
    pub fn delegatee_project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delegatee_project", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `enable_enhanced_ipv4_allocation` after provisioning.\n"]
    pub fn enable_enhanced_ipv4_allocation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_enhanced_ipv4_allocation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\n"]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_access_type` after provisioning.\n"]
    pub fn ipv6_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_access_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_address` after provisioning.\n"]
    pub fn is_address(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_address", self.base))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputePublicDelegatedPrefixTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl ComputePublicDelegatedPrefixTimeoutsEl {
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
}
impl ToListMappable for ComputePublicDelegatedPrefixTimeoutsEl {
    type O = BlockAssignable<ComputePublicDelegatedPrefixTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputePublicDelegatedPrefixTimeoutsEl {}
impl BuildComputePublicDelegatedPrefixTimeoutsEl {
    pub fn build(self) -> ComputePublicDelegatedPrefixTimeoutsEl {
        ComputePublicDelegatedPrefixTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct ComputePublicDelegatedPrefixTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputePublicDelegatedPrefixTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputePublicDelegatedPrefixTimeoutsElRef {
        ComputePublicDelegatedPrefixTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputePublicDelegatedPrefixTimeoutsElRef {
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
}
