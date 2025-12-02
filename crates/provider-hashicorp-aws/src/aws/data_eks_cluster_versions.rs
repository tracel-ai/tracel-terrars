use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataEksClusterVersionsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_versions_only: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_all: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_status: Option<PrimField<String>>,
}
struct DataEksClusterVersions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEksClusterVersionsData>,
}
#[derive(Clone)]
pub struct DataEksClusterVersions(Rc<DataEksClusterVersions_>);
impl DataEksClusterVersions {
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
    #[doc = "Set the field `cluster_type`.\n"]
    pub fn set_cluster_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cluster_type = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_versions_only`.\n"]
    pub fn set_cluster_versions_only(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().cluster_versions_only = Some(v.into());
        self
    }
    #[doc = "Set the field `default_only`.\n"]
    pub fn set_default_only(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default_only = Some(v.into());
        self
    }
    #[doc = "Set the field `include_all`.\n"]
    pub fn set_include_all(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_all = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `version_status`.\n"]
    pub fn set_version_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version_status = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `cluster_type` after provisioning.\n"]
    pub fn cluster_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_versions` after provisioning.\n"]
    pub fn cluster_versions(&self) -> ListRef<DataEksClusterVersionsClusterVersionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_versions_only` after provisioning.\n"]
    pub fn cluster_versions_only(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_versions_only", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_only` after provisioning.\n"]
    pub fn default_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_only", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_all` after provisioning.\n"]
    pub fn include_all(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_status` after provisioning.\n"]
    pub fn version_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_status", self.extract_ref()),
        )
    }
}
impl Referable for DataEksClusterVersions {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataEksClusterVersions {}
impl ToListMappable for DataEksClusterVersions {
    type O = ListRef<DataEksClusterVersionsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataEksClusterVersions_ {
    fn extract_datasource_type(&self) -> String {
        "aws_eks_cluster_versions".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataEksClusterVersions {
    pub tf_id: String,
}
impl BuildDataEksClusterVersions {
    pub fn build(self, stack: &mut Stack) -> DataEksClusterVersions {
        let out = DataEksClusterVersions(Rc::new(DataEksClusterVersions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEksClusterVersionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cluster_type: core::default::Default::default(),
                cluster_versions_only: core::default::Default::default(),
                default_only: core::default::Default::default(),
                include_all: core::default::Default::default(),
                region: core::default::Default::default(),
                version_status: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataEksClusterVersionsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEksClusterVersionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataEksClusterVersionsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `cluster_type` after provisioning.\n"]
    pub fn cluster_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_versions` after provisioning.\n"]
    pub fn cluster_versions(&self) -> ListRef<DataEksClusterVersionsClusterVersionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_versions_only` after provisioning.\n"]
    pub fn cluster_versions_only(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_versions_only", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_only` after provisioning.\n"]
    pub fn default_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_only", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_all` after provisioning.\n"]
    pub fn include_all(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_status` after provisioning.\n"]
    pub fn version_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_status", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataEksClusterVersionsClusterVersionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_platform_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_version: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_of_extended_support_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_of_standard_support_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kubernetes_patch_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    release_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_status: Option<PrimField<String>>,
}
impl DataEksClusterVersionsClusterVersionsEl {
    #[doc = "Set the field `cluster_type`.\n"]
    pub fn set_cluster_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_type = Some(v.into());
        self
    }
    #[doc = "Set the field `cluster_version`.\n"]
    pub fn set_cluster_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_version = Some(v.into());
        self
    }
    #[doc = "Set the field `default_platform_version`.\n"]
    pub fn set_default_platform_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_platform_version = Some(v.into());
        self
    }
    #[doc = "Set the field `default_version`.\n"]
    pub fn set_default_version(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.default_version = Some(v.into());
        self
    }
    #[doc = "Set the field `end_of_extended_support_date`.\n"]
    pub fn set_end_of_extended_support_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_of_extended_support_date = Some(v.into());
        self
    }
    #[doc = "Set the field `end_of_standard_support_date`.\n"]
    pub fn set_end_of_standard_support_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_of_standard_support_date = Some(v.into());
        self
    }
    #[doc = "Set the field `kubernetes_patch_version`.\n"]
    pub fn set_kubernetes_patch_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kubernetes_patch_version = Some(v.into());
        self
    }
    #[doc = "Set the field `release_date`.\n"]
    pub fn set_release_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.release_date = Some(v.into());
        self
    }
    #[doc = "Set the field `version_status`.\n"]
    pub fn set_version_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version_status = Some(v.into());
        self
    }
}
impl ToListMappable for DataEksClusterVersionsClusterVersionsEl {
    type O = BlockAssignable<DataEksClusterVersionsClusterVersionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEksClusterVersionsClusterVersionsEl {}
impl BuildDataEksClusterVersionsClusterVersionsEl {
    pub fn build(self) -> DataEksClusterVersionsClusterVersionsEl {
        DataEksClusterVersionsClusterVersionsEl {
            cluster_type: core::default::Default::default(),
            cluster_version: core::default::Default::default(),
            default_platform_version: core::default::Default::default(),
            default_version: core::default::Default::default(),
            end_of_extended_support_date: core::default::Default::default(),
            end_of_standard_support_date: core::default::Default::default(),
            kubernetes_patch_version: core::default::Default::default(),
            release_date: core::default::Default::default(),
            version_status: core::default::Default::default(),
        }
    }
}
pub struct DataEksClusterVersionsClusterVersionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEksClusterVersionsClusterVersionsElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterVersionsClusterVersionsElRef {
        DataEksClusterVersionsClusterVersionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEksClusterVersionsClusterVersionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_type` after provisioning.\n"]
    pub fn cluster_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_type", self.base))
    }
    #[doc = "Get a reference to the value of field `cluster_version` after provisioning.\n"]
    pub fn cluster_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_platform_version` after provisioning.\n"]
    pub fn default_platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_platform_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_version` after provisioning.\n"]
    pub fn default_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `end_of_extended_support_date` after provisioning.\n"]
    pub fn end_of_extended_support_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_of_extended_support_date", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `end_of_standard_support_date` after provisioning.\n"]
    pub fn end_of_standard_support_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_of_standard_support_date", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kubernetes_patch_version` after provisioning.\n"]
    pub fn kubernetes_patch_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kubernetes_patch_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `release_date` after provisioning.\n"]
    pub fn release_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_date", self.base))
    }
    #[doc = "Get a reference to the value of field `version_status` after provisioning.\n"]
    pub fn version_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_status", self.base),
        )
    }
}
