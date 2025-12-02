use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataAppmeshVirtualServiceData {
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
struct DataAppmeshVirtualService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAppmeshVirtualServiceData>,
}
#[derive(Clone)]
pub struct DataAppmeshVirtualService(Rc<DataAppmeshVirtualService_>);
impl DataAppmeshVirtualService {
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
    pub fn spec(&self) -> ListRef<DataAppmeshVirtualServiceSpecElRef> {
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
}
impl Referable for DataAppmeshVirtualService {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataAppmeshVirtualService {}
impl ToListMappable for DataAppmeshVirtualService {
    type O = ListRef<DataAppmeshVirtualServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataAppmeshVirtualService_ {
    fn extract_datasource_type(&self) -> String {
        "aws_appmesh_virtual_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataAppmeshVirtualService {
    pub tf_id: String,
    #[doc = ""]
    pub mesh_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildDataAppmeshVirtualService {
    pub fn build(self, stack: &mut Stack) -> DataAppmeshVirtualService {
        let out = DataAppmeshVirtualService(Rc::new(DataAppmeshVirtualService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAppmeshVirtualServiceData {
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
pub struct DataAppmeshVirtualServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshVirtualServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataAppmeshVirtualServiceRef {
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
    pub fn spec(&self) -> ListRef<DataAppmeshVirtualServiceSpecElRef> {
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
}
#[derive(Serialize)]
pub struct DataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_node_name: Option<PrimField<String>>,
}
impl DataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
    #[doc = "Set the field `virtual_node_name`.\n"]
    pub fn set_virtual_node_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.virtual_node_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
    type O = BlockAssignable<DataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {}
impl BuildDataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
    pub fn build(self) -> DataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
        DataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
            virtual_node_name: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
        DataAppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `virtual_node_name` after provisioning.\n"]
    pub fn virtual_node_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.virtual_node_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_router_name: Option<PrimField<String>>,
}
impl DataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
    #[doc = "Set the field `virtual_router_name`.\n"]
    pub fn set_virtual_router_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.virtual_router_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
    type O = BlockAssignable<DataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {}
impl BuildDataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
    pub fn build(self) -> DataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
        DataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
            virtual_router_name: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataAppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
        DataAppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `virtual_router_name` after provisioning.\n"]
    pub fn virtual_router_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.virtual_router_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAppmeshVirtualServiceSpecElProviderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_node: Option<ListField<DataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_router: Option<ListField<DataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl>>,
}
impl DataAppmeshVirtualServiceSpecElProviderEl {
    #[doc = "Set the field `virtual_node`.\n"]
    pub fn set_virtual_node(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualServiceSpecElProviderElVirtualNodeEl>>,
    ) -> Self {
        self.virtual_node = Some(v.into());
        self
    }
    #[doc = "Set the field `virtual_router`.\n"]
    pub fn set_virtual_router(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualServiceSpecElProviderElVirtualRouterEl>>,
    ) -> Self {
        self.virtual_router = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshVirtualServiceSpecElProviderEl {
    type O = BlockAssignable<DataAppmeshVirtualServiceSpecElProviderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshVirtualServiceSpecElProviderEl {}
impl BuildDataAppmeshVirtualServiceSpecElProviderEl {
    pub fn build(self) -> DataAppmeshVirtualServiceSpecElProviderEl {
        DataAppmeshVirtualServiceSpecElProviderEl {
            virtual_node: core::default::Default::default(),
            virtual_router: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshVirtualServiceSpecElProviderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshVirtualServiceSpecElProviderElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualServiceSpecElProviderElRef {
        DataAppmeshVirtualServiceSpecElProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshVirtualServiceSpecElProviderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `virtual_node` after provisioning.\n"]
    pub fn virtual_node(
        &self,
    ) -> ListRef<DataAppmeshVirtualServiceSpecElProviderElVirtualNodeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.virtual_node", self.base))
    }
    #[doc = "Get a reference to the value of field `virtual_router` after provisioning.\n"]
    pub fn virtual_router(
        &self,
    ) -> ListRef<DataAppmeshVirtualServiceSpecElProviderElVirtualRouterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_router", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataAppmeshVirtualServiceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<ListField<DataAppmeshVirtualServiceSpecElProviderEl>>,
}
impl DataAppmeshVirtualServiceSpecEl {
    #[doc = "Set the field `provider`.\n"]
    pub fn set_provider(
        mut self,
        v: impl Into<ListField<DataAppmeshVirtualServiceSpecElProviderEl>>,
    ) -> Self {
        self.provider = Some(v.into());
        self
    }
}
impl ToListMappable for DataAppmeshVirtualServiceSpecEl {
    type O = BlockAssignable<DataAppmeshVirtualServiceSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataAppmeshVirtualServiceSpecEl {}
impl BuildDataAppmeshVirtualServiceSpecEl {
    pub fn build(self) -> DataAppmeshVirtualServiceSpecEl {
        DataAppmeshVirtualServiceSpecEl {
            provider: core::default::Default::default(),
        }
    }
}
pub struct DataAppmeshVirtualServiceSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataAppmeshVirtualServiceSpecElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshVirtualServiceSpecElRef {
        DataAppmeshVirtualServiceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataAppmeshVirtualServiceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> ListRef<DataAppmeshVirtualServiceSpecElProviderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.provider", self.base))
    }
}
