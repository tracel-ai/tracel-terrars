use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudhsmV2ClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataCloudhsmV2Cluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudhsmV2ClusterData>,
}

#[derive(Clone)]
pub struct DataCloudhsmV2Cluster(Rc<DataCloudhsmV2Cluster_>);

impl DataCloudhsmV2Cluster {
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

    #[doc = "Set the field `cluster_state`.\n"]
    pub fn set_cluster_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cluster_state = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cluster_certificates` after provisioning.\n"]
    pub fn cluster_certificates(&self) -> ListRef<DataCloudhsmV2ClusterClusterCertificatesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cluster_certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_state` after provisioning.\n"]
    pub fn cluster_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }
}

impl Referable for DataCloudhsmV2Cluster {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataCloudhsmV2Cluster { }

impl ToListMappable for DataCloudhsmV2Cluster {
    type O = ListRef<DataCloudhsmV2ClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCloudhsmV2Cluster_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cloudhsm_v2_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudhsmV2Cluster {
    pub tf_id: String,
    #[doc = ""]
    pub cluster_id: PrimField<String>,
}

impl BuildDataCloudhsmV2Cluster {
    pub fn build(self, stack: &mut Stack) -> DataCloudhsmV2Cluster {
        let out = DataCloudhsmV2Cluster(Rc::new(DataCloudhsmV2Cluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudhsmV2ClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cluster_id: self.cluster_id,
                cluster_state: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudhsmV2ClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudhsmV2ClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataCloudhsmV2ClusterRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `cluster_certificates` after provisioning.\n"]
    pub fn cluster_certificates(&self) -> ListRef<DataCloudhsmV2ClusterClusterCertificatesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cluster_certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_state` after provisioning.\n"]
    pub fn cluster_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudhsmV2ClusterClusterCertificatesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_hardware_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_csr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hsm_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturer_hardware_certificate: Option<PrimField<String>>,
}

impl DataCloudhsmV2ClusterClusterCertificatesEl {
    #[doc = "Set the field `aws_hardware_certificate`.\n"]
    pub fn set_aws_hardware_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.aws_hardware_certificate = Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_certificate`.\n"]
    pub fn set_cluster_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_certificate = Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_csr`.\n"]
    pub fn set_cluster_csr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_csr = Some(v.into());
        self
    }

    #[doc = "Set the field `hsm_certificate`.\n"]
    pub fn set_hsm_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.hsm_certificate = Some(v.into());
        self
    }

    #[doc = "Set the field `manufacturer_hardware_certificate`.\n"]
    pub fn set_manufacturer_hardware_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.manufacturer_hardware_certificate = Some(v.into());
        self
    }
}

impl ToListMappable for DataCloudhsmV2ClusterClusterCertificatesEl {
    type O = BlockAssignable<DataCloudhsmV2ClusterClusterCertificatesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudhsmV2ClusterClusterCertificatesEl {}

impl BuildDataCloudhsmV2ClusterClusterCertificatesEl {
    pub fn build(self) -> DataCloudhsmV2ClusterClusterCertificatesEl {
        DataCloudhsmV2ClusterClusterCertificatesEl {
            aws_hardware_certificate: core::default::Default::default(),
            cluster_certificate: core::default::Default::default(),
            cluster_csr: core::default::Default::default(),
            hsm_certificate: core::default::Default::default(),
            manufacturer_hardware_certificate: core::default::Default::default(),
        }
    }
}

pub struct DataCloudhsmV2ClusterClusterCertificatesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudhsmV2ClusterClusterCertificatesElRef {
    fn new(shared: StackShared, base: String) -> DataCloudhsmV2ClusterClusterCertificatesElRef {
        DataCloudhsmV2ClusterClusterCertificatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudhsmV2ClusterClusterCertificatesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_hardware_certificate` after provisioning.\n"]
    pub fn aws_hardware_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_hardware_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_certificate` after provisioning.\n"]
    pub fn cluster_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_csr` after provisioning.\n"]
    pub fn cluster_csr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_csr", self.base))
    }

    #[doc = "Get a reference to the value of field `hsm_certificate` after provisioning.\n"]
    pub fn hsm_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hsm_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `manufacturer_hardware_certificate` after provisioning.\n"]
    pub fn manufacturer_hardware_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.manufacturer_hardware_certificate", self.base))
    }
}
