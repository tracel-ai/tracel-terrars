use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GlobalacceleratorCustomRoutingListenerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    accelerator_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<Vec<GlobalacceleratorCustomRoutingListenerPortRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GlobalacceleratorCustomRoutingListenerTimeoutsEl>,
    dynamic: GlobalacceleratorCustomRoutingListenerDynamic,
}
struct GlobalacceleratorCustomRoutingListener_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GlobalacceleratorCustomRoutingListenerData>,
}
#[derive(Clone)]
pub struct GlobalacceleratorCustomRoutingListener(Rc<GlobalacceleratorCustomRoutingListener_>);
impl GlobalacceleratorCustomRoutingListener {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        self,
        v: impl Into<BlockAssignable<GlobalacceleratorCustomRoutingListenerPortRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().port_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.port_range = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<GlobalacceleratorCustomRoutingListenerTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `accelerator_arn` after provisioning.\n"]
    pub fn accelerator_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlobalacceleratorCustomRoutingListenerTimeoutsElRef {
        GlobalacceleratorCustomRoutingListenerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for GlobalacceleratorCustomRoutingListener {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GlobalacceleratorCustomRoutingListener {}
impl ToListMappable for GlobalacceleratorCustomRoutingListener {
    type O = ListRef<GlobalacceleratorCustomRoutingListenerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GlobalacceleratorCustomRoutingListener_ {
    fn extract_resource_type(&self) -> String {
        "aws_globalaccelerator_custom_routing_listener".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGlobalacceleratorCustomRoutingListener {
    pub tf_id: String,
    #[doc = ""]
    pub accelerator_arn: PrimField<String>,
}
impl BuildGlobalacceleratorCustomRoutingListener {
    pub fn build(self, stack: &mut Stack) -> GlobalacceleratorCustomRoutingListener {
        let out = GlobalacceleratorCustomRoutingListener(Rc::new(
            GlobalacceleratorCustomRoutingListener_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(GlobalacceleratorCustomRoutingListenerData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    accelerator_arn: self.accelerator_arn,
                    id: core::default::Default::default(),
                    port_range: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GlobalacceleratorCustomRoutingListenerRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlobalacceleratorCustomRoutingListenerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GlobalacceleratorCustomRoutingListenerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_arn` after provisioning.\n"]
    pub fn accelerator_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlobalacceleratorCustomRoutingListenerTimeoutsElRef {
        GlobalacceleratorCustomRoutingListenerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GlobalacceleratorCustomRoutingListenerPortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_port: Option<PrimField<f64>>,
}
impl GlobalacceleratorCustomRoutingListenerPortRangeEl {
    #[doc = "Set the field `from_port`.\n"]
    pub fn set_from_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from_port = Some(v.into());
        self
    }
    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to_port = Some(v.into());
        self
    }
}
impl ToListMappable for GlobalacceleratorCustomRoutingListenerPortRangeEl {
    type O = BlockAssignable<GlobalacceleratorCustomRoutingListenerPortRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlobalacceleratorCustomRoutingListenerPortRangeEl {}
impl BuildGlobalacceleratorCustomRoutingListenerPortRangeEl {
    pub fn build(self) -> GlobalacceleratorCustomRoutingListenerPortRangeEl {
        GlobalacceleratorCustomRoutingListenerPortRangeEl {
            from_port: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}
pub struct GlobalacceleratorCustomRoutingListenerPortRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlobalacceleratorCustomRoutingListenerPortRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlobalacceleratorCustomRoutingListenerPortRangeElRef {
        GlobalacceleratorCustomRoutingListenerPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlobalacceleratorCustomRoutingListenerPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }
    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}
#[derive(Serialize)]
pub struct GlobalacceleratorCustomRoutingListenerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl GlobalacceleratorCustomRoutingListenerTimeoutsEl {
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
impl ToListMappable for GlobalacceleratorCustomRoutingListenerTimeoutsEl {
    type O = BlockAssignable<GlobalacceleratorCustomRoutingListenerTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlobalacceleratorCustomRoutingListenerTimeoutsEl {}
impl BuildGlobalacceleratorCustomRoutingListenerTimeoutsEl {
    pub fn build(self) -> GlobalacceleratorCustomRoutingListenerTimeoutsEl {
        GlobalacceleratorCustomRoutingListenerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct GlobalacceleratorCustomRoutingListenerTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlobalacceleratorCustomRoutingListenerTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlobalacceleratorCustomRoutingListenerTimeoutsElRef {
        GlobalacceleratorCustomRoutingListenerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlobalacceleratorCustomRoutingListenerTimeoutsElRef {
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
struct GlobalacceleratorCustomRoutingListenerDynamic {
    port_range: Option<DynamicBlock<GlobalacceleratorCustomRoutingListenerPortRangeEl>>,
}
