use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2InstanceTypeOfferingData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_instance_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataEc2InstanceTypeOfferingFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataEc2InstanceTypeOfferingTimeoutsEl>,
    dynamic: DataEc2InstanceTypeOfferingDynamic,
}

struct DataEc2InstanceTypeOffering_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEc2InstanceTypeOfferingData>,
}

#[derive(Clone)]
pub struct DataEc2InstanceTypeOffering(Rc<DataEc2InstanceTypeOffering_>);

impl DataEc2InstanceTypeOffering {
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

    #[doc = "Set the field `location_type`.\n"]
    pub fn set_location_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location_type = Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_instance_types`.\n"]
    pub fn set_preferred_instance_types(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().preferred_instance_types = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataEc2InstanceTypeOfferingFilterEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl Into<DataEc2InstanceTypeOfferingTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location_type` after provisioning.\n"]
    pub fn location_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_instance_types` after provisioning.\n"]
    pub fn preferred_instance_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.preferred_instance_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2InstanceTypeOfferingTimeoutsElRef {
        DataEc2InstanceTypeOfferingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for DataEc2InstanceTypeOffering {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataEc2InstanceTypeOffering { }

impl ToListMappable for DataEc2InstanceTypeOffering {
    type O = ListRef<DataEc2InstanceTypeOfferingRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEc2InstanceTypeOffering_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ec2_instance_type_offering".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2InstanceTypeOffering {
    pub tf_id: String,
}

impl BuildDataEc2InstanceTypeOffering {
    pub fn build(self, stack: &mut Stack) -> DataEc2InstanceTypeOffering {
        let out = DataEc2InstanceTypeOffering(Rc::new(DataEc2InstanceTypeOffering_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEc2InstanceTypeOfferingData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location_type: core::default::Default::default(),
                preferred_instance_types: core::default::Default::default(),
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

pub struct DataEc2InstanceTypeOfferingRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2InstanceTypeOfferingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataEc2InstanceTypeOfferingRef {
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

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location_type` after provisioning.\n"]
    pub fn location_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_instance_types` after provisioning.\n"]
    pub fn preferred_instance_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.preferred_instance_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2InstanceTypeOfferingTimeoutsElRef {
        DataEc2InstanceTypeOfferingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceTypeOfferingFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataEc2InstanceTypeOfferingFilterEl { }

impl ToListMappable for DataEc2InstanceTypeOfferingFilterEl {
    type O = BlockAssignable<DataEc2InstanceTypeOfferingFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2InstanceTypeOfferingFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataEc2InstanceTypeOfferingFilterEl {
    pub fn build(self) -> DataEc2InstanceTypeOfferingFilterEl {
        DataEc2InstanceTypeOfferingFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2InstanceTypeOfferingFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2InstanceTypeOfferingFilterElRef {
    fn new(shared: StackShared, base: String) -> DataEc2InstanceTypeOfferingFilterElRef {
        DataEc2InstanceTypeOfferingFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceTypeOfferingFilterElRef {
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
pub struct DataEc2InstanceTypeOfferingTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataEc2InstanceTypeOfferingTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2InstanceTypeOfferingTimeoutsEl {
    type O = BlockAssignable<DataEc2InstanceTypeOfferingTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2InstanceTypeOfferingTimeoutsEl {}

impl BuildDataEc2InstanceTypeOfferingTimeoutsEl {
    pub fn build(self) -> DataEc2InstanceTypeOfferingTimeoutsEl {
        DataEc2InstanceTypeOfferingTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataEc2InstanceTypeOfferingTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2InstanceTypeOfferingTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataEc2InstanceTypeOfferingTimeoutsElRef {
        DataEc2InstanceTypeOfferingTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceTypeOfferingTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEc2InstanceTypeOfferingDynamic {
    filter: Option<DynamicBlock<DataEc2InstanceTypeOfferingFilterEl>>,
}
