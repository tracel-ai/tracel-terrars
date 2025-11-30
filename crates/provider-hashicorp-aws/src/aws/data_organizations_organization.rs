use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataOrganizationsOrganizationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataOrganizationsOrganization_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOrganizationsOrganizationData>,
}

#[derive(Clone)]
pub struct DataOrganizationsOrganization(Rc<DataOrganizationsOrganization_>);

impl DataOrganizationsOrganization {
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
    pub fn accounts(&self) -> ListRef<DataOrganizationsOrganizationAccountsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accounts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_service_access_principals` after provisioning.\n"]
    pub fn aws_service_access_principals(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.aws_service_access_principals", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enabled_policy_types` after provisioning.\n"]
    pub fn enabled_policy_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.enabled_policy_types", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `feature_set` after provisioning.\n"]
    pub fn feature_set(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.feature_set", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_account_arn` after provisioning.\n"]
    pub fn master_account_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_account_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `master_account_email` after provisioning.\n"]
    pub fn master_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_account_email", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `master_account_id` after provisioning.\n"]
    pub fn master_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `master_account_name` after provisioning.\n"]
    pub fn master_account_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_account_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `non_master_accounts` after provisioning.\n"]
    pub fn non_master_accounts(
        &self,
    ) -> ListRef<DataOrganizationsOrganizationNonMasterAccountsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.non_master_accounts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `roots` after provisioning.\n"]
    pub fn roots(&self) -> ListRef<DataOrganizationsOrganizationRootsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.roots", self.extract_ref()),
        )
    }
}

impl Referable for DataOrganizationsOrganization {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataOrganizationsOrganization {}

impl ToListMappable for DataOrganizationsOrganization {
    type O = ListRef<DataOrganizationsOrganizationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOrganizationsOrganization_ {
    fn extract_datasource_type(&self) -> String {
        "aws_organizations_organization".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOrganizationsOrganization {
    pub tf_id: String,
}

impl BuildDataOrganizationsOrganization {
    pub fn build(self, stack: &mut Stack) -> DataOrganizationsOrganization {
        let out = DataOrganizationsOrganization(Rc::new(DataOrganizationsOrganization_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOrganizationsOrganizationData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOrganizationsOrganizationRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationsOrganizationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataOrganizationsOrganizationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> ListRef<DataOrganizationsOrganizationAccountsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accounts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_service_access_principals` after provisioning.\n"]
    pub fn aws_service_access_principals(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.aws_service_access_principals", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enabled_policy_types` after provisioning.\n"]
    pub fn enabled_policy_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.enabled_policy_types", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `feature_set` after provisioning.\n"]
    pub fn feature_set(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.feature_set", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_account_arn` after provisioning.\n"]
    pub fn master_account_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_account_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `master_account_email` after provisioning.\n"]
    pub fn master_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_account_email", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `master_account_id` after provisioning.\n"]
    pub fn master_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `master_account_name` after provisioning.\n"]
    pub fn master_account_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_account_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `non_master_accounts` after provisioning.\n"]
    pub fn non_master_accounts(
        &self,
    ) -> ListRef<DataOrganizationsOrganizationNonMasterAccountsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.non_master_accounts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `roots` after provisioning.\n"]
    pub fn roots(&self) -> ListRef<DataOrganizationsOrganizationRootsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.roots", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataOrganizationsOrganizationAccountsEl {
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

impl DataOrganizationsOrganizationAccountsEl {
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

impl ToListMappable for DataOrganizationsOrganizationAccountsEl {
    type O = BlockAssignable<DataOrganizationsOrganizationAccountsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationsOrganizationAccountsEl {}

impl BuildDataOrganizationsOrganizationAccountsEl {
    pub fn build(self) -> DataOrganizationsOrganizationAccountsEl {
        DataOrganizationsOrganizationAccountsEl {
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

pub struct DataOrganizationsOrganizationAccountsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationsOrganizationAccountsElRef {
    fn new(shared: StackShared, base: String) -> DataOrganizationsOrganizationAccountsElRef {
        DataOrganizationsOrganizationAccountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsOrganizationAccountsElRef {
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.joined_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `joined_timestamp` after provisioning.\n"]
    pub fn joined_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.joined_timestamp", self.base),
        )
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

#[derive(Serialize)]
pub struct DataOrganizationsOrganizationNonMasterAccountsEl {
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

impl DataOrganizationsOrganizationNonMasterAccountsEl {
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

impl ToListMappable for DataOrganizationsOrganizationNonMasterAccountsEl {
    type O = BlockAssignable<DataOrganizationsOrganizationNonMasterAccountsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationsOrganizationNonMasterAccountsEl {}

impl BuildDataOrganizationsOrganizationNonMasterAccountsEl {
    pub fn build(self) -> DataOrganizationsOrganizationNonMasterAccountsEl {
        DataOrganizationsOrganizationNonMasterAccountsEl {
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

pub struct DataOrganizationsOrganizationNonMasterAccountsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationsOrganizationNonMasterAccountsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOrganizationsOrganizationNonMasterAccountsElRef {
        DataOrganizationsOrganizationNonMasterAccountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsOrganizationNonMasterAccountsElRef {
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.joined_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `joined_timestamp` after provisioning.\n"]
    pub fn joined_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.joined_timestamp", self.base),
        )
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

#[derive(Serialize)]
pub struct DataOrganizationsOrganizationRootsElPolicyTypesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataOrganizationsOrganizationRootsElPolicyTypesEl {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataOrganizationsOrganizationRootsElPolicyTypesEl {
    type O = BlockAssignable<DataOrganizationsOrganizationRootsElPolicyTypesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationsOrganizationRootsElPolicyTypesEl {}

impl BuildDataOrganizationsOrganizationRootsElPolicyTypesEl {
    pub fn build(self) -> DataOrganizationsOrganizationRootsElPolicyTypesEl {
        DataOrganizationsOrganizationRootsElPolicyTypesEl {
            status: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataOrganizationsOrganizationRootsElPolicyTypesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationsOrganizationRootsElPolicyTypesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOrganizationsOrganizationRootsElPolicyTypesElRef {
        DataOrganizationsOrganizationRootsElPolicyTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsOrganizationRootsElPolicyTypesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationsOrganizationRootsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_types: Option<ListField<DataOrganizationsOrganizationRootsElPolicyTypesEl>>,
}

impl DataOrganizationsOrganizationRootsEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
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

    #[doc = "Set the field `policy_types`.\n"]
    pub fn set_policy_types(
        mut self,
        v: impl Into<ListField<DataOrganizationsOrganizationRootsElPolicyTypesEl>>,
    ) -> Self {
        self.policy_types = Some(v.into());
        self
    }
}

impl ToListMappable for DataOrganizationsOrganizationRootsEl {
    type O = BlockAssignable<DataOrganizationsOrganizationRootsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationsOrganizationRootsEl {}

impl BuildDataOrganizationsOrganizationRootsEl {
    pub fn build(self) -> DataOrganizationsOrganizationRootsEl {
        DataOrganizationsOrganizationRootsEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            policy_types: core::default::Default::default(),
        }
    }
}

pub struct DataOrganizationsOrganizationRootsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationsOrganizationRootsElRef {
    fn new(shared: StackShared, base: String) -> DataOrganizationsOrganizationRootsElRef {
        DataOrganizationsOrganizationRootsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsOrganizationRootsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_types` after provisioning.\n"]
    pub fn policy_types(&self) -> ListRef<DataOrganizationsOrganizationRootsElPolicyTypesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.policy_types", self.base))
    }
}
