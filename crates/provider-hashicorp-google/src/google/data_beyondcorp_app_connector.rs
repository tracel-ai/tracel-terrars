use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBeyondcorpAppConnectorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataBeyondcorpAppConnector_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBeyondcorpAppConnectorData>,
}
#[derive(Clone)]
pub struct DataBeyondcorpAppConnector(Rc<DataBeyondcorpAppConnector_>);
impl DataBeyondcorpAppConnector {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region of the AppConnector."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nAn arbitrary user-provided name for the AppConnector."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nID of the AppConnector."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal_info` after provisioning.\nPrincipal information about the Identity of the AppConnector."]
    pub fn principal_info(&self) -> ListRef<DataBeyondcorpAppConnectorPrincipalInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.principal_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the AppConnector."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nRepresents the different states of a AppConnector."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
}
impl Referable for DataBeyondcorpAppConnector {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBeyondcorpAppConnector {}
impl ToListMappable for DataBeyondcorpAppConnector {
    type O = ListRef<DataBeyondcorpAppConnectorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBeyondcorpAppConnector_ {
    fn extract_datasource_type(&self) -> String {
        "google_beyondcorp_app_connector".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBeyondcorpAppConnector {
    pub tf_id: String,
    #[doc = "ID of the AppConnector."]
    pub name: PrimField<String>,
}
impl BuildDataBeyondcorpAppConnector {
    pub fn build(self, stack: &mut Stack) -> DataBeyondcorpAppConnector {
        let out = DataBeyondcorpAppConnector(Rc::new(DataBeyondcorpAppConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBeyondcorpAppConnectorData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBeyondcorpAppConnectorRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpAppConnectorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBeyondcorpAppConnectorRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nAn arbitrary user-provided name for the AppConnector."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nID of the AppConnector."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal_info` after provisioning.\nPrincipal information about the Identity of the AppConnector."]
    pub fn principal_info(&self) -> ListRef<DataBeyondcorpAppConnectorPrincipalInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.principal_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the AppConnector."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nRepresents the different states of a AppConnector."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
}
impl DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
}
impl ToListMappable for DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl {
    type O = BlockAssignable<DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl {}
impl BuildDataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl {
    pub fn build(self) -> DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl {
        DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl {
            email: core::default::Default::default(),
        }
    }
}
pub struct DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountElRef {
        DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBeyondcorpAppConnectorPrincipalInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<ListField<DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl>>,
}
impl DataBeyondcorpAppConnectorPrincipalInfoEl {
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(
        mut self,
        v: impl Into<ListField<DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountEl>>,
    ) -> Self {
        self.service_account = Some(v.into());
        self
    }
}
impl ToListMappable for DataBeyondcorpAppConnectorPrincipalInfoEl {
    type O = BlockAssignable<DataBeyondcorpAppConnectorPrincipalInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBeyondcorpAppConnectorPrincipalInfoEl {}
impl BuildDataBeyondcorpAppConnectorPrincipalInfoEl {
    pub fn build(self) -> DataBeyondcorpAppConnectorPrincipalInfoEl {
        DataBeyondcorpAppConnectorPrincipalInfoEl {
            service_account: core::default::Default::default(),
        }
    }
}
pub struct DataBeyondcorpAppConnectorPrincipalInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpAppConnectorPrincipalInfoElRef {
    fn new(shared: StackShared, base: String) -> DataBeyondcorpAppConnectorPrincipalInfoElRef {
        DataBeyondcorpAppConnectorPrincipalInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBeyondcorpAppConnectorPrincipalInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(
        &self,
    ) -> ListRef<DataBeyondcorpAppConnectorPrincipalInfoElServiceAccountElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
}
