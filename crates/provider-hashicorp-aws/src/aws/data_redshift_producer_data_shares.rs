use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataRedshiftProducerDataSharesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    producer_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

struct DataRedshiftProducerDataShares_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRedshiftProducerDataSharesData>,
}

#[derive(Clone)]
pub struct DataRedshiftProducerDataShares(Rc<DataRedshiftProducerDataShares_>);

impl DataRedshiftProducerDataShares {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `data_shares` after provisioning.\n"]
    pub fn data_shares(&self) -> ListRef<DataRedshiftProducerDataSharesDataSharesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_shares", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `producer_arn` after provisioning.\n"]
    pub fn producer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.producer_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
}

impl Referable for DataRedshiftProducerDataShares {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataRedshiftProducerDataShares {}

impl ToListMappable for DataRedshiftProducerDataShares {
    type O = ListRef<DataRedshiftProducerDataSharesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRedshiftProducerDataShares_ {
    fn extract_datasource_type(&self) -> String {
        "aws_redshift_producer_data_shares".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRedshiftProducerDataShares {
    pub tf_id: String,
    #[doc = ""]
    pub producer_arn: PrimField<String>,
}

impl BuildDataRedshiftProducerDataShares {
    pub fn build(self, stack: &mut Stack) -> DataRedshiftProducerDataShares {
        let out = DataRedshiftProducerDataShares(Rc::new(DataRedshiftProducerDataShares_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRedshiftProducerDataSharesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                producer_arn: self.producer_arn,
                region: core::default::Default::default(),
                status: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRedshiftProducerDataSharesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRedshiftProducerDataSharesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataRedshiftProducerDataSharesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `data_shares` after provisioning.\n"]
    pub fn data_shares(&self) -> ListRef<DataRedshiftProducerDataSharesDataSharesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_shares", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `producer_arn` after provisioning.\n"]
    pub fn producer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.producer_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataRedshiftProducerDataSharesDataSharesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_share_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_by: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    producer_arn: Option<PrimField<String>>,
}

impl DataRedshiftProducerDataSharesDataSharesEl {
    #[doc = "Set the field `data_share_arn`.\n"]
    pub fn set_data_share_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_share_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `managed_by`.\n"]
    pub fn set_managed_by(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.managed_by = Some(v.into());
        self
    }

    #[doc = "Set the field `producer_arn`.\n"]
    pub fn set_producer_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.producer_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataRedshiftProducerDataSharesDataSharesEl {
    type O = BlockAssignable<DataRedshiftProducerDataSharesDataSharesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedshiftProducerDataSharesDataSharesEl {}

impl BuildDataRedshiftProducerDataSharesDataSharesEl {
    pub fn build(self) -> DataRedshiftProducerDataSharesDataSharesEl {
        DataRedshiftProducerDataSharesDataSharesEl {
            data_share_arn: core::default::Default::default(),
            managed_by: core::default::Default::default(),
            producer_arn: core::default::Default::default(),
        }
    }
}

pub struct DataRedshiftProducerDataSharesDataSharesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRedshiftProducerDataSharesDataSharesElRef {
    fn new(shared: StackShared, base: String) -> DataRedshiftProducerDataSharesDataSharesElRef {
        DataRedshiftProducerDataSharesDataSharesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedshiftProducerDataSharesDataSharesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_share_arn` after provisioning.\n"]
    pub fn data_share_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_share_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `managed_by` after provisioning.\n"]
    pub fn managed_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.managed_by", self.base))
    }

    #[doc = "Get a reference to the value of field `producer_arn` after provisioning.\n"]
    pub fn producer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.producer_arn", self.base))
    }
}
