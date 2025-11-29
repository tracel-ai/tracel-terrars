use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2TransitGatewayAttachmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_attachment_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataEc2TransitGatewayAttachmentFilterEl>>,
    dynamic: DataEc2TransitGatewayAttachmentDynamic,
}

struct DataEc2TransitGatewayAttachment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEc2TransitGatewayAttachmentData>,
}

#[derive(Clone)]
pub struct DataEc2TransitGatewayAttachment(Rc<DataEc2TransitGatewayAttachment_>);

impl DataEc2TransitGatewayAttachment {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_attachment_id`.\n"]
    pub fn set_transit_gateway_attachment_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transit_gateway_attachment_id = Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataEc2TransitGatewayAttachmentFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association_state` after provisioning.\n"]
    pub fn association_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.association_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association_transit_gateway_route_table_id` after provisioning.\n"]
    pub fn association_transit_gateway_route_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_transit_gateway_route_table_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_owner_id` after provisioning.\n"]
    pub fn resource_owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_attachment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_owner_id` after provisioning.\n"]
    pub fn transit_gateway_owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_owner_id", self.extract_ref()))
    }
}

impl Referable for DataEc2TransitGatewayAttachment {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataEc2TransitGatewayAttachment { }

impl ToListMappable for DataEc2TransitGatewayAttachment {
    type O = ListRef<DataEc2TransitGatewayAttachmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEc2TransitGatewayAttachment_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ec2_transit_gateway_attachment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2TransitGatewayAttachment {
    pub tf_id: String,
}

impl BuildDataEc2TransitGatewayAttachment {
    pub fn build(self, stack: &mut Stack) -> DataEc2TransitGatewayAttachment {
        let out = DataEc2TransitGatewayAttachment(Rc::new(DataEc2TransitGatewayAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEc2TransitGatewayAttachmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                transit_gateway_attachment_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2TransitGatewayAttachmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2TransitGatewayAttachmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataEc2TransitGatewayAttachmentRef {
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

    #[doc = "Get a reference to the value of field `association_state` after provisioning.\n"]
    pub fn association_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.association_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association_transit_gateway_route_table_id` after provisioning.\n"]
    pub fn association_transit_gateway_route_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_transit_gateway_route_table_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_owner_id` after provisioning.\n"]
    pub fn resource_owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_attachment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_owner_id` after provisioning.\n"]
    pub fn transit_gateway_owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_owner_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEc2TransitGatewayAttachmentFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataEc2TransitGatewayAttachmentFilterEl { }

impl ToListMappable for DataEc2TransitGatewayAttachmentFilterEl {
    type O = BlockAssignable<DataEc2TransitGatewayAttachmentFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayAttachmentFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataEc2TransitGatewayAttachmentFilterEl {
    pub fn build(self) -> DataEc2TransitGatewayAttachmentFilterEl {
        DataEc2TransitGatewayAttachmentFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2TransitGatewayAttachmentFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2TransitGatewayAttachmentFilterElRef {
    fn new(shared: StackShared, base: String) -> DataEc2TransitGatewayAttachmentFilterElRef {
        DataEc2TransitGatewayAttachmentFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayAttachmentFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEc2TransitGatewayAttachmentDynamic {
    filter: Option<DynamicBlock<DataEc2TransitGatewayAttachmentFilterEl>>,
}
