use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VpcEndpointServiceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    acceptance_required: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_principals: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway_load_balancer_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_load_balancer_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_dns_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_ip_address_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_regions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VpcEndpointServiceTimeoutsEl>,
}

struct VpcEndpointService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpcEndpointServiceData>,
}

#[derive(Clone)]
pub struct VpcEndpointService(Rc<VpcEndpointService_>);

impl VpcEndpointService {
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

    #[doc = "Set the field `allowed_principals`.\n"]
    pub fn set_allowed_principals(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().allowed_principals = Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_load_balancer_arns`.\n"]
    pub fn set_gateway_load_balancer_arns(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().gateway_load_balancer_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `network_load_balancer_arns`.\n"]
    pub fn set_network_load_balancer_arns(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().network_load_balancer_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `private_dns_name`.\n"]
    pub fn set_private_dns_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().private_dns_name = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `supported_ip_address_types`.\n"]
    pub fn set_supported_ip_address_types(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().supported_ip_address_types = Some(v.into());
        self
    }

    #[doc = "Set the field `supported_regions`.\n"]
    pub fn set_supported_regions(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().supported_regions = Some(v.into());
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VpcEndpointServiceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `acceptance_required` after provisioning.\n"]
    pub fn acceptance_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.acceptance_required", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_principals` after provisioning.\n"]
    pub fn allowed_principals(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_principals", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_endpoint_dns_names` after provisioning.\n"]
    pub fn base_endpoint_dns_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.base_endpoint_dns_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_load_balancer_arns` after provisioning.\n"]
    pub fn gateway_load_balancer_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.gateway_load_balancer_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manages_vpc_endpoints` after provisioning.\n"]
    pub fn manages_vpc_endpoints(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.manages_vpc_endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_load_balancer_arns` after provisioning.\n"]
    pub fn network_load_balancer_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.network_load_balancer_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name` after provisioning.\n"]
    pub fn private_dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name_configuration` after provisioning.\n"]
    pub fn private_dns_name_configuration(&self) -> ListRef<VpcEndpointServicePrivateDnsNameConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.private_dns_name_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_type` after provisioning.\n"]
    pub fn service_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_ip_address_types` after provisioning.\n"]
    pub fn supported_ip_address_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.supported_ip_address_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_regions` after provisioning.\n"]
    pub fn supported_regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.supported_regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcEndpointServiceTimeoutsElRef {
        VpcEndpointServiceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for VpcEndpointService {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for VpcEndpointService { }

impl ToListMappable for VpcEndpointService {
    type O = ListRef<VpcEndpointServiceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VpcEndpointService_ {
    fn extract_resource_type(&self) -> String {
        "aws_vpc_endpoint_service".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpcEndpointService {
    pub tf_id: String,
    #[doc = ""]
    pub acceptance_required: PrimField<bool>,
}

impl BuildVpcEndpointService {
    pub fn build(self, stack: &mut Stack) -> VpcEndpointService {
        let out = VpcEndpointService(Rc::new(VpcEndpointService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VpcEndpointServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                acceptance_required: self.acceptance_required,
                allowed_principals: core::default::Default::default(),
                gateway_load_balancer_arns: core::default::Default::default(),
                id: core::default::Default::default(),
                network_load_balancer_arns: core::default::Default::default(),
                private_dns_name: core::default::Default::default(),
                region: core::default::Default::default(),
                supported_ip_address_types: core::default::Default::default(),
                supported_regions: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpcEndpointServiceRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcEndpointServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl VpcEndpointServiceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acceptance_required` after provisioning.\n"]
    pub fn acceptance_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.acceptance_required", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_principals` after provisioning.\n"]
    pub fn allowed_principals(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_principals", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_endpoint_dns_names` after provisioning.\n"]
    pub fn base_endpoint_dns_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.base_endpoint_dns_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_load_balancer_arns` after provisioning.\n"]
    pub fn gateway_load_balancer_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.gateway_load_balancer_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manages_vpc_endpoints` after provisioning.\n"]
    pub fn manages_vpc_endpoints(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.manages_vpc_endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_load_balancer_arns` after provisioning.\n"]
    pub fn network_load_balancer_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.network_load_balancer_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name` after provisioning.\n"]
    pub fn private_dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name_configuration` after provisioning.\n"]
    pub fn private_dns_name_configuration(&self) -> ListRef<VpcEndpointServicePrivateDnsNameConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.private_dns_name_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_type` after provisioning.\n"]
    pub fn service_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_ip_address_types` after provisioning.\n"]
    pub fn supported_ip_address_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.supported_ip_address_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_regions` after provisioning.\n"]
    pub fn supported_regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.supported_regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcEndpointServiceTimeoutsElRef {
        VpcEndpointServiceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct VpcEndpointServicePrivateDnsNameConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl VpcEndpointServicePrivateDnsNameConfigurationEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for VpcEndpointServicePrivateDnsNameConfigurationEl {
    type O = BlockAssignable<VpcEndpointServicePrivateDnsNameConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcEndpointServicePrivateDnsNameConfigurationEl {}

impl BuildVpcEndpointServicePrivateDnsNameConfigurationEl {
    pub fn build(self) -> VpcEndpointServicePrivateDnsNameConfigurationEl {
        VpcEndpointServicePrivateDnsNameConfigurationEl {
            name: core::default::Default::default(),
            state: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct VpcEndpointServicePrivateDnsNameConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcEndpointServicePrivateDnsNameConfigurationElRef {
    fn new(shared: StackShared, base: String) -> VpcEndpointServicePrivateDnsNameConfigurationElRef {
        VpcEndpointServicePrivateDnsNameConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcEndpointServicePrivateDnsNameConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct VpcEndpointServiceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl VpcEndpointServiceTimeoutsEl {
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

impl ToListMappable for VpcEndpointServiceTimeoutsEl {
    type O = BlockAssignable<VpcEndpointServiceTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcEndpointServiceTimeoutsEl {}

impl BuildVpcEndpointServiceTimeoutsEl {
    pub fn build(self) -> VpcEndpointServiceTimeoutsEl {
        VpcEndpointServiceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct VpcEndpointServiceTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcEndpointServiceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VpcEndpointServiceTimeoutsElRef {
        VpcEndpointServiceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcEndpointServiceTimeoutsElRef {
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
