use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SagemakerUserProfileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    domain_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_sign_on_user_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_sign_on_user_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    user_profile_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_settings: Option<Vec<SagemakerUserProfileUserSettingsEl>>,
    dynamic: SagemakerUserProfileDynamic,
}

struct SagemakerUserProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerUserProfileData>,
}

#[derive(Clone)]
pub struct SagemakerUserProfile(Rc<SagemakerUserProfile_>);

impl SagemakerUserProfile {
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

    #[doc = "Set the field `single_sign_on_user_identifier`.\n"]
    pub fn set_single_sign_on_user_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().single_sign_on_user_identifier = Some(v.into());
        self
    }

    #[doc = "Set the field `single_sign_on_user_value`.\n"]
    pub fn set_single_sign_on_user_value(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().single_sign_on_user_value = Some(v.into());
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

    #[doc = "Set the field `user_settings`.\n"]
    pub fn set_user_settings(
        self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `home_efs_file_system_uid` after provisioning.\n"]
    pub fn home_efs_file_system_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.home_efs_file_system_uid", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `single_sign_on_user_identifier` after provisioning.\n"]
    pub fn single_sign_on_user_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_sign_on_user_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `single_sign_on_user_value` after provisioning.\n"]
    pub fn single_sign_on_user_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_sign_on_user_value", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `user_profile_name` after provisioning.\n"]
    pub fn user_profile_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_profile_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_settings` after provisioning.\n"]
    pub fn user_settings(&self) -> ListRef<SagemakerUserProfileUserSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_settings", self.extract_ref()),
        )
    }
}

impl Referable for SagemakerUserProfile {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SagemakerUserProfile {}

impl ToListMappable for SagemakerUserProfile {
    type O = ListRef<SagemakerUserProfileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerUserProfile_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_user_profile".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerUserProfile {
    pub tf_id: String,
    #[doc = ""]
    pub domain_id: PrimField<String>,
    #[doc = ""]
    pub user_profile_name: PrimField<String>,
}

impl BuildSagemakerUserProfile {
    pub fn build(self, stack: &mut Stack) -> SagemakerUserProfile {
        let out = SagemakerUserProfile(Rc::new(SagemakerUserProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerUserProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                domain_id: self.domain_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                single_sign_on_user_identifier: core::default::Default::default(),
                single_sign_on_user_value: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                user_profile_name: self.user_profile_name,
                user_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerUserProfileRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SagemakerUserProfileRef {
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

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `home_efs_file_system_uid` after provisioning.\n"]
    pub fn home_efs_file_system_uid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.home_efs_file_system_uid", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `single_sign_on_user_identifier` after provisioning.\n"]
    pub fn single_sign_on_user_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_sign_on_user_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `single_sign_on_user_value` after provisioning.\n"]
    pub fn single_sign_on_user_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.single_sign_on_user_value", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `user_profile_name` after provisioning.\n"]
    pub fn user_profile_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_profile_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_settings` after provisioning.\n"]
    pub fn user_settings(&self) -> ListRef<SagemakerUserProfileUserSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_settings", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable
    for SagemakerUserProfileUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl
{
    type O = BlockAssignable<
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl {
            status: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElDirectDeploySettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElCanvasAppSettingsElDirectDeploySettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElDirectDeploySettingsElRef {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElDirectDeploySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElDirectDeploySettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl {
    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable
    for SagemakerUserProfileUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl
{
    type O = BlockAssignable<
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl {
            execution_role_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElEmrServerlessSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElCanvasAppSettingsElEmrServerlessSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElEmrServerlessSettingsElRef {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElEmrServerlessSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElEmrServerlessSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amazon_bedrock_role_arn: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl {
    #[doc = "Set the field `amazon_bedrock_role_arn`.\n"]
    pub fn set_amazon_bedrock_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.amazon_bedrock_role_arn = Some(v.into());
        self
    }
}

impl ToListMappable
    for SagemakerUserProfileUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl
{
    type O = BlockAssignable<
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl {
            amazon_bedrock_role_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElGenerativeAiSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElCanvasAppSettingsElGenerativeAiSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElGenerativeAiSettingsElRef {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElGenerativeAiSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElGenerativeAiSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amazon_bedrock_role_arn` after provisioning.\n"]
    pub fn amazon_bedrock_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.amazon_bedrock_role_arn", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source_name: Option<PrimField<String>>,
    secret_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl {
    #[doc = "Set the field `data_source_name`.\n"]
    pub fn set_data_source_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_source_name = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable
    for SagemakerUserProfileUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl
{
    type O = BlockAssignable<
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl
{
    #[doc = ""]
    pub secret_arn: PrimField<String>,
}

impl BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl {
            data_source_name: core::default::Default::default(),
            secret_arn: self.secret_arn,
            status: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref
    for SagemakerUserProfileUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsElRef
    {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_source_name` after provisioning.\n"]
    pub fn data_source_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElKendraSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElKendraSettingsEl {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElCanvasAppSettingsElKendraSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElCanvasAppSettingsElKendraSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElKendraSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElKendraSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElKendraSettingsEl {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElKendraSettingsEl {
            status: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElKendraSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElCanvasAppSettingsElKendraSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElKendraSettingsElRef {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElKendraSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElKendraSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_account_model_register_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl {
    #[doc = "Set the field `cross_account_model_register_role_arn`.\n"]
    pub fn set_cross_account_model_register_role_arn(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.cross_account_model_register_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable
    for SagemakerUserProfileUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl
{
    type O = BlockAssignable<
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl {
            cross_account_model_register_role_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElModelRegisterSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElCanvasAppSettingsElModelRegisterSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElModelRegisterSettingsElRef {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElModelRegisterSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElModelRegisterSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cross_account_model_register_role_arn` after provisioning.\n"]
    pub fn cross_account_model_register_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cross_account_model_register_role_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amazon_forecast_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    #[doc = "Set the field `amazon_forecast_role_arn`.\n"]
    pub fn set_amazon_forecast_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.amazon_forecast_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable
    for SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl
{
    type O = BlockAssignable<
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl
{}

impl BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl {
            amazon_forecast_role_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref
    for SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef
    {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amazon_forecast_role_arn` after provisioning.\n"]
    pub fn amazon_forecast_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.amazon_forecast_role_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_artifact_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_kms_key_id: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl {
    #[doc = "Set the field `s3_artifact_path`.\n"]
    pub fn set_s3_artifact_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_artifact_path = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_kms_key_id`.\n"]
    pub fn set_s3_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_kms_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl {
    type O =
        BlockAssignable<SagemakerUserProfileUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl {
            s3_artifact_path: core::default::Default::default(),
            s3_kms_key_id: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElWorkspaceSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElCanvasAppSettingsElWorkspaceSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElWorkspaceSettingsElRef {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElWorkspaceSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElWorkspaceSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_artifact_path` after provisioning.\n"]
    pub fn s3_artifact_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_artifact_path", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_kms_key_id` after provisioning.\n"]
    pub fn s3_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_kms_key_id", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElDynamic {
    direct_deploy_settings: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl>,
    >,
    emr_serverless_settings: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl>,
    >,
    generative_ai_settings: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl>,
    >,
    identity_provider_oauth_settings: Option<
        DynamicBlock<
            SagemakerUserProfileUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl,
        >,
    >,
    kendra_settings:
        Option<DynamicBlock<SagemakerUserProfileUserSettingsElCanvasAppSettingsElKendraSettingsEl>>,
    model_register_settings: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl>,
    >,
    time_series_forecasting_settings: Option<
        DynamicBlock<
            SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl,
        >,
    >,
    workspace_settings: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_deploy_settings:
        Option<Vec<SagemakerUserProfileUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emr_serverless_settings:
        Option<Vec<SagemakerUserProfileUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generative_ai_settings:
        Option<Vec<SagemakerUserProfileUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_oauth_settings: Option<
        Vec<SagemakerUserProfileUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    kendra_settings:
        Option<Vec<SagemakerUserProfileUserSettingsElCanvasAppSettingsElKendraSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_register_settings:
        Option<Vec<SagemakerUserProfileUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_series_forecasting_settings: Option<
        Vec<SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    workspace_settings:
        Option<Vec<SagemakerUserProfileUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl>>,
    dynamic: SagemakerUserProfileUserSettingsElCanvasAppSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsEl {
    #[doc = "Set the field `direct_deploy_settings`.\n"]
    pub fn set_direct_deploy_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElCanvasAppSettingsElDirectDeploySettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.direct_deploy_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.direct_deploy_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `emr_serverless_settings`.\n"]
    pub fn set_emr_serverless_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElCanvasAppSettingsElEmrServerlessSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.emr_serverless_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.emr_serverless_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `generative_ai_settings`.\n"]
    pub fn set_generative_ai_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElCanvasAppSettingsElGenerativeAiSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.generative_ai_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.generative_ai_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `identity_provider_oauth_settings`.\n"]
    pub fn set_identity_provider_oauth_settings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerUserProfileUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.identity_provider_oauth_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.identity_provider_oauth_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `kendra_settings`.\n"]
    pub fn set_kendra_settings(
        mut self,
        v: impl Into<
            BlockAssignable<SagemakerUserProfileUserSettingsElCanvasAppSettingsElKendraSettingsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kendra_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kendra_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `model_register_settings`.\n"]
    pub fn set_model_register_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElCanvasAppSettingsElModelRegisterSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.model_register_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.model_register_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `time_series_forecasting_settings`.\n"]
    pub fn set_time_series_forecasting_settings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.time_series_forecasting_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.time_series_forecasting_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `workspace_settings`.\n"]
    pub fn set_workspace_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElCanvasAppSettingsElWorkspaceSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.workspace_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.workspace_settings = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElCanvasAppSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElCanvasAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElCanvasAppSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsEl {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsEl {
            direct_deploy_settings: core::default::Default::default(),
            emr_serverless_settings: core::default::Default::default(),
            generative_ai_settings: core::default::Default::default(),
            identity_provider_oauth_settings: core::default::Default::default(),
            kendra_settings: core::default::Default::default(),
            model_register_settings: core::default::Default::default(),
            time_series_forecasting_settings: core::default::Default::default(),
            workspace_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef {
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `direct_deploy_settings` after provisioning.\n"]
    pub fn direct_deploy_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElCanvasAppSettingsElDirectDeploySettingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.direct_deploy_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `emr_serverless_settings` after provisioning.\n"]
    pub fn emr_serverless_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElCanvasAppSettingsElEmrServerlessSettingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.emr_serverless_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `generative_ai_settings` after provisioning.\n"]
    pub fn generative_ai_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElCanvasAppSettingsElGenerativeAiSettingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.generative_ai_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `identity_provider_oauth_settings` after provisioning.\n"]
    pub fn identity_provider_oauth_settings(
        &self,
    ) -> ListRef<
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElIdentityProviderOauthSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity_provider_oauth_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `kendra_settings` after provisioning.\n"]
    pub fn kendra_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElCanvasAppSettingsElKendraSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kendra_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `model_register_settings` after provisioning.\n"]
    pub fn model_register_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElCanvasAppSettingsElModelRegisterSettingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_register_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `time_series_forecasting_settings` after provisioning.\n"]
    pub fn time_series_forecasting_settings(
        &self,
    ) -> ListRef<
        SagemakerUserProfileUserSettingsElCanvasAppSettingsElTimeSeriesForecastingSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.time_series_forecasting_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `workspace_settings` after provisioning.\n"]
    pub fn workspace_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElCanvasAppSettingsElWorkspaceSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workspace_settings", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout_in_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_management: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_idle_timeout_in_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_idle_timeout_in_minutes: Option<PrimField<f64>>,
}

impl
    SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl
{
    #[doc = "Set the field `idle_timeout_in_minutes`.\n"]
    pub fn set_idle_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.idle_timeout_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_management`.\n"]
    pub fn set_lifecycle_management(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_management = Some(v.into());
        self
    }

    #[doc = "Set the field `max_idle_timeout_in_minutes`.\n"]
    pub fn set_max_idle_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_idle_timeout_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `min_idle_timeout_in_minutes`.\n"]
    pub fn set_min_idle_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_idle_timeout_in_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    type O =
        BlockAssignable<
            SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl
{}

impl BuildSagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl {
        SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl {
            idle_timeout_in_minutes: core::default::Default::default(),
            lifecycle_management: core::default::Default::default(),
            max_idle_timeout_in_minutes: core::default::Default::default(),
            min_idle_timeout_in_minutes: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
        SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_timeout_in_minutes` after provisioning.\n"]
    pub fn idle_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_management` after provisioning.\n"]
    pub fn lifecycle_management(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_management", self.base))
    }

    #[doc = "Get a reference to the value of field `max_idle_timeout_in_minutes` after provisioning.\n"]
    pub fn max_idle_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_idle_timeout_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `min_idle_timeout_in_minutes` after provisioning.\n"]
    pub fn min_idle_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_idle_timeout_in_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElDynamic {
    idle_settings: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_settings: Option<
        Vec<SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl>,
    >,
    dynamic: SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElDynamic,
}

impl SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {
    #[doc = "Set the field `idle_settings`.\n"]
    pub fn set_idle_settings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.idle_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.idle_settings = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl
{
    type O = BlockAssignable<
        SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {
}

impl BuildSagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {
        SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl {
            idle_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef {
        SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_settings` after provisioning.\n"]
    pub fn idle_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElIdleSettingsElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.idle_settings", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElCustomImageEl {
    app_image_config_name: PrimField<String>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_number: Option<PrimField<f64>>,
}

impl SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElCustomImageEl {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_version_number = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElCustomImageEl {
    type O =
        BlockAssignable<SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElCustomImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCodeEditorAppSettingsElCustomImageEl {
    #[doc = ""]
    pub app_image_config_name: PrimField<String>,
    #[doc = ""]
    pub image_name: PrimField<String>,
}

impl BuildSagemakerUserProfileUserSettingsElCodeEditorAppSettingsElCustomImageEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElCustomImageEl {
        SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElCustomImageEl {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElCustomImageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElCustomImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElCustomImageElRef {
        SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElCustomImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_image_config_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_version_number", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable
    for SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl
{
    type O = BlockAssignable<
        SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {
        SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef {
        SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_alias", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_arn", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDynamic {
    app_lifecycle_management: Option<
        DynamicBlock<
            SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl,
        >,
    >,
    custom_image: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElCustomImageEl>,
    >,
    default_resource_spec: Option<
        DynamicBlock<
            SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCodeEditorAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    built_in_lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_lifecycle_management: Option<
        Vec<SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_image:
        Option<Vec<SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec:
        Option<Vec<SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElCodeEditorAppSettingsEl {
    #[doc = "Set the field `built_in_lifecycle_config_arn`.\n"]
    pub fn set_built_in_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.built_in_lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `app_lifecycle_management`.\n"]
    pub fn set_app_lifecycle_management(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.app_lifecycle_management = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.app_lifecycle_management = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v: impl Into<
            BlockAssignable<SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElCustomImageEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_image = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElCodeEditorAppSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElCodeEditorAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCodeEditorAppSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElCodeEditorAppSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElCodeEditorAppSettingsEl {
        SagemakerUserProfileUserSettingsElCodeEditorAppSettingsEl {
            built_in_lifecycle_config_arn: core::default::Default::default(),
            lifecycle_config_arns: core::default::Default::default(),
            app_lifecycle_management: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElRef {
        SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `built_in_lifecycle_config_arn` after provisioning.\n"]
    pub fn built_in_lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.built_in_lifecycle_config_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_arns", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `app_lifecycle_management` after provisioning.\n"]
    pub fn app_lifecycle_management(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElAppLifecycleManagementElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.app_lifecycle_management", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElCustomImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElDefaultResourceSpecElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_resource_spec", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
    file_system_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_path: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
    #[doc = "Set the field `file_system_path`.\n"]
    pub fn set_file_system_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_system_path = Some(v.into());
        self
    }
}

impl ToListMappable
    for SagemakerUserProfileUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl
{
    type O = BlockAssignable<
        SagemakerUserProfileUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
    #[doc = ""]
    pub file_system_id: PrimField<String>,
}

impl BuildSagemakerUserProfileUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
        SagemakerUserProfileUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl {
            file_system_id: self.file_system_id,
            file_system_path: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef {
        SagemakerUserProfileUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `file_system_path` after provisioning.\n"]
    pub fn file_system_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_system_path", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElCustomFileSystemConfigElDynamic {
    efs_file_system_config: Option<
        DynamicBlock<
            SagemakerUserProfileUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCustomFileSystemConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    efs_file_system_config: Option<
        Vec<SagemakerUserProfileUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl>,
    >,
    dynamic: SagemakerUserProfileUserSettingsElCustomFileSystemConfigElDynamic,
}

impl SagemakerUserProfileUserSettingsElCustomFileSystemConfigEl {
    #[doc = "Set the field `efs_file_system_config`.\n"]
    pub fn set_efs_file_system_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.efs_file_system_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.efs_file_system_config = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElCustomFileSystemConfigEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElCustomFileSystemConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCustomFileSystemConfigEl {}

impl BuildSagemakerUserProfileUserSettingsElCustomFileSystemConfigEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElCustomFileSystemConfigEl {
        SagemakerUserProfileUserSettingsElCustomFileSystemConfigEl {
            efs_file_system_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCustomFileSystemConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElCustomFileSystemConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCustomFileSystemConfigElRef {
        SagemakerUserProfileUserSettingsElCustomFileSystemConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCustomFileSystemConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `efs_file_system_config` after provisioning.\n"]
    pub fn efs_file_system_config(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElCustomFileSystemConfigElEfsFileSystemConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.efs_file_system_config", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElCustomPosixUserConfigEl {
    gid: PrimField<f64>,
    uid: PrimField<f64>,
}

impl SagemakerUserProfileUserSettingsElCustomPosixUserConfigEl {}

impl ToListMappable for SagemakerUserProfileUserSettingsElCustomPosixUserConfigEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElCustomPosixUserConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElCustomPosixUserConfigEl {
    #[doc = ""]
    pub gid: PrimField<f64>,
    #[doc = ""]
    pub uid: PrimField<f64>,
}

impl BuildSagemakerUserProfileUserSettingsElCustomPosixUserConfigEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElCustomPosixUserConfigEl {
        SagemakerUserProfileUserSettingsElCustomPosixUserConfigEl {
            gid: self.gid,
            uid: self.uid,
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElCustomPosixUserConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElCustomPosixUserConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElCustomPosixUserConfigElRef {
        SagemakerUserProfileUserSettingsElCustomPosixUserConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElCustomPosixUserConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gid` after provisioning.\n"]
    pub fn gid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.gid", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout_in_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_management: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_idle_timeout_in_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_idle_timeout_in_minutes: Option<PrimField<f64>>,
}

impl
    SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl
{
    #[doc = "Set the field `idle_timeout_in_minutes`.\n"]
    pub fn set_idle_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.idle_timeout_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_management`.\n"]
    pub fn set_lifecycle_management(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_management = Some(v.into());
        self
    }

    #[doc = "Set the field `max_idle_timeout_in_minutes`.\n"]
    pub fn set_max_idle_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_idle_timeout_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `min_idle_timeout_in_minutes`.\n"]
    pub fn set_min_idle_timeout_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_idle_timeout_in_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    type O =
        BlockAssignable<
            SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl
{}

impl BuildSagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
        SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl {
            idle_timeout_in_minutes: core::default::Default::default(),
            lifecycle_management: core::default::Default::default(),
            max_idle_timeout_in_minutes: core::default::Default::default(),
            min_idle_timeout_in_minutes: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
        SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_timeout_in_minutes` after provisioning.\n"]
    pub fn idle_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_management` after provisioning.\n"]
    pub fn lifecycle_management(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_management", self.base))
    }

    #[doc = "Get a reference to the value of field `max_idle_timeout_in_minutes` after provisioning.\n"]
    pub fn max_idle_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_idle_timeout_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `min_idle_timeout_in_minutes` after provisioning.\n"]
    pub fn min_idle_timeout_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_idle_timeout_in_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElDynamic {
    idle_settings: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_settings: Option<
        Vec<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl>,
    >,
    dynamic: SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElDynamic,
}

impl SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
    #[doc = "Set the field `idle_settings`.\n"]
    pub fn set_idle_settings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.idle_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.idle_settings = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl
{
    type O = BlockAssignable<
        SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
}

impl BuildSagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
        SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl {
            idle_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
        SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_settings` after provisioning.\n"]
    pub fn idle_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElIdleSettingsElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.idle_settings", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
    repository_url: PrimField<String>,
}

impl SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl {}

impl ToListMappable for SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
    type O =
        BlockAssignable<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
    #[doc = ""]
    pub repository_url: PrimField<String>,
}

impl BuildSagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
        SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl {
            repository_url: self.repository_url,
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
        SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCodeRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_url", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCustomImageEl {
    app_image_config_name: PrimField<String>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_number: Option<PrimField<f64>>,
}

impl SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCustomImageEl {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_version_number = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCustomImageEl {
    type O =
        BlockAssignable<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCustomImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCustomImageEl {
    #[doc = ""]
    pub app_image_config_name: PrimField<String>,
    #[doc = ""]
    pub image_name: PrimField<String>,
}

impl BuildSagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCustomImageEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCustomImageEl {
        SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCustomImageEl {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCustomImageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCustomImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCustomImageElRef {
        SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCustomImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_image_config_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_version_number", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable
    for SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl
{
    type O = BlockAssignable<
        SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
        SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
        SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_alias", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_arn", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElEmrSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    assumable_role_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_role_arns: Option<SetField<PrimField<String>>>,
}

impl SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElEmrSettingsEl {
    #[doc = "Set the field `assumable_role_arns`.\n"]
    pub fn set_assumable_role_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.assumable_role_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role_arns`.\n"]
    pub fn set_execution_role_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.execution_role_arns = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElEmrSettingsEl {
    type O =
        BlockAssignable<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElEmrSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElJupyterLabAppSettingsElEmrSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElJupyterLabAppSettingsElEmrSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElEmrSettingsEl {
        SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElEmrSettingsEl {
            assumable_role_arns: core::default::Default::default(),
            execution_role_arns: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElEmrSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElEmrSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElEmrSettingsElRef {
        SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElEmrSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElEmrSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `assumable_role_arns` after provisioning.\n"]
    pub fn assumable_role_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.assumable_role_arns", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `execution_role_arns` after provisioning.\n"]
    pub fn execution_role_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.execution_role_arns", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDynamic {
    app_lifecycle_management: Option<
        DynamicBlock<
            SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl,
        >,
    >,
    code_repository: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl>,
    >,
    custom_image: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCustomImageEl>,
    >,
    default_resource_spec: Option<
        DynamicBlock<
            SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl,
        >,
    >,
    emr_settings: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElEmrSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElJupyterLabAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    built_in_lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_lifecycle_management: Option<
        Vec<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_repository:
        Option<Vec<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_image:
        Option<Vec<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec:
        Option<Vec<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emr_settings:
        Option<Vec<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElEmrSettingsEl>>,
    dynamic: SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElJupyterLabAppSettingsEl {
    #[doc = "Set the field `built_in_lifecycle_config_arn`.\n"]
    pub fn set_built_in_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.built_in_lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `app_lifecycle_management`.\n"]
    pub fn set_app_lifecycle_management(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.app_lifecycle_management = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.app_lifecycle_management = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCodeRepositoryEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v: impl Into<
            BlockAssignable<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCustomImageEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_image = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `emr_settings`.\n"]
    pub fn set_emr_settings(
        mut self,
        v: impl Into<
            BlockAssignable<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElEmrSettingsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.emr_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.emr_settings = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElJupyterLabAppSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElJupyterLabAppSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElJupyterLabAppSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElJupyterLabAppSettingsEl {
        SagemakerUserProfileUserSettingsElJupyterLabAppSettingsEl {
            built_in_lifecycle_config_arn: core::default::Default::default(),
            lifecycle_config_arns: core::default::Default::default(),
            app_lifecycle_management: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            emr_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElRef {
        SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `built_in_lifecycle_config_arn` after provisioning.\n"]
    pub fn built_in_lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.built_in_lifecycle_config_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_arns", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `app_lifecycle_management` after provisioning.\n"]
    pub fn app_lifecycle_management(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElAppLifecycleManagementElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.app_lifecycle_management", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElCustomImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElDefaultResourceSpecElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_resource_spec", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `emr_settings` after provisioning.\n"]
    pub fn emr_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElEmrSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.emr_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    repository_url: PrimField<String>,
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {}

impl ToListMappable
    for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl
{
    type O = BlockAssignable<
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    #[doc = ""]
    pub repository_url: PrimField<String>,
}

impl BuildSagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl {
            repository_url: self.repository_url,
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_url", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable
    for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl
{
    type O = BlockAssignable<
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
}

impl BuildSagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_alias", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_arn", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDynamic {
    code_repository: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>,
    >,
    default_resource_spec: Option<
        DynamicBlock<
            SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_repository:
        Option<Vec<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<
        Vec<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl>,
    >,
    dynamic: SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl {
    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElCodeRepositoryEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl {
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl {
            lifecycle_config_arns: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef {
        SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_arns", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElDefaultResourceSpecElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_resource_spec", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    app_image_config_name: PrimField<String>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_number: Option<PrimField<f64>>,
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_version_number = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    type O =
        BlockAssignable<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    #[doc = ""]
    pub app_image_config_name: PrimField<String>,
    #[doc = ""]
    pub image_name: PrimField<String>,
}

impl BuildSagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
        SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
        SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_image_config_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_version_number", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable
    for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl
{
    type O = BlockAssignable<
        SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
}

impl BuildSagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
        SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
        SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_alias", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_arn", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDynamic {
    custom_image: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl>,
    >,
    default_resource_spec: Option<
        DynamicBlock<
            SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_image:
        Option<Vec<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<
        Vec<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl>,
    >,
    dynamic: SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl {
    #[doc = "Set the field `lifecycle_config_arns`.\n"]
    pub fn set_lifecycle_config_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.lifecycle_config_arns = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_image = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl {
        SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl {
            lifecycle_config_arns: core::default::Default::default(),
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef {
        SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arns` after provisioning.\n"]
    pub fn lifecycle_config_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_arns", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElCustomImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElDefaultResourceSpecElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_resource_spec", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
    app_image_config_name: PrimField<String>,
    image_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_number: Option<PrimField<f64>>,
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
    #[doc = "Set the field `image_version_number`.\n"]
    pub fn set_image_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_version_number = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
    #[doc = ""]
    pub app_image_config_name: PrimField<String>,
    #[doc = ""]
    pub image_name: PrimField<String>,
}

impl BuildSagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
        SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl {
            app_image_config_name: self.app_image_config_name,
            image_name: self.image_name,
            image_version_number: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageElRef {
        SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_image_config_name` after provisioning.\n"]
    pub fn app_image_config_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_image_config_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `image_name` after provisioning.\n"]
    pub fn image_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_name", self.base))
    }

    #[doc = "Get a reference to the value of field `image_version_number` after provisioning.\n"]
    pub fn image_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_version_number", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable
    for SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl
{
    type O = BlockAssignable<
        SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
        SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
        SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_alias", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_arn", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElRSessionAppSettingsElDynamic {
    custom_image:
        Option<DynamicBlock<SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl>>,
    default_resource_spec: Option<
        DynamicBlock<SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElRSessionAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_image: Option<Vec<SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec:
        Option<Vec<SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl>>,
    dynamic: SagemakerUserProfileUserSettingsElRSessionAppSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsEl {
    #[doc = "Set the field `custom_image`.\n"]
    pub fn set_custom_image(
        mut self,
        v: impl Into<
            BlockAssignable<SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_image = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_image = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElRSessionAppSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElRSessionAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElRSessionAppSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElRSessionAppSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElRSessionAppSettingsEl {
        SagemakerUserProfileUserSettingsElRSessionAppSettingsEl {
            custom_image: core::default::Default::default(),
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef {
        SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_image` after provisioning.\n"]
    pub fn custom_image(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElRSessionAppSettingsElCustomImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_image", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElRSessionAppSettingsElDefaultResourceSpecElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_resource_spec", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElRStudioServerProAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_group: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElRStudioServerProAppSettingsEl {
    #[doc = "Set the field `access_status`.\n"]
    pub fn set_access_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_status = Some(v.into());
        self
    }

    #[doc = "Set the field `user_group`.\n"]
    pub fn set_user_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_group = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElRStudioServerProAppSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElRStudioServerProAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElRStudioServerProAppSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElRStudioServerProAppSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElRStudioServerProAppSettingsEl {
        SagemakerUserProfileUserSettingsElRStudioServerProAppSettingsEl {
            access_status: core::default::Default::default(),
            user_group: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElRStudioServerProAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElRStudioServerProAppSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElRStudioServerProAppSettingsElRef {
        SagemakerUserProfileUserSettingsElRStudioServerProAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElRStudioServerProAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_status` after provisioning.\n"]
    pub fn access_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_status", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_group` after provisioning.\n"]
    pub fn user_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_group", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElSharingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    notebook_output_option: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_output_path: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElSharingSettingsEl {
    #[doc = "Set the field `notebook_output_option`.\n"]
    pub fn set_notebook_output_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.notebook_output_option = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_kms_key_id`.\n"]
    pub fn set_s3_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_output_path`.\n"]
    pub fn set_s3_output_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_output_path = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElSharingSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElSharingSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElSharingSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElSharingSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElSharingSettingsEl {
        SagemakerUserProfileUserSettingsElSharingSettingsEl {
            notebook_output_option: core::default::Default::default(),
            s3_kms_key_id: core::default::Default::default(),
            s3_output_path: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElSharingSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElSharingSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElSharingSettingsElRef {
        SagemakerUserProfileUserSettingsElSharingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElSharingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `notebook_output_option` after provisioning.\n"]
    pub fn notebook_output_option(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notebook_output_option", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_kms_key_id` after provisioning.\n"]
    pub fn s3_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_kms_key_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_output_path", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl {
    default_ebs_volume_size_in_gb: PrimField<f64>,
    maximum_ebs_volume_size_in_gb: PrimField<f64>,
}

impl SagemakerUserProfileUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl {}

impl ToListMappable
    for SagemakerUserProfileUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl
{
    type O = BlockAssignable<
        SagemakerUserProfileUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl
{
    #[doc = ""]
    pub default_ebs_volume_size_in_gb: PrimField<f64>,
    #[doc = ""]
    pub maximum_ebs_volume_size_in_gb: PrimField<f64>,
}

impl BuildSagemakerUserProfileUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl {
        SagemakerUserProfileUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl {
            default_ebs_volume_size_in_gb: self.default_ebs_volume_size_in_gb,
            maximum_ebs_volume_size_in_gb: self.maximum_ebs_volume_size_in_gb,
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref
    for SagemakerUserProfileUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef
    {
        SagemakerUserProfileUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_ebs_volume_size_in_gb` after provisioning.\n"]
    pub fn default_ebs_volume_size_in_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_ebs_volume_size_in_gb", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `maximum_ebs_volume_size_in_gb` after provisioning.\n"]
    pub fn maximum_ebs_volume_size_in_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_ebs_volume_size_in_gb", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElSpaceStorageSettingsElDynamic {
    default_ebs_storage_settings: Option<
        DynamicBlock<
            SagemakerUserProfileUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElSpaceStorageSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_ebs_storage_settings: Option<
        Vec<SagemakerUserProfileUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl>,
    >,
    dynamic: SagemakerUserProfileUserSettingsElSpaceStorageSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElSpaceStorageSettingsEl {
    #[doc = "Set the field `default_ebs_storage_settings`.\n"]
    pub fn set_default_ebs_storage_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_ebs_storage_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_ebs_storage_settings = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElSpaceStorageSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElSpaceStorageSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElSpaceStorageSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElSpaceStorageSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElSpaceStorageSettingsEl {
        SagemakerUserProfileUserSettingsElSpaceStorageSettingsEl {
            default_ebs_storage_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElSpaceStorageSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElSpaceStorageSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElSpaceStorageSettingsElRef {
        SagemakerUserProfileUserSettingsElSpaceStorageSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElSpaceStorageSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_ebs_storage_settings` after provisioning.\n"]
    pub fn default_ebs_storage_settings(
        &self,
    ) -> ListRef<
        SagemakerUserProfileUserSettingsElSpaceStorageSettingsElDefaultEbsStorageSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_ebs_storage_settings", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElStudioWebPortalSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hidden_app_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hidden_instance_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hidden_ml_tools: Option<SetField<PrimField<String>>>,
}

impl SagemakerUserProfileUserSettingsElStudioWebPortalSettingsEl {
    #[doc = "Set the field `hidden_app_types`.\n"]
    pub fn set_hidden_app_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.hidden_app_types = Some(v.into());
        self
    }

    #[doc = "Set the field `hidden_instance_types`.\n"]
    pub fn set_hidden_instance_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.hidden_instance_types = Some(v.into());
        self
    }

    #[doc = "Set the field `hidden_ml_tools`.\n"]
    pub fn set_hidden_ml_tools(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.hidden_ml_tools = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElStudioWebPortalSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElStudioWebPortalSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElStudioWebPortalSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElStudioWebPortalSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElStudioWebPortalSettingsEl {
        SagemakerUserProfileUserSettingsElStudioWebPortalSettingsEl {
            hidden_app_types: core::default::Default::default(),
            hidden_instance_types: core::default::Default::default(),
            hidden_ml_tools: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElStudioWebPortalSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElStudioWebPortalSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElStudioWebPortalSettingsElRef {
        SagemakerUserProfileUserSettingsElStudioWebPortalSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElStudioWebPortalSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hidden_app_types` after provisioning.\n"]
    pub fn hidden_app_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.hidden_app_types", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `hidden_instance_types` after provisioning.\n"]
    pub fn hidden_instance_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.hidden_instance_types", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `hidden_ml_tools` after provisioning.\n"]
    pub fn hidden_ml_tools(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.hidden_ml_tools", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_config_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_image_version_arn: Option<PrimField<String>>,
}

impl SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_config_arn`.\n"]
    pub fn set_lifecycle_config_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_config_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_arn`.\n"]
    pub fn set_sagemaker_image_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_alias`.\n"]
    pub fn set_sagemaker_image_version_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_alias = Some(v.into());
        self
    }

    #[doc = "Set the field `sagemaker_image_version_arn`.\n"]
    pub fn set_sagemaker_image_version_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sagemaker_image_version_arn = Some(v.into());
        self
    }
}

impl ToListMappable
    for SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl
{
    type O = BlockAssignable<
        SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {}

impl BuildSagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
    pub fn build(
        self,
    ) -> SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
        SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl {
            instance_type: core::default::Default::default(),
            lifecycle_config_arn: core::default::Default::default(),
            sagemaker_image_arn: core::default::Default::default(),
            sagemaker_image_version_alias: core::default::Default::default(),
            sagemaker_image_version_arn: core::default::Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
        SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `lifecycle_config_arn` after provisioning.\n"]
    pub fn lifecycle_config_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_config_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_arn` after provisioning.\n"]
    pub fn sagemaker_image_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_alias` after provisioning.\n"]
    pub fn sagemaker_image_version_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_alias", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sagemaker_image_version_arn` after provisioning.\n"]
    pub fn sagemaker_image_version_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sagemaker_image_version_arn", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDynamic {
    default_resource_spec: Option<
        DynamicBlock<
            SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_spec: Option<
        Vec<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl>,
    >,
    dynamic: SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl {
    #[doc = "Set the field `default_resource_spec`.\n"]
    pub fn set_default_resource_spec(
        mut self,
        v: impl Into<
            BlockAssignable<
                SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_resource_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_resource_spec = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl {}

impl BuildSagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl {
        SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl {
            default_resource_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef {
        SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_resource_spec` after provisioning.\n"]
    pub fn default_resource_spec(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElDefaultResourceSpecElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_resource_spec", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileUserSettingsElDynamic {
    canvas_app_settings:
        Option<DynamicBlock<SagemakerUserProfileUserSettingsElCanvasAppSettingsEl>>,
    code_editor_app_settings:
        Option<DynamicBlock<SagemakerUserProfileUserSettingsElCodeEditorAppSettingsEl>>,
    custom_file_system_config:
        Option<DynamicBlock<SagemakerUserProfileUserSettingsElCustomFileSystemConfigEl>>,
    custom_posix_user_config:
        Option<DynamicBlock<SagemakerUserProfileUserSettingsElCustomPosixUserConfigEl>>,
    jupyter_lab_app_settings:
        Option<DynamicBlock<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsEl>>,
    jupyter_server_app_settings:
        Option<DynamicBlock<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl>>,
    kernel_gateway_app_settings:
        Option<DynamicBlock<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl>>,
    r_session_app_settings:
        Option<DynamicBlock<SagemakerUserProfileUserSettingsElRSessionAppSettingsEl>>,
    r_studio_server_pro_app_settings:
        Option<DynamicBlock<SagemakerUserProfileUserSettingsElRStudioServerProAppSettingsEl>>,
    sharing_settings: Option<DynamicBlock<SagemakerUserProfileUserSettingsElSharingSettingsEl>>,
    space_storage_settings:
        Option<DynamicBlock<SagemakerUserProfileUserSettingsElSpaceStorageSettingsEl>>,
    studio_web_portal_settings:
        Option<DynamicBlock<SagemakerUserProfileUserSettingsElStudioWebPortalSettingsEl>>,
    tensor_board_app_settings:
        Option<DynamicBlock<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl>>,
}

#[derive(Serialize)]
pub struct SagemakerUserProfileUserSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_mount_home_efs: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_landing_uri: Option<PrimField<String>>,
    execution_role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    studio_web_portal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    canvas_app_settings: Option<Vec<SagemakerUserProfileUserSettingsElCanvasAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_editor_app_settings:
        Option<Vec<SagemakerUserProfileUserSettingsElCodeEditorAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_file_system_config:
        Option<Vec<SagemakerUserProfileUserSettingsElCustomFileSystemConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_posix_user_config:
        Option<Vec<SagemakerUserProfileUserSettingsElCustomPosixUserConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jupyter_lab_app_settings:
        Option<Vec<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jupyter_server_app_settings:
        Option<Vec<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel_gateway_app_settings:
        Option<Vec<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r_session_app_settings: Option<Vec<SagemakerUserProfileUserSettingsElRSessionAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r_studio_server_pro_app_settings:
        Option<Vec<SagemakerUserProfileUserSettingsElRStudioServerProAppSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sharing_settings: Option<Vec<SagemakerUserProfileUserSettingsElSharingSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    space_storage_settings: Option<Vec<SagemakerUserProfileUserSettingsElSpaceStorageSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    studio_web_portal_settings:
        Option<Vec<SagemakerUserProfileUserSettingsElStudioWebPortalSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tensor_board_app_settings:
        Option<Vec<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl>>,
    dynamic: SagemakerUserProfileUserSettingsElDynamic,
}

impl SagemakerUserProfileUserSettingsEl {
    #[doc = "Set the field `auto_mount_home_efs`.\n"]
    pub fn set_auto_mount_home_efs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_mount_home_efs = Some(v.into());
        self
    }

    #[doc = "Set the field `default_landing_uri`.\n"]
    pub fn set_default_landing_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_landing_uri = Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `studio_web_portal`.\n"]
    pub fn set_studio_web_portal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.studio_web_portal = Some(v.into());
        self
    }

    #[doc = "Set the field `canvas_app_settings`.\n"]
    pub fn set_canvas_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElCanvasAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.canvas_app_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.canvas_app_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `code_editor_app_settings`.\n"]
    pub fn set_code_editor_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElCodeEditorAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_editor_app_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_editor_app_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `custom_file_system_config`.\n"]
    pub fn set_custom_file_system_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElCustomFileSystemConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_file_system_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_file_system_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `custom_posix_user_config`.\n"]
    pub fn set_custom_posix_user_config(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElCustomPosixUserConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_posix_user_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_posix_user_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `jupyter_lab_app_settings`.\n"]
    pub fn set_jupyter_lab_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jupyter_lab_app_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_lab_app_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `jupyter_server_app_settings`.\n"]
    pub fn set_jupyter_server_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jupyter_server_app_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jupyter_server_app_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `kernel_gateway_app_settings`.\n"]
    pub fn set_kernel_gateway_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kernel_gateway_app_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kernel_gateway_app_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `r_session_app_settings`.\n"]
    pub fn set_r_session_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElRSessionAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.r_session_app_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.r_session_app_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `r_studio_server_pro_app_settings`.\n"]
    pub fn set_r_studio_server_pro_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElRStudioServerProAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.r_studio_server_pro_app_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.r_studio_server_pro_app_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `sharing_settings`.\n"]
    pub fn set_sharing_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElSharingSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sharing_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sharing_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `space_storage_settings`.\n"]
    pub fn set_space_storage_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElSpaceStorageSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.space_storage_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.space_storage_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `studio_web_portal_settings`.\n"]
    pub fn set_studio_web_portal_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElStudioWebPortalSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.studio_web_portal_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.studio_web_portal_settings = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `tensor_board_app_settings`.\n"]
    pub fn set_tensor_board_app_settings(
        mut self,
        v: impl Into<BlockAssignable<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tensor_board_app_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tensor_board_app_settings = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SagemakerUserProfileUserSettingsEl {
    type O = BlockAssignable<SagemakerUserProfileUserSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerUserProfileUserSettingsEl {
    #[doc = ""]
    pub execution_role: PrimField<String>,
}

impl BuildSagemakerUserProfileUserSettingsEl {
    pub fn build(self) -> SagemakerUserProfileUserSettingsEl {
        SagemakerUserProfileUserSettingsEl {
            auto_mount_home_efs: core::default::Default::default(),
            default_landing_uri: core::default::Default::default(),
            execution_role: self.execution_role,
            security_groups: core::default::Default::default(),
            studio_web_portal: core::default::Default::default(),
            canvas_app_settings: core::default::Default::default(),
            code_editor_app_settings: core::default::Default::default(),
            custom_file_system_config: core::default::Default::default(),
            custom_posix_user_config: core::default::Default::default(),
            jupyter_lab_app_settings: core::default::Default::default(),
            jupyter_server_app_settings: core::default::Default::default(),
            kernel_gateway_app_settings: core::default::Default::default(),
            r_session_app_settings: core::default::Default::default(),
            r_studio_server_pro_app_settings: core::default::Default::default(),
            sharing_settings: core::default::Default::default(),
            space_storage_settings: core::default::Default::default(),
            studio_web_portal_settings: core::default::Default::default(),
            tensor_board_app_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerUserProfileUserSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerUserProfileUserSettingsElRef {
    fn new(shared: StackShared, base: String) -> SagemakerUserProfileUserSettingsElRef {
        SagemakerUserProfileUserSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerUserProfileUserSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_mount_home_efs` after provisioning.\n"]
    pub fn auto_mount_home_efs(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_mount_home_efs", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `default_landing_uri` after provisioning.\n"]
    pub fn default_landing_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_landing_uri", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `studio_web_portal` after provisioning.\n"]
    pub fn studio_web_portal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.studio_web_portal", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `canvas_app_settings` after provisioning.\n"]
    pub fn canvas_app_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElCanvasAppSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.canvas_app_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `code_editor_app_settings` after provisioning.\n"]
    pub fn code_editor_app_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElCodeEditorAppSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.code_editor_app_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `custom_file_system_config` after provisioning.\n"]
    pub fn custom_file_system_config(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElCustomFileSystemConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_file_system_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `custom_posix_user_config` after provisioning.\n"]
    pub fn custom_posix_user_config(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElCustomPosixUserConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_posix_user_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `jupyter_lab_app_settings` after provisioning.\n"]
    pub fn jupyter_lab_app_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElJupyterLabAppSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jupyter_lab_app_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `jupyter_server_app_settings` after provisioning.\n"]
    pub fn jupyter_server_app_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElJupyterServerAppSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jupyter_server_app_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `kernel_gateway_app_settings` after provisioning.\n"]
    pub fn kernel_gateway_app_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElKernelGatewayAppSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kernel_gateway_app_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `r_session_app_settings` after provisioning.\n"]
    pub fn r_session_app_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElRSessionAppSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.r_session_app_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `r_studio_server_pro_app_settings` after provisioning.\n"]
    pub fn r_studio_server_pro_app_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElRStudioServerProAppSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.r_studio_server_pro_app_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `sharing_settings` after provisioning.\n"]
    pub fn sharing_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElSharingSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sharing_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `space_storage_settings` after provisioning.\n"]
    pub fn space_storage_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElSpaceStorageSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.space_storage_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `studio_web_portal_settings` after provisioning.\n"]
    pub fn studio_web_portal_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElStudioWebPortalSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.studio_web_portal_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `tensor_board_app_settings` after provisioning.\n"]
    pub fn tensor_board_app_settings(
        &self,
    ) -> ListRef<SagemakerUserProfileUserSettingsElTensorBoardAppSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tensor_board_app_settings", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerUserProfileDynamic {
    user_settings: Option<DynamicBlock<SagemakerUserProfileUserSettingsEl>>,
}
