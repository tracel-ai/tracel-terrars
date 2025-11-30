use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct RedshiftserverlessWorkgroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_vpc_routing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_capacity: Option<PrimField<f64>>,
    namespace_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publicly_accessible: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    track_name: Option<PrimField<String>>,
    workgroup_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_parameter: Option<Vec<RedshiftserverlessWorkgroupConfigParameterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_performance_target: Option<Vec<RedshiftserverlessWorkgroupPricePerformanceTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<RedshiftserverlessWorkgroupTimeoutsEl>,
    dynamic: RedshiftserverlessWorkgroupDynamic,
}

struct RedshiftserverlessWorkgroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RedshiftserverlessWorkgroupData>,
}

#[derive(Clone)]
pub struct RedshiftserverlessWorkgroup(Rc<RedshiftserverlessWorkgroup_>);

impl RedshiftserverlessWorkgroup {
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

    #[doc = "Set the field `base_capacity`.\n"]
    pub fn set_base_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().base_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `enhanced_vpc_routing`.\n"]
    pub fn set_enhanced_vpc_routing(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enhanced_vpc_routing = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `max_capacity`.\n"]
    pub fn set_max_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = Some(v.into());
        self
    }

    #[doc = "Set the field `publicly_accessible`.\n"]
    pub fn set_publicly_accessible(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().publicly_accessible = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_group_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().subnet_ids = Some(v.into());
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

    #[doc = "Set the field `track_name`.\n"]
    pub fn set_track_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().track_name = Some(v.into());
        self
    }

    #[doc = "Set the field `config_parameter`.\n"]
    pub fn set_config_parameter(
        self,
        v: impl Into<BlockAssignable<RedshiftserverlessWorkgroupConfigParameterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config_parameter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config_parameter = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `price_performance_target`.\n"]
    pub fn set_price_performance_target(
        self,
        v: impl Into<BlockAssignable<RedshiftserverlessWorkgroupPricePerformanceTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().price_performance_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.price_performance_target = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<RedshiftserverlessWorkgroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_capacity` after provisioning.\n"]
    pub fn base_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> ListRef<RedshiftserverlessWorkgroupEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enhanced_vpc_routing` after provisioning.\n"]
    pub fn enhanced_vpc_routing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enhanced_vpc_routing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `namespace_name` after provisioning.\n"]
    pub fn namespace_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publicly_accessible", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `track_name` after provisioning.\n"]
    pub fn track_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.track_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `workgroup_id` after provisioning.\n"]
    pub fn workgroup_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workgroup_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `workgroup_name` after provisioning.\n"]
    pub fn workgroup_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workgroup_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `price_performance_target` after provisioning.\n"]
    pub fn price_performance_target(
        &self,
    ) -> ListRef<RedshiftserverlessWorkgroupPricePerformanceTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.price_performance_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedshiftserverlessWorkgroupTimeoutsElRef {
        RedshiftserverlessWorkgroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for RedshiftserverlessWorkgroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for RedshiftserverlessWorkgroup {}

impl ToListMappable for RedshiftserverlessWorkgroup {
    type O = ListRef<RedshiftserverlessWorkgroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RedshiftserverlessWorkgroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_redshiftserverless_workgroup".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRedshiftserverlessWorkgroup {
    pub tf_id: String,
    #[doc = ""]
    pub namespace_name: PrimField<String>,
    #[doc = ""]
    pub workgroup_name: PrimField<String>,
}

impl BuildRedshiftserverlessWorkgroup {
    pub fn build(self, stack: &mut Stack) -> RedshiftserverlessWorkgroup {
        let out = RedshiftserverlessWorkgroup(Rc::new(RedshiftserverlessWorkgroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RedshiftserverlessWorkgroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                base_capacity: core::default::Default::default(),
                enhanced_vpc_routing: core::default::Default::default(),
                id: core::default::Default::default(),
                max_capacity: core::default::Default::default(),
                namespace_name: self.namespace_name,
                port: core::default::Default::default(),
                publicly_accessible: core::default::Default::default(),
                region: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                subnet_ids: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                track_name: core::default::Default::default(),
                workgroup_name: self.workgroup_name,
                config_parameter: core::default::Default::default(),
                price_performance_target: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RedshiftserverlessWorkgroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftserverlessWorkgroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl RedshiftserverlessWorkgroupRef {
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

    #[doc = "Get a reference to the value of field `base_capacity` after provisioning.\n"]
    pub fn base_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> ListRef<RedshiftserverlessWorkgroupEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enhanced_vpc_routing` after provisioning.\n"]
    pub fn enhanced_vpc_routing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enhanced_vpc_routing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `namespace_name` after provisioning.\n"]
    pub fn namespace_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publicly_accessible", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `track_name` after provisioning.\n"]
    pub fn track_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.track_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `workgroup_id` after provisioning.\n"]
    pub fn workgroup_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workgroup_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `workgroup_name` after provisioning.\n"]
    pub fn workgroup_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workgroup_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `price_performance_target` after provisioning.\n"]
    pub fn price_performance_target(
        &self,
    ) -> ListRef<RedshiftserverlessWorkgroupPricePerformanceTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.price_performance_target", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedshiftserverlessWorkgroupTimeoutsElRef {
        RedshiftserverlessWorkgroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
}

impl RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_zone = Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_interface_id = Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip_address`.\n"]
    pub fn set_private_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_ip_address = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet_id = Some(v.into());
        self
    }
}

impl ToListMappable for RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
    type O = BlockAssignable<RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {}

impl BuildRedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
    pub fn build(self) -> RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
        RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl {
            availability_zone: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
            private_ip_address: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
        }
    }
}

pub struct RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef {
        RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_interface_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `private_ip_address` after provisioning.\n"]
    pub fn private_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_ip_address", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface:
        Option<ListField<RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_endpoint_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
}

impl RedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
    #[doc = "Set the field `network_interface`.\n"]
    pub fn set_network_interface(
        mut self,
        v: impl Into<ListField<RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceEl>>,
    ) -> Self {
        self.network_interface = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_endpoint_id`.\n"]
    pub fn set_vpc_endpoint_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_endpoint_id = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_id = Some(v.into());
        self
    }
}

impl ToListMappable for RedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
    type O = BlockAssignable<RedshiftserverlessWorkgroupEndpointElVpcEndpointEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftserverlessWorkgroupEndpointElVpcEndpointEl {}

impl BuildRedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
    pub fn build(self) -> RedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
        RedshiftserverlessWorkgroupEndpointElVpcEndpointEl {
            network_interface: core::default::Default::default(),
            vpc_endpoint_id: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

pub struct RedshiftserverlessWorkgroupEndpointElVpcEndpointElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftserverlessWorkgroupEndpointElVpcEndpointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RedshiftserverlessWorkgroupEndpointElVpcEndpointElRef {
        RedshiftserverlessWorkgroupEndpointElVpcEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftserverlessWorkgroupEndpointElVpcEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(
        &self,
    ) -> ListRef<RedshiftserverlessWorkgroupEndpointElVpcEndpointElNetworkInterfaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_interface", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_endpoint_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftserverlessWorkgroupEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_endpoint: Option<ListField<RedshiftserverlessWorkgroupEndpointElVpcEndpointEl>>,
}

impl RedshiftserverlessWorkgroupEndpointEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_endpoint`.\n"]
    pub fn set_vpc_endpoint(
        mut self,
        v: impl Into<ListField<RedshiftserverlessWorkgroupEndpointElVpcEndpointEl>>,
    ) -> Self {
        self.vpc_endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for RedshiftserverlessWorkgroupEndpointEl {
    type O = BlockAssignable<RedshiftserverlessWorkgroupEndpointEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftserverlessWorkgroupEndpointEl {}

impl BuildRedshiftserverlessWorkgroupEndpointEl {
    pub fn build(self) -> RedshiftserverlessWorkgroupEndpointEl {
        RedshiftserverlessWorkgroupEndpointEl {
            address: core::default::Default::default(),
            port: core::default::Default::default(),
            vpc_endpoint: core::default::Default::default(),
        }
    }
}

pub struct RedshiftserverlessWorkgroupEndpointElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftserverlessWorkgroupEndpointElRef {
    fn new(shared: StackShared, base: String) -> RedshiftserverlessWorkgroupEndpointElRef {
        RedshiftserverlessWorkgroupEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftserverlessWorkgroupEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint` after provisioning.\n"]
    pub fn vpc_endpoint(&self) -> ListRef<RedshiftserverlessWorkgroupEndpointElVpcEndpointElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftserverlessWorkgroupConfigParameterEl {
    parameter_key: PrimField<String>,
    parameter_value: PrimField<String>,
}

impl RedshiftserverlessWorkgroupConfigParameterEl {}

impl ToListMappable for RedshiftserverlessWorkgroupConfigParameterEl {
    type O = BlockAssignable<RedshiftserverlessWorkgroupConfigParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftserverlessWorkgroupConfigParameterEl {
    #[doc = ""]
    pub parameter_key: PrimField<String>,
    #[doc = ""]
    pub parameter_value: PrimField<String>,
}

impl BuildRedshiftserverlessWorkgroupConfigParameterEl {
    pub fn build(self) -> RedshiftserverlessWorkgroupConfigParameterEl {
        RedshiftserverlessWorkgroupConfigParameterEl {
            parameter_key: self.parameter_key,
            parameter_value: self.parameter_value,
        }
    }
}

pub struct RedshiftserverlessWorkgroupConfigParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftserverlessWorkgroupConfigParameterElRef {
    fn new(shared: StackShared, base: String) -> RedshiftserverlessWorkgroupConfigParameterElRef {
        RedshiftserverlessWorkgroupConfigParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftserverlessWorkgroupConfigParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `parameter_key` after provisioning.\n"]
    pub fn parameter_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_key", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `parameter_value` after provisioning.\n"]
    pub fn parameter_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_value", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct RedshiftserverlessWorkgroupPricePerformanceTargetEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<PrimField<f64>>,
}

impl RedshiftserverlessWorkgroupPricePerformanceTargetEl {
    #[doc = "Set the field `level`.\n"]
    pub fn set_level(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.level = Some(v.into());
        self
    }
}

impl ToListMappable for RedshiftserverlessWorkgroupPricePerformanceTargetEl {
    type O = BlockAssignable<RedshiftserverlessWorkgroupPricePerformanceTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftserverlessWorkgroupPricePerformanceTargetEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildRedshiftserverlessWorkgroupPricePerformanceTargetEl {
    pub fn build(self) -> RedshiftserverlessWorkgroupPricePerformanceTargetEl {
        RedshiftserverlessWorkgroupPricePerformanceTargetEl {
            enabled: self.enabled,
            level: core::default::Default::default(),
        }
    }
}

pub struct RedshiftserverlessWorkgroupPricePerformanceTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftserverlessWorkgroupPricePerformanceTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RedshiftserverlessWorkgroupPricePerformanceTargetElRef {
        RedshiftserverlessWorkgroupPricePerformanceTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftserverlessWorkgroupPricePerformanceTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `level` after provisioning.\n"]
    pub fn level(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.level", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftserverlessWorkgroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl RedshiftserverlessWorkgroupTimeoutsEl {
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

impl ToListMappable for RedshiftserverlessWorkgroupTimeoutsEl {
    type O = BlockAssignable<RedshiftserverlessWorkgroupTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftserverlessWorkgroupTimeoutsEl {}

impl BuildRedshiftserverlessWorkgroupTimeoutsEl {
    pub fn build(self) -> RedshiftserverlessWorkgroupTimeoutsEl {
        RedshiftserverlessWorkgroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct RedshiftserverlessWorkgroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftserverlessWorkgroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> RedshiftserverlessWorkgroupTimeoutsElRef {
        RedshiftserverlessWorkgroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftserverlessWorkgroupTimeoutsElRef {
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
struct RedshiftserverlessWorkgroupDynamic {
    config_parameter: Option<DynamicBlock<RedshiftserverlessWorkgroupConfigParameterEl>>,
    price_performance_target:
        Option<DynamicBlock<RedshiftserverlessWorkgroupPricePerformanceTargetEl>>,
}
