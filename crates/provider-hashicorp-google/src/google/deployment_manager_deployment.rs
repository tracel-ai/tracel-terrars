use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DeploymentManagerDeploymentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<DeploymentManagerDeploymentLabelsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Vec<DeploymentManagerDeploymentTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DeploymentManagerDeploymentTimeoutsEl>,
    dynamic: DeploymentManagerDeploymentDynamic,
}
struct DeploymentManagerDeployment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DeploymentManagerDeploymentData>,
}
#[derive(Clone)]
pub struct DeploymentManagerDeployment(Rc<DeploymentManagerDeployment_>);
impl DeploymentManagerDeployment {
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
    #[doc = "Set the field `create_policy`.\nSet the policy to use for creating new resources. Only used on\ncreate and update. Valid values are 'CREATE_OR_ACQUIRE' (default) or\n'ACQUIRE'. If set to 'ACQUIRE' and resources do not already exist,\nthe deployment will fail. Note that updating this field does not\nactually affect the deployment, just how it is updated. Default value: \"CREATE_OR_ACQUIRE\" Possible values: [\"ACQUIRE\", \"CREATE_OR_ACQUIRE\"]"]
    pub fn set_create_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().create_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `delete_policy`.\nSet the policy to use for deleting new resources on update/delete.\nValid values are 'DELETE' (default) or 'ABANDON'. If 'DELETE',\nresource is deleted after removal from Deployment Manager. If\n'ABANDON', the resource is only removed from Deployment Manager\nand is not actually deleted. Note that updating this field does not\nactually change the deployment, just how it is updated. Default value: \"DELETE\" Possible values: [\"ABANDON\", \"DELETE\"]"]
    pub fn set_delete_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().delete_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nOptional user-provided description of deployment."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `preview`.\nIf set to true, a deployment is created with \"shell\" resources\nthat are not actually instantiated. This allows you to preview a\ndeployment. It can be updated to false to actually deploy\nwith real resources.\n ~>**NOTE:** Deployment Manager does not allow update\nof a deployment in preview (unless updating to preview=false). Thus,\nTerraform will force-recreate deployments if either preview is updated\nto true or if other fields are updated while preview is true."]
    pub fn set_preview(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().preview = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        self,
        v: impl Into<BlockAssignable<DeploymentManagerDeploymentLabelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().labels = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.labels = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        self,
        v: impl Into<BlockAssignable<DeploymentManagerDeploymentTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DeploymentManagerDeploymentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_policy` after provisioning.\nSet the policy to use for creating new resources. Only used on\ncreate and update. Valid values are 'CREATE_OR_ACQUIRE' (default) or\n'ACQUIRE'. If set to 'ACQUIRE' and resources do not already exist,\nthe deployment will fail. Note that updating this field does not\nactually affect the deployment, just how it is updated. Default value: \"CREATE_OR_ACQUIRE\" Possible values: [\"ACQUIRE\", \"CREATE_OR_ACQUIRE\"]"]
    pub fn create_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_policy` after provisioning.\nSet the policy to use for deleting new resources on update/delete.\nValid values are 'DELETE' (default) or 'ABANDON'. If 'DELETE',\nresource is deleted after removal from Deployment Manager. If\n'ABANDON', the resource is only removed from Deployment Manager\nand is not actually deleted. Note that updating this field does not\nactually change the deployment, just how it is updated. Default value: \"DELETE\" Possible values: [\"ABANDON\", \"DELETE\"]"]
    pub fn delete_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_id` after provisioning.\nUnique identifier for deployment. Output only."]
    pub fn deployment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional user-provided description of deployment."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `manifest` after provisioning.\nOutput only. URL of the manifest representing the last manifest that\nwas successfully deployed."]
    pub fn manifest(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manifest", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name for the deployment"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preview` after provisioning.\nIf set to true, a deployment is created with \"shell\" resources\nthat are not actually instantiated. This allows you to preview a\ndeployment. It can be updated to false to actually deploy\nwith real resources.\n ~>**NOTE:** Deployment Manager does not allow update\nof a deployment in preview (unless updating to preview=false). Thus,\nTerraform will force-recreate deployments if either preview is updated\nto true or if other fields are updated while preview is true."]
    pub fn preview(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preview", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nOutput only. Server defined URL for the resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<DeploymentManagerDeploymentTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DeploymentManagerDeploymentTimeoutsElRef {
        DeploymentManagerDeploymentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DeploymentManagerDeployment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DeploymentManagerDeployment {}
impl ToListMappable for DeploymentManagerDeployment {
    type O = ListRef<DeploymentManagerDeploymentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DeploymentManagerDeployment_ {
    fn extract_resource_type(&self) -> String {
        "google_deployment_manager_deployment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDeploymentManagerDeployment {
    pub tf_id: String,
    #[doc = "Unique name for the deployment"]
    pub name: PrimField<String>,
}
impl BuildDeploymentManagerDeployment {
    pub fn build(self, stack: &mut Stack) -> DeploymentManagerDeployment {
        let out = DeploymentManagerDeployment(Rc::new(DeploymentManagerDeployment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DeploymentManagerDeploymentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                create_policy: core::default::Default::default(),
                delete_policy: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                preview: core::default::Default::default(),
                project: core::default::Default::default(),
                labels: core::default::Default::default(),
                target: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DeploymentManagerDeploymentRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeploymentManagerDeploymentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DeploymentManagerDeploymentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_policy` after provisioning.\nSet the policy to use for creating new resources. Only used on\ncreate and update. Valid values are 'CREATE_OR_ACQUIRE' (default) or\n'ACQUIRE'. If set to 'ACQUIRE' and resources do not already exist,\nthe deployment will fail. Note that updating this field does not\nactually affect the deployment, just how it is updated. Default value: \"CREATE_OR_ACQUIRE\" Possible values: [\"ACQUIRE\", \"CREATE_OR_ACQUIRE\"]"]
    pub fn create_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_policy` after provisioning.\nSet the policy to use for deleting new resources on update/delete.\nValid values are 'DELETE' (default) or 'ABANDON'. If 'DELETE',\nresource is deleted after removal from Deployment Manager. If\n'ABANDON', the resource is only removed from Deployment Manager\nand is not actually deleted. Note that updating this field does not\nactually change the deployment, just how it is updated. Default value: \"DELETE\" Possible values: [\"ABANDON\", \"DELETE\"]"]
    pub fn delete_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_id` after provisioning.\nUnique identifier for deployment. Output only."]
    pub fn deployment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional user-provided description of deployment."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `manifest` after provisioning.\nOutput only. URL of the manifest representing the last manifest that\nwas successfully deployed."]
    pub fn manifest(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manifest", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name for the deployment"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preview` after provisioning.\nIf set to true, a deployment is created with \"shell\" resources\nthat are not actually instantiated. This allows you to preview a\ndeployment. It can be updated to false to actually deploy\nwith real resources.\n ~>**NOTE:** Deployment Manager does not allow update\nof a deployment in preview (unless updating to preview=false). Thus,\nTerraform will force-recreate deployments if either preview is updated\nto true or if other fields are updated while preview is true."]
    pub fn preview(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preview", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nOutput only. Server defined URL for the resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<DeploymentManagerDeploymentTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DeploymentManagerDeploymentTimeoutsElRef {
        DeploymentManagerDeploymentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DeploymentManagerDeploymentLabelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DeploymentManagerDeploymentLabelsEl {
    #[doc = "Set the field `key`.\nKey for label."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nValue of label."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DeploymentManagerDeploymentLabelsEl {
    type O = BlockAssignable<DeploymentManagerDeploymentLabelsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeploymentManagerDeploymentLabelsEl {}
impl BuildDeploymentManagerDeploymentLabelsEl {
    pub fn build(self) -> DeploymentManagerDeploymentLabelsEl {
        DeploymentManagerDeploymentLabelsEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DeploymentManagerDeploymentLabelsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeploymentManagerDeploymentLabelsElRef {
    fn new(shared: StackShared, base: String) -> DeploymentManagerDeploymentLabelsElRef {
        DeploymentManagerDeploymentLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeploymentManagerDeploymentLabelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nKey for label."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of label."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DeploymentManagerDeploymentTargetElConfigEl {
    content: PrimField<String>,
}
impl DeploymentManagerDeploymentTargetElConfigEl {}
impl ToListMappable for DeploymentManagerDeploymentTargetElConfigEl {
    type O = BlockAssignable<DeploymentManagerDeploymentTargetElConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeploymentManagerDeploymentTargetElConfigEl {
    #[doc = "The full YAML contents of your configuration file."]
    pub content: PrimField<String>,
}
impl BuildDeploymentManagerDeploymentTargetElConfigEl {
    pub fn build(self) -> DeploymentManagerDeploymentTargetElConfigEl {
        DeploymentManagerDeploymentTargetElConfigEl {
            content: self.content,
        }
    }
}
pub struct DeploymentManagerDeploymentTargetElConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeploymentManagerDeploymentTargetElConfigElRef {
    fn new(shared: StackShared, base: String) -> DeploymentManagerDeploymentTargetElConfigElRef {
        DeploymentManagerDeploymentTargetElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeploymentManagerDeploymentTargetElConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content` after provisioning.\nThe full YAML contents of your configuration file."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }
}
#[derive(Serialize)]
pub struct DeploymentManagerDeploymentTargetElImportsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DeploymentManagerDeploymentTargetElImportsEl {
    #[doc = "Set the field `content`.\nThe full contents of the template that you want to import."]
    pub fn set_content(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nThe name of the template to import, as declared in the YAML\nconfiguration."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DeploymentManagerDeploymentTargetElImportsEl {
    type O = BlockAssignable<DeploymentManagerDeploymentTargetElImportsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeploymentManagerDeploymentTargetElImportsEl {}
impl BuildDeploymentManagerDeploymentTargetElImportsEl {
    pub fn build(self) -> DeploymentManagerDeploymentTargetElImportsEl {
        DeploymentManagerDeploymentTargetElImportsEl {
            content: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DeploymentManagerDeploymentTargetElImportsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeploymentManagerDeploymentTargetElImportsElRef {
    fn new(shared: StackShared, base: String) -> DeploymentManagerDeploymentTargetElImportsElRef {
        DeploymentManagerDeploymentTargetElImportsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeploymentManagerDeploymentTargetElImportsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content` after provisioning.\nThe full contents of the template that you want to import."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the template to import, as declared in the YAML\nconfiguration."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct DeploymentManagerDeploymentTargetElDynamic {
    config: Option<DynamicBlock<DeploymentManagerDeploymentTargetElConfigEl>>,
    imports: Option<DynamicBlock<DeploymentManagerDeploymentTargetElImportsEl>>,
}
#[derive(Serialize)]
pub struct DeploymentManagerDeploymentTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<DeploymentManagerDeploymentTargetElConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    imports: Option<Vec<DeploymentManagerDeploymentTargetElImportsEl>>,
    dynamic: DeploymentManagerDeploymentTargetElDynamic,
}
impl DeploymentManagerDeploymentTargetEl {
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl Into<BlockAssignable<DeploymentManagerDeploymentTargetElConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `imports`.\n"]
    pub fn set_imports(
        mut self,
        v: impl Into<BlockAssignable<DeploymentManagerDeploymentTargetElImportsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.imports = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.imports = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DeploymentManagerDeploymentTargetEl {
    type O = BlockAssignable<DeploymentManagerDeploymentTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeploymentManagerDeploymentTargetEl {}
impl BuildDeploymentManagerDeploymentTargetEl {
    pub fn build(self) -> DeploymentManagerDeploymentTargetEl {
        DeploymentManagerDeploymentTargetEl {
            config: core::default::Default::default(),
            imports: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DeploymentManagerDeploymentTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeploymentManagerDeploymentTargetElRef {
    fn new(shared: StackShared, base: String) -> DeploymentManagerDeploymentTargetElRef {
        DeploymentManagerDeploymentTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeploymentManagerDeploymentTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<DeploymentManagerDeploymentTargetElConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
    #[doc = "Get a reference to the value of field `imports` after provisioning.\n"]
    pub fn imports(&self) -> ListRef<DeploymentManagerDeploymentTargetElImportsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.imports", self.base))
    }
}
#[derive(Serialize)]
pub struct DeploymentManagerDeploymentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DeploymentManagerDeploymentTimeoutsEl {
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
impl ToListMappable for DeploymentManagerDeploymentTimeoutsEl {
    type O = BlockAssignable<DeploymentManagerDeploymentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDeploymentManagerDeploymentTimeoutsEl {}
impl BuildDeploymentManagerDeploymentTimeoutsEl {
    pub fn build(self) -> DeploymentManagerDeploymentTimeoutsEl {
        DeploymentManagerDeploymentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DeploymentManagerDeploymentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DeploymentManagerDeploymentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DeploymentManagerDeploymentTimeoutsElRef {
        DeploymentManagerDeploymentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DeploymentManagerDeploymentTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct DeploymentManagerDeploymentDynamic {
    labels: Option<DynamicBlock<DeploymentManagerDeploymentLabelsEl>>,
    target: Option<DynamicBlock<DeploymentManagerDeploymentTargetEl>>,
}
