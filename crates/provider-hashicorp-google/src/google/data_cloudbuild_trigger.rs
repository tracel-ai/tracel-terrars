use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudbuildTriggerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    trigger_id: PrimField<String>,
}
struct DataCloudbuildTrigger_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudbuildTriggerData>,
}
#[derive(Clone)]
pub struct DataCloudbuildTrigger(Rc<DataCloudbuildTrigger_>);
impl DataCloudbuildTrigger {
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
    #[doc = "Get a reference to the value of field `approval_config` after provisioning.\nConfiguration for manual approval to start a build invocation of this BuildTrigger.\nBuilds created by this trigger will require approval before they execute.\nAny user with a Cloud Build Approver role for the project can approve a build."]
    pub fn approval_config(&self) -> ListRef<DataCloudbuildTriggerApprovalConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.approval_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bitbucket_server_trigger_config` after provisioning.\nBitbucketServerTriggerConfig describes the configuration of a trigger that creates a build whenever a Bitbucket Server event is received."]
    pub fn bitbucket_server_trigger_config(
        &self,
    ) -> ListRef<DataCloudbuildTriggerBitbucketServerTriggerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_server_trigger_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build` after provisioning.\nContents of the build template. Either a filename or build template must be provided."]
    pub fn build(&self) -> ListRef<DataCloudbuildTriggerBuildElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.build", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime when the trigger was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the trigger."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `developer_connect_event_config` after provisioning.\nConfiguration for triggers that respond to Developer Connect events."]
    pub fn developer_connect_event_config(
        &self,
    ) -> ListRef<DataCloudbuildTriggerDeveloperConnectEventConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.developer_connect_event_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the trigger is disabled or not. If true, the trigger will never result in a build."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filename` after provisioning.\nPath, from the source root, to a file whose contents is used for the template.\nEither a filename or build template must be provided. Set this only when using trigger_template or github.\nWhen using Pub/Sub, Webhook or Manual set the file name using git_file_source instead."]
    pub fn filename(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filename", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nA Common Expression Language string. Used only with Pub/Sub and Webhook."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `git_file_source` after provisioning.\nThe file source describing the local or remote Build template."]
    pub fn git_file_source(&self) -> ListRef<DataCloudbuildTriggerGitFileSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.git_file_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `github` after provisioning.\nDescribes the configuration of a trigger that creates a build whenever a GitHub event is received."]
    pub fn github(&self) -> ListRef<DataCloudbuildTriggerGithubElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.github", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignored_files` after provisioning.\nignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match\nextended with support for '**'.\n\nIf ignoredFiles and changed files are both empty, then they are not\nused to determine whether or not to trigger a build.\n\nIf ignoredFiles is not empty, then we ignore any files that match any\nof the ignored_file globs. If the change has no files that are outside\nof the ignoredFiles globs, then we do not trigger a build."]
    pub fn ignored_files(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ignored_files", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_build_logs` after provisioning.\nBuild logs will be sent back to GitHub as part of the checkrun\nresult.  Values can be INCLUDE_BUILD_LOGS_UNSPECIFIED or\nINCLUDE_BUILD_LOGS_WITH_STATUS Possible values: [\"INCLUDE_BUILD_LOGS_UNSPECIFIED\", \"INCLUDE_BUILD_LOGS_WITH_STATUS\"]"]
    pub fn include_build_logs(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_build_logs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `included_files` after provisioning.\nignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match\nextended with support for '**'.\n\nIf any of the files altered in the commit pass the ignoredFiles filter\nand includedFiles is empty, then as far as this filter is concerned, we\nshould trigger the build.\n\nIf any of the files altered in the commit pass the ignoredFiles filter\nand includedFiles is not empty, then we make sure that at least one of\nthose files matches a includedFiles glob. If not, then we do not trigger\na build."]
    pub fn included_files(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_files", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe [Cloud Build location](https://cloud.google.com/build/docs/locations) for the trigger.\nIf not specified, \"global\" is used."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the trigger. Must be unique within the project."]
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
    #[doc = "Get a reference to the value of field `pubsub_config` after provisioning.\nPubsubConfig describes the configuration of a trigger that creates\na build whenever a Pub/Sub message is published."]
    pub fn pubsub_config(&self) -> ListRef<DataCloudbuildTriggerPubsubConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pubsub_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_event_config` after provisioning.\nThe configuration of a trigger that creates a build whenever an event from Repo API is received."]
    pub fn repository_event_config(
        &self,
    ) -> ListRef<DataCloudbuildTriggerRepositoryEventConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.repository_event_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe service account used for all user-controlled operations including\ntriggers.patch, triggers.run, builds.create, and builds.cancel.\n\nIf no service account is set, then the standard Cloud Build service account\n([PROJECT_NUM]@system.gserviceaccount.com) will be used instead.\n\nFormat: projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_ID_OR_EMAIL}"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_to_build` after provisioning.\nThe repo and ref of the repository from which to build.\nThis field is used only for those triggers that do not respond to SCM events.\nTriggers that respond to such events build source at whatever commit caused the event.\nThis field is currently only used by Webhook, Pub/Sub, Manual, and Cron triggers."]
    pub fn source_to_build(&self) -> ListRef<DataCloudbuildTriggerSourceToBuildElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_to_build", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `substitutions` after provisioning.\nSubstitutions data for Build resource."]
    pub fn substitutions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.substitutions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags for annotation of a BuildTrigger"]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_id` after provisioning.\nThe unique identifier for the trigger."]
    pub fn trigger_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trigger_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_template` after provisioning.\nTemplate describing the types of source changes to trigger a build.\n\nBranch and tag names in trigger templates are interpreted as regular\nexpressions. Any branch or tag change that matches that regular\nexpression will trigger a build."]
    pub fn trigger_template(&self) -> ListRef<DataCloudbuildTriggerTriggerTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trigger_template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `webhook_config` after provisioning.\nWebhookConfig describes the configuration of a trigger that creates\na build whenever a webhook is sent to a trigger's webhook URL."]
    pub fn webhook_config(&self) -> ListRef<DataCloudbuildTriggerWebhookConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.webhook_config", self.extract_ref()),
        )
    }
}
impl Referable for DataCloudbuildTrigger {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudbuildTrigger {}
impl ToListMappable for DataCloudbuildTrigger {
    type O = ListRef<DataCloudbuildTriggerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudbuildTrigger_ {
    fn extract_datasource_type(&self) -> String {
        "google_cloudbuild_trigger".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudbuildTrigger {
    pub tf_id: String,
    #[doc = "The [Cloud Build location](https://cloud.google.com/build/docs/locations) for the trigger.\nIf not specified, \"global\" is used."]
    pub location: PrimField<String>,
    #[doc = "The unique identifier for the trigger."]
    pub trigger_id: PrimField<String>,
}
impl BuildDataCloudbuildTrigger {
    pub fn build(self, stack: &mut Stack) -> DataCloudbuildTrigger {
        let out = DataCloudbuildTrigger(Rc::new(DataCloudbuildTrigger_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudbuildTriggerData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                trigger_id: self.trigger_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCloudbuildTriggerRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudbuildTriggerRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `approval_config` after provisioning.\nConfiguration for manual approval to start a build invocation of this BuildTrigger.\nBuilds created by this trigger will require approval before they execute.\nAny user with a Cloud Build Approver role for the project can approve a build."]
    pub fn approval_config(&self) -> ListRef<DataCloudbuildTriggerApprovalConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.approval_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bitbucket_server_trigger_config` after provisioning.\nBitbucketServerTriggerConfig describes the configuration of a trigger that creates a build whenever a Bitbucket Server event is received."]
    pub fn bitbucket_server_trigger_config(
        &self,
    ) -> ListRef<DataCloudbuildTriggerBitbucketServerTriggerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bitbucket_server_trigger_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `build` after provisioning.\nContents of the build template. Either a filename or build template must be provided."]
    pub fn build(&self) -> ListRef<DataCloudbuildTriggerBuildElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.build", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime when the trigger was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the trigger."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `developer_connect_event_config` after provisioning.\nConfiguration for triggers that respond to Developer Connect events."]
    pub fn developer_connect_event_config(
        &self,
    ) -> ListRef<DataCloudbuildTriggerDeveloperConnectEventConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.developer_connect_event_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the trigger is disabled or not. If true, the trigger will never result in a build."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filename` after provisioning.\nPath, from the source root, to a file whose contents is used for the template.\nEither a filename or build template must be provided. Set this only when using trigger_template or github.\nWhen using Pub/Sub, Webhook or Manual set the file name using git_file_source instead."]
    pub fn filename(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filename", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nA Common Expression Language string. Used only with Pub/Sub and Webhook."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `git_file_source` after provisioning.\nThe file source describing the local or remote Build template."]
    pub fn git_file_source(&self) -> ListRef<DataCloudbuildTriggerGitFileSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.git_file_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `github` after provisioning.\nDescribes the configuration of a trigger that creates a build whenever a GitHub event is received."]
    pub fn github(&self) -> ListRef<DataCloudbuildTriggerGithubElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.github", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignored_files` after provisioning.\nignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match\nextended with support for '**'.\n\nIf ignoredFiles and changed files are both empty, then they are not\nused to determine whether or not to trigger a build.\n\nIf ignoredFiles is not empty, then we ignore any files that match any\nof the ignored_file globs. If the change has no files that are outside\nof the ignoredFiles globs, then we do not trigger a build."]
    pub fn ignored_files(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ignored_files", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_build_logs` after provisioning.\nBuild logs will be sent back to GitHub as part of the checkrun\nresult.  Values can be INCLUDE_BUILD_LOGS_UNSPECIFIED or\nINCLUDE_BUILD_LOGS_WITH_STATUS Possible values: [\"INCLUDE_BUILD_LOGS_UNSPECIFIED\", \"INCLUDE_BUILD_LOGS_WITH_STATUS\"]"]
    pub fn include_build_logs(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_build_logs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `included_files` after provisioning.\nignoredFiles and includedFiles are file glob matches using https://golang.org/pkg/path/filepath/#Match\nextended with support for '**'.\n\nIf any of the files altered in the commit pass the ignoredFiles filter\nand includedFiles is empty, then as far as this filter is concerned, we\nshould trigger the build.\n\nIf any of the files altered in the commit pass the ignoredFiles filter\nand includedFiles is not empty, then we make sure that at least one of\nthose files matches a includedFiles glob. If not, then we do not trigger\na build."]
    pub fn included_files(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_files", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe [Cloud Build location](https://cloud.google.com/build/docs/locations) for the trigger.\nIf not specified, \"global\" is used."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the trigger. Must be unique within the project."]
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
    #[doc = "Get a reference to the value of field `pubsub_config` after provisioning.\nPubsubConfig describes the configuration of a trigger that creates\na build whenever a Pub/Sub message is published."]
    pub fn pubsub_config(&self) -> ListRef<DataCloudbuildTriggerPubsubConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pubsub_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_event_config` after provisioning.\nThe configuration of a trigger that creates a build whenever an event from Repo API is received."]
    pub fn repository_event_config(
        &self,
    ) -> ListRef<DataCloudbuildTriggerRepositoryEventConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.repository_event_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe service account used for all user-controlled operations including\ntriggers.patch, triggers.run, builds.create, and builds.cancel.\n\nIf no service account is set, then the standard Cloud Build service account\n([PROJECT_NUM]@system.gserviceaccount.com) will be used instead.\n\nFormat: projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_ID_OR_EMAIL}"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_to_build` after provisioning.\nThe repo and ref of the repository from which to build.\nThis field is used only for those triggers that do not respond to SCM events.\nTriggers that respond to such events build source at whatever commit caused the event.\nThis field is currently only used by Webhook, Pub/Sub, Manual, and Cron triggers."]
    pub fn source_to_build(&self) -> ListRef<DataCloudbuildTriggerSourceToBuildElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_to_build", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `substitutions` after provisioning.\nSubstitutions data for Build resource."]
    pub fn substitutions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.substitutions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags for annotation of a BuildTrigger"]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_id` after provisioning.\nThe unique identifier for the trigger."]
    pub fn trigger_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trigger_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trigger_template` after provisioning.\nTemplate describing the types of source changes to trigger a build.\n\nBranch and tag names in trigger templates are interpreted as regular\nexpressions. Any branch or tag change that matches that regular\nexpression will trigger a build."]
    pub fn trigger_template(&self) -> ListRef<DataCloudbuildTriggerTriggerTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trigger_template", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `webhook_config` after provisioning.\nWebhookConfig describes the configuration of a trigger that creates\na build whenever a webhook is sent to a trigger's webhook URL."]
    pub fn webhook_config(&self) -> ListRef<DataCloudbuildTriggerWebhookConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.webhook_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerApprovalConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    approval_required: Option<PrimField<bool>>,
}
impl DataCloudbuildTriggerApprovalConfigEl {
    #[doc = "Set the field `approval_required`.\n"]
    pub fn set_approval_required(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.approval_required = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerApprovalConfigEl {
    type O = BlockAssignable<DataCloudbuildTriggerApprovalConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerApprovalConfigEl {}
impl BuildDataCloudbuildTriggerApprovalConfigEl {
    pub fn build(self) -> DataCloudbuildTriggerApprovalConfigEl {
        DataCloudbuildTriggerApprovalConfigEl {
            approval_required: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerApprovalConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerApprovalConfigElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerApprovalConfigElRef {
        DataCloudbuildTriggerApprovalConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerApprovalConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `approval_required` after provisioning.\n"]
    pub fn approval_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approval_required", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
}
impl DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }
    #[doc = "Set the field `comment_control`.\n"]
    pub fn set_comment_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comment_control = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\n"]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
    type O = BlockAssignable<DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {}
impl BuildDataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
    pub fn build(self) -> DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
        DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl {
            branch: core::default::Default::default(),
            comment_control: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef {
        DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }
    #[doc = "Get a reference to the value of field `comment_control` after provisioning.\n"]
    pub fn comment_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comment_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\n"]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\n"]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
    type O = BlockAssignable<DataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl {}
impl BuildDataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
    pub fn build(self) -> DataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
        DataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl {
            branch: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBitbucketServerTriggerConfigElPushElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBitbucketServerTriggerConfigElPushElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudbuildTriggerBitbucketServerTriggerConfigElPushElRef {
        DataCloudbuildTriggerBitbucketServerTriggerConfigElPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBitbucketServerTriggerConfigElPushElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\n"]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBitbucketServerTriggerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bitbucket_server_config_resource: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request:
        Option<ListField<DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push: Option<ListField<DataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_slug: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerBitbucketServerTriggerConfigEl {
    #[doc = "Set the field `bitbucket_server_config_resource`.\n"]
    pub fn set_bitbucket_server_config_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bitbucket_server_config_resource = Some(v.into());
        self
    }
    #[doc = "Set the field `project_key`.\n"]
    pub fn set_project_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_key = Some(v.into());
        self
    }
    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestEl>>,
    ) -> Self {
        self.pull_request = Some(v.into());
        self
    }
    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerBitbucketServerTriggerConfigElPushEl>>,
    ) -> Self {
        self.push = Some(v.into());
        self
    }
    #[doc = "Set the field `repo_slug`.\n"]
    pub fn set_repo_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_slug = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBitbucketServerTriggerConfigEl {
    type O = BlockAssignable<DataCloudbuildTriggerBitbucketServerTriggerConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBitbucketServerTriggerConfigEl {}
impl BuildDataCloudbuildTriggerBitbucketServerTriggerConfigEl {
    pub fn build(self) -> DataCloudbuildTriggerBitbucketServerTriggerConfigEl {
        DataCloudbuildTriggerBitbucketServerTriggerConfigEl {
            bitbucket_server_config_resource: core::default::Default::default(),
            project_key: core::default::Default::default(),
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            repo_slug: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBitbucketServerTriggerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBitbucketServerTriggerConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudbuildTriggerBitbucketServerTriggerConfigElRef {
        DataCloudbuildTriggerBitbucketServerTriggerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBitbucketServerTriggerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bitbucket_server_config_resource` after provisioning.\n"]
    pub fn bitbucket_server_config_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bitbucket_server_config_resource", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `project_key` after provisioning.\n"]
    pub fn project_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_key", self.base))
    }
    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(
        &self,
    ) -> ListRef<DataCloudbuildTriggerBitbucketServerTriggerConfigElPullRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }
    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> ListRef<DataCloudbuildTriggerBitbucketServerTriggerConfigElPushElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }
    #[doc = "Get a reference to the value of field `repo_slug` after provisioning.\n"]
    pub fn repo_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_slug", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    artifact_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
    #[doc = "Set the field `artifact_id`.\n"]
    pub fn set_artifact_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.artifact_id = Some(v.into());
        self
    }
    #[doc = "Set the field `group_id`.\n"]
    pub fn set_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_id = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
    type O = BlockAssignable<DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {}
impl BuildDataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
        DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl {
            artifact_id: core::default::Default::default(),
            group_id: core::default::Default::default(),
            path: core::default::Default::default(),
            repository: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef {
        DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `artifact_id` after provisioning.\n"]
    pub fn artifact_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.artifact_id", self.base))
    }
    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    package_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
    #[doc = "Set the field `package_path`.\n"]
    pub fn set_package_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.package_path = Some(v.into());
        self
    }
    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
    type O = BlockAssignable<DataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl {}
impl BuildDataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
        DataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl {
            package_path: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBuildElArtifactsElNpmPackagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBuildElArtifactsElNpmPackagesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudbuildTriggerBuildElArtifactsElNpmPackagesElRef {
        DataCloudbuildTriggerBuildElArtifactsElNpmPackagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBuildElArtifactsElNpmPackagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `package_path` after provisioning.\n"]
    pub fn package_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.package_path", self.base))
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
    type O = BlockAssignable<DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {}
impl BuildDataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
        DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef {
        DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElArtifactsElObjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paths: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timing: Option<ListField<DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl>>,
}
impl DataCloudbuildTriggerBuildElArtifactsElObjectsEl {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `paths`.\n"]
    pub fn set_paths(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.paths = Some(v.into());
        self
    }
    #[doc = "Set the field `timing`.\n"]
    pub fn set_timing(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingEl>>,
    ) -> Self {
        self.timing = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBuildElArtifactsElObjectsEl {
    type O = BlockAssignable<DataCloudbuildTriggerBuildElArtifactsElObjectsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBuildElArtifactsElObjectsEl {}
impl BuildDataCloudbuildTriggerBuildElArtifactsElObjectsEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElArtifactsElObjectsEl {
        DataCloudbuildTriggerBuildElArtifactsElObjectsEl {
            location: core::default::Default::default(),
            paths: core::default::Default::default(),
            timing: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBuildElArtifactsElObjectsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBuildElArtifactsElObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudbuildTriggerBuildElArtifactsElObjectsElRef {
        DataCloudbuildTriggerBuildElArtifactsElObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBuildElArtifactsElObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `paths` after provisioning.\n"]
    pub fn paths(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }
    #[doc = "Get a reference to the value of field `timing` after provisioning.\n"]
    pub fn timing(&self) -> ListRef<DataCloudbuildTriggerBuildElArtifactsElObjectsElTimingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timing", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    paths: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
    #[doc = "Set the field `paths`.\n"]
    pub fn set_paths(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.paths = Some(v.into());
        self
    }
    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
    type O = BlockAssignable<DataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl {}
impl BuildDataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
        DataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl {
            paths: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBuildElArtifactsElPythonPackagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBuildElArtifactsElPythonPackagesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudbuildTriggerBuildElArtifactsElPythonPackagesElRef {
        DataCloudbuildTriggerBuildElArtifactsElPythonPackagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBuildElArtifactsElPythonPackagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `paths` after provisioning.\n"]
    pub fn paths(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElArtifactsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maven_artifacts: Option<ListField<DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    npm_packages: Option<ListField<DataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    objects: Option<ListField<DataCloudbuildTriggerBuildElArtifactsElObjectsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_packages: Option<ListField<DataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl>>,
}
impl DataCloudbuildTriggerBuildElArtifactsEl {
    #[doc = "Set the field `images`.\n"]
    pub fn set_images(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.images = Some(v.into());
        self
    }
    #[doc = "Set the field `maven_artifacts`.\n"]
    pub fn set_maven_artifacts(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsEl>>,
    ) -> Self {
        self.maven_artifacts = Some(v.into());
        self
    }
    #[doc = "Set the field `npm_packages`.\n"]
    pub fn set_npm_packages(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerBuildElArtifactsElNpmPackagesEl>>,
    ) -> Self {
        self.npm_packages = Some(v.into());
        self
    }
    #[doc = "Set the field `objects`.\n"]
    pub fn set_objects(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerBuildElArtifactsElObjectsEl>>,
    ) -> Self {
        self.objects = Some(v.into());
        self
    }
    #[doc = "Set the field `python_packages`.\n"]
    pub fn set_python_packages(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerBuildElArtifactsElPythonPackagesEl>>,
    ) -> Self {
        self.python_packages = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBuildElArtifactsEl {
    type O = BlockAssignable<DataCloudbuildTriggerBuildElArtifactsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBuildElArtifactsEl {}
impl BuildDataCloudbuildTriggerBuildElArtifactsEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElArtifactsEl {
        DataCloudbuildTriggerBuildElArtifactsEl {
            images: core::default::Default::default(),
            maven_artifacts: core::default::Default::default(),
            npm_packages: core::default::Default::default(),
            objects: core::default::Default::default(),
            python_packages: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBuildElArtifactsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBuildElArtifactsElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerBuildElArtifactsElRef {
        DataCloudbuildTriggerBuildElArtifactsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBuildElArtifactsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `images` after provisioning.\n"]
    pub fn images(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.images", self.base))
    }
    #[doc = "Get a reference to the value of field `maven_artifacts` after provisioning.\n"]
    pub fn maven_artifacts(
        &self,
    ) -> ListRef<DataCloudbuildTriggerBuildElArtifactsElMavenArtifactsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.maven_artifacts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `npm_packages` after provisioning.\n"]
    pub fn npm_packages(&self) -> ListRef<DataCloudbuildTriggerBuildElArtifactsElNpmPackagesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.npm_packages", self.base))
    }
    #[doc = "Get a reference to the value of field `objects` after provisioning.\n"]
    pub fn objects(&self) -> ListRef<DataCloudbuildTriggerBuildElArtifactsElObjectsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.objects", self.base))
    }
    #[doc = "Get a reference to the value of field `python_packages` after provisioning.\n"]
    pub fn python_packages(
        &self,
    ) -> ListRef<DataCloudbuildTriggerBuildElArtifactsElPythonPackagesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.python_packages", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_name: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.env = Some(v.into());
        self
    }
    #[doc = "Set the field `version_name`.\n"]
    pub fn set_version_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
    type O = BlockAssignable<DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {}
impl BuildDataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
        DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl {
            env: core::default::Default::default(),
            version_name: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef {
        DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.env", self.base))
    }
    #[doc = "Get a reference to the value of field `version_name` after provisioning.\n"]
    pub fn version_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElAvailableSecretsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_manager:
        Option<ListField<DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl>>,
}
impl DataCloudbuildTriggerBuildElAvailableSecretsEl {
    #[doc = "Set the field `secret_manager`.\n"]
    pub fn set_secret_manager(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerEl>>,
    ) -> Self {
        self.secret_manager = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBuildElAvailableSecretsEl {
    type O = BlockAssignable<DataCloudbuildTriggerBuildElAvailableSecretsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBuildElAvailableSecretsEl {}
impl BuildDataCloudbuildTriggerBuildElAvailableSecretsEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElAvailableSecretsEl {
        DataCloudbuildTriggerBuildElAvailableSecretsEl {
            secret_manager: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBuildElAvailableSecretsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBuildElAvailableSecretsElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerBuildElAvailableSecretsElRef {
        DataCloudbuildTriggerBuildElAvailableSecretsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBuildElAvailableSecretsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_manager` after provisioning.\n"]
    pub fn secret_manager(
        &self,
    ) -> ListRef<DataCloudbuildTriggerBuildElAvailableSecretsElSecretManagerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secret_manager", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElOptionsElVolumesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerBuildElOptionsElVolumesEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBuildElOptionsElVolumesEl {
    type O = BlockAssignable<DataCloudbuildTriggerBuildElOptionsElVolumesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBuildElOptionsElVolumesEl {}
impl BuildDataCloudbuildTriggerBuildElOptionsElVolumesEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElOptionsElVolumesEl {
        DataCloudbuildTriggerBuildElOptionsElVolumesEl {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBuildElOptionsElVolumesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBuildElOptionsElVolumesElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerBuildElOptionsElVolumesElRef {
        DataCloudbuildTriggerBuildElOptionsElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBuildElOptionsElVolumesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynamic_substitutions: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_streaming_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requested_verify_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_env: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_provenance_hash: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    substitution_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes: Option<ListField<DataCloudbuildTriggerBuildElOptionsElVolumesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_pool: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerBuildElOptionsEl {
    #[doc = "Set the field `disk_size_gb`.\n"]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `dynamic_substitutions`.\n"]
    pub fn set_dynamic_substitutions(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.dynamic_substitutions = Some(v.into());
        self
    }
    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.env = Some(v.into());
        self
    }
    #[doc = "Set the field `log_streaming_option`.\n"]
    pub fn set_log_streaming_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_streaming_option = Some(v.into());
        self
    }
    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logging = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `requested_verify_option`.\n"]
    pub fn set_requested_verify_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.requested_verify_option = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_env`.\n"]
    pub fn set_secret_env(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.secret_env = Some(v.into());
        self
    }
    #[doc = "Set the field `source_provenance_hash`.\n"]
    pub fn set_source_provenance_hash(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.source_provenance_hash = Some(v.into());
        self
    }
    #[doc = "Set the field `substitution_option`.\n"]
    pub fn set_substitution_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.substitution_option = Some(v.into());
        self
    }
    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerBuildElOptionsElVolumesEl>>,
    ) -> Self {
        self.volumes = Some(v.into());
        self
    }
    #[doc = "Set the field `worker_pool`.\n"]
    pub fn set_worker_pool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.worker_pool = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBuildElOptionsEl {
    type O = BlockAssignable<DataCloudbuildTriggerBuildElOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBuildElOptionsEl {}
impl BuildDataCloudbuildTriggerBuildElOptionsEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElOptionsEl {
        DataCloudbuildTriggerBuildElOptionsEl {
            disk_size_gb: core::default::Default::default(),
            dynamic_substitutions: core::default::Default::default(),
            env: core::default::Default::default(),
            log_streaming_option: core::default::Default::default(),
            logging: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            requested_verify_option: core::default::Default::default(),
            secret_env: core::default::Default::default(),
            source_provenance_hash: core::default::Default::default(),
            substitution_option: core::default::Default::default(),
            volumes: core::default::Default::default(),
            worker_pool: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBuildElOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBuildElOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerBuildElOptionsElRef {
        DataCloudbuildTriggerBuildElOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBuildElOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\n"]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `dynamic_substitutions` after provisioning.\n"]
    pub fn dynamic_substitutions(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dynamic_substitutions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }
    #[doc = "Get a reference to the value of field `log_streaming_option` after provisioning.\n"]
    pub fn log_streaming_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_streaming_option", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logging", self.base))
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `requested_verify_option` after provisioning.\n"]
    pub fn requested_verify_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requested_verify_option", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_env` after provisioning.\n"]
    pub fn secret_env(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.secret_env", self.base))
    }
    #[doc = "Get a reference to the value of field `source_provenance_hash` after provisioning.\n"]
    pub fn source_provenance_hash(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_provenance_hash", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `substitution_option` after provisioning.\n"]
    pub fn substitution_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.substitution_option", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> ListRef<DataCloudbuildTriggerBuildElOptionsElVolumesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
    #[doc = "Get a reference to the value of field `worker_pool` after provisioning.\n"]
    pub fn worker_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_pool", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElSecretEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_env: Option<RecField<PrimField<String>>>,
}
impl DataCloudbuildTriggerBuildElSecretEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_env`.\n"]
    pub fn set_secret_env(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.secret_env = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBuildElSecretEl {
    type O = BlockAssignable<DataCloudbuildTriggerBuildElSecretEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBuildElSecretEl {}
impl BuildDataCloudbuildTriggerBuildElSecretEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElSecretEl {
        DataCloudbuildTriggerBuildElSecretEl {
            kms_key_name: core::default::Default::default(),
            secret_env: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBuildElSecretElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBuildElSecretElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerBuildElSecretElRef {
        DataCloudbuildTriggerBuildElSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBuildElSecretElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_env` after provisioning.\n"]
    pub fn secret_env(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.secret_env", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElSourceElRepoSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_sha: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    substitutions: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_name: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerBuildElSourceElRepoSourceEl {
    #[doc = "Set the field `branch_name`.\n"]
    pub fn set_branch_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch_name = Some(v.into());
        self
    }
    #[doc = "Set the field `commit_sha`.\n"]
    pub fn set_commit_sha(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.commit_sha = Some(v.into());
        self
    }
    #[doc = "Set the field `dir`.\n"]
    pub fn set_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dir = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\n"]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `repo_name`.\n"]
    pub fn set_repo_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_name = Some(v.into());
        self
    }
    #[doc = "Set the field `substitutions`.\n"]
    pub fn set_substitutions(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.substitutions = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_name`.\n"]
    pub fn set_tag_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBuildElSourceElRepoSourceEl {
    type O = BlockAssignable<DataCloudbuildTriggerBuildElSourceElRepoSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBuildElSourceElRepoSourceEl {}
impl BuildDataCloudbuildTriggerBuildElSourceElRepoSourceEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElSourceElRepoSourceEl {
        DataCloudbuildTriggerBuildElSourceElRepoSourceEl {
            branch_name: core::default::Default::default(),
            commit_sha: core::default::Default::default(),
            dir: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            project_id: core::default::Default::default(),
            repo_name: core::default::Default::default(),
            substitutions: core::default::Default::default(),
            tag_name: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBuildElSourceElRepoSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBuildElSourceElRepoSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudbuildTriggerBuildElSourceElRepoSourceElRef {
        DataCloudbuildTriggerBuildElSourceElRepoSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBuildElSourceElRepoSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch_name` after provisioning.\n"]
    pub fn branch_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }
    #[doc = "Get a reference to the value of field `commit_sha` after provisioning.\n"]
    pub fn commit_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.base))
    }
    #[doc = "Get a reference to the value of field `dir` after provisioning.\n"]
    pub fn dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\n"]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `repo_name` after provisioning.\n"]
    pub fn repo_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_name", self.base))
    }
    #[doc = "Get a reference to the value of field `substitutions` after provisioning.\n"]
    pub fn substitutions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.substitutions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag_name` after provisioning.\n"]
    pub fn tag_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElSourceElStorageSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerBuildElSourceElStorageSourceEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `generation`.\n"]
    pub fn set_generation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.generation = Some(v.into());
        self
    }
    #[doc = "Set the field `object`.\n"]
    pub fn set_object(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.object = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBuildElSourceElStorageSourceEl {
    type O = BlockAssignable<DataCloudbuildTriggerBuildElSourceElStorageSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBuildElSourceElStorageSourceEl {}
impl BuildDataCloudbuildTriggerBuildElSourceElStorageSourceEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElSourceElStorageSourceEl {
        DataCloudbuildTriggerBuildElSourceElStorageSourceEl {
            bucket: core::default::Default::default(),
            generation: core::default::Default::default(),
            object: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBuildElSourceElStorageSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBuildElSourceElStorageSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudbuildTriggerBuildElSourceElStorageSourceElRef {
        DataCloudbuildTriggerBuildElSourceElStorageSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBuildElSourceElStorageSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\n"]
    pub fn generation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }
    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_source: Option<ListField<DataCloudbuildTriggerBuildElSourceElRepoSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_source: Option<ListField<DataCloudbuildTriggerBuildElSourceElStorageSourceEl>>,
}
impl DataCloudbuildTriggerBuildElSourceEl {
    #[doc = "Set the field `repo_source`.\n"]
    pub fn set_repo_source(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerBuildElSourceElRepoSourceEl>>,
    ) -> Self {
        self.repo_source = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_source`.\n"]
    pub fn set_storage_source(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerBuildElSourceElStorageSourceEl>>,
    ) -> Self {
        self.storage_source = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBuildElSourceEl {
    type O = BlockAssignable<DataCloudbuildTriggerBuildElSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBuildElSourceEl {}
impl BuildDataCloudbuildTriggerBuildElSourceEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElSourceEl {
        DataCloudbuildTriggerBuildElSourceEl {
            repo_source: core::default::Default::default(),
            storage_source: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBuildElSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBuildElSourceElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerBuildElSourceElRef {
        DataCloudbuildTriggerBuildElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBuildElSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repo_source` after provisioning.\n"]
    pub fn repo_source(&self) -> ListRef<DataCloudbuildTriggerBuildElSourceElRepoSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.repo_source", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_source` after provisioning.\n"]
    pub fn storage_source(
        &self,
    ) -> ListRef<DataCloudbuildTriggerBuildElSourceElStorageSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_source", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElStepElVolumesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerBuildElStepElVolumesEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBuildElStepElVolumesEl {
    type O = BlockAssignable<DataCloudbuildTriggerBuildElStepElVolumesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBuildElStepElVolumesEl {}
impl BuildDataCloudbuildTriggerBuildElStepElVolumesEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElStepElVolumesEl {
        DataCloudbuildTriggerBuildElStepElVolumesEl {
            name: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBuildElStepElVolumesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBuildElStepElVolumesElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerBuildElStepElVolumesElRef {
        DataCloudbuildTriggerBuildElStepElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBuildElStepElVolumesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildElStepEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_exit_codes: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_failure: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entrypoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    script: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_env: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes: Option<ListField<DataCloudbuildTriggerBuildElStepElVolumesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for: Option<ListField<PrimField<String>>>,
}
impl DataCloudbuildTriggerBuildElStepEl {
    #[doc = "Set the field `allow_exit_codes`.\n"]
    pub fn set_allow_exit_codes(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.allow_exit_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_failure`.\n"]
    pub fn set_allow_failure(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_failure = Some(v.into());
        self
    }
    #[doc = "Set the field `args`.\n"]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `dir`.\n"]
    pub fn set_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dir = Some(v.into());
        self
    }
    #[doc = "Set the field `entrypoint`.\n"]
    pub fn set_entrypoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entrypoint = Some(v.into());
        self
    }
    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.env = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `script`.\n"]
    pub fn set_script(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.script = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_env`.\n"]
    pub fn set_secret_env(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.secret_env = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `timing`.\n"]
    pub fn set_timing(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timing = Some(v.into());
        self
    }
    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerBuildElStepElVolumesEl>>,
    ) -> Self {
        self.volumes = Some(v.into());
        self
    }
    #[doc = "Set the field `wait_for`.\n"]
    pub fn set_wait_for(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.wait_for = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBuildElStepEl {
    type O = BlockAssignable<DataCloudbuildTriggerBuildElStepEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBuildElStepEl {}
impl BuildDataCloudbuildTriggerBuildElStepEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildElStepEl {
        DataCloudbuildTriggerBuildElStepEl {
            allow_exit_codes: core::default::Default::default(),
            allow_failure: core::default::Default::default(),
            args: core::default::Default::default(),
            dir: core::default::Default::default(),
            entrypoint: core::default::Default::default(),
            env: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            script: core::default::Default::default(),
            secret_env: core::default::Default::default(),
            timeout: core::default::Default::default(),
            timing: core::default::Default::default(),
            volumes: core::default::Default::default(),
            wait_for: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBuildElStepElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBuildElStepElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerBuildElStepElRef {
        DataCloudbuildTriggerBuildElStepElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBuildElStepElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_exit_codes` after provisioning.\n"]
    pub fn allow_exit_codes(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_exit_codes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_failure` after provisioning.\n"]
    pub fn allow_failure(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_failure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `dir` after provisioning.\n"]
    pub fn dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }
    #[doc = "Get a reference to the value of field `entrypoint` after provisioning.\n"]
    pub fn entrypoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entrypoint", self.base))
    }
    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `script` after provisioning.\n"]
    pub fn script(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.script", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_env` after provisioning.\n"]
    pub fn secret_env(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.secret_env", self.base))
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `timing` after provisioning.\n"]
    pub fn timing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timing", self.base))
    }
    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> ListRef<DataCloudbuildTriggerBuildElStepElVolumesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
    #[doc = "Get a reference to the value of field `wait_for` after provisioning.\n"]
    pub fn wait_for(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.wait_for", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerBuildEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    artifacts: Option<ListField<DataCloudbuildTriggerBuildElArtifactsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    available_secrets: Option<ListField<DataCloudbuildTriggerBuildElAvailableSecretsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logs_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<ListField<DataCloudbuildTriggerBuildElOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue_ttl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<ListField<DataCloudbuildTriggerBuildElSecretEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<ListField<DataCloudbuildTriggerBuildElSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    step: Option<ListField<DataCloudbuildTriggerBuildElStepEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    substitutions: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerBuildEl {
    #[doc = "Set the field `artifacts`.\n"]
    pub fn set_artifacts(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerBuildElArtifactsEl>>,
    ) -> Self {
        self.artifacts = Some(v.into());
        self
    }
    #[doc = "Set the field `available_secrets`.\n"]
    pub fn set_available_secrets(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerBuildElAvailableSecretsEl>>,
    ) -> Self {
        self.available_secrets = Some(v.into());
        self
    }
    #[doc = "Set the field `images`.\n"]
    pub fn set_images(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.images = Some(v.into());
        self
    }
    #[doc = "Set the field `logs_bucket`.\n"]
    pub fn set_logs_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logs_bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `options`.\n"]
    pub fn set_options(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerBuildElOptionsEl>>,
    ) -> Self {
        self.options = Some(v.into());
        self
    }
    #[doc = "Set the field `queue_ttl`.\n"]
    pub fn set_queue_ttl(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.queue_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerBuildElSecretEl>>,
    ) -> Self {
        self.secret = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerBuildElSourceEl>>,
    ) -> Self {
        self.source = Some(v.into());
        self
    }
    #[doc = "Set the field `step`.\n"]
    pub fn set_step(mut self, v: impl Into<ListField<DataCloudbuildTriggerBuildElStepEl>>) -> Self {
        self.step = Some(v.into());
        self
    }
    #[doc = "Set the field `substitutions`.\n"]
    pub fn set_substitutions(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.substitutions = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerBuildEl {
    type O = BlockAssignable<DataCloudbuildTriggerBuildEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerBuildEl {}
impl BuildDataCloudbuildTriggerBuildEl {
    pub fn build(self) -> DataCloudbuildTriggerBuildEl {
        DataCloudbuildTriggerBuildEl {
            artifacts: core::default::Default::default(),
            available_secrets: core::default::Default::default(),
            images: core::default::Default::default(),
            logs_bucket: core::default::Default::default(),
            options: core::default::Default::default(),
            queue_ttl: core::default::Default::default(),
            secret: core::default::Default::default(),
            source: core::default::Default::default(),
            step: core::default::Default::default(),
            substitutions: core::default::Default::default(),
            tags: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerBuildElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerBuildElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerBuildElRef {
        DataCloudbuildTriggerBuildElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerBuildElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `artifacts` after provisioning.\n"]
    pub fn artifacts(&self) -> ListRef<DataCloudbuildTriggerBuildElArtifactsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.artifacts", self.base))
    }
    #[doc = "Get a reference to the value of field `available_secrets` after provisioning.\n"]
    pub fn available_secrets(&self) -> ListRef<DataCloudbuildTriggerBuildElAvailableSecretsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.available_secrets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `images` after provisioning.\n"]
    pub fn images(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.images", self.base))
    }
    #[doc = "Get a reference to the value of field `logs_bucket` after provisioning.\n"]
    pub fn logs_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logs_bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> ListRef<DataCloudbuildTriggerBuildElOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.options", self.base))
    }
    #[doc = "Get a reference to the value of field `queue_ttl` after provisioning.\n"]
    pub fn queue_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> ListRef<DataCloudbuildTriggerBuildElSecretElRef> {
        ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<DataCloudbuildTriggerBuildElSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
    #[doc = "Get a reference to the value of field `step` after provisioning.\n"]
    pub fn step(&self) -> ListRef<DataCloudbuildTriggerBuildElStepElRef> {
        ListRef::new(self.shared().clone(), format!("{}.step", self.base))
    }
    #[doc = "Get a reference to the value of field `substitutions` after provisioning.\n"]
    pub fn substitutions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.substitutions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
}
impl DataCloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }
    #[doc = "Set the field `comment_control`.\n"]
    pub fn set_comment_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comment_control = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\n"]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl {
    type O = BlockAssignable<DataCloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl {}
impl BuildDataCloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl {
    pub fn build(self) -> DataCloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl {
        DataCloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl {
            branch: core::default::Default::default(),
            comment_control: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerDeveloperConnectEventConfigElPullRequestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerDeveloperConnectEventConfigElPullRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudbuildTriggerDeveloperConnectEventConfigElPullRequestElRef {
        DataCloudbuildTriggerDeveloperConnectEventConfigElPullRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerDeveloperConnectEventConfigElPullRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }
    #[doc = "Get a reference to the value of field `comment_control` after provisioning.\n"]
    pub fn comment_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comment_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\n"]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerDeveloperConnectEventConfigElPushEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerDeveloperConnectEventConfigElPushEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\n"]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerDeveloperConnectEventConfigElPushEl {
    type O = BlockAssignable<DataCloudbuildTriggerDeveloperConnectEventConfigElPushEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerDeveloperConnectEventConfigElPushEl {}
impl BuildDataCloudbuildTriggerDeveloperConnectEventConfigElPushEl {
    pub fn build(self) -> DataCloudbuildTriggerDeveloperConnectEventConfigElPushEl {
        DataCloudbuildTriggerDeveloperConnectEventConfigElPushEl {
            branch: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerDeveloperConnectEventConfigElPushElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerDeveloperConnectEventConfigElPushElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudbuildTriggerDeveloperConnectEventConfigElPushElRef {
        DataCloudbuildTriggerDeveloperConnectEventConfigElPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerDeveloperConnectEventConfigElPushElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\n"]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerDeveloperConnectEventConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    git_repository_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git_repository_link_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request:
        Option<ListField<DataCloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push: Option<ListField<DataCloudbuildTriggerDeveloperConnectEventConfigElPushEl>>,
}
impl DataCloudbuildTriggerDeveloperConnectEventConfigEl {
    #[doc = "Set the field `git_repository_link`.\n"]
    pub fn set_git_repository_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.git_repository_link = Some(v.into());
        self
    }
    #[doc = "Set the field `git_repository_link_type`.\n"]
    pub fn set_git_repository_link_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.git_repository_link_type = Some(v.into());
        self
    }
    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerDeveloperConnectEventConfigElPullRequestEl>>,
    ) -> Self {
        self.pull_request = Some(v.into());
        self
    }
    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerDeveloperConnectEventConfigElPushEl>>,
    ) -> Self {
        self.push = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerDeveloperConnectEventConfigEl {
    type O = BlockAssignable<DataCloudbuildTriggerDeveloperConnectEventConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerDeveloperConnectEventConfigEl {}
impl BuildDataCloudbuildTriggerDeveloperConnectEventConfigEl {
    pub fn build(self) -> DataCloudbuildTriggerDeveloperConnectEventConfigEl {
        DataCloudbuildTriggerDeveloperConnectEventConfigEl {
            git_repository_link: core::default::Default::default(),
            git_repository_link_type: core::default::Default::default(),
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerDeveloperConnectEventConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerDeveloperConnectEventConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudbuildTriggerDeveloperConnectEventConfigElRef {
        DataCloudbuildTriggerDeveloperConnectEventConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerDeveloperConnectEventConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `git_repository_link` after provisioning.\n"]
    pub fn git_repository_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.git_repository_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `git_repository_link_type` after provisioning.\n"]
    pub fn git_repository_link_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.git_repository_link_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(
        &self,
    ) -> ListRef<DataCloudbuildTriggerDeveloperConnectEventConfigElPullRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }
    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> ListRef<DataCloudbuildTriggerDeveloperConnectEventConfigElPushElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerGitFileSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bitbucket_server_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github_enterprise_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerGitFileSourceEl {
    #[doc = "Set the field `bitbucket_server_config`.\n"]
    pub fn set_bitbucket_server_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bitbucket_server_config = Some(v.into());
        self
    }
    #[doc = "Set the field `github_enterprise_config`.\n"]
    pub fn set_github_enterprise_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.github_enterprise_config = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `repo_type`.\n"]
    pub fn set_repo_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_type = Some(v.into());
        self
    }
    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }
    #[doc = "Set the field `revision`.\n"]
    pub fn set_revision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerGitFileSourceEl {
    type O = BlockAssignable<DataCloudbuildTriggerGitFileSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerGitFileSourceEl {}
impl BuildDataCloudbuildTriggerGitFileSourceEl {
    pub fn build(self) -> DataCloudbuildTriggerGitFileSourceEl {
        DataCloudbuildTriggerGitFileSourceEl {
            bitbucket_server_config: core::default::Default::default(),
            github_enterprise_config: core::default::Default::default(),
            path: core::default::Default::default(),
            repo_type: core::default::Default::default(),
            repository: core::default::Default::default(),
            revision: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerGitFileSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerGitFileSourceElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerGitFileSourceElRef {
        DataCloudbuildTriggerGitFileSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerGitFileSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bitbucket_server_config` after provisioning.\n"]
    pub fn bitbucket_server_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bitbucket_server_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `github_enterprise_config` after provisioning.\n"]
    pub fn github_enterprise_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.github_enterprise_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `repo_type` after provisioning.\n"]
    pub fn repo_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_type", self.base))
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerGithubElPullRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
}
impl DataCloudbuildTriggerGithubElPullRequestEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }
    #[doc = "Set the field `comment_control`.\n"]
    pub fn set_comment_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comment_control = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\n"]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerGithubElPullRequestEl {
    type O = BlockAssignable<DataCloudbuildTriggerGithubElPullRequestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerGithubElPullRequestEl {}
impl BuildDataCloudbuildTriggerGithubElPullRequestEl {
    pub fn build(self) -> DataCloudbuildTriggerGithubElPullRequestEl {
        DataCloudbuildTriggerGithubElPullRequestEl {
            branch: core::default::Default::default(),
            comment_control: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerGithubElPullRequestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerGithubElPullRequestElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerGithubElPullRequestElRef {
        DataCloudbuildTriggerGithubElPullRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerGithubElPullRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }
    #[doc = "Get a reference to the value of field `comment_control` after provisioning.\n"]
    pub fn comment_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comment_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\n"]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerGithubElPushEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerGithubElPushEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\n"]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerGithubElPushEl {
    type O = BlockAssignable<DataCloudbuildTriggerGithubElPushEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerGithubElPushEl {}
impl BuildDataCloudbuildTriggerGithubElPushEl {
    pub fn build(self) -> DataCloudbuildTriggerGithubElPushEl {
        DataCloudbuildTriggerGithubElPushEl {
            branch: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerGithubElPushElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerGithubElPushElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerGithubElPushElRef {
        DataCloudbuildTriggerGithubElPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerGithubElPushElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\n"]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enterprise_config_resource_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request: Option<ListField<DataCloudbuildTriggerGithubElPullRequestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push: Option<ListField<DataCloudbuildTriggerGithubElPushEl>>,
}
impl DataCloudbuildTriggerGithubEl {
    #[doc = "Set the field `enterprise_config_resource_name`.\n"]
    pub fn set_enterprise_config_resource_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enterprise_config_resource_name = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `owner`.\n"]
    pub fn set_owner(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.owner = Some(v.into());
        self
    }
    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerGithubElPullRequestEl>>,
    ) -> Self {
        self.pull_request = Some(v.into());
        self
    }
    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerGithubElPushEl>>,
    ) -> Self {
        self.push = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerGithubEl {
    type O = BlockAssignable<DataCloudbuildTriggerGithubEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerGithubEl {}
impl BuildDataCloudbuildTriggerGithubEl {
    pub fn build(self) -> DataCloudbuildTriggerGithubEl {
        DataCloudbuildTriggerGithubEl {
            enterprise_config_resource_name: core::default::Default::default(),
            name: core::default::Default::default(),
            owner: core::default::Default::default(),
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerGithubElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerGithubElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerGithubElRef {
        DataCloudbuildTriggerGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enterprise_config_resource_name` after provisioning.\n"]
    pub fn enterprise_config_resource_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enterprise_config_resource_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }
    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(&self) -> ListRef<DataCloudbuildTriggerGithubElPullRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }
    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> ListRef<DataCloudbuildTriggerGithubElPushElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerPubsubConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerPubsubConfigEl {
    #[doc = "Set the field `service_account_email`.\n"]
    pub fn set_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account_email = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `subscription`.\n"]
    pub fn set_subscription(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subscription = Some(v.into());
        self
    }
    #[doc = "Set the field `topic`.\n"]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerPubsubConfigEl {
    type O = BlockAssignable<DataCloudbuildTriggerPubsubConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerPubsubConfigEl {}
impl BuildDataCloudbuildTriggerPubsubConfigEl {
    pub fn build(self) -> DataCloudbuildTriggerPubsubConfigEl {
        DataCloudbuildTriggerPubsubConfigEl {
            service_account_email: core::default::Default::default(),
            state: core::default::Default::default(),
            subscription: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerPubsubConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerPubsubConfigElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerPubsubConfigElRef {
        DataCloudbuildTriggerPubsubConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerPubsubConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n"]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `subscription` after provisioning.\n"]
    pub fn subscription(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.base))
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerRepositoryEventConfigElPullRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment_control: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
}
impl DataCloudbuildTriggerRepositoryEventConfigElPullRequestEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }
    #[doc = "Set the field `comment_control`.\n"]
    pub fn set_comment_control(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comment_control = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\n"]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerRepositoryEventConfigElPullRequestEl {
    type O = BlockAssignable<DataCloudbuildTriggerRepositoryEventConfigElPullRequestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerRepositoryEventConfigElPullRequestEl {}
impl BuildDataCloudbuildTriggerRepositoryEventConfigElPullRequestEl {
    pub fn build(self) -> DataCloudbuildTriggerRepositoryEventConfigElPullRequestEl {
        DataCloudbuildTriggerRepositoryEventConfigElPullRequestEl {
            branch: core::default::Default::default(),
            comment_control: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerRepositoryEventConfigElPullRequestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerRepositoryEventConfigElPullRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudbuildTriggerRepositoryEventConfigElPullRequestElRef {
        DataCloudbuildTriggerRepositoryEventConfigElPullRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerRepositoryEventConfigElPullRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }
    #[doc = "Get a reference to the value of field `comment_control` after provisioning.\n"]
    pub fn comment_control(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comment_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\n"]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerRepositoryEventConfigElPushEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerRepositoryEventConfigElPushEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\n"]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerRepositoryEventConfigElPushEl {
    type O = BlockAssignable<DataCloudbuildTriggerRepositoryEventConfigElPushEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerRepositoryEventConfigElPushEl {}
impl BuildDataCloudbuildTriggerRepositoryEventConfigElPushEl {
    pub fn build(self) -> DataCloudbuildTriggerRepositoryEventConfigElPushEl {
        DataCloudbuildTriggerRepositoryEventConfigElPushEl {
            branch: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerRepositoryEventConfigElPushElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerRepositoryEventConfigElPushElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudbuildTriggerRepositoryEventConfigElPushElRef {
        DataCloudbuildTriggerRepositoryEventConfigElPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerRepositoryEventConfigElPushElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\n"]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerRepositoryEventConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request: Option<ListField<DataCloudbuildTriggerRepositoryEventConfigElPullRequestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push: Option<ListField<DataCloudbuildTriggerRepositoryEventConfigElPushEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerRepositoryEventConfigEl {
    #[doc = "Set the field `pull_request`.\n"]
    pub fn set_pull_request(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerRepositoryEventConfigElPullRequestEl>>,
    ) -> Self {
        self.pull_request = Some(v.into());
        self
    }
    #[doc = "Set the field `push`.\n"]
    pub fn set_push(
        mut self,
        v: impl Into<ListField<DataCloudbuildTriggerRepositoryEventConfigElPushEl>>,
    ) -> Self {
        self.push = Some(v.into());
        self
    }
    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerRepositoryEventConfigEl {
    type O = BlockAssignable<DataCloudbuildTriggerRepositoryEventConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerRepositoryEventConfigEl {}
impl BuildDataCloudbuildTriggerRepositoryEventConfigEl {
    pub fn build(self) -> DataCloudbuildTriggerRepositoryEventConfigEl {
        DataCloudbuildTriggerRepositoryEventConfigEl {
            pull_request: core::default::Default::default(),
            push: core::default::Default::default(),
            repository: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerRepositoryEventConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerRepositoryEventConfigElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerRepositoryEventConfigElRef {
        DataCloudbuildTriggerRepositoryEventConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerRepositoryEventConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pull_request` after provisioning.\n"]
    pub fn pull_request(
        &self,
    ) -> ListRef<DataCloudbuildTriggerRepositoryEventConfigElPullRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pull_request", self.base))
    }
    #[doc = "Get a reference to the value of field `push` after provisioning.\n"]
    pub fn push(&self) -> ListRef<DataCloudbuildTriggerRepositoryEventConfigElPushElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push", self.base))
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerSourceToBuildEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bitbucket_server_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github_enterprise_config: Option<PrimField<String>>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    ref_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerSourceToBuildEl {
    #[doc = "Set the field `bitbucket_server_config`.\n"]
    pub fn set_bitbucket_server_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bitbucket_server_config = Some(v.into());
        self
    }
    #[doc = "Set the field `github_enterprise_config`.\n"]
    pub fn set_github_enterprise_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.github_enterprise_config = Some(v.into());
        self
    }
    #[doc = "Set the field `ref_`.\n"]
    pub fn set_ref(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ref_ = Some(v.into());
        self
    }
    #[doc = "Set the field `repo_type`.\n"]
    pub fn set_repo_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_type = Some(v.into());
        self
    }
    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerSourceToBuildEl {
    type O = BlockAssignable<DataCloudbuildTriggerSourceToBuildEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerSourceToBuildEl {}
impl BuildDataCloudbuildTriggerSourceToBuildEl {
    pub fn build(self) -> DataCloudbuildTriggerSourceToBuildEl {
        DataCloudbuildTriggerSourceToBuildEl {
            bitbucket_server_config: core::default::Default::default(),
            github_enterprise_config: core::default::Default::default(),
            ref_: core::default::Default::default(),
            repo_type: core::default::Default::default(),
            repository: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerSourceToBuildElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerSourceToBuildElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerSourceToBuildElRef {
        DataCloudbuildTriggerSourceToBuildElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerSourceToBuildElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bitbucket_server_config` after provisioning.\n"]
    pub fn bitbucket_server_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bitbucket_server_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `github_enterprise_config` after provisioning.\n"]
    pub fn github_enterprise_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.github_enterprise_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ref_` after provisioning.\n"]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }
    #[doc = "Get a reference to the value of field `repo_type` after provisioning.\n"]
    pub fn repo_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_type", self.base))
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerTriggerTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_sha: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_regex: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_name: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerTriggerTemplateEl {
    #[doc = "Set the field `branch_name`.\n"]
    pub fn set_branch_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch_name = Some(v.into());
        self
    }
    #[doc = "Set the field `commit_sha`.\n"]
    pub fn set_commit_sha(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.commit_sha = Some(v.into());
        self
    }
    #[doc = "Set the field `dir`.\n"]
    pub fn set_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dir = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_regex`.\n"]
    pub fn set_invert_regex(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `repo_name`.\n"]
    pub fn set_repo_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_name = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_name`.\n"]
    pub fn set_tag_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerTriggerTemplateEl {
    type O = BlockAssignable<DataCloudbuildTriggerTriggerTemplateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerTriggerTemplateEl {}
impl BuildDataCloudbuildTriggerTriggerTemplateEl {
    pub fn build(self) -> DataCloudbuildTriggerTriggerTemplateEl {
        DataCloudbuildTriggerTriggerTemplateEl {
            branch_name: core::default::Default::default(),
            commit_sha: core::default::Default::default(),
            dir: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            project_id: core::default::Default::default(),
            repo_name: core::default::Default::default(),
            tag_name: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerTriggerTemplateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerTriggerTemplateElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerTriggerTemplateElRef {
        DataCloudbuildTriggerTriggerTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerTriggerTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch_name` after provisioning.\n"]
    pub fn branch_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }
    #[doc = "Get a reference to the value of field `commit_sha` after provisioning.\n"]
    pub fn commit_sha(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.base))
    }
    #[doc = "Get a reference to the value of field `dir` after provisioning.\n"]
    pub fn dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }
    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\n"]
    pub fn invert_regex(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `repo_name` after provisioning.\n"]
    pub fn repo_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_name", self.base))
    }
    #[doc = "Get a reference to the value of field `tag_name` after provisioning.\n"]
    pub fn tag_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudbuildTriggerWebhookConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl DataCloudbuildTriggerWebhookConfigEl {
    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudbuildTriggerWebhookConfigEl {
    type O = BlockAssignable<DataCloudbuildTriggerWebhookConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudbuildTriggerWebhookConfigEl {}
impl BuildDataCloudbuildTriggerWebhookConfigEl {
    pub fn build(self) -> DataCloudbuildTriggerWebhookConfigEl {
        DataCloudbuildTriggerWebhookConfigEl {
            secret: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct DataCloudbuildTriggerWebhookConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudbuildTriggerWebhookConfigElRef {
    fn new(shared: StackShared, base: String) -> DataCloudbuildTriggerWebhookConfigElRef {
        DataCloudbuildTriggerWebhookConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudbuildTriggerWebhookConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
