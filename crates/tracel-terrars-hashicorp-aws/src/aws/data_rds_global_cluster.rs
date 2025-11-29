use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataRdsGlobalClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataRdsGlobalCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRdsGlobalClusterData>,
}

#[derive(Clone)]
pub struct DataRdsGlobalCluster(Rc<DataRdsGlobalCluster_>);

impl DataRdsGlobalCluster {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_lifecycle_support` after provisioning.\n"]
    pub fn engine_lifecycle_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_lifecycle_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `members` after provisioning.\n"]
    pub fn members(&self) -> ListRef<DataRdsGlobalClusterMembersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.members", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `storage_encrypted` after provisioning.\n"]
    pub fn storage_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Referable for DataRdsGlobalCluster {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRdsGlobalCluster { }

impl ToListMappable for DataRdsGlobalCluster {
    type O = ListRef<DataRdsGlobalClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRdsGlobalCluster_ {
    fn extract_datasource_type(&self) -> String {
        "aws_rds_global_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRdsGlobalCluster {
    pub tf_id: String,
    #[doc = ""]
    pub identifier: PrimField<String>,
}

impl BuildDataRdsGlobalCluster {
    pub fn build(self, stack: &mut Stack) -> DataRdsGlobalCluster {
        let out = DataRdsGlobalCluster(Rc::new(DataRdsGlobalCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRdsGlobalClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                identifier: self.identifier,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRdsGlobalClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRdsGlobalClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataRdsGlobalClusterRef {
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

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_lifecycle_support` after provisioning.\n"]
    pub fn engine_lifecycle_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_lifecycle_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `members` after provisioning.\n"]
    pub fn members(&self) -> ListRef<DataRdsGlobalClusterMembersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.members", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `storage_encrypted` after provisioning.\n"]
    pub fn storage_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRdsGlobalClusterMembersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    db_cluster_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_writer: Option<PrimField<bool>>,
}

impl DataRdsGlobalClusterMembersEl {
    #[doc = "Set the field `db_cluster_arn`.\n"]
    pub fn set_db_cluster_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_cluster_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `is_writer`.\n"]
    pub fn set_is_writer(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_writer = Some(v.into());
        self
    }
}

impl ToListMappable for DataRdsGlobalClusterMembersEl {
    type O = BlockAssignable<DataRdsGlobalClusterMembersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRdsGlobalClusterMembersEl {}

impl BuildDataRdsGlobalClusterMembersEl {
    pub fn build(self) -> DataRdsGlobalClusterMembersEl {
        DataRdsGlobalClusterMembersEl {
            db_cluster_arn: core::default::Default::default(),
            is_writer: core::default::Default::default(),
        }
    }
}

pub struct DataRdsGlobalClusterMembersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRdsGlobalClusterMembersElRef {
    fn new(shared: StackShared, base: String) -> DataRdsGlobalClusterMembersElRef {
        DataRdsGlobalClusterMembersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRdsGlobalClusterMembersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `db_cluster_arn` after provisioning.\n"]
    pub fn db_cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_cluster_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `is_writer` after provisioning.\n"]
    pub fn is_writer(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_writer", self.base))
    }
}
