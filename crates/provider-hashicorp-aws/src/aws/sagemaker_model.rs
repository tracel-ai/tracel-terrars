use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SagemakerModelData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_network_isolation: Option<PrimField<bool>>,
    execution_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container: Option<Vec<SagemakerModelContainerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inference_execution_config: Option<Vec<SagemakerModelInferenceExecutionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_container: Option<Vec<SagemakerModelPrimaryContainerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_config: Option<Vec<SagemakerModelVpcConfigEl>>,
    dynamic: SagemakerModelDynamic,
}
struct SagemakerModel_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerModelData>,
}
#[derive(Clone)]
pub struct SagemakerModel(Rc<SagemakerModel_>);
impl SagemakerModel {
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
    #[doc = "Set the field `enable_network_isolation`.\n"]
    pub fn set_enable_network_isolation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_network_isolation = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
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
    #[doc = "Set the field `container`.\n"]
    pub fn set_container(self, v: impl Into<BlockAssignable<SagemakerModelContainerEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().container = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.container = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `inference_execution_config`.\n"]
    pub fn set_inference_execution_config(
        self,
        v: impl Into<BlockAssignable<SagemakerModelInferenceExecutionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inference_execution_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inference_execution_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `primary_container`.\n"]
    pub fn set_primary_container(
        self,
        v: impl Into<BlockAssignable<SagemakerModelPrimaryContainerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().primary_container = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.primary_container = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(self, v: impl Into<BlockAssignable<SagemakerModelVpcConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `enable_network_isolation` after provisioning.\n"]
    pub fn enable_network_isolation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_network_isolation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> ListRef<SagemakerModelContainerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `inference_execution_config` after provisioning.\n"]
    pub fn inference_execution_config(
        &self,
    ) -> ListRef<SagemakerModelInferenceExecutionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inference_execution_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `primary_container` after provisioning.\n"]
    pub fn primary_container(&self) -> ListRef<SagemakerModelPrimaryContainerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.primary_container", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<SagemakerModelVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
impl Referable for SagemakerModel {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SagemakerModel {}
impl ToListMappable for SagemakerModel {
    type O = ListRef<SagemakerModelRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SagemakerModel_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_model".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSagemakerModel {
    pub tf_id: String,
    #[doc = ""]
    pub execution_role_arn: PrimField<String>,
}
impl BuildSagemakerModel {
    pub fn build(self, stack: &mut Stack) -> SagemakerModel {
        let out = SagemakerModel(Rc::new(SagemakerModel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerModelData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                enable_network_isolation: core::default::Default::default(),
                execution_role_arn: self.execution_role_arn,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                container: core::default::Default::default(),
                inference_execution_config: core::default::Default::default(),
                primary_container: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SagemakerModelRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SagemakerModelRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `enable_network_isolation` after provisioning.\n"]
    pub fn enable_network_isolation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_network_isolation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> ListRef<SagemakerModelContainerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `inference_execution_config` after provisioning.\n"]
    pub fn inference_execution_config(
        &self,
    ) -> ListRef<SagemakerModelInferenceExecutionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inference_execution_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `primary_container` after provisioning.\n"]
    pub fn primary_container(&self) -> ListRef<SagemakerModelPrimaryContainerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.primary_container", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<SagemakerModelVpcConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl {
    accept_eula: PrimField<bool>,
}
impl SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl {}
impl ToListMappable
    for SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl
{
    type O = BlockAssignable<
        SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl
{
    #[doc = ""]
    pub accept_eula: PrimField<bool>,
}
impl BuildSagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl {
    pub fn build(
        self,
    ) -> SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl {
        SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl {
            accept_eula: self.accept_eula,
        }
    }
}
pub struct SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigElRef
    {
        SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accept_eula` after provisioning.\n"]
    pub fn accept_eula(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.accept_eula", self.base))
    }
}
#[derive(Serialize, Default)]
struct SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElDynamic {
    model_access_config: Option<
        DynamicBlock<
            SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceEl {
    compression_type: PrimField<String>,
    s3_data_type: PrimField<String>,
    s3_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_access_config: Option<
        Vec<SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl>,
    >,
    dynamic: SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElDynamic,
}
impl SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceEl {
    #[doc = "Set the field `model_access_config`.\n"]
    pub fn set_model_access_config(
        mut self,
        v : impl Into < BlockAssignable < SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.model_access_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.model_access_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceEl {
    type O = BlockAssignable<SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceEl {
    #[doc = ""]
    pub compression_type: PrimField<String>,
    #[doc = ""]
    pub s3_data_type: PrimField<String>,
    #[doc = ""]
    pub s3_uri: PrimField<String>,
}
impl BuildSagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceEl {
    pub fn build(self) -> SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceEl {
        SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceEl {
            compression_type: self.compression_type,
            s3_data_type: self.s3_data_type,
            s3_uri: self.s3_uri,
            model_access_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElRef {
        SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compression_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_data_type` after provisioning.\n"]
    pub fn s3_data_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_data_type", self.base))
    }
    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `model_access_config` after provisioning.\n"]
    pub fn model_access_config(
        &self,
    ) -> ListRef<
        SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_access_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerModelContainerElAdditionalModelDataSourceElDynamic {
    s3_data_source:
        Option<DynamicBlock<SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceEl>>,
}
#[derive(Serialize)]
pub struct SagemakerModelContainerElAdditionalModelDataSourceEl {
    channel_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_data_source: Option<Vec<SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceEl>>,
    dynamic: SagemakerModelContainerElAdditionalModelDataSourceElDynamic,
}
impl SagemakerModelContainerElAdditionalModelDataSourceEl {
    #[doc = "Set the field `s3_data_source`.\n"]
    pub fn set_s3_data_source(
        mut self,
        v: impl Into<
            BlockAssignable<SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_data_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_data_source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerModelContainerElAdditionalModelDataSourceEl {
    type O = BlockAssignable<SagemakerModelContainerElAdditionalModelDataSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelContainerElAdditionalModelDataSourceEl {
    #[doc = ""]
    pub channel_name: PrimField<String>,
}
impl BuildSagemakerModelContainerElAdditionalModelDataSourceEl {
    pub fn build(self) -> SagemakerModelContainerElAdditionalModelDataSourceEl {
        SagemakerModelContainerElAdditionalModelDataSourceEl {
            channel_name: self.channel_name,
            s3_data_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerModelContainerElAdditionalModelDataSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelContainerElAdditionalModelDataSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerModelContainerElAdditionalModelDataSourceElRef {
        SagemakerModelContainerElAdditionalModelDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelContainerElAdditionalModelDataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel_name` after provisioning.\n"]
    pub fn channel_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel_name", self.base))
    }
    #[doc = "Get a reference to the value of field `s3_data_source` after provisioning.\n"]
    pub fn s3_data_source(
        &self,
    ) -> ListRef<SagemakerModelContainerElAdditionalModelDataSourceElS3DataSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_data_source", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl {
    repository_credentials_provider_arn: PrimField<String>,
}
impl SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl {}
impl ToListMappable for SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl {
    type O = BlockAssignable<SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelContainerElImageConfigElRepositoryAuthConfigEl {
    #[doc = ""]
    pub repository_credentials_provider_arn: PrimField<String>,
}
impl BuildSagemakerModelContainerElImageConfigElRepositoryAuthConfigEl {
    pub fn build(self) -> SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl {
        SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl {
            repository_credentials_provider_arn: self.repository_credentials_provider_arn,
        }
    }
}
pub struct SagemakerModelContainerElImageConfigElRepositoryAuthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelContainerElImageConfigElRepositoryAuthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerModelContainerElImageConfigElRepositoryAuthConfigElRef {
        SagemakerModelContainerElImageConfigElRepositoryAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelContainerElImageConfigElRepositoryAuthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repository_credentials_provider_arn` after provisioning.\n"]
    pub fn repository_credentials_provider_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_credentials_provider_arn", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerModelContainerElImageConfigElDynamic {
    repository_auth_config:
        Option<DynamicBlock<SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl>>,
}
#[derive(Serialize)]
pub struct SagemakerModelContainerElImageConfigEl {
    repository_access_mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_auth_config:
        Option<Vec<SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl>>,
    dynamic: SagemakerModelContainerElImageConfigElDynamic,
}
impl SagemakerModelContainerElImageConfigEl {
    #[doc = "Set the field `repository_auth_config`.\n"]
    pub fn set_repository_auth_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerModelContainerElImageConfigElRepositoryAuthConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.repository_auth_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.repository_auth_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerModelContainerElImageConfigEl {
    type O = BlockAssignable<SagemakerModelContainerElImageConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelContainerElImageConfigEl {
    #[doc = ""]
    pub repository_access_mode: PrimField<String>,
}
impl BuildSagemakerModelContainerElImageConfigEl {
    pub fn build(self) -> SagemakerModelContainerElImageConfigEl {
        SagemakerModelContainerElImageConfigEl {
            repository_access_mode: self.repository_access_mode,
            repository_auth_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerModelContainerElImageConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelContainerElImageConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerModelContainerElImageConfigElRef {
        SagemakerModelContainerElImageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelContainerElImageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repository_access_mode` after provisioning.\n"]
    pub fn repository_access_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_access_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `repository_auth_config` after provisioning.\n"]
    pub fn repository_auth_config(
        &self,
    ) -> ListRef<SagemakerModelContainerElImageConfigElRepositoryAuthConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.repository_auth_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerModelContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl {
    accept_eula: PrimField<bool>,
}
impl SagemakerModelContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl {}
impl ToListMappable
    for SagemakerModelContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl
{
    type O = BlockAssignable<
        SagemakerModelContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl {
    #[doc = ""]
    pub accept_eula: PrimField<bool>,
}
impl BuildSagemakerModelContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl {
    pub fn build(
        self,
    ) -> SagemakerModelContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl {
        SagemakerModelContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl {
            accept_eula: self.accept_eula,
        }
    }
}
pub struct SagemakerModelContainerElModelDataSourceElS3DataSourceElModelAccessConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelContainerElModelDataSourceElS3DataSourceElModelAccessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerModelContainerElModelDataSourceElS3DataSourceElModelAccessConfigElRef {
        SagemakerModelContainerElModelDataSourceElS3DataSourceElModelAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelContainerElModelDataSourceElS3DataSourceElModelAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accept_eula` after provisioning.\n"]
    pub fn accept_eula(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.accept_eula", self.base))
    }
}
#[derive(Serialize, Default)]
struct SagemakerModelContainerElModelDataSourceElS3DataSourceElDynamic {
    model_access_config: Option<
        DynamicBlock<SagemakerModelContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct SagemakerModelContainerElModelDataSourceElS3DataSourceEl {
    compression_type: PrimField<String>,
    s3_data_type: PrimField<String>,
    s3_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_access_config:
        Option<Vec<SagemakerModelContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl>>,
    dynamic: SagemakerModelContainerElModelDataSourceElS3DataSourceElDynamic,
}
impl SagemakerModelContainerElModelDataSourceElS3DataSourceEl {
    #[doc = "Set the field `model_access_config`.\n"]
    pub fn set_model_access_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerModelContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.model_access_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.model_access_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerModelContainerElModelDataSourceElS3DataSourceEl {
    type O = BlockAssignable<SagemakerModelContainerElModelDataSourceElS3DataSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelContainerElModelDataSourceElS3DataSourceEl {
    #[doc = ""]
    pub compression_type: PrimField<String>,
    #[doc = ""]
    pub s3_data_type: PrimField<String>,
    #[doc = ""]
    pub s3_uri: PrimField<String>,
}
impl BuildSagemakerModelContainerElModelDataSourceElS3DataSourceEl {
    pub fn build(self) -> SagemakerModelContainerElModelDataSourceElS3DataSourceEl {
        SagemakerModelContainerElModelDataSourceElS3DataSourceEl {
            compression_type: self.compression_type,
            s3_data_type: self.s3_data_type,
            s3_uri: self.s3_uri,
            model_access_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerModelContainerElModelDataSourceElS3DataSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelContainerElModelDataSourceElS3DataSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerModelContainerElModelDataSourceElS3DataSourceElRef {
        SagemakerModelContainerElModelDataSourceElS3DataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelContainerElModelDataSourceElS3DataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compression_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_data_type` after provisioning.\n"]
    pub fn s3_data_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_data_type", self.base))
    }
    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `model_access_config` after provisioning.\n"]
    pub fn model_access_config(
        &self,
    ) -> ListRef<SagemakerModelContainerElModelDataSourceElS3DataSourceElModelAccessConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_access_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerModelContainerElModelDataSourceElDynamic {
    s3_data_source: Option<DynamicBlock<SagemakerModelContainerElModelDataSourceElS3DataSourceEl>>,
}
#[derive(Serialize)]
pub struct SagemakerModelContainerElModelDataSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_data_source: Option<Vec<SagemakerModelContainerElModelDataSourceElS3DataSourceEl>>,
    dynamic: SagemakerModelContainerElModelDataSourceElDynamic,
}
impl SagemakerModelContainerElModelDataSourceEl {
    #[doc = "Set the field `s3_data_source`.\n"]
    pub fn set_s3_data_source(
        mut self,
        v: impl Into<BlockAssignable<SagemakerModelContainerElModelDataSourceElS3DataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_data_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_data_source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerModelContainerElModelDataSourceEl {
    type O = BlockAssignable<SagemakerModelContainerElModelDataSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelContainerElModelDataSourceEl {}
impl BuildSagemakerModelContainerElModelDataSourceEl {
    pub fn build(self) -> SagemakerModelContainerElModelDataSourceEl {
        SagemakerModelContainerElModelDataSourceEl {
            s3_data_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerModelContainerElModelDataSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelContainerElModelDataSourceElRef {
    fn new(shared: StackShared, base: String) -> SagemakerModelContainerElModelDataSourceElRef {
        SagemakerModelContainerElModelDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelContainerElModelDataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_data_source` after provisioning.\n"]
    pub fn s3_data_source(
        &self,
    ) -> ListRef<SagemakerModelContainerElModelDataSourceElS3DataSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_data_source", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerModelContainerElMultiModelConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    model_cache_setting: Option<PrimField<String>>,
}
impl SagemakerModelContainerElMultiModelConfigEl {
    #[doc = "Set the field `model_cache_setting`.\n"]
    pub fn set_model_cache_setting(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_cache_setting = Some(v.into());
        self
    }
}
impl ToListMappable for SagemakerModelContainerElMultiModelConfigEl {
    type O = BlockAssignable<SagemakerModelContainerElMultiModelConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelContainerElMultiModelConfigEl {}
impl BuildSagemakerModelContainerElMultiModelConfigEl {
    pub fn build(self) -> SagemakerModelContainerElMultiModelConfigEl {
        SagemakerModelContainerElMultiModelConfigEl {
            model_cache_setting: core::default::Default::default(),
        }
    }
}
pub struct SagemakerModelContainerElMultiModelConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelContainerElMultiModelConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerModelContainerElMultiModelConfigElRef {
        SagemakerModelContainerElMultiModelConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelContainerElMultiModelConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `model_cache_setting` after provisioning.\n"]
    pub fn model_cache_setting(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_cache_setting", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerModelContainerElDynamic {
    additional_model_data_source:
        Option<DynamicBlock<SagemakerModelContainerElAdditionalModelDataSourceEl>>,
    image_config: Option<DynamicBlock<SagemakerModelContainerElImageConfigEl>>,
    model_data_source: Option<DynamicBlock<SagemakerModelContainerElModelDataSourceEl>>,
    multi_model_config: Option<DynamicBlock<SagemakerModelContainerElMultiModelConfigEl>>,
}
#[derive(Serialize)]
pub struct SagemakerModelContainerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_hostname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inference_specification_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_data_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_package_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_model_data_source: Option<Vec<SagemakerModelContainerElAdditionalModelDataSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_config: Option<Vec<SagemakerModelContainerElImageConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_data_source: Option<Vec<SagemakerModelContainerElModelDataSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_model_config: Option<Vec<SagemakerModelContainerElMultiModelConfigEl>>,
    dynamic: SagemakerModelContainerElDynamic,
}
impl SagemakerModelContainerEl {
    #[doc = "Set the field `container_hostname`.\n"]
    pub fn set_container_hostname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_hostname = Some(v.into());
        self
    }
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.environment = Some(v.into());
        self
    }
    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image = Some(v.into());
        self
    }
    #[doc = "Set the field `inference_specification_name`.\n"]
    pub fn set_inference_specification_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.inference_specification_name = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `model_data_url`.\n"]
    pub fn set_model_data_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_data_url = Some(v.into());
        self
    }
    #[doc = "Set the field `model_package_name`.\n"]
    pub fn set_model_package_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_package_name = Some(v.into());
        self
    }
    #[doc = "Set the field `additional_model_data_source`.\n"]
    pub fn set_additional_model_data_source(
        mut self,
        v: impl Into<BlockAssignable<SagemakerModelContainerElAdditionalModelDataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.additional_model_data_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_model_data_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `image_config`.\n"]
    pub fn set_image_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerModelContainerElImageConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.image_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.image_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `model_data_source`.\n"]
    pub fn set_model_data_source(
        mut self,
        v: impl Into<BlockAssignable<SagemakerModelContainerElModelDataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.model_data_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.model_data_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `multi_model_config`.\n"]
    pub fn set_multi_model_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerModelContainerElMultiModelConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.multi_model_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.multi_model_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerModelContainerEl {
    type O = BlockAssignable<SagemakerModelContainerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelContainerEl {}
impl BuildSagemakerModelContainerEl {
    pub fn build(self) -> SagemakerModelContainerEl {
        SagemakerModelContainerEl {
            container_hostname: core::default::Default::default(),
            environment: core::default::Default::default(),
            image: core::default::Default::default(),
            inference_specification_name: core::default::Default::default(),
            mode: core::default::Default::default(),
            model_data_url: core::default::Default::default(),
            model_package_name: core::default::Default::default(),
            additional_model_data_source: core::default::Default::default(),
            image_config: core::default::Default::default(),
            model_data_source: core::default::Default::default(),
            multi_model_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerModelContainerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelContainerElRef {
    fn new(shared: StackShared, base: String) -> SagemakerModelContainerElRef {
        SagemakerModelContainerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelContainerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_hostname` after provisioning.\n"]
    pub fn container_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_hostname", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `inference_specification_name` after provisioning.\n"]
    pub fn inference_specification_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inference_specification_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `model_data_url` after provisioning.\n"]
    pub fn model_data_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_data_url", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `model_package_name` after provisioning.\n"]
    pub fn model_package_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_package_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `additional_model_data_source` after provisioning.\n"]
    pub fn additional_model_data_source(
        &self,
    ) -> ListRef<SagemakerModelContainerElAdditionalModelDataSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_model_data_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_config` after provisioning.\n"]
    pub fn image_config(&self) -> ListRef<SagemakerModelContainerElImageConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_config", self.base))
    }
    #[doc = "Get a reference to the value of field `model_data_source` after provisioning.\n"]
    pub fn model_data_source(&self) -> ListRef<SagemakerModelContainerElModelDataSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_data_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `multi_model_config` after provisioning.\n"]
    pub fn multi_model_config(&self) -> ListRef<SagemakerModelContainerElMultiModelConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multi_model_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerModelInferenceExecutionConfigEl {
    mode: PrimField<String>,
}
impl SagemakerModelInferenceExecutionConfigEl {}
impl ToListMappable for SagemakerModelInferenceExecutionConfigEl {
    type O = BlockAssignable<SagemakerModelInferenceExecutionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelInferenceExecutionConfigEl {
    #[doc = ""]
    pub mode: PrimField<String>,
}
impl BuildSagemakerModelInferenceExecutionConfigEl {
    pub fn build(self) -> SagemakerModelInferenceExecutionConfigEl {
        SagemakerModelInferenceExecutionConfigEl { mode: self.mode }
    }
}
pub struct SagemakerModelInferenceExecutionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelInferenceExecutionConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerModelInferenceExecutionConfigElRef {
        SagemakerModelInferenceExecutionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelInferenceExecutionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}
#[derive(Serialize)]
pub struct SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl
{
    accept_eula: PrimField<bool>,
}
impl SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl {}
impl ToListMappable
    for SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl
{
    type O = BlockAssignable < SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl
{
    #[doc = ""]
    pub accept_eula: PrimField<bool>,
}
impl BuildSagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl { pub fn build (self) -> SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl { SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl { accept_eula : self . accept_eula , } } }
pub struct SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigElRef { fn new (shared : StackShared , base : String) -> SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigElRef { SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigElRef { shared : shared , base : base . to_string () , } } }
impl
    SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accept_eula` after provisioning.\n"]
    pub fn accept_eula(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.accept_eula", self.base))
    }
}
#[derive(Serialize, Default)]
struct SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElDynamic { model_access_config : Option < DynamicBlock < SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl >> , }
#[derive(Serialize)]
pub struct SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceEl { compression_type : PrimField < String > , s3_data_type : PrimField < String > , s3_uri : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] model_access_config : Option < Vec < SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl > > , dynamic : SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElDynamic , }
impl SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceEl {
    #[doc = "Set the field `model_access_config`.\n"]
    pub fn set_model_access_config(
        mut self,
        v : impl Into < BlockAssignable < SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.model_access_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.model_access_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceEl {
    type O =
        BlockAssignable<SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceEl {
    #[doc = ""]
    pub compression_type: PrimField<String>,
    #[doc = ""]
    pub s3_data_type: PrimField<String>,
    #[doc = ""]
    pub s3_uri: PrimField<String>,
}
impl BuildSagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceEl {
    pub fn build(
        self,
    ) -> SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceEl {
        SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceEl {
            compression_type: self.compression_type,
            s3_data_type: self.s3_data_type,
            s3_uri: self.s3_uri,
            model_access_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElRef {
        SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compression_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_data_type` after provisioning.\n"]
    pub fn s3_data_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_data_type", self.base))
    }
    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `model_access_config` after provisioning.\n"]    pub fn model_access_config (& self) -> ListRef < SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElModelAccessConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_access_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerModelPrimaryContainerElAdditionalModelDataSourceElDynamic {
    s3_data_source: Option<
        DynamicBlock<SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceEl>,
    >,
}
#[derive(Serialize)]
pub struct SagemakerModelPrimaryContainerElAdditionalModelDataSourceEl {
    channel_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_data_source:
        Option<Vec<SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceEl>>,
    dynamic: SagemakerModelPrimaryContainerElAdditionalModelDataSourceElDynamic,
}
impl SagemakerModelPrimaryContainerElAdditionalModelDataSourceEl {
    #[doc = "Set the field `s3_data_source`.\n"]
    pub fn set_s3_data_source(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_data_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_data_source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerModelPrimaryContainerElAdditionalModelDataSourceEl {
    type O = BlockAssignable<SagemakerModelPrimaryContainerElAdditionalModelDataSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelPrimaryContainerElAdditionalModelDataSourceEl {
    #[doc = ""]
    pub channel_name: PrimField<String>,
}
impl BuildSagemakerModelPrimaryContainerElAdditionalModelDataSourceEl {
    pub fn build(self) -> SagemakerModelPrimaryContainerElAdditionalModelDataSourceEl {
        SagemakerModelPrimaryContainerElAdditionalModelDataSourceEl {
            channel_name: self.channel_name,
            s3_data_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerModelPrimaryContainerElAdditionalModelDataSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelPrimaryContainerElAdditionalModelDataSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerModelPrimaryContainerElAdditionalModelDataSourceElRef {
        SagemakerModelPrimaryContainerElAdditionalModelDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelPrimaryContainerElAdditionalModelDataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel_name` after provisioning.\n"]
    pub fn channel_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel_name", self.base))
    }
    #[doc = "Get a reference to the value of field `s3_data_source` after provisioning.\n"]
    pub fn s3_data_source(
        &self,
    ) -> ListRef<SagemakerModelPrimaryContainerElAdditionalModelDataSourceElS3DataSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_data_source", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl {
    repository_credentials_provider_arn: PrimField<String>,
}
impl SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl {}
impl ToListMappable for SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl {
    type O = BlockAssignable<SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl {
    #[doc = ""]
    pub repository_credentials_provider_arn: PrimField<String>,
}
impl BuildSagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl {
    pub fn build(self) -> SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl {
        SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl {
            repository_credentials_provider_arn: self.repository_credentials_provider_arn,
        }
    }
}
pub struct SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigElRef {
        SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repository_credentials_provider_arn` after provisioning.\n"]
    pub fn repository_credentials_provider_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_credentials_provider_arn", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerModelPrimaryContainerElImageConfigElDynamic {
    repository_auth_config:
        Option<DynamicBlock<SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl>>,
}
#[derive(Serialize)]
pub struct SagemakerModelPrimaryContainerElImageConfigEl {
    repository_access_mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_auth_config:
        Option<Vec<SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl>>,
    dynamic: SagemakerModelPrimaryContainerElImageConfigElDynamic,
}
impl SagemakerModelPrimaryContainerElImageConfigEl {
    #[doc = "Set the field `repository_auth_config`.\n"]
    pub fn set_repository_auth_config(
        mut self,
        v: impl Into<
            BlockAssignable<SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.repository_auth_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.repository_auth_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerModelPrimaryContainerElImageConfigEl {
    type O = BlockAssignable<SagemakerModelPrimaryContainerElImageConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelPrimaryContainerElImageConfigEl {
    #[doc = ""]
    pub repository_access_mode: PrimField<String>,
}
impl BuildSagemakerModelPrimaryContainerElImageConfigEl {
    pub fn build(self) -> SagemakerModelPrimaryContainerElImageConfigEl {
        SagemakerModelPrimaryContainerElImageConfigEl {
            repository_access_mode: self.repository_access_mode,
            repository_auth_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerModelPrimaryContainerElImageConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelPrimaryContainerElImageConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerModelPrimaryContainerElImageConfigElRef {
        SagemakerModelPrimaryContainerElImageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelPrimaryContainerElImageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repository_access_mode` after provisioning.\n"]
    pub fn repository_access_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_access_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `repository_auth_config` after provisioning.\n"]
    pub fn repository_auth_config(
        &self,
    ) -> ListRef<SagemakerModelPrimaryContainerElImageConfigElRepositoryAuthConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.repository_auth_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl {
    accept_eula: PrimField<bool>,
}
impl SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl {}
impl ToListMappable
    for SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl
{
    type O = BlockAssignable<
        SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl {
    #[doc = ""]
    pub accept_eula: PrimField<bool>,
}
impl BuildSagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl {
    pub fn build(
        self,
    ) -> SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl {
        SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl {
            accept_eula: self.accept_eula,
        }
    }
}
pub struct SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElModelAccessConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElModelAccessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElModelAccessConfigElRef {
        SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElModelAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElModelAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accept_eula` after provisioning.\n"]
    pub fn accept_eula(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.accept_eula", self.base))
    }
}
#[derive(Serialize, Default)]
struct SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElDynamic {
    model_access_config: Option<
        DynamicBlock<
            SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceEl {
    compression_type: PrimField<String>,
    s3_data_type: PrimField<String>,
    s3_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_access_config: Option<
        Vec<SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl>,
    >,
    dynamic: SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElDynamic,
}
impl SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceEl {
    #[doc = "Set the field `model_access_config`.\n"]
    pub fn set_model_access_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElModelAccessConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.model_access_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.model_access_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceEl {
    type O = BlockAssignable<SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceEl {
    #[doc = ""]
    pub compression_type: PrimField<String>,
    #[doc = ""]
    pub s3_data_type: PrimField<String>,
    #[doc = ""]
    pub s3_uri: PrimField<String>,
}
impl BuildSagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceEl {
    pub fn build(self) -> SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceEl {
        SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceEl {
            compression_type: self.compression_type,
            s3_data_type: self.s3_data_type,
            s3_uri: self.s3_uri,
            model_access_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElRef {
        SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compression_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_data_type` after provisioning.\n"]
    pub fn s3_data_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_data_type", self.base))
    }
    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\n"]
    pub fn s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `model_access_config` after provisioning.\n"]
    pub fn model_access_config(
        &self,
    ) -> ListRef<
        SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElModelAccessConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_access_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerModelPrimaryContainerElModelDataSourceElDynamic {
    s3_data_source:
        Option<DynamicBlock<SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceEl>>,
}
#[derive(Serialize)]
pub struct SagemakerModelPrimaryContainerElModelDataSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_data_source: Option<Vec<SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceEl>>,
    dynamic: SagemakerModelPrimaryContainerElModelDataSourceElDynamic,
}
impl SagemakerModelPrimaryContainerElModelDataSourceEl {
    #[doc = "Set the field `s3_data_source`.\n"]
    pub fn set_s3_data_source(
        mut self,
        v: impl Into<BlockAssignable<SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_data_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_data_source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerModelPrimaryContainerElModelDataSourceEl {
    type O = BlockAssignable<SagemakerModelPrimaryContainerElModelDataSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelPrimaryContainerElModelDataSourceEl {}
impl BuildSagemakerModelPrimaryContainerElModelDataSourceEl {
    pub fn build(self) -> SagemakerModelPrimaryContainerElModelDataSourceEl {
        SagemakerModelPrimaryContainerElModelDataSourceEl {
            s3_data_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerModelPrimaryContainerElModelDataSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelPrimaryContainerElModelDataSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerModelPrimaryContainerElModelDataSourceElRef {
        SagemakerModelPrimaryContainerElModelDataSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelPrimaryContainerElModelDataSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_data_source` after provisioning.\n"]
    pub fn s3_data_source(
        &self,
    ) -> ListRef<SagemakerModelPrimaryContainerElModelDataSourceElS3DataSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_data_source", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerModelPrimaryContainerElMultiModelConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    model_cache_setting: Option<PrimField<String>>,
}
impl SagemakerModelPrimaryContainerElMultiModelConfigEl {
    #[doc = "Set the field `model_cache_setting`.\n"]
    pub fn set_model_cache_setting(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_cache_setting = Some(v.into());
        self
    }
}
impl ToListMappable for SagemakerModelPrimaryContainerElMultiModelConfigEl {
    type O = BlockAssignable<SagemakerModelPrimaryContainerElMultiModelConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelPrimaryContainerElMultiModelConfigEl {}
impl BuildSagemakerModelPrimaryContainerElMultiModelConfigEl {
    pub fn build(self) -> SagemakerModelPrimaryContainerElMultiModelConfigEl {
        SagemakerModelPrimaryContainerElMultiModelConfigEl {
            model_cache_setting: core::default::Default::default(),
        }
    }
}
pub struct SagemakerModelPrimaryContainerElMultiModelConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelPrimaryContainerElMultiModelConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerModelPrimaryContainerElMultiModelConfigElRef {
        SagemakerModelPrimaryContainerElMultiModelConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelPrimaryContainerElMultiModelConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `model_cache_setting` after provisioning.\n"]
    pub fn model_cache_setting(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_cache_setting", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SagemakerModelPrimaryContainerElDynamic {
    additional_model_data_source:
        Option<DynamicBlock<SagemakerModelPrimaryContainerElAdditionalModelDataSourceEl>>,
    image_config: Option<DynamicBlock<SagemakerModelPrimaryContainerElImageConfigEl>>,
    model_data_source: Option<DynamicBlock<SagemakerModelPrimaryContainerElModelDataSourceEl>>,
    multi_model_config: Option<DynamicBlock<SagemakerModelPrimaryContainerElMultiModelConfigEl>>,
}
#[derive(Serialize)]
pub struct SagemakerModelPrimaryContainerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_hostname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inference_specification_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_data_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_package_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_model_data_source:
        Option<Vec<SagemakerModelPrimaryContainerElAdditionalModelDataSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_config: Option<Vec<SagemakerModelPrimaryContainerElImageConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_data_source: Option<Vec<SagemakerModelPrimaryContainerElModelDataSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_model_config: Option<Vec<SagemakerModelPrimaryContainerElMultiModelConfigEl>>,
    dynamic: SagemakerModelPrimaryContainerElDynamic,
}
impl SagemakerModelPrimaryContainerEl {
    #[doc = "Set the field `container_hostname`.\n"]
    pub fn set_container_hostname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_hostname = Some(v.into());
        self
    }
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.environment = Some(v.into());
        self
    }
    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image = Some(v.into());
        self
    }
    #[doc = "Set the field `inference_specification_name`.\n"]
    pub fn set_inference_specification_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.inference_specification_name = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `model_data_url`.\n"]
    pub fn set_model_data_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_data_url = Some(v.into());
        self
    }
    #[doc = "Set the field `model_package_name`.\n"]
    pub fn set_model_package_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model_package_name = Some(v.into());
        self
    }
    #[doc = "Set the field `additional_model_data_source`.\n"]
    pub fn set_additional_model_data_source(
        mut self,
        v: impl Into<BlockAssignable<SagemakerModelPrimaryContainerElAdditionalModelDataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.additional_model_data_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_model_data_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `image_config`.\n"]
    pub fn set_image_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerModelPrimaryContainerElImageConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.image_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.image_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `model_data_source`.\n"]
    pub fn set_model_data_source(
        mut self,
        v: impl Into<BlockAssignable<SagemakerModelPrimaryContainerElModelDataSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.model_data_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.model_data_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `multi_model_config`.\n"]
    pub fn set_multi_model_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerModelPrimaryContainerElMultiModelConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.multi_model_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.multi_model_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerModelPrimaryContainerEl {
    type O = BlockAssignable<SagemakerModelPrimaryContainerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelPrimaryContainerEl {}
impl BuildSagemakerModelPrimaryContainerEl {
    pub fn build(self) -> SagemakerModelPrimaryContainerEl {
        SagemakerModelPrimaryContainerEl {
            container_hostname: core::default::Default::default(),
            environment: core::default::Default::default(),
            image: core::default::Default::default(),
            inference_specification_name: core::default::Default::default(),
            mode: core::default::Default::default(),
            model_data_url: core::default::Default::default(),
            model_package_name: core::default::Default::default(),
            additional_model_data_source: core::default::Default::default(),
            image_config: core::default::Default::default(),
            model_data_source: core::default::Default::default(),
            multi_model_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerModelPrimaryContainerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelPrimaryContainerElRef {
    fn new(shared: StackShared, base: String) -> SagemakerModelPrimaryContainerElRef {
        SagemakerModelPrimaryContainerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelPrimaryContainerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_hostname` after provisioning.\n"]
    pub fn container_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_hostname", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `inference_specification_name` after provisioning.\n"]
    pub fn inference_specification_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inference_specification_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `model_data_url` after provisioning.\n"]
    pub fn model_data_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_data_url", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `model_package_name` after provisioning.\n"]
    pub fn model_package_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_package_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `additional_model_data_source` after provisioning.\n"]
    pub fn additional_model_data_source(
        &self,
    ) -> ListRef<SagemakerModelPrimaryContainerElAdditionalModelDataSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_model_data_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_config` after provisioning.\n"]
    pub fn image_config(&self) -> ListRef<SagemakerModelPrimaryContainerElImageConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_config", self.base))
    }
    #[doc = "Get a reference to the value of field `model_data_source` after provisioning.\n"]
    pub fn model_data_source(
        &self,
    ) -> ListRef<SagemakerModelPrimaryContainerElModelDataSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_data_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `multi_model_config` after provisioning.\n"]
    pub fn multi_model_config(
        &self,
    ) -> ListRef<SagemakerModelPrimaryContainerElMultiModelConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multi_model_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerModelVpcConfigEl {
    security_group_ids: SetField<PrimField<String>>,
    subnets: SetField<PrimField<String>>,
}
impl SagemakerModelVpcConfigEl {}
impl ToListMappable for SagemakerModelVpcConfigEl {
    type O = BlockAssignable<SagemakerModelVpcConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerModelVpcConfigEl {
    #[doc = ""]
    pub security_group_ids: SetField<PrimField<String>>,
    #[doc = ""]
    pub subnets: SetField<PrimField<String>>,
}
impl BuildSagemakerModelVpcConfigEl {
    pub fn build(self) -> SagemakerModelVpcConfigEl {
        SagemakerModelVpcConfigEl {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
        }
    }
}
pub struct SagemakerModelVpcConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerModelVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerModelVpcConfigElRef {
        SagemakerModelVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerModelVpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}
#[derive(Serialize, Default)]
struct SagemakerModelDynamic {
    container: Option<DynamicBlock<SagemakerModelContainerEl>>,
    inference_execution_config: Option<DynamicBlock<SagemakerModelInferenceExecutionConfigEl>>,
    primary_container: Option<DynamicBlock<SagemakerModelPrimaryContainerEl>>,
    vpc_config: Option<DynamicBlock<SagemakerModelVpcConfigEl>>,
}
