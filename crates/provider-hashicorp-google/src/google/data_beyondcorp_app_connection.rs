use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBeyondcorpAppConnectionData {
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
struct DataBeyondcorpAppConnection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBeyondcorpAppConnectionData>,
}
#[derive(Clone)]
pub struct DataBeyondcorpAppConnection(Rc<DataBeyondcorpAppConnection_>);
impl DataBeyondcorpAppConnection {
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
    #[doc = "Set the field `region`.\nThe region of the AppConnection."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `application_endpoint` after provisioning.\nAddress of the remote application endpoint for the BeyondCorp AppConnection."]
    pub fn application_endpoint(
        &self,
    ) -> ListRef<DataBeyondcorpAppConnectionApplicationEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.application_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connectors` after provisioning.\nList of AppConnectors that are authorised to be associated with this AppConnection"]
    pub fn connectors(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connectors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nAn arbitrary user-provided name for the AppConnection."]
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
    #[doc = "Get a reference to the value of field `gateway` after provisioning.\nGateway used by the AppConnection."]
    pub fn gateway(&self) -> ListRef<DataBeyondcorpAppConnectionGatewayElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gateway", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nID of the AppConnection."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the AppConnection."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of network connectivity used by the AppConnection. Refer\nto https://cloud.google.com/beyondcorp/docs/reference/rest/v1/projects.locations.appConnections#type\nfor a list of possible values."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
impl Referable for DataBeyondcorpAppConnection {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBeyondcorpAppConnection {}
impl ToListMappable for DataBeyondcorpAppConnection {
    type O = ListRef<DataBeyondcorpAppConnectionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBeyondcorpAppConnection_ {
    fn extract_datasource_type(&self) -> String {
        "google_beyondcorp_app_connection".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBeyondcorpAppConnection {
    pub tf_id: String,
    #[doc = "ID of the AppConnection."]
    pub name: PrimField<String>,
}
impl BuildDataBeyondcorpAppConnection {
    pub fn build(self, stack: &mut Stack) -> DataBeyondcorpAppConnection {
        let out = DataBeyondcorpAppConnection(Rc::new(DataBeyondcorpAppConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBeyondcorpAppConnectionData {
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
pub struct DataBeyondcorpAppConnectionRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpAppConnectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBeyondcorpAppConnectionRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `application_endpoint` after provisioning.\nAddress of the remote application endpoint for the BeyondCorp AppConnection."]
    pub fn application_endpoint(
        &self,
    ) -> ListRef<DataBeyondcorpAppConnectionApplicationEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.application_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connectors` after provisioning.\nList of AppConnectors that are authorised to be associated with this AppConnection"]
    pub fn connectors(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connectors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nAn arbitrary user-provided name for the AppConnection."]
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
    #[doc = "Get a reference to the value of field `gateway` after provisioning.\nGateway used by the AppConnection."]
    pub fn gateway(&self) -> ListRef<DataBeyondcorpAppConnectionGatewayElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gateway", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nID of the AppConnection."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the AppConnection."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of network connectivity used by the AppConnection. Refer\nto https://cloud.google.com/beyondcorp/docs/reference/rest/v1/projects.locations.appConnections#type\nfor a list of possible values."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataBeyondcorpAppConnectionApplicationEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}
impl DataBeyondcorpAppConnectionApplicationEndpointEl {
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}
impl ToListMappable for DataBeyondcorpAppConnectionApplicationEndpointEl {
    type O = BlockAssignable<DataBeyondcorpAppConnectionApplicationEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBeyondcorpAppConnectionApplicationEndpointEl {}
impl BuildDataBeyondcorpAppConnectionApplicationEndpointEl {
    pub fn build(self) -> DataBeyondcorpAppConnectionApplicationEndpointEl {
        DataBeyondcorpAppConnectionApplicationEndpointEl {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}
pub struct DataBeyondcorpAppConnectionApplicationEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpAppConnectionApplicationEndpointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBeyondcorpAppConnectionApplicationEndpointElRef {
        DataBeyondcorpAppConnectionApplicationEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBeyondcorpAppConnectionApplicationEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBeyondcorpAppConnectionGatewayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    app_gateway: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_port: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl DataBeyondcorpAppConnectionGatewayEl {
    #[doc = "Set the field `app_gateway`.\n"]
    pub fn set_app_gateway(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.app_gateway = Some(v.into());
        self
    }
    #[doc = "Set the field `ingress_port`.\n"]
    pub fn set_ingress_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ingress_port = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for DataBeyondcorpAppConnectionGatewayEl {
    type O = BlockAssignable<DataBeyondcorpAppConnectionGatewayEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBeyondcorpAppConnectionGatewayEl {}
impl BuildDataBeyondcorpAppConnectionGatewayEl {
    pub fn build(self) -> DataBeyondcorpAppConnectionGatewayEl {
        DataBeyondcorpAppConnectionGatewayEl {
            app_gateway: core::default::Default::default(),
            ingress_port: core::default::Default::default(),
            type_: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}
pub struct DataBeyondcorpAppConnectionGatewayElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpAppConnectionGatewayElRef {
    fn new(shared: StackShared, base: String) -> DataBeyondcorpAppConnectionGatewayElRef {
        DataBeyondcorpAppConnectionGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBeyondcorpAppConnectionGatewayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_gateway` after provisioning.\n"]
    pub fn app_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_gateway", self.base))
    }
    #[doc = "Get a reference to the value of field `ingress_port` after provisioning.\n"]
    pub fn ingress_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ingress_port", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
