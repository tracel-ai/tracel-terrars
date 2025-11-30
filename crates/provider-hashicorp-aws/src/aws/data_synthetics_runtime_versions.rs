use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataSyntheticsRuntimeVersionsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataSyntheticsRuntimeVersions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSyntheticsRuntimeVersionsData>,
}

#[derive(Clone)]
pub struct DataSyntheticsRuntimeVersions(Rc<DataSyntheticsRuntimeVersions_>);

impl DataSyntheticsRuntimeVersions {
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

    #[doc = "Get a reference to the value of field `runtime_versions` after provisioning.\n"]
    pub fn runtime_versions(&self) -> ListRef<DataSyntheticsRuntimeVersionsRuntimeVersionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime_versions", self.extract_ref()),
        )
    }
}

impl Referable for DataSyntheticsRuntimeVersions {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataSyntheticsRuntimeVersions {}

impl ToListMappable for DataSyntheticsRuntimeVersions {
    type O = ListRef<DataSyntheticsRuntimeVersionsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSyntheticsRuntimeVersions_ {
    fn extract_datasource_type(&self) -> String {
        "aws_synthetics_runtime_versions".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSyntheticsRuntimeVersions {
    pub tf_id: String,
}

impl BuildDataSyntheticsRuntimeVersions {
    pub fn build(self, stack: &mut Stack) -> DataSyntheticsRuntimeVersions {
        let out = DataSyntheticsRuntimeVersions(Rc::new(DataSyntheticsRuntimeVersions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSyntheticsRuntimeVersionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSyntheticsRuntimeVersionsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSyntheticsRuntimeVersionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataSyntheticsRuntimeVersionsRef {
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

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `runtime_versions` after provisioning.\n"]
    pub fn runtime_versions(&self) -> ListRef<DataSyntheticsRuntimeVersionsRuntimeVersionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime_versions", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataSyntheticsRuntimeVersionsRuntimeVersionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deprecation_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    release_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_name: Option<PrimField<String>>,
}

impl DataSyntheticsRuntimeVersionsRuntimeVersionsEl {
    #[doc = "Set the field `deprecation_date`.\n"]
    pub fn set_deprecation_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deprecation_date = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `release_date`.\n"]
    pub fn set_release_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.release_date = Some(v.into());
        self
    }

    #[doc = "Set the field `version_name`.\n"]
    pub fn set_version_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataSyntheticsRuntimeVersionsRuntimeVersionsEl {
    type O = BlockAssignable<DataSyntheticsRuntimeVersionsRuntimeVersionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSyntheticsRuntimeVersionsRuntimeVersionsEl {}

impl BuildDataSyntheticsRuntimeVersionsRuntimeVersionsEl {
    pub fn build(self) -> DataSyntheticsRuntimeVersionsRuntimeVersionsEl {
        DataSyntheticsRuntimeVersionsRuntimeVersionsEl {
            deprecation_date: core::default::Default::default(),
            description: core::default::Default::default(),
            release_date: core::default::Default::default(),
            version_name: core::default::Default::default(),
        }
    }
}

pub struct DataSyntheticsRuntimeVersionsRuntimeVersionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSyntheticsRuntimeVersionsRuntimeVersionsElRef {
    fn new(shared: StackShared, base: String) -> DataSyntheticsRuntimeVersionsRuntimeVersionsElRef {
        DataSyntheticsRuntimeVersionsRuntimeVersionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSyntheticsRuntimeVersionsRuntimeVersionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deprecation_date` after provisioning.\n"]
    pub fn deprecation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deprecation_date", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `release_date` after provisioning.\n"]
    pub fn release_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_date", self.base))
    }

    #[doc = "Get a reference to the value of field `version_name` after provisioning.\n"]
    pub fn version_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_name", self.base))
    }
}
