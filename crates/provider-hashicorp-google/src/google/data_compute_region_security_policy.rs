use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeRegionSecurityPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataComputeRegionSecurityPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeRegionSecurityPolicyData>,
}
#[derive(Clone)]
pub struct DataComputeRegionSecurityPolicy(Rc<DataComputeRegionSecurityPolicy_>);
impl DataComputeRegionSecurityPolicy {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
    #[doc = "Get a reference to the value of field `advanced_options_config` after provisioning.\nAdvanced Options Config of this security policy."]
    pub fn advanced_options_config(
        &self,
    ) -> ListRef<DataComputeRegionSecurityPolicyAdvancedOptionsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_options_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ddos_protection_config` after provisioning.\nConfiguration for Google Cloud Armor DDOS Proctection Config."]
    pub fn ddos_protection_config(
        &self,
    ) -> ListRef<DataComputeRegionSecurityPolicyDdosProtectionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ddos_protection_config", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `rules` after provisioning.\nThe set of rules that belong to this policy. There must always be a default rule (rule with priority 2147483647 and match \"*\"). If no rules are provided when creating a security policy, a default rule with action \"allow\" will be added."]
    pub fn rules(&self) -> ListRef<DataComputeRegionSecurityPolicyRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `user_defined_fields` after provisioning.\nDefinitions of user-defined fields for CLOUD_ARMOR_NETWORK policies.\nA user-defined field consists of up to 4 bytes extracted from a fixed offset in the packet, relative to the IPv4, IPv6, TCP, or UDP header, with an optional mask to select certain bits.\nRules may then specify matching values for these fields."]
    pub fn user_defined_fields(
        &self,
    ) -> ListRef<DataComputeRegionSecurityPolicyUserDefinedFieldsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_defined_fields", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeRegionSecurityPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeRegionSecurityPolicy {}
impl ToListMappable for DataComputeRegionSecurityPolicy {
    type O = ListRef<DataComputeRegionSecurityPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeRegionSecurityPolicy_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_region_security_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeRegionSecurityPolicy {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])? which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildDataComputeRegionSecurityPolicy {
    pub fn build(self, stack: &mut Stack) -> DataComputeRegionSecurityPolicy {
        let out = DataComputeRegionSecurityPolicy(Rc::new(DataComputeRegionSecurityPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeRegionSecurityPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeRegionSecurityPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeRegionSecurityPolicyRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `advanced_options_config` after provisioning.\nAdvanced Options Config of this security policy."]
    pub fn advanced_options_config(
        &self,
    ) -> ListRef<DataComputeRegionSecurityPolicyAdvancedOptionsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_options_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ddos_protection_config` after provisioning.\nConfiguration for Google Cloud Armor DDOS Proctection Config."]
    pub fn ddos_protection_config(
        &self,
    ) -> ListRef<DataComputeRegionSecurityPolicyDdosProtectionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ddos_protection_config", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `rules` after provisioning.\nThe set of rules that belong to this policy. There must always be a default rule (rule with priority 2147483647 and match \"*\"). If no rules are provided when creating a security policy, a default rule with action \"allow\" will be added."]
    pub fn rules(&self) -> ListRef<DataComputeRegionSecurityPolicyRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `user_defined_fields` after provisioning.\nDefinitions of user-defined fields for CLOUD_ARMOR_NETWORK policies.\nA user-defined field consists of up to 4 bytes extracted from a fixed offset in the packet, relative to the IPv4, IPv6, TCP, or UDP header, with an optional mask to select certain bits.\nRules may then specify matching values for these fields."]
    pub fn user_defined_fields(
        &self,
    ) -> ListRef<DataComputeRegionSecurityPolicyUserDefinedFieldsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_defined_fields", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content_types: Option<SetField<PrimField<String>>>,
}
impl DataComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    #[doc = "Set the field `content_types`.\n"]
    pub fn set_content_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.content_types = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    type O =
        BlockAssignable<DataComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {}
impl BuildDataComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    pub fn build(self) -> DataComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
        DataComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
            content_types: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
        DataComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content_types` after provisioning.\n"]
    pub fn content_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.content_types", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyAdvancedOptionsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    json_custom_config:
        Option<ListField<DataComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_parsing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_ip_request_headers: Option<SetField<PrimField<String>>>,
}
impl DataComputeRegionSecurityPolicyAdvancedOptionsConfigEl {
    #[doc = "Set the field `json_custom_config`.\n"]
    pub fn set_json_custom_config(
        mut self,
        v: impl Into<
            ListField<DataComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>,
        >,
    ) -> Self {
        self.json_custom_config = Some(v.into());
        self
    }
    #[doc = "Set the field `json_parsing`.\n"]
    pub fn set_json_parsing(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.json_parsing = Some(v.into());
        self
    }
    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_level = Some(v.into());
        self
    }
    #[doc = "Set the field `user_ip_request_headers`.\n"]
    pub fn set_user_ip_request_headers(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.user_ip_request_headers = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionSecurityPolicyAdvancedOptionsConfigEl {
    type O = BlockAssignable<DataComputeRegionSecurityPolicyAdvancedOptionsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyAdvancedOptionsConfigEl {}
impl BuildDataComputeRegionSecurityPolicyAdvancedOptionsConfigEl {
    pub fn build(self) -> DataComputeRegionSecurityPolicyAdvancedOptionsConfigEl {
        DataComputeRegionSecurityPolicyAdvancedOptionsConfigEl {
            json_custom_config: core::default::Default::default(),
            json_parsing: core::default::Default::default(),
            log_level: core::default::Default::default(),
            user_ip_request_headers: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyAdvancedOptionsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyAdvancedOptionsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionSecurityPolicyAdvancedOptionsConfigElRef {
        DataComputeRegionSecurityPolicyAdvancedOptionsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionSecurityPolicyAdvancedOptionsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `json_custom_config` after provisioning.\n"]
    pub fn json_custom_config(
        &self,
    ) -> ListRef<DataComputeRegionSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.json_custom_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `json_parsing` after provisioning.\n"]
    pub fn json_parsing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.json_parsing", self.base))
    }
    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }
    #[doc = "Get a reference to the value of field `user_ip_request_headers` after provisioning.\n"]
    pub fn user_ip_request_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.user_ip_request_headers", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyDdosProtectionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ddos_protection: Option<PrimField<String>>,
}
impl DataComputeRegionSecurityPolicyDdosProtectionConfigEl {
    #[doc = "Set the field `ddos_protection`.\n"]
    pub fn set_ddos_protection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ddos_protection = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionSecurityPolicyDdosProtectionConfigEl {
    type O = BlockAssignable<DataComputeRegionSecurityPolicyDdosProtectionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyDdosProtectionConfigEl {}
impl BuildDataComputeRegionSecurityPolicyDdosProtectionConfigEl {
    pub fn build(self) -> DataComputeRegionSecurityPolicyDdosProtectionConfigEl {
        DataComputeRegionSecurityPolicyDdosProtectionConfigEl {
            ddos_protection: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyDdosProtectionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyDdosProtectionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionSecurityPolicyDdosProtectionConfigElRef {
        DataComputeRegionSecurityPolicyDdosProtectionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionSecurityPolicyDdosProtectionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ddos_protection` after provisioning.\n"]
    pub fn ddos_protection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ddos_protection", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyRulesElMatchElConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    src_ip_ranges: Option<ListField<PrimField<String>>>,
}
impl DataComputeRegionSecurityPolicyRulesElMatchElConfigEl {
    #[doc = "Set the field `src_ip_ranges`.\n"]
    pub fn set_src_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_ip_ranges = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionSecurityPolicyRulesElMatchElConfigEl {
    type O = BlockAssignable<DataComputeRegionSecurityPolicyRulesElMatchElConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyRulesElMatchElConfigEl {}
impl BuildDataComputeRegionSecurityPolicyRulesElMatchElConfigEl {
    pub fn build(self) -> DataComputeRegionSecurityPolicyRulesElMatchElConfigEl {
        DataComputeRegionSecurityPolicyRulesElMatchElConfigEl {
            src_ip_ranges: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyRulesElMatchElConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyRulesElMatchElConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionSecurityPolicyRulesElMatchElConfigElRef {
        DataComputeRegionSecurityPolicyRulesElMatchElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionSecurityPolicyRulesElMatchElConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `src_ip_ranges` after provisioning.\n"]
    pub fn src_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_ip_ranges", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyRulesElMatchElExprEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
}
impl DataComputeRegionSecurityPolicyRulesElMatchElExprEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionSecurityPolicyRulesElMatchElExprEl {
    type O = BlockAssignable<DataComputeRegionSecurityPolicyRulesElMatchElExprEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyRulesElMatchElExprEl {}
impl BuildDataComputeRegionSecurityPolicyRulesElMatchElExprEl {
    pub fn build(self) -> DataComputeRegionSecurityPolicyRulesElMatchElExprEl {
        DataComputeRegionSecurityPolicyRulesElMatchElExprEl {
            expression: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyRulesElMatchElExprElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyRulesElMatchElExprElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionSecurityPolicyRulesElMatchElExprElRef {
        DataComputeRegionSecurityPolicyRulesElMatchElExprElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionSecurityPolicyRulesElMatchElExprElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyRulesElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<ListField<DataComputeRegionSecurityPolicyRulesElMatchElConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expr: Option<ListField<DataComputeRegionSecurityPolicyRulesElMatchElExprEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    versioned_expr: Option<PrimField<String>>,
}
impl DataComputeRegionSecurityPolicyRulesElMatchEl {
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl Into<ListField<DataComputeRegionSecurityPolicyRulesElMatchElConfigEl>>,
    ) -> Self {
        self.config = Some(v.into());
        self
    }
    #[doc = "Set the field `expr`.\n"]
    pub fn set_expr(
        mut self,
        v: impl Into<ListField<DataComputeRegionSecurityPolicyRulesElMatchElExprEl>>,
    ) -> Self {
        self.expr = Some(v.into());
        self
    }
    #[doc = "Set the field `versioned_expr`.\n"]
    pub fn set_versioned_expr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.versioned_expr = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionSecurityPolicyRulesElMatchEl {
    type O = BlockAssignable<DataComputeRegionSecurityPolicyRulesElMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyRulesElMatchEl {}
impl BuildDataComputeRegionSecurityPolicyRulesElMatchEl {
    pub fn build(self) -> DataComputeRegionSecurityPolicyRulesElMatchEl {
        DataComputeRegionSecurityPolicyRulesElMatchEl {
            config: core::default::Default::default(),
            expr: core::default::Default::default(),
            versioned_expr: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyRulesElMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyRulesElMatchElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRegionSecurityPolicyRulesElMatchElRef {
        DataComputeRegionSecurityPolicyRulesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionSecurityPolicyRulesElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<DataComputeRegionSecurityPolicyRulesElMatchElConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
    #[doc = "Get a reference to the value of field `expr` after provisioning.\n"]
    pub fn expr(&self) -> ListRef<DataComputeRegionSecurityPolicyRulesElMatchElExprElRef> {
        ListRef::new(self.shared().clone(), format!("{}.expr", self.base))
    }
    #[doc = "Get a reference to the value of field `versioned_expr` after provisioning.\n"]
    pub fn versioned_expr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.versioned_expr", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}
impl DataComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl {
    type O =
        BlockAssignable<DataComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl {}
impl BuildDataComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl {
    pub fn build(self) -> DataComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl {
        DataComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl {
            name: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsElRef {
        DataComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyRulesElNetworkMatchEl {
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
        Option<ListField<DataComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl>>,
}
impl DataComputeRegionSecurityPolicyRulesElNetworkMatchEl {
    #[doc = "Set the field `dest_ip_ranges`.\n"]
    pub fn set_dest_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `dest_ports`.\n"]
    pub fn set_dest_ports(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_ports = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_protocols`.\n"]
    pub fn set_ip_protocols(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_protocols = Some(v.into());
        self
    }
    #[doc = "Set the field `src_asns`.\n"]
    pub fn set_src_asns(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.src_asns = Some(v.into());
        self
    }
    #[doc = "Set the field `src_ip_ranges`.\n"]
    pub fn set_src_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `src_ports`.\n"]
    pub fn set_src_ports(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_ports = Some(v.into());
        self
    }
    #[doc = "Set the field `src_region_codes`.\n"]
    pub fn set_src_region_codes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_region_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `user_defined_fields`.\n"]
    pub fn set_user_defined_fields(
        mut self,
        v: impl Into<ListField<DataComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsEl>>,
    ) -> Self {
        self.user_defined_fields = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionSecurityPolicyRulesElNetworkMatchEl {
    type O = BlockAssignable<DataComputeRegionSecurityPolicyRulesElNetworkMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyRulesElNetworkMatchEl {}
impl BuildDataComputeRegionSecurityPolicyRulesElNetworkMatchEl {
    pub fn build(self) -> DataComputeRegionSecurityPolicyRulesElNetworkMatchEl {
        DataComputeRegionSecurityPolicyRulesElNetworkMatchEl {
            dest_ip_ranges: core::default::Default::default(),
            dest_ports: core::default::Default::default(),
            ip_protocols: core::default::Default::default(),
            src_asns: core::default::Default::default(),
            src_ip_ranges: core::default::Default::default(),
            src_ports: core::default::Default::default(),
            src_region_codes: core::default::Default::default(),
            user_defined_fields: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyRulesElNetworkMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyRulesElNetworkMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionSecurityPolicyRulesElNetworkMatchElRef {
        DataComputeRegionSecurityPolicyRulesElNetworkMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionSecurityPolicyRulesElNetworkMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dest_ip_ranges` after provisioning.\n"]
    pub fn dest_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dest_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dest_ports` after provisioning.\n"]
    pub fn dest_ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dest_ports", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_protocols` after provisioning.\n"]
    pub fn ip_protocols(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_protocols", self.base))
    }
    #[doc = "Get a reference to the value of field `src_asns` after provisioning.\n"]
    pub fn src_asns(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.src_asns", self.base))
    }
    #[doc = "Get a reference to the value of field `src_ip_ranges` after provisioning.\n"]
    pub fn src_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `src_ports` after provisioning.\n"]
    pub fn src_ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.src_ports", self.base))
    }
    #[doc = "Get a reference to the value of field `src_region_codes` after provisioning.\n"]
    pub fn src_region_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.src_region_codes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_defined_fields` after provisioning.\n"]
    pub fn user_defined_fields(
        &self,
    ) -> ListRef<DataComputeRegionSecurityPolicyRulesElNetworkMatchElUserDefinedFieldsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_defined_fields", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl {
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl
{
    type O = BlockAssignable<
        DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl
{}
impl BuildDataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl {
    pub fn build(
        self,
    ) -> DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl
    {
        DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl {
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieElRef
    {
        DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl {
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl
{
    type O = BlockAssignable<
        DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl
{}
impl BuildDataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl {
    pub fn build(
        self,
    ) -> DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl
    {
        DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl {
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderElRef
    {
        DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl {
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl
{
    type O = BlockAssignable < DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl
{}
impl BuildDataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl { pub fn build (self) -> DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl { DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl { operator : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef { fn new (shared : StackShared , base : String) -> DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef { DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef { shared : shared , base : base . to_string () , } } }
impl
    DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl {
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl
{
    type O = BlockAssignable<
        DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl
{}
impl BuildDataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl {
    pub fn build(
        self,
    ) -> DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl {
        DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl {
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriElRef
    {
        DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl { # [serde (skip_serializing_if = "Option::is_none")] request_cookie : Option < ListField < DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl > > , # [serde (skip_serializing_if = "Option::is_none")] request_header : Option < ListField < DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl > > , # [serde (skip_serializing_if = "Option::is_none")] request_query_param : Option < ListField < DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl > > , # [serde (skip_serializing_if = "Option::is_none")] request_uri : Option < ListField < DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl > > , # [serde (skip_serializing_if = "Option::is_none")] target_rule_ids : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] target_rule_set : Option < PrimField < String > > , }
impl DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl {
    #[doc = "Set the field `request_cookie`.\n"]
    pub fn set_request_cookie(
        mut self,
        v : impl Into < ListField < DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieEl > >,
    ) -> Self {
        self.request_cookie = Some(v.into());
        self
    }
    #[doc = "Set the field `request_header`.\n"]
    pub fn set_request_header(
        mut self,
        v : impl Into < ListField < DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderEl > >,
    ) -> Self {
        self.request_header = Some(v.into());
        self
    }
    #[doc = "Set the field `request_query_param`.\n"]
    pub fn set_request_query_param(
        mut self,
        v : impl Into < ListField < DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamEl > >,
    ) -> Self {
        self.request_query_param = Some(v.into());
        self
    }
    #[doc = "Set the field `request_uri`.\n"]
    pub fn set_request_uri(
        mut self,
        v : impl Into < ListField < DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriEl > >,
    ) -> Self {
        self.request_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `target_rule_ids`.\n"]
    pub fn set_target_rule_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.target_rule_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `target_rule_set`.\n"]
    pub fn set_target_rule_set(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_rule_set = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl {
    type O =
        BlockAssignable<DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl {}
impl BuildDataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl {
    pub fn build(
        self,
    ) -> DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl {
        DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl {
            request_cookie: core::default::Default::default(),
            request_header: core::default::Default::default(),
            request_query_param: core::default::Default::default(),
            request_uri: core::default::Default::default(),
            target_rule_ids: core::default::Default::default(),
            target_rule_set: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRef {
        DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `request_cookie` after provisioning.\n"]
    pub fn request_cookie(
        &self,
    ) -> ListRef<
        DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestCookieElRef,
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
        DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestHeaderElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_header", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_query_param` after provisioning.\n"]    pub fn request_query_param (& self) -> ListRef < DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestQueryParamElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_query_param", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_uri` after provisioning.\n"]
    pub fn request_uri(
        &self,
    ) -> ListRef<
        DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRequestUriElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.request_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `target_rule_ids` after provisioning.\n"]
    pub fn target_rule_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_rule_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_rule_set` after provisioning.\n"]
    pub fn target_rule_set(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_rule_set", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion: Option<
        ListField<DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl>,
    >,
}
impl DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl {
    #[doc = "Set the field `exclusion`.\n"]
    pub fn set_exclusion(
        mut self,
        v: impl Into<
            ListField<DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionEl>,
        >,
    ) -> Self {
        self.exclusion = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl {
    type O = BlockAssignable<DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl {}
impl BuildDataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl {
    pub fn build(self) -> DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl {
        DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl {
            exclusion: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElRef {
        DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exclusion` after provisioning.\n"]
    pub fn exclusion(
        &self,
    ) -> ListRef<DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElExclusionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclusion", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_sec: Option<PrimField<f64>>,
}
impl DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `interval_sec`.\n"]
    pub fn set_interval_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval_sec = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl {
    type O =
        BlockAssignable<DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl {}
impl BuildDataComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl {
    pub fn build(self) -> DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl {
        DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl {
            count: core::default::Default::default(),
            interval_sec: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdElRef {
        DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `interval_sec` after provisioning.\n"]
    pub fn interval_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_sec", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_type: Option<PrimField<String>>,
}
impl DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl {
    #[doc = "Set the field `enforce_on_key_name`.\n"]
    pub fn set_enforce_on_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_on_key_type`.\n"]
    pub fn set_enforce_on_key_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key_type = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl
{
    type O = BlockAssignable<
        DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl {}
impl BuildDataComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl {
    pub fn build(
        self,
    ) -> DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl {
        DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl {
            enforce_on_key_name: core::default::Default::default(),
            enforce_on_key_type: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsElRef {
        DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enforce_on_key_name` after provisioning.\n"]
    pub fn enforce_on_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_on_key_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_on_key_type` after provisioning.\n"]
    pub fn enforce_on_key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_on_key_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_sec: Option<PrimField<f64>>,
}
impl DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }
    #[doc = "Set the field `interval_sec`.\n"]
    pub fn set_interval_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval_sec = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl
{
    type O = BlockAssignable<
        DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl {}
impl BuildDataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl {
    pub fn build(
        self,
    ) -> DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl {
        DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl {
            count: core::default::Default::default(),
            interval_sec: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdElRef {
        DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }
    #[doc = "Get a reference to the value of field `interval_sec` after provisioning.\n"]
    pub fn interval_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_sec", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyRulesElRateLimitOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ban_duration_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ban_threshold:
        Option<ListField<DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conform_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_configs: Option<
        ListField<DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exceed_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_threshold: Option<
        ListField<DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl>,
    >,
}
impl DataComputeRegionSecurityPolicyRulesElRateLimitOptionsEl {
    #[doc = "Set the field `ban_duration_sec`.\n"]
    pub fn set_ban_duration_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ban_duration_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `ban_threshold`.\n"]
    pub fn set_ban_threshold(
        mut self,
        v: impl Into<ListField<DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdEl>>,
    ) -> Self {
        self.ban_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `conform_action`.\n"]
    pub fn set_conform_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.conform_action = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_on_key`.\n"]
    pub fn set_enforce_on_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_on_key_configs`.\n"]
    pub fn set_enforce_on_key_configs(
        mut self,
        v: impl Into<
            ListField<
                DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsEl,
            >,
        >,
    ) -> Self {
        self.enforce_on_key_configs = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_on_key_name`.\n"]
    pub fn set_enforce_on_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `exceed_action`.\n"]
    pub fn set_exceed_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exceed_action = Some(v.into());
        self
    }
    #[doc = "Set the field `rate_limit_threshold`.\n"]
    pub fn set_rate_limit_threshold(
        mut self,
        v: impl Into<
            ListField<DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdEl>,
        >,
    ) -> Self {
        self.rate_limit_threshold = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionSecurityPolicyRulesElRateLimitOptionsEl {
    type O = BlockAssignable<DataComputeRegionSecurityPolicyRulesElRateLimitOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyRulesElRateLimitOptionsEl {}
impl BuildDataComputeRegionSecurityPolicyRulesElRateLimitOptionsEl {
    pub fn build(self) -> DataComputeRegionSecurityPolicyRulesElRateLimitOptionsEl {
        DataComputeRegionSecurityPolicyRulesElRateLimitOptionsEl {
            ban_duration_sec: core::default::Default::default(),
            ban_threshold: core::default::Default::default(),
            conform_action: core::default::Default::default(),
            enforce_on_key: core::default::Default::default(),
            enforce_on_key_configs: core::default::Default::default(),
            enforce_on_key_name: core::default::Default::default(),
            exceed_action: core::default::Default::default(),
            rate_limit_threshold: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRef {
        DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ban_duration_sec` after provisioning.\n"]
    pub fn ban_duration_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ban_duration_sec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ban_threshold` after provisioning.\n"]
    pub fn ban_threshold(
        &self,
    ) -> ListRef<DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElBanThresholdElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ban_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `conform_action` after provisioning.\n"]
    pub fn conform_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.conform_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_on_key` after provisioning.\n"]
    pub fn enforce_on_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_on_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_on_key_configs` after provisioning.\n"]
    pub fn enforce_on_key_configs(
        &self,
    ) -> ListRef<DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElEnforceOnKeyConfigsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enforce_on_key_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_on_key_name` after provisioning.\n"]
    pub fn enforce_on_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_on_key_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exceed_action` after provisioning.\n"]
    pub fn exceed_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exceed_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rate_limit_threshold` after provisioning.\n"]
    pub fn rate_limit_threshold(
        &self,
    ) -> ListRef<DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRateLimitThresholdElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rate_limit_threshold", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<DataComputeRegionSecurityPolicyRulesElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_match: Option<ListField<DataComputeRegionSecurityPolicyRulesElNetworkMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preconfigured_waf_config:
        Option<ListField<DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_options: Option<ListField<DataComputeRegionSecurityPolicyRulesElRateLimitOptionsEl>>,
}
impl DataComputeRegionSecurityPolicyRulesEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<ListField<DataComputeRegionSecurityPolicyRulesElMatchEl>>,
    ) -> Self {
        self.match_ = Some(v.into());
        self
    }
    #[doc = "Set the field `network_match`.\n"]
    pub fn set_network_match(
        mut self,
        v: impl Into<ListField<DataComputeRegionSecurityPolicyRulesElNetworkMatchEl>>,
    ) -> Self {
        self.network_match = Some(v.into());
        self
    }
    #[doc = "Set the field `preconfigured_waf_config`.\n"]
    pub fn set_preconfigured_waf_config(
        mut self,
        v: impl Into<ListField<DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigEl>>,
    ) -> Self {
        self.preconfigured_waf_config = Some(v.into());
        self
    }
    #[doc = "Set the field `preview`.\n"]
    pub fn set_preview(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preview = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
    #[doc = "Set the field `rate_limit_options`.\n"]
    pub fn set_rate_limit_options(
        mut self,
        v: impl Into<ListField<DataComputeRegionSecurityPolicyRulesElRateLimitOptionsEl>>,
    ) -> Self {
        self.rate_limit_options = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionSecurityPolicyRulesEl {
    type O = BlockAssignable<DataComputeRegionSecurityPolicyRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyRulesEl {}
impl BuildDataComputeRegionSecurityPolicyRulesEl {
    pub fn build(self) -> DataComputeRegionSecurityPolicyRulesEl {
        DataComputeRegionSecurityPolicyRulesEl {
            action: core::default::Default::default(),
            description: core::default::Default::default(),
            match_: core::default::Default::default(),
            network_match: core::default::Default::default(),
            preconfigured_waf_config: core::default::Default::default(),
            preview: core::default::Default::default(),
            priority: core::default::Default::default(),
            rate_limit_options: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyRulesElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRegionSecurityPolicyRulesElRef {
        DataComputeRegionSecurityPolicyRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionSecurityPolicyRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<DataComputeRegionSecurityPolicyRulesElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
    #[doc = "Get a reference to the value of field `network_match` after provisioning.\n"]
    pub fn network_match(
        &self,
    ) -> ListRef<DataComputeRegionSecurityPolicyRulesElNetworkMatchElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_match", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `preconfigured_waf_config` after provisioning.\n"]
    pub fn preconfigured_waf_config(
        &self,
    ) -> ListRef<DataComputeRegionSecurityPolicyRulesElPreconfiguredWafConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preconfigured_waf_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `preview` after provisioning.\n"]
    pub fn preview(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preview", self.base))
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `rate_limit_options` after provisioning.\n"]
    pub fn rate_limit_options(
        &self,
    ) -> ListRef<DataComputeRegionSecurityPolicyRulesElRateLimitOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rate_limit_options", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicyUserDefinedFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mask: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
}
impl DataComputeRegionSecurityPolicyUserDefinedFieldsEl {
    #[doc = "Set the field `base`.\n"]
    pub fn set_base(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.base = Some(v.into());
        self
    }
    #[doc = "Set the field `mask`.\n"]
    pub fn set_mask(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mask = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `offset`.\n"]
    pub fn set_offset(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.offset = Some(v.into());
        self
    }
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionSecurityPolicyUserDefinedFieldsEl {
    type O = BlockAssignable<DataComputeRegionSecurityPolicyUserDefinedFieldsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionSecurityPolicyUserDefinedFieldsEl {}
impl BuildDataComputeRegionSecurityPolicyUserDefinedFieldsEl {
    pub fn build(self) -> DataComputeRegionSecurityPolicyUserDefinedFieldsEl {
        DataComputeRegionSecurityPolicyUserDefinedFieldsEl {
            base: core::default::Default::default(),
            mask: core::default::Default::default(),
            name: core::default::Default::default(),
            offset: core::default::Default::default(),
            size: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionSecurityPolicyUserDefinedFieldsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionSecurityPolicyUserDefinedFieldsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionSecurityPolicyUserDefinedFieldsElRef {
        DataComputeRegionSecurityPolicyUserDefinedFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionSecurityPolicyUserDefinedFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base` after provisioning.\n"]
    pub fn base(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }
    #[doc = "Get a reference to the value of field `mask` after provisioning.\n"]
    pub fn mask(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mask", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `offset` after provisioning.\n"]
    pub fn offset(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.offset", self.base))
    }
    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
}
