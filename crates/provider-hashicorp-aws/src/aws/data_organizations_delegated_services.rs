use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataOrganizationsDelegatedServicesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataOrganizationsDelegatedServices_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOrganizationsDelegatedServicesData>,
}

#[derive(Clone)]
pub struct DataOrganizationsDelegatedServices(Rc<DataOrganizationsDelegatedServices_>);

impl DataOrganizationsDelegatedServices {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delegated_services` after provisioning.\n"]
    pub fn delegated_services(
        &self,
    ) -> SetRef<DataOrganizationsDelegatedServicesDelegatedServicesElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.delegated_services", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl Referable for DataOrganizationsDelegatedServices {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataOrganizationsDelegatedServices {}

impl ToListMappable for DataOrganizationsDelegatedServices {
    type O = ListRef<DataOrganizationsDelegatedServicesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOrganizationsDelegatedServices_ {
    fn extract_datasource_type(&self) -> String {
        "aws_organizations_delegated_services".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOrganizationsDelegatedServices {
    pub tf_id: String,
    #[doc = ""]
    pub account_id: PrimField<String>,
}

impl BuildDataOrganizationsDelegatedServices {
    pub fn build(self, stack: &mut Stack) -> DataOrganizationsDelegatedServices {
        let out =
            DataOrganizationsDelegatedServices(Rc::new(DataOrganizationsDelegatedServices_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataOrganizationsDelegatedServicesData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    account_id: self.account_id,
                    id: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOrganizationsDelegatedServicesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationsDelegatedServicesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataOrganizationsDelegatedServicesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delegated_services` after provisioning.\n"]
    pub fn delegated_services(
        &self,
    ) -> SetRef<DataOrganizationsDelegatedServicesDelegatedServicesElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.delegated_services", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationsDelegatedServicesDelegatedServicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delegation_enabled_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_principal: Option<PrimField<String>>,
}

impl DataOrganizationsDelegatedServicesDelegatedServicesEl {
    #[doc = "Set the field `delegation_enabled_date`.\n"]
    pub fn set_delegation_enabled_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delegation_enabled_date = Some(v.into());
        self
    }

    #[doc = "Set the field `service_principal`.\n"]
    pub fn set_service_principal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_principal = Some(v.into());
        self
    }
}

impl ToListMappable for DataOrganizationsDelegatedServicesDelegatedServicesEl {
    type O = BlockAssignable<DataOrganizationsDelegatedServicesDelegatedServicesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationsDelegatedServicesDelegatedServicesEl {}

impl BuildDataOrganizationsDelegatedServicesDelegatedServicesEl {
    pub fn build(self) -> DataOrganizationsDelegatedServicesDelegatedServicesEl {
        DataOrganizationsDelegatedServicesDelegatedServicesEl {
            delegation_enabled_date: core::default::Default::default(),
            service_principal: core::default::Default::default(),
        }
    }
}

pub struct DataOrganizationsDelegatedServicesDelegatedServicesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationsDelegatedServicesDelegatedServicesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOrganizationsDelegatedServicesDelegatedServicesElRef {
        DataOrganizationsDelegatedServicesDelegatedServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsDelegatedServicesDelegatedServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delegation_enabled_date` after provisioning.\n"]
    pub fn delegation_enabled_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delegation_enabled_date", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `service_principal` after provisioning.\n"]
    pub fn service_principal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_principal", self.base),
        )
    }
}
