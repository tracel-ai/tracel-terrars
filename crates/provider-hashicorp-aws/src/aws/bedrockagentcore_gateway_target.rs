use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BedrockagentcoreGatewayTargetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    gateway_identifier: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credential_provider_configuration:
        Option<Vec<BedrockagentcoreGatewayTargetCredentialProviderConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_configuration: Option<Vec<BedrockagentcoreGatewayTargetTargetConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BedrockagentcoreGatewayTargetTimeoutsEl>,
    dynamic: BedrockagentcoreGatewayTargetDynamic,
}
struct BedrockagentcoreGatewayTarget_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockagentcoreGatewayTargetData>,
}
#[derive(Clone)]
pub struct BedrockagentcoreGatewayTarget(Rc<BedrockagentcoreGatewayTarget_>);
impl BedrockagentcoreGatewayTarget {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `credential_provider_configuration`.\n"]
    pub fn set_credential_provider_configuration(
        self,
        v: impl Into<BlockAssignable<BedrockagentcoreGatewayTargetCredentialProviderConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().credential_provider_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .credential_provider_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target_configuration`.\n"]
    pub fn set_target_configuration(
        self,
        v: impl Into<BlockAssignable<BedrockagentcoreGatewayTargetTargetConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BedrockagentcoreGatewayTargetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_identifier` after provisioning.\n"]
    pub fn gateway_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_id` after provisioning.\n"]
    pub fn target_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `credential_provider_configuration` after provisioning.\n"]
    pub fn credential_provider_configuration(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayTargetCredentialProviderConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.credential_provider_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_configuration` after provisioning.\n"]
    pub fn target_configuration(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayTargetTargetConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreGatewayTargetTimeoutsElRef {
        BedrockagentcoreGatewayTargetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BedrockagentcoreGatewayTarget {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BedrockagentcoreGatewayTarget {}
impl ToListMappable for BedrockagentcoreGatewayTarget {
    type O = ListRef<BedrockagentcoreGatewayTargetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BedrockagentcoreGatewayTarget_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrockagentcore_gateway_target".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBedrockagentcoreGatewayTarget {
    pub tf_id: String,
    #[doc = ""]
    pub gateway_identifier: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTarget {
    pub fn build(self, stack: &mut Stack) -> BedrockagentcoreGatewayTarget {
        let out = BedrockagentcoreGatewayTarget(Rc::new(BedrockagentcoreGatewayTarget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockagentcoreGatewayTargetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                gateway_identifier: self.gateway_identifier,
                name: self.name,
                region: core::default::Default::default(),
                credential_provider_configuration: core::default::Default::default(),
                target_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BedrockagentcoreGatewayTargetRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BedrockagentcoreGatewayTargetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_identifier` after provisioning.\n"]
    pub fn gateway_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_id` after provisioning.\n"]
    pub fn target_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `credential_provider_configuration` after provisioning.\n"]
    pub fn credential_provider_configuration(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayTargetCredentialProviderConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.credential_provider_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_configuration` after provisioning.\n"]
    pub fn target_configuration(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayTargetTargetConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreGatewayTargetTimeoutsElRef {
        BedrockagentcoreGatewayTargetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetCredentialProviderConfigurationElApiKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    credential_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credential_parameter_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credential_prefix: Option<PrimField<String>>,
    provider_arn: PrimField<String>,
}
impl BedrockagentcoreGatewayTargetCredentialProviderConfigurationElApiKeyEl {
    #[doc = "Set the field `credential_location`.\n"]
    pub fn set_credential_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.credential_location = Some(v.into());
        self
    }
    #[doc = "Set the field `credential_parameter_name`.\n"]
    pub fn set_credential_parameter_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.credential_parameter_name = Some(v.into());
        self
    }
    #[doc = "Set the field `credential_prefix`.\n"]
    pub fn set_credential_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.credential_prefix = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockagentcoreGatewayTargetCredentialProviderConfigurationElApiKeyEl {
    type O =
        BlockAssignable<BedrockagentcoreGatewayTargetCredentialProviderConfigurationElApiKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetCredentialProviderConfigurationElApiKeyEl {
    #[doc = ""]
    pub provider_arn: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetCredentialProviderConfigurationElApiKeyEl {
    pub fn build(self) -> BedrockagentcoreGatewayTargetCredentialProviderConfigurationElApiKeyEl {
        BedrockagentcoreGatewayTargetCredentialProviderConfigurationElApiKeyEl {
            credential_location: core::default::Default::default(),
            credential_parameter_name: core::default::Default::default(),
            credential_prefix: core::default::Default::default(),
            provider_arn: self.provider_arn,
        }
    }
}
pub struct BedrockagentcoreGatewayTargetCredentialProviderConfigurationElApiKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetCredentialProviderConfigurationElApiKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayTargetCredentialProviderConfigurationElApiKeyElRef {
        BedrockagentcoreGatewayTargetCredentialProviderConfigurationElApiKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTargetCredentialProviderConfigurationElApiKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `credential_location` after provisioning.\n"]
    pub fn credential_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credential_location", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `credential_parameter_name` after provisioning.\n"]
    pub fn credential_parameter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credential_parameter_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `credential_prefix` after provisioning.\n"]
    pub fn credential_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credential_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `provider_arn` after provisioning.\n"]
    pub fn provider_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetCredentialProviderConfigurationElGatewayIamRoleEl {}
impl BedrockagentcoreGatewayTargetCredentialProviderConfigurationElGatewayIamRoleEl {}
impl ToListMappable
    for BedrockagentcoreGatewayTargetCredentialProviderConfigurationElGatewayIamRoleEl
{
    type O = BlockAssignable<
        BedrockagentcoreGatewayTargetCredentialProviderConfigurationElGatewayIamRoleEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetCredentialProviderConfigurationElGatewayIamRoleEl {}
impl BuildBedrockagentcoreGatewayTargetCredentialProviderConfigurationElGatewayIamRoleEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTargetCredentialProviderConfigurationElGatewayIamRoleEl {
        BedrockagentcoreGatewayTargetCredentialProviderConfigurationElGatewayIamRoleEl {}
    }
}
pub struct BedrockagentcoreGatewayTargetCredentialProviderConfigurationElGatewayIamRoleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetCredentialProviderConfigurationElGatewayIamRoleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayTargetCredentialProviderConfigurationElGatewayIamRoleElRef {
        BedrockagentcoreGatewayTargetCredentialProviderConfigurationElGatewayIamRoleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTargetCredentialProviderConfigurationElGatewayIamRoleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetCredentialProviderConfigurationElOauthEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_parameters: Option<RecField<PrimField<String>>>,
    provider_arn: PrimField<String>,
    scopes: SetField<PrimField<String>>,
}
impl BedrockagentcoreGatewayTargetCredentialProviderConfigurationElOauthEl {
    #[doc = "Set the field `custom_parameters`.\n"]
    pub fn set_custom_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.custom_parameters = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockagentcoreGatewayTargetCredentialProviderConfigurationElOauthEl {
    type O = BlockAssignable<BedrockagentcoreGatewayTargetCredentialProviderConfigurationElOauthEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetCredentialProviderConfigurationElOauthEl {
    #[doc = ""]
    pub provider_arn: PrimField<String>,
    #[doc = ""]
    pub scopes: SetField<PrimField<String>>,
}
impl BuildBedrockagentcoreGatewayTargetCredentialProviderConfigurationElOauthEl {
    pub fn build(self) -> BedrockagentcoreGatewayTargetCredentialProviderConfigurationElOauthEl {
        BedrockagentcoreGatewayTargetCredentialProviderConfigurationElOauthEl {
            custom_parameters: core::default::Default::default(),
            provider_arn: self.provider_arn,
            scopes: self.scopes,
        }
    }
}
pub struct BedrockagentcoreGatewayTargetCredentialProviderConfigurationElOauthElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetCredentialProviderConfigurationElOauthElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayTargetCredentialProviderConfigurationElOauthElRef {
        BedrockagentcoreGatewayTargetCredentialProviderConfigurationElOauthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTargetCredentialProviderConfigurationElOauthElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_parameters` after provisioning.\n"]
    pub fn custom_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.custom_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `provider_arn` after provisioning.\n"]
    pub fn provider_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
}
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetCredentialProviderConfigurationElDynamic {
    api_key: Option<
        DynamicBlock<BedrockagentcoreGatewayTargetCredentialProviderConfigurationElApiKeyEl>,
    >,
    gateway_iam_role: Option<
        DynamicBlock<
            BedrockagentcoreGatewayTargetCredentialProviderConfigurationElGatewayIamRoleEl,
        >,
    >,
    oauth:
        Option<DynamicBlock<BedrockagentcoreGatewayTargetCredentialProviderConfigurationElOauthEl>>,
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetCredentialProviderConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<Vec<BedrockagentcoreGatewayTargetCredentialProviderConfigurationElApiKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway_iam_role:
        Option<Vec<BedrockagentcoreGatewayTargetCredentialProviderConfigurationElGatewayIamRoleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth: Option<Vec<BedrockagentcoreGatewayTargetCredentialProviderConfigurationElOauthEl>>,
    dynamic: BedrockagentcoreGatewayTargetCredentialProviderConfigurationElDynamic,
}
impl BedrockagentcoreGatewayTargetCredentialProviderConfigurationEl {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(
        mut self,
        v: impl Into<
            BlockAssignable<BedrockagentcoreGatewayTargetCredentialProviderConfigurationElApiKeyEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.api_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.api_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gateway_iam_role`.\n"]
    pub fn set_gateway_iam_role(
        mut self,
        v: impl Into<
            BlockAssignable<
                BedrockagentcoreGatewayTargetCredentialProviderConfigurationElGatewayIamRoleEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gateway_iam_role = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gateway_iam_role = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oauth`.\n"]
    pub fn set_oauth(
        mut self,
        v: impl Into<
            BlockAssignable<BedrockagentcoreGatewayTargetCredentialProviderConfigurationElOauthEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BedrockagentcoreGatewayTargetCredentialProviderConfigurationEl {
    type O = BlockAssignable<BedrockagentcoreGatewayTargetCredentialProviderConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetCredentialProviderConfigurationEl {}
impl BuildBedrockagentcoreGatewayTargetCredentialProviderConfigurationEl {
    pub fn build(self) -> BedrockagentcoreGatewayTargetCredentialProviderConfigurationEl {
        BedrockagentcoreGatewayTargetCredentialProviderConfigurationEl {
            api_key: core::default::Default::default(),
            gateway_iam_role: core::default::Default::default(),
            oauth: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BedrockagentcoreGatewayTargetCredentialProviderConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetCredentialProviderConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayTargetCredentialProviderConfigurationElRef {
        BedrockagentcoreGatewayTargetCredentialProviderConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTargetCredentialProviderConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayTargetCredentialProviderConfigurationElApiKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
    #[doc = "Get a reference to the value of field `gateway_iam_role` after provisioning.\n"]
    pub fn gateway_iam_role(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayTargetCredentialProviderConfigurationElGatewayIamRoleElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gateway_iam_role", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth` after provisioning.\n"]
    pub fn oauth(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayTargetCredentialProviderConfigurationElOauthElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oauth", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElItemsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items_json: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties_json: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElItemsEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `items_json`.\n"] pub fn set_items_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . items_json = Some (v . into ()) ; self } # [doc = "Set the field `properties_json`.\n"] pub fn set_properties_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . properties_json = Some (v . into ()) ; self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElItemsEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElItemsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElItemsEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElItemsEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElItemsEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElItemsEl { description : core :: default :: Default :: default () , items_json : core :: default :: Default :: default () , properties_json : core :: default :: Default :: default () , type_ : self . type_ , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElItemsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElItemsElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElItemsElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElItemsElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElItemsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `items_json` after provisioning.\n"] pub fn items_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.items_json" , self . base)) } # [doc = "Get a reference to the value of field `properties_json` after provisioning.\n"] pub fn properties_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.properties_json" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElPropertyEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items_json: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties_json: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<PrimField<bool>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElPropertyEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `items_json`.\n"] pub fn set_items_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . items_json = Some (v . into ()) ; self } # [doc = "Set the field `properties_json`.\n"] pub fn set_properties_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . properties_json = Some (v . into ()) ; self } # [doc = "Set the field `required`.\n"] pub fn set_required (mut self , v : impl Into < PrimField < bool > >) -> Self { self . required = Some (v . into ()) ; self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElPropertyEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElPropertyEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElPropertyEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElPropertyEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElPropertyEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElPropertyEl { description : core :: default :: Default :: default () , items_json : core :: default :: Default :: default () , name : self . name , properties_json : core :: default :: Default :: default () , required : core :: default :: Default :: default () , type_ : self . type_ , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElPropertyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElPropertyElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElPropertyElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElPropertyElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElPropertyElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `items_json` after provisioning.\n"] pub fn items_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.items_json" , self . base)) } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `properties_json` after provisioning.\n"] pub fn properties_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.properties_json" , self . base)) } # [doc = "Get a reference to the value of field `required` after provisioning.\n"] pub fn required (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.required" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } }
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElDynamic { items : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElItemsEl >> , property : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElPropertyEl >> , }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsEl { # [serde (skip_serializing_if = "Option::is_none")] description : Option < PrimField < String > > , # [serde (rename = "type")] type_ : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] items : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElItemsEl > > , # [serde (skip_serializing_if = "Option::is_none")] property : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElPropertyEl > > , dynamic : BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElDynamic , }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `items`.\n"] pub fn set_items (mut self , v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElItemsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . items = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . items = Some (d) ; } } self } # [doc = "Set the field `property`.\n"] pub fn set_property (mut self , v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElPropertyEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . property = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . property = Some (d) ; } } self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsEl { description : core :: default :: Default :: default () , type_ : self . type_ , items : core :: default :: Default :: default () , property : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } # [doc = "Get a reference to the value of field `items` after provisioning.\n"] pub fn items (& self) -> ListRef < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElItemsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.items" , self . base)) } }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElItemsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items_json: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties_json: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElItemsEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `items_json`.\n"] pub fn set_items_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . items_json = Some (v . into ()) ; self } # [doc = "Set the field `properties_json`.\n"] pub fn set_properties_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . properties_json = Some (v . into ()) ; self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElItemsEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElItemsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElItemsEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElItemsEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElItemsEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElItemsEl { description : core :: default :: Default :: default () , items_json : core :: default :: Default :: default () , properties_json : core :: default :: Default :: default () , type_ : self . type_ , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElItemsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElItemsElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElItemsElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElItemsElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElItemsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `items_json` after provisioning.\n"] pub fn items_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.items_json" , self . base)) } # [doc = "Get a reference to the value of field `properties_json` after provisioning.\n"] pub fn properties_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.properties_json" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElPropertyEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items_json: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties_json: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<PrimField<bool>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElPropertyEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `items_json`.\n"] pub fn set_items_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . items_json = Some (v . into ()) ; self } # [doc = "Set the field `properties_json`.\n"] pub fn set_properties_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . properties_json = Some (v . into ()) ; self } # [doc = "Set the field `required`.\n"] pub fn set_required (mut self , v : impl Into < PrimField < bool > >) -> Self { self . required = Some (v . into ()) ; self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElPropertyEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElPropertyEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElPropertyEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElPropertyEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElPropertyEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElPropertyEl { description : core :: default :: Default :: default () , items_json : core :: default :: Default :: default () , name : self . name , properties_json : core :: default :: Default :: default () , required : core :: default :: Default :: default () , type_ : self . type_ , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElPropertyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElPropertyElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElPropertyElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElPropertyElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElPropertyElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `items_json` after provisioning.\n"] pub fn items_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.items_json" , self . base)) } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `properties_json` after provisioning.\n"] pub fn properties_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.properties_json" , self . base)) } # [doc = "Get a reference to the value of field `required` after provisioning.\n"] pub fn required (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.required" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } }
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElDynamic { items : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElItemsEl >> , property : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElPropertyEl >> , }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsEl { # [serde (skip_serializing_if = "Option::is_none")] description : Option < PrimField < String > > , # [serde (rename = "type")] type_ : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] items : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElItemsEl > > , # [serde (skip_serializing_if = "Option::is_none")] property : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElPropertyEl > > , dynamic : BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElDynamic , }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `items`.\n"] pub fn set_items (mut self , v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElItemsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . items = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . items = Some (d) ; } } self } # [doc = "Set the field `property`.\n"] pub fn set_property (mut self , v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElPropertyEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . property = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . property = Some (d) ; } } self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsEl { description : core :: default :: Default :: default () , type_ : self . type_ , items : core :: default :: Default :: default () , property : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } # [doc = "Get a reference to the value of field `items` after provisioning.\n"] pub fn items (& self) -> ListRef < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElItemsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.items" , self . base)) } }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElPropertyEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items_json: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties_json: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<PrimField<bool>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElPropertyEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `items_json`.\n"] pub fn set_items_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . items_json = Some (v . into ()) ; self } # [doc = "Set the field `properties_json`.\n"] pub fn set_properties_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . properties_json = Some (v . into ()) ; self } # [doc = "Set the field `required`.\n"] pub fn set_required (mut self , v : impl Into < PrimField < bool > >) -> Self { self . required = Some (v . into ()) ; self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElPropertyEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElPropertyEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElPropertyEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElPropertyEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElPropertyEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElPropertyEl { description : core :: default :: Default :: default () , items_json : core :: default :: Default :: default () , name : self . name , properties_json : core :: default :: Default :: default () , required : core :: default :: Default :: default () , type_ : self . type_ , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElPropertyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElPropertyElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElPropertyElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElPropertyElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElPropertyElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `items_json` after provisioning.\n"] pub fn items_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.items_json" , self . base)) } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `properties_json` after provisioning.\n"] pub fn properties_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.properties_json" , self . base)) } # [doc = "Get a reference to the value of field `required` after provisioning.\n"] pub fn required (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.required" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } }
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElDynamic { items : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsEl >> , property : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElPropertyEl >> , }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyEl { # [serde (skip_serializing_if = "Option::is_none")] description : Option < PrimField < String > > , name : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] required : Option < PrimField < bool > > , # [serde (rename = "type")] type_ : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] items : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsEl > > , # [serde (skip_serializing_if = "Option::is_none")] property : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElPropertyEl > > , dynamic : BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElDynamic , }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `required`.\n"] pub fn set_required (mut self , v : impl Into < PrimField < bool > >) -> Self { self . required = Some (v . into ()) ; self } # [doc = "Set the field `items`.\n"] pub fn set_items (mut self , v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . items = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . items = Some (d) ; } } self } # [doc = "Set the field `property`.\n"] pub fn set_property (mut self , v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElPropertyEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . property = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . property = Some (d) ; } } self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyEl { description : core :: default :: Default :: default () , name : self . name , required : core :: default :: Default :: default () , type_ : self . type_ , items : core :: default :: Default :: default () , property : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `required` after provisioning.\n"] pub fn required (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.required" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } # [doc = "Get a reference to the value of field `items` after provisioning.\n"] pub fn items (& self) -> ListRef < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyElItemsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.items" , self . base)) } }
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElDynamic { items : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsEl >> , property : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyEl >> , }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaEl { # [serde (skip_serializing_if = "Option::is_none")] description : Option < PrimField < String > > , # [serde (rename = "type")] type_ : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] items : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsEl > > , # [serde (skip_serializing_if = "Option::is_none")] property : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyEl > > , dynamic : BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElDynamic , }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `items`.\n"] pub fn set_items (mut self , v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . items = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . items = Some (d) ; } } self } # [doc = "Set the field `property`.\n"] pub fn set_property (mut self , v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElPropertyEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . property = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . property = Some (d) ; } } self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaEl { description : core :: default :: Default :: default () , type_ : self . type_ , items : core :: default :: Default :: default () , property : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } # [doc = "Get a reference to the value of field `items` after provisioning.\n"] pub fn items (& self) -> ListRef < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElItemsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.items" , self . base)) } }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElItemsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items_json: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties_json: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElItemsEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `items_json`.\n"] pub fn set_items_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . items_json = Some (v . into ()) ; self } # [doc = "Set the field `properties_json`.\n"] pub fn set_properties_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . properties_json = Some (v . into ()) ; self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElItemsEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElItemsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElItemsEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElItemsEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElItemsEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElItemsEl { description : core :: default :: Default :: default () , items_json : core :: default :: Default :: default () , properties_json : core :: default :: Default :: default () , type_ : self . type_ , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElItemsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElItemsElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElItemsElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElItemsElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElItemsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `items_json` after provisioning.\n"] pub fn items_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.items_json" , self . base)) } # [doc = "Get a reference to the value of field `properties_json` after provisioning.\n"] pub fn properties_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.properties_json" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElPropertyEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items_json: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties_json: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<PrimField<bool>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElPropertyEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `items_json`.\n"] pub fn set_items_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . items_json = Some (v . into ()) ; self } # [doc = "Set the field `properties_json`.\n"] pub fn set_properties_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . properties_json = Some (v . into ()) ; self } # [doc = "Set the field `required`.\n"] pub fn set_required (mut self , v : impl Into < PrimField < bool > >) -> Self { self . required = Some (v . into ()) ; self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElPropertyEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElPropertyEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElPropertyEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElPropertyEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElPropertyEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElPropertyEl { description : core :: default :: Default :: default () , items_json : core :: default :: Default :: default () , name : self . name , properties_json : core :: default :: Default :: default () , required : core :: default :: Default :: default () , type_ : self . type_ , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElPropertyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElPropertyElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElPropertyElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElPropertyElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElPropertyElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `items_json` after provisioning.\n"] pub fn items_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.items_json" , self . base)) } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `properties_json` after provisioning.\n"] pub fn properties_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.properties_json" , self . base)) } # [doc = "Get a reference to the value of field `required` after provisioning.\n"] pub fn required (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.required" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } }
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElDynamic { items : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElItemsEl >> , property : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElPropertyEl >> , }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsEl { # [serde (skip_serializing_if = "Option::is_none")] description : Option < PrimField < String > > , # [serde (rename = "type")] type_ : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] items : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElItemsEl > > , # [serde (skip_serializing_if = "Option::is_none")] property : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElPropertyEl > > , dynamic : BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElDynamic , }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `items`.\n"] pub fn set_items (mut self , v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElItemsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . items = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . items = Some (d) ; } } self } # [doc = "Set the field `property`.\n"] pub fn set_property (mut self , v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElPropertyEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . property = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . property = Some (d) ; } } self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsEl { description : core :: default :: Default :: default () , type_ : self . type_ , items : core :: default :: Default :: default () , property : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } # [doc = "Get a reference to the value of field `items` after provisioning.\n"] pub fn items (& self) -> ListRef < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElItemsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.items" , self . base)) } }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElItemsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items_json: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties_json: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElItemsEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `items_json`.\n"] pub fn set_items_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . items_json = Some (v . into ()) ; self } # [doc = "Set the field `properties_json`.\n"] pub fn set_properties_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . properties_json = Some (v . into ()) ; self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElItemsEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElItemsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElItemsEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElItemsEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElItemsEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElItemsEl { description : core :: default :: Default :: default () , items_json : core :: default :: Default :: default () , properties_json : core :: default :: Default :: default () , type_ : self . type_ , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElItemsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElItemsElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElItemsElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElItemsElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElItemsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `items_json` after provisioning.\n"] pub fn items_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.items_json" , self . base)) } # [doc = "Get a reference to the value of field `properties_json` after provisioning.\n"] pub fn properties_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.properties_json" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElPropertyEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items_json: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties_json: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<PrimField<bool>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElPropertyEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `items_json`.\n"] pub fn set_items_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . items_json = Some (v . into ()) ; self } # [doc = "Set the field `properties_json`.\n"] pub fn set_properties_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . properties_json = Some (v . into ()) ; self } # [doc = "Set the field `required`.\n"] pub fn set_required (mut self , v : impl Into < PrimField < bool > >) -> Self { self . required = Some (v . into ()) ; self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElPropertyEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElPropertyEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElPropertyEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElPropertyEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElPropertyEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElPropertyEl { description : core :: default :: Default :: default () , items_json : core :: default :: Default :: default () , name : self . name , properties_json : core :: default :: Default :: default () , required : core :: default :: Default :: default () , type_ : self . type_ , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElPropertyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElPropertyElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElPropertyElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElPropertyElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElPropertyElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `items_json` after provisioning.\n"] pub fn items_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.items_json" , self . base)) } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `properties_json` after provisioning.\n"] pub fn properties_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.properties_json" , self . base)) } # [doc = "Get a reference to the value of field `required` after provisioning.\n"] pub fn required (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.required" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } }
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElDynamic { items : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElItemsEl >> , property : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElPropertyEl >> , }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsEl { # [serde (skip_serializing_if = "Option::is_none")] description : Option < PrimField < String > > , # [serde (rename = "type")] type_ : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] items : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElItemsEl > > , # [serde (skip_serializing_if = "Option::is_none")] property : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElPropertyEl > > , dynamic : BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElDynamic , }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `items`.\n"] pub fn set_items (mut self , v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElItemsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . items = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . items = Some (d) ; } } self } # [doc = "Set the field `property`.\n"] pub fn set_property (mut self , v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElPropertyEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . property = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . property = Some (d) ; } } self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsEl { description : core :: default :: Default :: default () , type_ : self . type_ , items : core :: default :: Default :: default () , property : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } # [doc = "Get a reference to the value of field `items` after provisioning.\n"] pub fn items (& self) -> ListRef < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElItemsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.items" , self . base)) } }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElPropertyEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items_json: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties_json: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<PrimField<bool>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElPropertyEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `items_json`.\n"] pub fn set_items_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . items_json = Some (v . into ()) ; self } # [doc = "Set the field `properties_json`.\n"] pub fn set_properties_json (mut self , v : impl Into < PrimField < String > >) -> Self { self . properties_json = Some (v . into ()) ; self } # [doc = "Set the field `required`.\n"] pub fn set_required (mut self , v : impl Into < PrimField < bool > >) -> Self { self . required = Some (v . into ()) ; self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElPropertyEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElPropertyEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElPropertyEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElPropertyEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElPropertyEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElPropertyEl { description : core :: default :: Default :: default () , items_json : core :: default :: Default :: default () , name : self . name , properties_json : core :: default :: Default :: default () , required : core :: default :: Default :: default () , type_ : self . type_ , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElPropertyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElPropertyElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElPropertyElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElPropertyElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElPropertyElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `items_json` after provisioning.\n"] pub fn items_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.items_json" , self . base)) } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `properties_json` after provisioning.\n"] pub fn properties_json (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.properties_json" , self . base)) } # [doc = "Get a reference to the value of field `required` after provisioning.\n"] pub fn required (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.required" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } }
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElDynamic { items : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsEl >> , property : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElPropertyEl >> , }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyEl { # [serde (skip_serializing_if = "Option::is_none")] description : Option < PrimField < String > > , name : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] required : Option < PrimField < bool > > , # [serde (rename = "type")] type_ : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] items : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsEl > > , # [serde (skip_serializing_if = "Option::is_none")] property : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElPropertyEl > > , dynamic : BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElDynamic , }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `required`.\n"] pub fn set_required (mut self , v : impl Into < PrimField < bool > >) -> Self { self . required = Some (v . into ()) ; self } # [doc = "Set the field `items`.\n"] pub fn set_items (mut self , v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . items = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . items = Some (d) ; } } self } # [doc = "Set the field `property`.\n"] pub fn set_property (mut self , v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElPropertyEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . property = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . property = Some (d) ; } } self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyEl
{
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyEl { description : core :: default :: Default :: default () , name : self . name , required : core :: default :: Default :: default () , type_ : self . type_ , items : core :: default :: Default :: default () , property : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `required` after provisioning.\n"] pub fn required (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.required" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } # [doc = "Get a reference to the value of field `items` after provisioning.\n"] pub fn items (& self) -> ListRef < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyElItemsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.items" , self . base)) } }
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElDynamic { items : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsEl >> , property : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyEl >> , }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaEl { # [serde (skip_serializing_if = "Option::is_none")] description : Option < PrimField < String > > , # [serde (rename = "type")] type_ : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] items : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsEl > > , # [serde (skip_serializing_if = "Option::is_none")] property : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyEl > > , dynamic : BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElDynamic , }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaEl { # [doc = "Set the field `description`.\n"] pub fn set_description (mut self , v : impl Into < PrimField < String > >) -> Self { self . description = Some (v . into ()) ; self } # [doc = "Set the field `items`.\n"] pub fn set_items (mut self , v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . items = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . items = Some (d) ; } } self } # [doc = "Set the field `property`.\n"] pub fn set_property (mut self , v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElPropertyEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . property = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . property = Some (d) ; } } self } }
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaEl { type O = BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaEl { pub fn build (self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaEl { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaEl { description : core :: default :: Default :: default () , type_ : self . type_ , items : core :: default :: Default :: default () , property : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `description` after provisioning.\n"] pub fn description (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.description" , self . base)) } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } # [doc = "Get a reference to the value of field `items` after provisioning.\n"] pub fn items (& self) -> ListRef < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElItemsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.items" , self . base)) } }
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElDynamic { input_schema : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaEl >> , output_schema : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaEl >> , }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadEl { description : PrimField < String > , name : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] input_schema : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaEl > > , # [serde (skip_serializing_if = "Option::is_none")] output_schema : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaEl > > , dynamic : BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElDynamic , }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadEl {
    #[doc = "Set the field `input_schema`.\n"]
    pub fn set_input_schema(
        mut self,
        v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_schema = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_schema = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `output_schema`.\n"]
    pub fn set_output_schema(
        mut self,
        v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output_schema = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output_schema = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadEl
{
    type O = BlockAssignable<
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadEl
{
    #[doc = ""]
    pub description: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl
    BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadEl
{
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadEl
    {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadEl {
            description: self.description,
            name: self.name,
            input_schema: core::default::Default::default(),
            output_schema: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElRef { fn new (shared : StackShared , base : String) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElRef { BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElRef { shared : shared , base : base . to_string () , } } }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `input_schema` after provisioning.\n"]    pub fn input_schema (& self) -> ListRef < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElInputSchemaElRef >{
        ListRef::new(self.shared().clone(), format!("{}.input_schema", self.base))
    }
    #[doc = "Get a reference to the value of field `output_schema` after provisioning.\n"]    pub fn output_schema (& self) -> ListRef < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElOutputSchemaElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_schema", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElS3El {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_owner_account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElS3El {
    #[doc = "Set the field `bucket_owner_account_id`.\n"]
    pub fn set_bucket_owner_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_owner_account_id = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable
    for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElS3El
{
    type O = BlockAssignable<
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElS3El,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElS3El {}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElS3El {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElS3El {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElS3El {
            bucket_owner_account_id: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElS3ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElS3ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElS3ElRef {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_owner_account_id` after provisioning.\n"]
    pub fn bucket_owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_owner_account_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElDynamic { inline_payload : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadEl >> , s3 : Option < DynamicBlock < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElS3El >> , }
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaEl { # [serde (skip_serializing_if = "Option::is_none")] inline_payload : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadEl > > , # [serde (skip_serializing_if = "Option::is_none")] s3 : Option < Vec < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElS3El > > , dynamic : BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElDynamic , }
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaEl {
    #[doc = "Set the field `inline_payload`.\n"]
    pub fn set_inline_payload(
        mut self,
        v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inline_payload = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_payload = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl Into<
            BlockAssignable<
                BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElS3El,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaEl
{
    type O = BlockAssignable<
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaEl {}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaEl {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaEl {
            inline_payload: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElRef {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `inline_payload` after provisioning.\n"]    pub fn inline_payload (& self) -> ListRef < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElInlinePayloadElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.inline_payload", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElS3ElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElDynamic {
    tool_schema: Option<
        DynamicBlock<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaEl>,
    >,
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaEl {
    lambda_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_schema:
        Option<Vec<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaEl>>,
    dynamic: BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElDynamic,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaEl {
    #[doc = "Set the field `tool_schema`.\n"]
    pub fn set_tool_schema(
        mut self,
        v: impl Into<
            BlockAssignable<
                BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tool_schema = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_schema = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaEl {
    type O = BlockAssignable<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaEl {
    #[doc = ""]
    pub lambda_arn: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaEl {
    pub fn build(self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaEl {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaEl {
            lambda_arn: self.lambda_arn,
            tool_schema: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElRef {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `tool_schema` after provisioning.\n"]
    pub fn tool_schema(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElToolSchemaElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.tool_schema", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElMcpServerEl {
    endpoint: PrimField<String>,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElMcpServerEl {}
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElMcpServerEl {
    type O = BlockAssignable<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElMcpServerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElMcpServerEl {
    #[doc = ""]
    pub endpoint: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElMcpServerEl {
    pub fn build(self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElMcpServerEl {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElMcpServerEl {
            endpoint: self.endpoint,
        }
    }
}
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElMcpServerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElMcpServerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElMcpServerElRef {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElMcpServerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElMcpServerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElInlinePayloadEl {
    payload: PrimField<String>,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElInlinePayloadEl {}
impl ToListMappable
    for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElInlinePayloadEl
{
    type O = BlockAssignable<
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElInlinePayloadEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElInlinePayloadEl
{
    #[doc = ""]
    pub payload: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElInlinePayloadEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElInlinePayloadEl {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElInlinePayloadEl {
            payload: self.payload,
        }
    }
}
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElInlinePayloadElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElInlinePayloadElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElInlinePayloadElRef
    {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElInlinePayloadElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElInlinePayloadElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `payload` after provisioning.\n"]
    pub fn payload(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElS3El {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_owner_account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElS3El {
    #[doc = "Set the field `bucket_owner_account_id`.\n"]
    pub fn set_bucket_owner_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_owner_account_id = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElS3El {
    type O =
        BlockAssignable<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElS3El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElS3El {}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElS3El {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElS3El {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElS3El {
            bucket_owner_account_id: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElS3ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElS3ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElS3ElRef {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_owner_account_id` after provisioning.\n"]
    pub fn bucket_owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_owner_account_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElDynamic {
    inline_payload: Option<
        DynamicBlock<
            BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElInlinePayloadEl,
        >,
    >,
    s3: Option<
        DynamicBlock<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElS3El>,
    >,
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_payload: Option<
        Vec<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElInlinePayloadEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElS3El>>,
    dynamic: BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElDynamic,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaEl {
    #[doc = "Set the field `inline_payload`.\n"]
    pub fn set_inline_payload(
        mut self,
        v : impl Into < BlockAssignable < BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElInlinePayloadEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inline_payload = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_payload = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl Into<
            BlockAssignable<
                BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElS3El,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaEl {
    type O =
        BlockAssignable<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaEl {}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaEl {
    pub fn build(self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaEl {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaEl {
            inline_payload: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElRef {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `inline_payload` after provisioning.\n"]
    pub fn inline_payload(
        &self,
    ) -> ListRef<
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElInlinePayloadElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inline_payload", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElS3ElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElInlinePayloadEl {
    payload: PrimField<String>,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElInlinePayloadEl {}
impl ToListMappable
    for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElInlinePayloadEl
{
    type O = BlockAssignable<
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElInlinePayloadEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElInlinePayloadEl
{
    #[doc = ""]
    pub payload: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElInlinePayloadEl {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElInlinePayloadEl {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElInlinePayloadEl {
            payload: self.payload,
        }
    }
}
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElInlinePayloadElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElInlinePayloadElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElInlinePayloadElRef
    {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElInlinePayloadElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElInlinePayloadElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `payload` after provisioning.\n"]
    pub fn payload(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElS3El {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_owner_account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElS3El {
    #[doc = "Set the field `bucket_owner_account_id`.\n"]
    pub fn set_bucket_owner_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_owner_account_id = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElS3El {
    type O =
        BlockAssignable<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElS3El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElS3El {}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElS3El {
    pub fn build(self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElS3El {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElS3El {
            bucket_owner_account_id: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElS3ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElS3ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElS3ElRef {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_owner_account_id` after provisioning.\n"]
    pub fn bucket_owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_owner_account_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElDynamic {
    inline_payload: Option<
        DynamicBlock<
            BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElInlinePayloadEl,
        >,
    >,
    s3: Option<
        DynamicBlock<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElS3El>,
    >,
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_payload: Option<
        Vec<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElInlinePayloadEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElS3El>>,
    dynamic: BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElDynamic,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelEl {
    #[doc = "Set the field `inline_payload`.\n"]
    pub fn set_inline_payload(
        mut self,
        v: impl Into<
            BlockAssignable<
                BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElInlinePayloadEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inline_payload = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_payload = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl Into<
            BlockAssignable<
                BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElS3El,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelEl {
    type O = BlockAssignable<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelEl {}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelEl {
    pub fn build(self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelEl {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelEl {
            inline_payload: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElRef {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `inline_payload` after provisioning.\n"]
    pub fn inline_payload(
        &self,
    ) -> ListRef<
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElInlinePayloadElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inline_payload", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElDynamic {
    lambda: Option<DynamicBlock<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaEl>>,
    mcp_server:
        Option<DynamicBlock<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElMcpServerEl>>,
    open_api_schema: Option<
        DynamicBlock<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaEl>,
    >,
    smithy_model:
        Option<DynamicBlock<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelEl>>,
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda: Option<Vec<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mcp_server: Option<Vec<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElMcpServerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_api_schema:
        Option<Vec<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    smithy_model: Option<Vec<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelEl>>,
    dynamic: BedrockagentcoreGatewayTargetTargetConfigurationElMcpElDynamic,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpEl {
    #[doc = "Set the field `lambda`.\n"]
    pub fn set_lambda(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mcp_server`.\n"]
    pub fn set_mcp_server(
        mut self,
        v: impl Into<
            BlockAssignable<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElMcpServerEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mcp_server = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mcp_server = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `open_api_schema`.\n"]
    pub fn set_open_api_schema(
        mut self,
        v: impl Into<
            BlockAssignable<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.open_api_schema = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.open_api_schema = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `smithy_model`.\n"]
    pub fn set_smithy_model(
        mut self,
        v: impl Into<
            BlockAssignable<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.smithy_model = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.smithy_model = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationElMcpEl {
    type O = BlockAssignable<BedrockagentcoreGatewayTargetTargetConfigurationElMcpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpEl {}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationElMcpEl {
    pub fn build(self) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpEl {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpEl {
            lambda: core::default::Default::default(),
            mcp_server: core::default::Default::default(),
            open_api_schema: core::default::Default::default(),
            smithy_model: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElMcpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElMcpElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElMcpElRef {
        BedrockagentcoreGatewayTargetTargetConfigurationElMcpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElMcpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `lambda` after provisioning.\n"]
    pub fn lambda(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElLambdaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lambda", self.base))
    }
    #[doc = "Get a reference to the value of field `mcp_server` after provisioning.\n"]
    pub fn mcp_server(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElMcpServerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mcp_server", self.base))
    }
    #[doc = "Get a reference to the value of field `open_api_schema` after provisioning.\n"]
    pub fn open_api_schema(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElOpenApiSchemaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.open_api_schema", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `smithy_model` after provisioning.\n"]
    pub fn smithy_model(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElSmithyModelElRef> {
        ListRef::new(self.shared().clone(), format!("{}.smithy_model", self.base))
    }
}
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetTargetConfigurationElDynamic {
    mcp: Option<DynamicBlock<BedrockagentcoreGatewayTargetTargetConfigurationElMcpEl>>,
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTargetConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mcp: Option<Vec<BedrockagentcoreGatewayTargetTargetConfigurationElMcpEl>>,
    dynamic: BedrockagentcoreGatewayTargetTargetConfigurationElDynamic,
}
impl BedrockagentcoreGatewayTargetTargetConfigurationEl {
    #[doc = "Set the field `mcp`.\n"]
    pub fn set_mcp(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentcoreGatewayTargetTargetConfigurationElMcpEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mcp = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mcp = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BedrockagentcoreGatewayTargetTargetConfigurationEl {
    type O = BlockAssignable<BedrockagentcoreGatewayTargetTargetConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetTargetConfigurationEl {}
impl BuildBedrockagentcoreGatewayTargetTargetConfigurationEl {
    pub fn build(self) -> BedrockagentcoreGatewayTargetTargetConfigurationEl {
        BedrockagentcoreGatewayTargetTargetConfigurationEl {
            mcp: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BedrockagentcoreGatewayTargetTargetConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTargetConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayTargetTargetConfigurationElRef {
        BedrockagentcoreGatewayTargetTargetConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTargetTargetConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mcp` after provisioning.\n"]
    pub fn mcp(&self) -> ListRef<BedrockagentcoreGatewayTargetTargetConfigurationElMcpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mcp", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTargetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BedrockagentcoreGatewayTargetTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockagentcoreGatewayTargetTimeoutsEl {
    type O = BlockAssignable<BedrockagentcoreGatewayTargetTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTargetTimeoutsEl {}
impl BuildBedrockagentcoreGatewayTargetTimeoutsEl {
    pub fn build(self) -> BedrockagentcoreGatewayTargetTimeoutsEl {
        BedrockagentcoreGatewayTargetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BedrockagentcoreGatewayTargetTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTargetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreGatewayTargetTimeoutsElRef {
        BedrockagentcoreGatewayTargetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTargetTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayTargetDynamic {
    credential_provider_configuration:
        Option<DynamicBlock<BedrockagentcoreGatewayTargetCredentialProviderConfigurationEl>>,
    target_configuration: Option<DynamicBlock<BedrockagentcoreGatewayTargetTargetConfigurationEl>>,
}
