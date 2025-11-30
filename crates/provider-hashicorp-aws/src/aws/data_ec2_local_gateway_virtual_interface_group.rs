use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataEc2LocalGatewayVirtualInterfaceGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataEc2LocalGatewayVirtualInterfaceGroupFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataEc2LocalGatewayVirtualInterfaceGroupTimeoutsEl>,
    dynamic: DataEc2LocalGatewayVirtualInterfaceGroupDynamic,
}

struct DataEc2LocalGatewayVirtualInterfaceGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEc2LocalGatewayVirtualInterfaceGroupData>,
}

#[derive(Clone)]
pub struct DataEc2LocalGatewayVirtualInterfaceGroup(Rc<DataEc2LocalGatewayVirtualInterfaceGroup_>);

impl DataEc2LocalGatewayVirtualInterfaceGroup {
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

    #[doc = "Set the field `local_gateway_id`.\n"]
    pub fn set_local_gateway_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().local_gateway_id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl Into<BlockAssignable<DataEc2LocalGatewayVirtualInterfaceGroupFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<DataEc2LocalGatewayVirtualInterfaceGroupTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_gateway_id` after provisioning.\n"]
    pub fn local_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_gateway_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `local_gateway_virtual_interface_ids` after provisioning.\n"]
    pub fn local_gateway_virtual_interface_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.local_gateway_virtual_interface_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2LocalGatewayVirtualInterfaceGroupTimeoutsElRef {
        DataEc2LocalGatewayVirtualInterfaceGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for DataEc2LocalGatewayVirtualInterfaceGroup {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataEc2LocalGatewayVirtualInterfaceGroup {}

impl ToListMappable for DataEc2LocalGatewayVirtualInterfaceGroup {
    type O = ListRef<DataEc2LocalGatewayVirtualInterfaceGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEc2LocalGatewayVirtualInterfaceGroup_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ec2_local_gateway_virtual_interface_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2LocalGatewayVirtualInterfaceGroup {
    pub tf_id: String,
}

impl BuildDataEc2LocalGatewayVirtualInterfaceGroup {
    pub fn build(self, stack: &mut Stack) -> DataEc2LocalGatewayVirtualInterfaceGroup {
        let out = DataEc2LocalGatewayVirtualInterfaceGroup(Rc::new(
            DataEc2LocalGatewayVirtualInterfaceGroup_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataEc2LocalGatewayVirtualInterfaceGroupData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    local_gateway_id: core::default::Default::default(),
                    region: core::default::Default::default(),
                    tags: core::default::Default::default(),
                    filter: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2LocalGatewayVirtualInterfaceGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2LocalGatewayVirtualInterfaceGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataEc2LocalGatewayVirtualInterfaceGroupRef {
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

    #[doc = "Get a reference to the value of field `local_gateway_id` after provisioning.\n"]
    pub fn local_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_gateway_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `local_gateway_virtual_interface_ids` after provisioning.\n"]
    pub fn local_gateway_virtual_interface_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.local_gateway_virtual_interface_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2LocalGatewayVirtualInterfaceGroupTimeoutsElRef {
        DataEc2LocalGatewayVirtualInterfaceGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2LocalGatewayVirtualInterfaceGroupFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataEc2LocalGatewayVirtualInterfaceGroupFilterEl {}

impl ToListMappable for DataEc2LocalGatewayVirtualInterfaceGroupFilterEl {
    type O = BlockAssignable<DataEc2LocalGatewayVirtualInterfaceGroupFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2LocalGatewayVirtualInterfaceGroupFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataEc2LocalGatewayVirtualInterfaceGroupFilterEl {
    pub fn build(self) -> DataEc2LocalGatewayVirtualInterfaceGroupFilterEl {
        DataEc2LocalGatewayVirtualInterfaceGroupFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2LocalGatewayVirtualInterfaceGroupFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2LocalGatewayVirtualInterfaceGroupFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEc2LocalGatewayVirtualInterfaceGroupFilterElRef {
        DataEc2LocalGatewayVirtualInterfaceGroupFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2LocalGatewayVirtualInterfaceGroupFilterElRef {
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
pub struct DataEc2LocalGatewayVirtualInterfaceGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataEc2LocalGatewayVirtualInterfaceGroupTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2LocalGatewayVirtualInterfaceGroupTimeoutsEl {
    type O = BlockAssignable<DataEc2LocalGatewayVirtualInterfaceGroupTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2LocalGatewayVirtualInterfaceGroupTimeoutsEl {}

impl BuildDataEc2LocalGatewayVirtualInterfaceGroupTimeoutsEl {
    pub fn build(self) -> DataEc2LocalGatewayVirtualInterfaceGroupTimeoutsEl {
        DataEc2LocalGatewayVirtualInterfaceGroupTimeoutsEl {
            read: core::default::Default::default(),
        }
    }
}

pub struct DataEc2LocalGatewayVirtualInterfaceGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2LocalGatewayVirtualInterfaceGroupTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEc2LocalGatewayVirtualInterfaceGroupTimeoutsElRef {
        DataEc2LocalGatewayVirtualInterfaceGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2LocalGatewayVirtualInterfaceGroupTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEc2LocalGatewayVirtualInterfaceGroupDynamic {
    filter: Option<DynamicBlock<DataEc2LocalGatewayVirtualInterfaceGroupFilterEl>>,
}
