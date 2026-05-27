use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataSourcerepoRepositoryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataSourcerepoRepository_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSourcerepoRepositoryData>,
}
#[derive(Clone)]
pub struct DataSourcerepoRepository(Rc<DataSourcerepoRepository_>);
impl DataSourcerepoRepository {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
    #[doc = "Get a reference to the value of field `create_ignore_already_exists` after provisioning.\nIf set to true, skip repository creation if a repository with the same name already exists."]
    pub fn create_ignore_already_exists(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_ignore_already_exists", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name of the repository, of the form '{{repo}}'.\nThe repo name may contain slashes. eg, 'name/with/slash'"]
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
    #[doc = "Get a reference to the value of field `pubsub_configs` after provisioning.\nHow this repository publishes a change in the repository through Cloud Pub/Sub.\nKeyed by the topic names."]
    pub fn pubsub_configs(&self) -> SetRef<DataSourcerepoRepositoryPubsubConfigsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.pubsub_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `size` after provisioning.\nThe disk usage of the repo, in bytes."]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nURL to clone the repository from Google Cloud Source Repositories."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}
impl Referable for DataSourcerepoRepository {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataSourcerepoRepository {}
impl ToListMappable for DataSourcerepoRepository {
    type O = ListRef<DataSourcerepoRepositoryRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataSourcerepoRepository_ {
    fn extract_datasource_type(&self) -> String {
        "google_sourcerepo_repository".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataSourcerepoRepository {
    pub tf_id: String,
    #[doc = "Resource name of the repository, of the form '{{repo}}'.\nThe repo name may contain slashes. eg, 'name/with/slash'"]
    pub name: PrimField<String>,
}
impl BuildDataSourcerepoRepository {
    pub fn build(self, stack: &mut Stack) -> DataSourcerepoRepository {
        let out = DataSourcerepoRepository(Rc::new(DataSourcerepoRepository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSourcerepoRepositoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataSourcerepoRepositoryRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSourcerepoRepositoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataSourcerepoRepositoryRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `create_ignore_already_exists` after provisioning.\nIf set to true, skip repository creation if a repository with the same name already exists."]
    pub fn create_ignore_already_exists(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_ignore_already_exists", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name of the repository, of the form '{{repo}}'.\nThe repo name may contain slashes. eg, 'name/with/slash'"]
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
    #[doc = "Get a reference to the value of field `pubsub_configs` after provisioning.\nHow this repository publishes a change in the repository through Cloud Pub/Sub.\nKeyed by the topic names."]
    pub fn pubsub_configs(&self) -> SetRef<DataSourcerepoRepositoryPubsubConfigsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.pubsub_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `size` after provisioning.\nThe disk usage of the repo, in bytes."]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nURL to clone the repository from Google Cloud Source Repositories."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}
#[derive(Serialize)]
pub struct DataSourcerepoRepositoryPubsubConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
}
impl DataSourcerepoRepositoryPubsubConfigsEl {
    #[doc = "Set the field `message_format`.\n"]
    pub fn set_message_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_format = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_email`.\n"]
    pub fn set_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `topic`.\n"]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
}
impl ToListMappable for DataSourcerepoRepositoryPubsubConfigsEl {
    type O = BlockAssignable<DataSourcerepoRepositoryPubsubConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSourcerepoRepositoryPubsubConfigsEl {}
impl BuildDataSourcerepoRepositoryPubsubConfigsEl {
    pub fn build(self) -> DataSourcerepoRepositoryPubsubConfigsEl {
        DataSourcerepoRepositoryPubsubConfigsEl {
            message_format: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}
pub struct DataSourcerepoRepositoryPubsubConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSourcerepoRepositoryPubsubConfigsElRef {
    fn new(shared: StackShared, base: String) -> DataSourcerepoRepositoryPubsubConfigsElRef {
        DataSourcerepoRepositoryPubsubConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSourcerepoRepositoryPubsubConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message_format` after provisioning.\n"]
    pub fn message_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n"]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
