use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SecureSourceManagerRepositoryData {
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
    id: Option<PrimField<String>>,
    instance: PrimField<String>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    repository_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_config: Option<Vec<SecureSourceManagerRepositoryInitialConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SecureSourceManagerRepositoryTimeoutsEl>,
    dynamic: SecureSourceManagerRepositoryDynamic,
}
struct SecureSourceManagerRepository_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecureSourceManagerRepositoryData>,
}
#[derive(Clone)]
pub struct SecureSourceManagerRepository(Rc<SecureSourceManagerRepository_>);
impl SecureSourceManagerRepository {
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
    #[doc = "Set the field `description`.\nDescription of the repository, which cannot exceed 500 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_config`.\n"]
    pub fn set_initial_config(
        self,
        v: impl Into<BlockAssignable<SecureSourceManagerRepositoryInitialConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().initial_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.initial_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SecureSourceManagerRepositoryTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the repository was created in UTC."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the repository, which cannot exceed 500 characters."]
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
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe name of the instance in which the repository is hosted."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the Repository."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the Repository."]
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
    #[doc = "Get a reference to the value of field `repository_id` after provisioning.\nThe ID for the Repository."]
    pub fn repository_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique identifier of the repository."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the repository was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uris` after provisioning.\nURIs for the repository."]
    pub fn uris(&self) -> ListRef<SecureSourceManagerRepositoryUrisElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.uris", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `initial_config` after provisioning.\n"]
    pub fn initial_config(&self) -> ListRef<SecureSourceManagerRepositoryInitialConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.initial_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecureSourceManagerRepositoryTimeoutsElRef {
        SecureSourceManagerRepositoryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for SecureSourceManagerRepository {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SecureSourceManagerRepository {}
impl ToListMappable for SecureSourceManagerRepository {
    type O = ListRef<SecureSourceManagerRepositoryRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SecureSourceManagerRepository_ {
    fn extract_resource_type(&self) -> String {
        "google_secure_source_manager_repository".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSecureSourceManagerRepository {
    pub tf_id: String,
    #[doc = "The name of the instance in which the repository is hosted."]
    pub instance: PrimField<String>,
    #[doc = "The location for the Repository."]
    pub location: PrimField<String>,
    #[doc = "The ID for the Repository."]
    pub repository_id: PrimField<String>,
}
impl BuildSecureSourceManagerRepository {
    pub fn build(self, stack: &mut Stack) -> SecureSourceManagerRepository {
        let out = SecureSourceManagerRepository(Rc::new(SecureSourceManagerRepository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecureSourceManagerRepositoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                project: core::default::Default::default(),
                repository_id: self.repository_id,
                initial_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SecureSourceManagerRepositoryRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecureSourceManagerRepositoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SecureSourceManagerRepositoryRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the repository was created in UTC."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the repository, which cannot exceed 500 characters."]
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
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe name of the instance in which the repository is hosted."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the Repository."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the Repository."]
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
    #[doc = "Get a reference to the value of field `repository_id` after provisioning.\nThe ID for the Repository."]
    pub fn repository_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique identifier of the repository."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the repository was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uris` after provisioning.\nURIs for the repository."]
    pub fn uris(&self) -> ListRef<SecureSourceManagerRepositoryUrisElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.uris", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `initial_config` after provisioning.\n"]
    pub fn initial_config(&self) -> ListRef<SecureSourceManagerRepositoryInitialConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.initial_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecureSourceManagerRepositoryTimeoutsElRef {
        SecureSourceManagerRepositoryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SecureSourceManagerRepositoryUrisEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git_https: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    html: Option<PrimField<String>>,
}
impl SecureSourceManagerRepositoryUrisEl {
    #[doc = "Set the field `api`.\n"]
    pub fn set_api(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api = Some(v.into());
        self
    }
    #[doc = "Set the field `git_https`.\n"]
    pub fn set_git_https(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.git_https = Some(v.into());
        self
    }
    #[doc = "Set the field `html`.\n"]
    pub fn set_html(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.html = Some(v.into());
        self
    }
}
impl ToListMappable for SecureSourceManagerRepositoryUrisEl {
    type O = BlockAssignable<SecureSourceManagerRepositoryUrisEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecureSourceManagerRepositoryUrisEl {}
impl BuildSecureSourceManagerRepositoryUrisEl {
    pub fn build(self) -> SecureSourceManagerRepositoryUrisEl {
        SecureSourceManagerRepositoryUrisEl {
            api: core::default::Default::default(),
            git_https: core::default::Default::default(),
            html: core::default::Default::default(),
        }
    }
}
pub struct SecureSourceManagerRepositoryUrisElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecureSourceManagerRepositoryUrisElRef {
    fn new(shared: StackShared, base: String) -> SecureSourceManagerRepositoryUrisElRef {
        SecureSourceManagerRepositoryUrisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecureSourceManagerRepositoryUrisElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api` after provisioning.\n"]
    pub fn api(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api", self.base))
    }
    #[doc = "Get a reference to the value of field `git_https` after provisioning.\n"]
    pub fn git_https(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.git_https", self.base))
    }
    #[doc = "Get a reference to the value of field `html` after provisioning.\n"]
    pub fn html(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.html", self.base))
    }
}
#[derive(Serialize)]
pub struct SecureSourceManagerRepositoryInitialConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitignores: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    readme: Option<PrimField<String>>,
}
impl SecureSourceManagerRepositoryInitialConfigEl {
    #[doc = "Set the field `default_branch`.\nDefault branch name of the repository."]
    pub fn set_default_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_branch = Some(v.into());
        self
    }
    #[doc = "Set the field `gitignores`.\nList of gitignore template names user can choose from.\nValid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig."]
    pub fn set_gitignores(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.gitignores = Some(v.into());
        self
    }
    #[doc = "Set the field `license`.\nLicense template name user can choose from.\nValid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig."]
    pub fn set_license(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.license = Some(v.into());
        self
    }
    #[doc = "Set the field `readme`.\nREADME template name.\nValid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig."]
    pub fn set_readme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.readme = Some(v.into());
        self
    }
}
impl ToListMappable for SecureSourceManagerRepositoryInitialConfigEl {
    type O = BlockAssignable<SecureSourceManagerRepositoryInitialConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecureSourceManagerRepositoryInitialConfigEl {}
impl BuildSecureSourceManagerRepositoryInitialConfigEl {
    pub fn build(self) -> SecureSourceManagerRepositoryInitialConfigEl {
        SecureSourceManagerRepositoryInitialConfigEl {
            default_branch: core::default::Default::default(),
            gitignores: core::default::Default::default(),
            license: core::default::Default::default(),
            readme: core::default::Default::default(),
        }
    }
}
pub struct SecureSourceManagerRepositoryInitialConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecureSourceManagerRepositoryInitialConfigElRef {
    fn new(shared: StackShared, base: String) -> SecureSourceManagerRepositoryInitialConfigElRef {
        SecureSourceManagerRepositoryInitialConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecureSourceManagerRepositoryInitialConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_branch` after provisioning.\nDefault branch name of the repository."]
    pub fn default_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_branch", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gitignores` after provisioning.\nList of gitignore template names user can choose from.\nValid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig."]
    pub fn gitignores(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.gitignores", self.base))
    }
    #[doc = "Get a reference to the value of field `license` after provisioning.\nLicense template name user can choose from.\nValid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig."]
    pub fn license(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.license", self.base))
    }
    #[doc = "Get a reference to the value of field `readme` after provisioning.\nREADME template name.\nValid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig."]
    pub fn readme(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.readme", self.base))
    }
}
#[derive(Serialize)]
pub struct SecureSourceManagerRepositoryTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SecureSourceManagerRepositoryTimeoutsEl {
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
impl ToListMappable for SecureSourceManagerRepositoryTimeoutsEl {
    type O = BlockAssignable<SecureSourceManagerRepositoryTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecureSourceManagerRepositoryTimeoutsEl {}
impl BuildSecureSourceManagerRepositoryTimeoutsEl {
    pub fn build(self) -> SecureSourceManagerRepositoryTimeoutsEl {
        SecureSourceManagerRepositoryTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SecureSourceManagerRepositoryTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecureSourceManagerRepositoryTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SecureSourceManagerRepositoryTimeoutsElRef {
        SecureSourceManagerRepositoryTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecureSourceManagerRepositoryTimeoutsElRef {
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
struct SecureSourceManagerRepositoryDynamic {
    initial_config: Option<DynamicBlock<SecureSourceManagerRepositoryInitialConfigEl>>,
}
