use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeRegionSecurityPolicyData {
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
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_options_config: Option<Vec<ComputeRegionSecurityPolicyAdvancedOptionsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ddos_protection_config: Option<Vec<ComputeRegionSecurityPolicyDdosProtectionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<ComputeRegionSecurityPolicyRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeRegionSecurityPolicyTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_defined_fields: Option<Vec<ComputeRegionSecurityPolicyUserDefinedFieldsEl>>,
    dynamic: ComputeRegionSecurityPolicyDynamic,
}
struct ComputeRegionSecurityPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeRegionSecurityPolicyData>,
}
#[derive(Clone)]
pub struct ComputeRegionSecurityPolicy(Rc<ComputeRegionSecurityPolicy_>);
impl ComputeRegionSecurityPolicy {
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
    #[doc = "Set the field `description`.\nAn optional description of this resource. Provide this property when you create the resource."]
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
    #[doc = "Set the field `region`.\nThe Region in which the created Region Security Policy should reside.\nIf it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThe type indicates the intended use of the security policy.\n- CLOUD_ARMOR: Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers.\n- CLOUD_ARMOR_EDGE: Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache.\n- CLOUD_ARMOR_NETWORK: Cloud Armor network policies can be configured to filter packets targeting network load balancing resources such as backend services, target pools, target instances, and instances with external IPs. They filter requests before the request is served from the application.\nThis field can be set only at resource creation time. Possible values: [\"CLOUD_ARMOR\", \"CLOUD_ARMOR_EDGE\", \"CLOUD_ARMOR_NETWORK\"]"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `advanced_options_config`.\n"]
    pub fn set_advanced_options_config(
        self,
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyAdvancedOptionsConfigEl>>,
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
    #[doc = "Set the field `ddos_protection_config`.\n"]
    pub fn set_ddos_protection_config(
        self,
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyDdosProtectionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ddos_protection_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ddos_protection_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        self,
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeRegionSecurityPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `user_defined_fields`.\n"]
    pub fn set_user_defined_fields(
        self,
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyUserDefinedFieldsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_defined_fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_defined_fields = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])? which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created Region Security Policy should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link_with_policy_id` after provisioning.\nServer-defined URL for this resource with the resource id."]
    pub fn self_link_with_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link_with_policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type indicates the intended use of the security policy.\n- CLOUD_ARMOR: Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers.\n- CLOUD_ARMOR_EDGE: Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache.\n- CLOUD_ARMOR_NETWORK: Cloud Armor network policies can be configured to filter packets targeting network load balancing resources such as backend services, target pools, target instances, and instances with external IPs. They filter requests before the request is served from the application.\nThis field can be set only at resource creation time. Possible values: [\"CLOUD_ARMOR\", \"CLOUD_ARMOR_EDGE\", \"CLOUD_ARMOR_NETWORK\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_options_config` after provisioning.\n"]
    pub fn advanced_options_config(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyAdvancedOptionsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_options_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ddos_protection_config` after provisioning.\n"]
    pub fn ddos_protection_config(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyDdosProtectionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ddos_protection_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<ComputeRegionSecurityPolicyRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionSecurityPolicyTimeoutsElRef {
        ComputeRegionSecurityPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_defined_fields` after provisioning.\n"]
    pub fn user_defined_fields(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyUserDefinedFieldsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_defined_fields", self.extract_ref()),
        )
    }
}
impl Referable for ComputeRegionSecurityPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeRegionSecurityPolicy {}
impl ToListMappable for ComputeRegionSecurityPolicy {
    type O = ListRef<ComputeRegionSecurityPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeRegionSecurityPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_region_security_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeRegionSecurityPolicy {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])? which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildComputeRegionSecurityPolicy {
    pub fn build(self, stack: &mut Stack) -> ComputeRegionSecurityPolicy {
        let out = ComputeRegionSecurityPolicy(Rc::new(ComputeRegionSecurityPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeRegionSecurityPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                type_: core::default::Default::default(),
                advanced_options_config: core::default::Default::default(),
                ddos_protection_config: core::default::Default::default(),
                rules: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                user_defined_fields: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeRegionSecurityPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeRegionSecurityPolicyRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])? which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created Region Security Policy should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link_with_policy_id` after provisioning.\nServer-defined URL for this resource with the resource id."]
    pub fn self_link_with_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link_with_policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type indicates the intended use of the security policy.\n- CLOUD_ARMOR: Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers.\n- CLOUD_ARMOR_EDGE: Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache.\n- CLOUD_ARMOR_NETWORK: Cloud Armor network policies can be configured to filter packets targeting network load balancing resources such as backend services, target pools, target instances, and instances with external IPs. They filter requests before the request is served from the application.\nThis field can be set only at resource creation time. Possible values: [\"CLOUD_ARMOR\", \"CLOUD_ARMOR_EDGE\", \"CLOUD_ARMOR_NETWORK\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_options_config` after provisioning.\n"]
    pub fn advanced_options_config(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyAdvancedOptionsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_options_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ddos_protection_config` after provisioning.\n"]
    pub fn ddos_protection_config(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyDdosProtectionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ddos_protection_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<ComputeRegionSecurityPolicyRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionSecurityPolicyTimeoutsElRef {
        ComputeRegionSecurityPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_defined_fields` after provisioning.\n"]
    pub fn user_defined_fields(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyUserDefinedFieldsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_defined_fields", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    content_types: SetField<PrimField<String>>,
}
impl ComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {}
impl ToListMappable for ComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    #[doc = "A list of custom Content-Type header values to apply the JSON parsing."]
    pub content_types: SetField<PrimField<String>>,
}
impl BuildComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
        ComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
            content_types: self.content_types,
        }
    }
}
pub struct ComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
        ComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content_types` after provisioning.\nA list of custom Content-Type header values to apply the JSON parsing."]
    pub fn content_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.content_types", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicyAdvancedOptionsConfigElDynamic {
    json_custom_config:
        Option<DynamicBlock<ComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyAdvancedOptionsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    json_parsing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_ip_request_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_custom_config:
        Option<Vec<ComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>>,
    dynamic: ComputeRegionSecurityPolicyAdvancedOptionsConfigElDynamic,
}
impl ComputeRegionSecurityPolicyAdvancedOptionsConfigEl {
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
    #[doc = "Set the field `user_ip_request_headers`.\nAn optional list of case-insensitive request header names to use for resolving the callers client IP address."]
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
            BlockAssignable<ComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>,
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
impl ToListMappable for ComputeRegionSecurityPolicyAdvancedOptionsConfigEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyAdvancedOptionsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyAdvancedOptionsConfigEl {}
impl BuildComputeRegionSecurityPolicyAdvancedOptionsConfigEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyAdvancedOptionsConfigEl {
        ComputeRegionSecurityPolicyAdvancedOptionsConfigEl {
            json_parsing: core::default::Default::default(),
            log_level: core::default::Default::default(),
            user_ip_request_headers: core::default::Default::default(),
            json_custom_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyAdvancedOptionsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyAdvancedOptionsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyAdvancedOptionsConfigElRef {
        ComputeRegionSecurityPolicyAdvancedOptionsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyAdvancedOptionsConfigElRef {
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
    #[doc = "Get a reference to the value of field `user_ip_request_headers` after provisioning.\nAn optional list of case-insensitive request header names to use for resolving the callers client IP address."]
    pub fn user_ip_request_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.user_ip_request_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `json_custom_config` after provisioning.\n"]
    pub fn json_custom_config(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.json_custom_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyDdosProtectionConfigEl {
    ddos_protection: PrimField<String>,
}
impl ComputeRegionSecurityPolicyDdosProtectionConfigEl {}
impl ToListMappable for ComputeRegionSecurityPolicyDdosProtectionConfigEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyDdosProtectionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyDdosProtectionConfigEl {
    #[doc = "Google Cloud Armor offers the following options to help protect systems against DDoS attacks:\n- STANDARD: basic always-on protection for network load balancers, protocol forwarding, or VMs with public IP addresses.\n- ADVANCED: additional protections for Managed Protection Plus subscribers who use network load balancers, protocol forwarding, or VMs with public IP addresses.\n- ADVANCED_PREVIEW: flag to enable the security policy in preview mode. Possible values: [\"ADVANCED\", \"ADVANCED_PREVIEW\", \"STANDARD\"]"]
    pub ddos_protection: PrimField<String>,
}
impl BuildComputeRegionSecurityPolicyDdosProtectionConfigEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyDdosProtectionConfigEl {
        ComputeRegionSecurityPolicyDdosProtectionConfigEl {
            ddos_protection: self.ddos_protection,
        }
    }
}
pub struct ComputeRegionSecurityPolicyDdosProtectionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyDdosProtectionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyDdosProtectionConfigElRef {
        ComputeRegionSecurityPolicyDdosProtectionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyDdosProtectionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ddos_protection` after provisioning.\nGoogle Cloud Armor offers the following options to help protect systems against DDoS attacks:\n- STANDARD: basic always-on protection for network load balancers, protocol forwarding, or VMs with public IP addresses.\n- ADVANCED: additional protections for Managed Protection Plus subscribers who use network load balancers, protocol forwarding, or VMs with public IP addresses.\n- ADVANCED_PREVIEW: flag to enable the security policy in preview mode. Possible values: [\"ADVANCED\", \"ADVANCED_PREVIEW\", \"STANDARD\"]"]
    pub fn ddos_protection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ddos_protection", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulesElMatchElConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    src_ip_ranges: Option<ListField<PrimField<String>>>,
}
impl ComputeRegionSecurityPolicyRulesElMatchElConfigEl {
    #[doc = "Set the field `src_ip_ranges`.\nCIDR IP address range. Maximum number of srcIpRanges allowed is 10."]
    pub fn set_src_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_ip_ranges = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionSecurityPolicyRulesElMatchElConfigEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRulesElMatchElConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulesElMatchElConfigEl {}
impl BuildComputeRegionSecurityPolicyRulesElMatchElConfigEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRulesElMatchElConfigEl {
        ComputeRegionSecurityPolicyRulesElMatchElConfigEl {
            src_ip_ranges: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulesElMatchElConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulesElMatchElConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulesElMatchElConfigElRef {
        ComputeRegionSecurityPolicyRulesElMatchElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulesElMatchElConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `src_ip_ranges` after provisioning.\nCIDR IP address range. Maximum number of srcIpRanges allowed is 10."]
    pub fn src_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_ip_ranges", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulesElMatchElExprEl {
    expression: PrimField<String>,
}
impl ComputeRegionSecurityPolicyRulesElMatchElExprEl {}
impl ToListMappable for ComputeRegionSecurityPolicyRulesElMatchElExprEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRulesElMatchElExprEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulesElMatchElExprEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
    pub expression: PrimField<String>,
}
impl BuildComputeRegionSecurityPolicyRulesElMatchElExprEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRulesElMatchElExprEl {
        ComputeRegionSecurityPolicyRulesElMatchElExprEl {
            expression: self.expression,
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulesElMatchElExprElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulesElMatchElExprElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulesElMatchElExprElRef {
        ComputeRegionSecurityPolicyRulesElMatchElExprElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulesElMatchElExprElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicyRulesElMatchElDynamic {
    config: Option<DynamicBlock<ComputeRegionSecurityPolicyRulesElMatchElConfigEl>>,
    expr: Option<DynamicBlock<ComputeRegionSecurityPolicyRulesElMatchElExprEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulesElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    versioned_expr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<ComputeRegionSecurityPolicyRulesElMatchElConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expr: Option<Vec<ComputeRegionSecurityPolicyRulesElMatchElExprEl>>,
    dynamic: ComputeRegionSecurityPolicyRulesElMatchElDynamic,
}
impl ComputeRegionSecurityPolicyRulesElMatchEl {
    #[doc = "Set the field `versioned_expr`.\nPreconfigured versioned expression. If this field is specified, config must also be specified.\nAvailable preconfigured expressions along with their requirements are: SRC_IPS_V1 - must specify the corresponding srcIpRange field in config. Possible values: [\"SRC_IPS_V1\"]"]
    pub fn set_versioned_expr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.versioned_expr = Some(v.into());
        self
    }
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyRulesElMatchElConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `expr`.\n"]
    pub fn set_expr(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyRulesElMatchElExprEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.expr = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.expr = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionSecurityPolicyRulesElMatchEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRulesElMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulesElMatchEl {}
impl BuildComputeRegionSecurityPolicyRulesElMatchEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRulesElMatchEl {
        ComputeRegionSecurityPolicyRulesElMatchEl {
            versioned_expr: core::default::Default::default(),
            config: core::default::Default::default(),
            expr: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulesElMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulesElMatchElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionSecurityPolicyRulesElMatchElRef {
        ComputeRegionSecurityPolicyRulesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulesElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `versioned_expr` after provisioning.\nPreconfigured versioned expression. If this field is specified, config must also be specified.\nAvailable preconfigured expressions along with their requirements are: SRC_IPS_V1 - must specify the corresponding srcIpRange field in config. Possible values: [\"SRC_IPS_V1\"]"]
    pub fn versioned_expr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.versioned_expr", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<ComputeRegionSecurityPolicyRulesElMatchElConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
    #[doc = "Get a reference to the value of field `expr` after provisioning.\n"]
    pub fn expr(&self) -> ListRef<ComputeRegionSecurityPolicyRulesElMatchElExprElRef> {
        ListRef::new(self.shared().clone(), format!("{}.expr", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl ComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl {
    #[doc = "Set the field `name`.\nName of the user-defined field, as given in the definition."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\nMatching values of the field. Each element can be a 32-bit unsigned decimal or hexadecimal (starting with \"0x\") number (e.g. \"64\") or range (e.g. \"0x400-0x7ff\")."]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl {}
impl BuildComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl {
        ComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl {
            name: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsElRef {
        ComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the user-defined field, as given in the definition."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\nMatching values of the field. Each element can be a 32-bit unsigned decimal or hexadecimal (starting with \"0x\") number (e.g. \"64\") or range (e.g. \"0x400-0x7ff\")."]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicyRulesElNetworkMatchElDynamic {
    user_defined_fields:
        Option<DynamicBlock<ComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulesElNetworkMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_ports: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_protocols: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_asns: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_ports: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_region_codes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_defined_fields:
        Option<Vec<ComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl>>,
    dynamic: ComputeRegionSecurityPolicyRulesElNetworkMatchElDynamic,
}
impl ComputeRegionSecurityPolicyRulesElNetworkMatchEl {
    #[doc = "Set the field `dest_ip_ranges`.\nDestination IPv4/IPv6 addresses or CIDR prefixes, in standard text format."]
    pub fn set_dest_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `dest_ports`.\nDestination port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. \"80\") or range (e.g. \"0-1023\")."]
    pub fn set_dest_ports(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_ports = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_protocols`.\nIPv4 protocol / IPv6 next header (after extension headers). Each element can be an 8-bit unsigned decimal number (e.g. \"6\"), range (e.g. \"253-254\"), or one of the following protocol names: \"tcp\", \"udp\", \"icmp\", \"esp\", \"ah\", \"ipip\", or \"sctp\"."]
    pub fn set_ip_protocols(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_protocols = Some(v.into());
        self
    }
    #[doc = "Set the field `src_asns`.\nBGP Autonomous System Number associated with the source IP address."]
    pub fn set_src_asns(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.src_asns = Some(v.into());
        self
    }
    #[doc = "Set the field `src_ip_ranges`.\nSource IPv4/IPv6 addresses or CIDR prefixes, in standard text format."]
    pub fn set_src_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `src_ports`.\nSource port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. \"80\") or range (e.g. \"0-1023\")."]
    pub fn set_src_ports(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_ports = Some(v.into());
        self
    }
    #[doc = "Set the field `src_region_codes`.\nTwo-letter ISO 3166-1 alpha-2 country code associated with the source IP address."]
    pub fn set_src_region_codes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_region_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `user_defined_fields`.\n"]
    pub fn set_user_defined_fields(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.user_defined_fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.user_defined_fields = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionSecurityPolicyRulesElNetworkMatchEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRulesElNetworkMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulesElNetworkMatchEl {}
impl BuildComputeRegionSecurityPolicyRulesElNetworkMatchEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRulesElNetworkMatchEl {
        ComputeRegionSecurityPolicyRulesElNetworkMatchEl {
            dest_ip_ranges: core::default::Default::default(),
            dest_ports: core::default::Default::default(),
            ip_protocols: core::default::Default::default(),
            src_asns: core::default::Default::default(),
            src_ip_ranges: core::default::Default::default(),
            src_ports: core::default::Default::default(),
            src_region_codes: core::default::Default::default(),
            user_defined_fields: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulesElNetworkMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulesElNetworkMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulesElNetworkMatchElRef {
        ComputeRegionSecurityPolicyRulesElNetworkMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulesElNetworkMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dest_ip_ranges` after provisioning.\nDestination IPv4/IPv6 addresses or CIDR prefixes, in standard text format."]
    pub fn dest_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dest_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dest_ports` after provisioning.\nDestination port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. \"80\") or range (e.g. \"0-1023\")."]
    pub fn dest_ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dest_ports", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_protocols` after provisioning.\nIPv4 protocol / IPv6 next header (after extension headers). Each element can be an 8-bit unsigned decimal number (e.g. \"6\"), range (e.g. \"253-254\"), or one of the following protocol names: \"tcp\", \"udp\", \"icmp\", \"esp\", \"ah\", \"ipip\", or \"sctp\"."]
    pub fn ip_protocols(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_protocols", self.base))
    }
    #[doc = "Get a reference to the value of field `src_asns` after provisioning.\nBGP Autonomous System Number associated with the source IP address."]
    pub fn src_asns(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.src_asns", self.base))
    }
    #[doc = "Get a reference to the value of field `src_ip_ranges` after provisioning.\nSource IPv4/IPv6 addresses or CIDR prefixes, in standard text format."]
    pub fn src_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `src_ports` after provisioning.\nSource port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. \"80\") or range (e.g. \"0-1023\")."]
    pub fn src_ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.src_ports", self.base))
    }
    #[doc = "Get a reference to the value of field `src_region_codes` after provisioning.\nTwo-letter ISO 3166-1 alpha-2 country code associated with the source IP address."]
    pub fn src_region_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_region_codes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_defined_fields` after provisioning.\n"]
    pub fn user_defined_fields(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_defined_fields", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl {
    operator: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl {
    #[doc = "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl
{
    type O = BlockAssignable<
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl
{
    #[doc = "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub operator: PrimField<String>,
}
impl BuildComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl {
    pub fn build(
        self,
    ) -> ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl {
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieElRef
    {
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl {
    operator: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl {
    #[doc = "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl
{
    type O = BlockAssignable<
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl
{
    #[doc = "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub operator: PrimField<String>,
}
impl BuildComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl {
    pub fn build(
        self,
    ) -> ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl {
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderElRef
    {
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl
{
    operator: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl {
    #[doc = "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl
{
    type O = BlockAssignable<
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl
{
    #[doc = "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub operator: PrimField<String>,
}
impl BuildComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl {
    pub fn build(
        self,
    ) -> ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl
    {
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef
    {
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef { shared : shared , base : base . to_string () , }
    }
}
impl ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl {
    operator: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl {
    #[doc = "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl
{
    type O = BlockAssignable<
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl {
    #[doc = "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub operator: PrimField<String>,
}
impl BuildComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl {
    pub fn build(
        self,
    ) -> ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl {
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriElRef {
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElDynamic { request_cookie : Option < DynamicBlock < ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl >> , request_header : Option < DynamicBlock < ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl >> , request_query_param : Option < DynamicBlock < ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl >> , request_uri : Option < DynamicBlock < ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl >> , }
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl { # [serde (skip_serializing_if = "Option::is_none")] target_rule_ids : Option < ListField < PrimField < String > > > , target_rule_set : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] request_cookie : Option < Vec < ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl > > , # [serde (skip_serializing_if = "Option::is_none")] request_header : Option < Vec < ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl > > , # [serde (skip_serializing_if = "Option::is_none")] request_query_param : Option < Vec < ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl > > , # [serde (skip_serializing_if = "Option::is_none")] request_uri : Option < Vec < ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl > > , dynamic : ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElDynamic , }
impl ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl {
    #[doc = "Set the field `target_rule_ids`.\nA list of target rule IDs under the WAF rule set to apply the preconfigured WAF exclusion.\nIf omitted, it refers to all the rule IDs under the WAF rule set."]
    pub fn set_target_rule_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.target_rule_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `request_cookie`.\n"]
    pub fn set_request_cookie(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_cookie = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_cookie = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `request_header`.\n"]
    pub fn set_request_header(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_header = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_header = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `request_query_param`.\n"]
    pub fn set_request_query_param(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_query_param = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_query_param = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `request_uri`.\n"]
    pub fn set_request_uri(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_uri = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_uri = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl {
    #[doc = "Target WAF rule set to apply the preconfigured WAF exclusion."]
    pub target_rule_set: PrimField<String>,
}
impl BuildComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl {
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl {
            target_rule_ids: core::default::Default::default(),
            target_rule_set: self.target_rule_set,
            request_cookie: core::default::Default::default(),
            request_header: core::default::Default::default(),
            request_query_param: core::default::Default::default(),
            request_uri: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRef {
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_rule_ids` after provisioning.\nA list of target rule IDs under the WAF rule set to apply the preconfigured WAF exclusion.\nIf omitted, it refers to all the rule IDs under the WAF rule set."]
    pub fn target_rule_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_rule_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_rule_set` after provisioning.\nTarget WAF rule set to apply the preconfigured WAF exclusion."]
    pub fn target_rule_set(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_rule_set", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_cookie` after provisioning.\n"]
    pub fn request_cookie(
        &self,
    ) -> ListRef<
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_cookie", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_header` after provisioning.\n"]
    pub fn request_header(
        &self,
    ) -> ListRef<
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_header", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_query_param` after provisioning.\n"]
    pub fn request_query_param(
        &self,
    ) -> ListRef<
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_query_param", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_uri` after provisioning.\n"]
    pub fn request_uri(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.request_uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElDynamic {
    exclusion:
        Option<DynamicBlock<ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion: Option<Vec<ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl>>,
    dynamic: ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElDynamic,
}
impl ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl {
    #[doc = "Set the field `exclusion`.\n"]
    pub fn set_exclusion(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclusion = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl {}
impl BuildComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl {
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl {
            exclusion: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElRef {
        ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exclusion` after provisioning.\n"]
    pub fn exclusion(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclusion", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_sec: Option<PrimField<f64>>,
}
impl ComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl {
    #[doc = "Set the field `count`.\nNumber of HTTP(S) requests for calculating the threshold."]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `interval_sec`.\nInterval over which the threshold is computed."]
    pub fn set_interval_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval_sec = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl {}
impl BuildComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl {
        ComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl {
            count: core::default::Default::default(),
            interval_sec: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdElRef {
        ComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\nNumber of HTTP(S) requests for calculating the threshold."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `interval_sec` after provisioning.\nInterval over which the threshold is computed."]
    pub fn interval_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_sec", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_type: Option<PrimField<String>>,
}
impl ComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl {
    #[doc = "Set the field `enforce_on_key_name`.\nRate limit key name applicable only for the following key types:\nHTTP_HEADER -- Name of the HTTP header whose value is taken as the key value.\nHTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn set_enforce_on_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_on_key_type`.\nDetermines the key to enforce the rateLimitThreshold on. Possible values are:\n* ALL: A single rate limit threshold is applied to all the requests matching this rule. This is the default value if \"enforceOnKeyConfigs\" is not configured.\n* IP: The source IP address of the request is the key. Each IP has this limit enforced separately.\n* HTTP_HEADER: The value of the HTTP header whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the header value. If no such header is present in the request, the key type defaults to ALL.\n* XFF_IP: The first IP address (i.e. the originating client IP address) specified in the list of IPs under X-Forwarded-For HTTP header. If no such header is present or the value is not a valid IP, the key defaults to the source IP address of the request i.e. key type IP.\n* HTTP_COOKIE: The value of the HTTP cookie whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the cookie value. If no such cookie is present in the request, the key type defaults to ALL.\n* HTTP_PATH: The URL path of the HTTP request. The key value is truncated to the first 128 bytes.\n* SNI: Server name indication in the TLS session of the HTTPS request. The key value is truncated to the first 128 bytes. The key type defaults to ALL on a HTTP session.\n* REGION_CODE: The country/region from which the request originates.\n* TLS_JA3_FINGERPRINT: JA3 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* TLS_JA4_FINGERPRINT: JA4 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* USER_IP: The IP address of the originating client, which is resolved based on \"userIpRequestHeaders\" configured with the security policy. If there is no \"userIpRequestHeaders\" configuration or an IP address cannot be resolved from it, the key type defaults to IP. Possible values: [\"ALL\", \"IP\", \"HTTP_HEADER\", \"XFF_IP\", \"HTTP_COOKIE\", \"HTTP_PATH\", \"SNI\", \"REGION_CODE\", \"TLS_JA3_FINGERPRINT\", \"TLS_JA4_FINGERPRINT\", \"USER_IP\"]"]
    pub fn set_enforce_on_key_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key_type = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl {
    type O =
        BlockAssignable<ComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl {}
impl BuildComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl {
    pub fn build(
        self,
    ) -> ComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl {
        ComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl {
            enforce_on_key_name: core::default::Default::default(),
            enforce_on_key_type: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsElRef {
        ComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enforce_on_key_name` after provisioning.\nRate limit key name applicable only for the following key types:\nHTTP_HEADER -- Name of the HTTP header whose value is taken as the key value.\nHTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn enforce_on_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_on_key_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_on_key_type` after provisioning.\nDetermines the key to enforce the rateLimitThreshold on. Possible values are:\n* ALL: A single rate limit threshold is applied to all the requests matching this rule. This is the default value if \"enforceOnKeyConfigs\" is not configured.\n* IP: The source IP address of the request is the key. Each IP has this limit enforced separately.\n* HTTP_HEADER: The value of the HTTP header whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the header value. If no such header is present in the request, the key type defaults to ALL.\n* XFF_IP: The first IP address (i.e. the originating client IP address) specified in the list of IPs under X-Forwarded-For HTTP header. If no such header is present or the value is not a valid IP, the key defaults to the source IP address of the request i.e. key type IP.\n* HTTP_COOKIE: The value of the HTTP cookie whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the cookie value. If no such cookie is present in the request, the key type defaults to ALL.\n* HTTP_PATH: The URL path of the HTTP request. The key value is truncated to the first 128 bytes.\n* SNI: Server name indication in the TLS session of the HTTPS request. The key value is truncated to the first 128 bytes. The key type defaults to ALL on a HTTP session.\n* REGION_CODE: The country/region from which the request originates.\n* TLS_JA3_FINGERPRINT: JA3 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* TLS_JA4_FINGERPRINT: JA4 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* USER_IP: The IP address of the originating client, which is resolved based on \"userIpRequestHeaders\" configured with the security policy. If there is no \"userIpRequestHeaders\" configuration or an IP address cannot be resolved from it, the key type defaults to IP. Possible values: [\"ALL\", \"IP\", \"HTTP_HEADER\", \"XFF_IP\", \"HTTP_COOKIE\", \"HTTP_PATH\", \"SNI\", \"REGION_CODE\", \"TLS_JA3_FINGERPRINT\", \"TLS_JA4_FINGERPRINT\", \"USER_IP\"]"]
    pub fn enforce_on_key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_on_key_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_sec: Option<PrimField<f64>>,
}
impl ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl {
    #[doc = "Set the field `count`.\nNumber of HTTP(S) requests for calculating the threshold."]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `interval_sec`.\nInterval over which the threshold is computed."]
    pub fn set_interval_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval_sec = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl {
    type O =
        BlockAssignable<ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl {}
impl BuildComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl {
        ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl {
            count: core::default::Default::default(),
            interval_sec: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdElRef {
        ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\nNumber of HTTP(S) requests for calculating the threshold."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `interval_sec` after provisioning.\nInterval over which the threshold is computed."]
    pub fn interval_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_sec", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicyRulesElRateLimitOptionsElDynamic {
    ban_threshold:
        Option<DynamicBlock<ComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl>>,
    enforce_on_key_configs: Option<
        DynamicBlock<ComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl>,
    >,
    rate_limit_threshold: Option<
        DynamicBlock<ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl>,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulesElRateLimitOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ban_duration_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conform_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exceed_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ban_threshold: Option<Vec<ComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_configs:
        Option<Vec<ComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_threshold:
        Option<Vec<ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl>>,
    dynamic: ComputeRegionSecurityPolicyRulesElRateLimitOptionsElDynamic,
}
impl ComputeRegionSecurityPolicyRulesElRateLimitOptionsEl {
    #[doc = "Set the field `ban_duration_sec`.\nCan only be specified if the action for the rule is \"rate_based_ban\".\nIf specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold."]
    pub fn set_ban_duration_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ban_duration_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `conform_action`.\nAction to take for requests that are under the configured rate limit threshold.\nValid option is \"allow\" only."]
    pub fn set_conform_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.conform_action = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_on_key`.\nDetermines the key to enforce the rateLimitThreshold on. Possible values are:\n* ALL: A single rate limit threshold is applied to all the requests matching this rule. This is the default value if \"enforceOnKey\" is not configured.\n* IP: The source IP address of the request is the key. Each IP has this limit enforced separately.\n* HTTP_HEADER: The value of the HTTP header whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the header value. If no such header is present in the request, the key type defaults to ALL.\n* XFF_IP: The first IP address (i.e. the originating client IP address) specified in the list of IPs under X-Forwarded-For HTTP header. If no such header is present or the value is not a valid IP, the key defaults to the source IP address of the request i.e. key type IP.\n* HTTP_COOKIE: The value of the HTTP cookie whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the cookie value. If no such cookie is present in the request, the key type defaults to ALL.\n* HTTP_PATH: The URL path of the HTTP request. The key value is truncated to the first 128 bytes.\n* SNI: Server name indication in the TLS session of the HTTPS request. The key value is truncated to the first 128 bytes. The key type defaults to ALL on a HTTP session.\n* REGION_CODE: The country/region from which the request originates.\n* TLS_JA3_FINGERPRINT: JA3 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* TLS_JA4_FINGERPRINT: JA4 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* USER_IP: The IP address of the originating client, which is resolved based on \"userIpRequestHeaders\" configured with the security policy. If there is no \"userIpRequestHeaders\" configuration or an IP address cannot be resolved from it, the key type defaults to IP. Possible values: [\"ALL\", \"IP\", \"HTTP_HEADER\", \"XFF_IP\", \"HTTP_COOKIE\", \"HTTP_PATH\", \"SNI\", \"REGION_CODE\", \"TLS_JA3_FINGERPRINT\", \"TLS_JA4_FINGERPRINT\", \"USER_IP\"]"]
    pub fn set_enforce_on_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_on_key_name`.\nRate limit key name applicable only for the following key types:\nHTTP_HEADER -- Name of the HTTP header whose value is taken as the key value.\nHTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn set_enforce_on_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `exceed_action`.\nAction to take for requests that are above the configured rate limit threshold, to deny with a specified HTTP response code.\nValid options are deny(STATUS), where valid values for STATUS are 403, 404, 429, and 502."]
    pub fn set_exceed_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exceed_action = Some(v.into());
        self
    }
    #[doc = "Set the field `ban_threshold`.\n"]
    pub fn set_ban_threshold(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ban_threshold = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ban_threshold = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `enforce_on_key_configs`.\n"]
    pub fn set_enforce_on_key_configs(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.enforce_on_key_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.enforce_on_key_configs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rate_limit_threshold`.\n"]
    pub fn set_rate_limit_threshold(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rate_limit_threshold = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rate_limit_threshold = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionSecurityPolicyRulesElRateLimitOptionsEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRulesElRateLimitOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulesElRateLimitOptionsEl {}
impl BuildComputeRegionSecurityPolicyRulesElRateLimitOptionsEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRulesElRateLimitOptionsEl {
        ComputeRegionSecurityPolicyRulesElRateLimitOptionsEl {
            ban_duration_sec: core::default::Default::default(),
            conform_action: core::default::Default::default(),
            enforce_on_key: core::default::Default::default(),
            enforce_on_key_name: core::default::Default::default(),
            exceed_action: core::default::Default::default(),
            ban_threshold: core::default::Default::default(),
            enforce_on_key_configs: core::default::Default::default(),
            rate_limit_threshold: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRef {
        ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ban_duration_sec` after provisioning.\nCan only be specified if the action for the rule is \"rate_based_ban\".\nIf specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold."]
    pub fn ban_duration_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ban_duration_sec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `conform_action` after provisioning.\nAction to take for requests that are under the configured rate limit threshold.\nValid option is \"allow\" only."]
    pub fn conform_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.conform_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_on_key` after provisioning.\nDetermines the key to enforce the rateLimitThreshold on. Possible values are:\n* ALL: A single rate limit threshold is applied to all the requests matching this rule. This is the default value if \"enforceOnKey\" is not configured.\n* IP: The source IP address of the request is the key. Each IP has this limit enforced separately.\n* HTTP_HEADER: The value of the HTTP header whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the header value. If no such header is present in the request, the key type defaults to ALL.\n* XFF_IP: The first IP address (i.e. the originating client IP address) specified in the list of IPs under X-Forwarded-For HTTP header. If no such header is present or the value is not a valid IP, the key defaults to the source IP address of the request i.e. key type IP.\n* HTTP_COOKIE: The value of the HTTP cookie whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the cookie value. If no such cookie is present in the request, the key type defaults to ALL.\n* HTTP_PATH: The URL path of the HTTP request. The key value is truncated to the first 128 bytes.\n* SNI: Server name indication in the TLS session of the HTTPS request. The key value is truncated to the first 128 bytes. The key type defaults to ALL on a HTTP session.\n* REGION_CODE: The country/region from which the request originates.\n* TLS_JA3_FINGERPRINT: JA3 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* TLS_JA4_FINGERPRINT: JA4 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* USER_IP: The IP address of the originating client, which is resolved based on \"userIpRequestHeaders\" configured with the security policy. If there is no \"userIpRequestHeaders\" configuration or an IP address cannot be resolved from it, the key type defaults to IP. Possible values: [\"ALL\", \"IP\", \"HTTP_HEADER\", \"XFF_IP\", \"HTTP_COOKIE\", \"HTTP_PATH\", \"SNI\", \"REGION_CODE\", \"TLS_JA3_FINGERPRINT\", \"TLS_JA4_FINGERPRINT\", \"USER_IP\"]"]
    pub fn enforce_on_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_on_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_on_key_name` after provisioning.\nRate limit key name applicable only for the following key types:\nHTTP_HEADER -- Name of the HTTP header whose value is taken as the key value.\nHTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn enforce_on_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_on_key_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exceed_action` after provisioning.\nAction to take for requests that are above the configured rate limit threshold, to deny with a specified HTTP response code.\nValid options are deny(STATUS), where valid values for STATUS are 403, 404, 429, and 502."]
    pub fn exceed_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exceed_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ban_threshold` after provisioning.\n"]
    pub fn ban_threshold(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ban_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_on_key_configs` after provisioning.\n"]
    pub fn enforce_on_key_configs(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enforce_on_key_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rate_limit_threshold` after provisioning.\n"]
    pub fn rate_limit_threshold(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rate_limit_threshold", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicyRulesElDynamic {
    match_: Option<DynamicBlock<ComputeRegionSecurityPolicyRulesElMatchEl>>,
    network_match: Option<DynamicBlock<ComputeRegionSecurityPolicyRulesElNetworkMatchEl>>,
    preconfigured_waf_config:
        Option<DynamicBlock<ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl>>,
    rate_limit_options: Option<DynamicBlock<ComputeRegionSecurityPolicyRulesElRateLimitOptionsEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyRulesEl {
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<PrimField<bool>>,
    priority: PrimField<f64>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<ComputeRegionSecurityPolicyRulesElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_match: Option<Vec<ComputeRegionSecurityPolicyRulesElNetworkMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preconfigured_waf_config:
        Option<Vec<ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_options: Option<Vec<ComputeRegionSecurityPolicyRulesElRateLimitOptionsEl>>,
    dynamic: ComputeRegionSecurityPolicyRulesElDynamic,
}
impl ComputeRegionSecurityPolicyRulesEl {
    #[doc = "Set the field `description`.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `preview`.\nIf set to true, the specified action is not enforced."]
    pub fn set_preview(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preview = Some(v.into());
        self
    }
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyRulesElMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_match`.\n"]
    pub fn set_network_match(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyRulesElNetworkMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_match = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_match = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `preconfigured_waf_config`.\n"]
    pub fn set_preconfigured_waf_config(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.preconfigured_waf_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.preconfigured_waf_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rate_limit_options`.\n"]
    pub fn set_rate_limit_options(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionSecurityPolicyRulesElRateLimitOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rate_limit_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rate_limit_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionSecurityPolicyRulesEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyRulesEl {
    #[doc = "The Action to perform when the rule is matched. The following are the valid actions:\n\n* allow: allow access to target.\n\n* deny(STATUS): deny access to target, returns the HTTP response code specified. Valid values for STATUS are 403, 404, and 502.\n\n* rate_based_ban: limit client traffic to the configured threshold and ban the client if the traffic exceeds the threshold. Configure parameters for this action in RateLimitOptions. Requires rateLimitOptions to be set.\n\n* redirect: redirect to a different target. This can either be an internal reCAPTCHA redirect, or an external URL-based redirect via a 302 response. Parameters for this action can be configured via redirectOptions. This action is only supported in Global Security Policies of type CLOUD_ARMOR.\n\n* throttle: limit client traffic to the configured threshold. Configure parameters for this action in rateLimitOptions. Requires rateLimitOptions to be set for this."]
    pub action: PrimField<String>,
    #[doc = "An integer indicating the priority of a rule in the list.\nThe priority must be a positive value between 0 and 2147483647.\nRules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest priority."]
    pub priority: PrimField<f64>,
}
impl BuildComputeRegionSecurityPolicyRulesEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyRulesEl {
        ComputeRegionSecurityPolicyRulesEl {
            action: self.action,
            description: core::default::Default::default(),
            preview: core::default::Default::default(),
            priority: self.priority,
            match_: core::default::Default::default(),
            network_match: core::default::Default::default(),
            preconfigured_waf_config: core::default::Default::default(),
            rate_limit_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyRulesElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionSecurityPolicyRulesElRef {
        ComputeRegionSecurityPolicyRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe Action to perform when the rule is matched. The following are the valid actions:\n\n* allow: allow access to target.\n\n* deny(STATUS): deny access to target, returns the HTTP response code specified. Valid values for STATUS are 403, 404, and 502.\n\n* rate_based_ban: limit client traffic to the configured threshold and ban the client if the traffic exceeds the threshold. Configure parameters for this action in RateLimitOptions. Requires rateLimitOptions to be set.\n\n* redirect: redirect to a different target. This can either be an internal reCAPTCHA redirect, or an external URL-based redirect via a 302 response. Parameters for this action can be configured via redirectOptions. This action is only supported in Global Security Policies of type CLOUD_ARMOR.\n\n* throttle: limit client traffic to the configured threshold. Configure parameters for this action in rateLimitOptions. Requires rateLimitOptions to be set for this."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `preview` after provisioning.\nIf set to true, the specified action is not enforced."]
    pub fn preview(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preview", self.base))
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nAn integer indicating the priority of a rule in the list.\nThe priority must be a positive value between 0 and 2147483647.\nRules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest priority."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<ComputeRegionSecurityPolicyRulesElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
    #[doc = "Get a reference to the value of field `network_match` after provisioning.\n"]
    pub fn network_match(&self) -> ListRef<ComputeRegionSecurityPolicyRulesElNetworkMatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_match", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `preconfigured_waf_config` after provisioning.\n"]
    pub fn preconfigured_waf_config(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preconfigured_waf_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rate_limit_options` after provisioning.\n"]
    pub fn rate_limit_options(
        &self,
    ) -> ListRef<ComputeRegionSecurityPolicyRulesElRateLimitOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rate_limit_options", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeRegionSecurityPolicyTimeoutsEl {
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
impl ToListMappable for ComputeRegionSecurityPolicyTimeoutsEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyTimeoutsEl {}
impl BuildComputeRegionSecurityPolicyTimeoutsEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyTimeoutsEl {
        ComputeRegionSecurityPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionSecurityPolicyTimeoutsElRef {
        ComputeRegionSecurityPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyTimeoutsElRef {
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
pub struct ComputeRegionSecurityPolicyUserDefinedFieldsEl {
    base: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mask: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
}
impl ComputeRegionSecurityPolicyUserDefinedFieldsEl {
    #[doc = "Set the field `mask`.\nIf specified, apply this mask (bitwise AND) to the field to ignore bits before matching.\nEncoded as a hexadecimal number (starting with \"0x\").\nThe last byte of the field (in network byte order) corresponds to the least significant byte of the mask."]
    pub fn set_mask(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mask = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nThe name of this field. Must be unique within the policy."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `offset`.\nOffset of the first byte of the field (in network byte order) relative to 'base'."]
    pub fn set_offset(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.offset = Some(v.into());
        self
    }
    #[doc = "Set the field `size`.\nSize of the field in bytes. Valid values: 1-4."]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionSecurityPolicyUserDefinedFieldsEl {
    type O = BlockAssignable<ComputeRegionSecurityPolicyUserDefinedFieldsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionSecurityPolicyUserDefinedFieldsEl {
    #[doc = "The base relative to which 'offset' is measured. Possible values are:\n- IPV4: Points to the beginning of the IPv4 header.\n- IPV6: Points to the beginning of the IPv6 header.\n- TCP: Points to the beginning of the TCP header, skipping over any IPv4 options or IPv6 extension headers. Not present for non-first fragments.\n- UDP: Points to the beginning of the UDP header, skipping over any IPv4 options or IPv6 extension headers. Not present for non-first fragments. Possible values: [\"IPV4\", \"IPV6\", \"TCP\", \"UDP\"]"]
    pub base: PrimField<String>,
}
impl BuildComputeRegionSecurityPolicyUserDefinedFieldsEl {
    pub fn build(self) -> ComputeRegionSecurityPolicyUserDefinedFieldsEl {
        ComputeRegionSecurityPolicyUserDefinedFieldsEl {
            base: self.base,
            mask: core::default::Default::default(),
            name: core::default::Default::default(),
            offset: core::default::Default::default(),
            size: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionSecurityPolicyUserDefinedFieldsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionSecurityPolicyUserDefinedFieldsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionSecurityPolicyUserDefinedFieldsElRef {
        ComputeRegionSecurityPolicyUserDefinedFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionSecurityPolicyUserDefinedFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base` after provisioning.\nThe base relative to which 'offset' is measured. Possible values are:\n- IPV4: Points to the beginning of the IPv4 header.\n- IPV6: Points to the beginning of the IPv6 header.\n- TCP: Points to the beginning of the TCP header, skipping over any IPv4 options or IPv6 extension headers. Not present for non-first fragments.\n- UDP: Points to the beginning of the UDP header, skipping over any IPv4 options or IPv6 extension headers. Not present for non-first fragments. Possible values: [\"IPV4\", \"IPV6\", \"TCP\", \"UDP\"]"]
    pub fn base(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }
    #[doc = "Get a reference to the value of field `mask` after provisioning.\nIf specified, apply this mask (bitwise AND) to the field to ignore bits before matching.\nEncoded as a hexadecimal number (starting with \"0x\").\nThe last byte of the field (in network byte order) corresponds to the least significant byte of the mask."]
    pub fn mask(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mask", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this field. Must be unique within the policy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `offset` after provisioning.\nOffset of the first byte of the field (in network byte order) relative to 'base'."]
    pub fn offset(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.offset", self.base))
    }
    #[doc = "Get a reference to the value of field `size` after provisioning.\nSize of the field in bytes. Valid values: 1-4."]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicyDynamic {
    advanced_options_config:
        Option<DynamicBlock<ComputeRegionSecurityPolicyAdvancedOptionsConfigEl>>,
    ddos_protection_config: Option<DynamicBlock<ComputeRegionSecurityPolicyDdosProtectionConfigEl>>,
    rules: Option<DynamicBlock<ComputeRegionSecurityPolicyRulesEl>>,
    user_defined_fields: Option<DynamicBlock<ComputeRegionSecurityPolicyUserDefinedFieldsEl>>,
}
