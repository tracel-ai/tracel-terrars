use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataAppmeshGatewayRouteData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    mesh_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mesh_owner: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    virtual_gateway_name: PrimField<String>,
}
struct DataAppmeshGatewayRoute_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAppmeshGatewayRouteData>,
}
#[derive(Clone)]
pub struct DataAppmeshGatewayRoute(Rc<DataAppmeshGatewayRoute_>);
impl DataAppmeshGatewayRoute {
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
    #[doc = "Set the field `mesh_owner`.\n"]
    pub fn set_mesh_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mesh_owner = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mesh_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mesh_owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<DataAppmeshGatewayRouteSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `virtual_gateway_name` after provisioning.\n"]
    pub fn virtual_gateway_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.virtual_gateway_name", self.extract_ref()),
        )
    }
}
impl Referable for DataAppmeshGatewayRoute {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataAppmeshGatewayRoute {}
impl ToListMappable for DataAppmeshGatewayRoute {
    type O = ListRef<DataAppmeshGatewayRouteRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataAppmeshGatewayRoute_ {
    fn extract_datasource_type(&self) -> String {
        "aws_appmesh_gateway_route".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataAppmeshGatewayRoute {
    pub tf_id: String,
    #[doc = ""]
    pub mesh_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub virtual_gateway_name: PrimField<String>,
}
impl BuildDataAppmeshGatewayRoute {
    pub fn build(self, stack: &mut Stack) -> DataAppmeshGatewayRoute {
        let out = DataAppmeshGatewayRoute(Rc::new(DataAppmeshGatewayRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAppmeshGatewayRouteData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                mesh_name: self.mesh_name,
                mesh_owner: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                virtual_gateway_name: self.virtual_gateway_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataAppmeshGatewayRouteRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataAppmeshGatewayRouteRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mesh_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mesh_owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<DataAppmeshGatewayRouteSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `virtual_gateway_name` after provisioning.\n"]
    pub fn virtual_gateway_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.virtual_gateway_name", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_service_name: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
    #[doc = "Set the field `virtual_service_name`.\n"]
    pub fn set_virtual_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.virtual_service_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
    type O =
        BlockAssignable<DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {}
impl BuildDataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
        DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
            virtual_service_name: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef {
        DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `virtual_service_name` after provisioning.\n"]
    pub fn virtual_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.virtual_service_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_service:
        Option<ListField<DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl>>,
}
impl DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `virtual_service`.\n"]
    pub fn set_virtual_service(
        mut self,
        v: impl Into<
            ListField<DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl>,
        >,
    ) -> Self {
        self.virtual_service = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {}
impl BuildDataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
        DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
            port: core::default::Default::default(),
            virtual_service: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef {
        DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `virtual_service` after provisioning.\n"]
    pub fn virtual_service(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_service", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElGrpcRouteElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<ListField<DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl>>,
}
impl DataAppmeshGatewayRouteSpecElGrpcRouteElActionEl {
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl>>,
    ) -> Self {
        self.target = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElGrpcRouteElActionEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElGrpcRouteElActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElGrpcRouteElActionEl {}
impl BuildDataAppmeshGatewayRouteSpecElGrpcRouteElActionEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElGrpcRouteElActionEl {
        DataAppmeshGatewayRouteSpecElGrpcRouteElActionEl {
            target: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElGrpcRouteElActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElGrpcRouteElActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElGrpcRouteElActionElRef {
        DataAppmeshGatewayRouteSpecElGrpcRouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElGrpcRouteElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<DataAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_name: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `service_name`.\n"]
    pub fn set_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElGrpcRouteElMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElGrpcRouteElMatchEl {}
impl BuildDataAppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
        DataAppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
            port: core::default::Default::default(),
            service_name: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElGrpcRouteElMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElGrpcRouteElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElGrpcRouteElMatchElRef {
        DataAppmeshGatewayRouteSpecElGrpcRouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElGrpcRouteElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElGrpcRouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<ListField<DataAppmeshGatewayRouteSpecElGrpcRouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<DataAppmeshGatewayRouteSpecElGrpcRouteElMatchEl>>,
}
impl DataAppmeshGatewayRouteSpecElGrpcRouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElGrpcRouteElActionEl>>,
    ) -> Self {
        self.action = Some(v.into());
        self
    }
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElGrpcRouteElMatchEl>>,
    ) -> Self {
        self.match_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElGrpcRouteEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElGrpcRouteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElGrpcRouteEl {}
impl BuildDataAppmeshGatewayRouteSpecElGrpcRouteEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElGrpcRouteEl {
        DataAppmeshGatewayRouteSpecElGrpcRouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElGrpcRouteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElGrpcRouteElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshGatewayRouteSpecElGrpcRouteElRef {
        DataAppmeshGatewayRouteSpecElGrpcRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElGrpcRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<DataAppmeshGatewayRouteSpecElGrpcRouteElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<DataAppmeshGatewayRouteSpecElGrpcRouteElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_target_hostname: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
    #[doc = "Set the field `default_target_hostname`.\n"]
    pub fn set_default_target_hostname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_target_hostname = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
        DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
            default_target_hostname: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef {
        DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_target_hostname` after provisioning.\n"]
    pub fn default_target_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_target_hostname", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPathEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPathEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPathEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPathEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPathEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPathEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPathEl {
        DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPathEl {
            exact: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPathElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPathElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPathElRef {
        DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
    #[doc = "Set the field `default_prefix`.\n"]
    pub fn set_default_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
        DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
            default_prefix: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef {
        DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_prefix` after provisioning.\n"]
    pub fn default_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname:
        Option<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPathEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl>>,
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl>>,
    ) -> Self {
        self.hostname = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPathEl>>,
    ) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl>>,
    ) -> Self {
        self.prefix = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
        DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
            hostname: core::default::Default::default(),
            path: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef {
        DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPathElRef> {
        ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef> {
        ListRef::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_service_name: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
    #[doc = "Set the field `virtual_service_name`.\n"]
    pub fn set_virtual_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.virtual_service_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
    type O =
        BlockAssignable<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
    pub fn build(
        self,
    ) -> DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
        DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
            virtual_service_name: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef {
        DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `virtual_service_name` after provisioning.\n"]
    pub fn virtual_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.virtual_service_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_service: Option<
        ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl>,
    >,
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `virtual_service`.\n"]
    pub fn set_virtual_service(
        mut self,
        v: impl Into<
            ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl>,
        >,
    ) -> Self {
        self.virtual_service = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
        DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
            port: core::default::Default::default(),
            virtual_service: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef {
        DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `virtual_service` after provisioning.\n"]
    pub fn virtual_service(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_service", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rewrite: Option<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl>>,
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElActionEl {
    #[doc = "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl>>,
    ) -> Self {
        self.rewrite = Some(v.into());
        self
    }
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl>>,
    ) -> Self {
        self.target = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttp2RouteElActionEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttp2RouteElActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttp2RouteElActionEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttp2RouteElActionEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttp2RouteElActionEl {
        DataAppmeshGatewayRouteSpecElHttp2RouteElActionEl {
            rewrite: core::default::Default::default(),
            target: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRef {
        DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<f64>>,
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.end = Some(v.into());
        self
    }
    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.start = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
    type O =
        BlockAssignable<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
        DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
        DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range:
        Option<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl>>,
    ) -> Self {
        self.range = Some(v.into());
        self
    }
    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix`.\n"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
        DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            range: core::default::Default::default(),
            regex: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
        DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\n"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    invert: Option<PrimField<bool>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderEl {
    #[doc = "Set the field `invert`.\n"]
    pub fn set_invert(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert = Some(v.into());
        self
    }
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchEl>>,
    ) -> Self {
        self.match_ = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderEl {
        DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderEl {
            invert: core::default::Default::default(),
            match_: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElRef {
        DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `invert` after provisioning.\n"]
    pub fn invert(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert", self.base))
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix`.\n"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
        DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
            exact: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef {
        DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\n"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElPathEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElPathEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElPathEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElPathEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttp2RouteElMatchElPathEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttp2RouteElMatchElPathEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElPathEl {
        DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElPathEl {
            exact: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElPathElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElPathElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElPathElRef {
        DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElPathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {
    type O =
        BlockAssignable<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {
        DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {
            exact: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef {
        DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterEl {
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_:
        Option<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterEl {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl>>,
    ) -> Self {
        self.match_ = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterEl {
        DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterEl {
            match_: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElRef {
        DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<SetField<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElPathEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_parameter:
        Option<SetField<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterEl>>,
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl Into<SetField<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderEl>>,
    ) -> Self {
        self.header = Some(v.into());
        self
    }
    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl>>,
    ) -> Self {
        self.hostname = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElPathEl>>,
    ) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `query_parameter`.\n"]
    pub fn set_query_parameter(
        mut self,
        v: impl Into<SetField<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterEl>>,
    ) -> Self {
        self.query_parameter = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttp2RouteElMatchEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
        DataAppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
            header: core::default::Default::default(),
            hostname: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            query_parameter: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElRef {
        DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> SetRef<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHeaderElRef> {
        SetRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> ListRef<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElPathElRef> {
        ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `query_parameter` after provisioning.\n"]
    pub fn query_parameter(
        &self,
    ) -> SetRef<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElQueryParameterElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.query_parameter", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchEl>>,
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElActionEl>>,
    ) -> Self {
        self.action = Some(v.into());
        self
    }
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchEl>>,
    ) -> Self {
        self.match_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttp2RouteEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttp2RouteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttp2RouteEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttp2RouteEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttp2RouteEl {
        DataAppmeshGatewayRouteSpecElHttp2RouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttp2RouteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttp2RouteElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshGatewayRouteSpecElHttp2RouteElRef {
        DataAppmeshGatewayRouteSpecElHttp2RouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttp2RouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<DataAppmeshGatewayRouteSpecElHttp2RouteElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<DataAppmeshGatewayRouteSpecElHttp2RouteElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_target_hostname: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
    #[doc = "Set the field `default_target_hostname`.\n"]
    pub fn set_default_target_hostname(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_target_hostname = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
        DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
            default_target_hostname: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef {
        DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_target_hostname` after provisioning.\n"]
    pub fn default_target_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_target_hostname", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPathEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPathEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPathEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPathEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPathEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPathEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPathEl {
        DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPathEl {
            exact: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPathElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPathElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPathElRef {
        DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
    #[doc = "Set the field `default_prefix`.\n"]
    pub fn set_default_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
        DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
            default_prefix: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef {
        DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_prefix` after provisioning.\n"]
    pub fn default_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname:
        Option<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPathEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl>>,
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl>>,
    ) -> Self {
        self.hostname = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPathEl>>,
    ) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl>>,
    ) -> Self {
        self.prefix = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
        DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
            hostname: core::default::Default::default(),
            path: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef {
        DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPathElRef> {
        ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef> {
        ListRef::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_service_name: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
    #[doc = "Set the field `virtual_service_name`.\n"]
    pub fn set_virtual_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.virtual_service_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
    type O =
        BlockAssignable<DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
        DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
            virtual_service_name: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef {
        DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `virtual_service_name` after provisioning.\n"]
    pub fn virtual_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.virtual_service_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_service:
        Option<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl>>,
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `virtual_service`.\n"]
    pub fn set_virtual_service(
        mut self,
        v: impl Into<
            ListField<DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl>,
        >,
    ) -> Self {
        self.virtual_service = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
        DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
            port: core::default::Default::default(),
            virtual_service: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef {
        DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `virtual_service` after provisioning.\n"]
    pub fn virtual_service(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_service", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rewrite: Option<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl>>,
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElActionEl {
    #[doc = "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl>>,
    ) -> Self {
        self.rewrite = Some(v.into());
        self
    }
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl>>,
    ) -> Self {
        self.target = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttpRouteElActionEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttpRouteElActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttpRouteElActionEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttpRouteElActionEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttpRouteElActionEl {
        DataAppmeshGatewayRouteSpecElHttpRouteElActionEl {
            rewrite: core::default::Default::default(),
            target: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttpRouteElActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttpRouteElActionElRef {
        DataAppmeshGatewayRouteSpecElHttpRouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(&self) -> ListRef<DataAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<DataAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<f64>>,
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.end = Some(v.into());
        self
    }
    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.start = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
        DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
        DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl>>,
    ) -> Self {
        self.range = Some(v.into());
        self
    }
    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix`.\n"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
        DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            range: core::default::Default::default(),
            regex: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
        DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\n"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    invert: Option<PrimField<bool>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderEl {
    #[doc = "Set the field `invert`.\n"]
    pub fn set_invert(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert = Some(v.into());
        self
    }
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchEl>>,
    ) -> Self {
        self.match_ = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderEl {
        DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderEl {
            invert: core::default::Default::default(),
            match_: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElRef {
        DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `invert` after provisioning.\n"]
    pub fn invert(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert", self.base))
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix`.\n"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
        DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
            exact: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef {
        DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `suffix` after provisioning.\n"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElMatchElPathEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElMatchElPathEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttpRouteElMatchElPathEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElPathEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttpRouteElMatchElPathEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttpRouteElMatchElPathEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttpRouteElMatchElPathEl {
        DataAppmeshGatewayRouteSpecElHttpRouteElMatchElPathEl {
            exact: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElMatchElPathElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttpRouteElMatchElPathElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttpRouteElMatchElPathElRef {
        DataAppmeshGatewayRouteSpecElHttpRouteElMatchElPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElMatchElPathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {
    type O =
        BlockAssignable<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {
        DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {
            exact: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef {
        DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterEl {
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_:
        Option<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterEl {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl>>,
    ) -> Self {
        self.match_ = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterEl {
        DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterEl {
            match_: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElRef {
        DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<SetField<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElPathEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_parameter:
        Option<SetField<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterEl>>,
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElMatchEl {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl Into<SetField<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderEl>>,
    ) -> Self {
        self.header = Some(v.into());
        self
    }
    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl>>,
    ) -> Self {
        self.hostname = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElPathEl>>,
    ) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `query_parameter`.\n"]
    pub fn set_query_parameter(
        mut self,
        v: impl Into<SetField<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterEl>>,
    ) -> Self {
        self.query_parameter = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttpRouteElMatchEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttpRouteElMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttpRouteElMatchEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttpRouteElMatchEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttpRouteElMatchEl {
        DataAppmeshGatewayRouteSpecElHttpRouteElMatchEl {
            header: core::default::Default::default(),
            hostname: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            query_parameter: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttpRouteElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshGatewayRouteSpecElHttpRouteElMatchElRef {
        DataAppmeshGatewayRouteSpecElHttpRouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> SetRef<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHeaderElRef> {
        SetRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(
        &self,
    ) -> ListRef<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> ListRef<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElPathElRef> {
        ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `query_parameter` after provisioning.\n"]
    pub fn query_parameter(
        &self,
    ) -> SetRef<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElQueryParameterElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.query_parameter", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecElHttpRouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElMatchEl>>,
}
impl DataAppmeshGatewayRouteSpecElHttpRouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElActionEl>>,
    ) -> Self {
        self.action = Some(v.into());
        self
    }
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttpRouteElMatchEl>>,
    ) -> Self {
        self.match_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecElHttpRouteEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecElHttpRouteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecElHttpRouteEl {}
impl BuildDataAppmeshGatewayRouteSpecElHttpRouteEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecElHttpRouteEl {
        DataAppmeshGatewayRouteSpecElHttpRouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElHttpRouteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElHttpRouteElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshGatewayRouteSpecElHttpRouteElRef {
        DataAppmeshGatewayRouteSpecElHttpRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElHttpRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<DataAppmeshGatewayRouteSpecElHttpRouteElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<DataAppmeshGatewayRouteSpecElHttpRouteElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}
#[derive(Serialize)]
pub struct DataAppmeshGatewayRouteSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_route: Option<ListField<DataAppmeshGatewayRouteSpecElGrpcRouteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http2_route: Option<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_route: Option<ListField<DataAppmeshGatewayRouteSpecElHttpRouteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
}
impl DataAppmeshGatewayRouteSpecEl {
    #[doc = "Set the field `grpc_route`.\n"]
    pub fn set_grpc_route(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElGrpcRouteEl>>,
    ) -> Self {
        self.grpc_route = Some(v.into());
        self
    }
    #[doc = "Set the field `http2_route`.\n"]
    pub fn set_http2_route(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttp2RouteEl>>,
    ) -> Self {
        self.http2_route = Some(v.into());
        self
    }
    #[doc = "Set the field `http_route`.\n"]
    pub fn set_http_route(
        mut self,
        v: impl Into<ListField<DataAppmeshGatewayRouteSpecElHttpRouteEl>>,
    ) -> Self {
        self.http_route = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshGatewayRouteSpecEl {
    type O = BlockAssignable<DataAppmeshGatewayRouteSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshGatewayRouteSpecEl {}
impl BuildDataAppmeshGatewayRouteSpecEl {
    pub fn build(self) -> DataAppmeshGatewayRouteSpecEl {
        DataAppmeshGatewayRouteSpecEl {
            grpc_route: core::default::Default::default(),
            http2_route: core::default::Default::default(),
            http_route: core::default::Default::default(),
            priority: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshGatewayRouteSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshGatewayRouteSpecElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshGatewayRouteSpecElRef {
        DataAppmeshGatewayRouteSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshGatewayRouteSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `grpc_route` after provisioning.\n"]
    pub fn grpc_route(&self) -> ListRef<DataAppmeshGatewayRouteSpecElGrpcRouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc_route", self.base))
    }
    #[doc = "Get a reference to the value of field `http2_route` after provisioning.\n"]
    pub fn http2_route(&self) -> ListRef<DataAppmeshGatewayRouteSpecElHttp2RouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http2_route", self.base))
    }
    #[doc = "Get a reference to the value of field `http_route` after provisioning.\n"]
    pub fn http_route(&self) -> ListRef<DataAppmeshGatewayRouteSpecElHttpRouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_route", self.base))
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
}
