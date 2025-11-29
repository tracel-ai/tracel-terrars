use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RedshiftDataShareConsumerAssociationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_writes: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    associate_entire_account: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_region: Option<PrimField<String>>,
    data_share_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct RedshiftDataShareConsumerAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RedshiftDataShareConsumerAssociationData>,
}

#[derive(Clone)]
pub struct RedshiftDataShareConsumerAssociation(Rc<RedshiftDataShareConsumerAssociation_>);

impl RedshiftDataShareConsumerAssociation {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => true,
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `allow_writes`.\n"]
    pub fn set_allow_writes(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_writes = Some(v.into());
        self
    }

    #[doc = "Set the field `associate_entire_account`.\n"]
    pub fn set_associate_entire_account(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().associate_entire_account = Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_arn`.\n"]
    pub fn set_consumer_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().consumer_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_region`.\n"]
    pub fn set_consumer_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().consumer_region = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `allow_writes` after provisioning.\n"]
    pub fn allow_writes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_writes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associate_entire_account` after provisioning.\n"]
    pub fn associate_entire_account(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.associate_entire_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `consumer_arn` after provisioning.\n"]
    pub fn consumer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.consumer_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `consumer_region` after provisioning.\n"]
    pub fn consumer_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.consumer_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_share_arn` after provisioning.\n"]
    pub fn data_share_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_share_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_by` after provisioning.\n"]
    pub fn managed_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.managed_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `producer_arn` after provisioning.\n"]
    pub fn producer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.producer_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for RedshiftDataShareConsumerAssociation {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RedshiftDataShareConsumerAssociation { }

impl ToListMappable for RedshiftDataShareConsumerAssociation {
    type O = ListRef<RedshiftDataShareConsumerAssociationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RedshiftDataShareConsumerAssociation_ {
    fn extract_resource_type(&self) -> String {
        "aws_redshift_data_share_consumer_association".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRedshiftDataShareConsumerAssociation {
    pub tf_id: String,
    #[doc = ""]
    pub data_share_arn: PrimField<String>,
}

impl BuildRedshiftDataShareConsumerAssociation {
    pub fn build(self, stack: &mut Stack) -> RedshiftDataShareConsumerAssociation {
        let out = RedshiftDataShareConsumerAssociation(Rc::new(RedshiftDataShareConsumerAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RedshiftDataShareConsumerAssociationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allow_writes: core::default::Default::default(),
                associate_entire_account: core::default::Default::default(),
                consumer_arn: core::default::Default::default(),
                consumer_region: core::default::Default::default(),
                data_share_arn: self.data_share_arn,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RedshiftDataShareConsumerAssociationRef {
    shared: StackShared,
    base: String,
}

impl Ref for RedshiftDataShareConsumerAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl RedshiftDataShareConsumerAssociationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_writes` after provisioning.\n"]
    pub fn allow_writes(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_writes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associate_entire_account` after provisioning.\n"]
    pub fn associate_entire_account(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.associate_entire_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `consumer_arn` after provisioning.\n"]
    pub fn consumer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.consumer_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `consumer_region` after provisioning.\n"]
    pub fn consumer_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.consumer_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_share_arn` after provisioning.\n"]
    pub fn data_share_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_share_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_by` after provisioning.\n"]
    pub fn managed_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.managed_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `producer_arn` after provisioning.\n"]
    pub fn producer_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.producer_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}
