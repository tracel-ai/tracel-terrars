use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataOdbCloudAutonomousVmClustersData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataOdbCloudAutonomousVmClusters_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOdbCloudAutonomousVmClustersData>,
}

#[derive(Clone)]
pub struct DataOdbCloudAutonomousVmClusters(Rc<DataOdbCloudAutonomousVmClusters_>);

impl DataOdbCloudAutonomousVmClusters {
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

    #[doc = "Get a reference to the value of field `cloud_autonomous_vm_clusters` after provisioning.\nList of Cloud Autonomous VM Clusters. The list going to contain basic information about the cloud autonomous VM clusters."]
    pub fn cloud_autonomous_vm_clusters(
        &self,
    ) -> ListRef<DataOdbCloudAutonomousVmClustersCloudAutonomousVmClustersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_autonomous_vm_clusters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}

impl Referable for DataOdbCloudAutonomousVmClusters {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataOdbCloudAutonomousVmClusters {}

impl ToListMappable for DataOdbCloudAutonomousVmClusters {
    type O = ListRef<DataOdbCloudAutonomousVmClustersRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOdbCloudAutonomousVmClusters_ {
    fn extract_datasource_type(&self) -> String {
        "aws_odb_cloud_autonomous_vm_clusters".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOdbCloudAutonomousVmClusters {
    pub tf_id: String,
}

impl BuildDataOdbCloudAutonomousVmClusters {
    pub fn build(self, stack: &mut Stack) -> DataOdbCloudAutonomousVmClusters {
        let out = DataOdbCloudAutonomousVmClusters(Rc::new(DataOdbCloudAutonomousVmClusters_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOdbCloudAutonomousVmClustersData {
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

pub struct DataOdbCloudAutonomousVmClustersRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbCloudAutonomousVmClustersRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataOdbCloudAutonomousVmClustersRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `cloud_autonomous_vm_clusters` after provisioning.\nList of Cloud Autonomous VM Clusters. The list going to contain basic information about the cloud autonomous VM clusters."]
    pub fn cloud_autonomous_vm_clusters(
        &self,
    ) -> ListRef<DataOdbCloudAutonomousVmClustersCloudAutonomousVmClustersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_autonomous_vm_clusters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataOdbCloudAutonomousVmClustersCloudAutonomousVmClustersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_exadata_infrastructure_id: Option<PrimField<String>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    odb_network_id: Option<PrimField<String>>,
}

impl DataOdbCloudAutonomousVmClustersCloudAutonomousVmClustersEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_exadata_infrastructure_id`.\n"]
    pub fn set_cloud_exadata_infrastructure_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloud_exadata_infrastructure_id = Some(v.into());
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

    #[doc = "Set the field `odb_network_id`.\n"]
    pub fn set_odb_network_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.odb_network_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataOdbCloudAutonomousVmClustersCloudAutonomousVmClustersEl {
    type O = BlockAssignable<DataOdbCloudAutonomousVmClustersCloudAutonomousVmClustersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbCloudAutonomousVmClustersCloudAutonomousVmClustersEl {}

impl BuildDataOdbCloudAutonomousVmClustersCloudAutonomousVmClustersEl {
    pub fn build(self) -> DataOdbCloudAutonomousVmClustersCloudAutonomousVmClustersEl {
        DataOdbCloudAutonomousVmClustersCloudAutonomousVmClustersEl {
            arn: core::default::Default::default(),
            cloud_exadata_infrastructure_id: core::default::Default::default(),
            display_name: core::default::Default::default(),
            id: core::default::Default::default(),
            oci_resource_anchor_name: core::default::Default::default(),
            oci_url: core::default::Default::default(),
            ocid: core::default::Default::default(),
            odb_network_id: core::default::Default::default(),
        }
    }
}

pub struct DataOdbCloudAutonomousVmClustersCloudAutonomousVmClustersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOdbCloudAutonomousVmClustersCloudAutonomousVmClustersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOdbCloudAutonomousVmClustersCloudAutonomousVmClustersElRef {
        DataOdbCloudAutonomousVmClustersCloudAutonomousVmClustersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbCloudAutonomousVmClustersCloudAutonomousVmClustersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\n"]
    pub fn cloud_exadata_infrastructure_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.base),
        )
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_resource_anchor_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\n"]
    pub fn oci_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.base))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\n"]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }

    #[doc = "Get a reference to the value of field `odb_network_id` after provisioning.\n"]
    pub fn odb_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.odb_network_id", self.base),
        )
    }
}
