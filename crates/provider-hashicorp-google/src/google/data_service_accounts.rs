use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataServiceAccountsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
}
struct DataServiceAccounts_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataServiceAccountsData>,
}
#[derive(Clone)]
pub struct DataServiceAccounts(Rc<DataServiceAccounts_>);
impl DataServiceAccounts {
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
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().regex = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> ListRef<DataServiceAccountsAccountsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accounts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.regex", self.extract_ref()),
        )
    }
}
impl Referable for DataServiceAccounts {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataServiceAccounts {}
impl ToListMappable for DataServiceAccounts {
    type O = ListRef<DataServiceAccountsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataServiceAccounts_ {
    fn extract_datasource_type(&self) -> String {
        "google_service_accounts".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataServiceAccounts {
    pub tf_id: String,
}
impl BuildDataServiceAccounts {
    pub fn build(self, stack: &mut Stack) -> DataServiceAccounts {
        let out = DataServiceAccounts(Rc::new(DataServiceAccounts_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataServiceAccountsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                prefix: core::default::Default::default(),
                project: core::default::Default::default(),
                regex: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataServiceAccountsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataServiceAccountsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataServiceAccountsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> ListRef<DataServiceAccountsAccountsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accounts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.regex", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataServiceAccountsAccountsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_id: Option<PrimField<String>>,
}
impl DataServiceAccountsAccountsEl {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account_id = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
    #[doc = "Set the field `member`.\n"]
    pub fn set_member(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.member = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `unique_id`.\n"]
    pub fn set_unique_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unique_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataServiceAccountsAccountsEl {
    type O = BlockAssignable<DataServiceAccountsAccountsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataServiceAccountsAccountsEl {}
impl BuildDataServiceAccountsAccountsEl {
    pub fn build(self) -> DataServiceAccountsAccountsEl {
        DataServiceAccountsAccountsEl {
            account_id: core::default::Default::default(),
            disabled: core::default::Default::default(),
            display_name: core::default::Default::default(),
            email: core::default::Default::default(),
            member: core::default::Default::default(),
            name: core::default::Default::default(),
            unique_id: core::default::Default::default(),
        }
    }
}
pub struct DataServiceAccountsAccountsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataServiceAccountsAccountsElRef {
    fn new(shared: StackShared, base: String) -> DataServiceAccountsAccountsElRef {
        DataServiceAccountsAccountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataServiceAccountsAccountsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
    #[doc = "Get a reference to the value of field `member` after provisioning.\n"]
    pub fn member(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.member", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `unique_id` after provisioning.\n"]
    pub fn unique_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unique_id", self.base))
    }
}
