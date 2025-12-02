use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudhsmV2ClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    hsm_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_backup_identifier: Option<PrimField<String>>,
    subnet_ids: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudhsmV2ClusterTimeoutsEl>,
}
struct CloudhsmV2Cluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudhsmV2ClusterData>,
}
#[derive(Clone)]
pub struct CloudhsmV2Cluster(Rc<CloudhsmV2Cluster_>);
impl CloudhsmV2Cluster {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }
    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }
    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }
    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => true,
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    }
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }
    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(r.extract_ref());
        self
    }
    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(attr.to_string());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mode = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `source_backup_identifier`.\n"]
    pub fn set_source_backup_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_backup_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudhsmV2ClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `cluster_certificates` after provisioning.\n"]
    pub fn cluster_certificates(&self) -> ListRef<CloudhsmV2ClusterClusterCertificatesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_certificates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_state` after provisioning.\n"]
    pub fn cluster_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hsm_type` after provisioning.\n"]
    pub fn hsm_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hsm_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_backup_identifier` after provisioning.\n"]
    pub fn source_backup_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_backup_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudhsmV2ClusterTimeoutsElRef {
        CloudhsmV2ClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CloudhsmV2Cluster {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudhsmV2Cluster {}
impl ToListMappable for CloudhsmV2Cluster {
    type O = ListRef<CloudhsmV2ClusterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudhsmV2Cluster_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudhsm_v2_cluster".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudhsmV2Cluster {
    pub tf_id: String,
    #[doc = ""]
    pub hsm_type: PrimField<String>,
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
}
impl BuildCloudhsmV2Cluster {
    pub fn build(self, stack: &mut Stack) -> CloudhsmV2Cluster {
        let out = CloudhsmV2Cluster(Rc::new(CloudhsmV2Cluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudhsmV2ClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                hsm_type: self.hsm_type,
                id: core::default::Default::default(),
                mode: core::default::Default::default(),
                region: core::default::Default::default(),
                source_backup_identifier: core::default::Default::default(),
                subnet_ids: self.subnet_ids,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudhsmV2ClusterRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudhsmV2ClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudhsmV2ClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cluster_certificates` after provisioning.\n"]
    pub fn cluster_certificates(&self) -> ListRef<CloudhsmV2ClusterClusterCertificatesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_certificates", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_state` after provisioning.\n"]
    pub fn cluster_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hsm_type` after provisioning.\n"]
    pub fn hsm_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hsm_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_group_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_backup_identifier` after provisioning.\n"]
    pub fn source_backup_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_backup_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudhsmV2ClusterTimeoutsElRef {
        CloudhsmV2ClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudhsmV2ClusterClusterCertificatesEl {
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
impl CloudhsmV2ClusterClusterCertificatesEl {
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
    pub fn set_manufacturer_hardware_certificate(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.manufacturer_hardware_certificate = Some(v.into());
        self
    }
}
impl ToListMappable for CloudhsmV2ClusterClusterCertificatesEl {
    type O = BlockAssignable<CloudhsmV2ClusterClusterCertificatesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudhsmV2ClusterClusterCertificatesEl {}
impl BuildCloudhsmV2ClusterClusterCertificatesEl {
    pub fn build(self) -> CloudhsmV2ClusterClusterCertificatesEl {
        CloudhsmV2ClusterClusterCertificatesEl {
            aws_hardware_certificate: core::default::Default::default(),
            cluster_certificate: core::default::Default::default(),
            cluster_csr: core::default::Default::default(),
            hsm_certificate: core::default::Default::default(),
            manufacturer_hardware_certificate: core::default::Default::default(),
        }
    }
}
pub struct CloudhsmV2ClusterClusterCertificatesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudhsmV2ClusterClusterCertificatesElRef {
    fn new(shared: StackShared, base: String) -> CloudhsmV2ClusterClusterCertificatesElRef {
        CloudhsmV2ClusterClusterCertificatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudhsmV2ClusterClusterCertificatesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aws_hardware_certificate` after provisioning.\n"]
    pub fn aws_hardware_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_hardware_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_certificate` after provisioning.\n"]
    pub fn cluster_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_csr` after provisioning.\n"]
    pub fn cluster_csr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_csr", self.base))
    }
    #[doc = "Get a reference to the value of field `hsm_certificate` after provisioning.\n"]
    pub fn hsm_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hsm_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `manufacturer_hardware_certificate` after provisioning.\n"]
    pub fn manufacturer_hardware_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manufacturer_hardware_certificate", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudhsmV2ClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudhsmV2ClusterTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for CloudhsmV2ClusterTimeoutsEl {
    type O = BlockAssignable<CloudhsmV2ClusterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudhsmV2ClusterTimeoutsEl {}
impl BuildCloudhsmV2ClusterTimeoutsEl {
    pub fn build(self) -> CloudhsmV2ClusterTimeoutsEl {
        CloudhsmV2ClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudhsmV2ClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudhsmV2ClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudhsmV2ClusterTimeoutsElRef {
        CloudhsmV2ClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudhsmV2ClusterTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
