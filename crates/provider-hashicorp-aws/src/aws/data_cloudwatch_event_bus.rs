use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudwatchEventBusData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataCloudwatchEventBus_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudwatchEventBusData>,
}

#[derive(Clone)]
pub struct DataCloudwatchEventBus(Rc<DataCloudwatchEventBus_>);

impl DataCloudwatchEventBus {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> ListRef<DataCloudwatchEventBusDeadLetterConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_identifier` after provisioning.\n"]
    pub fn kms_key_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<DataCloudwatchEventBusLogConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
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
}

impl Referable for DataCloudwatchEventBus {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataCloudwatchEventBus { }

impl ToListMappable for DataCloudwatchEventBus {
    type O = ListRef<DataCloudwatchEventBusRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCloudwatchEventBus_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cloudwatch_event_bus".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudwatchEventBus {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataCloudwatchEventBus {
    pub fn build(self, stack: &mut Stack) -> DataCloudwatchEventBus {
        let out = DataCloudwatchEventBus(Rc::new(DataCloudwatchEventBus_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudwatchEventBusData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudwatchEventBusRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudwatchEventBusRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataCloudwatchEventBusRef {
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

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(&self) -> ListRef<DataCloudwatchEventBusDeadLetterConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_identifier` after provisioning.\n"]
    pub fn kms_key_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<DataCloudwatchEventBusLogConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataCloudwatchEventBusDeadLetterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
}

impl DataCloudwatchEventBusDeadLetterConfigEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataCloudwatchEventBusDeadLetterConfigEl {
    type O = BlockAssignable<DataCloudwatchEventBusDeadLetterConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchEventBusDeadLetterConfigEl {}

impl BuildDataCloudwatchEventBusDeadLetterConfigEl {
    pub fn build(self) -> DataCloudwatchEventBusDeadLetterConfigEl {
        DataCloudwatchEventBusDeadLetterConfigEl { arn: core::default::Default::default() }
    }
}

pub struct DataCloudwatchEventBusDeadLetterConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudwatchEventBusDeadLetterConfigElRef {
    fn new(shared: StackShared, base: String) -> DataCloudwatchEventBusDeadLetterConfigElRef {
        DataCloudwatchEventBusDeadLetterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchEventBusDeadLetterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudwatchEventBusLogConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_detail: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<PrimField<String>>,
}

impl DataCloudwatchEventBusLogConfigEl {
    #[doc = "Set the field `include_detail`.\n"]
    pub fn set_include_detail(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.include_detail = Some(v.into());
        self
    }

    #[doc = "Set the field `level`.\n"]
    pub fn set_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.level = Some(v.into());
        self
    }
}

impl ToListMappable for DataCloudwatchEventBusLogConfigEl {
    type O = BlockAssignable<DataCloudwatchEventBusLogConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchEventBusLogConfigEl {}

impl BuildDataCloudwatchEventBusLogConfigEl {
    pub fn build(self) -> DataCloudwatchEventBusLogConfigEl {
        DataCloudwatchEventBusLogConfigEl {
            include_detail: core::default::Default::default(),
            level: core::default::Default::default(),
        }
    }
}

pub struct DataCloudwatchEventBusLogConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudwatchEventBusLogConfigElRef {
    fn new(shared: StackShared, base: String) -> DataCloudwatchEventBusLogConfigElRef {
        DataCloudwatchEventBusLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchEventBusLogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_detail` after provisioning.\n"]
    pub fn include_detail(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_detail", self.base))
    }

    #[doc = "Get a reference to the value of field `level` after provisioning.\n"]
    pub fn level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.level", self.base))
    }
}
