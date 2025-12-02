use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkmanagerDeviceData {
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
    global_network_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serial_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    site_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vendor: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_location: Option<Vec<NetworkmanagerDeviceAwsLocationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<Vec<NetworkmanagerDeviceLocationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkmanagerDeviceTimeoutsEl>,
    dynamic: NetworkmanagerDeviceDynamic,
}
struct NetworkmanagerDevice_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkmanagerDeviceData>,
}
#[derive(Clone)]
pub struct NetworkmanagerDevice(Rc<NetworkmanagerDevice_>);
impl NetworkmanagerDevice {
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
    #[doc = "Set the field `model`.\n"]
    pub fn set_model(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().model = Some(v.into());
        self
    }
    #[doc = "Set the field `serial_number`.\n"]
    pub fn set_serial_number(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().serial_number = Some(v.into());
        self
    }
    #[doc = "Set the field `site_id`.\n"]
    pub fn set_site_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().site_id = Some(v.into());
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
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `vendor`.\n"]
    pub fn set_vendor(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vendor = Some(v.into());
        self
    }
    #[doc = "Set the field `aws_location`.\n"]
    pub fn set_aws_location(
        self,
        v: impl Into<BlockAssignable<NetworkmanagerDeviceAwsLocationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().aws_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.aws_location = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(
        self,
        v: impl Into<BlockAssignable<NetworkmanagerDeviceLocationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.location = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkmanagerDeviceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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
    #[doc = "Get a reference to the value of field `global_network_id` after provisioning.\n"]
    pub fn global_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_network_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.model", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `serial_number` after provisioning.\n"]
    pub fn serial_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serial_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `site_id` after provisioning.\n"]
    pub fn site_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.site_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vendor` after provisioning.\n"]
    pub fn vendor(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vendor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aws_location` after provisioning.\n"]
    pub fn aws_location(&self) -> ListRef<NetworkmanagerDeviceAwsLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aws_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> ListRef<NetworkmanagerDeviceLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerDeviceTimeoutsElRef {
        NetworkmanagerDeviceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkmanagerDevice {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkmanagerDevice {}
impl ToListMappable for NetworkmanagerDevice {
    type O = ListRef<NetworkmanagerDeviceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkmanagerDevice_ {
    fn extract_resource_type(&self) -> String {
        "aws_networkmanager_device".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkmanagerDevice {
    pub tf_id: String,
    #[doc = ""]
    pub global_network_id: PrimField<String>,
}
impl BuildNetworkmanagerDevice {
    pub fn build(self, stack: &mut Stack) -> NetworkmanagerDevice {
        let out = NetworkmanagerDevice(Rc::new(NetworkmanagerDevice_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkmanagerDeviceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                global_network_id: self.global_network_id,
                id: core::default::Default::default(),
                model: core::default::Default::default(),
                serial_number: core::default::Default::default(),
                site_id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: core::default::Default::default(),
                vendor: core::default::Default::default(),
                aws_location: core::default::Default::default(),
                location: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkmanagerDeviceRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkmanagerDeviceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkmanagerDeviceRef {
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
    #[doc = "Get a reference to the value of field `global_network_id` after provisioning.\n"]
    pub fn global_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_network_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.model", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `serial_number` after provisioning.\n"]
    pub fn serial_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serial_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `site_id` after provisioning.\n"]
    pub fn site_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.site_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vendor` after provisioning.\n"]
    pub fn vendor(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vendor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aws_location` after provisioning.\n"]
    pub fn aws_location(&self) -> ListRef<NetworkmanagerDeviceAwsLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aws_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> ListRef<NetworkmanagerDeviceLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerDeviceTimeoutsElRef {
        NetworkmanagerDeviceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkmanagerDeviceAwsLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl NetworkmanagerDeviceAwsLocationEl {
    #[doc = "Set the field `subnet_arn`.\n"]
    pub fn set_subnet_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkmanagerDeviceAwsLocationEl {
    type O = BlockAssignable<NetworkmanagerDeviceAwsLocationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkmanagerDeviceAwsLocationEl {}
impl BuildNetworkmanagerDeviceAwsLocationEl {
    pub fn build(self) -> NetworkmanagerDeviceAwsLocationEl {
        NetworkmanagerDeviceAwsLocationEl {
            subnet_arn: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct NetworkmanagerDeviceAwsLocationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkmanagerDeviceAwsLocationElRef {
    fn new(shared: StackShared, base: String) -> NetworkmanagerDeviceAwsLocationElRef {
        NetworkmanagerDeviceAwsLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkmanagerDeviceAwsLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `subnet_arn` after provisioning.\n"]
    pub fn subnet_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkmanagerDeviceLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latitude: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    longitude: Option<PrimField<String>>,
}
impl NetworkmanagerDeviceLocationEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }
    #[doc = "Set the field `latitude`.\n"]
    pub fn set_latitude(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.latitude = Some(v.into());
        self
    }
    #[doc = "Set the field `longitude`.\n"]
    pub fn set_longitude(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.longitude = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkmanagerDeviceLocationEl {
    type O = BlockAssignable<NetworkmanagerDeviceLocationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkmanagerDeviceLocationEl {}
impl BuildNetworkmanagerDeviceLocationEl {
    pub fn build(self) -> NetworkmanagerDeviceLocationEl {
        NetworkmanagerDeviceLocationEl {
            address: core::default::Default::default(),
            latitude: core::default::Default::default(),
            longitude: core::default::Default::default(),
        }
    }
}
pub struct NetworkmanagerDeviceLocationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkmanagerDeviceLocationElRef {
    fn new(shared: StackShared, base: String) -> NetworkmanagerDeviceLocationElRef {
        NetworkmanagerDeviceLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkmanagerDeviceLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
    #[doc = "Get a reference to the value of field `latitude` after provisioning.\n"]
    pub fn latitude(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.latitude", self.base))
    }
    #[doc = "Get a reference to the value of field `longitude` after provisioning.\n"]
    pub fn longitude(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.longitude", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkmanagerDeviceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkmanagerDeviceTimeoutsEl {
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
impl ToListMappable for NetworkmanagerDeviceTimeoutsEl {
    type O = BlockAssignable<NetworkmanagerDeviceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkmanagerDeviceTimeoutsEl {}
impl BuildNetworkmanagerDeviceTimeoutsEl {
    pub fn build(self) -> NetworkmanagerDeviceTimeoutsEl {
        NetworkmanagerDeviceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkmanagerDeviceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkmanagerDeviceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkmanagerDeviceTimeoutsElRef {
        NetworkmanagerDeviceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkmanagerDeviceTimeoutsElRef {
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
struct NetworkmanagerDeviceDynamic {
    aws_location: Option<DynamicBlock<NetworkmanagerDeviceAwsLocationEl>>,
    location: Option<DynamicBlock<NetworkmanagerDeviceLocationEl>>,
}
