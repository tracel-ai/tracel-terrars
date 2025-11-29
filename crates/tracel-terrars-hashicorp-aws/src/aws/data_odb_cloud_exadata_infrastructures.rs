use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOdbCloudExadataInfrastructuresData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataOdbCloudExadataInfrastructures_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOdbCloudExadataInfrastructuresData>,
}

#[derive(Clone)]
pub struct DataOdbCloudExadataInfrastructures(Rc<DataOdbCloudExadataInfrastructures_>);

impl DataOdbCloudExadataInfrastructures {
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

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructures` after provisioning.\nList of Cloud Exadata Infrastructures. Returns basic information about the Cloud Exadata Infrastructures."]
    pub fn cloud_exadata_infrastructures(
        &self,
    ) -> ListRef<DataOdbCloudExadataInfrastructuresCloudExadataInfrastructuresElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloud_exadata_infrastructures", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for DataOdbCloudExadataInfrastructures {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataOdbCloudExadataInfrastructures { }

impl ToListMappable for DataOdbCloudExadataInfrastructures {
    type O = ListRef<DataOdbCloudExadataInfrastructuresRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOdbCloudExadataInfrastructures_ {
    fn extract_datasource_type(&self) -> String {
        "aws_odb_cloud_exadata_infrastructures".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOdbCloudExadataInfrastructures {
    pub tf_id: String,
}

impl BuildDataOdbCloudExadataInfrastructures {
    pub fn build(self, stack: &mut Stack) -> DataOdbCloudExadataInfrastructures {
        let out = DataOdbCloudExadataInfrastructures(Rc::new(DataOdbCloudExadataInfrastructures_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOdbCloudExadataInfrastructuresData {
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

pub struct DataOdbCloudExadataInfrastructuresRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbCloudExadataInfrastructuresRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataOdbCloudExadataInfrastructuresRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructures` after provisioning.\nList of Cloud Exadata Infrastructures. Returns basic information about the Cloud Exadata Infrastructures."]
    pub fn cloud_exadata_infrastructures(
        &self,
    ) -> ListRef<DataOdbCloudExadataInfrastructuresCloudExadataInfrastructuresElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloud_exadata_infrastructures", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOdbCloudExadataInfrastructuresCloudExadataInfrastructuresEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oci_resource_anchor_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oci_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ocid: Option<PrimField<String>>,
}

impl DataOdbCloudExadataInfrastructuresCloudExadataInfrastructuresEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `oci_resource_anchor_name`.\n"]
    pub fn set_oci_resource_anchor_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oci_resource_anchor_name = Some(v.into());
        self
    }

    #[doc = "Set the field `oci_url`.\n"]
    pub fn set_oci_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oci_url = Some(v.into());
        self
    }

    #[doc = "Set the field `ocid`.\n"]
    pub fn set_ocid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ocid = Some(v.into());
        self
    }
}

impl ToListMappable for DataOdbCloudExadataInfrastructuresCloudExadataInfrastructuresEl {
    type O = BlockAssignable<DataOdbCloudExadataInfrastructuresCloudExadataInfrastructuresEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbCloudExadataInfrastructuresCloudExadataInfrastructuresEl {}

impl BuildDataOdbCloudExadataInfrastructuresCloudExadataInfrastructuresEl {
    pub fn build(self) -> DataOdbCloudExadataInfrastructuresCloudExadataInfrastructuresEl {
        DataOdbCloudExadataInfrastructuresCloudExadataInfrastructuresEl {
            arn: core::default::Default::default(),
            display_name: core::default::Default::default(),
            id: core::default::Default::default(),
            oci_resource_anchor_name: core::default::Default::default(),
            oci_url: core::default::Default::default(),
            ocid: core::default::Default::default(),
        }
    }
}

pub struct DataOdbCloudExadataInfrastructuresCloudExadataInfrastructuresElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbCloudExadataInfrastructuresCloudExadataInfrastructuresElRef {
    fn new(shared: StackShared, base: String) -> DataOdbCloudExadataInfrastructuresCloudExadataInfrastructuresElRef {
        DataOdbCloudExadataInfrastructuresCloudExadataInfrastructuresElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbCloudExadataInfrastructuresCloudExadataInfrastructuresElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `oci_resource_anchor_name` after provisioning.\n"]
    pub fn oci_resource_anchor_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_resource_anchor_name", self.base))
    }

    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\n"]
    pub fn oci_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.base))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\n"]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }
}
