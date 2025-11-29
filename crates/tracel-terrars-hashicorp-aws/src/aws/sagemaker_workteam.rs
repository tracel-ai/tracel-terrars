use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerWorkteamData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    description: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workforce_name: Option<PrimField<String>>,
    workteam_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_definition: Option<Vec<SagemakerWorkteamMemberDefinitionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_configuration: Option<Vec<SagemakerWorkteamNotificationConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_access_configuration: Option<Vec<SagemakerWorkteamWorkerAccessConfigurationEl>>,
    dynamic: SagemakerWorkteamDynamic,
}

struct SagemakerWorkteam_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerWorkteamData>,
}

#[derive(Clone)]
pub struct SagemakerWorkteam(Rc<SagemakerWorkteam_>);

impl SagemakerWorkteam {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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

    #[doc = "Set the field `workforce_name`.\n"]
    pub fn set_workforce_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().workforce_name = Some(v.into());
        self
    }

    #[doc = "Set the field `member_definition`.\n"]
    pub fn set_member_definition(self, v: impl Into<BlockAssignable<SagemakerWorkteamMemberDefinitionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().member_definition = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.member_definition = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification_configuration`.\n"]
    pub fn set_notification_configuration(
        self,
        v: impl Into<BlockAssignable<SagemakerWorkteamNotificationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `worker_access_configuration`.\n"]
    pub fn set_worker_access_configuration(
        self,
        v: impl Into<BlockAssignable<SagemakerWorkteamWorkerAccessConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().worker_access_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.worker_access_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdomain` after provisioning.\n"]
    pub fn subdomain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workforce_name` after provisioning.\n"]
    pub fn workforce_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workforce_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workteam_name` after provisioning.\n"]
    pub fn workteam_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workteam_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `member_definition` after provisioning.\n"]
    pub fn member_definition(&self) -> ListRef<SagemakerWorkteamMemberDefinitionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.member_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_configuration` after provisioning.\n"]
    pub fn notification_configuration(&self) -> ListRef<SagemakerWorkteamNotificationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.notification_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_access_configuration` after provisioning.\n"]
    pub fn worker_access_configuration(&self) -> ListRef<SagemakerWorkteamWorkerAccessConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.worker_access_configuration", self.extract_ref()))
    }
}

impl Referable for SagemakerWorkteam {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SagemakerWorkteam { }

impl ToListMappable for SagemakerWorkteam {
    type O = ListRef<SagemakerWorkteamRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerWorkteam_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_workteam".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerWorkteam {
    pub tf_id: String,
    #[doc = ""]
    pub description: PrimField<String>,
    #[doc = ""]
    pub workteam_name: PrimField<String>,
}

impl BuildSagemakerWorkteam {
    pub fn build(self, stack: &mut Stack) -> SagemakerWorkteam {
        let out = SagemakerWorkteam(Rc::new(SagemakerWorkteam_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerWorkteamData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: self.description,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                workforce_name: core::default::Default::default(),
                workteam_name: self.workteam_name,
                member_definition: core::default::Default::default(),
                notification_configuration: core::default::Default::default(),
                worker_access_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerWorkteamRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerWorkteamRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SagemakerWorkteamRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdomain` after provisioning.\n"]
    pub fn subdomain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subdomain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workforce_name` after provisioning.\n"]
    pub fn workforce_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workforce_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workteam_name` after provisioning.\n"]
    pub fn workteam_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workteam_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `member_definition` after provisioning.\n"]
    pub fn member_definition(&self) -> ListRef<SagemakerWorkteamMemberDefinitionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.member_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_configuration` after provisioning.\n"]
    pub fn notification_configuration(&self) -> ListRef<SagemakerWorkteamNotificationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.notification_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_access_configuration` after provisioning.\n"]
    pub fn worker_access_configuration(&self) -> ListRef<SagemakerWorkteamWorkerAccessConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.worker_access_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerWorkteamMemberDefinitionElCognitoMemberDefinitionEl {
    client_id: PrimField<String>,
    user_group: PrimField<String>,
    user_pool: PrimField<String>,
}

impl SagemakerWorkteamMemberDefinitionElCognitoMemberDefinitionEl { }

impl ToListMappable for SagemakerWorkteamMemberDefinitionElCognitoMemberDefinitionEl {
    type O = BlockAssignable<SagemakerWorkteamMemberDefinitionElCognitoMemberDefinitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkteamMemberDefinitionElCognitoMemberDefinitionEl {
    #[doc = ""]
    pub client_id: PrimField<String>,
    #[doc = ""]
    pub user_group: PrimField<String>,
    #[doc = ""]
    pub user_pool: PrimField<String>,
}

impl BuildSagemakerWorkteamMemberDefinitionElCognitoMemberDefinitionEl {
    pub fn build(self) -> SagemakerWorkteamMemberDefinitionElCognitoMemberDefinitionEl {
        SagemakerWorkteamMemberDefinitionElCognitoMemberDefinitionEl {
            client_id: self.client_id,
            user_group: self.user_group,
            user_pool: self.user_pool,
        }
    }
}

pub struct SagemakerWorkteamMemberDefinitionElCognitoMemberDefinitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerWorkteamMemberDefinitionElCognitoMemberDefinitionElRef {
    fn new(shared: StackShared, base: String) -> SagemakerWorkteamMemberDefinitionElCognitoMemberDefinitionElRef {
        SagemakerWorkteamMemberDefinitionElCognitoMemberDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkteamMemberDefinitionElCognitoMemberDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `user_group` after provisioning.\n"]
    pub fn user_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_group", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool` after provisioning.\n"]
    pub fn user_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerWorkteamMemberDefinitionElOidcMemberDefinitionEl {
    groups: SetField<PrimField<String>>,
}

impl SagemakerWorkteamMemberDefinitionElOidcMemberDefinitionEl { }

impl ToListMappable for SagemakerWorkteamMemberDefinitionElOidcMemberDefinitionEl {
    type O = BlockAssignable<SagemakerWorkteamMemberDefinitionElOidcMemberDefinitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkteamMemberDefinitionElOidcMemberDefinitionEl {
    #[doc = ""]
    pub groups: SetField<PrimField<String>>,
}

impl BuildSagemakerWorkteamMemberDefinitionElOidcMemberDefinitionEl {
    pub fn build(self) -> SagemakerWorkteamMemberDefinitionElOidcMemberDefinitionEl {
        SagemakerWorkteamMemberDefinitionElOidcMemberDefinitionEl { groups: self.groups }
    }
}

pub struct SagemakerWorkteamMemberDefinitionElOidcMemberDefinitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerWorkteamMemberDefinitionElOidcMemberDefinitionElRef {
    fn new(shared: StackShared, base: String) -> SagemakerWorkteamMemberDefinitionElOidcMemberDefinitionElRef {
        SagemakerWorkteamMemberDefinitionElOidcMemberDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkteamMemberDefinitionElOidcMemberDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.groups", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerWorkteamMemberDefinitionElDynamic {
    cognito_member_definition: Option<DynamicBlock<SagemakerWorkteamMemberDefinitionElCognitoMemberDefinitionEl>>,
    oidc_member_definition: Option<DynamicBlock<SagemakerWorkteamMemberDefinitionElOidcMemberDefinitionEl>>,
}

#[derive(Serialize)]
pub struct SagemakerWorkteamMemberDefinitionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cognito_member_definition: Option<Vec<SagemakerWorkteamMemberDefinitionElCognitoMemberDefinitionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oidc_member_definition: Option<Vec<SagemakerWorkteamMemberDefinitionElOidcMemberDefinitionEl>>,
    dynamic: SagemakerWorkteamMemberDefinitionElDynamic,
}

impl SagemakerWorkteamMemberDefinitionEl {
    #[doc = "Set the field `cognito_member_definition`.\n"]
    pub fn set_cognito_member_definition(
        mut self,
        v: impl Into<BlockAssignable<SagemakerWorkteamMemberDefinitionElCognitoMemberDefinitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cognito_member_definition = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cognito_member_definition = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oidc_member_definition`.\n"]
    pub fn set_oidc_member_definition(
        mut self,
        v: impl Into<BlockAssignable<SagemakerWorkteamMemberDefinitionElOidcMemberDefinitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oidc_member_definition = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oidc_member_definition = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerWorkteamMemberDefinitionEl {
    type O = BlockAssignable<SagemakerWorkteamMemberDefinitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkteamMemberDefinitionEl {}

impl BuildSagemakerWorkteamMemberDefinitionEl {
    pub fn build(self) -> SagemakerWorkteamMemberDefinitionEl {
        SagemakerWorkteamMemberDefinitionEl {
            cognito_member_definition: core::default::Default::default(),
            oidc_member_definition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerWorkteamMemberDefinitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerWorkteamMemberDefinitionElRef {
    fn new(shared: StackShared, base: String) -> SagemakerWorkteamMemberDefinitionElRef {
        SagemakerWorkteamMemberDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkteamMemberDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cognito_member_definition` after provisioning.\n"]
    pub fn cognito_member_definition(&self) -> ListRef<SagemakerWorkteamMemberDefinitionElCognitoMemberDefinitionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cognito_member_definition", self.base))
    }

    #[doc = "Get a reference to the value of field `oidc_member_definition` after provisioning.\n"]
    pub fn oidc_member_definition(&self) -> ListRef<SagemakerWorkteamMemberDefinitionElOidcMemberDefinitionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oidc_member_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerWorkteamNotificationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_topic_arn: Option<PrimField<String>>,
}

impl SagemakerWorkteamNotificationConfigurationEl {
    #[doc = "Set the field `notification_topic_arn`.\n"]
    pub fn set_notification_topic_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.notification_topic_arn = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerWorkteamNotificationConfigurationEl {
    type O = BlockAssignable<SagemakerWorkteamNotificationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkteamNotificationConfigurationEl {}

impl BuildSagemakerWorkteamNotificationConfigurationEl {
    pub fn build(self) -> SagemakerWorkteamNotificationConfigurationEl {
        SagemakerWorkteamNotificationConfigurationEl { notification_topic_arn: core::default::Default::default() }
    }
}

pub struct SagemakerWorkteamNotificationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerWorkteamNotificationConfigurationElRef {
    fn new(shared: StackShared, base: String) -> SagemakerWorkteamNotificationConfigurationElRef {
        SagemakerWorkteamNotificationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkteamNotificationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `notification_topic_arn` after provisioning.\n"]
    pub fn notification_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.notification_topic_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerWorkteamWorkerAccessConfigurationElS3PresignElIamPolicyConstraintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_source_ip: Option<PrimField<String>>,
}

impl SagemakerWorkteamWorkerAccessConfigurationElS3PresignElIamPolicyConstraintsEl {
    #[doc = "Set the field `source_ip`.\n"]
    pub fn set_source_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_ip = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_source_ip`.\n"]
    pub fn set_vpc_source_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_source_ip = Some(v.into());
        self
    }
}

impl ToListMappable for SagemakerWorkteamWorkerAccessConfigurationElS3PresignElIamPolicyConstraintsEl {
    type O = BlockAssignable<SagemakerWorkteamWorkerAccessConfigurationElS3PresignElIamPolicyConstraintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkteamWorkerAccessConfigurationElS3PresignElIamPolicyConstraintsEl {}

impl BuildSagemakerWorkteamWorkerAccessConfigurationElS3PresignElIamPolicyConstraintsEl {
    pub fn build(self) -> SagemakerWorkteamWorkerAccessConfigurationElS3PresignElIamPolicyConstraintsEl {
        SagemakerWorkteamWorkerAccessConfigurationElS3PresignElIamPolicyConstraintsEl {
            source_ip: core::default::Default::default(),
            vpc_source_ip: core::default::Default::default(),
        }
    }
}

pub struct SagemakerWorkteamWorkerAccessConfigurationElS3PresignElIamPolicyConstraintsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerWorkteamWorkerAccessConfigurationElS3PresignElIamPolicyConstraintsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SagemakerWorkteamWorkerAccessConfigurationElS3PresignElIamPolicyConstraintsElRef {
        SagemakerWorkteamWorkerAccessConfigurationElS3PresignElIamPolicyConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkteamWorkerAccessConfigurationElS3PresignElIamPolicyConstraintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_source_ip` after provisioning.\n"]
    pub fn vpc_source_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_source_ip", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerWorkteamWorkerAccessConfigurationElS3PresignElDynamic {
    iam_policy_constraints: Option<
        DynamicBlock<SagemakerWorkteamWorkerAccessConfigurationElS3PresignElIamPolicyConstraintsEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerWorkteamWorkerAccessConfigurationElS3PresignEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_policy_constraints: Option<Vec<SagemakerWorkteamWorkerAccessConfigurationElS3PresignElIamPolicyConstraintsEl>>,
    dynamic: SagemakerWorkteamWorkerAccessConfigurationElS3PresignElDynamic,
}

impl SagemakerWorkteamWorkerAccessConfigurationElS3PresignEl {
    #[doc = "Set the field `iam_policy_constraints`.\n"]
    pub fn set_iam_policy_constraints(
        mut self,
        v: impl Into<BlockAssignable<SagemakerWorkteamWorkerAccessConfigurationElS3PresignElIamPolicyConstraintsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.iam_policy_constraints = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.iam_policy_constraints = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerWorkteamWorkerAccessConfigurationElS3PresignEl {
    type O = BlockAssignable<SagemakerWorkteamWorkerAccessConfigurationElS3PresignEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkteamWorkerAccessConfigurationElS3PresignEl {}

impl BuildSagemakerWorkteamWorkerAccessConfigurationElS3PresignEl {
    pub fn build(self) -> SagemakerWorkteamWorkerAccessConfigurationElS3PresignEl {
        SagemakerWorkteamWorkerAccessConfigurationElS3PresignEl {
            iam_policy_constraints: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerWorkteamWorkerAccessConfigurationElS3PresignElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerWorkteamWorkerAccessConfigurationElS3PresignElRef {
    fn new(shared: StackShared, base: String) -> SagemakerWorkteamWorkerAccessConfigurationElS3PresignElRef {
        SagemakerWorkteamWorkerAccessConfigurationElS3PresignElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkteamWorkerAccessConfigurationElS3PresignElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iam_policy_constraints` after provisioning.\n"]
    pub fn iam_policy_constraints(
        &self,
    ) -> ListRef<SagemakerWorkteamWorkerAccessConfigurationElS3PresignElIamPolicyConstraintsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iam_policy_constraints", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerWorkteamWorkerAccessConfigurationElDynamic {
    s3_presign: Option<DynamicBlock<SagemakerWorkteamWorkerAccessConfigurationElS3PresignEl>>,
}

#[derive(Serialize)]
pub struct SagemakerWorkteamWorkerAccessConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_presign: Option<Vec<SagemakerWorkteamWorkerAccessConfigurationElS3PresignEl>>,
    dynamic: SagemakerWorkteamWorkerAccessConfigurationElDynamic,
}

impl SagemakerWorkteamWorkerAccessConfigurationEl {
    #[doc = "Set the field `s3_presign`.\n"]
    pub fn set_s3_presign(
        mut self,
        v: impl Into<BlockAssignable<SagemakerWorkteamWorkerAccessConfigurationElS3PresignEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_presign = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_presign = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SagemakerWorkteamWorkerAccessConfigurationEl {
    type O = BlockAssignable<SagemakerWorkteamWorkerAccessConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerWorkteamWorkerAccessConfigurationEl {}

impl BuildSagemakerWorkteamWorkerAccessConfigurationEl {
    pub fn build(self) -> SagemakerWorkteamWorkerAccessConfigurationEl {
        SagemakerWorkteamWorkerAccessConfigurationEl {
            s3_presign: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SagemakerWorkteamWorkerAccessConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerWorkteamWorkerAccessConfigurationElRef {
    fn new(shared: StackShared, base: String) -> SagemakerWorkteamWorkerAccessConfigurationElRef {
        SagemakerWorkteamWorkerAccessConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerWorkteamWorkerAccessConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_presign` after provisioning.\n"]
    pub fn s3_presign(&self) -> ListRef<SagemakerWorkteamWorkerAccessConfigurationElS3PresignElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_presign", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerWorkteamDynamic {
    member_definition: Option<DynamicBlock<SagemakerWorkteamMemberDefinitionEl>>,
    notification_configuration: Option<DynamicBlock<SagemakerWorkteamNotificationConfigurationEl>>,
    worker_access_configuration: Option<DynamicBlock<SagemakerWorkteamWorkerAccessConfigurationEl>>,
}
