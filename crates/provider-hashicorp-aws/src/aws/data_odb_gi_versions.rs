use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataOdbGiVersionsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shape: Option<PrimField<String>>,
}
struct DataOdbGiVersions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOdbGiVersionsData>,
}
#[derive(Clone)]
pub struct DataOdbGiVersions(Rc<DataOdbGiVersions_>);
impl DataOdbGiVersions {
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
    #[doc = "Set the field `shape`.\nThe system shape."]
    pub fn set_shape(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().shape = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `gi_versions` after provisioning.\nInformation about a specific version of Oracle Grid Infrastructure (GI) software that can be installed on a VM cluster."]
    pub fn gi_versions(&self) -> ListRef<DataOdbGiVersionsGiVersionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gi_versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shape` after provisioning.\nThe system shape."]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shape", self.extract_ref()),
        )
    }
}
impl Referable for DataOdbGiVersions {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataOdbGiVersions {}
impl ToListMappable for DataOdbGiVersions {
    type O = ListRef<DataOdbGiVersionsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataOdbGiVersions_ {
    fn extract_datasource_type(&self) -> String {
        "aws_odb_gi_versions".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataOdbGiVersions {
    pub tf_id: String,
}
impl BuildDataOdbGiVersions {
    pub fn build(self, stack: &mut Stack) -> DataOdbGiVersions {
        let out = DataOdbGiVersions(Rc::new(DataOdbGiVersions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOdbGiVersionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
                shape: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataOdbGiVersionsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOdbGiVersionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataOdbGiVersionsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `gi_versions` after provisioning.\nInformation about a specific version of Oracle Grid Infrastructure (GI) software that can be installed on a VM cluster."]
    pub fn gi_versions(&self) -> ListRef<DataOdbGiVersionsGiVersionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gi_versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shape` after provisioning.\nThe system shape."]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shape", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataOdbGiVersionsGiVersionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataOdbGiVersionsGiVersionsEl {
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for DataOdbGiVersionsGiVersionsEl {
    type O = BlockAssignable<DataOdbGiVersionsGiVersionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOdbGiVersionsGiVersionsEl {}
impl BuildDataOdbGiVersionsGiVersionsEl {
    pub fn build(self) -> DataOdbGiVersionsGiVersionsEl {
        DataOdbGiVersionsGiVersionsEl {
            version: core::default::Default::default(),
        }
    }
}
pub struct DataOdbGiVersionsGiVersionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOdbGiVersionsGiVersionsElRef {
    fn new(shared: StackShared, base: String) -> DataOdbGiVersionsGiVersionsElRef {
        DataOdbGiVersionsGiVersionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOdbGiVersionsGiVersionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
