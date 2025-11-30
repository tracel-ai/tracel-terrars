use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataGlobalacceleratorAcceleratorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

struct DataGlobalacceleratorAccelerator_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataGlobalacceleratorAcceleratorData>,
}

#[derive(Clone)]
pub struct DataGlobalacceleratorAccelerator(Rc<DataGlobalacceleratorAccelerator_>);

impl DataGlobalacceleratorAccelerator {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> ListRef<DataGlobalacceleratorAcceleratorAttributesElRef> {
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

    #[doc = "Get a reference to the value of field `dual_stack_dns_name` after provisioning.\n"]
    pub fn dual_stack_dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dual_stack_dns_name", self.extract_ref()),
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
    pub fn ip_sets(&self) -> ListRef<DataGlobalacceleratorAcceleratorIpSetsElRef> {
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

impl Referable for DataGlobalacceleratorAccelerator {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataGlobalacceleratorAccelerator {}

impl ToListMappable for DataGlobalacceleratorAccelerator {
    type O = ListRef<DataGlobalacceleratorAcceleratorRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataGlobalacceleratorAccelerator_ {
    fn extract_datasource_type(&self) -> String {
        "aws_globalaccelerator_accelerator".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataGlobalacceleratorAccelerator {
    pub tf_id: String,
}

impl BuildDataGlobalacceleratorAccelerator {
    pub fn build(self, stack: &mut Stack) -> DataGlobalacceleratorAccelerator {
        let out = DataGlobalacceleratorAccelerator(Rc::new(DataGlobalacceleratorAccelerator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataGlobalacceleratorAcceleratorData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                arn: core::default::Default::default(),
                name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataGlobalacceleratorAcceleratorRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGlobalacceleratorAcceleratorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataGlobalacceleratorAcceleratorRef {
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
    pub fn attributes(&self) -> ListRef<DataGlobalacceleratorAcceleratorAttributesElRef> {
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

    #[doc = "Get a reference to the value of field `dual_stack_dns_name` after provisioning.\n"]
    pub fn dual_stack_dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dual_stack_dns_name", self.extract_ref()),
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
    pub fn ip_sets(&self) -> ListRef<DataGlobalacceleratorAcceleratorIpSetsElRef> {
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
pub struct DataGlobalacceleratorAcceleratorAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_logs_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_logs_s3_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_logs_s3_prefix: Option<PrimField<String>>,
}

impl DataGlobalacceleratorAcceleratorAttributesEl {
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

impl ToListMappable for DataGlobalacceleratorAcceleratorAttributesEl {
    type O = BlockAssignable<DataGlobalacceleratorAcceleratorAttributesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlobalacceleratorAcceleratorAttributesEl {}

impl BuildDataGlobalacceleratorAcceleratorAttributesEl {
    pub fn build(self) -> DataGlobalacceleratorAcceleratorAttributesEl {
        DataGlobalacceleratorAcceleratorAttributesEl {
            flow_logs_enabled: core::default::Default::default(),
            flow_logs_s3_bucket: core::default::Default::default(),
            flow_logs_s3_prefix: core::default::Default::default(),
        }
    }
}

pub struct DataGlobalacceleratorAcceleratorAttributesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGlobalacceleratorAcceleratorAttributesElRef {
    fn new(shared: StackShared, base: String) -> DataGlobalacceleratorAcceleratorAttributesElRef {
        DataGlobalacceleratorAcceleratorAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlobalacceleratorAcceleratorAttributesElRef {
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
pub struct DataGlobalacceleratorAcceleratorIpSetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_family: Option<PrimField<String>>,
}

impl DataGlobalacceleratorAcceleratorIpSetsEl {
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

impl ToListMappable for DataGlobalacceleratorAcceleratorIpSetsEl {
    type O = BlockAssignable<DataGlobalacceleratorAcceleratorIpSetsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlobalacceleratorAcceleratorIpSetsEl {}

impl BuildDataGlobalacceleratorAcceleratorIpSetsEl {
    pub fn build(self) -> DataGlobalacceleratorAcceleratorIpSetsEl {
        DataGlobalacceleratorAcceleratorIpSetsEl {
            ip_addresses: core::default::Default::default(),
            ip_family: core::default::Default::default(),
        }
    }
}

pub struct DataGlobalacceleratorAcceleratorIpSetsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGlobalacceleratorAcceleratorIpSetsElRef {
    fn new(shared: StackShared, base: String) -> DataGlobalacceleratorAcceleratorIpSetsElRef {
        DataGlobalacceleratorAcceleratorIpSetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlobalacceleratorAcceleratorIpSetsElRef {
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
