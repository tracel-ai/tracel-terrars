use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SecureSourceManagerInstanceData {
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
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_config: Option<Vec<SecureSourceManagerInstancePrivateConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SecureSourceManagerInstanceTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workforce_identity_federation_config:
        Option<Vec<SecureSourceManagerInstanceWorkforceIdentityFederationConfigEl>>,
    dynamic: SecureSourceManagerInstanceDynamic,
}
struct SecureSourceManagerInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecureSourceManagerInstanceData>,
}
#[derive(Clone)]
pub struct SecureSourceManagerInstance(Rc<SecureSourceManagerInstance_>);
impl SecureSourceManagerInstance {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"PREVENT\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key`.\nCustomer-managed encryption key name, in the format projects/*/locations/*/keyRings/*/cryptoKeys/*."]
    pub fn set_kms_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels as key value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `private_config`.\n"]
    pub fn set_private_config(
        self,
        v: impl Into<BlockAssignable<SecureSourceManagerInstancePrivateConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SecureSourceManagerInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `workforce_identity_federation_config`.\n"]
    pub fn set_workforce_identity_federation_config(
        self,
        v: impl Into<BlockAssignable<SecureSourceManagerInstanceWorkforceIdentityFederationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0
                    .data
                    .borrow_mut()
                    .workforce_identity_federation_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .workforce_identity_federation_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the Instance was created in UTC."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"PREVENT\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `host_config` after provisioning.\nA list of hostnames for this instance."]
    pub fn host_config(&self) -> ListRef<SecureSourceManagerInstanceHostConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.host_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe name for the Instance."]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nCustomer-managed encryption key name, in the format projects/*/locations/*/keyRings/*/cryptoKeys/*."]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the Instance."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the Instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the Instance."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_note` after provisioning.\nProvides information about the current instance state."]
    pub fn state_note(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_note", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the Instance was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_config` after provisioning.\n"]
    pub fn private_config(&self) -> ListRef<SecureSourceManagerInstancePrivateConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecureSourceManagerInstanceTimeoutsElRef {
        SecureSourceManagerInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workforce_identity_federation_config` after provisioning.\n"]
    pub fn workforce_identity_federation_config(
        &self,
    ) -> ListRef<SecureSourceManagerInstanceWorkforceIdentityFederationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.workforce_identity_federation_config",
                self.extract_ref()
            ),
        )
    }
}
impl Referable for SecureSourceManagerInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SecureSourceManagerInstance {}
impl ToListMappable for SecureSourceManagerInstance {
    type O = ListRef<SecureSourceManagerInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SecureSourceManagerInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_secure_source_manager_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSecureSourceManagerInstance {
    pub tf_id: String,
    #[doc = "The name for the Instance."]
    pub instance_id: PrimField<String>,
    #[doc = "The location for the Instance."]
    pub location: PrimField<String>,
}
impl BuildSecureSourceManagerInstance {
    pub fn build(self, stack: &mut Stack) -> SecureSourceManagerInstance {
        let out = SecureSourceManagerInstance(Rc::new(SecureSourceManagerInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecureSourceManagerInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                kms_key: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                private_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                workforce_identity_federation_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SecureSourceManagerInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecureSourceManagerInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SecureSourceManagerInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the Instance was created in UTC."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"PREVENT\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `host_config` after provisioning.\nA list of hostnames for this instance."]
    pub fn host_config(&self) -> ListRef<SecureSourceManagerInstanceHostConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.host_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe name for the Instance."]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nCustomer-managed encryption key name, in the format projects/*/locations/*/keyRings/*/cryptoKeys/*."]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the Instance."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the Instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the Instance."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_note` after provisioning.\nProvides information about the current instance state."]
    pub fn state_note(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_note", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the Instance was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_config` after provisioning.\n"]
    pub fn private_config(&self) -> ListRef<SecureSourceManagerInstancePrivateConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecureSourceManagerInstanceTimeoutsElRef {
        SecureSourceManagerInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workforce_identity_federation_config` after provisioning.\n"]
    pub fn workforce_identity_federation_config(
        &self,
    ) -> ListRef<SecureSourceManagerInstanceWorkforceIdentityFederationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.workforce_identity_federation_config",
                self.extract_ref()
            ),
        )
    }
}
#[derive(Serialize)]
pub struct SecureSourceManagerInstanceHostConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git_http: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git_ssh: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    html: Option<PrimField<String>>,
}
impl SecureSourceManagerInstanceHostConfigEl {
    #[doc = "Set the field `api`.\n"]
    pub fn set_api(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api = Some(v.into());
        self
    }
    #[doc = "Set the field `git_http`.\n"]
    pub fn set_git_http(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.git_http = Some(v.into());
        self
    }
    #[doc = "Set the field `git_ssh`.\n"]
    pub fn set_git_ssh(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.git_ssh = Some(v.into());
        self
    }
    #[doc = "Set the field `html`.\n"]
    pub fn set_html(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.html = Some(v.into());
        self
    }
}
impl ToListMappable for SecureSourceManagerInstanceHostConfigEl {
    type O = BlockAssignable<SecureSourceManagerInstanceHostConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecureSourceManagerInstanceHostConfigEl {}
impl BuildSecureSourceManagerInstanceHostConfigEl {
    pub fn build(self) -> SecureSourceManagerInstanceHostConfigEl {
        SecureSourceManagerInstanceHostConfigEl {
            api: core::default::Default::default(),
            git_http: core::default::Default::default(),
            git_ssh: core::default::Default::default(),
            html: core::default::Default::default(),
        }
    }
}
pub struct SecureSourceManagerInstanceHostConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecureSourceManagerInstanceHostConfigElRef {
    fn new(shared: StackShared, base: String) -> SecureSourceManagerInstanceHostConfigElRef {
        SecureSourceManagerInstanceHostConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecureSourceManagerInstanceHostConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api` after provisioning.\n"]
    pub fn api(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api", self.base))
    }
    #[doc = "Get a reference to the value of field `git_http` after provisioning.\n"]
    pub fn git_http(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.git_http", self.base))
    }
    #[doc = "Get a reference to the value of field `git_ssh` after provisioning.\n"]
    pub fn git_ssh(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.git_ssh", self.base))
    }
    #[doc = "Get a reference to the value of field `html` after provisioning.\n"]
    pub fn html(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.html", self.base))
    }
}
#[derive(Serialize)]
pub struct SecureSourceManagerInstancePrivateConfigElCustomHostConfigEl {
    api: PrimField<String>,
    git_http: PrimField<String>,
    git_ssh: PrimField<String>,
    html: PrimField<String>,
}
impl SecureSourceManagerInstancePrivateConfigElCustomHostConfigEl {}
impl ToListMappable for SecureSourceManagerInstancePrivateConfigElCustomHostConfigEl {
    type O = BlockAssignable<SecureSourceManagerInstancePrivateConfigElCustomHostConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecureSourceManagerInstancePrivateConfigElCustomHostConfigEl {
    #[doc = "API hostname."]
    pub api: PrimField<String>,
    #[doc = "Git HTTP hostname."]
    pub git_http: PrimField<String>,
    #[doc = "Git SSH hostname."]
    pub git_ssh: PrimField<String>,
    #[doc = "HTML hostname."]
    pub html: PrimField<String>,
}
impl BuildSecureSourceManagerInstancePrivateConfigElCustomHostConfigEl {
    pub fn build(self) -> SecureSourceManagerInstancePrivateConfigElCustomHostConfigEl {
        SecureSourceManagerInstancePrivateConfigElCustomHostConfigEl {
            api: self.api,
            git_http: self.git_http,
            git_ssh: self.git_ssh,
            html: self.html,
        }
    }
}
pub struct SecureSourceManagerInstancePrivateConfigElCustomHostConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecureSourceManagerInstancePrivateConfigElCustomHostConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecureSourceManagerInstancePrivateConfigElCustomHostConfigElRef {
        SecureSourceManagerInstancePrivateConfigElCustomHostConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecureSourceManagerInstancePrivateConfigElCustomHostConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api` after provisioning.\nAPI hostname."]
    pub fn api(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api", self.base))
    }
    #[doc = "Get a reference to the value of field `git_http` after provisioning.\nGit HTTP hostname."]
    pub fn git_http(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.git_http", self.base))
    }
    #[doc = "Get a reference to the value of field `git_ssh` after provisioning.\nGit SSH hostname."]
    pub fn git_ssh(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.git_ssh", self.base))
    }
    #[doc = "Get a reference to the value of field `html` after provisioning.\nHTML hostname."]
    pub fn html(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.html", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecureSourceManagerInstancePrivateConfigElDynamic {
    custom_host_config:
        Option<DynamicBlock<SecureSourceManagerInstancePrivateConfigElCustomHostConfigEl>>,
}
#[derive(Serialize)]
pub struct SecureSourceManagerInstancePrivateConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_pool: Option<PrimField<String>>,
    is_private: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_host_config: Option<Vec<SecureSourceManagerInstancePrivateConfigElCustomHostConfigEl>>,
    dynamic: SecureSourceManagerInstancePrivateConfigElDynamic,
}
impl SecureSourceManagerInstancePrivateConfigEl {
    #[doc = "Set the field `ca_pool`.\nCA pool resource, resource must in the format of 'projects/{project}/locations/{location}/caPools/{ca_pool}'."]
    pub fn set_ca_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ca_pool = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_host_config`.\n"]
    pub fn set_custom_host_config(
        mut self,
        v: impl Into<BlockAssignable<SecureSourceManagerInstancePrivateConfigElCustomHostConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_host_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_host_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecureSourceManagerInstancePrivateConfigEl {
    type O = BlockAssignable<SecureSourceManagerInstancePrivateConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecureSourceManagerInstancePrivateConfigEl {
    #[doc = "'Indicate if it's private instance.'"]
    pub is_private: PrimField<bool>,
}
impl BuildSecureSourceManagerInstancePrivateConfigEl {
    pub fn build(self) -> SecureSourceManagerInstancePrivateConfigEl {
        SecureSourceManagerInstancePrivateConfigEl {
            ca_pool: core::default::Default::default(),
            is_private: self.is_private,
            custom_host_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecureSourceManagerInstancePrivateConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecureSourceManagerInstancePrivateConfigElRef {
    fn new(shared: StackShared, base: String) -> SecureSourceManagerInstancePrivateConfigElRef {
        SecureSourceManagerInstancePrivateConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecureSourceManagerInstancePrivateConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_pool` after provisioning.\nCA pool resource, resource must in the format of 'projects/{project}/locations/{location}/caPools/{ca_pool}'."]
    pub fn ca_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ca_pool", self.base))
    }
    #[doc = "Get a reference to the value of field `http_service_attachment` after provisioning.\nService Attachment for HTTP, resource is in the format of 'projects/{project}/regions/{region}/serviceAttachments/{service_attachment}'."]
    pub fn http_service_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_service_attachment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_private` after provisioning.\n'Indicate if it's private instance.'"]
    pub fn is_private(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_private", self.base))
    }
    #[doc = "Get a reference to the value of field `ssh_service_attachment` after provisioning.\nService Attachment for SSH, resource is in the format of 'projects/{project}/regions/{region}/serviceAttachments/{service_attachment}'."]
    pub fn ssh_service_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssh_service_attachment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_host_config` after provisioning.\n"]
    pub fn custom_host_config(
        &self,
    ) -> ListRef<SecureSourceManagerInstancePrivateConfigElCustomHostConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_host_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SecureSourceManagerInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SecureSourceManagerInstanceTimeoutsEl {
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
impl ToListMappable for SecureSourceManagerInstanceTimeoutsEl {
    type O = BlockAssignable<SecureSourceManagerInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecureSourceManagerInstanceTimeoutsEl {}
impl BuildSecureSourceManagerInstanceTimeoutsEl {
    pub fn build(self) -> SecureSourceManagerInstanceTimeoutsEl {
        SecureSourceManagerInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SecureSourceManagerInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecureSourceManagerInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SecureSourceManagerInstanceTimeoutsElRef {
        SecureSourceManagerInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecureSourceManagerInstanceTimeoutsElRef {
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
pub struct SecureSourceManagerInstanceWorkforceIdentityFederationConfigEl {
    enabled: PrimField<bool>,
}
impl SecureSourceManagerInstanceWorkforceIdentityFederationConfigEl {}
impl ToListMappable for SecureSourceManagerInstanceWorkforceIdentityFederationConfigEl {
    type O = BlockAssignable<SecureSourceManagerInstanceWorkforceIdentityFederationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecureSourceManagerInstanceWorkforceIdentityFederationConfigEl {
    #[doc = "'Whether Workforce Identity Federation is enabled.'"]
    pub enabled: PrimField<bool>,
}
impl BuildSecureSourceManagerInstanceWorkforceIdentityFederationConfigEl {
    pub fn build(self) -> SecureSourceManagerInstanceWorkforceIdentityFederationConfigEl {
        SecureSourceManagerInstanceWorkforceIdentityFederationConfigEl {
            enabled: self.enabled,
        }
    }
}
pub struct SecureSourceManagerInstanceWorkforceIdentityFederationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecureSourceManagerInstanceWorkforceIdentityFederationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecureSourceManagerInstanceWorkforceIdentityFederationConfigElRef {
        SecureSourceManagerInstanceWorkforceIdentityFederationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecureSourceManagerInstanceWorkforceIdentityFederationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n'Whether Workforce Identity Federation is enabled.'"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecureSourceManagerInstanceDynamic {
    private_config: Option<DynamicBlock<SecureSourceManagerInstancePrivateConfigEl>>,
    workforce_identity_federation_config:
        Option<DynamicBlock<SecureSourceManagerInstanceWorkforceIdentityFederationConfigEl>>,
}
