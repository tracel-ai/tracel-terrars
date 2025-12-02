use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AppmeshVirtualServiceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spec: Option<Vec<AppmeshVirtualServiceSpecEl>>,
    dynamic: AppmeshVirtualServiceDynamic,
}
struct AppmeshVirtualService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppmeshVirtualServiceData>,
}
#[derive(Clone)]
pub struct AppmeshVirtualService(Rc<AppmeshVirtualService_>);
impl AppmeshVirtualService {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl Into<BlockAssignable<AppmeshVirtualServiceSpecEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = Some(d);
            }
        }
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
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<AppmeshVirtualServiceSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
}
impl Referable for AppmeshVirtualService {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AppmeshVirtualService {}
impl ToListMappable for AppmeshVirtualService {
    type O = ListRef<AppmeshVirtualServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AppmeshVirtualService_ {
    fn extract_resource_type(&self) -> String {
        "aws_appmesh_virtual_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAppmeshVirtualService {
    pub tf_id: String,
    #[doc = ""]
    pub mesh_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildAppmeshVirtualService {
    pub fn build(self, stack: &mut Stack) -> AppmeshVirtualService {
        let out = AppmeshVirtualService(Rc::new(AppmeshVirtualService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppmeshVirtualServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                mesh_name: self.mesh_name,
                mesh_owner: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                spec: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AppmeshVirtualServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AppmeshVirtualServiceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<AppmeshVirtualServiceSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
    virtual_node_name: PrimField<String>,
}
impl AppmeshVirtualServiceSpecElProviderElVirtualNodeEl {}
impl ToListMappable for AppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
    type O = BlockAssignable<AppmeshVirtualServiceSpecElProviderElVirtualNodeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
    #[doc = ""]
    pub virtual_node_name: PrimField<String>,
}
impl BuildAppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
    pub fn build(self) -> AppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
        AppmeshVirtualServiceSpecElProviderElVirtualNodeEl {
            virtual_node_name: self.virtual_node_name,
        }
    }
}
pub struct AppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
        AppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualServiceSpecElProviderElVirtualNodeElRef {
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
pub struct AppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
    virtual_router_name: PrimField<String>,
}
impl AppmeshVirtualServiceSpecElProviderElVirtualRouterEl {}
impl ToListMappable for AppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
    type O = BlockAssignable<AppmeshVirtualServiceSpecElProviderElVirtualRouterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
    #[doc = ""]
    pub virtual_router_name: PrimField<String>,
}
impl BuildAppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
    pub fn build(self) -> AppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
        AppmeshVirtualServiceSpecElProviderElVirtualRouterEl {
            virtual_router_name: self.virtual_router_name,
        }
    }
}
pub struct AppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
        AppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualServiceSpecElProviderElVirtualRouterElRef {
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
#[derive(Serialize, Default)]
struct AppmeshVirtualServiceSpecElProviderElDynamic {
    virtual_node: Option<DynamicBlock<AppmeshVirtualServiceSpecElProviderElVirtualNodeEl>>,
    virtual_router: Option<DynamicBlock<AppmeshVirtualServiceSpecElProviderElVirtualRouterEl>>,
}
#[derive(Serialize)]
pub struct AppmeshVirtualServiceSpecElProviderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_node: Option<Vec<AppmeshVirtualServiceSpecElProviderElVirtualNodeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_router: Option<Vec<AppmeshVirtualServiceSpecElProviderElVirtualRouterEl>>,
    dynamic: AppmeshVirtualServiceSpecElProviderElDynamic,
}
impl AppmeshVirtualServiceSpecElProviderEl {
    #[doc = "Set the field `virtual_node`.\n"]
    pub fn set_virtual_node(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualServiceSpecElProviderElVirtualNodeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.virtual_node = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.virtual_node = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `virtual_router`.\n"]
    pub fn set_virtual_router(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualServiceSpecElProviderElVirtualRouterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.virtual_router = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.virtual_router = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppmeshVirtualServiceSpecElProviderEl {
    type O = BlockAssignable<AppmeshVirtualServiceSpecElProviderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualServiceSpecElProviderEl {}
impl BuildAppmeshVirtualServiceSpecElProviderEl {
    pub fn build(self) -> AppmeshVirtualServiceSpecElProviderEl {
        AppmeshVirtualServiceSpecElProviderEl {
            virtual_node: core::default::Default::default(),
            virtual_router: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualServiceSpecElProviderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualServiceSpecElProviderElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualServiceSpecElProviderElRef {
        AppmeshVirtualServiceSpecElProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualServiceSpecElProviderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `virtual_node` after provisioning.\n"]
    pub fn virtual_node(&self) -> ListRef<AppmeshVirtualServiceSpecElProviderElVirtualNodeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.virtual_node", self.base))
    }
    #[doc = "Get a reference to the value of field `virtual_router` after provisioning.\n"]
    pub fn virtual_router(
        &self,
    ) -> ListRef<AppmeshVirtualServiceSpecElProviderElVirtualRouterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_router", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualServiceSpecElDynamic {
    provider: Option<DynamicBlock<AppmeshVirtualServiceSpecElProviderEl>>,
}
#[derive(Serialize)]
pub struct AppmeshVirtualServiceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<Vec<AppmeshVirtualServiceSpecElProviderEl>>,
    dynamic: AppmeshVirtualServiceSpecElDynamic,
}
impl AppmeshVirtualServiceSpecEl {
    #[doc = "Set the field `provider`.\n"]
    pub fn set_provider(
        mut self,
        v: impl Into<BlockAssignable<AppmeshVirtualServiceSpecElProviderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.provider = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.provider = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AppmeshVirtualServiceSpecEl {
    type O = BlockAssignable<AppmeshVirtualServiceSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppmeshVirtualServiceSpecEl {}
impl BuildAppmeshVirtualServiceSpecEl {
    pub fn build(self) -> AppmeshVirtualServiceSpecEl {
        AppmeshVirtualServiceSpecEl {
            provider: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AppmeshVirtualServiceSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppmeshVirtualServiceSpecElRef {
    fn new(shared: StackShared, base: String) -> AppmeshVirtualServiceSpecElRef {
        AppmeshVirtualServiceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppmeshVirtualServiceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> ListRef<AppmeshVirtualServiceSpecElProviderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.provider", self.base))
    }
}
#[derive(Serialize, Default)]
struct AppmeshVirtualServiceDynamic {
    spec: Option<DynamicBlock<AppmeshVirtualServiceSpecEl>>,
}
