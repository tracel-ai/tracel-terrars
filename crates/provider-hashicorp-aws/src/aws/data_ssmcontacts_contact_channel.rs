use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataSsmcontactsContactChannelData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataSsmcontactsContactChannel_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSsmcontactsContactChannelData>,
}

#[derive(Clone)]
pub struct DataSsmcontactsContactChannel(Rc<DataSsmcontactsContactChannel_>);

impl DataSsmcontactsContactChannel {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `activation_status` after provisioning.\n"]
    pub fn activation_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activation_status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `contact_id` after provisioning.\n"]
    pub fn contact_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contact_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delivery_address` after provisioning.\n"]
    pub fn delivery_address(&self) -> ListRef<DataSsmcontactsContactChannelDeliveryAddressElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delivery_address", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}

impl Referable for DataSsmcontactsContactChannel {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataSsmcontactsContactChannel {}

impl ToListMappable for DataSsmcontactsContactChannel {
    type O = ListRef<DataSsmcontactsContactChannelRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSsmcontactsContactChannel_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ssmcontacts_contact_channel".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsmcontactsContactChannel {
    pub tf_id: String,
    #[doc = ""]
    pub arn: PrimField<String>,
}

impl BuildDataSsmcontactsContactChannel {
    pub fn build(self, stack: &mut Stack) -> DataSsmcontactsContactChannel {
        let out = DataSsmcontactsContactChannel(Rc::new(DataSsmcontactsContactChannel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSsmcontactsContactChannelData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                arn: self.arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsmcontactsContactChannelRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsContactChannelRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataSsmcontactsContactChannelRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `activation_status` after provisioning.\n"]
    pub fn activation_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activation_status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `contact_id` after provisioning.\n"]
    pub fn contact_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contact_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delivery_address` after provisioning.\n"]
    pub fn delivery_address(&self) -> ListRef<DataSsmcontactsContactChannelDeliveryAddressElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delivery_address", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsContactChannelDeliveryAddressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    simple_address: Option<PrimField<String>>,
}

impl DataSsmcontactsContactChannelDeliveryAddressEl {
    #[doc = "Set the field `simple_address`.\n"]
    pub fn set_simple_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.simple_address = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmcontactsContactChannelDeliveryAddressEl {
    type O = BlockAssignable<DataSsmcontactsContactChannelDeliveryAddressEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsContactChannelDeliveryAddressEl {}

impl BuildDataSsmcontactsContactChannelDeliveryAddressEl {
    pub fn build(self) -> DataSsmcontactsContactChannelDeliveryAddressEl {
        DataSsmcontactsContactChannelDeliveryAddressEl {
            simple_address: core::default::Default::default(),
        }
    }
}

pub struct DataSsmcontactsContactChannelDeliveryAddressElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsContactChannelDeliveryAddressElRef {
    fn new(shared: StackShared, base: String) -> DataSsmcontactsContactChannelDeliveryAddressElRef {
        DataSsmcontactsContactChannelDeliveryAddressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsContactChannelDeliveryAddressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `simple_address` after provisioning.\n"]
    pub fn simple_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.simple_address", self.base),
        )
    }
}
