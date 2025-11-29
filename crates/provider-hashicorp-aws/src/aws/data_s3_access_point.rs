use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataS3AccessPointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataS3AccessPoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataS3AccessPointData>,
}

#[derive(Clone)]
pub struct DataS3AccessPoint(Rc<DataS3AccessPoint_>);

impl DataS3AccessPoint {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_account_id` after provisioning.\n"]
    pub fn bucket_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_type` after provisioning.\n"]
    pub fn data_source_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_origin` after provisioning.\n"]
    pub fn network_origin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_access_block_configuration` after provisioning.\n"]
    pub fn public_access_block_configuration(&self) -> ListRef<DataS3AccessPointPublicAccessBlockConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.public_access_block_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_configuration` after provisioning.\n"]
    pub fn vpc_configuration(&self) -> ListRef<DataS3AccessPointVpcConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_configuration", self.extract_ref()))
    }
}

impl Referable for DataS3AccessPoint {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataS3AccessPoint { }

impl ToListMappable for DataS3AccessPoint {
    type O = ListRef<DataS3AccessPointRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataS3AccessPoint_ {
    fn extract_datasource_type(&self) -> String {
        "aws_s3_access_point".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataS3AccessPoint {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataS3AccessPoint {
    pub fn build(self, stack: &mut Stack) -> DataS3AccessPoint {
        let out = DataS3AccessPoint(Rc::new(DataS3AccessPoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataS3AccessPointData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                account_id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataS3AccessPointRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataS3AccessPointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataS3AccessPointRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_account_id` after provisioning.\n"]
    pub fn bucket_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_type` after provisioning.\n"]
    pub fn data_source_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_origin` after provisioning.\n"]
    pub fn network_origin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_access_block_configuration` after provisioning.\n"]
    pub fn public_access_block_configuration(&self) -> ListRef<DataS3AccessPointPublicAccessBlockConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.public_access_block_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_configuration` after provisioning.\n"]
    pub fn vpc_configuration(&self) -> ListRef<DataS3AccessPointVpcConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataS3AccessPointPublicAccessBlockConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    block_public_acls: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_public_policy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_public_acls: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrict_public_buckets: Option<PrimField<bool>>,
}

impl DataS3AccessPointPublicAccessBlockConfigurationEl {
    #[doc = "Set the field `block_public_acls`.\n"]
    pub fn set_block_public_acls(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.block_public_acls = Some(v.into());
        self
    }

    #[doc = "Set the field `block_public_policy`.\n"]
    pub fn set_block_public_policy(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.block_public_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_public_acls`.\n"]
    pub fn set_ignore_public_acls(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_public_acls = Some(v.into());
        self
    }

    #[doc = "Set the field `restrict_public_buckets`.\n"]
    pub fn set_restrict_public_buckets(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.restrict_public_buckets = Some(v.into());
        self
    }
}

impl ToListMappable for DataS3AccessPointPublicAccessBlockConfigurationEl {
    type O = BlockAssignable<DataS3AccessPointPublicAccessBlockConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3AccessPointPublicAccessBlockConfigurationEl {}

impl BuildDataS3AccessPointPublicAccessBlockConfigurationEl {
    pub fn build(self) -> DataS3AccessPointPublicAccessBlockConfigurationEl {
        DataS3AccessPointPublicAccessBlockConfigurationEl {
            block_public_acls: core::default::Default::default(),
            block_public_policy: core::default::Default::default(),
            ignore_public_acls: core::default::Default::default(),
            restrict_public_buckets: core::default::Default::default(),
        }
    }
}

pub struct DataS3AccessPointPublicAccessBlockConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataS3AccessPointPublicAccessBlockConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataS3AccessPointPublicAccessBlockConfigurationElRef {
        DataS3AccessPointPublicAccessBlockConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3AccessPointPublicAccessBlockConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `block_public_acls` after provisioning.\n"]
    pub fn block_public_acls(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.block_public_acls", self.base))
    }

    #[doc = "Get a reference to the value of field `block_public_policy` after provisioning.\n"]
    pub fn block_public_policy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.block_public_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_public_acls` after provisioning.\n"]
    pub fn ignore_public_acls(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_public_acls", self.base))
    }

    #[doc = "Get a reference to the value of field `restrict_public_buckets` after provisioning.\n"]
    pub fn restrict_public_buckets(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.restrict_public_buckets", self.base))
    }
}

#[derive(Serialize)]
pub struct DataS3AccessPointVpcConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
}

impl DataS3AccessPointVpcConfigurationEl {
    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataS3AccessPointVpcConfigurationEl {
    type O = BlockAssignable<DataS3AccessPointVpcConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3AccessPointVpcConfigurationEl {}

impl BuildDataS3AccessPointVpcConfigurationEl {
    pub fn build(self) -> DataS3AccessPointVpcConfigurationEl {
        DataS3AccessPointVpcConfigurationEl { vpc_id: core::default::Default::default() }
    }
}

pub struct DataS3AccessPointVpcConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataS3AccessPointVpcConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataS3AccessPointVpcConfigurationElRef {
        DataS3AccessPointVpcConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3AccessPointVpcConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}
