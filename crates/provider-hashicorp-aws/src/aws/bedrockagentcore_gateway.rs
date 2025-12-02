use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BedrockagentcoreGatewayData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    authorizer_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exception_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    name: PrimField<String>,
    protocol_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorizer_configuration: Option<Vec<BedrockagentcoreGatewayAuthorizerConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol_configuration: Option<Vec<BedrockagentcoreGatewayProtocolConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BedrockagentcoreGatewayTimeoutsEl>,
    dynamic: BedrockagentcoreGatewayDynamic,
}
struct BedrockagentcoreGateway_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockagentcoreGatewayData>,
}
#[derive(Clone)]
pub struct BedrockagentcoreGateway(Rc<BedrockagentcoreGateway_>);
impl BedrockagentcoreGateway {
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
    #[doc = "Set the field `exception_level`.\n"]
    pub fn set_exception_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().exception_level = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `authorizer_configuration`.\n"]
    pub fn set_authorizer_configuration(
        self,
        v: impl Into<BlockAssignable<BedrockagentcoreGatewayAuthorizerConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authorizer_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authorizer_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `protocol_configuration`.\n"]
    pub fn set_protocol_configuration(
        self,
        v: impl Into<BlockAssignable<BedrockagentcoreGatewayProtocolConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().protocol_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.protocol_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BedrockagentcoreGatewayTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `authorizer_type` after provisioning.\n"]
    pub fn authorizer_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorizer_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exception_level` after provisioning.\n"]
    pub fn exception_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exception_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_url` after provisioning.\n"]
    pub fn gateway_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocol_type` after provisioning.\n"]
    pub fn protocol_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_details` after provisioning.\n"]
    pub fn workload_identity_details(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayWorkloadIdentityDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_identity_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorizer_configuration` after provisioning.\n"]
    pub fn authorizer_configuration(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayAuthorizerConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorizer_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocol_configuration` after provisioning.\n"]
    pub fn protocol_configuration(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayProtocolConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.protocol_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreGatewayTimeoutsElRef {
        BedrockagentcoreGatewayTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BedrockagentcoreGateway {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BedrockagentcoreGateway {}
impl ToListMappable for BedrockagentcoreGateway {
    type O = ListRef<BedrockagentcoreGatewayRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BedrockagentcoreGateway_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrockagentcore_gateway".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBedrockagentcoreGateway {
    pub tf_id: String,
    #[doc = ""]
    pub authorizer_type: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub protocol_type: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildBedrockagentcoreGateway {
    pub fn build(self, stack: &mut Stack) -> BedrockagentcoreGateway {
        let out = BedrockagentcoreGateway(Rc::new(BedrockagentcoreGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockagentcoreGatewayData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                authorizer_type: self.authorizer_type,
                description: core::default::Default::default(),
                exception_level: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                name: self.name,
                protocol_type: self.protocol_type,
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                authorizer_configuration: core::default::Default::default(),
                protocol_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BedrockagentcoreGatewayRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BedrockagentcoreGatewayRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authorizer_type` after provisioning.\n"]
    pub fn authorizer_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorizer_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `exception_level` after provisioning.\n"]
    pub fn exception_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exception_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_url` after provisioning.\n"]
    pub fn gateway_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocol_type` after provisioning.\n"]
    pub fn protocol_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_identity_details` after provisioning.\n"]
    pub fn workload_identity_details(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayWorkloadIdentityDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_identity_details", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorizer_configuration` after provisioning.\n"]
    pub fn authorizer_configuration(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayAuthorizerConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorizer_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocol_configuration` after provisioning.\n"]
    pub fn protocol_configuration(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayProtocolConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.protocol_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreGatewayTimeoutsElRef {
        BedrockagentcoreGatewayTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayWorkloadIdentityDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    workload_identity_arn: Option<PrimField<String>>,
}
impl BedrockagentcoreGatewayWorkloadIdentityDetailsEl {
    #[doc = "Set the field `workload_identity_arn`.\n"]
    pub fn set_workload_identity_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.workload_identity_arn = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockagentcoreGatewayWorkloadIdentityDetailsEl {
    type O = BlockAssignable<BedrockagentcoreGatewayWorkloadIdentityDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayWorkloadIdentityDetailsEl {}
impl BuildBedrockagentcoreGatewayWorkloadIdentityDetailsEl {
    pub fn build(self) -> BedrockagentcoreGatewayWorkloadIdentityDetailsEl {
        BedrockagentcoreGatewayWorkloadIdentityDetailsEl {
            workload_identity_arn: core::default::Default::default(),
        }
    }
}
pub struct BedrockagentcoreGatewayWorkloadIdentityDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayWorkloadIdentityDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayWorkloadIdentityDetailsElRef {
        BedrockagentcoreGatewayWorkloadIdentityDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayWorkloadIdentityDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `workload_identity_arn` after provisioning.\n"]
    pub fn workload_identity_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_identity_arn", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayAuthorizerConfigurationElCustomJwtAuthorizerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_audience: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_clients: Option<SetField<PrimField<String>>>,
    discovery_url: PrimField<String>,
}
impl BedrockagentcoreGatewayAuthorizerConfigurationElCustomJwtAuthorizerEl {
    #[doc = "Set the field `allowed_audience`.\n"]
    pub fn set_allowed_audience(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_audience = Some(v.into());
        self
    }
    #[doc = "Set the field `allowed_clients`.\n"]
    pub fn set_allowed_clients(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_clients = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockagentcoreGatewayAuthorizerConfigurationElCustomJwtAuthorizerEl {
    type O = BlockAssignable<BedrockagentcoreGatewayAuthorizerConfigurationElCustomJwtAuthorizerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayAuthorizerConfigurationElCustomJwtAuthorizerEl {
    #[doc = ""]
    pub discovery_url: PrimField<String>,
}
impl BuildBedrockagentcoreGatewayAuthorizerConfigurationElCustomJwtAuthorizerEl {
    pub fn build(self) -> BedrockagentcoreGatewayAuthorizerConfigurationElCustomJwtAuthorizerEl {
        BedrockagentcoreGatewayAuthorizerConfigurationElCustomJwtAuthorizerEl {
            allowed_audience: core::default::Default::default(),
            allowed_clients: core::default::Default::default(),
            discovery_url: self.discovery_url,
        }
    }
}
pub struct BedrockagentcoreGatewayAuthorizerConfigurationElCustomJwtAuthorizerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayAuthorizerConfigurationElCustomJwtAuthorizerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayAuthorizerConfigurationElCustomJwtAuthorizerElRef {
        BedrockagentcoreGatewayAuthorizerConfigurationElCustomJwtAuthorizerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayAuthorizerConfigurationElCustomJwtAuthorizerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_audience` after provisioning.\n"]
    pub fn allowed_audience(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_audience", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_clients` after provisioning.\n"]
    pub fn allowed_clients(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_clients", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_url` after provisioning.\n"]
    pub fn discovery_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.discovery_url", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayAuthorizerConfigurationElDynamic {
    custom_jwt_authorizer:
        Option<DynamicBlock<BedrockagentcoreGatewayAuthorizerConfigurationElCustomJwtAuthorizerEl>>,
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayAuthorizerConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_jwt_authorizer:
        Option<Vec<BedrockagentcoreGatewayAuthorizerConfigurationElCustomJwtAuthorizerEl>>,
    dynamic: BedrockagentcoreGatewayAuthorizerConfigurationElDynamic,
}
impl BedrockagentcoreGatewayAuthorizerConfigurationEl {
    #[doc = "Set the field `custom_jwt_authorizer`.\n"]
    pub fn set_custom_jwt_authorizer(
        mut self,
        v: impl Into<
            BlockAssignable<BedrockagentcoreGatewayAuthorizerConfigurationElCustomJwtAuthorizerEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_jwt_authorizer = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_jwt_authorizer = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BedrockagentcoreGatewayAuthorizerConfigurationEl {
    type O = BlockAssignable<BedrockagentcoreGatewayAuthorizerConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayAuthorizerConfigurationEl {}
impl BuildBedrockagentcoreGatewayAuthorizerConfigurationEl {
    pub fn build(self) -> BedrockagentcoreGatewayAuthorizerConfigurationEl {
        BedrockagentcoreGatewayAuthorizerConfigurationEl {
            custom_jwt_authorizer: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BedrockagentcoreGatewayAuthorizerConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayAuthorizerConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayAuthorizerConfigurationElRef {
        BedrockagentcoreGatewayAuthorizerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayAuthorizerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_jwt_authorizer` after provisioning.\n"]
    pub fn custom_jwt_authorizer(
        &self,
    ) -> ListRef<BedrockagentcoreGatewayAuthorizerConfigurationElCustomJwtAuthorizerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_jwt_authorizer", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayProtocolConfigurationElMcpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instructions: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_versions: Option<SetField<PrimField<String>>>,
}
impl BedrockagentcoreGatewayProtocolConfigurationElMcpEl {
    #[doc = "Set the field `instructions`.\n"]
    pub fn set_instructions(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instructions = Some(v.into());
        self
    }
    #[doc = "Set the field `search_type`.\n"]
    pub fn set_search_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.search_type = Some(v.into());
        self
    }
    #[doc = "Set the field `supported_versions`.\n"]
    pub fn set_supported_versions(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.supported_versions = Some(v.into());
        self
    }
}
impl ToListMappable for BedrockagentcoreGatewayProtocolConfigurationElMcpEl {
    type O = BlockAssignable<BedrockagentcoreGatewayProtocolConfigurationElMcpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayProtocolConfigurationElMcpEl {}
impl BuildBedrockagentcoreGatewayProtocolConfigurationElMcpEl {
    pub fn build(self) -> BedrockagentcoreGatewayProtocolConfigurationElMcpEl {
        BedrockagentcoreGatewayProtocolConfigurationElMcpEl {
            instructions: core::default::Default::default(),
            search_type: core::default::Default::default(),
            supported_versions: core::default::Default::default(),
        }
    }
}
pub struct BedrockagentcoreGatewayProtocolConfigurationElMcpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayProtocolConfigurationElMcpElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentcoreGatewayProtocolConfigurationElMcpElRef {
        BedrockagentcoreGatewayProtocolConfigurationElMcpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayProtocolConfigurationElMcpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instructions` after provisioning.\n"]
    pub fn instructions(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instructions", self.base))
    }
    #[doc = "Get a reference to the value of field `search_type` after provisioning.\n"]
    pub fn search_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search_type", self.base))
    }
    #[doc = "Get a reference to the value of field `supported_versions` after provisioning.\n"]
    pub fn supported_versions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.supported_versions", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayProtocolConfigurationElDynamic {
    mcp: Option<DynamicBlock<BedrockagentcoreGatewayProtocolConfigurationElMcpEl>>,
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayProtocolConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mcp: Option<Vec<BedrockagentcoreGatewayProtocolConfigurationElMcpEl>>,
    dynamic: BedrockagentcoreGatewayProtocolConfigurationElDynamic,
}
impl BedrockagentcoreGatewayProtocolConfigurationEl {
    #[doc = "Set the field `mcp`.\n"]
    pub fn set_mcp(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentcoreGatewayProtocolConfigurationElMcpEl>>,
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
impl ToListMappable for BedrockagentcoreGatewayProtocolConfigurationEl {
    type O = BlockAssignable<BedrockagentcoreGatewayProtocolConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayProtocolConfigurationEl {}
impl BuildBedrockagentcoreGatewayProtocolConfigurationEl {
    pub fn build(self) -> BedrockagentcoreGatewayProtocolConfigurationEl {
        BedrockagentcoreGatewayProtocolConfigurationEl {
            mcp: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BedrockagentcoreGatewayProtocolConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayProtocolConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreGatewayProtocolConfigurationElRef {
        BedrockagentcoreGatewayProtocolConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayProtocolConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mcp` after provisioning.\n"]
    pub fn mcp(&self) -> ListRef<BedrockagentcoreGatewayProtocolConfigurationElMcpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mcp", self.base))
    }
}
#[derive(Serialize)]
pub struct BedrockagentcoreGatewayTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BedrockagentcoreGatewayTimeoutsEl {
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
impl ToListMappable for BedrockagentcoreGatewayTimeoutsEl {
    type O = BlockAssignable<BedrockagentcoreGatewayTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBedrockagentcoreGatewayTimeoutsEl {}
impl BuildBedrockagentcoreGatewayTimeoutsEl {
    pub fn build(self) -> BedrockagentcoreGatewayTimeoutsEl {
        BedrockagentcoreGatewayTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BedrockagentcoreGatewayTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BedrockagentcoreGatewayTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentcoreGatewayTimeoutsElRef {
        BedrockagentcoreGatewayTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BedrockagentcoreGatewayTimeoutsElRef {
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
struct BedrockagentcoreGatewayDynamic {
    authorizer_configuration:
        Option<DynamicBlock<BedrockagentcoreGatewayAuthorizerConfigurationEl>>,
    protocol_configuration: Option<DynamicBlock<BedrockagentcoreGatewayProtocolConfigurationEl>>,
}
