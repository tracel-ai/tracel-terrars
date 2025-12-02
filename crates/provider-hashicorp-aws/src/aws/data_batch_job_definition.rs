use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBatchJobDefinitionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}
struct DataBatchJobDefinition_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBatchJobDefinitionData>,
}
#[derive(Clone)]
pub struct DataBatchJobDefinition(Rc<DataBatchJobDefinition_>);
impl DataBatchJobDefinition {
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
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().arn = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `revision`.\n"]
    pub fn set_revision(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().revision = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `arn_prefix` after provisioning.\n"]
    pub fn arn_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.arn_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `container_orchestration_type` after provisioning.\n"]
    pub fn container_orchestration_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_orchestration_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `eks_properties` after provisioning.\n"]
    pub fn eks_properties(&self) -> ListRef<DataBatchJobDefinitionEksPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eks_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_properties` after provisioning.\n"]
    pub fn node_properties(&self) -> ListRef<DataBatchJobDefinitionNodePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(&self) -> ListRef<DataBatchJobDefinitionRetryStrategyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scheduling_priority` after provisioning.\n"]
    pub fn scheduling_priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduling_priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<DataBatchJobDefinitionTimeoutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
impl Referable for DataBatchJobDefinition {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBatchJobDefinition {}
impl ToListMappable for DataBatchJobDefinition {
    type O = ListRef<DataBatchJobDefinitionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBatchJobDefinition_ {
    fn extract_datasource_type(&self) -> String {
        "aws_batch_job_definition".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBatchJobDefinition {
    pub tf_id: String,
}
impl BuildDataBatchJobDefinition {
    pub fn build(self, stack: &mut Stack) -> DataBatchJobDefinition {
        let out = DataBatchJobDefinition(Rc::new(DataBatchJobDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBatchJobDefinitionData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                arn: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                revision: core::default::Default::default(),
                status: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBatchJobDefinitionRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBatchJobDefinitionRef {
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
    #[doc = "Get a reference to the value of field `arn_prefix` after provisioning.\n"]
    pub fn arn_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.arn_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `container_orchestration_type` after provisioning.\n"]
    pub fn container_orchestration_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_orchestration_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `eks_properties` after provisioning.\n"]
    pub fn eks_properties(&self) -> ListRef<DataBatchJobDefinitionEksPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eks_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_properties` after provisioning.\n"]
    pub fn node_properties(&self) -> ListRef<DataBatchJobDefinitionNodePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(&self) -> ListRef<DataBatchJobDefinitionRetryStrategyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scheduling_priority` after provisioning.\n"]
    pub fn scheduling_priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduling_priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<DataBatchJobDefinitionTimeoutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl {
    type O = BlockAssignable<DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl {}
impl BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl {
    pub fn build(self) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvElRef {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    limits: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requests: Option<RecField<PrimField<String>>>,
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl {
    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.limits = Some(v.into());
        self
    }
    #[doc = "Set the field `requests`.\n"]
    pub fn set_requests(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.requests = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl
{
    type O = BlockAssignable<
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl {}
impl BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl {
    pub fn build(
        self,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl {
            limits: core::default::Default::default(),
            requests: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesElRef {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }
    #[doc = "Get a reference to the value of field `requests` after provisioning.\n"]
    pub fn requests(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.requests", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    privileged: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only_root_file_system: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_as_group: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_as_non_root: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_as_user: Option<PrimField<f64>>,
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl {
    #[doc = "Set the field `privileged`.\n"]
    pub fn set_privileged(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.privileged = Some(v.into());
        self
    }
    #[doc = "Set the field `read_only_root_file_system`.\n"]
    pub fn set_read_only_root_file_system(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only_root_file_system = Some(v.into());
        self
    }
    #[doc = "Set the field `run_as_group`.\n"]
    pub fn set_run_as_group(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.run_as_group = Some(v.into());
        self
    }
    #[doc = "Set the field `run_as_non_root`.\n"]
    pub fn set_run_as_non_root(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.run_as_non_root = Some(v.into());
        self
    }
    #[doc = "Set the field `run_as_user`.\n"]
    pub fn set_run_as_user(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.run_as_user = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl
{
    type O = BlockAssignable<
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl {
}
impl BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl {
    pub fn build(
        self,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl {
            privileged: core::default::Default::default(),
            read_only_root_file_system: core::default::Default::default(),
            run_as_group: core::default::Default::default(),
            run_as_non_root: core::default::Default::default(),
            run_as_user: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextElRef {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `privileged` after provisioning.\n"]
    pub fn privileged(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.privileged", self.base))
    }
    #[doc = "Get a reference to the value of field `read_only_root_file_system` after provisioning.\n"]
    pub fn read_only_root_file_system(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_only_root_file_system", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `run_as_group` after provisioning.\n"]
    pub fn run_as_group(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_as_group", self.base))
    }
    #[doc = "Get a reference to the value of field `run_as_non_root` after provisioning.\n"]
    pub fn run_as_non_root(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_as_non_root", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `run_as_user` after provisioning.\n"]
    pub fn run_as_user(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_as_user", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl {
    #[doc = "Set the field `mount_path`.\n"]
    pub fn set_mount_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mount_path = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `read_only`.\n"]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl
{
    type O = BlockAssignable<
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl {}
impl BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl {
    pub fn build(
        self,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl {
            mount_path: core::default::Default::default(),
            name: core::default::Default::default(),
            read_only: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsElRef {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_pull_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<
        ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_context: Option<
        ListField<
            DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_mounts: Option<
        ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl>,
    >,
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl {
    #[doc = "Set the field `args`.\n"]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }
    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl Into<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvEl>>,
    ) -> Self {
        self.env = Some(v.into());
        self
    }
    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image = Some(v.into());
        self
    }
    #[doc = "Set the field `image_pull_policy`.\n"]
    pub fn set_image_pull_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_pull_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl Into<
            ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesEl>,
        >,
    ) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `security_context`.\n"]
    pub fn set_security_context(
        mut self,
        v: impl Into<
            ListField<
                DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextEl,
            >,
        >,
    ) -> Self {
        self.security_context = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_mounts`.\n"]
    pub fn set_volume_mounts(
        mut self,
        v: impl Into<
            ListField<
                DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsEl,
            >,
        >,
    ) -> Self {
        self.volume_mounts = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl {
    type O = BlockAssignable<DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl {}
impl BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl {
    pub fn build(self) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            env: core::default::Default::default(),
            image: core::default::Default::default(),
            image_pull_policy: core::default::Default::default(),
            name: core::default::Default::default(),
            resources: core::default::Default::default(),
            security_context: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElRef {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(
        &self,
    ) -> ListRef<DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElEnvElRef> {
        ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `image_pull_policy` after provisioning.\n"]
    pub fn image_pull_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_pull_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(
        &self,
    ) -> ListRef<DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElResourcesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `security_context` after provisioning.\n"]
    pub fn security_context(
        &self,
    ) -> ListRef<DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElSecurityContextElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_context", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(
        &self,
    ) -> ListRef<DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElVolumeMountsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.volume_mounts", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretsEl {
    type O =
        BlockAssignable<DataBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretsEl {}
impl BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretsEl {
    pub fn build(self) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretsEl {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretsEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretsElRef {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl {
    type O =
        BlockAssignable<DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl {}
impl BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl {
    pub fn build(
        self,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvElRef {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    limits: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requests: Option<RecField<PrimField<String>>>,
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl {
    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.limits = Some(v.into());
        self
    }
    #[doc = "Set the field `requests`.\n"]
    pub fn set_requests(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.requests = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl
{
    type O = BlockAssignable<
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl {}
impl BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl {
    pub fn build(
        self,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl {
            limits: core::default::Default::default(),
            requests: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesElRef {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }
    #[doc = "Get a reference to the value of field `requests` after provisioning.\n"]
    pub fn requests(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.requests", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    privileged: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only_root_file_system: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_as_group: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_as_non_root: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_as_user: Option<PrimField<f64>>,
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl {
    #[doc = "Set the field `privileged`.\n"]
    pub fn set_privileged(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.privileged = Some(v.into());
        self
    }
    #[doc = "Set the field `read_only_root_file_system`.\n"]
    pub fn set_read_only_root_file_system(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only_root_file_system = Some(v.into());
        self
    }
    #[doc = "Set the field `run_as_group`.\n"]
    pub fn set_run_as_group(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.run_as_group = Some(v.into());
        self
    }
    #[doc = "Set the field `run_as_non_root`.\n"]
    pub fn set_run_as_non_root(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.run_as_non_root = Some(v.into());
        self
    }
    #[doc = "Set the field `run_as_user`.\n"]
    pub fn set_run_as_user(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.run_as_user = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl
{
    type O = BlockAssignable<
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl
{}
impl BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl {
    pub fn build(
        self,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl {
            privileged: core::default::Default::default(),
            read_only_root_file_system: core::default::Default::default(),
            run_as_group: core::default::Default::default(),
            run_as_non_root: core::default::Default::default(),
            run_as_user: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextElRef
    {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `privileged` after provisioning.\n"]
    pub fn privileged(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.privileged", self.base))
    }
    #[doc = "Get a reference to the value of field `read_only_root_file_system` after provisioning.\n"]
    pub fn read_only_root_file_system(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_only_root_file_system", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `run_as_group` after provisioning.\n"]
    pub fn run_as_group(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_as_group", self.base))
    }
    #[doc = "Get a reference to the value of field `run_as_non_root` after provisioning.\n"]
    pub fn run_as_non_root(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.run_as_non_root", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `run_as_user` after provisioning.\n"]
    pub fn run_as_user(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_as_user", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl {
    #[doc = "Set the field `mount_path`.\n"]
    pub fn set_mount_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mount_path = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `read_only`.\n"]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl
{
    type O = BlockAssignable<
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl
{}
impl BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl {
    pub fn build(
        self,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl {
            mount_path: core::default::Default::default(),
            name: core::default::Default::default(),
            read_only: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsElRef {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<
        ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_pull_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<
        ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_context: Option<
        ListField<
            DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_mounts: Option<
        ListField<
            DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl,
        >,
    >,
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl {
    #[doc = "Set the field `args`.\n"]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }
    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl Into<
            ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvEl>,
        >,
    ) -> Self {
        self.env = Some(v.into());
        self
    }
    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image = Some(v.into());
        self
    }
    #[doc = "Set the field `image_pull_policy`.\n"]
    pub fn set_image_pull_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_pull_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl Into<
            ListField<
                DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesEl,
            >,
        >,
    ) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `security_context`.\n"]
    pub fn set_security_context(
        mut self,
        v : impl Into < ListField < DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextEl > >,
    ) -> Self {
        self.security_context = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_mounts`.\n"]
    pub fn set_volume_mounts(
        mut self,
        v: impl Into<
            ListField<
                DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsEl,
            >,
        >,
    ) -> Self {
        self.volume_mounts = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl {
    type O = BlockAssignable<DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl {}
impl BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl {
    pub fn build(self) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            env: core::default::Default::default(),
            image: core::default::Default::default(),
            image_pull_policy: core::default::Default::default(),
            name: core::default::Default::default(),
            resources: core::default::Default::default(),
            security_context: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElRef {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(
        &self,
    ) -> ListRef<DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElEnvElRef> {
        ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `image_pull_policy` after provisioning.\n"]
    pub fn image_pull_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_pull_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(
        &self,
    ) -> ListRef<DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElResourcesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `security_context` after provisioning.\n"]
    pub fn security_context(
        &self,
    ) -> ListRef<
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElSecurityContextElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_context", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(
        &self,
    ) -> ListRef<
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElVolumeMountsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.volume_mounts", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl {
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl {
    type O = BlockAssignable<DataBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl {}
impl BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl {
    pub fn build(self) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl {
            labels: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataElRef {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    medium: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_limit: Option<PrimField<String>>,
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl {
    #[doc = "Set the field `medium`.\n"]
    pub fn set_medium(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.medium = Some(v.into());
        self
    }
    #[doc = "Set the field `size_limit`.\n"]
    pub fn set_size_limit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.size_limit = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl {
    type O =
        BlockAssignable<DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl {}
impl BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl {
    pub fn build(self) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl {
            medium: core::default::Default::default(),
            size_limit: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirElRef {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `medium` after provisioning.\n"]
    pub fn medium(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.medium", self.base))
    }
    #[doc = "Get a reference to the value of field `size_limit` after provisioning.\n"]
    pub fn size_limit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_limit", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl {
    type O =
        BlockAssignable<DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl {}
impl BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl {
    pub fn build(self) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl {
            path: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathElRef {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    optional: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_name: Option<PrimField<String>>,
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl {
    #[doc = "Set the field `optional`.\n"]
    pub fn set_optional(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.optional = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_name`.\n"]
    pub fn set_secret_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl {
    type O = BlockAssignable<DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl {}
impl BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl {
    pub fn build(self) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl {
            optional: core::default::Default::default(),
            secret_name: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretElRef {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `optional` after provisioning.\n"]
    pub fn optional(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.optional", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    empty_dir:
        Option<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_path:
        Option<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret:
        Option<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl>>,
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl {
    #[doc = "Set the field `empty_dir`.\n"]
    pub fn set_empty_dir(
        mut self,
        v: impl Into<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirEl>>,
    ) -> Self {
        self.empty_dir = Some(v.into());
        self
    }
    #[doc = "Set the field `host_path`.\n"]
    pub fn set_host_path(
        mut self,
        v: impl Into<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathEl>>,
    ) -> Self {
        self.host_path = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v: impl Into<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretEl>>,
    ) -> Self {
        self.secret = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl {
    type O = BlockAssignable<DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl {}
impl BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl {
    pub fn build(self) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl {
            empty_dir: core::default::Default::default(),
            host_path: core::default::Default::default(),
            name: core::default::Default::default(),
            secret: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElRef {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `empty_dir` after provisioning.\n"]
    pub fn empty_dir(
        &self,
    ) -> ListRef<DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElEmptyDirElRef> {
        ListRef::new(self.shared().clone(), format!("{}.empty_dir", self.base))
    }
    #[doc = "Get a reference to the value of field `host_path` after provisioning.\n"]
    pub fn host_path(
        &self,
    ) -> ListRef<DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElHostPathElRef> {
        ListRef::new(self.shared().clone(), format!("{}.host_path", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(
        &self,
    ) -> ListRef<DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElSecretElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    containers: Option<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_network: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_pull_secrets:
        Option<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    init_containers:
        Option<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_process_namespace: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes: Option<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl>>,
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesEl {
    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v: impl Into<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersEl>>,
    ) -> Self {
        self.containers = Some(v.into());
        self
    }
    #[doc = "Set the field `dns_policy`.\n"]
    pub fn set_dns_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dns_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `host_network`.\n"]
    pub fn set_host_network(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.host_network = Some(v.into());
        self
    }
    #[doc = "Set the field `image_pull_secrets`.\n"]
    pub fn set_image_pull_secrets(
        mut self,
        v: impl Into<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretsEl>>,
    ) -> Self {
        self.image_pull_secrets = Some(v.into());
        self
    }
    #[doc = "Set the field `init_containers`.\n"]
    pub fn set_init_containers(
        mut self,
        v: impl Into<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersEl>>,
    ) -> Self {
        self.init_containers = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl Into<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataEl>>,
    ) -> Self {
        self.metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_name`.\n"]
    pub fn set_service_account_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_name = Some(v.into());
        self
    }
    #[doc = "Set the field `share_process_namespace`.\n"]
    pub fn set_share_process_namespace(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.share_process_namespace = Some(v.into());
        self
    }
    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl Into<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesEl>>,
    ) -> Self {
        self.volumes = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionEksPropertiesElPodPropertiesEl {
    type O = BlockAssignable<DataBatchJobDefinitionEksPropertiesElPodPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesEl {}
impl BuildDataBatchJobDefinitionEksPropertiesElPodPropertiesEl {
    pub fn build(self) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesEl {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesEl {
            containers: core::default::Default::default(),
            dns_policy: core::default::Default::default(),
            host_network: core::default::Default::default(),
            image_pull_secrets: core::default::Default::default(),
            init_containers: core::default::Default::default(),
            metadata: core::default::Default::default(),
            service_account_name: core::default::Default::default(),
            share_process_namespace: core::default::Default::default(),
            volumes: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElPodPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionEksPropertiesElPodPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionEksPropertiesElPodPropertiesElRef {
        DataBatchJobDefinitionEksPropertiesElPodPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElPodPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(
        &self,
    ) -> ListRef<DataBatchJobDefinitionEksPropertiesElPodPropertiesElContainersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }
    #[doc = "Get a reference to the value of field `dns_policy` after provisioning.\n"]
    pub fn dns_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_policy", self.base))
    }
    #[doc = "Get a reference to the value of field `host_network` after provisioning.\n"]
    pub fn host_network(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_network", self.base))
    }
    #[doc = "Get a reference to the value of field `image_pull_secrets` after provisioning.\n"]
    pub fn image_pull_secrets(
        &self,
    ) -> ListRef<DataBatchJobDefinitionEksPropertiesElPodPropertiesElImagePullSecretsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_pull_secrets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `init_containers` after provisioning.\n"]
    pub fn init_containers(
        &self,
    ) -> ListRef<DataBatchJobDefinitionEksPropertiesElPodPropertiesElInitContainersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.init_containers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(
        &self,
    ) -> ListRef<DataBatchJobDefinitionEksPropertiesElPodPropertiesElMetadataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_name` after provisioning.\n"]
    pub fn service_account_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `share_process_namespace` after provisioning.\n"]
    pub fn share_process_namespace(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.share_process_namespace", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(
        &self,
    ) -> ListRef<DataBatchJobDefinitionEksPropertiesElPodPropertiesElVolumesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionEksPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pod_properties: Option<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesEl>>,
}
impl DataBatchJobDefinitionEksPropertiesEl {
    #[doc = "Set the field `pod_properties`.\n"]
    pub fn set_pod_properties(
        mut self,
        v: impl Into<ListField<DataBatchJobDefinitionEksPropertiesElPodPropertiesEl>>,
    ) -> Self {
        self.pod_properties = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionEksPropertiesEl {
    type O = BlockAssignable<DataBatchJobDefinitionEksPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionEksPropertiesEl {}
impl BuildDataBatchJobDefinitionEksPropertiesEl {
    pub fn build(self) -> DataBatchJobDefinitionEksPropertiesEl {
        DataBatchJobDefinitionEksPropertiesEl {
            pod_properties: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionEksPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionEksPropertiesElRef {
    fn new(shared: StackShared, base: String) -> DataBatchJobDefinitionEksPropertiesElRef {
        DataBatchJobDefinitionEksPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionEksPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pod_properties` after provisioning.\n"]
    pub fn pod_properties(
        &self,
    ) -> ListRef<DataBatchJobDefinitionEksPropertiesElPodPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pod_properties", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEnvironmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEnvironmentEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEnvironmentEl
{
    type O = BlockAssignable<
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEnvironmentEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEnvironmentEl
{}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEnvironmentEl {
    pub fn build(
        self,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEnvironmentEl {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEnvironmentEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEnvironmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEnvironmentElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEnvironmentElRef
    {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEnvironmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEphemeralStorageEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    size_in_gib: Option<PrimField<f64>>,
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEphemeralStorageEl {
    #[doc = "Set the field `size_in_gib`.\n"]
    pub fn set_size_in_gib(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size_in_gib = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEphemeralStorageEl
{
    type O = BlockAssignable<
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEphemeralStorageEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEphemeralStorageEl
{}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEphemeralStorageEl {
    pub fn build(
        self,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEphemeralStorageEl
    {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEphemeralStorageEl {
            size_in_gib: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEphemeralStorageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEphemeralStorageElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEphemeralStorageElRef
    {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEphemeralStorageElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEphemeralStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `size_in_gib` after provisioning.\n"]
    pub fn size_in_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_in_gib", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElFargatePlatformConfigurationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_version: Option<PrimField<String>>,
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElFargatePlatformConfigurationEl { # [doc = "Set the field `platform_version`.\n"] pub fn set_platform_version (mut self , v : impl Into < PrimField < String > >) -> Self { self . platform_version = Some (v . into ()) ; self } }
impl ToListMappable for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElFargatePlatformConfigurationEl { type O = BlockAssignable < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElFargatePlatformConfigurationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElFargatePlatformConfigurationEl
{}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElFargatePlatformConfigurationEl { pub fn build (self) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElFargatePlatformConfigurationEl { DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElFargatePlatformConfigurationEl { platform_version : core :: default :: Default :: default () , } } }
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElFargatePlatformConfigurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElFargatePlatformConfigurationElRef { fn new (shared : StackShared , base : String) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElFargatePlatformConfigurationElRef { DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElFargatePlatformConfigurationElRef { shared : shared , base : base . to_string () , } } }
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElFargatePlatformConfigurationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `platform_version` after provisioning.\n"] pub fn platform_version (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.platform_version" , self . base)) } }
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElDevicesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    container_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<ListField<PrimField<String>>>,
}
impl
    DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElDevicesEl
{
    #[doc = "Set the field `container_path`.\n"]
    pub fn set_container_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_path = Some(v.into());
        self
    }
    #[doc = "Set the field `host_path`.\n"]
    pub fn set_host_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_path = Some(v.into());
        self
    }
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.permissions = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElDevicesEl { type O = BlockAssignable < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElDevicesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElDevicesEl
{}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElDevicesEl { pub fn build (self) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElDevicesEl { DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElDevicesEl { container_path : core :: default :: Default :: default () , host_path : core :: default :: Default :: default () , permissions : core :: default :: Default :: default () , } } }
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElDevicesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElDevicesElRef { fn new (shared : StackShared , base : String) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElDevicesElRef { DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElDevicesElRef { shared : shared , base : base . to_string () , } } }
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElDevicesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `container_path` after provisioning.\n"] pub fn container_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.container_path" , self . base)) } # [doc = "Get a reference to the value of field `host_path` after provisioning.\n"] pub fn host_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.host_path" , self . base)) } # [doc = "Get a reference to the value of field `permissions` after provisioning.\n"] pub fn permissions (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.permissions" , self . base)) } }
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElTmpfsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    container_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_options: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
}
impl
    DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElTmpfsEl
{
    #[doc = "Set the field `container_path`.\n"]
    pub fn set_container_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_path = Some(v.into());
        self
    }
    #[doc = "Set the field `mount_options`.\n"]
    pub fn set_mount_options(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.mount_options = Some(v.into());
        self
    }
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElTmpfsEl { type O = BlockAssignable < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElTmpfsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElTmpfsEl
{}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElTmpfsEl { pub fn build (self) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElTmpfsEl { DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElTmpfsEl { container_path : core :: default :: Default :: default () , mount_options : core :: default :: Default :: default () , size : core :: default :: Default :: default () , } } }
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElTmpfsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElTmpfsElRef { fn new (shared : StackShared , base : String) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElTmpfsElRef { DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElTmpfsElRef { shared : shared , base : base . to_string () , } } }
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElTmpfsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `container_path` after provisioning.\n"] pub fn container_path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.container_path" , self . base)) } # [doc = "Get a reference to the value of field `mount_options` after provisioning.\n"] pub fn mount_options (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.mount_options" , self . base)) } # [doc = "Get a reference to the value of field `size` after provisioning.\n"] pub fn size (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.size" , self . base)) } }
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersEl { # [serde (skip_serializing_if = "Option::is_none")] devices : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElDevicesEl > > , # [serde (skip_serializing_if = "Option::is_none")] init_process_enabled : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] max_swap : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] shared_memory_size : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] swappiness : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] tmpfs : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElTmpfsEl > > , }
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersEl {
    #[doc = "Set the field `devices`.\n"]
    pub fn set_devices(
        mut self,
        v : impl Into < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElDevicesEl > >,
    ) -> Self {
        self.devices = Some(v.into());
        self
    }
    #[doc = "Set the field `init_process_enabled`.\n"]
    pub fn set_init_process_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.init_process_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `max_swap`.\n"]
    pub fn set_max_swap(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_swap = Some(v.into());
        self
    }
    #[doc = "Set the field `shared_memory_size`.\n"]
    pub fn set_shared_memory_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.shared_memory_size = Some(v.into());
        self
    }
    #[doc = "Set the field `swappiness`.\n"]
    pub fn set_swappiness(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.swappiness = Some(v.into());
        self
    }
    #[doc = "Set the field `tmpfs`.\n"]
    pub fn set_tmpfs(
        mut self,
        v : impl Into < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElTmpfsEl > >,
    ) -> Self {
        self.tmpfs = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersEl
{
    type O = BlockAssignable<
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersEl
{}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersEl {
    pub fn build(
        self,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersEl
    {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersEl {
            devices: core::default::Default::default(),
            init_process_enabled: core::default::Default::default(),
            max_swap: core::default::Default::default(),
            shared_memory_size: core::default::Default::default(),
            swappiness: core::default::Default::default(),
            tmpfs: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElRef
    {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `devices` after provisioning.\n"]    pub fn devices (& self) -> ListRef < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElDevicesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.devices", self.base))
    }
    #[doc = "Get a reference to the value of field `init_process_enabled` after provisioning.\n"]
    pub fn init_process_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.init_process_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_swap` after provisioning.\n"]
    pub fn max_swap(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_swap", self.base))
    }
    #[doc = "Get a reference to the value of field `shared_memory_size` after provisioning.\n"]
    pub fn shared_memory_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.shared_memory_size", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `swappiness` after provisioning.\n"]
    pub fn swappiness(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.swappiness", self.base))
    }
    #[doc = "Get a reference to the value of field `tmpfs` after provisioning.\n"]    pub fn tmpfs (& self) -> ListRef < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElTmpfsElRef >{
        ListRef::new(self.shared().clone(), format!("{}.tmpfs", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElSecretOptionsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_from: Option<PrimField<String>>,
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElSecretOptionsEl { # [doc = "Set the field `name`.\n"] pub fn set_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . name = Some (v . into ()) ; self } # [doc = "Set the field `value_from`.\n"] pub fn set_value_from (mut self , v : impl Into < PrimField < String > >) -> Self { self . value_from = Some (v . into ()) ; self } }
impl ToListMappable for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElSecretOptionsEl { type O = BlockAssignable < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElSecretOptionsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElSecretOptionsEl
{}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElSecretOptionsEl { pub fn build (self) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElSecretOptionsEl { DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElSecretOptionsEl { name : core :: default :: Default :: default () , value_from : core :: default :: Default :: default () , } } }
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElSecretOptionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElSecretOptionsElRef { fn new (shared : StackShared , base : String) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElSecretOptionsElRef { DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElSecretOptionsElRef { shared : shared , base : base . to_string () , } } }
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElSecretOptionsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\n"] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `value_from` after provisioning.\n"] pub fn value_from (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value_from" , self . base)) } }
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationEl { # [serde (skip_serializing_if = "Option::is_none")] log_driver : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] options : Option < RecField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] secret_options : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElSecretOptionsEl > > , }
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationEl {
    #[doc = "Set the field `log_driver`.\n"]
    pub fn set_log_driver(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_driver = Some(v.into());
        self
    }
    #[doc = "Set the field `options`.\n"]
    pub fn set_options(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.options = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_options`.\n"]
    pub fn set_secret_options(
        mut self,
        v : impl Into < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElSecretOptionsEl > >,
    ) -> Self {
        self.secret_options = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationEl
{
    type O = BlockAssignable<
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationEl
{}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationEl {
    pub fn build(
        self,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationEl
    {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationEl {
            log_driver: core::default::Default::default(),
            options: core::default::Default::default(),
            secret_options: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElRef
    {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `log_driver` after provisioning.\n"]
    pub fn log_driver(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_driver", self.base))
    }
    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.options", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_options` after provisioning.\n"]    pub fn secret_options (& self) -> ListRef < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElSecretOptionsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_options", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElMountPointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_volume: Option<PrimField<String>>,
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElMountPointsEl {
    #[doc = "Set the field `container_path`.\n"]
    pub fn set_container_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_path = Some(v.into());
        self
    }
    #[doc = "Set the field `read_only`.\n"]
    pub fn set_read_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_only = Some(v.into());
        self
    }
    #[doc = "Set the field `source_volume`.\n"]
    pub fn set_source_volume(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_volume = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElMountPointsEl
{
    type O = BlockAssignable<
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElMountPointsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElMountPointsEl
{}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElMountPointsEl {
    pub fn build(
        self,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElMountPointsEl {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElMountPointsEl {
            container_path: core::default::Default::default(),
            read_only: core::default::Default::default(),
            source_volume: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElMountPointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElMountPointsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElMountPointsElRef
    {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElMountPointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElMountPointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_path` after provisioning.\n"]
    pub fn container_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }
    #[doc = "Get a reference to the value of field `source_volume` after provisioning.\n"]
    pub fn source_volume(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_volume", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElNetworkConfigurationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    assign_public_ip: Option<PrimField<bool>>,
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElNetworkConfigurationEl {
    #[doc = "Set the field `assign_public_ip`.\n"]
    pub fn set_assign_public_ip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.assign_public_ip = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElNetworkConfigurationEl
{
    type O = BlockAssignable < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElNetworkConfigurationEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElNetworkConfigurationEl
{}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElNetworkConfigurationEl { pub fn build (self) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElNetworkConfigurationEl { DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElNetworkConfigurationEl { assign_public_ip : core :: default :: Default :: default () , } } }
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElNetworkConfigurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElNetworkConfigurationElRef { fn new (shared : StackShared , base : String) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElNetworkConfigurationElRef { DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElNetworkConfigurationElRef { shared : shared , base : base . to_string () , } } }
impl
    DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElNetworkConfigurationElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `assign_public_ip` after provisioning.\n"]
    pub fn assign_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assign_public_ip", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElResourceRequirementsEl
{
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElResourceRequirementsEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElResourceRequirementsEl
{
    type O = BlockAssignable < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElResourceRequirementsEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElResourceRequirementsEl
{}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElResourceRequirementsEl { pub fn build (self) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElResourceRequirementsEl { DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElResourceRequirementsEl { type_ : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElResourceRequirementsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElResourceRequirementsElRef { fn new (shared : StackShared , base : String) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElResourceRequirementsElRef { DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElResourceRequirementsElRef { shared : shared , base : base . to_string () , } } }
impl
    DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElResourceRequirementsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRuntimePlatformEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_architecture: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operating_system_family: Option<PrimField<String>>,
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRuntimePlatformEl {
    #[doc = "Set the field `cpu_architecture`.\n"]
    pub fn set_cpu_architecture(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu_architecture = Some(v.into());
        self
    }
    #[doc = "Set the field `operating_system_family`.\n"]
    pub fn set_operating_system_family(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operating_system_family = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRuntimePlatformEl
{
    type O = BlockAssignable<
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRuntimePlatformEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRuntimePlatformEl
{}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRuntimePlatformEl {
    pub fn build(
        self,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRuntimePlatformEl
    {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRuntimePlatformEl {
            cpu_architecture: core::default::Default::default(),
            operating_system_family: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRuntimePlatformElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRuntimePlatformElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRuntimePlatformElRef
    {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRuntimePlatformElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRuntimePlatformElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu_architecture` after provisioning.\n"]
    pub fn cpu_architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_architecture", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `operating_system_family` after provisioning.\n"]
    pub fn operating_system_family(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operating_system_family", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElSecretsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_from: Option<PrimField<String>>,
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElSecretsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value_from`.\n"]
    pub fn set_value_from(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value_from = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElSecretsEl
{
    type O = BlockAssignable<
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElSecretsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElSecretsEl {}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElSecretsEl {
    pub fn build(
        self,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElSecretsEl {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElSecretsEl {
            name: core::default::Default::default(),
            value_from: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElSecretsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElSecretsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElSecretsElRef {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElSecretsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElSecretsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value_from` after provisioning.\n"]
    pub fn value_from(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value_from", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElUlimitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hard_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    soft_limit: Option<PrimField<f64>>,
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElUlimitsEl {
    #[doc = "Set the field `hard_limit`.\n"]
    pub fn set_hard_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hard_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `soft_limit`.\n"]
    pub fn set_soft_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.soft_limit = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElUlimitsEl
{
    type O = BlockAssignable<
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElUlimitsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElUlimitsEl {}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElUlimitsEl {
    pub fn build(
        self,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElUlimitsEl {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElUlimitsEl {
            hard_limit: core::default::Default::default(),
            name: core::default::Default::default(),
            soft_limit: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElUlimitsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElUlimitsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElUlimitsElRef {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElUlimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElUlimitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hard_limit` after provisioning.\n"]
    pub fn hard_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hard_limit", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `soft_limit` after provisioning.\n"]
    pub fn soft_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.soft_limit", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElAuthorizationConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    access_point_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam: Option<PrimField<String>>,
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElAuthorizationConfigEl { # [doc = "Set the field `access_point_id`.\n"] pub fn set_access_point_id (mut self , v : impl Into < PrimField < String > >) -> Self { self . access_point_id = Some (v . into ()) ; self } # [doc = "Set the field `iam`.\n"] pub fn set_iam (mut self , v : impl Into < PrimField < String > >) -> Self { self . iam = Some (v . into ()) ; self } }
impl ToListMappable for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElAuthorizationConfigEl { type O = BlockAssignable < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElAuthorizationConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElAuthorizationConfigEl
{}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElAuthorizationConfigEl { pub fn build (self) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElAuthorizationConfigEl { DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElAuthorizationConfigEl { access_point_id : core :: default :: Default :: default () , iam : core :: default :: Default :: default () , } } }
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElAuthorizationConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElAuthorizationConfigElRef { fn new (shared : StackShared , base : String) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElAuthorizationConfigElRef { DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElAuthorizationConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElAuthorizationConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `access_point_id` after provisioning.\n"] pub fn access_point_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.access_point_id" , self . base)) } # [doc = "Get a reference to the value of field `iam` after provisioning.\n"] pub fn iam (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.iam" , self . base)) } }
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationEl { # [serde (skip_serializing_if = "Option::is_none")] authorization_config : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElAuthorizationConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] file_system_id : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] root_directory : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] transit_encryption : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] transit_encryption_port : Option < PrimField < f64 > > , }
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationEl { # [doc = "Set the field `authorization_config`.\n"] pub fn set_authorization_config (mut self , v : impl Into < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElAuthorizationConfigEl > >) -> Self { self . authorization_config = Some (v . into ()) ; self } # [doc = "Set the field `file_system_id`.\n"] pub fn set_file_system_id (mut self , v : impl Into < PrimField < String > >) -> Self { self . file_system_id = Some (v . into ()) ; self } # [doc = "Set the field `root_directory`.\n"] pub fn set_root_directory (mut self , v : impl Into < PrimField < String > >) -> Self { self . root_directory = Some (v . into ()) ; self } # [doc = "Set the field `transit_encryption`.\n"] pub fn set_transit_encryption (mut self , v : impl Into < PrimField < String > >) -> Self { self . transit_encryption = Some (v . into ()) ; self } # [doc = "Set the field `transit_encryption_port`.\n"] pub fn set_transit_encryption_port (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . transit_encryption_port = Some (v . into ()) ; self } }
impl ToListMappable for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationEl { type O = BlockAssignable < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationEl
{}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationEl { pub fn build (self) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationEl { DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationEl { authorization_config : core :: default :: Default :: default () , file_system_id : core :: default :: Default :: default () , root_directory : core :: default :: Default :: default () , transit_encryption : core :: default :: Default :: default () , transit_encryption_port : core :: default :: Default :: default () , } } }
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElRef { fn new (shared : StackShared , base : String) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElRef { DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElRef { shared : shared , base : base . to_string () , } } }
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `authorization_config` after provisioning.\n"] pub fn authorization_config (& self) -> ListRef < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElAuthorizationConfigElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.authorization_config" , self . base)) } # [doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"] pub fn file_system_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.file_system_id" , self . base)) } # [doc = "Get a reference to the value of field `root_directory` after provisioning.\n"] pub fn root_directory (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.root_directory" , self . base)) } # [doc = "Get a reference to the value of field `transit_encryption` after provisioning.\n"] pub fn transit_encryption (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.transit_encryption" , self . base)) } # [doc = "Get a reference to the value of field `transit_encryption_port` after provisioning.\n"] pub fn transit_encryption_port (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.transit_encryption_port" , self . base)) } }
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElHostEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_path: Option<PrimField<String>>,
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElHostEl {
    #[doc = "Set the field `source_path`.\n"]
    pub fn set_source_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_path = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElHostEl
{
    type O = BlockAssignable<
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElHostEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElHostEl
{}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElHostEl {
    pub fn build(
        self,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElHostEl {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElHostEl {
            source_path: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElHostElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElHostElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElHostElRef
    {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElHostElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElHostElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `source_path` after provisioning.\n"]
    pub fn source_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_path", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesEl { # [serde (skip_serializing_if = "Option::is_none")] efs_volume_configuration : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationEl > > , # [serde (skip_serializing_if = "Option::is_none")] host : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElHostEl > > , # [serde (skip_serializing_if = "Option::is_none")] name : Option < PrimField < String > > , }
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesEl {
    #[doc = "Set the field `efs_volume_configuration`.\n"]
    pub fn set_efs_volume_configuration(
        mut self,
        v : impl Into < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationEl > >,
    ) -> Self {
        self.efs_volume_configuration = Some(v.into());
        self
    }
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(
        mut self,
        v : impl Into < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElHostEl > >,
    ) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesEl
{
    type O = BlockAssignable<
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesEl {}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesEl {
    pub fn build(
        self,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesEl {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesEl {
            efs_volume_configuration: core::default::Default::default(),
            host: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElRef {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `efs_volume_configuration` after provisioning.\n"]    pub fn efs_volume_configuration (& self) -> ListRef < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElEfsVolumeConfigurationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.efs_volume_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(
        &self,
    ) -> ListRef<
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElHostElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerEl { # [serde (skip_serializing_if = "Option::is_none")] command : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] environment : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEnvironmentEl > > , # [serde (skip_serializing_if = "Option::is_none")] ephemeral_storage : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEphemeralStorageEl > > , # [serde (skip_serializing_if = "Option::is_none")] execution_role_arn : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] fargate_platform_configuration : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElFargatePlatformConfigurationEl > > , # [serde (skip_serializing_if = "Option::is_none")] image : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] instance_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] job_role_arn : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] linux_parameters : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersEl > > , # [serde (skip_serializing_if = "Option::is_none")] log_configuration : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationEl > > , # [serde (skip_serializing_if = "Option::is_none")] mount_points : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElMountPointsEl > > , # [serde (skip_serializing_if = "Option::is_none")] network_configuration : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElNetworkConfigurationEl > > , # [serde (skip_serializing_if = "Option::is_none")] privileged : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] readonly_root_filesystem : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] resource_requirements : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElResourceRequirementsEl > > , # [serde (skip_serializing_if = "Option::is_none")] runtime_platform : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRuntimePlatformEl > > , # [serde (skip_serializing_if = "Option::is_none")] secrets : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElSecretsEl > > , # [serde (skip_serializing_if = "Option::is_none")] ulimits : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElUlimitsEl > > , # [serde (skip_serializing_if = "Option::is_none")] user : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] volumes : Option < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesEl > > , }
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerEl {
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl Into<
            ListField<
                DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEnvironmentEl,
            >,
        >,
    ) -> Self {
        self.environment = Some(v.into());
        self
    }
    #[doc = "Set the field `ephemeral_storage`.\n"]
    pub fn set_ephemeral_storage(
        mut self,
        v : impl Into < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEphemeralStorageEl > >,
    ) -> Self {
        self.ephemeral_storage = Some(v.into());
        self
    }
    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `fargate_platform_configuration`.\n"]
    pub fn set_fargate_platform_configuration(
        mut self,
        v : impl Into < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElFargatePlatformConfigurationEl > >,
    ) -> Self {
        self.fargate_platform_configuration = Some(v.into());
        self
    }
    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `job_role_arn`.\n"]
    pub fn set_job_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.job_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `linux_parameters`.\n"]
    pub fn set_linux_parameters(
        mut self,
        v : impl Into < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersEl > >,
    ) -> Self {
        self.linux_parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `log_configuration`.\n"]
    pub fn set_log_configuration(
        mut self,
        v : impl Into < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationEl > >,
    ) -> Self {
        self.log_configuration = Some(v.into());
        self
    }
    #[doc = "Set the field `mount_points`.\n"]
    pub fn set_mount_points(
        mut self,
        v: impl Into<
            ListField<
                DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElMountPointsEl,
            >,
        >,
    ) -> Self {
        self.mount_points = Some(v.into());
        self
    }
    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        mut self,
        v : impl Into < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElNetworkConfigurationEl > >,
    ) -> Self {
        self.network_configuration = Some(v.into());
        self
    }
    #[doc = "Set the field `privileged`.\n"]
    pub fn set_privileged(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.privileged = Some(v.into());
        self
    }
    #[doc = "Set the field `readonly_root_filesystem`.\n"]
    pub fn set_readonly_root_filesystem(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.readonly_root_filesystem = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_requirements`.\n"]
    pub fn set_resource_requirements(
        mut self,
        v : impl Into < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElResourceRequirementsEl > >,
    ) -> Self {
        self.resource_requirements = Some(v.into());
        self
    }
    #[doc = "Set the field `runtime_platform`.\n"]
    pub fn set_runtime_platform(
        mut self,
        v : impl Into < ListField < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRuntimePlatformEl > >,
    ) -> Self {
        self.runtime_platform = Some(v.into());
        self
    }
    #[doc = "Set the field `secrets`.\n"]
    pub fn set_secrets(
        mut self,
        v: impl Into<
            ListField<
                DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElSecretsEl,
            >,
        >,
    ) -> Self {
        self.secrets = Some(v.into());
        self
    }
    #[doc = "Set the field `ulimits`.\n"]
    pub fn set_ulimits(
        mut self,
        v: impl Into<
            ListField<
                DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElUlimitsEl,
            >,
        >,
    ) -> Self {
        self.ulimits = Some(v.into());
        self
    }
    #[doc = "Set the field `user`.\n"]
    pub fn set_user(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user = Some(v.into());
        self
    }
    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl Into<
            ListField<
                DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesEl,
            >,
        >,
    ) -> Self {
        self.volumes = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerEl {
    type O =
        BlockAssignable<DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerEl {}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerEl {
    pub fn build(self) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerEl {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerEl {
            command: core::default::Default::default(),
            environment: core::default::Default::default(),
            ephemeral_storage: core::default::Default::default(),
            execution_role_arn: core::default::Default::default(),
            fargate_platform_configuration: core::default::Default::default(),
            image: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            job_role_arn: core::default::Default::default(),
            linux_parameters: core::default::Default::default(),
            log_configuration: core::default::Default::default(),
            mount_points: core::default::Default::default(),
            network_configuration: core::default::Default::default(),
            privileged: core::default::Default::default(),
            readonly_root_filesystem: core::default::Default::default(),
            resource_requirements: core::default::Default::default(),
            runtime_platform: core::default::Default::default(),
            secrets: core::default::Default::default(),
            ulimits: core::default::Default::default(),
            user: core::default::Default::default(),
            volumes: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRef {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(
        &self,
    ) -> ListRef<
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEnvironmentElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }
    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(
        &self,
    ) -> ListRef<
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElEphemeralStorageElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ephemeral_storage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fargate_platform_configuration` after provisioning.\n"]    pub fn fargate_platform_configuration (& self) -> ListRef < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElFargatePlatformConfigurationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.fargate_platform_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `job_role_arn` after provisioning.\n"]
    pub fn job_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `linux_parameters` after provisioning.\n"]
    pub fn linux_parameters(
        &self,
    ) -> ListRef<
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLinuxParametersElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linux_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(
        &self,
    ) -> ListRef<
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElLogConfigurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mount_points` after provisioning.\n"]
    pub fn mount_points(
        &self,
    ) -> ListRef<
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElMountPointsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.mount_points", self.base))
    }
    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]    pub fn network_configuration (& self) -> ListRef < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElNetworkConfigurationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `privileged` after provisioning.\n"]
    pub fn privileged(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.privileged", self.base))
    }
    #[doc = "Get a reference to the value of field `readonly_root_filesystem` after provisioning.\n"]
    pub fn readonly_root_filesystem(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.readonly_root_filesystem", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_requirements` after provisioning.\n"]    pub fn resource_requirements (& self) -> ListRef < DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElResourceRequirementsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_requirements", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_platform` after provisioning.\n"]
    pub fn runtime_platform(
        &self,
    ) -> ListRef<
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRuntimePlatformElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.runtime_platform", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secrets` after provisioning.\n"]
    pub fn secrets(
        &self,
    ) -> ListRef<DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElSecretsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.secrets", self.base))
    }
    #[doc = "Get a reference to the value of field `ulimits` after provisioning.\n"]
    pub fn ulimits(
        &self,
    ) -> ListRef<DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElUlimitsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.ulimits", self.base))
    }
    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(
        &self,
    ) -> ListRef<DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElVolumesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container:
        Option<ListField<DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_nodes: Option<PrimField<String>>,
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesEl {
    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        mut self,
        v: impl Into<ListField<DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerEl>>,
    ) -> Self {
        self.container = Some(v.into());
        self
    }
    #[doc = "Set the field `target_nodes`.\n"]
    pub fn set_target_nodes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_nodes = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesEl {
    type O = BlockAssignable<DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesEl {}
impl BuildDataBatchJobDefinitionNodePropertiesElNodeRangePropertiesEl {
    pub fn build(self) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesEl {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesEl {
            container: core::default::Default::default(),
            target_nodes: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElRef {
        DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(
        &self,
    ) -> ListRef<DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElContainerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.container", self.base))
    }
    #[doc = "Get a reference to the value of field `target_nodes` after provisioning.\n"]
    pub fn target_nodes(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_nodes", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionNodePropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    main_node: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_range_properties:
        Option<ListField<DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_nodes: Option<PrimField<f64>>,
}
impl DataBatchJobDefinitionNodePropertiesEl {
    #[doc = "Set the field `main_node`.\n"]
    pub fn set_main_node(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.main_node = Some(v.into());
        self
    }
    #[doc = "Set the field `node_range_properties`.\n"]
    pub fn set_node_range_properties(
        mut self,
        v: impl Into<ListField<DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesEl>>,
    ) -> Self {
        self.node_range_properties = Some(v.into());
        self
    }
    #[doc = "Set the field `num_nodes`.\n"]
    pub fn set_num_nodes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_nodes = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionNodePropertiesEl {
    type O = BlockAssignable<DataBatchJobDefinitionNodePropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionNodePropertiesEl {}
impl BuildDataBatchJobDefinitionNodePropertiesEl {
    pub fn build(self) -> DataBatchJobDefinitionNodePropertiesEl {
        DataBatchJobDefinitionNodePropertiesEl {
            main_node: core::default::Default::default(),
            node_range_properties: core::default::Default::default(),
            num_nodes: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionNodePropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionNodePropertiesElRef {
    fn new(shared: StackShared, base: String) -> DataBatchJobDefinitionNodePropertiesElRef {
        DataBatchJobDefinitionNodePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionNodePropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `main_node` after provisioning.\n"]
    pub fn main_node(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.main_node", self.base))
    }
    #[doc = "Get a reference to the value of field `node_range_properties` after provisioning.\n"]
    pub fn node_range_properties(
        &self,
    ) -> ListRef<DataBatchJobDefinitionNodePropertiesElNodeRangePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_range_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `num_nodes` after provisioning.\n"]
    pub fn num_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_nodes", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_exit_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_status_reason: Option<PrimField<String>>,
}
impl DataBatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }
    #[doc = "Set the field `on_exit_code`.\n"]
    pub fn set_on_exit_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_exit_code = Some(v.into());
        self
    }
    #[doc = "Set the field `on_reason`.\n"]
    pub fn set_on_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_reason = Some(v.into());
        self
    }
    #[doc = "Set the field `on_status_reason`.\n"]
    pub fn set_on_status_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_status_reason = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    type O = BlockAssignable<DataBatchJobDefinitionRetryStrategyElEvaluateOnExitEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionRetryStrategyElEvaluateOnExitEl {}
impl BuildDataBatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
    pub fn build(self) -> DataBatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
        DataBatchJobDefinitionRetryStrategyElEvaluateOnExitEl {
            action: core::default::Default::default(),
            on_exit_code: core::default::Default::default(),
            on_reason: core::default::Default::default(),
            on_status_reason: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
        DataBatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionRetryStrategyElEvaluateOnExitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `on_exit_code` after provisioning.\n"]
    pub fn on_exit_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_exit_code", self.base))
    }
    #[doc = "Get a reference to the value of field `on_reason` after provisioning.\n"]
    pub fn on_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_reason", self.base))
    }
    #[doc = "Get a reference to the value of field `on_status_reason` after provisioning.\n"]
    pub fn on_status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_status_reason", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionRetryStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attempts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluate_on_exit: Option<ListField<DataBatchJobDefinitionRetryStrategyElEvaluateOnExitEl>>,
}
impl DataBatchJobDefinitionRetryStrategyEl {
    #[doc = "Set the field `attempts`.\n"]
    pub fn set_attempts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.attempts = Some(v.into());
        self
    }
    #[doc = "Set the field `evaluate_on_exit`.\n"]
    pub fn set_evaluate_on_exit(
        mut self,
        v: impl Into<ListField<DataBatchJobDefinitionRetryStrategyElEvaluateOnExitEl>>,
    ) -> Self {
        self.evaluate_on_exit = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionRetryStrategyEl {
    type O = BlockAssignable<DataBatchJobDefinitionRetryStrategyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionRetryStrategyEl {}
impl BuildDataBatchJobDefinitionRetryStrategyEl {
    pub fn build(self) -> DataBatchJobDefinitionRetryStrategyEl {
        DataBatchJobDefinitionRetryStrategyEl {
            attempts: core::default::Default::default(),
            evaluate_on_exit: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionRetryStrategyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionRetryStrategyElRef {
    fn new(shared: StackShared, base: String) -> DataBatchJobDefinitionRetryStrategyElRef {
        DataBatchJobDefinitionRetryStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionRetryStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attempts` after provisioning.\n"]
    pub fn attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.attempts", self.base))
    }
    #[doc = "Get a reference to the value of field `evaluate_on_exit` after provisioning.\n"]
    pub fn evaluate_on_exit(
        &self,
    ) -> ListRef<DataBatchJobDefinitionRetryStrategyElEvaluateOnExitElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.evaluate_on_exit", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBatchJobDefinitionTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attempt_duration_seconds: Option<PrimField<f64>>,
}
impl DataBatchJobDefinitionTimeoutEl {
    #[doc = "Set the field `attempt_duration_seconds`.\n"]
    pub fn set_attempt_duration_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.attempt_duration_seconds = Some(v.into());
        self
    }
}
impl ToListMappable for DataBatchJobDefinitionTimeoutEl {
    type O = BlockAssignable<DataBatchJobDefinitionTimeoutEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBatchJobDefinitionTimeoutEl {}
impl BuildDataBatchJobDefinitionTimeoutEl {
    pub fn build(self) -> DataBatchJobDefinitionTimeoutEl {
        DataBatchJobDefinitionTimeoutEl {
            attempt_duration_seconds: core::default::Default::default(),
        }
    }
}
pub struct DataBatchJobDefinitionTimeoutElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBatchJobDefinitionTimeoutElRef {
    fn new(shared: StackShared, base: String) -> DataBatchJobDefinitionTimeoutElRef {
        DataBatchJobDefinitionTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBatchJobDefinitionTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attempt_duration_seconds` after provisioning.\n"]
    pub fn attempt_duration_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.attempt_duration_seconds", self.base),
        )
    }
}
