use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBeyondcorpAppGatewayData {
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
struct DataBeyondcorpAppGateway_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBeyondcorpAppGatewayData>,
}
#[derive(Clone)]
pub struct DataBeyondcorpAppGateway(Rc<DataBeyondcorpAppGateway_>);
impl DataBeyondcorpAppGateway {
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
    #[doc = "Set the field `region`.\nThe region of the AppGateway."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `allocated_connections` after provisioning.\nA list of connections allocated for the Gateway."]
    pub fn allocated_connections(
        &self,
    ) -> ListRef<DataBeyondcorpAppGatewayAllocatedConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allocated_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nAn arbitrary user-provided name for the AppGateway."]
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
    #[doc = "Get a reference to the value of field `host_type` after provisioning.\nThe type of hosting used by the AppGateway. Default value: \"HOST_TYPE_UNSPECIFIED\" Possible values: [\"HOST_TYPE_UNSPECIFIED\", \"GCP_REGIONAL_MIG\"]"]
    pub fn host_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_type", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nID of the AppGateway."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the AppGateway."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nRepresents the different states of a AppGateway."]
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of network connectivity used by the AppGateway. Default value: \"TYPE_UNSPECIFIED\" Possible values: [\"TYPE_UNSPECIFIED\", \"TCP_PROXY\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nServer-defined URI for this resource."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }
}
impl Referable for DataBeyondcorpAppGateway {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBeyondcorpAppGateway {}
impl ToListMappable for DataBeyondcorpAppGateway {
    type O = ListRef<DataBeyondcorpAppGatewayRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBeyondcorpAppGateway_ {
    fn extract_datasource_type(&self) -> String {
        "google_beyondcorp_app_gateway".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBeyondcorpAppGateway {
    pub tf_id: String,
    #[doc = "ID of the AppGateway."]
    pub name: PrimField<String>,
}
impl BuildDataBeyondcorpAppGateway {
    pub fn build(self, stack: &mut Stack) -> DataBeyondcorpAppGateway {
        let out = DataBeyondcorpAppGateway(Rc::new(DataBeyondcorpAppGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBeyondcorpAppGatewayData {
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
pub struct DataBeyondcorpAppGatewayRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpAppGatewayRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBeyondcorpAppGatewayRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `allocated_connections` after provisioning.\nA list of connections allocated for the Gateway."]
    pub fn allocated_connections(
        &self,
    ) -> ListRef<DataBeyondcorpAppGatewayAllocatedConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allocated_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nAn arbitrary user-provided name for the AppGateway."]
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
    #[doc = "Get a reference to the value of field `host_type` after provisioning.\nThe type of hosting used by the AppGateway. Default value: \"HOST_TYPE_UNSPECIFIED\" Possible values: [\"HOST_TYPE_UNSPECIFIED\", \"GCP_REGIONAL_MIG\"]"]
    pub fn host_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_type", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nID of the AppGateway."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the AppGateway."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nRepresents the different states of a AppGateway."]
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
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of network connectivity used by the AppGateway. Default value: \"TYPE_UNSPECIFIED\" Possible values: [\"TYPE_UNSPECIFIED\", \"TCP_PROXY\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nServer-defined URI for this resource."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }
}
#[derive(Serialize)]
pub struct DataBeyondcorpAppGatewayAllocatedConnectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_uri: Option<PrimField<String>>,
}
impl DataBeyondcorpAppGatewayAllocatedConnectionsEl {
    #[doc = "Set the field `ingress_port`.\n"]
    pub fn set_ingress_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ingress_port = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_uri`.\n"]
    pub fn set_psc_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.psc_uri = Some(v.into());
        self
    }
}
impl ToListMappable for DataBeyondcorpAppGatewayAllocatedConnectionsEl {
    type O = BlockAssignable<DataBeyondcorpAppGatewayAllocatedConnectionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBeyondcorpAppGatewayAllocatedConnectionsEl {}
impl BuildDataBeyondcorpAppGatewayAllocatedConnectionsEl {
    pub fn build(self) -> DataBeyondcorpAppGatewayAllocatedConnectionsEl {
        DataBeyondcorpAppGatewayAllocatedConnectionsEl {
            ingress_port: core::default::Default::default(),
            psc_uri: core::default::Default::default(),
        }
    }
}
pub struct DataBeyondcorpAppGatewayAllocatedConnectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpAppGatewayAllocatedConnectionsElRef {
    fn new(shared: StackShared, base: String) -> DataBeyondcorpAppGatewayAllocatedConnectionsElRef {
        DataBeyondcorpAppGatewayAllocatedConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBeyondcorpAppGatewayAllocatedConnectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ingress_port` after provisioning.\n"]
    pub fn ingress_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ingress_port", self.base))
    }
    #[doc = "Get a reference to the value of field `psc_uri` after provisioning.\n"]
    pub fn psc_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.psc_uri", self.base))
    }
}
