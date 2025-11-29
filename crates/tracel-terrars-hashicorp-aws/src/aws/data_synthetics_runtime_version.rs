use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSyntheticsRuntimeVersionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latest: Option<PrimField<bool>>,
    prefix: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

struct DataSyntheticsRuntimeVersion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSyntheticsRuntimeVersionData>,
}

#[derive(Clone)]
pub struct DataSyntheticsRuntimeVersion(Rc<DataSyntheticsRuntimeVersion_>);

impl DataSyntheticsRuntimeVersion {
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

    #[doc = "Set the field `latest`.\n"]
    pub fn set_latest(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().latest = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `deprecation_date` after provisioning.\n"]
    pub fn deprecation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deprecation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `latest` after provisioning.\n"]
    pub fn latest(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.latest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_date` after provisioning.\n"]
    pub fn release_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_name` after provisioning.\n"]
    pub fn version_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_name", self.extract_ref()))
    }
}

impl Referable for DataSyntheticsRuntimeVersion {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataSyntheticsRuntimeVersion { }

impl ToListMappable for DataSyntheticsRuntimeVersion {
    type O = ListRef<DataSyntheticsRuntimeVersionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSyntheticsRuntimeVersion_ {
    fn extract_datasource_type(&self) -> String {
        "aws_synthetics_runtime_version".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSyntheticsRuntimeVersion {
    pub tf_id: String,
    #[doc = ""]
    pub prefix: PrimField<String>,
}

impl BuildDataSyntheticsRuntimeVersion {
    pub fn build(self, stack: &mut Stack) -> DataSyntheticsRuntimeVersion {
        let out = DataSyntheticsRuntimeVersion(Rc::new(DataSyntheticsRuntimeVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSyntheticsRuntimeVersionData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                latest: core::default::Default::default(),
                prefix: self.prefix,
                region: core::default::Default::default(),
                version: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSyntheticsRuntimeVersionRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSyntheticsRuntimeVersionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataSyntheticsRuntimeVersionRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `deprecation_date` after provisioning.\n"]
    pub fn deprecation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deprecation_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `latest` after provisioning.\n"]
    pub fn latest(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.latest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_date` after provisioning.\n"]
    pub fn release_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_name` after provisioning.\n"]
    pub fn version_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_name", self.extract_ref()))
    }
}
