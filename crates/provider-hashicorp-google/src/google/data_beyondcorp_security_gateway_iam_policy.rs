use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBeyondcorpSecurityGatewayIamPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    security_gateway_id: PrimField<String>,
}
struct DataBeyondcorpSecurityGatewayIamPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBeyondcorpSecurityGatewayIamPolicyData>,
}
#[derive(Clone)]
pub struct DataBeyondcorpSecurityGatewayIamPolicy(Rc<DataBeyondcorpSecurityGatewayIamPolicy_>);
impl DataBeyondcorpSecurityGatewayIamPolicy {
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
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_data` after provisioning.\n"]
    pub fn policy_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_gateway_id` after provisioning.\n"]
    pub fn security_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_gateway_id", self.extract_ref()),
        )
    }
}
impl Referable for DataBeyondcorpSecurityGatewayIamPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBeyondcorpSecurityGatewayIamPolicy {}
impl ToListMappable for DataBeyondcorpSecurityGatewayIamPolicy {
    type O = ListRef<DataBeyondcorpSecurityGatewayIamPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBeyondcorpSecurityGatewayIamPolicy_ {
    fn extract_datasource_type(&self) -> String {
        "google_beyondcorp_security_gateway_iam_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBeyondcorpSecurityGatewayIamPolicy {
    pub tf_id: String,
    #[doc = ""]
    pub security_gateway_id: PrimField<String>,
}
impl BuildDataBeyondcorpSecurityGatewayIamPolicy {
    pub fn build(self, stack: &mut Stack) -> DataBeyondcorpSecurityGatewayIamPolicy {
        let out = DataBeyondcorpSecurityGatewayIamPolicy(Rc::new(
            DataBeyondcorpSecurityGatewayIamPolicy_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataBeyondcorpSecurityGatewayIamPolicyData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    location: core::default::Default::default(),
                    project: core::default::Default::default(),
                    security_gateway_id: self.security_gateway_id,
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBeyondcorpSecurityGatewayIamPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpSecurityGatewayIamPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBeyondcorpSecurityGatewayIamPolicyRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_data` after provisioning.\n"]
    pub fn policy_data(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_gateway_id` after provisioning.\n"]
    pub fn security_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_gateway_id", self.extract_ref()),
        )
    }
}
