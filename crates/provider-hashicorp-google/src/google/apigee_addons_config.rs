use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApigeeAddonsConfigData {
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
    id: Option<PrimField<String>>,
    org: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    addons_config: Option<Vec<ApigeeAddonsConfigAddonsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeAddonsConfigTimeoutsEl>,
    dynamic: ApigeeAddonsConfigDynamic,
}
struct ApigeeAddonsConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeAddonsConfigData>,
}
#[derive(Clone)]
pub struct ApigeeAddonsConfig(Rc<ApigeeAddonsConfig_>);
impl ApigeeAddonsConfig {
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
    #[doc = "Set the field `addons_config`.\n"]
    pub fn set_addons_config(
        self,
        v: impl Into<BlockAssignable<ApigeeAddonsConfigAddonsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().addons_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.addons_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApigeeAddonsConfigTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `org` after provisioning.\nName of the Apigee organization."]
    pub fn org(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `addons_config` after provisioning.\n"]
    pub fn addons_config(&self) -> ListRef<ApigeeAddonsConfigAddonsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.addons_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeAddonsConfigTimeoutsElRef {
        ApigeeAddonsConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApigeeAddonsConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApigeeAddonsConfig {}
impl ToListMappable for ApigeeAddonsConfig {
    type O = ListRef<ApigeeAddonsConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApigeeAddonsConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_addons_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApigeeAddonsConfig {
    pub tf_id: String,
    #[doc = "Name of the Apigee organization."]
    pub org: PrimField<String>,
}
impl BuildApigeeAddonsConfig {
    pub fn build(self, stack: &mut Stack) -> ApigeeAddonsConfig {
        let out = ApigeeAddonsConfig(Rc::new(ApigeeAddonsConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeAddonsConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                org: self.org,
                addons_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApigeeAddonsConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeAddonsConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApigeeAddonsConfigRef {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `org` after provisioning.\nName of the Apigee organization."]
    pub fn org(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `addons_config` after provisioning.\n"]
    pub fn addons_config(&self) -> ListRef<ApigeeAddonsConfigAddonsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.addons_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeAddonsConfigTimeoutsElRef {
        ApigeeAddonsConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl {
    #[doc = "Set the field `enabled`.\nFlag that specifies whether the Advanced API Ops add-on is enabled."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl {
    type O = BlockAssignable<ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl {}
impl BuildApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl {
    pub fn build(self) -> ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl {
        ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigElRef {
        ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nFlag that specifies whether the Advanced API Ops add-on is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl {
    #[doc = "Set the field `enabled`.\nFlag that specifies whether the API security add-on is enabled."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl {
    type O = BlockAssignable<ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl {}
impl BuildApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl {
    pub fn build(self) -> ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl {
        ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct ApigeeAddonsConfigAddonsConfigElApiSecurityConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeAddonsConfigAddonsConfigElApiSecurityConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeAddonsConfigAddonsConfigElApiSecurityConfigElRef {
        ApigeeAddonsConfigAddonsConfigElApiSecurityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeAddonsConfigAddonsConfigElApiSecurityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nFlag that specifies whether the API security add-on is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `expires_at` after provisioning.\nTime at which the API Security add-on expires in in milliseconds since epoch. If unspecified, the add-on will never expire."]
    pub fn expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl {
    #[doc = "Set the field `enabled`.\nFlag that specifies whether the Connectors Platform add-on is enabled."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl {
    type O = BlockAssignable<ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl {}
impl BuildApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl {
    pub fn build(self) -> ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl {
        ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigElRef {
        ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nFlag that specifies whether the Connectors Platform add-on is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `expires_at` after provisioning.\nTime at which the Connectors Platform add-on expires in milliseconds since epoch. If unspecified, the add-on will never expire."]
    pub fn expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl {
    #[doc = "Set the field `enabled`.\nFlag that specifies whether the Integration add-on is enabled."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl {
    type O = BlockAssignable<ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeAddonsConfigAddonsConfigElIntegrationConfigEl {}
impl BuildApigeeAddonsConfigAddonsConfigElIntegrationConfigEl {
    pub fn build(self) -> ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl {
        ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct ApigeeAddonsConfigAddonsConfigElIntegrationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeAddonsConfigAddonsConfigElIntegrationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeAddonsConfigAddonsConfigElIntegrationConfigElRef {
        ApigeeAddonsConfigAddonsConfigElIntegrationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeAddonsConfigAddonsConfigElIntegrationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nFlag that specifies whether the Integration add-on is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl {
    #[doc = "Set the field `enabled`.\nFlag that specifies whether the Monetization add-on is enabled."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl {
    type O = BlockAssignable<ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeAddonsConfigAddonsConfigElMonetizationConfigEl {}
impl BuildApigeeAddonsConfigAddonsConfigElMonetizationConfigEl {
    pub fn build(self) -> ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl {
        ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct ApigeeAddonsConfigAddonsConfigElMonetizationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeAddonsConfigAddonsConfigElMonetizationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeAddonsConfigAddonsConfigElMonetizationConfigElRef {
        ApigeeAddonsConfigAddonsConfigElMonetizationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeAddonsConfigAddonsConfigElMonetizationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nFlag that specifies whether the Monetization add-on is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApigeeAddonsConfigAddonsConfigElDynamic {
    advanced_api_ops_config:
        Option<DynamicBlock<ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl>>,
    api_security_config: Option<DynamicBlock<ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl>>,
    connectors_platform_config:
        Option<DynamicBlock<ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl>>,
    integration_config: Option<DynamicBlock<ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl>>,
    monetization_config: Option<DynamicBlock<ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl>>,
}
#[derive(Serialize)]
pub struct ApigeeAddonsConfigAddonsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_api_ops_config: Option<Vec<ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_security_config: Option<Vec<ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connectors_platform_config:
        Option<Vec<ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integration_config: Option<Vec<ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monetization_config: Option<Vec<ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl>>,
    dynamic: ApigeeAddonsConfigAddonsConfigElDynamic,
}
impl ApigeeAddonsConfigAddonsConfigEl {
    #[doc = "Set the field `advanced_api_ops_config`.\n"]
    pub fn set_advanced_api_ops_config(
        mut self,
        v: impl Into<BlockAssignable<ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advanced_api_ops_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_api_ops_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `api_security_config`.\n"]
    pub fn set_api_security_config(
        mut self,
        v: impl Into<BlockAssignable<ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.api_security_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.api_security_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `connectors_platform_config`.\n"]
    pub fn set_connectors_platform_config(
        mut self,
        v: impl Into<BlockAssignable<ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.connectors_platform_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.connectors_platform_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `integration_config`.\n"]
    pub fn set_integration_config(
        mut self,
        v: impl Into<BlockAssignable<ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.integration_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.integration_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `monetization_config`.\n"]
    pub fn set_monetization_config(
        mut self,
        v: impl Into<BlockAssignable<ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.monetization_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.monetization_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApigeeAddonsConfigAddonsConfigEl {
    type O = BlockAssignable<ApigeeAddonsConfigAddonsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeAddonsConfigAddonsConfigEl {}
impl BuildApigeeAddonsConfigAddonsConfigEl {
    pub fn build(self) -> ApigeeAddonsConfigAddonsConfigEl {
        ApigeeAddonsConfigAddonsConfigEl {
            advanced_api_ops_config: core::default::Default::default(),
            api_security_config: core::default::Default::default(),
            connectors_platform_config: core::default::Default::default(),
            integration_config: core::default::Default::default(),
            monetization_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApigeeAddonsConfigAddonsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeAddonsConfigAddonsConfigElRef {
    fn new(shared: StackShared, base: String) -> ApigeeAddonsConfigAddonsConfigElRef {
        ApigeeAddonsConfigAddonsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeAddonsConfigAddonsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `advanced_api_ops_config` after provisioning.\n"]
    pub fn advanced_api_ops_config(
        &self,
    ) -> ListRef<ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_api_ops_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `api_security_config` after provisioning.\n"]
    pub fn api_security_config(
        &self,
    ) -> ListRef<ApigeeAddonsConfigAddonsConfigElApiSecurityConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_security_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `connectors_platform_config` after provisioning.\n"]
    pub fn connectors_platform_config(
        &self,
    ) -> ListRef<ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connectors_platform_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `integration_config` after provisioning.\n"]
    pub fn integration_config(
        &self,
    ) -> ListRef<ApigeeAddonsConfigAddonsConfigElIntegrationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.integration_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `monetization_config` after provisioning.\n"]
    pub fn monetization_config(
        &self,
    ) -> ListRef<ApigeeAddonsConfigAddonsConfigElMonetizationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.monetization_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeAddonsConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApigeeAddonsConfigTimeoutsEl {
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
impl ToListMappable for ApigeeAddonsConfigTimeoutsEl {
    type O = BlockAssignable<ApigeeAddonsConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeAddonsConfigTimeoutsEl {}
impl BuildApigeeAddonsConfigTimeoutsEl {
    pub fn build(self) -> ApigeeAddonsConfigTimeoutsEl {
        ApigeeAddonsConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApigeeAddonsConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeAddonsConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeAddonsConfigTimeoutsElRef {
        ApigeeAddonsConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeAddonsConfigTimeoutsElRef {
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
struct ApigeeAddonsConfigDynamic {
    addons_config: Option<DynamicBlock<ApigeeAddonsConfigAddonsConfigEl>>,
}
