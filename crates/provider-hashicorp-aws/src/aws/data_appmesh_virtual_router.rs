use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAppmeshVirtualRouterData {
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
}

struct DataAppmeshVirtualRouter_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAppmeshVirtualRouterData>,
}

#[derive(Clone)]
pub struct DataAppmeshVirtualRouter(Rc<DataAppmeshVirtualRouter_>);

impl DataAppmeshVirtualRouter {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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
        PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mesh_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mesh_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<DataAppmeshVirtualRouterSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Referable for DataAppmeshVirtualRouter {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataAppmeshVirtualRouter { }

impl ToListMappable for DataAppmeshVirtualRouter {
    type O = ListRef<DataAppmeshVirtualRouterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAppmeshVirtualRouter_ {
    fn extract_datasource_type(&self) -> String {
        "aws_appmesh_virtual_router".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAppmeshVirtualRouter {
    pub tf_id: String,
    #[doc = ""]
    pub mesh_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataAppmeshVirtualRouter {
    pub fn build(self, stack: &mut Stack) -> DataAppmeshVirtualRouter {
        let out = DataAppmeshVirtualRouter(Rc::new(DataAppmeshVirtualRouter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAppmeshVirtualRouterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                mesh_name: self.mesh_name,
                mesh_owner: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAppmeshVirtualRouterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualRouterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataAppmeshVirtualRouterRef {
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
        PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mesh_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mesh_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<DataAppmeshVirtualRouterSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualRouterSpecElListenerElPortMappingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
}

impl DataAppmeshVirtualRouterSpecElListenerElPortMappingEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualRouterSpecElListenerElPortMappingEl {
    type O = BlockAssignable<DataAppmeshVirtualRouterSpecElListenerElPortMappingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualRouterSpecElListenerElPortMappingEl {}

impl BuildDataAppmeshVirtualRouterSpecElListenerElPortMappingEl {
    pub fn build(self) -> DataAppmeshVirtualRouterSpecElListenerElPortMappingEl {
        DataAppmeshVirtualRouterSpecElListenerElPortMappingEl {
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshVirtualRouterSpecElListenerElPortMappingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualRouterSpecElListenerElPortMappingElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualRouterSpecElListenerElPortMappingElRef {
        DataAppmeshVirtualRouterSpecElListenerElPortMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualRouterSpecElListenerElPortMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualRouterSpecElListenerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port_mapping: Option<ListField<DataAppmeshVirtualRouterSpecElListenerElPortMappingEl>>,
}

impl DataAppmeshVirtualRouterSpecElListenerEl {
    #[doc = "Set the field `port_mapping`.\n"]
    pub fn set_port_mapping(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualRouterSpecElListenerElPortMappingEl>>,
    ) -> Self {
        self.port_mapping = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualRouterSpecElListenerEl {
    type O = BlockAssignable<DataAppmeshVirtualRouterSpecElListenerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualRouterSpecElListenerEl {}

impl BuildDataAppmeshVirtualRouterSpecElListenerEl {
    pub fn build(self) -> DataAppmeshVirtualRouterSpecElListenerEl {
        DataAppmeshVirtualRouterSpecElListenerEl { port_mapping: core::default::Default::default() }
    }
}

pub struct DataAppmeshVirtualRouterSpecElListenerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualRouterSpecElListenerElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualRouterSpecElListenerElRef {
        DataAppmeshVirtualRouterSpecElListenerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualRouterSpecElListenerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port_mapping` after provisioning.\n"]
    pub fn port_mapping(&self) -> ListRef<DataAppmeshVirtualRouterSpecElListenerElPortMappingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.port_mapping", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualRouterSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    listener: Option<ListField<DataAppmeshVirtualRouterSpecElListenerEl>>,
}

impl DataAppmeshVirtualRouterSpecEl {
    #[doc = "Set the field `listener`.\n"]
    pub fn set_listener(mut self, v: impl Into<ListField<DataAppmeshVirtualRouterSpecElListenerEl>>) -> Self {
        self.listener = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshVirtualRouterSpecEl {
    type O = BlockAssignable<DataAppmeshVirtualRouterSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualRouterSpecEl {}

impl BuildDataAppmeshVirtualRouterSpecEl {
    pub fn build(self) -> DataAppmeshVirtualRouterSpecEl {
        DataAppmeshVirtualRouterSpecEl { listener: core::default::Default::default() }
    }
}

pub struct DataAppmeshVirtualRouterSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshVirtualRouterSpecElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualRouterSpecElRef {
        DataAppmeshVirtualRouterSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualRouterSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `listener` after provisioning.\n"]
    pub fn listener(&self) -> ListRef<DataAppmeshVirtualRouterSpecElListenerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.listener", self.base))
    }
}
