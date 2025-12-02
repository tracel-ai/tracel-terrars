use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BatchComputeEnvironmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_resources: Option<Vec<BatchComputeEnvironmentComputeResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eks_configuration: Option<Vec<BatchComputeEnvironmentEksConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_policy: Option<Vec<BatchComputeEnvironmentUpdatePolicyEl>>,
    dynamic: BatchComputeEnvironmentDynamic,
}
struct BatchComputeEnvironment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BatchComputeEnvironmentData>,
}
#[derive(Clone)]
pub struct BatchComputeEnvironment(Rc<BatchComputeEnvironment_>);
impl BatchComputeEnvironment {
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
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `service_role`.\n"]
    pub fn set_service_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_role = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state = Some(v.into());
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
    #[doc = "Set the field `compute_resources`.\n"]
    pub fn set_compute_resources(
        self,
        v: impl Into<BlockAssignable<BatchComputeEnvironmentComputeResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.compute_resources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `eks_configuration`.\n"]
    pub fn set_eks_configuration(
        self,
        v: impl Into<BlockAssignable<BatchComputeEnvironmentEksConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().eks_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.eks_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `update_policy`.\n"]
    pub fn set_update_policy(
        self,
        v: impl Into<BlockAssignable<BatchComputeEnvironmentUpdatePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().update_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.update_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ecs_cluster_arn` after provisioning.\n"]
    pub fn ecs_cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ecs_cluster_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_reason", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compute_resources` after provisioning.\n"]
    pub fn compute_resources(&self) -> ListRef<BatchComputeEnvironmentComputeResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `eks_configuration` after provisioning.\n"]
    pub fn eks_configuration(&self) -> ListRef<BatchComputeEnvironmentEksConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eks_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_policy` after provisioning.\n"]
    pub fn update_policy(&self) -> ListRef<BatchComputeEnvironmentUpdatePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.update_policy", self.extract_ref()),
        )
    }
}
impl Referable for BatchComputeEnvironment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BatchComputeEnvironment {}
impl ToListMappable for BatchComputeEnvironment {
    type O = ListRef<BatchComputeEnvironmentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BatchComputeEnvironment_ {
    fn extract_resource_type(&self) -> String {
        "aws_batch_compute_environment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBatchComputeEnvironment {
    pub tf_id: String,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBatchComputeEnvironment {
    pub fn build(self, stack: &mut Stack) -> BatchComputeEnvironment {
        let out = BatchComputeEnvironment(Rc::new(BatchComputeEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BatchComputeEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                region: core::default::Default::default(),
                service_role: core::default::Default::default(),
                state: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                compute_resources: core::default::Default::default(),
                eks_configuration: core::default::Default::default(),
                update_policy: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BatchComputeEnvironmentRef {
    shared: StackShared,
    base: String,
}
impl Ref for BatchComputeEnvironmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BatchComputeEnvironmentRef {
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
    #[doc = "Get a reference to the value of field `ecs_cluster_arn` after provisioning.\n"]
    pub fn ecs_cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ecs_cluster_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_reason", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compute_resources` after provisioning.\n"]
    pub fn compute_resources(&self) -> ListRef<BatchComputeEnvironmentComputeResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `eks_configuration` after provisioning.\n"]
    pub fn eks_configuration(&self) -> ListRef<BatchComputeEnvironmentEksConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eks_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_policy` after provisioning.\n"]
    pub fn update_policy(&self) -> ListRef<BatchComputeEnvironmentUpdatePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.update_policy", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_id_override: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_kubernetes_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_type: Option<PrimField<String>>,
}
impl BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl {
    #[doc = "Set the field `image_id_override`.\n"]
    pub fn set_image_id_override(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_id_override = Some(v.into());
        self
    }
    #[doc = "Set the field `image_kubernetes_version`.\n"]
    pub fn set_image_kubernetes_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_kubernetes_version = Some(v.into());
        self
    }
    #[doc = "Set the field `image_type`.\n"]
    pub fn set_image_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_type = Some(v.into());
        self
    }
}
impl ToListMappable for BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl {
    type O = BlockAssignable<BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl {}
impl BuildBatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl {
    pub fn build(self) -> BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl {
        BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl {
            image_id_override: core::default::Default::default(),
            image_kubernetes_version: core::default::Default::default(),
            image_type: core::default::Default::default(),
        }
    }
}
pub struct BatchComputeEnvironmentComputeResourcesElEc2ConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BatchComputeEnvironmentComputeResourcesElEc2ConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchComputeEnvironmentComputeResourcesElEc2ConfigurationElRef {
        BatchComputeEnvironmentComputeResourcesElEc2ConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BatchComputeEnvironmentComputeResourcesElEc2ConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `image_id_override` after provisioning.\n"]
    pub fn image_id_override(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_id_override", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_kubernetes_version` after provisioning.\n"]
    pub fn image_kubernetes_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_kubernetes_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_type` after provisioning.\n"]
    pub fn image_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_type", self.base))
    }
}
#[derive(Serialize)]
pub struct BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl {
    #[doc = "Set the field `launch_template_id`.\n"]
    pub fn set_launch_template_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.launch_template_id = Some(v.into());
        self
    }
    #[doc = "Set the field `launch_template_name`.\n"]
    pub fn set_launch_template_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.launch_template_name = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl {
    type O = BlockAssignable<BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBatchComputeEnvironmentComputeResourcesElLaunchTemplateEl {}
impl BuildBatchComputeEnvironmentComputeResourcesElLaunchTemplateEl {
    pub fn build(self) -> BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl {
        BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl {
            launch_template_id: core::default::Default::default(),
            launch_template_name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct BatchComputeEnvironmentComputeResourcesElLaunchTemplateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BatchComputeEnvironmentComputeResourcesElLaunchTemplateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BatchComputeEnvironmentComputeResourcesElLaunchTemplateElRef {
        BatchComputeEnvironmentComputeResourcesElLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BatchComputeEnvironmentComputeResourcesElLaunchTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `launch_template_id` after provisioning.\n"]
    pub fn launch_template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_template_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `launch_template_name` after provisioning.\n"]
    pub fn launch_template_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.launch_template_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize, Default)]
struct BatchComputeEnvironmentComputeResourcesElDynamic {
    ec2_configuration:
        Option<DynamicBlock<BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl>>,
    launch_template:
        Option<DynamicBlock<BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl>>,
}
#[derive(Serialize)]
pub struct BatchComputeEnvironmentComputeResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bid_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_vcpus: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ec2_key_pair: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<SetField<PrimField<String>>>,
    max_vcpus: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_vcpus: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_iam_fleet_role: Option<PrimField<String>>,
    subnets: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ec2_configuration: Option<Vec<BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template: Option<Vec<BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl>>,
    dynamic: BatchComputeEnvironmentComputeResourcesElDynamic,
}
impl BatchComputeEnvironmentComputeResourcesEl {
    #[doc = "Set the field `allocation_strategy`.\n"]
    pub fn set_allocation_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allocation_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `bid_percentage`.\n"]
    pub fn set_bid_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bid_percentage = Some(v.into());
        self
    }
    #[doc = "Set the field `desired_vcpus`.\n"]
    pub fn set_desired_vcpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.desired_vcpus = Some(v.into());
        self
    }
    #[doc = "Set the field `ec2_key_pair`.\n"]
    pub fn set_ec2_key_pair(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ec2_key_pair = Some(v.into());
        self
    }
    #[doc = "Set the field `image_id`.\n"]
    pub fn set_image_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_id = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_role`.\n"]
    pub fn set_instance_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_role = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `min_vcpus`.\n"]
    pub fn set_min_vcpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_vcpus = Some(v.into());
        self
    }
    #[doc = "Set the field `placement_group`.\n"]
    pub fn set_placement_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.placement_group = Some(v.into());
        self
    }
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `spot_iam_fleet_role`.\n"]
    pub fn set_spot_iam_fleet_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.spot_iam_fleet_role = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `ec2_configuration`.\n"]
    pub fn set_ec2_configuration(
        mut self,
        v: impl Into<BlockAssignable<BatchComputeEnvironmentComputeResourcesElEc2ConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ec2_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ec2_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(
        mut self,
        v: impl Into<BlockAssignable<BatchComputeEnvironmentComputeResourcesElLaunchTemplateEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.launch_template = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_template = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BatchComputeEnvironmentComputeResourcesEl {
    type O = BlockAssignable<BatchComputeEnvironmentComputeResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBatchComputeEnvironmentComputeResourcesEl {
    #[doc = ""]
    pub max_vcpus: PrimField<f64>,
    #[doc = ""]
    pub subnets: SetField<PrimField<String>>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildBatchComputeEnvironmentComputeResourcesEl {
    pub fn build(self) -> BatchComputeEnvironmentComputeResourcesEl {
        BatchComputeEnvironmentComputeResourcesEl {
            allocation_strategy: core::default::Default::default(),
            bid_percentage: core::default::Default::default(),
            desired_vcpus: core::default::Default::default(),
            ec2_key_pair: core::default::Default::default(),
            image_id: core::default::Default::default(),
            instance_role: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            max_vcpus: self.max_vcpus,
            min_vcpus: core::default::Default::default(),
            placement_group: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            spot_iam_fleet_role: core::default::Default::default(),
            subnets: self.subnets,
            tags: core::default::Default::default(),
            type_: self.type_,
            ec2_configuration: core::default::Default::default(),
            launch_template: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BatchComputeEnvironmentComputeResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BatchComputeEnvironmentComputeResourcesElRef {
    fn new(shared: StackShared, base: String) -> BatchComputeEnvironmentComputeResourcesElRef {
        BatchComputeEnvironmentComputeResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BatchComputeEnvironmentComputeResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocation_strategy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bid_percentage` after provisioning.\n"]
    pub fn bid_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bid_percentage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `desired_vcpus` after provisioning.\n"]
    pub fn desired_vcpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_vcpus", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ec2_key_pair` after provisioning.\n"]
    pub fn ec2_key_pair(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ec2_key_pair", self.base))
    }
    #[doc = "Get a reference to the value of field `image_id` after provisioning.\n"]
    pub fn image_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_id", self.base))
    }
    #[doc = "Get a reference to the value of field `instance_role` after provisioning.\n"]
    pub fn instance_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_role", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_vcpus` after provisioning.\n"]
    pub fn max_vcpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_vcpus", self.base))
    }
    #[doc = "Get a reference to the value of field `min_vcpus` after provisioning.\n"]
    pub fn min_vcpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_vcpus", self.base))
    }
    #[doc = "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.placement_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `spot_iam_fleet_role` after provisioning.\n"]
    pub fn spot_iam_fleet_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_iam_fleet_role", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `ec2_configuration` after provisioning.\n"]
    pub fn ec2_configuration(
        &self,
    ) -> ListRef<BatchComputeEnvironmentComputeResourcesElEc2ConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ec2_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(
        &self,
    ) -> ListRef<BatchComputeEnvironmentComputeResourcesElLaunchTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BatchComputeEnvironmentEksConfigurationEl {
    eks_cluster_arn: PrimField<String>,
    kubernetes_namespace: PrimField<String>,
}
impl BatchComputeEnvironmentEksConfigurationEl {}
impl ToListMappable for BatchComputeEnvironmentEksConfigurationEl {
    type O = BlockAssignable<BatchComputeEnvironmentEksConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBatchComputeEnvironmentEksConfigurationEl {
    #[doc = ""]
    pub eks_cluster_arn: PrimField<String>,
    #[doc = ""]
    pub kubernetes_namespace: PrimField<String>,
}
impl BuildBatchComputeEnvironmentEksConfigurationEl {
    pub fn build(self) -> BatchComputeEnvironmentEksConfigurationEl {
        BatchComputeEnvironmentEksConfigurationEl {
            eks_cluster_arn: self.eks_cluster_arn,
            kubernetes_namespace: self.kubernetes_namespace,
        }
    }
}
pub struct BatchComputeEnvironmentEksConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BatchComputeEnvironmentEksConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BatchComputeEnvironmentEksConfigurationElRef {
        BatchComputeEnvironmentEksConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BatchComputeEnvironmentEksConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `eks_cluster_arn` after provisioning.\n"]
    pub fn eks_cluster_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.eks_cluster_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kubernetes_namespace` after provisioning.\n"]
    pub fn kubernetes_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kubernetes_namespace", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BatchComputeEnvironmentUpdatePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    job_execution_timeout_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terminate_jobs_on_update: Option<PrimField<bool>>,
}
impl BatchComputeEnvironmentUpdatePolicyEl {
    #[doc = "Set the field `job_execution_timeout_minutes`.\n"]
    pub fn set_job_execution_timeout_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.job_execution_timeout_minutes = Some(v.into());
        self
    }
    #[doc = "Set the field `terminate_jobs_on_update`.\n"]
    pub fn set_terminate_jobs_on_update(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.terminate_jobs_on_update = Some(v.into());
        self
    }
}
impl ToListMappable for BatchComputeEnvironmentUpdatePolicyEl {
    type O = BlockAssignable<BatchComputeEnvironmentUpdatePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBatchComputeEnvironmentUpdatePolicyEl {}
impl BuildBatchComputeEnvironmentUpdatePolicyEl {
    pub fn build(self) -> BatchComputeEnvironmentUpdatePolicyEl {
        BatchComputeEnvironmentUpdatePolicyEl {
            job_execution_timeout_minutes: core::default::Default::default(),
            terminate_jobs_on_update: core::default::Default::default(),
        }
    }
}
pub struct BatchComputeEnvironmentUpdatePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BatchComputeEnvironmentUpdatePolicyElRef {
    fn new(shared: StackShared, base: String) -> BatchComputeEnvironmentUpdatePolicyElRef {
        BatchComputeEnvironmentUpdatePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BatchComputeEnvironmentUpdatePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `job_execution_timeout_minutes` after provisioning.\n"]
    pub fn job_execution_timeout_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_execution_timeout_minutes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `terminate_jobs_on_update` after provisioning.\n"]
    pub fn terminate_jobs_on_update(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_jobs_on_update", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BatchComputeEnvironmentDynamic {
    compute_resources: Option<DynamicBlock<BatchComputeEnvironmentComputeResourcesEl>>,
    eks_configuration: Option<DynamicBlock<BatchComputeEnvironmentEksConfigurationEl>>,
    update_policy: Option<DynamicBlock<BatchComputeEnvironmentUpdatePolicyEl>>,
}
