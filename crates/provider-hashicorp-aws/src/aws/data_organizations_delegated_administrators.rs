use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataOrganizationsDelegatedAdministratorsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_principal: Option<PrimField<String>>,
}
struct DataOrganizationsDelegatedAdministrators_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOrganizationsDelegatedAdministratorsData>,
}
#[derive(Clone)]
pub struct DataOrganizationsDelegatedAdministrators(Rc<DataOrganizationsDelegatedAdministrators_>);
impl DataOrganizationsDelegatedAdministrators {
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
    #[doc = "Set the field `service_principal`.\n"]
    pub fn set_service_principal(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_principal = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `delegated_administrators` after provisioning.\n"]
    pub fn delegated_administrators(
        &self,
    ) -> SetRef<DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.delegated_administrators", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `service_principal` after provisioning.\n"]
    pub fn service_principal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_principal", self.extract_ref()),
        )
    }
}
impl Referable for DataOrganizationsDelegatedAdministrators {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataOrganizationsDelegatedAdministrators {}
impl ToListMappable for DataOrganizationsDelegatedAdministrators {
    type O = ListRef<DataOrganizationsDelegatedAdministratorsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataOrganizationsDelegatedAdministrators_ {
    fn extract_datasource_type(&self) -> String {
        "aws_organizations_delegated_administrators".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataOrganizationsDelegatedAdministrators {
    pub tf_id: String,
}
impl BuildDataOrganizationsDelegatedAdministrators {
    pub fn build(self, stack: &mut Stack) -> DataOrganizationsDelegatedAdministrators {
        let out = DataOrganizationsDelegatedAdministrators(Rc::new(
            DataOrganizationsDelegatedAdministrators_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataOrganizationsDelegatedAdministratorsData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    service_principal: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataOrganizationsDelegatedAdministratorsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOrganizationsDelegatedAdministratorsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataOrganizationsDelegatedAdministratorsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `delegated_administrators` after provisioning.\n"]
    pub fn delegated_administrators(
        &self,
    ) -> SetRef<DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.delegated_administrators", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `service_principal` after provisioning.\n"]
    pub fn service_principal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_principal", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delegation_enabled_date: Option<PrimField<String>>,
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
    status: Option<PrimField<String>>,
}
impl DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }
    #[doc = "Set the field `delegation_enabled_date`.\n"]
    pub fn set_delegation_enabled_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delegation_enabled_date = Some(v.into());
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
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsEl {
    type O = BlockAssignable<DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOrganizationsDelegatedAdministratorsDelegatedAdministratorsEl {}
impl BuildDataOrganizationsDelegatedAdministratorsDelegatedAdministratorsEl {
    pub fn build(self) -> DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsEl {
        DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsEl {
            arn: core::default::Default::default(),
            delegation_enabled_date: core::default::Default::default(),
            email: core::default::Default::default(),
            id: core::default::Default::default(),
            joined_method: core::default::Default::default(),
            joined_timestamp: core::default::Default::default(),
            name: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsElRef {
        DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
    #[doc = "Get a reference to the value of field `delegation_enabled_date` after provisioning.\n"]
    pub fn delegation_enabled_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delegation_enabled_date", self.base),
        )
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
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
