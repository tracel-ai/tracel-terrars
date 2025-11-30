use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CloudwatchEventEndpointData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_bus: Option<Vec<CloudwatchEventEndpointEventBusEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_config: Option<Vec<CloudwatchEventEndpointReplicationConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_config: Option<Vec<CloudwatchEventEndpointRoutingConfigEl>>,
    dynamic: CloudwatchEventEndpointDynamic,
}

struct CloudwatchEventEndpoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudwatchEventEndpointData>,
}

#[derive(Clone)]
pub struct CloudwatchEventEndpoint(Rc<CloudwatchEventEndpoint_>);

impl CloudwatchEventEndpoint {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `event_bus`.\n"]
    pub fn set_event_bus(
        self,
        v: impl Into<BlockAssignable<CloudwatchEventEndpointEventBusEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_bus = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_bus = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `replication_config`.\n"]
    pub fn set_replication_config(
        self,
        v: impl Into<BlockAssignable<CloudwatchEventEndpointReplicationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().replication_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.replication_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `routing_config`.\n"]
    pub fn set_routing_config(
        self,
        v: impl Into<BlockAssignable<CloudwatchEventEndpointRoutingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().routing_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.routing_config = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint_url` after provisioning.\n"]
    pub fn endpoint_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_url", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `event_bus` after provisioning.\n"]
    pub fn event_bus(&self) -> ListRef<CloudwatchEventEndpointEventBusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_bus", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_config` after provisioning.\n"]
    pub fn replication_config(&self) -> ListRef<CloudwatchEventEndpointReplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `routing_config` after provisioning.\n"]
    pub fn routing_config(&self) -> ListRef<CloudwatchEventEndpointRoutingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.routing_config", self.extract_ref()),
        )
    }
}

impl Referable for CloudwatchEventEndpoint {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CloudwatchEventEndpoint {}

impl ToListMappable for CloudwatchEventEndpoint {
    type O = ListRef<CloudwatchEventEndpointRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudwatchEventEndpoint_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudwatch_event_endpoint".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudwatchEventEndpoint {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCloudwatchEventEndpoint {
    pub fn build(self, stack: &mut Stack) -> CloudwatchEventEndpoint {
        let out = CloudwatchEventEndpoint(Rc::new(CloudwatchEventEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudwatchEventEndpointData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                role_arn: core::default::Default::default(),
                event_bus: core::default::Default::default(),
                replication_config: core::default::Default::default(),
                routing_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudwatchEventEndpointRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventEndpointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CloudwatchEventEndpointRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint_url` after provisioning.\n"]
    pub fn endpoint_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_url", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `event_bus` after provisioning.\n"]
    pub fn event_bus(&self) -> ListRef<CloudwatchEventEndpointEventBusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_bus", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_config` after provisioning.\n"]
    pub fn replication_config(&self) -> ListRef<CloudwatchEventEndpointReplicationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `routing_config` after provisioning.\n"]
    pub fn routing_config(&self) -> ListRef<CloudwatchEventEndpointRoutingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.routing_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventEndpointEventBusEl {
    event_bus_arn: PrimField<String>,
}

impl CloudwatchEventEndpointEventBusEl {}

impl ToListMappable for CloudwatchEventEndpointEventBusEl {
    type O = BlockAssignable<CloudwatchEventEndpointEventBusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventEndpointEventBusEl {
    #[doc = ""]
    pub event_bus_arn: PrimField<String>,
}

impl BuildCloudwatchEventEndpointEventBusEl {
    pub fn build(self) -> CloudwatchEventEndpointEventBusEl {
        CloudwatchEventEndpointEventBusEl {
            event_bus_arn: self.event_bus_arn,
        }
    }
}

pub struct CloudwatchEventEndpointEventBusElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventEndpointEventBusElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchEventEndpointEventBusElRef {
        CloudwatchEventEndpointEventBusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventEndpointEventBusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_bus_arn` after provisioning.\n"]
    pub fn event_bus_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_bus_arn", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventEndpointReplicationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}

impl CloudwatchEventEndpointReplicationConfigEl {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchEventEndpointReplicationConfigEl {
    type O = BlockAssignable<CloudwatchEventEndpointReplicationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventEndpointReplicationConfigEl {}

impl BuildCloudwatchEventEndpointReplicationConfigEl {
    pub fn build(self) -> CloudwatchEventEndpointReplicationConfigEl {
        CloudwatchEventEndpointReplicationConfigEl {
            state: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventEndpointReplicationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventEndpointReplicationConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchEventEndpointReplicationConfigElRef {
        CloudwatchEventEndpointReplicationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventEndpointReplicationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventEndpointRoutingConfigElFailoverConfigElPrimaryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check: Option<PrimField<String>>,
}

impl CloudwatchEventEndpointRoutingConfigElFailoverConfigElPrimaryEl {
    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.health_check = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchEventEndpointRoutingConfigElFailoverConfigElPrimaryEl {
    type O = BlockAssignable<CloudwatchEventEndpointRoutingConfigElFailoverConfigElPrimaryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventEndpointRoutingConfigElFailoverConfigElPrimaryEl {}

impl BuildCloudwatchEventEndpointRoutingConfigElFailoverConfigElPrimaryEl {
    pub fn build(self) -> CloudwatchEventEndpointRoutingConfigElFailoverConfigElPrimaryEl {
        CloudwatchEventEndpointRoutingConfigElFailoverConfigElPrimaryEl {
            health_check: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventEndpointRoutingConfigElFailoverConfigElPrimaryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventEndpointRoutingConfigElFailoverConfigElPrimaryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventEndpointRoutingConfigElFailoverConfigElPrimaryElRef {
        CloudwatchEventEndpointRoutingConfigElFailoverConfigElPrimaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventEndpointRoutingConfigElFailoverConfigElPrimaryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventEndpointRoutingConfigElFailoverConfigElSecondaryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    route: Option<PrimField<String>>,
}

impl CloudwatchEventEndpointRoutingConfigElFailoverConfigElSecondaryEl {
    #[doc = "Set the field `route`.\n"]
    pub fn set_route(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.route = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchEventEndpointRoutingConfigElFailoverConfigElSecondaryEl {
    type O = BlockAssignable<CloudwatchEventEndpointRoutingConfigElFailoverConfigElSecondaryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventEndpointRoutingConfigElFailoverConfigElSecondaryEl {}

impl BuildCloudwatchEventEndpointRoutingConfigElFailoverConfigElSecondaryEl {
    pub fn build(self) -> CloudwatchEventEndpointRoutingConfigElFailoverConfigElSecondaryEl {
        CloudwatchEventEndpointRoutingConfigElFailoverConfigElSecondaryEl {
            route: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventEndpointRoutingConfigElFailoverConfigElSecondaryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventEndpointRoutingConfigElFailoverConfigElSecondaryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventEndpointRoutingConfigElFailoverConfigElSecondaryElRef {
        CloudwatchEventEndpointRoutingConfigElFailoverConfigElSecondaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventEndpointRoutingConfigElFailoverConfigElSecondaryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `route` after provisioning.\n"]
    pub fn route(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventEndpointRoutingConfigElFailoverConfigElDynamic {
    primary: Option<DynamicBlock<CloudwatchEventEndpointRoutingConfigElFailoverConfigElPrimaryEl>>,
    secondary:
        Option<DynamicBlock<CloudwatchEventEndpointRoutingConfigElFailoverConfigElSecondaryEl>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventEndpointRoutingConfigElFailoverConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    primary: Option<Vec<CloudwatchEventEndpointRoutingConfigElFailoverConfigElPrimaryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary: Option<Vec<CloudwatchEventEndpointRoutingConfigElFailoverConfigElSecondaryEl>>,
    dynamic: CloudwatchEventEndpointRoutingConfigElFailoverConfigElDynamic,
}

impl CloudwatchEventEndpointRoutingConfigElFailoverConfigEl {
    #[doc = "Set the field `primary`.\n"]
    pub fn set_primary(
        mut self,
        v: impl Into<BlockAssignable<CloudwatchEventEndpointRoutingConfigElFailoverConfigElPrimaryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.primary = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.primary = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `secondary`.\n"]
    pub fn set_secondary(
        mut self,
        v: impl Into<BlockAssignable<CloudwatchEventEndpointRoutingConfigElFailoverConfigElSecondaryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.secondary = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.secondary = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CloudwatchEventEndpointRoutingConfigElFailoverConfigEl {
    type O = BlockAssignable<CloudwatchEventEndpointRoutingConfigElFailoverConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventEndpointRoutingConfigElFailoverConfigEl {}

impl BuildCloudwatchEventEndpointRoutingConfigElFailoverConfigEl {
    pub fn build(self) -> CloudwatchEventEndpointRoutingConfigElFailoverConfigEl {
        CloudwatchEventEndpointRoutingConfigElFailoverConfigEl {
            primary: core::default::Default::default(),
            secondary: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudwatchEventEndpointRoutingConfigElFailoverConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventEndpointRoutingConfigElFailoverConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventEndpointRoutingConfigElFailoverConfigElRef {
        CloudwatchEventEndpointRoutingConfigElFailoverConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventEndpointRoutingConfigElFailoverConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `primary` after provisioning.\n"]
    pub fn primary(
        &self,
    ) -> ListRef<CloudwatchEventEndpointRoutingConfigElFailoverConfigElPrimaryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.primary", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary` after provisioning.\n"]
    pub fn secondary(
        &self,
    ) -> ListRef<CloudwatchEventEndpointRoutingConfigElFailoverConfigElSecondaryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secondary", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventEndpointRoutingConfigElDynamic {
    failover_config: Option<DynamicBlock<CloudwatchEventEndpointRoutingConfigElFailoverConfigEl>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventEndpointRoutingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failover_config: Option<Vec<CloudwatchEventEndpointRoutingConfigElFailoverConfigEl>>,
    dynamic: CloudwatchEventEndpointRoutingConfigElDynamic,
}

impl CloudwatchEventEndpointRoutingConfigEl {
    #[doc = "Set the field `failover_config`.\n"]
    pub fn set_failover_config(
        mut self,
        v: impl Into<BlockAssignable<CloudwatchEventEndpointRoutingConfigElFailoverConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.failover_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.failover_config = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CloudwatchEventEndpointRoutingConfigEl {
    type O = BlockAssignable<CloudwatchEventEndpointRoutingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventEndpointRoutingConfigEl {}

impl BuildCloudwatchEventEndpointRoutingConfigEl {
    pub fn build(self) -> CloudwatchEventEndpointRoutingConfigEl {
        CloudwatchEventEndpointRoutingConfigEl {
            failover_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudwatchEventEndpointRoutingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchEventEndpointRoutingConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchEventEndpointRoutingConfigElRef {
        CloudwatchEventEndpointRoutingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventEndpointRoutingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failover_config` after provisioning.\n"]
    pub fn failover_config(
        &self,
    ) -> ListRef<CloudwatchEventEndpointRoutingConfigElFailoverConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.failover_config", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventEndpointDynamic {
    event_bus: Option<DynamicBlock<CloudwatchEventEndpointEventBusEl>>,
    replication_config: Option<DynamicBlock<CloudwatchEventEndpointReplicationConfigEl>>,
    routing_config: Option<DynamicBlock<CloudwatchEventEndpointRoutingConfigEl>>,
}
