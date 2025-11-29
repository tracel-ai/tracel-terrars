use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlobalacceleratorCustomRoutingEndpointGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_group_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    listener_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_configuration: Option<Vec<GlobalacceleratorCustomRoutingEndpointGroupDestinationConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_configuration: Option<Vec<GlobalacceleratorCustomRoutingEndpointGroupEndpointConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GlobalacceleratorCustomRoutingEndpointGroupTimeoutsEl>,
    dynamic: GlobalacceleratorCustomRoutingEndpointGroupDynamic,
}

struct GlobalacceleratorCustomRoutingEndpointGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GlobalacceleratorCustomRoutingEndpointGroupData>,
}

#[derive(Clone)]
pub struct GlobalacceleratorCustomRoutingEndpointGroup(Rc<GlobalacceleratorCustomRoutingEndpointGroup_>);

impl GlobalacceleratorCustomRoutingEndpointGroup {
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

    #[doc = "Set the field `endpoint_group_region`.\n"]
    pub fn set_endpoint_group_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().endpoint_group_region = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_configuration`.\n"]
    pub fn set_destination_configuration(
        self,
        v: impl Into<BlockAssignable<GlobalacceleratorCustomRoutingEndpointGroupDestinationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `endpoint_configuration`.\n"]
    pub fn set_endpoint_configuration(
        self,
        v: impl Into<BlockAssignable<GlobalacceleratorCustomRoutingEndpointGroupEndpointConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GlobalacceleratorCustomRoutingEndpointGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_group_region` after provisioning.\n"]
    pub fn endpoint_group_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_group_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_arn` after provisioning.\n"]
    pub fn listener_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.listener_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlobalacceleratorCustomRoutingEndpointGroupTimeoutsElRef {
        GlobalacceleratorCustomRoutingEndpointGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for GlobalacceleratorCustomRoutingEndpointGroup {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GlobalacceleratorCustomRoutingEndpointGroup { }

impl ToListMappable for GlobalacceleratorCustomRoutingEndpointGroup {
    type O = ListRef<GlobalacceleratorCustomRoutingEndpointGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GlobalacceleratorCustomRoutingEndpointGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_globalaccelerator_custom_routing_endpoint_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlobalacceleratorCustomRoutingEndpointGroup {
    pub tf_id: String,
    #[doc = ""]
    pub listener_arn: PrimField<String>,
}

impl BuildGlobalacceleratorCustomRoutingEndpointGroup {
    pub fn build(self, stack: &mut Stack) -> GlobalacceleratorCustomRoutingEndpointGroup {
        let out = GlobalacceleratorCustomRoutingEndpointGroup(Rc::new(GlobalacceleratorCustomRoutingEndpointGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GlobalacceleratorCustomRoutingEndpointGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                endpoint_group_region: core::default::Default::default(),
                id: core::default::Default::default(),
                listener_arn: self.listener_arn,
                destination_configuration: core::default::Default::default(),
                endpoint_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlobalacceleratorCustomRoutingEndpointGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlobalacceleratorCustomRoutingEndpointGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl GlobalacceleratorCustomRoutingEndpointGroupRef {
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

    #[doc = "Get a reference to the value of field `endpoint_group_region` after provisioning.\n"]
    pub fn endpoint_group_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_group_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_arn` after provisioning.\n"]
    pub fn listener_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.listener_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlobalacceleratorCustomRoutingEndpointGroupTimeoutsElRef {
        GlobalacceleratorCustomRoutingEndpointGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct GlobalacceleratorCustomRoutingEndpointGroupDestinationConfigurationEl {
    from_port: PrimField<f64>,
    protocols: SetField<PrimField<String>>,
    to_port: PrimField<f64>,
}

impl GlobalacceleratorCustomRoutingEndpointGroupDestinationConfigurationEl { }

impl ToListMappable for GlobalacceleratorCustomRoutingEndpointGroupDestinationConfigurationEl {
    type O = BlockAssignable<GlobalacceleratorCustomRoutingEndpointGroupDestinationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlobalacceleratorCustomRoutingEndpointGroupDestinationConfigurationEl {
    #[doc = ""]
    pub from_port: PrimField<f64>,
    #[doc = ""]
    pub protocols: SetField<PrimField<String>>,
    #[doc = ""]
    pub to_port: PrimField<f64>,
}

impl BuildGlobalacceleratorCustomRoutingEndpointGroupDestinationConfigurationEl {
    pub fn build(self) -> GlobalacceleratorCustomRoutingEndpointGroupDestinationConfigurationEl {
        GlobalacceleratorCustomRoutingEndpointGroupDestinationConfigurationEl {
            from_port: self.from_port,
            protocols: self.protocols,
            to_port: self.to_port,
        }
    }
}

pub struct GlobalacceleratorCustomRoutingEndpointGroupDestinationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlobalacceleratorCustomRoutingEndpointGroupDestinationConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlobalacceleratorCustomRoutingEndpointGroupDestinationConfigurationElRef {
        GlobalacceleratorCustomRoutingEndpointGroupDestinationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorCustomRoutingEndpointGroupDestinationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.protocols", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct GlobalacceleratorCustomRoutingEndpointGroupEndpointConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_id: Option<PrimField<String>>,
}

impl GlobalacceleratorCustomRoutingEndpointGroupEndpointConfigurationEl {
    #[doc = "Set the field `endpoint_id`.\n"]
    pub fn set_endpoint_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint_id = Some(v.into());
        self
    }
}

impl ToListMappable for GlobalacceleratorCustomRoutingEndpointGroupEndpointConfigurationEl {
    type O = BlockAssignable<GlobalacceleratorCustomRoutingEndpointGroupEndpointConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlobalacceleratorCustomRoutingEndpointGroupEndpointConfigurationEl {}

impl BuildGlobalacceleratorCustomRoutingEndpointGroupEndpointConfigurationEl {
    pub fn build(self) -> GlobalacceleratorCustomRoutingEndpointGroupEndpointConfigurationEl {
        GlobalacceleratorCustomRoutingEndpointGroupEndpointConfigurationEl {
            endpoint_id: core::default::Default::default(),
        }
    }
}

pub struct GlobalacceleratorCustomRoutingEndpointGroupEndpointConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlobalacceleratorCustomRoutingEndpointGroupEndpointConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlobalacceleratorCustomRoutingEndpointGroupEndpointConfigurationElRef {
        GlobalacceleratorCustomRoutingEndpointGroupEndpointConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorCustomRoutingEndpointGroupEndpointConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.base))
    }
}

#[derive(Serialize)]
pub struct GlobalacceleratorCustomRoutingEndpointGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl GlobalacceleratorCustomRoutingEndpointGroupTimeoutsEl {
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
}

impl ToListMappable for GlobalacceleratorCustomRoutingEndpointGroupTimeoutsEl {
    type O = BlockAssignable<GlobalacceleratorCustomRoutingEndpointGroupTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlobalacceleratorCustomRoutingEndpointGroupTimeoutsEl {}

impl BuildGlobalacceleratorCustomRoutingEndpointGroupTimeoutsEl {
    pub fn build(self) -> GlobalacceleratorCustomRoutingEndpointGroupTimeoutsEl {
        GlobalacceleratorCustomRoutingEndpointGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct GlobalacceleratorCustomRoutingEndpointGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlobalacceleratorCustomRoutingEndpointGroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GlobalacceleratorCustomRoutingEndpointGroupTimeoutsElRef {
        GlobalacceleratorCustomRoutingEndpointGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorCustomRoutingEndpointGroupTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct GlobalacceleratorCustomRoutingEndpointGroupDynamic {
    destination_configuration: Option<
        DynamicBlock<GlobalacceleratorCustomRoutingEndpointGroupDestinationConfigurationEl>,
    >,
    endpoint_configuration: Option<
        DynamicBlock<GlobalacceleratorCustomRoutingEndpointGroupEndpointConfigurationEl>,
    >,
}
