use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FinspaceKxEnvironmentData {
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
    kms_key_id: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_dns_configuration: Option<Vec<FinspaceKxEnvironmentCustomDnsConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FinspaceKxEnvironmentTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_configuration: Option<Vec<FinspaceKxEnvironmentTransitGatewayConfigurationEl>>,
    dynamic: FinspaceKxEnvironmentDynamic,
}

struct FinspaceKxEnvironment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FinspaceKxEnvironmentData>,
}

#[derive(Clone)]
pub struct FinspaceKxEnvironment(Rc<FinspaceKxEnvironment_>);

impl FinspaceKxEnvironment {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_dns_configuration`.\n"]
    pub fn set_custom_dns_configuration(
        self,
        v: impl Into<BlockAssignable<FinspaceKxEnvironmentCustomDnsConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_dns_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_dns_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FinspaceKxEnvironmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_configuration`.\n"]
    pub fn set_transit_gateway_configuration(
        self,
        v: impl Into<BlockAssignable<FinspaceKxEnvironmentTransitGatewayConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().transit_gateway_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.transit_gateway_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_timestamp` after provisioning.\n"]
    pub fn created_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infrastructure_account_id` after provisioning.\n"]
    pub fn infrastructure_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_timestamp` after provisioning.\n"]
    pub fn last_modified_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_dns_configuration` after provisioning.\n"]
    pub fn custom_dns_configuration(&self) -> ListRef<FinspaceKxEnvironmentCustomDnsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_dns_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FinspaceKxEnvironmentTimeoutsElRef {
        FinspaceKxEnvironmentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_configuration` after provisioning.\n"]
    pub fn transit_gateway_configuration(&self) -> ListRef<FinspaceKxEnvironmentTransitGatewayConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.transit_gateway_configuration", self.extract_ref()))
    }
}

impl Referable for FinspaceKxEnvironment {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for FinspaceKxEnvironment { }

impl ToListMappable for FinspaceKxEnvironment {
    type O = ListRef<FinspaceKxEnvironmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FinspaceKxEnvironment_ {
    fn extract_resource_type(&self) -> String {
        "aws_finspace_kx_environment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFinspaceKxEnvironment {
    pub tf_id: String,
    #[doc = ""]
    pub kms_key_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildFinspaceKxEnvironment {
    pub fn build(self, stack: &mut Stack) -> FinspaceKxEnvironment {
        let out = FinspaceKxEnvironment(Rc::new(FinspaceKxEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FinspaceKxEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                kms_key_id: self.kms_key_id,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                custom_dns_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                transit_gateway_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FinspaceKxEnvironmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxEnvironmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl FinspaceKxEnvironmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_timestamp` after provisioning.\n"]
    pub fn created_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infrastructure_account_id` after provisioning.\n"]
    pub fn infrastructure_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_timestamp` after provisioning.\n"]
    pub fn last_modified_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_dns_configuration` after provisioning.\n"]
    pub fn custom_dns_configuration(&self) -> ListRef<FinspaceKxEnvironmentCustomDnsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_dns_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FinspaceKxEnvironmentTimeoutsElRef {
        FinspaceKxEnvironmentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_configuration` after provisioning.\n"]
    pub fn transit_gateway_configuration(&self) -> ListRef<FinspaceKxEnvironmentTransitGatewayConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.transit_gateway_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxEnvironmentCustomDnsConfigurationEl {
    custom_dns_server_ip: PrimField<String>,
    custom_dns_server_name: PrimField<String>,
}

impl FinspaceKxEnvironmentCustomDnsConfigurationEl { }

impl ToListMappable for FinspaceKxEnvironmentCustomDnsConfigurationEl {
    type O = BlockAssignable<FinspaceKxEnvironmentCustomDnsConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxEnvironmentCustomDnsConfigurationEl {
    #[doc = ""]
    pub custom_dns_server_ip: PrimField<String>,
    #[doc = ""]
    pub custom_dns_server_name: PrimField<String>,
}

impl BuildFinspaceKxEnvironmentCustomDnsConfigurationEl {
    pub fn build(self) -> FinspaceKxEnvironmentCustomDnsConfigurationEl {
        FinspaceKxEnvironmentCustomDnsConfigurationEl {
            custom_dns_server_ip: self.custom_dns_server_ip,
            custom_dns_server_name: self.custom_dns_server_name,
        }
    }
}

pub struct FinspaceKxEnvironmentCustomDnsConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxEnvironmentCustomDnsConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FinspaceKxEnvironmentCustomDnsConfigurationElRef {
        FinspaceKxEnvironmentCustomDnsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxEnvironmentCustomDnsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_dns_server_ip` after provisioning.\n"]
    pub fn custom_dns_server_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_dns_server_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_dns_server_name` after provisioning.\n"]
    pub fn custom_dns_server_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_dns_server_name", self.base))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxEnvironmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl FinspaceKxEnvironmentTimeoutsEl {
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

impl ToListMappable for FinspaceKxEnvironmentTimeoutsEl {
    type O = BlockAssignable<FinspaceKxEnvironmentTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxEnvironmentTimeoutsEl {}

impl BuildFinspaceKxEnvironmentTimeoutsEl {
    pub fn build(self) -> FinspaceKxEnvironmentTimeoutsEl {
        FinspaceKxEnvironmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FinspaceKxEnvironmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxEnvironmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FinspaceKxEnvironmentTimeoutsElRef {
        FinspaceKxEnvironmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxEnvironmentTimeoutsElRef {
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
pub struct FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElIcmpTypeCodeEl {
    code: PrimField<f64>,
    #[serde(rename = "type")]
    type_: PrimField<f64>,
}

impl FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElIcmpTypeCodeEl { }

impl ToListMappable for FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElIcmpTypeCodeEl {
    type O =
        BlockAssignable<
            FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElIcmpTypeCodeEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElIcmpTypeCodeEl {
    #[doc = ""]
    pub code: PrimField<f64>,
    #[doc = ""]
    pub type_: PrimField<f64>,
}

impl BuildFinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElIcmpTypeCodeEl {
    pub fn build(
        self,
    ) -> FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElIcmpTypeCodeEl {
        FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElIcmpTypeCodeEl {
            code: self.code,
            type_: self.type_,
        }
    }
}

pub struct FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElIcmpTypeCodeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElIcmpTypeCodeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElIcmpTypeCodeElRef {
        FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElIcmpTypeCodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElIcmpTypeCodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElPortRangeEl {
    from: PrimField<f64>,
    to: PrimField<f64>,
}

impl FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElPortRangeEl { }

impl ToListMappable for FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElPortRangeEl {
    type O =
        BlockAssignable<
            FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElPortRangeEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElPortRangeEl {
    #[doc = ""]
    pub from: PrimField<f64>,
    #[doc = ""]
    pub to: PrimField<f64>,
}

impl BuildFinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElPortRangeEl {
    pub fn build(
        self,
    ) -> FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElPortRangeEl {
        FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElPortRangeEl {
            from: self.from,
            to: self.to,
        }
    }
}

pub struct FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElPortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElPortRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElPortRangeElRef {
        FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize, Default)]
struct FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElDynamic {
    icmp_type_code: Option<
        DynamicBlock<
            FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElIcmpTypeCodeEl,
        >,
    >,
    port_range: Option<
        DynamicBlock<FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElPortRangeEl>,
    >,
}

#[derive(Serialize)]
pub struct FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationEl {
    cidr_block: PrimField<String>,
    protocol: PrimField<String>,
    rule_action: PrimField<String>,
    rule_number: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icmp_type_code: Option<
        Vec<FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElIcmpTypeCodeEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<
        Vec<FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElPortRangeEl>,
    >,
    dynamic: FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElDynamic,
}

impl FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationEl {
    #[doc = "Set the field `icmp_type_code`.\n"]
    pub fn set_icmp_type_code(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElIcmpTypeCodeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.icmp_type_code = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.icmp_type_code = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElPortRangeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.port_range = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.port_range = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationEl {
    type O = BlockAssignable<FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationEl {
    #[doc = ""]
    pub cidr_block: PrimField<String>,
    #[doc = ""]
    pub protocol: PrimField<String>,
    #[doc = ""]
    pub rule_action: PrimField<String>,
    #[doc = ""]
    pub rule_number: PrimField<f64>,
}

impl BuildFinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationEl {
    pub fn build(self) -> FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationEl {
        FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationEl {
            cidr_block: self.cidr_block,
            protocol: self.protocol,
            rule_action: self.rule_action,
            rule_number: self.rule_number,
            icmp_type_code: core::default::Default::default(),
            port_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElRef {
        FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_action` after provisioning.\n"]
    pub fn rule_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_action", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_number` after provisioning.\n"]
    pub fn rule_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_number", self.base))
    }

    #[doc = "Get a reference to the value of field `icmp_type_code` after provisioning.\n"]
    pub fn icmp_type_code(
        &self,
    ) -> ListRef<
        FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElIcmpTypeCodeElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.icmp_type_code", self.base))
    }

    #[doc = "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(
        &self,
    ) -> ListRef<FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElPortRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.port_range", self.base))
    }
}

#[derive(Serialize, Default)]
struct FinspaceKxEnvironmentTransitGatewayConfigurationElDynamic {
    attachment_network_acl_configuration: Option<
        DynamicBlock<FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct FinspaceKxEnvironmentTransitGatewayConfigurationEl {
    routable_cidr_space: PrimField<String>,
    transit_gateway_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment_network_acl_configuration: Option<
        Vec<FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationEl>,
    >,
    dynamic: FinspaceKxEnvironmentTransitGatewayConfigurationElDynamic,
}

impl FinspaceKxEnvironmentTransitGatewayConfigurationEl {
    #[doc = "Set the field `attachment_network_acl_configuration`.\n"]
    pub fn set_attachment_network_acl_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.attachment_network_acl_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.attachment_network_acl_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for FinspaceKxEnvironmentTransitGatewayConfigurationEl {
    type O = BlockAssignable<FinspaceKxEnvironmentTransitGatewayConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxEnvironmentTransitGatewayConfigurationEl {
    #[doc = ""]
    pub routable_cidr_space: PrimField<String>,
    #[doc = ""]
    pub transit_gateway_id: PrimField<String>,
}

impl BuildFinspaceKxEnvironmentTransitGatewayConfigurationEl {
    pub fn build(self) -> FinspaceKxEnvironmentTransitGatewayConfigurationEl {
        FinspaceKxEnvironmentTransitGatewayConfigurationEl {
            routable_cidr_space: self.routable_cidr_space,
            transit_gateway_id: self.transit_gateway_id,
            attachment_network_acl_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct FinspaceKxEnvironmentTransitGatewayConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FinspaceKxEnvironmentTransitGatewayConfigurationElRef {
    fn new(shared: StackShared, base: String) -> FinspaceKxEnvironmentTransitGatewayConfigurationElRef {
        FinspaceKxEnvironmentTransitGatewayConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxEnvironmentTransitGatewayConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `routable_cidr_space` after provisioning.\n"]
    pub fn routable_cidr_space(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.routable_cidr_space", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `attachment_network_acl_configuration` after provisioning.\n"]
    pub fn attachment_network_acl_configuration(
        &self,
    ) -> ListRef<FinspaceKxEnvironmentTransitGatewayConfigurationElAttachmentNetworkAclConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.attachment_network_acl_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct FinspaceKxEnvironmentDynamic {
    custom_dns_configuration: Option<DynamicBlock<FinspaceKxEnvironmentCustomDnsConfigurationEl>>,
    transit_gateway_configuration: Option<DynamicBlock<FinspaceKxEnvironmentTransitGatewayConfigurationEl>>,
}
