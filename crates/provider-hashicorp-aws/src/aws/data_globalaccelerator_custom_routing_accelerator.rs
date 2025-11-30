use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataGlobalacceleratorCustomRoutingAcceleratorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataGlobalacceleratorCustomRoutingAccelerator_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataGlobalacceleratorCustomRoutingAcceleratorData>,
}

#[derive(Clone)]
pub struct DataGlobalacceleratorCustomRoutingAccelerator(
    Rc<DataGlobalacceleratorCustomRoutingAccelerator_>,
);

impl DataGlobalacceleratorCustomRoutingAccelerator {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(
        &self,
    ) -> ListRef<DataGlobalacceleratorCustomRoutingAcceleratorAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hosted_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ip_sets` after provisioning.\n"]
    pub fn ip_sets(&self) -> ListRef<DataGlobalacceleratorCustomRoutingAcceleratorIpSetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_sets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}

impl Referable for DataGlobalacceleratorCustomRoutingAccelerator {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataGlobalacceleratorCustomRoutingAccelerator {}

impl ToListMappable for DataGlobalacceleratorCustomRoutingAccelerator {
    type O = ListRef<DataGlobalacceleratorCustomRoutingAcceleratorRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataGlobalacceleratorCustomRoutingAccelerator_ {
    fn extract_datasource_type(&self) -> String {
        "aws_globalaccelerator_custom_routing_accelerator".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataGlobalacceleratorCustomRoutingAccelerator {
    pub tf_id: String,
}

impl BuildDataGlobalacceleratorCustomRoutingAccelerator {
    pub fn build(self, stack: &mut Stack) -> DataGlobalacceleratorCustomRoutingAccelerator {
        let out = DataGlobalacceleratorCustomRoutingAccelerator(Rc::new(
            DataGlobalacceleratorCustomRoutingAccelerator_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataGlobalacceleratorCustomRoutingAcceleratorData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    arn: core::default::Default::default(),
                    id: core::default::Default::default(),
                    name: core::default::Default::default(),
                    tags: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataGlobalacceleratorCustomRoutingAcceleratorRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGlobalacceleratorCustomRoutingAcceleratorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataGlobalacceleratorCustomRoutingAcceleratorRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(
        &self,
    ) -> ListRef<DataGlobalacceleratorCustomRoutingAcceleratorAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hosted_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ip_sets` after provisioning.\n"]
    pub fn ip_sets(&self) -> ListRef<DataGlobalacceleratorCustomRoutingAcceleratorIpSetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_sets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataGlobalacceleratorCustomRoutingAcceleratorAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_logs_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_logs_s3_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_logs_s3_prefix: Option<PrimField<String>>,
}

impl DataGlobalacceleratorCustomRoutingAcceleratorAttributesEl {
    #[doc = "Set the field `flow_logs_enabled`.\n"]
    pub fn set_flow_logs_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.flow_logs_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `flow_logs_s3_bucket`.\n"]
    pub fn set_flow_logs_s3_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.flow_logs_s3_bucket = Some(v.into());
        self
    }

    #[doc = "Set the field `flow_logs_s3_prefix`.\n"]
    pub fn set_flow_logs_s3_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.flow_logs_s3_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for DataGlobalacceleratorCustomRoutingAcceleratorAttributesEl {
    type O = BlockAssignable<DataGlobalacceleratorCustomRoutingAcceleratorAttributesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlobalacceleratorCustomRoutingAcceleratorAttributesEl {}

impl BuildDataGlobalacceleratorCustomRoutingAcceleratorAttributesEl {
    pub fn build(self) -> DataGlobalacceleratorCustomRoutingAcceleratorAttributesEl {
        DataGlobalacceleratorCustomRoutingAcceleratorAttributesEl {
            flow_logs_enabled: core::default::Default::default(),
            flow_logs_s3_bucket: core::default::Default::default(),
            flow_logs_s3_prefix: core::default::Default::default(),
        }
    }
}

pub struct DataGlobalacceleratorCustomRoutingAcceleratorAttributesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGlobalacceleratorCustomRoutingAcceleratorAttributesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGlobalacceleratorCustomRoutingAcceleratorAttributesElRef {
        DataGlobalacceleratorCustomRoutingAcceleratorAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlobalacceleratorCustomRoutingAcceleratorAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `flow_logs_enabled` after provisioning.\n"]
    pub fn flow_logs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.flow_logs_enabled", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `flow_logs_s3_bucket` after provisioning.\n"]
    pub fn flow_logs_s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.flow_logs_s3_bucket", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `flow_logs_s3_prefix` after provisioning.\n"]
    pub fn flow_logs_s3_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.flow_logs_s3_prefix", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataGlobalacceleratorCustomRoutingAcceleratorIpSetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_family: Option<PrimField<String>>,
}

impl DataGlobalacceleratorCustomRoutingAcceleratorIpSetsEl {
    #[doc = "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_addresses = Some(v.into());
        self
    }

    #[doc = "Set the field `ip_family`.\n"]
    pub fn set_ip_family(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_family = Some(v.into());
        self
    }
}

impl ToListMappable for DataGlobalacceleratorCustomRoutingAcceleratorIpSetsEl {
    type O = BlockAssignable<DataGlobalacceleratorCustomRoutingAcceleratorIpSetsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlobalacceleratorCustomRoutingAcceleratorIpSetsEl {}

impl BuildDataGlobalacceleratorCustomRoutingAcceleratorIpSetsEl {
    pub fn build(self) -> DataGlobalacceleratorCustomRoutingAcceleratorIpSetsEl {
        DataGlobalacceleratorCustomRoutingAcceleratorIpSetsEl {
            ip_addresses: core::default::Default::default(),
            ip_family: core::default::Default::default(),
        }
    }
}

pub struct DataGlobalacceleratorCustomRoutingAcceleratorIpSetsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGlobalacceleratorCustomRoutingAcceleratorIpSetsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGlobalacceleratorCustomRoutingAcceleratorIpSetsElRef {
        DataGlobalacceleratorCustomRoutingAcceleratorIpSetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlobalacceleratorCustomRoutingAcceleratorIpSetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_family` after provisioning.\n"]
    pub fn ip_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_family", self.base))
    }
}
