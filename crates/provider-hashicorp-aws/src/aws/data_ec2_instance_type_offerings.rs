use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataEc2InstanceTypeOfferingsData {
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
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataEc2InstanceTypeOfferingsFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataEc2InstanceTypeOfferingsTimeoutsEl>,
    dynamic: DataEc2InstanceTypeOfferingsDynamic,
}
struct DataEc2InstanceTypeOfferings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEc2InstanceTypeOfferingsData>,
}
#[derive(Clone)]
pub struct DataEc2InstanceTypeOfferings(Rc<DataEc2InstanceTypeOfferings_>);
impl DataEc2InstanceTypeOfferings {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl Into<BlockAssignable<DataEc2InstanceTypeOfferingsFilterEl>>,
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
    pub fn set_timeouts(self, v: impl Into<DataEc2InstanceTypeOfferingsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_types` after provisioning.\n"]
    pub fn instance_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location_type` after provisioning.\n"]
    pub fn location_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location_types` after provisioning.\n"]
    pub fn location_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.location_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2InstanceTypeOfferingsTimeoutsElRef {
        DataEc2InstanceTypeOfferingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataEc2InstanceTypeOfferings {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataEc2InstanceTypeOfferings {}
impl ToListMappable for DataEc2InstanceTypeOfferings {
    type O = ListRef<DataEc2InstanceTypeOfferingsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataEc2InstanceTypeOfferings_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ec2_instance_type_offerings".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataEc2InstanceTypeOfferings {
    pub tf_id: String,
}
impl BuildDataEc2InstanceTypeOfferings {
    pub fn build(self, stack: &mut Stack) -> DataEc2InstanceTypeOfferings {
        let out = DataEc2InstanceTypeOfferings(Rc::new(DataEc2InstanceTypeOfferings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEc2InstanceTypeOfferingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location_type: core::default::Default::default(),
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
pub struct DataEc2InstanceTypeOfferingsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEc2InstanceTypeOfferingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataEc2InstanceTypeOfferingsRef {
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
    #[doc = "Get a reference to the value of field `instance_types` after provisioning.\n"]
    pub fn instance_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location_type` after provisioning.\n"]
    pub fn location_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location_types` after provisioning.\n"]
    pub fn location_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.location_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2InstanceTypeOfferingsTimeoutsElRef {
        DataEc2InstanceTypeOfferingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataEc2InstanceTypeOfferingsFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}
impl DataEc2InstanceTypeOfferingsFilterEl {}
impl ToListMappable for DataEc2InstanceTypeOfferingsFilterEl {
    type O = BlockAssignable<DataEc2InstanceTypeOfferingsFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEc2InstanceTypeOfferingsFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}
impl BuildDataEc2InstanceTypeOfferingsFilterEl {
    pub fn build(self) -> DataEc2InstanceTypeOfferingsFilterEl {
        DataEc2InstanceTypeOfferingsFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}
pub struct DataEc2InstanceTypeOfferingsFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEc2InstanceTypeOfferingsFilterElRef {
    fn new(shared: StackShared, base: String) -> DataEc2InstanceTypeOfferingsFilterElRef {
        DataEc2InstanceTypeOfferingsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEc2InstanceTypeOfferingsFilterElRef {
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
pub struct DataEc2InstanceTypeOfferingsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}
impl DataEc2InstanceTypeOfferingsTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}
impl ToListMappable for DataEc2InstanceTypeOfferingsTimeoutsEl {
    type O = BlockAssignable<DataEc2InstanceTypeOfferingsTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEc2InstanceTypeOfferingsTimeoutsEl {}
impl BuildDataEc2InstanceTypeOfferingsTimeoutsEl {
    pub fn build(self) -> DataEc2InstanceTypeOfferingsTimeoutsEl {
        DataEc2InstanceTypeOfferingsTimeoutsEl {
            read: core::default::Default::default(),
        }
    }
}
pub struct DataEc2InstanceTypeOfferingsTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEc2InstanceTypeOfferingsTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataEc2InstanceTypeOfferingsTimeoutsElRef {
        DataEc2InstanceTypeOfferingsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEc2InstanceTypeOfferingsTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataEc2InstanceTypeOfferingsDynamic {
    filter: Option<DynamicBlock<DataEc2InstanceTypeOfferingsFilterEl>>,
}
