use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataConnectUserData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<String>>,
}

struct DataConnectUser_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataConnectUserData>,
}

#[derive(Clone)]
pub struct DataConnectUser(Rc<DataConnectUser_>);

impl DataConnectUser {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
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

    #[doc = "Set the field `user_id`.\n"]
    pub fn set_user_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_id = Some(v.into());
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

    #[doc = "Get a reference to the value of field `identity_info` after provisioning.\n"]
    pub fn identity_info(&self) -> ListRef<DataConnectUserIdentityInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity_info", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `phone_config` after provisioning.\n"]
    pub fn phone_config(&self) -> ListRef<DataConnectUserPhoneConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.phone_config", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_id", self.extract_ref()),
        )
    }
}

impl Referable for DataConnectUser {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataConnectUser {}

impl ToListMappable for DataConnectUser {
    type O = ListRef<DataConnectUserRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataConnectUser_ {
    fn extract_datasource_type(&self) -> String {
        "aws_connect_user".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataConnectUser {
    pub tf_id: String,
    #[doc = ""]
    pub instance_id: PrimField<String>,
}

impl BuildDataConnectUser {
    pub fn build(self, stack: &mut Stack) -> DataConnectUser {
        let out = DataConnectUser(Rc::new(DataConnectUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataConnectUserData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                user_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataConnectUserRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectUserRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataConnectUserRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `identity_info` after provisioning.\n"]
    pub fn identity_info(&self) -> ListRef<DataConnectUserIdentityInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity_info", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `phone_config` after provisioning.\n"]
    pub fn phone_config(&self) -> ListRef<DataConnectUserPhoneConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.phone_config", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_id", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataConnectUserIdentityInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_email: Option<PrimField<String>>,
}

impl DataConnectUserIdentityInfoEl {
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

impl ToListMappable for DataConnectUserIdentityInfoEl {
    type O = BlockAssignable<DataConnectUserIdentityInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserIdentityInfoEl {}

impl BuildDataConnectUserIdentityInfoEl {
    pub fn build(self) -> DataConnectUserIdentityInfoEl {
        DataConnectUserIdentityInfoEl {
            email: core::default::Default::default(),
            first_name: core::default::Default::default(),
            last_name: core::default::Default::default(),
            secondary_email: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserIdentityInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectUserIdentityInfoElRef {
    fn new(shared: StackShared, base: String) -> DataConnectUserIdentityInfoElRef {
        DataConnectUserIdentityInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserIdentityInfoElRef {
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
pub struct DataConnectUserPhoneConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    after_contact_work_time_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_accept: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desk_phone_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_type: Option<PrimField<String>>,
}

impl DataConnectUserPhoneConfigEl {
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

    #[doc = "Set the field `phone_type`.\n"]
    pub fn set_phone_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.phone_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataConnectUserPhoneConfigEl {
    type O = BlockAssignable<DataConnectUserPhoneConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserPhoneConfigEl {}

impl BuildDataConnectUserPhoneConfigEl {
    pub fn build(self) -> DataConnectUserPhoneConfigEl {
        DataConnectUserPhoneConfigEl {
            after_contact_work_time_limit: core::default::Default::default(),
            auto_accept: core::default::Default::default(),
            desk_phone_number: core::default::Default::default(),
            phone_type: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserPhoneConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectUserPhoneConfigElRef {
    fn new(shared: StackShared, base: String) -> DataConnectUserPhoneConfigElRef {
        DataConnectUserPhoneConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserPhoneConfigElRef {
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
