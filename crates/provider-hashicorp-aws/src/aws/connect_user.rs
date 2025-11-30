use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ConnectUserData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    directory_user_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hierarchy_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    routing_profile_id: PrimField<String>,
    security_profile_ids: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_info: Option<Vec<ConnectUserIdentityInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_config: Option<Vec<ConnectUserPhoneConfigEl>>,
    dynamic: ConnectUserDynamic,
}

struct ConnectUser_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConnectUserData>,
}

#[derive(Clone)]
pub struct ConnectUser(Rc<ConnectUser_>);

impl ConnectUser {
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

    #[doc = "Set the field `directory_user_id`.\n"]
    pub fn set_directory_user_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().directory_user_id = Some(v.into());
        self
    }

    #[doc = "Set the field `hierarchy_group_id`.\n"]
    pub fn set_hierarchy_group_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().hierarchy_group_id = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `password`.\n"]
    pub fn set_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().password = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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

    #[doc = "Set the field `identity_info`.\n"]
    pub fn set_identity_info(
        self,
        v: impl Into<BlockAssignable<ConnectUserIdentityInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().identity_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.identity_info = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `phone_config`.\n"]
    pub fn set_phone_config(self, v: impl Into<BlockAssignable<ConnectUserPhoneConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().phone_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.phone_config = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_user_id` after provisioning.\n"]
    pub fn directory_user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_user_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hierarchy_group_id` after provisioning.\n"]
    pub fn hierarchy_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hierarchy_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `routing_profile_id` after provisioning.\n"]
    pub fn routing_profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_profile_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_profile_ids` after provisioning.\n"]
    pub fn security_profile_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_profile_ids", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `identity_info` after provisioning.\n"]
    pub fn identity_info(&self) -> ListRef<ConnectUserIdentityInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity_info", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `phone_config` after provisioning.\n"]
    pub fn phone_config(&self) -> ListRef<ConnectUserPhoneConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.phone_config", self.extract_ref()),
        )
    }
}

impl Referable for ConnectUser {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for ConnectUser {}

impl ToListMappable for ConnectUser {
    type O = ListRef<ConnectUserRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ConnectUser_ {
    fn extract_resource_type(&self) -> String {
        "aws_connect_user".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConnectUser {
    pub tf_id: String,
    #[doc = ""]
    pub instance_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub routing_profile_id: PrimField<String>,
    #[doc = ""]
    pub security_profile_ids: SetField<PrimField<String>>,
}

impl BuildConnectUser {
    pub fn build(self, stack: &mut Stack) -> ConnectUser {
        let out = ConnectUser(Rc::new(ConnectUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConnectUserData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                directory_user_id: core::default::Default::default(),
                hierarchy_group_id: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: self.name,
                password: core::default::Default::default(),
                region: core::default::Default::default(),
                routing_profile_id: self.routing_profile_id,
                security_profile_ids: self.security_profile_ids,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                identity_info: core::default::Default::default(),
                phone_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConnectUserRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectUserRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl ConnectUserRef {
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

    #[doc = "Get a reference to the value of field `directory_user_id` after provisioning.\n"]
    pub fn directory_user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_user_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hierarchy_group_id` after provisioning.\n"]
    pub fn hierarchy_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hierarchy_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `routing_profile_id` after provisioning.\n"]
    pub fn routing_profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_profile_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `security_profile_ids` after provisioning.\n"]
    pub fn security_profile_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_profile_ids", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `identity_info` after provisioning.\n"]
    pub fn identity_info(&self) -> ListRef<ConnectUserIdentityInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity_info", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `phone_config` after provisioning.\n"]
    pub fn phone_config(&self) -> ListRef<ConnectUserPhoneConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.phone_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ConnectUserIdentityInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_email: Option<PrimField<String>>,
}

impl ConnectUserIdentityInfoEl {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc = "Set the field `first_name`.\n"]
    pub fn set_first_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.first_name = Some(v.into());
        self
    }

    #[doc = "Set the field `last_name`.\n"]
    pub fn set_last_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_name = Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_email`.\n"]
    pub fn set_secondary_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secondary_email = Some(v.into());
        self
    }
}

impl ToListMappable for ConnectUserIdentityInfoEl {
    type O = BlockAssignable<ConnectUserIdentityInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserIdentityInfoEl {}

impl BuildConnectUserIdentityInfoEl {
    pub fn build(self) -> ConnectUserIdentityInfoEl {
        ConnectUserIdentityInfoEl {
            email: core::default::Default::default(),
            first_name: core::default::Default::default(),
            last_name: core::default::Default::default(),
            secondary_email: core::default::Default::default(),
        }
    }
}

pub struct ConnectUserIdentityInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectUserIdentityInfoElRef {
    fn new(shared: StackShared, base: String) -> ConnectUserIdentityInfoElRef {
        ConnectUserIdentityInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserIdentityInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.base))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_email` after provisioning.\n"]
    pub fn secondary_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secondary_email", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ConnectUserPhoneConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    after_contact_work_time_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_accept: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desk_phone_number: Option<PrimField<String>>,
    phone_type: PrimField<String>,
}

impl ConnectUserPhoneConfigEl {
    #[doc = "Set the field `after_contact_work_time_limit`.\n"]
    pub fn set_after_contact_work_time_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.after_contact_work_time_limit = Some(v.into());
        self
    }

    #[doc = "Set the field `auto_accept`.\n"]
    pub fn set_auto_accept(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_accept = Some(v.into());
        self
    }

    #[doc = "Set the field `desk_phone_number`.\n"]
    pub fn set_desk_phone_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.desk_phone_number = Some(v.into());
        self
    }
}

impl ToListMappable for ConnectUserPhoneConfigEl {
    type O = BlockAssignable<ConnectUserPhoneConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserPhoneConfigEl {
    #[doc = ""]
    pub phone_type: PrimField<String>,
}

impl BuildConnectUserPhoneConfigEl {
    pub fn build(self) -> ConnectUserPhoneConfigEl {
        ConnectUserPhoneConfigEl {
            after_contact_work_time_limit: core::default::Default::default(),
            auto_accept: core::default::Default::default(),
            desk_phone_number: core::default::Default::default(),
            phone_type: self.phone_type,
        }
    }
}

pub struct ConnectUserPhoneConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectUserPhoneConfigElRef {
    fn new(shared: StackShared, base: String) -> ConnectUserPhoneConfigElRef {
        ConnectUserPhoneConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserPhoneConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `after_contact_work_time_limit` after provisioning.\n"]
    pub fn after_contact_work_time_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.after_contact_work_time_limit", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auto_accept` after provisioning.\n"]
    pub fn auto_accept(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_accept", self.base))
    }

    #[doc = "Get a reference to the value of field `desk_phone_number` after provisioning.\n"]
    pub fn desk_phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desk_phone_number", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `phone_type` after provisioning.\n"]
    pub fn phone_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConnectUserDynamic {
    identity_info: Option<DynamicBlock<ConnectUserIdentityInfoEl>>,
    phone_config: Option<DynamicBlock<ConnectUserPhoneConfigEl>>,
}
