use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AmplifyBranchData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backend_environment_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_auth_credentials: Option<PrimField<String>>,
    branch_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_auto_build: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_basic_auth: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_notification: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_performance_mode: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_pull_request_preview: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_skew_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    framework: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request_environment_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<String>>,
}

struct AmplifyBranch_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AmplifyBranchData>,
}

#[derive(Clone)]
pub struct AmplifyBranch(Rc<AmplifyBranch_>);

impl AmplifyBranch {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `backend_environment_arn`.\n"]
    pub fn set_backend_environment_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().backend_environment_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `basic_auth_credentials`.\n"]
    pub fn set_basic_auth_credentials(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().basic_auth_credentials = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_auto_build`.\n"]
    pub fn set_enable_auto_build(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_auto_build = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_basic_auth`.\n"]
    pub fn set_enable_basic_auth(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_basic_auth = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_notification`.\n"]
    pub fn set_enable_notification(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_notification = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_performance_mode`.\n"]
    pub fn set_enable_performance_mode(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_performance_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_pull_request_preview`.\n"]
    pub fn set_enable_pull_request_preview(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_pull_request_preview = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_skew_protection`.\n"]
    pub fn set_enable_skew_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_skew_protection = Some(v.into());
        self
    }

    #[doc = "Set the field `environment_variables`.\n"]
    pub fn set_environment_variables(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().environment_variables = Some(v.into());
        self
    }

    #[doc = "Set the field `framework`.\n"]
    pub fn set_framework(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().framework = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `pull_request_environment_name`.\n"]
    pub fn set_pull_request_environment_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().pull_request_environment_name = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `stage`.\n"]
    pub fn set_stage(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().stage = Some(v.into());
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

    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ttl = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `app_id` after provisioning.\n"]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associated_resources` after provisioning.\n"]
    pub fn associated_resources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.associated_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backend_environment_arn` after provisioning.\n"]
    pub fn backend_environment_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backend_environment_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `basic_auth_credentials` after provisioning.\n"]
    pub fn basic_auth_credentials(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.basic_auth_credentials", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `branch_name` after provisioning.\n"]
    pub fn branch_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_domains` after provisioning.\n"]
    pub fn custom_domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.custom_domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_branch` after provisioning.\n"]
    pub fn destination_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_branch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_auto_build` after provisioning.\n"]
    pub fn enable_auto_build(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_auto_build", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_basic_auth` after provisioning.\n"]
    pub fn enable_basic_auth(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_basic_auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_notification` after provisioning.\n"]
    pub fn enable_notification(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_notification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_performance_mode` after provisioning.\n"]
    pub fn enable_performance_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_performance_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_pull_request_preview` after provisioning.\n"]
    pub fn enable_pull_request_preview(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_pull_request_preview", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_skew_protection` after provisioning.\n"]
    pub fn enable_skew_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_skew_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `framework` after provisioning.\n"]
    pub fn framework(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.framework", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pull_request_environment_name` after provisioning.\n"]
    pub fn pull_request_environment_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pull_request_environment_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_branch` after provisioning.\n"]
    pub fn source_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_branch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
}

impl Referable for AmplifyBranch {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AmplifyBranch { }

impl ToListMappable for AmplifyBranch {
    type O = ListRef<AmplifyBranchRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AmplifyBranch_ {
    fn extract_resource_type(&self) -> String {
        "aws_amplify_branch".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAmplifyBranch {
    pub tf_id: String,
    #[doc = ""]
    pub app_id: PrimField<String>,
    #[doc = ""]
    pub branch_name: PrimField<String>,
}

impl BuildAmplifyBranch {
    pub fn build(self, stack: &mut Stack) -> AmplifyBranch {
        let out = AmplifyBranch(Rc::new(AmplifyBranch_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AmplifyBranchData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app_id: self.app_id,
                backend_environment_arn: core::default::Default::default(),
                basic_auth_credentials: core::default::Default::default(),
                branch_name: self.branch_name,
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                enable_auto_build: core::default::Default::default(),
                enable_basic_auth: core::default::Default::default(),
                enable_notification: core::default::Default::default(),
                enable_performance_mode: core::default::Default::default(),
                enable_pull_request_preview: core::default::Default::default(),
                enable_skew_protection: core::default::Default::default(),
                environment_variables: core::default::Default::default(),
                framework: core::default::Default::default(),
                id: core::default::Default::default(),
                pull_request_environment_name: core::default::Default::default(),
                region: core::default::Default::default(),
                stage: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                ttl: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AmplifyBranchRef {
    shared: StackShared,
    base: String,
}

impl Ref for AmplifyBranchRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl AmplifyBranchRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_id` after provisioning.\n"]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associated_resources` after provisioning.\n"]
    pub fn associated_resources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.associated_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backend_environment_arn` after provisioning.\n"]
    pub fn backend_environment_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backend_environment_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `basic_auth_credentials` after provisioning.\n"]
    pub fn basic_auth_credentials(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.basic_auth_credentials", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `branch_name` after provisioning.\n"]
    pub fn branch_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_domains` after provisioning.\n"]
    pub fn custom_domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.custom_domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_branch` after provisioning.\n"]
    pub fn destination_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_branch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_auto_build` after provisioning.\n"]
    pub fn enable_auto_build(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_auto_build", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_basic_auth` after provisioning.\n"]
    pub fn enable_basic_auth(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_basic_auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_notification` after provisioning.\n"]
    pub fn enable_notification(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_notification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_performance_mode` after provisioning.\n"]
    pub fn enable_performance_mode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_performance_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_pull_request_preview` after provisioning.\n"]
    pub fn enable_pull_request_preview(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_pull_request_preview", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_skew_protection` after provisioning.\n"]
    pub fn enable_skew_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_skew_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `framework` after provisioning.\n"]
    pub fn framework(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.framework", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pull_request_environment_name` after provisioning.\n"]
    pub fn pull_request_environment_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pull_request_environment_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_branch` after provisioning.\n"]
    pub fn source_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_branch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
}
