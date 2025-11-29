use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2CapacityBlockOfferingData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    capacity_duration_hours: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date_range: Option<PrimField<String>>,
    instance_count: PrimField<f64>,
    instance_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date_range: Option<PrimField<String>>,
}

struct DataEc2CapacityBlockOffering_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEc2CapacityBlockOfferingData>,
}

#[derive(Clone)]
pub struct DataEc2CapacityBlockOffering(Rc<DataEc2CapacityBlockOffering_>);

impl DataEc2CapacityBlockOffering {
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

    #[doc = "Set the field `end_date_range`.\n"]
    pub fn set_end_date_range(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().end_date_range = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `start_date_range`.\n"]
    pub fn set_start_date_range(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().start_date_range = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_block_offering_id` after provisioning.\n"]
    pub fn capacity_block_offering_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_block_offering_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_duration_hours` after provisioning.\n"]
    pub fn capacity_duration_hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_duration_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `currency_code` after provisioning.\n"]
    pub fn currency_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.currency_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end_date_range` after provisioning.\n"]
    pub fn end_date_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_date_range", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_date_range` after provisioning.\n"]
    pub fn start_date_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_date_range", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upfront_fee` after provisioning.\n"]
    pub fn upfront_fee(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.upfront_fee", self.extract_ref()))
    }
}

impl Referable for DataEc2CapacityBlockOffering {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataEc2CapacityBlockOffering { }

impl ToListMappable for DataEc2CapacityBlockOffering {
    type O = ListRef<DataEc2CapacityBlockOfferingRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEc2CapacityBlockOffering_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ec2_capacity_block_offering".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2CapacityBlockOffering {
    pub tf_id: String,
    #[doc = ""]
    pub capacity_duration_hours: PrimField<f64>,
    #[doc = ""]
    pub instance_count: PrimField<f64>,
    #[doc = ""]
    pub instance_type: PrimField<String>,
}

impl BuildDataEc2CapacityBlockOffering {
    pub fn build(self, stack: &mut Stack) -> DataEc2CapacityBlockOffering {
        let out = DataEc2CapacityBlockOffering(Rc::new(DataEc2CapacityBlockOffering_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEc2CapacityBlockOfferingData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                capacity_duration_hours: self.capacity_duration_hours,
                end_date_range: core::default::Default::default(),
                instance_count: self.instance_count,
                instance_type: self.instance_type,
                region: core::default::Default::default(),
                start_date_range: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2CapacityBlockOfferingRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2CapacityBlockOfferingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataEc2CapacityBlockOfferingRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_block_offering_id` after provisioning.\n"]
    pub fn capacity_block_offering_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_block_offering_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_duration_hours` after provisioning.\n"]
    pub fn capacity_duration_hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_duration_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `currency_code` after provisioning.\n"]
    pub fn currency_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.currency_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end_date_range` after provisioning.\n"]
    pub fn end_date_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_date_range", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_date_range` after provisioning.\n"]
    pub fn start_date_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_date_range", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upfront_fee` after provisioning.\n"]
    pub fn upfront_fee(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.upfront_fee", self.extract_ref()))
    }
}
