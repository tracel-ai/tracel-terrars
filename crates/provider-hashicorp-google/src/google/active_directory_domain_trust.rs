use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ActiveDirectoryDomainTrustData {
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
    domain: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selective_authentication: Option<PrimField<bool>>,
    target_dns_ip_addresses: SetField<PrimField<String>>,
    target_domain_name: PrimField<String>,
    trust_direction: PrimField<String>,
    trust_handshake_secret: PrimField<String>,
    trust_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ActiveDirectoryDomainTrustTimeoutsEl>,
}
struct ActiveDirectoryDomainTrust_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ActiveDirectoryDomainTrustData>,
}
#[derive(Clone)]
pub struct ActiveDirectoryDomainTrust(Rc<ActiveDirectoryDomainTrust_>);
impl ActiveDirectoryDomainTrust {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `selective_authentication`.\nWhether the trusted side has forest/domain wide access or selective access to an approved set of resources."]
    pub fn set_selective_authentication(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().selective_authentication = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ActiveDirectoryDomainTrustTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe fully qualified domain name. e.g. mydomain.myorganization.com, with the restrictions\nof https://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `selective_authentication` after provisioning.\nWhether the trusted side has forest/domain wide access or selective access to an approved set of resources."]
    pub fn selective_authentication(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.selective_authentication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_dns_ip_addresses` after provisioning.\nThe target DNS server IP addresses which can resolve the remote domain involved in the trust."]
    pub fn target_dns_ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.target_dns_ip_addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_domain_name` after provisioning.\nThe fully qualified target domain name which will be in trust with the current domain."]
    pub fn target_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trust_direction` after provisioning.\nThe trust direction, which decides if the current domain is trusted, trusting, or both. Possible values: [\"INBOUND\", \"OUTBOUND\", \"BIDIRECTIONAL\"]"]
    pub fn trust_direction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trust_direction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trust_handshake_secret` after provisioning.\nThe trust secret used for the handshake with the target domain. This will not be stored."]
    pub fn trust_handshake_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trust_handshake_secret", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trust_type` after provisioning.\nThe type of trust represented by the trust resource. Possible values: [\"FOREST\", \"EXTERNAL\"]"]
    pub fn trust_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trust_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ActiveDirectoryDomainTrustTimeoutsElRef {
        ActiveDirectoryDomainTrustTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ActiveDirectoryDomainTrust {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ActiveDirectoryDomainTrust {}
impl ToListMappable for ActiveDirectoryDomainTrust {
    type O = ListRef<ActiveDirectoryDomainTrustRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ActiveDirectoryDomainTrust_ {
    fn extract_resource_type(&self) -> String {
        "google_active_directory_domain_trust".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildActiveDirectoryDomainTrust {
    pub tf_id: String,
    #[doc = "The fully qualified domain name. e.g. mydomain.myorganization.com, with the restrictions\nof https://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains."]
    pub domain: PrimField<String>,
    #[doc = "The target DNS server IP addresses which can resolve the remote domain involved in the trust."]
    pub target_dns_ip_addresses: SetField<PrimField<String>>,
    #[doc = "The fully qualified target domain name which will be in trust with the current domain."]
    pub target_domain_name: PrimField<String>,
    #[doc = "The trust direction, which decides if the current domain is trusted, trusting, or both. Possible values: [\"INBOUND\", \"OUTBOUND\", \"BIDIRECTIONAL\"]"]
    pub trust_direction: PrimField<String>,
    #[doc = "The trust secret used for the handshake with the target domain. This will not be stored."]
    pub trust_handshake_secret: PrimField<String>,
    #[doc = "The type of trust represented by the trust resource. Possible values: [\"FOREST\", \"EXTERNAL\"]"]
    pub trust_type: PrimField<String>,
}
impl BuildActiveDirectoryDomainTrust {
    pub fn build(self, stack: &mut Stack) -> ActiveDirectoryDomainTrust {
        let out = ActiveDirectoryDomainTrust(Rc::new(ActiveDirectoryDomainTrust_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ActiveDirectoryDomainTrustData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                domain: self.domain,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                selective_authentication: core::default::Default::default(),
                target_dns_ip_addresses: self.target_dns_ip_addresses,
                target_domain_name: self.target_domain_name,
                trust_direction: self.trust_direction,
                trust_handshake_secret: self.trust_handshake_secret,
                trust_type: self.trust_type,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ActiveDirectoryDomainTrustRef {
    shared: StackShared,
    base: String,
}
impl Ref for ActiveDirectoryDomainTrustRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ActiveDirectoryDomainTrustRef {
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
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nThe fully qualified domain name. e.g. mydomain.myorganization.com, with the restrictions\nof https://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `selective_authentication` after provisioning.\nWhether the trusted side has forest/domain wide access or selective access to an approved set of resources."]
    pub fn selective_authentication(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.selective_authentication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_dns_ip_addresses` after provisioning.\nThe target DNS server IP addresses which can resolve the remote domain involved in the trust."]
    pub fn target_dns_ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.target_dns_ip_addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_domain_name` after provisioning.\nThe fully qualified target domain name which will be in trust with the current domain."]
    pub fn target_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trust_direction` after provisioning.\nThe trust direction, which decides if the current domain is trusted, trusting, or both. Possible values: [\"INBOUND\", \"OUTBOUND\", \"BIDIRECTIONAL\"]"]
    pub fn trust_direction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trust_direction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trust_handshake_secret` after provisioning.\nThe trust secret used for the handshake with the target domain. This will not be stored."]
    pub fn trust_handshake_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trust_handshake_secret", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trust_type` after provisioning.\nThe type of trust represented by the trust resource. Possible values: [\"FOREST\", \"EXTERNAL\"]"]
    pub fn trust_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trust_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ActiveDirectoryDomainTrustTimeoutsElRef {
        ActiveDirectoryDomainTrustTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ActiveDirectoryDomainTrustTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ActiveDirectoryDomainTrustTimeoutsEl {
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
impl ToListMappable for ActiveDirectoryDomainTrustTimeoutsEl {
    type O = BlockAssignable<ActiveDirectoryDomainTrustTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildActiveDirectoryDomainTrustTimeoutsEl {}
impl BuildActiveDirectoryDomainTrustTimeoutsEl {
    pub fn build(self) -> ActiveDirectoryDomainTrustTimeoutsEl {
        ActiveDirectoryDomainTrustTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ActiveDirectoryDomainTrustTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ActiveDirectoryDomainTrustTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ActiveDirectoryDomainTrustTimeoutsElRef {
        ActiveDirectoryDomainTrustTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ActiveDirectoryDomainTrustTimeoutsElRef {
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
