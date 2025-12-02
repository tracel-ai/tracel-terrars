use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SagemakerAppImageConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app_image_config_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_editor_app_image_config: Option<Vec<SagemakerAppImageConfigCodeEditorAppImageConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jupyter_lab_image_config: Option<Vec<SagemakerAppImageConfigJupyterLabImageConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel_gateway_image_config: Option<Vec<SagemakerAppImageConfigKernelGatewayImageConfigEl>>,
    dynamic: SagemakerAppImageConfigDynamic,
}
struct SagemakerAppImageConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerAppImageConfigData>,
}
#[derive(Clone)]
pub struct SagemakerAppImageConfig(Rc<SagemakerAppImageConfig_>);
impl SagemakerAppImageConfig {
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
    #[doc = "Set the field `code_editor_app_image_config`.\n"]
    pub fn set_code_editor_app_image_config(
        self,
        v: impl Into<BlockAssignable<SagemakerAppImageConfigCodeEditorAppImageConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().code_editor_app_image_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .code_editor_app_image_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `jupyter_lab_image_config`.\n"]
    pub fn set_jupyter_lab_image_config(
        self,
        v: impl Into<BlockAssignable<SagemakerAppImageConfigJupyterLabImageConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().jupyter_lab_image_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.jupyter_lab_image_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kernel_gateway_image_config`.\n"]
    pub fn set_kernel_gateway_image_config(
        self,
        v: impl Into<BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kernel_gateway_image_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kernel_gateway_image_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_image_config_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `code_editor_app_image_config` after provisioning.\n"]
    pub fn code_editor_app_image_config(
        &self,
    ) -> ListRef<SagemakerAppImageConfigCodeEditorAppImageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.code_editor_app_image_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `jupyter_lab_image_config` after provisioning.\n"]
    pub fn jupyter_lab_image_config(
        &self,
    ) -> ListRef<SagemakerAppImageConfigJupyterLabImageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jupyter_lab_image_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kernel_gateway_image_config` after provisioning.\n"]
    pub fn kernel_gateway_image_config(
        &self,
    ) -> ListRef<SagemakerAppImageConfigKernelGatewayImageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kernel_gateway_image_config", self.extract_ref()),
        )
    }
}
impl Referable for SagemakerAppImageConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SagemakerAppImageConfig {}
impl ToListMappable for SagemakerAppImageConfig {
    type O = ListRef<SagemakerAppImageConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SagemakerAppImageConfig_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_app_image_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSagemakerAppImageConfig {
    pub tf_id: String,
    #[doc = ""]
    pub app_image_config_name: PrimField<String>,
}
impl BuildSagemakerAppImageConfig {
    pub fn build(self, stack: &mut Stack) -> SagemakerAppImageConfig {
        let out = SagemakerAppImageConfig(Rc::new(SagemakerAppImageConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerAppImageConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app_image_config_name: self.app_image_config_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                code_editor_app_image_config: core::default::Default::default(),
                jupyter_lab_image_config: core::default::Default::default(),
                kernel_gateway_image_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SagemakerAppImageConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerAppImageConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SagemakerAppImageConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_image_config_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `code_editor_app_image_config` after provisioning.\n"]
    pub fn code_editor_app_image_config(
        &self,
    ) -> ListRef<SagemakerAppImageConfigCodeEditorAppImageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.code_editor_app_image_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `jupyter_lab_image_config` after provisioning.\n"]
    pub fn jupyter_lab_image_config(
        &self,
    ) -> ListRef<SagemakerAppImageConfigJupyterLabImageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jupyter_lab_image_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kernel_gateway_image_config` after provisioning.\n"]
    pub fn kernel_gateway_image_config(
        &self,
    ) -> ListRef<SagemakerAppImageConfigKernelGatewayImageConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kernel_gateway_image_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerAppImageConfigCodeEditorAppImageConfigElContainerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_arguments: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_entrypoint: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_environment_variables: Option<RecField<PrimField<String>>>,
}
impl SagemakerAppImageConfigCodeEditorAppImageConfigElContainerConfigEl {
    #[doc = "Set the field `container_arguments`.\n"]
    pub fn set_container_arguments(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.container_arguments = Some(v.into());
        self
    }
    #[doc = "Set the field `container_entrypoint`.\n"]
    pub fn set_container_entrypoint(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.container_entrypoint = Some(v.into());
        self
    }
    #[doc = "Set the field `container_environment_variables`.\n"]
    pub fn set_container_environment_variables(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.container_environment_variables = Some(v.into());
        self
    }
}
impl ToListMappable for SagemakerAppImageConfigCodeEditorAppImageConfigElContainerConfigEl {
    type O = BlockAssignable<SagemakerAppImageConfigCodeEditorAppImageConfigElContainerConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerAppImageConfigCodeEditorAppImageConfigElContainerConfigEl {}
impl BuildSagemakerAppImageConfigCodeEditorAppImageConfigElContainerConfigEl {
    pub fn build(self) -> SagemakerAppImageConfigCodeEditorAppImageConfigElContainerConfigEl {
        SagemakerAppImageConfigCodeEditorAppImageConfigElContainerConfigEl {
            container_arguments: core::default::Default::default(),
            container_entrypoint: core::default::Default::default(),
            container_environment_variables: core::default::Default::default(),
        }
    }
}
pub struct SagemakerAppImageConfigCodeEditorAppImageConfigElContainerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerAppImageConfigCodeEditorAppImageConfigElContainerConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerAppImageConfigCodeEditorAppImageConfigElContainerConfigElRef {
        SagemakerAppImageConfigCodeEditorAppImageConfigElContainerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerAppImageConfigCodeEditorAppImageConfigElContainerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_arguments` after provisioning.\n"]
    pub fn container_arguments(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_arguments", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_entrypoint` after provisioning.\n"]
    pub fn container_entrypoint(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_entrypoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_environment_variables` after provisioning.\n"]
    pub fn container_environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.container_environment_variables", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerAppImageConfigCodeEditorAppImageConfigElFileSystemConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_gid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_uid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_path: Option<PrimField<String>>,
}
impl SagemakerAppImageConfigCodeEditorAppImageConfigElFileSystemConfigEl {
    #[doc = "Set the field `default_gid`.\n"]
    pub fn set_default_gid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_gid = Some(v.into());
        self
    }
    #[doc = "Set the field `default_uid`.\n"]
    pub fn set_default_uid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_uid = Some(v.into());
        self
    }
    #[doc = "Set the field `mount_path`.\n"]
    pub fn set_mount_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mount_path = Some(v.into());
        self
    }
}
impl ToListMappable for SagemakerAppImageConfigCodeEditorAppImageConfigElFileSystemConfigEl {
    type O = BlockAssignable<SagemakerAppImageConfigCodeEditorAppImageConfigElFileSystemConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerAppImageConfigCodeEditorAppImageConfigElFileSystemConfigEl {}
impl BuildSagemakerAppImageConfigCodeEditorAppImageConfigElFileSystemConfigEl {
    pub fn build(self) -> SagemakerAppImageConfigCodeEditorAppImageConfigElFileSystemConfigEl {
        SagemakerAppImageConfigCodeEditorAppImageConfigElFileSystemConfigEl {
            default_gid: core::default::Default::default(),
            default_uid: core::default::Default::default(),
            mount_path: core::default::Default::default(),
        }
    }
}
pub struct SagemakerAppImageConfigCodeEditorAppImageConfigElFileSystemConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerAppImageConfigCodeEditorAppImageConfigElFileSystemConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerAppImageConfigCodeEditorAppImageConfigElFileSystemConfigElRef {
        SagemakerAppImageConfigCodeEditorAppImageConfigElFileSystemConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerAppImageConfigCodeEditorAppImageConfigElFileSystemConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_gid` after provisioning.\n"]
    pub fn default_gid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_gid", self.base))
    }
    #[doc = "Get a reference to the value of field `default_uid` after provisioning.\n"]
    pub fn default_uid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_uid", self.base))
    }
    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
}
#[derive(Serialize, Default)]
struct SagemakerAppImageConfigCodeEditorAppImageConfigElDynamic {
    container_config:
        Option<DynamicBlock<SagemakerAppImageConfigCodeEditorAppImageConfigElContainerConfigEl>>,
    file_system_config:
        Option<DynamicBlock<SagemakerAppImageConfigCodeEditorAppImageConfigElFileSystemConfigEl>>,
}
#[derive(Serialize)]
pub struct SagemakerAppImageConfigCodeEditorAppImageConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_config:
        Option<Vec<SagemakerAppImageConfigCodeEditorAppImageConfigElContainerConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_config:
        Option<Vec<SagemakerAppImageConfigCodeEditorAppImageConfigElFileSystemConfigEl>>,
    dynamic: SagemakerAppImageConfigCodeEditorAppImageConfigElDynamic,
}
impl SagemakerAppImageConfigCodeEditorAppImageConfigEl {
    #[doc = "Set the field `container_config`.\n"]
    pub fn set_container_config(
        mut self,
        v: impl Into<
            BlockAssignable<SagemakerAppImageConfigCodeEditorAppImageConfigElContainerConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `file_system_config`.\n"]
    pub fn set_file_system_config(
        mut self,
        v: impl Into<
            BlockAssignable<SagemakerAppImageConfigCodeEditorAppImageConfigElFileSystemConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file_system_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file_system_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerAppImageConfigCodeEditorAppImageConfigEl {
    type O = BlockAssignable<SagemakerAppImageConfigCodeEditorAppImageConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerAppImageConfigCodeEditorAppImageConfigEl {}
impl BuildSagemakerAppImageConfigCodeEditorAppImageConfigEl {
    pub fn build(self) -> SagemakerAppImageConfigCodeEditorAppImageConfigEl {
        SagemakerAppImageConfigCodeEditorAppImageConfigEl {
            container_config: core::default::Default::default(),
            file_system_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerAppImageConfigCodeEditorAppImageConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerAppImageConfigCodeEditorAppImageConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerAppImageConfigCodeEditorAppImageConfigElRef {
        SagemakerAppImageConfigCodeEditorAppImageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerAppImageConfigCodeEditorAppImageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_config` after provisioning.\n"]
    pub fn container_config(
        &self,
    ) -> ListRef<SagemakerAppImageConfigCodeEditorAppImageConfigElContainerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(
        &self,
    ) -> ListRef<SagemakerAppImageConfigCodeEditorAppImageConfigElFileSystemConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.file_system_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerAppImageConfigJupyterLabImageConfigElContainerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_arguments: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_entrypoint: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_environment_variables: Option<RecField<PrimField<String>>>,
}
impl SagemakerAppImageConfigJupyterLabImageConfigElContainerConfigEl {
    #[doc = "Set the field `container_arguments`.\n"]
    pub fn set_container_arguments(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.container_arguments = Some(v.into());
        self
    }
    #[doc = "Set the field `container_entrypoint`.\n"]
    pub fn set_container_entrypoint(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.container_entrypoint = Some(v.into());
        self
    }
    #[doc = "Set the field `container_environment_variables`.\n"]
    pub fn set_container_environment_variables(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.container_environment_variables = Some(v.into());
        self
    }
}
impl ToListMappable for SagemakerAppImageConfigJupyterLabImageConfigElContainerConfigEl {
    type O = BlockAssignable<SagemakerAppImageConfigJupyterLabImageConfigElContainerConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerAppImageConfigJupyterLabImageConfigElContainerConfigEl {}
impl BuildSagemakerAppImageConfigJupyterLabImageConfigElContainerConfigEl {
    pub fn build(self) -> SagemakerAppImageConfigJupyterLabImageConfigElContainerConfigEl {
        SagemakerAppImageConfigJupyterLabImageConfigElContainerConfigEl {
            container_arguments: core::default::Default::default(),
            container_entrypoint: core::default::Default::default(),
            container_environment_variables: core::default::Default::default(),
        }
    }
}
pub struct SagemakerAppImageConfigJupyterLabImageConfigElContainerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerAppImageConfigJupyterLabImageConfigElContainerConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerAppImageConfigJupyterLabImageConfigElContainerConfigElRef {
        SagemakerAppImageConfigJupyterLabImageConfigElContainerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerAppImageConfigJupyterLabImageConfigElContainerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_arguments` after provisioning.\n"]
    pub fn container_arguments(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_arguments", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_entrypoint` after provisioning.\n"]
    pub fn container_entrypoint(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_entrypoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_environment_variables` after provisioning.\n"]
    pub fn container_environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.container_environment_variables", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerAppImageConfigJupyterLabImageConfigElFileSystemConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_gid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_uid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_path: Option<PrimField<String>>,
}
impl SagemakerAppImageConfigJupyterLabImageConfigElFileSystemConfigEl {
    #[doc = "Set the field `default_gid`.\n"]
    pub fn set_default_gid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_gid = Some(v.into());
        self
    }
    #[doc = "Set the field `default_uid`.\n"]
    pub fn set_default_uid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_uid = Some(v.into());
        self
    }
    #[doc = "Set the field `mount_path`.\n"]
    pub fn set_mount_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mount_path = Some(v.into());
        self
    }
}
impl ToListMappable for SagemakerAppImageConfigJupyterLabImageConfigElFileSystemConfigEl {
    type O = BlockAssignable<SagemakerAppImageConfigJupyterLabImageConfigElFileSystemConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerAppImageConfigJupyterLabImageConfigElFileSystemConfigEl {}
impl BuildSagemakerAppImageConfigJupyterLabImageConfigElFileSystemConfigEl {
    pub fn build(self) -> SagemakerAppImageConfigJupyterLabImageConfigElFileSystemConfigEl {
        SagemakerAppImageConfigJupyterLabImageConfigElFileSystemConfigEl {
            default_gid: core::default::Default::default(),
            default_uid: core::default::Default::default(),
            mount_path: core::default::Default::default(),
        }
    }
}
pub struct SagemakerAppImageConfigJupyterLabImageConfigElFileSystemConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerAppImageConfigJupyterLabImageConfigElFileSystemConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerAppImageConfigJupyterLabImageConfigElFileSystemConfigElRef {
        SagemakerAppImageConfigJupyterLabImageConfigElFileSystemConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerAppImageConfigJupyterLabImageConfigElFileSystemConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_gid` after provisioning.\n"]
    pub fn default_gid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_gid", self.base))
    }
    #[doc = "Get a reference to the value of field `default_uid` after provisioning.\n"]
    pub fn default_uid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_uid", self.base))
    }
    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
}
#[derive(Serialize, Default)]
struct SagemakerAppImageConfigJupyterLabImageConfigElDynamic {
    container_config:
        Option<DynamicBlock<SagemakerAppImageConfigJupyterLabImageConfigElContainerConfigEl>>,
    file_system_config:
        Option<DynamicBlock<SagemakerAppImageConfigJupyterLabImageConfigElFileSystemConfigEl>>,
}
#[derive(Serialize)]
pub struct SagemakerAppImageConfigJupyterLabImageConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_config: Option<Vec<SagemakerAppImageConfigJupyterLabImageConfigElContainerConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_config:
        Option<Vec<SagemakerAppImageConfigJupyterLabImageConfigElFileSystemConfigEl>>,
    dynamic: SagemakerAppImageConfigJupyterLabImageConfigElDynamic,
}
impl SagemakerAppImageConfigJupyterLabImageConfigEl {
    #[doc = "Set the field `container_config`.\n"]
    pub fn set_container_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerAppImageConfigJupyterLabImageConfigElContainerConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `file_system_config`.\n"]
    pub fn set_file_system_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerAppImageConfigJupyterLabImageConfigElFileSystemConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file_system_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file_system_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerAppImageConfigJupyterLabImageConfigEl {
    type O = BlockAssignable<SagemakerAppImageConfigJupyterLabImageConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerAppImageConfigJupyterLabImageConfigEl {}
impl BuildSagemakerAppImageConfigJupyterLabImageConfigEl {
    pub fn build(self) -> SagemakerAppImageConfigJupyterLabImageConfigEl {
        SagemakerAppImageConfigJupyterLabImageConfigEl {
            container_config: core::default::Default::default(),
            file_system_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerAppImageConfigJupyterLabImageConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerAppImageConfigJupyterLabImageConfigElRef {
    fn new(shared: StackShared, base: String) -> SagemakerAppImageConfigJupyterLabImageConfigElRef {
        SagemakerAppImageConfigJupyterLabImageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerAppImageConfigJupyterLabImageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_config` after provisioning.\n"]
    pub fn container_config(
        &self,
    ) -> ListRef<SagemakerAppImageConfigJupyterLabImageConfigElContainerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(
        &self,
    ) -> ListRef<SagemakerAppImageConfigJupyterLabImageConfigElFileSystemConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.file_system_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_gid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_uid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mount_path: Option<PrimField<String>>,
}
impl SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl {
    #[doc = "Set the field `default_gid`.\n"]
    pub fn set_default_gid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_gid = Some(v.into());
        self
    }
    #[doc = "Set the field `default_uid`.\n"]
    pub fn set_default_uid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_uid = Some(v.into());
        self
    }
    #[doc = "Set the field `mount_path`.\n"]
    pub fn set_mount_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mount_path = Some(v.into());
        self
    }
}
impl ToListMappable for SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl {
    type O = BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl {}
impl BuildSagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl {
    pub fn build(self) -> SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl {
        SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl {
            default_gid: core::default::Default::default(),
            default_uid: core::default::Default::default(),
            mount_path: core::default::Default::default(),
        }
    }
}
pub struct SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigElRef {
        SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_gid` after provisioning.\n"]
    pub fn default_gid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_gid", self.base))
    }
    #[doc = "Get a reference to the value of field `default_uid` after provisioning.\n"]
    pub fn default_uid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_uid", self.base))
    }
    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }
}
#[derive(Serialize)]
pub struct SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    name: PrimField<String>,
}
impl SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable for SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
    type O = BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildSagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
    pub fn build(self) -> SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
        SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl {
            display_name: core::default::Default::default(),
            name: self.name,
        }
    }
}
pub struct SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecElRef {
        SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct SagemakerAppImageConfigKernelGatewayImageConfigElDynamic {
    file_system_config:
        Option<DynamicBlock<SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl>>,
    kernel_spec:
        Option<DynamicBlock<SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl>>,
}
#[derive(Serialize)]
pub struct SagemakerAppImageConfigKernelGatewayImageConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_config:
        Option<Vec<SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel_spec: Option<Vec<SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl>>,
    dynamic: SagemakerAppImageConfigKernelGatewayImageConfigElDynamic,
}
impl SagemakerAppImageConfigKernelGatewayImageConfigEl {
    #[doc = "Set the field `file_system_config`.\n"]
    pub fn set_file_system_config(
        mut self,
        v: impl Into<
            BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file_system_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file_system_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kernel_spec`.\n"]
    pub fn set_kernel_spec(
        mut self,
        v: impl Into<BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kernel_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kernel_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SagemakerAppImageConfigKernelGatewayImageConfigEl {
    type O = BlockAssignable<SagemakerAppImageConfigKernelGatewayImageConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSagemakerAppImageConfigKernelGatewayImageConfigEl {}
impl BuildSagemakerAppImageConfigKernelGatewayImageConfigEl {
    pub fn build(self) -> SagemakerAppImageConfigKernelGatewayImageConfigEl {
        SagemakerAppImageConfigKernelGatewayImageConfigEl {
            file_system_config: core::default::Default::default(),
            kernel_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SagemakerAppImageConfigKernelGatewayImageConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SagemakerAppImageConfigKernelGatewayImageConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerAppImageConfigKernelGatewayImageConfigElRef {
        SagemakerAppImageConfigKernelGatewayImageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SagemakerAppImageConfigKernelGatewayImageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `file_system_config` after provisioning.\n"]
    pub fn file_system_config(
        &self,
    ) -> ListRef<SagemakerAppImageConfigKernelGatewayImageConfigElFileSystemConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.file_system_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kernel_spec` after provisioning.\n"]
    pub fn kernel_spec(
        &self,
    ) -> ListRef<SagemakerAppImageConfigKernelGatewayImageConfigElKernelSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kernel_spec", self.base))
    }
}
#[derive(Serialize, Default)]
struct SagemakerAppImageConfigDynamic {
    code_editor_app_image_config:
        Option<DynamicBlock<SagemakerAppImageConfigCodeEditorAppImageConfigEl>>,
    jupyter_lab_image_config: Option<DynamicBlock<SagemakerAppImageConfigJupyterLabImageConfigEl>>,
    kernel_gateway_image_config:
        Option<DynamicBlock<SagemakerAppImageConfigKernelGatewayImageConfigEl>>,
}
