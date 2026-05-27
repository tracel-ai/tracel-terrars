use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeOrganizationSecurityPolicyData {
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
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_options_config: Option<Vec<ComputeOrganizationSecurityPolicyAdvancedOptionsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeOrganizationSecurityPolicyTimeoutsEl>,
    dynamic: ComputeOrganizationSecurityPolicyDynamic,
}
struct ComputeOrganizationSecurityPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeOrganizationSecurityPolicyData>,
}
#[derive(Clone)]
pub struct ComputeOrganizationSecurityPolicy(Rc<ComputeOrganizationSecurityPolicy_>);
impl ComputeOrganizationSecurityPolicy {
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
    #[doc = "Set the field `description`.\nA textual description for the organization security policy."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nUser-provided name of the organization security policy. The name should be unique in the organization in which the security policy is created. This should only be used when SecurityPolicyType is FIREWALL."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `short_name`.\nUser-provided name of the organization security policy. The name should be unique in the organization in which the security policy is created. This should only be used when SecurityPolicyType is CLOUD_ARMOR."]
    pub fn set_short_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().short_name = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThe type indicates the intended use of the security policy. This field can be set only at resource creation time.\n\n**NOTE** : 'FIREWALL' type is deprecated and will be removed in a future major release. Please use 'google_compute_firewall_policy' instead.\" Possible values: [\"FIREWALL\", \"CLOUD_ARMOR\", \"CLOUD_ARMOR_EDGE\", \"CLOUD_ARMOR_INTERNAL_SERVICE\", \"CLOUD_ARMOR_NETWORK\"]"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `advanced_options_config`.\n"]
    pub fn set_advanced_options_config(
        self,
        v: impl Into<BlockAssignable<ComputeOrganizationSecurityPolicyAdvancedOptionsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_options_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_options_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeOrganizationSecurityPolicyTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA textual description for the organization security policy."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-provided name of the organization security policy. The name should be unique in the organization in which the security policy is created. This should only be used when SecurityPolicyType is FIREWALL."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. This field is used internally during\nupdates of this resource."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of this OrganizationSecurityPolicy in the Cloud Resource Hierarchy.\nFormat: organizations/{organization_id} or folders/{folder_id}"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `short_name` after provisioning.\nUser-provided name of the organization security policy. The name should be unique in the organization in which the security policy is created. This should only be used when SecurityPolicyType is CLOUD_ARMOR."]
    pub fn short_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.short_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type indicates the intended use of the security policy. This field can be set only at resource creation time.\n\n**NOTE** : 'FIREWALL' type is deprecated and will be removed in a future major release. Please use 'google_compute_firewall_policy' instead.\" Possible values: [\"FIREWALL\", \"CLOUD_ARMOR\", \"CLOUD_ARMOR_EDGE\", \"CLOUD_ARMOR_INTERNAL_SERVICE\", \"CLOUD_ARMOR_NETWORK\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_options_config` after provisioning.\n"]
    pub fn advanced_options_config(
        &self,
    ) -> ListRef<ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_options_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeOrganizationSecurityPolicyTimeoutsElRef {
        ComputeOrganizationSecurityPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeOrganizationSecurityPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeOrganizationSecurityPolicy {}
impl ToListMappable for ComputeOrganizationSecurityPolicy {
    type O = ListRef<ComputeOrganizationSecurityPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeOrganizationSecurityPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_organization_security_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeOrganizationSecurityPolicy {
    pub tf_id: String,
    #[doc = "The parent of this OrganizationSecurityPolicy in the Cloud Resource Hierarchy.\nFormat: organizations/{organization_id} or folders/{folder_id}"]
    pub parent: PrimField<String>,
}
impl BuildComputeOrganizationSecurityPolicy {
    pub fn build(self, stack: &mut Stack) -> ComputeOrganizationSecurityPolicy {
        let out = ComputeOrganizationSecurityPolicy(Rc::new(ComputeOrganizationSecurityPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeOrganizationSecurityPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                parent: self.parent,
                short_name: core::default::Default::default(),
                type_: core::default::Default::default(),
                advanced_options_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeOrganizationSecurityPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeOrganizationSecurityPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeOrganizationSecurityPolicyRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA textual description for the organization security policy."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-provided name of the organization security policy. The name should be unique in the organization in which the security policy is created. This should only be used when SecurityPolicyType is FIREWALL."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. This field is used internally during\nupdates of this resource."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of this OrganizationSecurityPolicy in the Cloud Resource Hierarchy.\nFormat: organizations/{organization_id} or folders/{folder_id}"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `short_name` after provisioning.\nUser-provided name of the organization security policy. The name should be unique in the organization in which the security policy is created. This should only be used when SecurityPolicyType is CLOUD_ARMOR."]
    pub fn short_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.short_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type indicates the intended use of the security policy. This field can be set only at resource creation time.\n\n**NOTE** : 'FIREWALL' type is deprecated and will be removed in a future major release. Please use 'google_compute_firewall_policy' instead.\" Possible values: [\"FIREWALL\", \"CLOUD_ARMOR\", \"CLOUD_ARMOR_EDGE\", \"CLOUD_ARMOR_INTERNAL_SERVICE\", \"CLOUD_ARMOR_NETWORK\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_options_config` after provisioning.\n"]
    pub fn advanced_options_config(
        &self,
    ) -> ListRef<ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_options_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeOrganizationSecurityPolicyTimeoutsElRef {
        ComputeOrganizationSecurityPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    content_types: SetField<PrimField<String>>,
}
impl ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {}
impl ToListMappable for ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    type O =
        BlockAssignable<ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeOrganizationSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    #[doc = "A list of content types to be parsed as JSON."]
    pub content_types: SetField<PrimField<String>>,
}
impl BuildComputeOrganizationSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    pub fn build(
        self,
    ) -> ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
        ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
            content_types: self.content_types,
        }
    }
}
pub struct ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
        ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content_types` after provisioning.\nA list of content types to be parsed as JSON."]
    pub fn content_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.content_types", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElDynamic {
    json_custom_config: Option<
        DynamicBlock<ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyAdvancedOptionsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    json_parsing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_body_inspection_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_ip_request_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_custom_config:
        Option<Vec<ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>>,
    dynamic: ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElDynamic,
}
impl ComputeOrganizationSecurityPolicyAdvancedOptionsConfigEl {
    #[doc = "Set the field `json_parsing`.\nJSON body parsing. Supported values include: \"DISABLED\", \"STANDARD\", \"STANDARD_WITH_GRAPHQL\". Possible values: [\"DISABLED\", \"STANDARD\", \"STANDARD_WITH_GRAPHQL\"]"]
    pub fn set_json_parsing(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.json_parsing = Some(v.into());
        self
    }
    #[doc = "Set the field `log_level`.\nLogging level. Supported values include: \"NORMAL\", \"VERBOSE\". Possible values: [\"NORMAL\", \"VERBOSE\"]"]
    pub fn set_log_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_level = Some(v.into());
        self
    }
    #[doc = "Set the field `request_body_inspection_size`.\nThe maximum request size chosen by the customer with Waf enabled. Values supported are \"8KB\", \"16KB\", \"32KB\", \"48KB\" and \"64KB\".\nValues are case insensitive. Possible values: [\"8KB\", \"16KB\", \"32KB\", \"48KB\", \"64KB\"]"]
    pub fn set_request_body_inspection_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_body_inspection_size = Some(v.into());
        self
    }
    #[doc = "Set the field `user_ip_request_headers`.\nAn optional list of case-insensitive request header names to use for resolving the client source IP address."]
    pub fn set_user_ip_request_headers(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.user_ip_request_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `json_custom_config`.\n"]
    pub fn set_json_custom_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.json_custom_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.json_custom_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeOrganizationSecurityPolicyAdvancedOptionsConfigEl {
    type O = BlockAssignable<ComputeOrganizationSecurityPolicyAdvancedOptionsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeOrganizationSecurityPolicyAdvancedOptionsConfigEl {}
impl BuildComputeOrganizationSecurityPolicyAdvancedOptionsConfigEl {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyAdvancedOptionsConfigEl {
        ComputeOrganizationSecurityPolicyAdvancedOptionsConfigEl {
            json_parsing: core::default::Default::default(),
            log_level: core::default::Default::default(),
            request_body_inspection_size: core::default::Default::default(),
            user_ip_request_headers: core::default::Default::default(),
            json_custom_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElRef {
        ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `json_parsing` after provisioning.\nJSON body parsing. Supported values include: \"DISABLED\", \"STANDARD\", \"STANDARD_WITH_GRAPHQL\". Possible values: [\"DISABLED\", \"STANDARD\", \"STANDARD_WITH_GRAPHQL\"]"]
    pub fn json_parsing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.json_parsing", self.base))
    }
    #[doc = "Get a reference to the value of field `log_level` after provisioning.\nLogging level. Supported values include: \"NORMAL\", \"VERBOSE\". Possible values: [\"NORMAL\", \"VERBOSE\"]"]
    pub fn log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }
    #[doc = "Get a reference to the value of field `request_body_inspection_size` after provisioning.\nThe maximum request size chosen by the customer with Waf enabled. Values supported are \"8KB\", \"16KB\", \"32KB\", \"48KB\" and \"64KB\".\nValues are case insensitive. Possible values: [\"8KB\", \"16KB\", \"32KB\", \"48KB\", \"64KB\"]"]
    pub fn request_body_inspection_size(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_body_inspection_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_ip_request_headers` after provisioning.\nAn optional list of case-insensitive request header names to use for resolving the client source IP address."]
    pub fn user_ip_request_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.user_ip_request_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `json_custom_config` after provisioning.\n"]
    pub fn json_custom_config(
        &self,
    ) -> ListRef<ComputeOrganizationSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.json_custom_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeOrganizationSecurityPolicyTimeoutsEl {
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
impl ToListMappable for ComputeOrganizationSecurityPolicyTimeoutsEl {
    type O = BlockAssignable<ComputeOrganizationSecurityPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeOrganizationSecurityPolicyTimeoutsEl {}
impl BuildComputeOrganizationSecurityPolicyTimeoutsEl {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyTimeoutsEl {
        ComputeOrganizationSecurityPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeOrganizationSecurityPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeOrganizationSecurityPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeOrganizationSecurityPolicyTimeoutsElRef {
        ComputeOrganizationSecurityPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeOrganizationSecurityPolicyTimeoutsElRef {
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
struct ComputeOrganizationSecurityPolicyDynamic {
    advanced_options_config:
        Option<DynamicBlock<ComputeOrganizationSecurityPolicyAdvancedOptionsConfigEl>>,
}
