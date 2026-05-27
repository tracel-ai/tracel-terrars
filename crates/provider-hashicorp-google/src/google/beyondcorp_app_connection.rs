use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BeyondcorpAppConnectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connectors: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_endpoint: Option<Vec<BeyondcorpAppConnectionApplicationEndpointEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway: Option<Vec<BeyondcorpAppConnectionGatewayEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BeyondcorpAppConnectionTimeoutsEl>,
    dynamic: BeyondcorpAppConnectionDynamic,
}
struct BeyondcorpAppConnection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BeyondcorpAppConnectionData>,
}
#[derive(Clone)]
pub struct BeyondcorpAppConnection(Rc<BeyondcorpAppConnection_>);
impl BeyondcorpAppConnection {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `connectors`.\nList of AppConnectors that are authorised to be associated with this AppConnection"]
    pub fn set_connectors(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().connectors = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nAn arbitrary user-provided name for the AppConnection."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nResource labels to represent user provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
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
    #[doc = "Set the field `type_`.\nThe type of network connectivity used by the AppConnection. Refer\nto https://cloud.google.com/beyondcorp/docs/reference/rest/v1/projects.locations.appConnections#type\nfor a list of possible values."]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `application_endpoint`.\n"]
    pub fn set_application_endpoint(
        self,
        v: impl Into<BlockAssignable<BeyondcorpAppConnectionApplicationEndpointEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().application_endpoint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.application_endpoint = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `gateway`.\n"]
    pub fn set_gateway(
        self,
        v: impl Into<BlockAssignable<BeyondcorpAppConnectionGatewayEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gateway = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gateway = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BeyondcorpAppConnectionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
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
    #[doc = "Get a reference to the value of field `application_endpoint` after provisioning.\n"]
    pub fn application_endpoint(&self) -> ListRef<BeyondcorpAppConnectionApplicationEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.application_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway` after provisioning.\n"]
    pub fn gateway(&self) -> ListRef<BeyondcorpAppConnectionGatewayElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gateway", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BeyondcorpAppConnectionTimeoutsElRef {
        BeyondcorpAppConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BeyondcorpAppConnection {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BeyondcorpAppConnection {}
impl ToListMappable for BeyondcorpAppConnection {
    type O = ListRef<BeyondcorpAppConnectionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BeyondcorpAppConnection_ {
    fn extract_resource_type(&self) -> String {
        "google_beyondcorp_app_connection".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBeyondcorpAppConnection {
    pub tf_id: String,
    #[doc = "ID of the AppConnection."]
    pub name: PrimField<String>,
}
impl BuildBeyondcorpAppConnection {
    pub fn build(self, stack: &mut Stack) -> BeyondcorpAppConnection {
        let out = BeyondcorpAppConnection(Rc::new(BeyondcorpAppConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BeyondcorpAppConnectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                connectors: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                type_: core::default::Default::default(),
                application_endpoint: core::default::Default::default(),
                gateway: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BeyondcorpAppConnectionRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpAppConnectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BeyondcorpAppConnectionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `application_endpoint` after provisioning.\n"]
    pub fn application_endpoint(&self) -> ListRef<BeyondcorpAppConnectionApplicationEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.application_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gateway` after provisioning.\n"]
    pub fn gateway(&self) -> ListRef<BeyondcorpAppConnectionGatewayElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gateway", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BeyondcorpAppConnectionTimeoutsElRef {
        BeyondcorpAppConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BeyondcorpAppConnectionApplicationEndpointEl {
    host: PrimField<String>,
    port: PrimField<f64>,
}
impl BeyondcorpAppConnectionApplicationEndpointEl {}
impl ToListMappable for BeyondcorpAppConnectionApplicationEndpointEl {
    type O = BlockAssignable<BeyondcorpAppConnectionApplicationEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpAppConnectionApplicationEndpointEl {
    #[doc = "Hostname or IP address of the remote application endpoint."]
    pub host: PrimField<String>,
    #[doc = "Port of the remote application endpoint."]
    pub port: PrimField<f64>,
}
impl BuildBeyondcorpAppConnectionApplicationEndpointEl {
    pub fn build(self) -> BeyondcorpAppConnectionApplicationEndpointEl {
        BeyondcorpAppConnectionApplicationEndpointEl {
            host: self.host,
            port: self.port,
        }
    }
}
pub struct BeyondcorpAppConnectionApplicationEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpAppConnectionApplicationEndpointElRef {
    fn new(shared: StackShared, base: String) -> BeyondcorpAppConnectionApplicationEndpointElRef {
        BeyondcorpAppConnectionApplicationEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpAppConnectionApplicationEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nHostname or IP address of the remote application endpoint."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort of the remote application endpoint."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct BeyondcorpAppConnectionGatewayEl {
    app_gateway: PrimField<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl BeyondcorpAppConnectionGatewayEl {
    #[doc = "Set the field `type_`.\nThe type of hosting used by the gateway. Refer to\nhttps://cloud.google.com/beyondcorp/docs/reference/rest/v1/projects.locations.appConnections#Type_1\nfor a list of possible values."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for BeyondcorpAppConnectionGatewayEl {
    type O = BlockAssignable<BeyondcorpAppConnectionGatewayEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpAppConnectionGatewayEl {
    #[doc = "AppGateway name in following format: projects/{project_id}/locations/{locationId}/appgateways/{gateway_id}."]
    pub app_gateway: PrimField<String>,
}
impl BuildBeyondcorpAppConnectionGatewayEl {
    pub fn build(self) -> BeyondcorpAppConnectionGatewayEl {
        BeyondcorpAppConnectionGatewayEl {
            app_gateway: self.app_gateway,
            type_: core::default::Default::default(),
        }
    }
}
pub struct BeyondcorpAppConnectionGatewayElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpAppConnectionGatewayElRef {
    fn new(shared: StackShared, base: String) -> BeyondcorpAppConnectionGatewayElRef {
        BeyondcorpAppConnectionGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpAppConnectionGatewayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_gateway` after provisioning.\nAppGateway name in following format: projects/{project_id}/locations/{locationId}/appgateways/{gateway_id}."]
    pub fn app_gateway(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_gateway", self.base))
    }
    #[doc = "Get a reference to the value of field `ingress_port` after provisioning.\nIngress port reserved on the gateways for this AppConnection, if not specified or zero, the default port is 19443."]
    pub fn ingress_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ingress_port", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of hosting used by the gateway. Refer to\nhttps://cloud.google.com/beyondcorp/docs/reference/rest/v1/projects.locations.appConnections#Type_1\nfor a list of possible values."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nServer-defined URI for this resource."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct BeyondcorpAppConnectionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BeyondcorpAppConnectionTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for BeyondcorpAppConnectionTimeoutsEl {
    type O = BlockAssignable<BeyondcorpAppConnectionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpAppConnectionTimeoutsEl {}
impl BuildBeyondcorpAppConnectionTimeoutsEl {
    pub fn build(self) -> BeyondcorpAppConnectionTimeoutsEl {
        BeyondcorpAppConnectionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BeyondcorpAppConnectionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpAppConnectionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BeyondcorpAppConnectionTimeoutsElRef {
        BeyondcorpAppConnectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpAppConnectionTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct BeyondcorpAppConnectionDynamic {
    application_endpoint: Option<DynamicBlock<BeyondcorpAppConnectionApplicationEndpointEl>>,
    gateway: Option<DynamicBlock<BeyondcorpAppConnectionGatewayEl>>,
}
