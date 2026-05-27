use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NotebooksEnvironmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_startup_script: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_image: Option<Vec<NotebooksEnvironmentContainerImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NotebooksEnvironmentTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vm_image: Option<Vec<NotebooksEnvironmentVmImageEl>>,
    dynamic: NotebooksEnvironmentDynamic,
}
struct NotebooksEnvironment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NotebooksEnvironmentData>,
}
#[derive(Clone)]
pub struct NotebooksEnvironment(Rc<NotebooksEnvironment_>);
impl NotebooksEnvironment {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA brief description of this environment."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nDisplay name of this environment for the UI."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `post_startup_script`.\nPath to a Bash script that automatically runs after a notebook instance fully boots up.\nThe path must be a URL or Cloud Storage path. Example: \"gs://path-to-file/file-name\""]
    pub fn set_post_startup_script(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().post_startup_script = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `container_image`.\n"]
    pub fn set_container_image(
        self,
        v: impl Into<BlockAssignable<NotebooksEnvironmentContainerImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().container_image = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.container_image = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NotebooksEnvironmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `vm_image`.\n"]
    pub fn set_vm_image(
        self,
        v: impl Into<BlockAssignable<NotebooksEnvironmentVmImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vm_image = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vm_image = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nInstance creation time"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA brief description of this environment."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of this environment for the UI."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nA reference to the zone where the machine resides."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name specified for the Environment instance.\nFormat: projects/{project_id}/locations/{location}/environments/{environmentId}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `post_startup_script` after provisioning.\nPath to a Bash script that automatically runs after a notebook instance fully boots up.\nThe path must be a URL or Cloud Storage path. Example: \"gs://path-to-file/file-name\""]
    pub fn post_startup_script(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.post_startup_script", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `container_image` after provisioning.\n"]
    pub fn container_image(&self) -> ListRef<NotebooksEnvironmentContainerImageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_image", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NotebooksEnvironmentTimeoutsElRef {
        NotebooksEnvironmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vm_image` after provisioning.\n"]
    pub fn vm_image(&self) -> ListRef<NotebooksEnvironmentVmImageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vm_image", self.extract_ref()),
        )
    }
}
impl Referable for NotebooksEnvironment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NotebooksEnvironment {}
impl ToListMappable for NotebooksEnvironment {
    type O = ListRef<NotebooksEnvironmentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NotebooksEnvironment_ {
    fn extract_resource_type(&self) -> String {
        "google_notebooks_environment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNotebooksEnvironment {
    pub tf_id: String,
    #[doc = "A reference to the zone where the machine resides."]
    pub location: PrimField<String>,
    #[doc = "The name specified for the Environment instance.\nFormat: projects/{project_id}/locations/{location}/environments/{environmentId}"]
    pub name: PrimField<String>,
}
impl BuildNotebooksEnvironment {
    pub fn build(self, stack: &mut Stack) -> NotebooksEnvironment {
        let out = NotebooksEnvironment(Rc::new(NotebooksEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NotebooksEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                post_startup_script: core::default::Default::default(),
                project: core::default::Default::default(),
                container_image: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vm_image: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NotebooksEnvironmentRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksEnvironmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NotebooksEnvironmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nInstance creation time"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA brief description of this environment."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of this environment for the UI."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nA reference to the zone where the machine resides."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name specified for the Environment instance.\nFormat: projects/{project_id}/locations/{location}/environments/{environmentId}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `post_startup_script` after provisioning.\nPath to a Bash script that automatically runs after a notebook instance fully boots up.\nThe path must be a URL or Cloud Storage path. Example: \"gs://path-to-file/file-name\""]
    pub fn post_startup_script(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.post_startup_script", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `container_image` after provisioning.\n"]
    pub fn container_image(&self) -> ListRef<NotebooksEnvironmentContainerImageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_image", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NotebooksEnvironmentTimeoutsElRef {
        NotebooksEnvironmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vm_image` after provisioning.\n"]
    pub fn vm_image(&self) -> ListRef<NotebooksEnvironmentVmImageElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vm_image", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NotebooksEnvironmentContainerImageEl {
    repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}
impl NotebooksEnvironmentContainerImageEl {
    #[doc = "Set the field `tag`.\nThe tag of the container image. If not specified, this defaults to the latest tag."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
}
impl ToListMappable for NotebooksEnvironmentContainerImageEl {
    type O = BlockAssignable<NotebooksEnvironmentContainerImageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksEnvironmentContainerImageEl {
    #[doc = "The path to the container image repository.\nFor example: gcr.io/{project_id}/{imageName}"]
    pub repository: PrimField<String>,
}
impl BuildNotebooksEnvironmentContainerImageEl {
    pub fn build(self) -> NotebooksEnvironmentContainerImageEl {
        NotebooksEnvironmentContainerImageEl {
            repository: self.repository,
            tag: core::default::Default::default(),
        }
    }
}
pub struct NotebooksEnvironmentContainerImageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksEnvironmentContainerImageElRef {
    fn new(shared: StackShared, base: String) -> NotebooksEnvironmentContainerImageElRef {
        NotebooksEnvironmentContainerImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksEnvironmentContainerImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\nThe path to the container image repository.\nFor example: gcr.io/{project_id}/{imageName}"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\nThe tag of the container image. If not specified, this defaults to the latest tag."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}
#[derive(Serialize)]
pub struct NotebooksEnvironmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NotebooksEnvironmentTimeoutsEl {
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
impl ToListMappable for NotebooksEnvironmentTimeoutsEl {
    type O = BlockAssignable<NotebooksEnvironmentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksEnvironmentTimeoutsEl {}
impl BuildNotebooksEnvironmentTimeoutsEl {
    pub fn build(self) -> NotebooksEnvironmentTimeoutsEl {
        NotebooksEnvironmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NotebooksEnvironmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksEnvironmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NotebooksEnvironmentTimeoutsElRef {
        NotebooksEnvironmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksEnvironmentTimeoutsElRef {
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
#[derive(Serialize)]
pub struct NotebooksEnvironmentVmImageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_family: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_name: Option<PrimField<String>>,
    project: PrimField<String>,
}
impl NotebooksEnvironmentVmImageEl {
    #[doc = "Set the field `image_family`.\nUse this VM image family to find the image; the newest image in this family will be used."]
    pub fn set_image_family(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_family = Some(v.into());
        self
    }
    #[doc = "Set the field `image_name`.\nUse VM image name to find the image."]
    pub fn set_image_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_name = Some(v.into());
        self
    }
}
impl ToListMappable for NotebooksEnvironmentVmImageEl {
    type O = BlockAssignable<NotebooksEnvironmentVmImageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksEnvironmentVmImageEl {
    #[doc = "The name of the Google Cloud project that this VM image belongs to.\nFormat: projects/{project_id}"]
    pub project: PrimField<String>,
}
impl BuildNotebooksEnvironmentVmImageEl {
    pub fn build(self) -> NotebooksEnvironmentVmImageEl {
        NotebooksEnvironmentVmImageEl {
            image_family: core::default::Default::default(),
            image_name: core::default::Default::default(),
            project: self.project,
        }
    }
}
pub struct NotebooksEnvironmentVmImageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksEnvironmentVmImageElRef {
    fn new(shared: StackShared, base: String) -> NotebooksEnvironmentVmImageElRef {
        NotebooksEnvironmentVmImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksEnvironmentVmImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `image_family` after provisioning.\nUse this VM image family to find the image; the newest image in this family will be used."]
    pub fn image_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_family", self.base))
    }
    #[doc = "Get a reference to the value of field `image_name` after provisioning.\nUse VM image name to find the image."]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe name of the Google Cloud project that this VM image belongs to.\nFormat: projects/{project_id}"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}
#[derive(Serialize, Default)]
struct NotebooksEnvironmentDynamic {
    container_image: Option<DynamicBlock<NotebooksEnvironmentContainerImageEl>>,
    vm_image: Option<DynamicBlock<NotebooksEnvironmentVmImageEl>>,
}
