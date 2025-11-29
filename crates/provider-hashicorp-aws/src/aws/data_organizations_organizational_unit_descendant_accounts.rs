use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOrganizationsOrganizationalUnitDescendantAccountsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    parent_id: PrimField<String>,
}

struct DataOrganizationsOrganizationalUnitDescendantAccounts_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOrganizationsOrganizationalUnitDescendantAccountsData>,
}

#[derive(Clone)]
pub struct DataOrganizationsOrganizationalUnitDescendantAccounts(
    Rc<DataOrganizationsOrganizationalUnitDescendantAccounts_>,
);

impl DataOrganizationsOrganizationalUnitDescendantAccounts {
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

    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> ListRef<DataOrganizationsOrganizationalUnitDescendantAccountsAccountsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.accounts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_id` after provisioning.\n"]
    pub fn parent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_id", self.extract_ref()))
    }
}

impl Referable for DataOrganizationsOrganizationalUnitDescendantAccounts {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataOrganizationsOrganizationalUnitDescendantAccounts { }

impl ToListMappable for DataOrganizationsOrganizationalUnitDescendantAccounts {
    type O = ListRef<DataOrganizationsOrganizationalUnitDescendantAccountsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOrganizationsOrganizationalUnitDescendantAccounts_ {
    fn extract_datasource_type(&self) -> String {
        "aws_organizations_organizational_unit_descendant_accounts".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOrganizationsOrganizationalUnitDescendantAccounts {
    pub tf_id: String,
    #[doc = ""]
    pub parent_id: PrimField<String>,
}

impl BuildDataOrganizationsOrganizationalUnitDescendantAccounts {
    pub fn build(self, stack: &mut Stack) -> DataOrganizationsOrganizationalUnitDescendantAccounts {
        let out =
            DataOrganizationsOrganizationalUnitDescendantAccounts(
                Rc::new(DataOrganizationsOrganizationalUnitDescendantAccounts_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: RefCell::new(DataOrganizationsOrganizationalUnitDescendantAccountsData {
                        depends_on: core::default::Default::default(),
                        provider: None,
                        for_each: None,
                        id: core::default::Default::default(),
                        parent_id: self.parent_id,
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOrganizationsOrganizationalUnitDescendantAccountsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationsOrganizationalUnitDescendantAccountsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataOrganizationsOrganizationalUnitDescendantAccountsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> ListRef<DataOrganizationsOrganizationalUnitDescendantAccountsAccountsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.accounts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_id` after provisioning.\n"]
    pub fn parent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationsOrganizationalUnitDescendantAccountsAccountsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    joined_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    joined_timestamp: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl DataOrganizationsOrganizationalUnitDescendantAccountsAccountsEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `joined_method`.\n"]
    pub fn set_joined_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.joined_method = Some(v.into());
        self
    }

    #[doc = "Set the field `joined_timestamp`.\n"]
    pub fn set_joined_timestamp(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.joined_timestamp = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for DataOrganizationsOrganizationalUnitDescendantAccountsAccountsEl {
    type O = BlockAssignable<DataOrganizationsOrganizationalUnitDescendantAccountsAccountsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationsOrganizationalUnitDescendantAccountsAccountsEl {}

impl BuildDataOrganizationsOrganizationalUnitDescendantAccountsAccountsEl {
    pub fn build(self) -> DataOrganizationsOrganizationalUnitDescendantAccountsAccountsEl {
        DataOrganizationsOrganizationalUnitDescendantAccountsAccountsEl {
            arn: core::default::Default::default(),
            email: core::default::Default::default(),
            id: core::default::Default::default(),
            joined_method: core::default::Default::default(),
            joined_timestamp: core::default::Default::default(),
            name: core::default::Default::default(),
            state: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct DataOrganizationsOrganizationalUnitDescendantAccountsAccountsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationsOrganizationalUnitDescendantAccountsAccountsElRef {
    fn new(shared: StackShared, base: String) -> DataOrganizationsOrganizationalUnitDescendantAccountsAccountsElRef {
        DataOrganizationsOrganizationalUnitDescendantAccountsAccountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsOrganizationalUnitDescendantAccountsAccountsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `joined_method` after provisioning.\n"]
    pub fn joined_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.joined_method", self.base))
    }

    #[doc = "Get a reference to the value of field `joined_timestamp` after provisioning.\n"]
    pub fn joined_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.joined_timestamp", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
