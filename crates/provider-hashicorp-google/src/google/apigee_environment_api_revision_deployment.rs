use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApigeeEnvironmentApiRevisionDeploymentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    api: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    environment: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    org_id: PrimField<String>,
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    override_: Option<PrimField<bool>>,
    revision: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sequenced_rollout: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeEnvironmentApiRevisionDeploymentTimeoutsEl>,
}
struct ApigeeEnvironmentApiRevisionDeployment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeEnvironmentApiRevisionDeploymentData>,
}
#[derive(Clone)]
pub struct ApigeeEnvironmentApiRevisionDeployment(Rc<ApigeeEnvironmentApiRevisionDeployment_>);
impl ApigeeEnvironmentApiRevisionDeployment {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `override_`.\nIf true, replaces other deployed revisions of this proxy in the environment."]
    pub fn set_override(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().override_ = Some(v.into());
        self
    }
    #[doc = "Set the field `sequenced_rollout`.\nIf true, enables sequenced rollout for safe traffic switching."]
    pub fn set_sequenced_rollout(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().sequenced_rollout = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account`.\nOptional service account the deployed proxy runs as."]
    pub fn set_service_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_account = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<ApigeeEnvironmentApiRevisionDeploymentTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `api` after provisioning.\nApigee API proxy name."]
    pub fn api(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `basepaths` after provisioning.\nBasepaths associated with the deployed proxy."]
    pub fn basepaths(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.basepaths", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deploy_start_time` after provisioning.\nRFC3339 timestamp when deployment started."]
    pub fn deploy_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deploy_start_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\nApigee environment name."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nApigee organization ID."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `override_` after provisioning.\nIf true, replaces other deployed revisions of this proxy in the environment."]
    pub fn override_(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.override", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\nAPI proxy revision number to deploy."]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sequenced_rollout` after provisioning.\nIf true, enables sequenced rollout for safe traffic switching."]
    pub fn sequenced_rollout(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sequenced_rollout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nOptional service account the deployed proxy runs as."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nDeployment state reported by Apigee."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeEnvironmentApiRevisionDeploymentTimeoutsElRef {
        ApigeeEnvironmentApiRevisionDeploymentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApigeeEnvironmentApiRevisionDeployment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApigeeEnvironmentApiRevisionDeployment {}
impl ToListMappable for ApigeeEnvironmentApiRevisionDeployment {
    type O = ListRef<ApigeeEnvironmentApiRevisionDeploymentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApigeeEnvironmentApiRevisionDeployment_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_environment_api_revision_deployment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApigeeEnvironmentApiRevisionDeployment {
    pub tf_id: String,
    #[doc = "Apigee API proxy name."]
    pub api: PrimField<String>,
    #[doc = "Apigee environment name."]
    pub environment: PrimField<String>,
    #[doc = "Apigee organization ID."]
    pub org_id: PrimField<String>,
    #[doc = "API proxy revision number to deploy."]
    pub revision: PrimField<f64>,
}
impl BuildApigeeEnvironmentApiRevisionDeployment {
    pub fn build(self, stack: &mut Stack) -> ApigeeEnvironmentApiRevisionDeployment {
        let out = ApigeeEnvironmentApiRevisionDeployment(Rc::new(
            ApigeeEnvironmentApiRevisionDeployment_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(ApigeeEnvironmentApiRevisionDeploymentData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    api: self.api,
                    deletion_policy: core::default::Default::default(),
                    environment: self.environment,
                    id: core::default::Default::default(),
                    org_id: self.org_id,
                    override_: core::default::Default::default(),
                    revision: self.revision,
                    sequenced_rollout: core::default::Default::default(),
                    service_account: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApigeeEnvironmentApiRevisionDeploymentRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeEnvironmentApiRevisionDeploymentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApigeeEnvironmentApiRevisionDeploymentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api` after provisioning.\nApigee API proxy name."]
    pub fn api(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `basepaths` after provisioning.\nBasepaths associated with the deployed proxy."]
    pub fn basepaths(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.basepaths", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deploy_start_time` after provisioning.\nRFC3339 timestamp when deployment started."]
    pub fn deploy_start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deploy_start_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\nApigee environment name."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nApigee organization ID."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `override_` after provisioning.\nIf true, replaces other deployed revisions of this proxy in the environment."]
    pub fn override_(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.override", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\nAPI proxy revision number to deploy."]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sequenced_rollout` after provisioning.\nIf true, enables sequenced rollout for safe traffic switching."]
    pub fn sequenced_rollout(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sequenced_rollout", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nOptional service account the deployed proxy runs as."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nDeployment state reported by Apigee."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeEnvironmentApiRevisionDeploymentTimeoutsElRef {
        ApigeeEnvironmentApiRevisionDeploymentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeEnvironmentApiRevisionDeploymentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl ApigeeEnvironmentApiRevisionDeploymentTimeoutsEl {
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
}
impl ToListMappable for ApigeeEnvironmentApiRevisionDeploymentTimeoutsEl {
    type O = BlockAssignable<ApigeeEnvironmentApiRevisionDeploymentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeEnvironmentApiRevisionDeploymentTimeoutsEl {}
impl BuildApigeeEnvironmentApiRevisionDeploymentTimeoutsEl {
    pub fn build(self) -> ApigeeEnvironmentApiRevisionDeploymentTimeoutsEl {
        ApigeeEnvironmentApiRevisionDeploymentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct ApigeeEnvironmentApiRevisionDeploymentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeEnvironmentApiRevisionDeploymentTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeEnvironmentApiRevisionDeploymentTimeoutsElRef {
        ApigeeEnvironmentApiRevisionDeploymentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeEnvironmentApiRevisionDeploymentTimeoutsElRef {
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
}
