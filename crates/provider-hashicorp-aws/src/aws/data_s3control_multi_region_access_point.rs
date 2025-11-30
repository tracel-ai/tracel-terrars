use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataS3controlMultiRegionAccessPointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataS3controlMultiRegionAccessPoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataS3controlMultiRegionAccessPointData>,
}

#[derive(Clone)]
pub struct DataS3controlMultiRegionAccessPoint(Rc<DataS3controlMultiRegionAccessPoint_>);

impl DataS3controlMultiRegionAccessPoint {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `public_access_block` after provisioning.\n"]
    pub fn public_access_block(
        &self,
    ) -> ListRef<DataS3controlMultiRegionAccessPointPublicAccessBlockElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_access_block", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> ListRef<DataS3controlMultiRegionAccessPointRegionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.regions", self.extract_ref()),
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

impl Referable for DataS3controlMultiRegionAccessPoint {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataS3controlMultiRegionAccessPoint {}

impl ToListMappable for DataS3controlMultiRegionAccessPoint {
    type O = ListRef<DataS3controlMultiRegionAccessPointRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataS3controlMultiRegionAccessPoint_ {
    fn extract_datasource_type(&self) -> String {
        "aws_s3control_multi_region_access_point".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataS3controlMultiRegionAccessPoint {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataS3controlMultiRegionAccessPoint {
    pub fn build(self, stack: &mut Stack) -> DataS3controlMultiRegionAccessPoint {
        let out =
            DataS3controlMultiRegionAccessPoint(Rc::new(DataS3controlMultiRegionAccessPoint_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataS3controlMultiRegionAccessPointData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    account_id: core::default::Default::default(),
                    id: core::default::Default::default(),
                    name: self.name,
                    region: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataS3controlMultiRegionAccessPointRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataS3controlMultiRegionAccessPointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataS3controlMultiRegionAccessPointRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.alias", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `public_access_block` after provisioning.\n"]
    pub fn public_access_block(
        &self,
    ) -> ListRef<DataS3controlMultiRegionAccessPointPublicAccessBlockElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_access_block", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> ListRef<DataS3controlMultiRegionAccessPointRegionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.regions", self.extract_ref()),
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
pub struct DataS3controlMultiRegionAccessPointPublicAccessBlockEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    block_public_acls: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_public_policy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_public_acls: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrict_public_buckets: Option<PrimField<bool>>,
}

impl DataS3controlMultiRegionAccessPointPublicAccessBlockEl {
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

impl ToListMappable for DataS3controlMultiRegionAccessPointPublicAccessBlockEl {
    type O = BlockAssignable<DataS3controlMultiRegionAccessPointPublicAccessBlockEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3controlMultiRegionAccessPointPublicAccessBlockEl {}

impl BuildDataS3controlMultiRegionAccessPointPublicAccessBlockEl {
    pub fn build(self) -> DataS3controlMultiRegionAccessPointPublicAccessBlockEl {
        DataS3controlMultiRegionAccessPointPublicAccessBlockEl {
            block_public_acls: core::default::Default::default(),
            block_public_policy: core::default::Default::default(),
            ignore_public_acls: core::default::Default::default(),
            restrict_public_buckets: core::default::Default::default(),
        }
    }
}

pub struct DataS3controlMultiRegionAccessPointPublicAccessBlockElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataS3controlMultiRegionAccessPointPublicAccessBlockElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataS3controlMultiRegionAccessPointPublicAccessBlockElRef {
        DataS3controlMultiRegionAccessPointPublicAccessBlockElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3controlMultiRegionAccessPointPublicAccessBlockElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `block_public_acls` after provisioning.\n"]
    pub fn block_public_acls(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.block_public_acls", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `block_public_policy` after provisioning.\n"]
    pub fn block_public_policy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.block_public_policy", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `ignore_public_acls` after provisioning.\n"]
    pub fn ignore_public_acls(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_public_acls", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `restrict_public_buckets` after provisioning.\n"]
    pub fn restrict_public_buckets(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.restrict_public_buckets", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataS3controlMultiRegionAccessPointRegionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

impl DataS3controlMultiRegionAccessPointRegionsEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_account_id`.\n"]
    pub fn set_bucket_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_account_id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}

impl ToListMappable for DataS3controlMultiRegionAccessPointRegionsEl {
    type O = BlockAssignable<DataS3controlMultiRegionAccessPointRegionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3controlMultiRegionAccessPointRegionsEl {}

impl BuildDataS3controlMultiRegionAccessPointRegionsEl {
    pub fn build(self) -> DataS3controlMultiRegionAccessPointRegionsEl {
        DataS3controlMultiRegionAccessPointRegionsEl {
            bucket: core::default::Default::default(),
            bucket_account_id: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

pub struct DataS3controlMultiRegionAccessPointRegionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataS3controlMultiRegionAccessPointRegionsElRef {
    fn new(shared: StackShared, base: String) -> DataS3controlMultiRegionAccessPointRegionsElRef {
        DataS3controlMultiRegionAccessPointRegionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3controlMultiRegionAccessPointRegionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_account_id` after provisioning.\n"]
    pub fn bucket_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_account_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}
