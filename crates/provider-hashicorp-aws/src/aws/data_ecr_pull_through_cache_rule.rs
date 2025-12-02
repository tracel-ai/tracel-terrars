use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataEcrPullThroughCacheRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    ecr_repository_prefix: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataEcrPullThroughCacheRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEcrPullThroughCacheRuleData>,
}
#[derive(Clone)]
pub struct DataEcrPullThroughCacheRule(Rc<DataEcrPullThroughCacheRule_>);
impl DataEcrPullThroughCacheRule {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `credential_arn` after provisioning.\n"]
    pub fn credential_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credential_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_role_arn` after provisioning.\n"]
    pub fn custom_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ecr_repository_prefix` after provisioning.\n"]
    pub fn ecr_repository_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ecr_repository_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registry_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upstream_registry_url` after provisioning.\n"]
    pub fn upstream_registry_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.upstream_registry_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upstream_repository_prefix` after provisioning.\n"]
    pub fn upstream_repository_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.upstream_repository_prefix", self.extract_ref()),
        )
    }
}
impl Referable for DataEcrPullThroughCacheRule {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataEcrPullThroughCacheRule {}
impl ToListMappable for DataEcrPullThroughCacheRule {
    type O = ListRef<DataEcrPullThroughCacheRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataEcrPullThroughCacheRule_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ecr_pull_through_cache_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataEcrPullThroughCacheRule {
    pub tf_id: String,
    #[doc = ""]
    pub ecr_repository_prefix: PrimField<String>,
}
impl BuildDataEcrPullThroughCacheRule {
    pub fn build(self, stack: &mut Stack) -> DataEcrPullThroughCacheRule {
        let out = DataEcrPullThroughCacheRule(Rc::new(DataEcrPullThroughCacheRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEcrPullThroughCacheRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                ecr_repository_prefix: self.ecr_repository_prefix,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataEcrPullThroughCacheRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEcrPullThroughCacheRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataEcrPullThroughCacheRuleRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `credential_arn` after provisioning.\n"]
    pub fn credential_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credential_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_role_arn` after provisioning.\n"]
    pub fn custom_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ecr_repository_prefix` after provisioning.\n"]
    pub fn ecr_repository_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ecr_repository_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registry_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upstream_registry_url` after provisioning.\n"]
    pub fn upstream_registry_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.upstream_registry_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upstream_repository_prefix` after provisioning.\n"]
    pub fn upstream_repository_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.upstream_repository_prefix", self.extract_ref()),
        )
    }
}
