use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2TransitGatewayVpcAttachmentsData {
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
    filter: Option<Vec<DataEc2TransitGatewayVpcAttachmentsFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataEc2TransitGatewayVpcAttachmentsTimeoutsEl>,
    dynamic: DataEc2TransitGatewayVpcAttachmentsDynamic,
}

struct DataEc2TransitGatewayVpcAttachments_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEc2TransitGatewayVpcAttachmentsData>,
}

#[derive(Clone)]
pub struct DataEc2TransitGatewayVpcAttachments(Rc<DataEc2TransitGatewayVpcAttachments_>);

impl DataEc2TransitGatewayVpcAttachments {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataEc2TransitGatewayVpcAttachmentsFilterEl>>) -> Self {
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataEc2TransitGatewayVpcAttachmentsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ids` after provisioning.\n"]
    pub fn ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2TransitGatewayVpcAttachmentsTimeoutsElRef {
        DataEc2TransitGatewayVpcAttachmentsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for DataEc2TransitGatewayVpcAttachments {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataEc2TransitGatewayVpcAttachments { }

impl ToListMappable for DataEc2TransitGatewayVpcAttachments {
    type O = ListRef<DataEc2TransitGatewayVpcAttachmentsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEc2TransitGatewayVpcAttachments_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ec2_transit_gateway_vpc_attachments".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2TransitGatewayVpcAttachments {
    pub tf_id: String,
}

impl BuildDataEc2TransitGatewayVpcAttachments {
    pub fn build(self, stack: &mut Stack) -> DataEc2TransitGatewayVpcAttachments {
        let out = DataEc2TransitGatewayVpcAttachments(Rc::new(DataEc2TransitGatewayVpcAttachments_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEc2TransitGatewayVpcAttachmentsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2TransitGatewayVpcAttachmentsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2TransitGatewayVpcAttachmentsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataEc2TransitGatewayVpcAttachmentsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ids` after provisioning.\n"]
    pub fn ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2TransitGatewayVpcAttachmentsTimeoutsElRef {
        DataEc2TransitGatewayVpcAttachmentsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2TransitGatewayVpcAttachmentsFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataEc2TransitGatewayVpcAttachmentsFilterEl { }

impl ToListMappable for DataEc2TransitGatewayVpcAttachmentsFilterEl {
    type O = BlockAssignable<DataEc2TransitGatewayVpcAttachmentsFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayVpcAttachmentsFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataEc2TransitGatewayVpcAttachmentsFilterEl {
    pub fn build(self) -> DataEc2TransitGatewayVpcAttachmentsFilterEl {
        DataEc2TransitGatewayVpcAttachmentsFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2TransitGatewayVpcAttachmentsFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2TransitGatewayVpcAttachmentsFilterElRef {
    fn new(shared: StackShared, base: String) -> DataEc2TransitGatewayVpcAttachmentsFilterElRef {
        DataEc2TransitGatewayVpcAttachmentsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayVpcAttachmentsFilterElRef {
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

#[derive(Serialize)]
pub struct DataEc2TransitGatewayVpcAttachmentsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataEc2TransitGatewayVpcAttachmentsTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2TransitGatewayVpcAttachmentsTimeoutsEl {
    type O = BlockAssignable<DataEc2TransitGatewayVpcAttachmentsTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayVpcAttachmentsTimeoutsEl {}

impl BuildDataEc2TransitGatewayVpcAttachmentsTimeoutsEl {
    pub fn build(self) -> DataEc2TransitGatewayVpcAttachmentsTimeoutsEl {
        DataEc2TransitGatewayVpcAttachmentsTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataEc2TransitGatewayVpcAttachmentsTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2TransitGatewayVpcAttachmentsTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataEc2TransitGatewayVpcAttachmentsTimeoutsElRef {
        DataEc2TransitGatewayVpcAttachmentsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayVpcAttachmentsTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEc2TransitGatewayVpcAttachmentsDynamic {
    filter: Option<DynamicBlock<DataEc2TransitGatewayVpcAttachmentsFilterEl>>,
}
