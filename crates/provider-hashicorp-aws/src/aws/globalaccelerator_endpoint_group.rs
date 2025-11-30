use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct GlobalacceleratorEndpointGroupData {
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
    health_check_interval_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    listener_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traffic_dial_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_configuration: Option<Vec<GlobalacceleratorEndpointGroupEndpointConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_override: Option<Vec<GlobalacceleratorEndpointGroupPortOverrideEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GlobalacceleratorEndpointGroupTimeoutsEl>,
    dynamic: GlobalacceleratorEndpointGroupDynamic,
}

struct GlobalacceleratorEndpointGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GlobalacceleratorEndpointGroupData>,
}

#[derive(Clone)]
pub struct GlobalacceleratorEndpointGroup(Rc<GlobalacceleratorEndpointGroup_>);

impl GlobalacceleratorEndpointGroup {
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

    #[doc = "Set the field `endpoint_group_region`.\n"]
    pub fn set_endpoint_group_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().endpoint_group_region = Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_interval_seconds`.\n"]
    pub fn set_health_check_interval_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().health_check_interval_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_path`.\n"]
    pub fn set_health_check_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().health_check_path = Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_port`.\n"]
    pub fn set_health_check_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().health_check_port = Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_protocol`.\n"]
    pub fn set_health_check_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().health_check_protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `threshold_count`.\n"]
    pub fn set_threshold_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().threshold_count = Some(v.into());
        self
    }

    #[doc = "Set the field `traffic_dial_percentage`.\n"]
    pub fn set_traffic_dial_percentage(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().traffic_dial_percentage = Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_configuration`.\n"]
    pub fn set_endpoint_configuration(
        self,
        v: impl Into<BlockAssignable<GlobalacceleratorEndpointGroupEndpointConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `port_override`.\n"]
    pub fn set_port_override(
        self,
        v: impl Into<BlockAssignable<GlobalacceleratorEndpointGroupPortOverrideEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().port_override = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.port_override = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GlobalacceleratorEndpointGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_group_region` after provisioning.\n"]
    pub fn endpoint_group_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_group_region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_interval_seconds` after provisioning.\n"]
    pub fn health_check_interval_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_interval_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_path` after provisioning.\n"]
    pub fn health_check_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_path", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_port` after provisioning.\n"]
    pub fn health_check_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_port", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_protocol` after provisioning.\n"]
    pub fn health_check_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_protocol", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_arn` after provisioning.\n"]
    pub fn listener_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listener_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `threshold_count` after provisioning.\n"]
    pub fn threshold_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threshold_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `traffic_dial_percentage` after provisioning.\n"]
    pub fn traffic_dial_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traffic_dial_percentage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlobalacceleratorEndpointGroupTimeoutsElRef {
        GlobalacceleratorEndpointGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for GlobalacceleratorEndpointGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for GlobalacceleratorEndpointGroup {}

impl ToListMappable for GlobalacceleratorEndpointGroup {
    type O = ListRef<GlobalacceleratorEndpointGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GlobalacceleratorEndpointGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_globalaccelerator_endpoint_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlobalacceleratorEndpointGroup {
    pub tf_id: String,
    #[doc = ""]
    pub listener_arn: PrimField<String>,
}

impl BuildGlobalacceleratorEndpointGroup {
    pub fn build(self, stack: &mut Stack) -> GlobalacceleratorEndpointGroup {
        let out = GlobalacceleratorEndpointGroup(Rc::new(GlobalacceleratorEndpointGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GlobalacceleratorEndpointGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                endpoint_group_region: core::default::Default::default(),
                health_check_interval_seconds: core::default::Default::default(),
                health_check_path: core::default::Default::default(),
                health_check_port: core::default::Default::default(),
                health_check_protocol: core::default::Default::default(),
                id: core::default::Default::default(),
                listener_arn: self.listener_arn,
                threshold_count: core::default::Default::default(),
                traffic_dial_percentage: core::default::Default::default(),
                endpoint_configuration: core::default::Default::default(),
                port_override: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlobalacceleratorEndpointGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlobalacceleratorEndpointGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl GlobalacceleratorEndpointGroupRef {
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_group_region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_interval_seconds` after provisioning.\n"]
    pub fn health_check_interval_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_interval_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_path` after provisioning.\n"]
    pub fn health_check_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_path", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_port` after provisioning.\n"]
    pub fn health_check_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_port", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_protocol` after provisioning.\n"]
    pub fn health_check_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_protocol", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_arn` after provisioning.\n"]
    pub fn listener_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listener_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `threshold_count` after provisioning.\n"]
    pub fn threshold_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.threshold_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `traffic_dial_percentage` after provisioning.\n"]
    pub fn traffic_dial_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traffic_dial_percentage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlobalacceleratorEndpointGroupTimeoutsElRef {
        GlobalacceleratorEndpointGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct GlobalacceleratorEndpointGroupEndpointConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_ip_preservation_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl GlobalacceleratorEndpointGroupEndpointConfigurationEl {
    #[doc = "Set the field `attachment_arn`.\n"]
    pub fn set_attachment_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attachment_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `client_ip_preservation_enabled`.\n"]
    pub fn set_client_ip_preservation_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.client_ip_preservation_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_id`.\n"]
    pub fn set_endpoint_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint_id = Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for GlobalacceleratorEndpointGroupEndpointConfigurationEl {
    type O = BlockAssignable<GlobalacceleratorEndpointGroupEndpointConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlobalacceleratorEndpointGroupEndpointConfigurationEl {}

impl BuildGlobalacceleratorEndpointGroupEndpointConfigurationEl {
    pub fn build(self) -> GlobalacceleratorEndpointGroupEndpointConfigurationEl {
        GlobalacceleratorEndpointGroupEndpointConfigurationEl {
            attachment_arn: core::default::Default::default(),
            client_ip_preservation_enabled: core::default::Default::default(),
            endpoint_id: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

pub struct GlobalacceleratorEndpointGroupEndpointConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlobalacceleratorEndpointGroupEndpointConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlobalacceleratorEndpointGroupEndpointConfigurationElRef {
        GlobalacceleratorEndpointGroupEndpointConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorEndpointGroupEndpointConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment_arn` after provisioning.\n"]
    pub fn attachment_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.attachment_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `client_ip_preservation_enabled` after provisioning.\n"]
    pub fn client_ip_preservation_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_ip_preservation_enabled", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct GlobalacceleratorEndpointGroupPortOverrideEl {
    endpoint_port: PrimField<f64>,
    listener_port: PrimField<f64>,
}

impl GlobalacceleratorEndpointGroupPortOverrideEl {}

impl ToListMappable for GlobalacceleratorEndpointGroupPortOverrideEl {
    type O = BlockAssignable<GlobalacceleratorEndpointGroupPortOverrideEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlobalacceleratorEndpointGroupPortOverrideEl {
    #[doc = ""]
    pub endpoint_port: PrimField<f64>,
    #[doc = ""]
    pub listener_port: PrimField<f64>,
}

impl BuildGlobalacceleratorEndpointGroupPortOverrideEl {
    pub fn build(self) -> GlobalacceleratorEndpointGroupPortOverrideEl {
        GlobalacceleratorEndpointGroupPortOverrideEl {
            endpoint_port: self.endpoint_port,
            listener_port: self.listener_port,
        }
    }
}

pub struct GlobalacceleratorEndpointGroupPortOverrideElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlobalacceleratorEndpointGroupPortOverrideElRef {
    fn new(shared: StackShared, base: String) -> GlobalacceleratorEndpointGroupPortOverrideElRef {
        GlobalacceleratorEndpointGroupPortOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorEndpointGroupPortOverrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_port` after provisioning.\n"]
    pub fn endpoint_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_port", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `listener_port` after provisioning.\n"]
    pub fn listener_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listener_port", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct GlobalacceleratorEndpointGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl GlobalacceleratorEndpointGroupTimeoutsEl {
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

impl ToListMappable for GlobalacceleratorEndpointGroupTimeoutsEl {
    type O = BlockAssignable<GlobalacceleratorEndpointGroupTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlobalacceleratorEndpointGroupTimeoutsEl {}

impl BuildGlobalacceleratorEndpointGroupTimeoutsEl {
    pub fn build(self) -> GlobalacceleratorEndpointGroupTimeoutsEl {
        GlobalacceleratorEndpointGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct GlobalacceleratorEndpointGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlobalacceleratorEndpointGroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GlobalacceleratorEndpointGroupTimeoutsElRef {
        GlobalacceleratorEndpointGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorEndpointGroupTimeoutsElRef {
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
struct GlobalacceleratorEndpointGroupDynamic {
    endpoint_configuration:
        Option<DynamicBlock<GlobalacceleratorEndpointGroupEndpointConfigurationEl>>,
    port_override: Option<DynamicBlock<GlobalacceleratorEndpointGroupPortOverrideEl>>,
}
