use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct AppmeshRouteData {
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
    virtual_router_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spec: Option<Vec<AppmeshRouteSpecEl>>,
    dynamic: AppmeshRouteDynamic,
}

struct AppmeshRoute_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppmeshRouteData>,
}

#[derive(Clone)]
pub struct AppmeshRoute(Rc<AppmeshRoute_>);

impl AppmeshRoute {
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
    pub fn set_spec(self, v: impl Into<BlockAssignable<AppmeshRouteSpecEl>>) -> Self {
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

    #[doc = "Get a reference to the value of field `virtual_router_name` after provisioning.\n"]
    pub fn virtual_router_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.virtual_router_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<AppmeshRouteSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
}

impl Referable for AppmeshRoute {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for AppmeshRoute {}

impl ToListMappable for AppmeshRoute {
    type O = ListRef<AppmeshRouteRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AppmeshRoute_ {
    fn extract_resource_type(&self) -> String {
        "aws_appmesh_route".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppmeshRoute {
    pub tf_id: String,
    #[doc = ""]
    pub mesh_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub virtual_router_name: PrimField<String>,
}

impl BuildAppmeshRoute {
    pub fn build(self, stack: &mut Stack) -> AppmeshRoute {
        let out = AppmeshRoute(Rc::new(AppmeshRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppmeshRouteData {
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
                virtual_router_name: self.virtual_router_name,
                spec: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppmeshRouteRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl AppmeshRouteRef {
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

    #[doc = "Get a reference to the value of field `virtual_router_name` after provisioning.\n"]
    pub fn virtual_router_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.virtual_router_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<AppmeshRouteSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    virtual_node: PrimField<String>,
    weight: PrimField<f64>,
}

impl AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
    type O = BlockAssignable<AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
    #[doc = ""]
    pub virtual_node: PrimField<String>,
    #[doc = ""]
    pub weight: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
        AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
            port: core::default::Default::default(),
            virtual_node: self.virtual_node,
            weight: self.weight,
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetElRef {
        AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_node` after provisioning.\n"]
    pub fn virtual_node(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_node", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElGrpcRouteElActionElDynamic {
    weighted_target: Option<DynamicBlock<AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_target: Option<Vec<AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl>>,
    dynamic: AppmeshRouteSpecElGrpcRouteElActionElDynamic,
}

impl AppmeshRouteSpecElGrpcRouteElActionEl {
    #[doc = "Set the field `weighted_target`.\n"]
    pub fn set_weighted_target(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weighted_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_target = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElGrpcRouteElActionEl {
    type O = BlockAssignable<AppmeshRouteSpecElGrpcRouteElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElActionEl {}

impl BuildAppmeshRouteSpecElGrpcRouteElActionEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElActionEl {
        AppmeshRouteSpecElGrpcRouteElActionEl {
            weighted_target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElGrpcRouteElActionElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElGrpcRouteElActionElRef {
        AppmeshRouteSpecElGrpcRouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
    end: PrimField<f64>,
    start: PrimField<f64>,
}

impl AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {}

impl ToListMappable for AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
    type O = BlockAssignable<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
    #[doc = ""]
    pub end: PrimField<f64>,
    #[doc = ""]
    pub start: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
        AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
            end: self.end,
            start: self.start,
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef {
        AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef {
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

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElDynamic {
    range: Option<DynamicBlock<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<Vec<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl>>,
    dynamic: AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElDynamic,
}

impl AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
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

    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.range = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
    type O = BlockAssignable<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {}

impl BuildAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
        AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
            suffix: core::default::Default::default(),
            range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef {
        AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef {
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

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `suffix` after provisioning.\n"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }

    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(
        &self,
    ) -> ListRef<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElGrpcRouteElMatchElMetadataElDynamic {
    match_: Option<DynamicBlock<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    invert: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl>>,
    dynamic: AppmeshRouteSpecElGrpcRouteElMatchElMetadataElDynamic,
}

impl AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
    #[doc = "Set the field `invert`.\n"]
    pub fn set_invert(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert = Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
    type O = BlockAssignable<AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
        AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
            invert: core::default::Default::default(),
            name: self.name,
            match_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElMatchElMetadataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElGrpcRouteElMatchElMetadataElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElGrpcRouteElMatchElMetadataElRef {
        AppmeshRouteSpecElGrpcRouteElMatchElMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElMatchElMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invert` after provisioning.\n"]
    pub fn invert(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<AppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElGrpcRouteElMatchElDynamic {
    metadata: Option<DynamicBlock<AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    method_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Vec<AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl>>,
    dynamic: AppmeshRouteSpecElGrpcRouteElMatchElDynamic,
}

impl AppmeshRouteSpecElGrpcRouteElMatchEl {
    #[doc = "Set the field `method_name`.\n"]
    pub fn set_method_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.method_name = Some(v.into());
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

    #[doc = "Set the field `service_name`.\n"]
    pub fn set_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_name = Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElGrpcRouteElMatchElMetadataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metadata = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElGrpcRouteElMatchEl {
    type O = BlockAssignable<AppmeshRouteSpecElGrpcRouteElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElMatchEl {}

impl BuildAppmeshRouteSpecElGrpcRouteElMatchEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElMatchEl {
        AppmeshRouteSpecElGrpcRouteElMatchEl {
            method_name: core::default::Default::default(),
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            service_name: core::default::Default::default(),
            metadata: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElGrpcRouteElMatchElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElGrpcRouteElMatchElRef {
        AppmeshRouteSpecElGrpcRouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `method_name` after provisioning.\n"]
    pub fn method_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {}

impl ToListMappable for AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
    type O = BlockAssignable<AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
        AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef {
        AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElGrpcRouteElRetryPolicyElDynamic {
    per_retry_timeout:
        Option<DynamicBlock<AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_retry_events: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_retry_events: Option<SetField<PrimField<String>>>,
    max_retries: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_retry_events: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_retry_timeout: Option<Vec<AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl>>,
    dynamic: AppmeshRouteSpecElGrpcRouteElRetryPolicyElDynamic,
}

impl AppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
    #[doc = "Set the field `grpc_retry_events`.\n"]
    pub fn set_grpc_retry_events(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.grpc_retry_events = Some(v.into());
        self
    }

    #[doc = "Set the field `http_retry_events`.\n"]
    pub fn set_http_retry_events(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.http_retry_events = Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_retry_events`.\n"]
    pub fn set_tcp_retry_events(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.tcp_retry_events = Some(v.into());
        self
    }

    #[doc = "Set the field `per_retry_timeout`.\n"]
    pub fn set_per_retry_timeout(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.per_retry_timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.per_retry_timeout = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
    type O = BlockAssignable<AppmeshRouteSpecElGrpcRouteElRetryPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
    #[doc = ""]
    pub max_retries: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
        AppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
            grpc_retry_events: core::default::Default::default(),
            http_retry_events: core::default::Default::default(),
            max_retries: self.max_retries,
            tcp_retry_events: core::default::Default::default(),
            per_retry_timeout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElRetryPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElGrpcRouteElRetryPolicyElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElGrpcRouteElRetryPolicyElRef {
        AppmeshRouteSpecElGrpcRouteElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc_retry_events` after provisioning.\n"]
    pub fn grpc_retry_events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.grpc_retry_events", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `http_retry_events` after provisioning.\n"]
    pub fn http_retry_events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.http_retry_events", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_retry_events` after provisioning.\n"]
    pub fn tcp_retry_events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.tcp_retry_events", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `per_retry_timeout` after provisioning.\n"]
    pub fn per_retry_timeout(
        &self,
    ) -> ListRef<AppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.per_retry_timeout", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {}

impl ToListMappable for AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
    type O = BlockAssignable<AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
        AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef {
        AppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {}

impl ToListMappable for AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
    type O = BlockAssignable<AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
        AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef {
        AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElGrpcRouteElTimeoutElDynamic {
    idle: Option<DynamicBlock<AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl>>,
    per_request: Option<DynamicBlock<AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle: Option<Vec<AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_request: Option<Vec<AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl>>,
    dynamic: AppmeshRouteSpecElGrpcRouteElTimeoutElDynamic,
}

impl AppmeshRouteSpecElGrpcRouteElTimeoutEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.idle = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.idle = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `per_request`.\n"]
    pub fn set_per_request(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.per_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.per_request = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElGrpcRouteElTimeoutEl {
    type O = BlockAssignable<AppmeshRouteSpecElGrpcRouteElTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteElTimeoutEl {}

impl BuildAppmeshRouteSpecElGrpcRouteElTimeoutEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteElTimeoutEl {
        AppmeshRouteSpecElGrpcRouteElTimeoutEl {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElGrpcRouteElTimeoutElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElGrpcRouteElTimeoutElRef {
        AppmeshRouteSpecElGrpcRouteElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> ListRef<AppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> ListRef<AppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElGrpcRouteElDynamic {
    action: Option<DynamicBlock<AppmeshRouteSpecElGrpcRouteElActionEl>>,
    match_: Option<DynamicBlock<AppmeshRouteSpecElGrpcRouteElMatchEl>>,
    retry_policy: Option<DynamicBlock<AppmeshRouteSpecElGrpcRouteElRetryPolicyEl>>,
    timeout: Option<DynamicBlock<AppmeshRouteSpecElGrpcRouteElTimeoutEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElGrpcRouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<AppmeshRouteSpecElGrpcRouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<AppmeshRouteSpecElGrpcRouteElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<Vec<AppmeshRouteSpecElGrpcRouteElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Vec<AppmeshRouteSpecElGrpcRouteElTimeoutEl>>,
    dynamic: AppmeshRouteSpecElGrpcRouteElDynamic,
}

impl AppmeshRouteSpecElGrpcRouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElGrpcRouteElActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElGrpcRouteElMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElGrpcRouteElRetryPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retry_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElGrpcRouteElTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElGrpcRouteEl {
    type O = BlockAssignable<AppmeshRouteSpecElGrpcRouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElGrpcRouteEl {}

impl BuildAppmeshRouteSpecElGrpcRouteEl {
    pub fn build(self) -> AppmeshRouteSpecElGrpcRouteEl {
        AppmeshRouteSpecElGrpcRouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElGrpcRouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElGrpcRouteElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElGrpcRouteElRef {
        AppmeshRouteSpecElGrpcRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElGrpcRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<AppmeshRouteSpecElGrpcRouteElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<AppmeshRouteSpecElGrpcRouteElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<AppmeshRouteSpecElGrpcRouteElRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<AppmeshRouteSpecElGrpcRouteElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    virtual_node: PrimField<String>,
    weight: PrimField<f64>,
}

impl AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
    #[doc = ""]
    pub virtual_node: PrimField<String>,
    #[doc = ""]
    pub weight: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
        AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
            port: core::default::Default::default(),
            virtual_node: self.virtual_node,
            weight: self.weight,
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetElRef {
        AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_node` after provisioning.\n"]
    pub fn virtual_node(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_node", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttp2RouteElActionElDynamic {
    weighted_target: Option<DynamicBlock<AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_target: Option<Vec<AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl>>,
    dynamic: AppmeshRouteSpecElHttp2RouteElActionElDynamic,
}

impl AppmeshRouteSpecElHttp2RouteElActionEl {
    #[doc = "Set the field `weighted_target`.\n"]
    pub fn set_weighted_target(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weighted_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_target = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttp2RouteElActionEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttp2RouteElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElActionEl {}

impl BuildAppmeshRouteSpecElHttp2RouteElActionEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElActionEl {
        AppmeshRouteSpecElHttp2RouteElActionEl {
            weighted_target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttp2RouteElActionElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElHttp2RouteElActionElRef {
        AppmeshRouteSpecElHttp2RouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
    end: PrimField<f64>,
    start: PrimField<f64>,
}

impl AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {}

impl ToListMappable for AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
    #[doc = ""]
    pub end: PrimField<f64>,
    #[doc = ""]
    pub start: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
        AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
            end: self.end,
            start: self.start,
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
        AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
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

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElDynamic {
    range: Option<DynamicBlock<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<Vec<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl>>,
    dynamic: AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElDynamic,
}

impl AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
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

    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.range = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {}

impl BuildAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
        AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
            suffix: core::default::Default::default(),
            range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
        AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
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

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `suffix` after provisioning.\n"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }

    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> ListRef<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttp2RouteElMatchElHeaderElDynamic {
    match_: Option<DynamicBlock<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    invert: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl>>,
    dynamic: AppmeshRouteSpecElHttp2RouteElMatchElHeaderElDynamic,
}

impl AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
    #[doc = "Set the field `invert`.\n"]
    pub fn set_invert(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert = Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
        AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
            invert: core::default::Default::default(),
            name: self.name,
            match_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElMatchElHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttp2RouteElMatchElHeaderElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElHttp2RouteElMatchElHeaderElRef {
        AppmeshRouteSpecElHttp2RouteElMatchElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElMatchElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invert` after provisioning.\n"]
    pub fn invert(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<AppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElMatchElPathEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
}

impl AppmeshRouteSpecElHttp2RouteElMatchElPathEl {
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

impl ToListMappable for AppmeshRouteSpecElHttp2RouteElMatchElPathEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElPathEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElMatchElPathEl {}

impl BuildAppmeshRouteSpecElHttp2RouteElMatchElPathEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElMatchElPathEl {
        AppmeshRouteSpecElHttp2RouteElMatchElPathEl {
            exact: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElMatchElPathElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttp2RouteElMatchElPathElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElHttp2RouteElMatchElPathElRef {
        AppmeshRouteSpecElHttp2RouteElMatchElPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElMatchElPathElRef {
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
pub struct AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
}

impl AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {}

impl BuildAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {
        AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {
            exact: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef {
        AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElDynamic {
    match_: Option<DynamicBlock<AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl {
    name: PrimField<String>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl>>,
    dynamic: AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElDynamic,
}

impl AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl {
        AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl {
            name: self.name,
            match_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElRef {
        AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> ListRef<AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttp2RouteElMatchElDynamic {
    header: Option<DynamicBlock<AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl>>,
    path: Option<DynamicBlock<AppmeshRouteSpecElHttp2RouteElMatchElPathEl>>,
    query_parameter: Option<DynamicBlock<AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheme: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<Vec<AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<Vec<AppmeshRouteSpecElHttp2RouteElMatchElPathEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_parameter: Option<Vec<AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl>>,
    dynamic: AppmeshRouteSpecElHttp2RouteElMatchElDynamic,
}

impl AppmeshRouteSpecElHttp2RouteElMatchEl {
    #[doc = "Set the field `method`.\n"]
    pub fn set_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.method = Some(v.into());
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

    #[doc = "Set the field `scheme`.\n"]
    pub fn set_scheme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scheme = Some(v.into());
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElHeaderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElPathEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.path = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.path = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `query_parameter`.\n"]
    pub fn set_query_parameter(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_parameter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_parameter = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttp2RouteElMatchEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElMatchEl {}

impl BuildAppmeshRouteSpecElHttp2RouteElMatchEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElMatchEl {
        AppmeshRouteSpecElHttp2RouteElMatchEl {
            method: core::default::Default::default(),
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            scheme: core::default::Default::default(),
            header: core::default::Default::default(),
            path: core::default::Default::default(),
            query_parameter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttp2RouteElMatchElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElHttp2RouteElMatchElRef {
        AppmeshRouteSpecElHttp2RouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `scheme` after provisioning.\n"]
    pub fn scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> ListRef<AppmeshRouteSpecElHttp2RouteElMatchElPathElRef> {
        ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {}

impl ToListMappable for AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
        AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef {
        AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttp2RouteElRetryPolicyElDynamic {
    per_retry_timeout:
        Option<DynamicBlock<AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_retry_events: Option<SetField<PrimField<String>>>,
    max_retries: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_retry_events: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_retry_timeout: Option<Vec<AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl>>,
    dynamic: AppmeshRouteSpecElHttp2RouteElRetryPolicyElDynamic,
}

impl AppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
    #[doc = "Set the field `http_retry_events`.\n"]
    pub fn set_http_retry_events(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.http_retry_events = Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_retry_events`.\n"]
    pub fn set_tcp_retry_events(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.tcp_retry_events = Some(v.into());
        self
    }

    #[doc = "Set the field `per_retry_timeout`.\n"]
    pub fn set_per_retry_timeout(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.per_retry_timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.per_retry_timeout = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttp2RouteElRetryPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
    #[doc = ""]
    pub max_retries: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
        AppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
            http_retry_events: core::default::Default::default(),
            max_retries: self.max_retries,
            tcp_retry_events: core::default::Default::default(),
            per_retry_timeout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElRetryPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttp2RouteElRetryPolicyElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElHttp2RouteElRetryPolicyElRef {
        AppmeshRouteSpecElHttp2RouteElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_retry_events` after provisioning.\n"]
    pub fn http_retry_events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.http_retry_events", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_retry_events` after provisioning.\n"]
    pub fn tcp_retry_events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.tcp_retry_events", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `per_retry_timeout` after provisioning.\n"]
    pub fn per_retry_timeout(
        &self,
    ) -> ListRef<AppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.per_retry_timeout", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {}

impl ToListMappable for AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
        AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef {
        AppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {}

impl ToListMappable for AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
        AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef {
        AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttp2RouteElTimeoutElDynamic {
    idle: Option<DynamicBlock<AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl>>,
    per_request: Option<DynamicBlock<AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle: Option<Vec<AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_request: Option<Vec<AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl>>,
    dynamic: AppmeshRouteSpecElHttp2RouteElTimeoutElDynamic,
}

impl AppmeshRouteSpecElHttp2RouteElTimeoutEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.idle = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.idle = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `per_request`.\n"]
    pub fn set_per_request(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.per_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.per_request = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttp2RouteElTimeoutEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttp2RouteElTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteElTimeoutEl {}

impl BuildAppmeshRouteSpecElHttp2RouteElTimeoutEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteElTimeoutEl {
        AppmeshRouteSpecElHttp2RouteElTimeoutEl {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttp2RouteElTimeoutElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElHttp2RouteElTimeoutElRef {
        AppmeshRouteSpecElHttp2RouteElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> ListRef<AppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> ListRef<AppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttp2RouteElDynamic {
    action: Option<DynamicBlock<AppmeshRouteSpecElHttp2RouteElActionEl>>,
    match_: Option<DynamicBlock<AppmeshRouteSpecElHttp2RouteElMatchEl>>,
    retry_policy: Option<DynamicBlock<AppmeshRouteSpecElHttp2RouteElRetryPolicyEl>>,
    timeout: Option<DynamicBlock<AppmeshRouteSpecElHttp2RouteElTimeoutEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttp2RouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<AppmeshRouteSpecElHttp2RouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<AppmeshRouteSpecElHttp2RouteElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<Vec<AppmeshRouteSpecElHttp2RouteElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Vec<AppmeshRouteSpecElHttp2RouteElTimeoutEl>>,
    dynamic: AppmeshRouteSpecElHttp2RouteElDynamic,
}

impl AppmeshRouteSpecElHttp2RouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttp2RouteElActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttp2RouteElMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttp2RouteElRetryPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retry_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttp2RouteElTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttp2RouteEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttp2RouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttp2RouteEl {}

impl BuildAppmeshRouteSpecElHttp2RouteEl {
    pub fn build(self) -> AppmeshRouteSpecElHttp2RouteEl {
        AppmeshRouteSpecElHttp2RouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttp2RouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttp2RouteElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElHttp2RouteElRef {
        AppmeshRouteSpecElHttp2RouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttp2RouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<AppmeshRouteSpecElHttp2RouteElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<AppmeshRouteSpecElHttp2RouteElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<AppmeshRouteSpecElHttp2RouteElRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<AppmeshRouteSpecElHttp2RouteElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    virtual_node: PrimField<String>,
    weight: PrimField<f64>,
}

impl AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
    #[doc = ""]
    pub virtual_node: PrimField<String>,
    #[doc = ""]
    pub weight: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
        AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
            port: core::default::Default::default(),
            virtual_node: self.virtual_node,
            weight: self.weight,
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElActionElWeightedTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttpRouteElActionElWeightedTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElHttpRouteElActionElWeightedTargetElRef {
        AppmeshRouteSpecElHttpRouteElActionElWeightedTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElActionElWeightedTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_node` after provisioning.\n"]
    pub fn virtual_node(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_node", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttpRouteElActionElDynamic {
    weighted_target: Option<DynamicBlock<AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_target: Option<Vec<AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl>>,
    dynamic: AppmeshRouteSpecElHttpRouteElActionElDynamic,
}

impl AppmeshRouteSpecElHttpRouteElActionEl {
    #[doc = "Set the field `weighted_target`.\n"]
    pub fn set_weighted_target(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weighted_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_target = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttpRouteElActionEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttpRouteElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElActionEl {}

impl BuildAppmeshRouteSpecElHttpRouteElActionEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElActionEl {
        AppmeshRouteSpecElHttpRouteElActionEl {
            weighted_target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttpRouteElActionElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElHttpRouteElActionElRef {
        AppmeshRouteSpecElHttpRouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
    end: PrimField<f64>,
    start: PrimField<f64>,
}

impl AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {}

impl ToListMappable for AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
    #[doc = ""]
    pub end: PrimField<f64>,
    #[doc = ""]
    pub start: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
        AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
            end: self.end,
            start: self.start,
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
        AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
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

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElDynamic {
    range: Option<DynamicBlock<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<Vec<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl>>,
    dynamic: AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElDynamic,
}

impl AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
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

    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.range = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {}

impl BuildAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
        AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
            suffix: core::default::Default::default(),
            range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
        AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
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

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `suffix` after provisioning.\n"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }

    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> ListRef<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttpRouteElMatchElHeaderElDynamic {
    match_: Option<DynamicBlock<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    invert: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl>>,
    dynamic: AppmeshRouteSpecElHttpRouteElMatchElHeaderElDynamic,
}

impl AppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
    #[doc = "Set the field `invert`.\n"]
    pub fn set_invert(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert = Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
        AppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
            invert: core::default::Default::default(),
            name: self.name,
            match_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElMatchElHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttpRouteElMatchElHeaderElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElHttpRouteElMatchElHeaderElRef {
        AppmeshRouteSpecElHttpRouteElMatchElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElMatchElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invert` after provisioning.\n"]
    pub fn invert(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<AppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElMatchElPathEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
}

impl AppmeshRouteSpecElHttpRouteElMatchElPathEl {
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

impl ToListMappable for AppmeshRouteSpecElHttpRouteElMatchElPathEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElPathEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElMatchElPathEl {}

impl BuildAppmeshRouteSpecElHttpRouteElMatchElPathEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElMatchElPathEl {
        AppmeshRouteSpecElHttpRouteElMatchElPathEl {
            exact: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElMatchElPathElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttpRouteElMatchElPathElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElHttpRouteElMatchElPathElRef {
        AppmeshRouteSpecElHttpRouteElMatchElPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElMatchElPathElRef {
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
pub struct AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
}

impl AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {}

impl BuildAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {
        AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {
            exact: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef {
        AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElDynamic {
    match_: Option<DynamicBlock<AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl {
    name: PrimField<String>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl>>,
    dynamic: AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElDynamic,
}

impl AppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildAppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl {
        AppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl {
            name: self.name,
            match_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElRef {
        AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> ListRef<AppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttpRouteElMatchElDynamic {
    header: Option<DynamicBlock<AppmeshRouteSpecElHttpRouteElMatchElHeaderEl>>,
    path: Option<DynamicBlock<AppmeshRouteSpecElHttpRouteElMatchElPathEl>>,
    query_parameter: Option<DynamicBlock<AppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheme: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<Vec<AppmeshRouteSpecElHttpRouteElMatchElHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<Vec<AppmeshRouteSpecElHttpRouteElMatchElPathEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_parameter: Option<Vec<AppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl>>,
    dynamic: AppmeshRouteSpecElHttpRouteElMatchElDynamic,
}

impl AppmeshRouteSpecElHttpRouteElMatchEl {
    #[doc = "Set the field `method`.\n"]
    pub fn set_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.method = Some(v.into());
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

    #[doc = "Set the field `scheme`.\n"]
    pub fn set_scheme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scheme = Some(v.into());
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElHeaderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElPathEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.path = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.path = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `query_parameter`.\n"]
    pub fn set_query_parameter(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_parameter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_parameter = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttpRouteElMatchEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElMatchEl {}

impl BuildAppmeshRouteSpecElHttpRouteElMatchEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElMatchEl {
        AppmeshRouteSpecElHttpRouteElMatchEl {
            method: core::default::Default::default(),
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            scheme: core::default::Default::default(),
            header: core::default::Default::default(),
            path: core::default::Default::default(),
            query_parameter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttpRouteElMatchElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElHttpRouteElMatchElRef {
        AppmeshRouteSpecElHttpRouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `scheme` after provisioning.\n"]
    pub fn scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> ListRef<AppmeshRouteSpecElHttpRouteElMatchElPathElRef> {
        ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {}

impl ToListMappable for AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
        AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef {
        AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttpRouteElRetryPolicyElDynamic {
    per_retry_timeout:
        Option<DynamicBlock<AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_retry_events: Option<SetField<PrimField<String>>>,
    max_retries: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_retry_events: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_retry_timeout: Option<Vec<AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl>>,
    dynamic: AppmeshRouteSpecElHttpRouteElRetryPolicyElDynamic,
}

impl AppmeshRouteSpecElHttpRouteElRetryPolicyEl {
    #[doc = "Set the field `http_retry_events`.\n"]
    pub fn set_http_retry_events(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.http_retry_events = Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_retry_events`.\n"]
    pub fn set_tcp_retry_events(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.tcp_retry_events = Some(v.into());
        self
    }

    #[doc = "Set the field `per_retry_timeout`.\n"]
    pub fn set_per_retry_timeout(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.per_retry_timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.per_retry_timeout = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttpRouteElRetryPolicyEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttpRouteElRetryPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElRetryPolicyEl {
    #[doc = ""]
    pub max_retries: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttpRouteElRetryPolicyEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElRetryPolicyEl {
        AppmeshRouteSpecElHttpRouteElRetryPolicyEl {
            http_retry_events: core::default::Default::default(),
            max_retries: self.max_retries,
            tcp_retry_events: core::default::Default::default(),
            per_retry_timeout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElRetryPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttpRouteElRetryPolicyElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElHttpRouteElRetryPolicyElRef {
        AppmeshRouteSpecElHttpRouteElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_retry_events` after provisioning.\n"]
    pub fn http_retry_events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.http_retry_events", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_retry_events` after provisioning.\n"]
    pub fn tcp_retry_events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.tcp_retry_events", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `per_retry_timeout` after provisioning.\n"]
    pub fn per_retry_timeout(
        &self,
    ) -> ListRef<AppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.per_retry_timeout", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {}

impl ToListMappable for AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
        AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef {
        AppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {}

impl ToListMappable for AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
        AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef {
        AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttpRouteElTimeoutElDynamic {
    idle: Option<DynamicBlock<AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl>>,
    per_request: Option<DynamicBlock<AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle: Option<Vec<AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_request: Option<Vec<AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl>>,
    dynamic: AppmeshRouteSpecElHttpRouteElTimeoutElDynamic,
}

impl AppmeshRouteSpecElHttpRouteElTimeoutEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttpRouteElTimeoutElIdleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.idle = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.idle = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `per_request`.\n"]
    pub fn set_per_request(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.per_request = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.per_request = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttpRouteElTimeoutEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttpRouteElTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteElTimeoutEl {}

impl BuildAppmeshRouteSpecElHttpRouteElTimeoutEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteElTimeoutEl {
        AppmeshRouteSpecElHttpRouteElTimeoutEl {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttpRouteElTimeoutElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElHttpRouteElTimeoutElRef {
        AppmeshRouteSpecElHttpRouteElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> ListRef<AppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> ListRef<AppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElHttpRouteElDynamic {
    action: Option<DynamicBlock<AppmeshRouteSpecElHttpRouteElActionEl>>,
    match_: Option<DynamicBlock<AppmeshRouteSpecElHttpRouteElMatchEl>>,
    retry_policy: Option<DynamicBlock<AppmeshRouteSpecElHttpRouteElRetryPolicyEl>>,
    timeout: Option<DynamicBlock<AppmeshRouteSpecElHttpRouteElTimeoutEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElHttpRouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<AppmeshRouteSpecElHttpRouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<AppmeshRouteSpecElHttpRouteElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<Vec<AppmeshRouteSpecElHttpRouteElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Vec<AppmeshRouteSpecElHttpRouteElTimeoutEl>>,
    dynamic: AppmeshRouteSpecElHttpRouteElDynamic,
}

impl AppmeshRouteSpecElHttpRouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttpRouteElActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttpRouteElMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttpRouteElRetryPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retry_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttpRouteElTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElHttpRouteEl {
    type O = BlockAssignable<AppmeshRouteSpecElHttpRouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElHttpRouteEl {}

impl BuildAppmeshRouteSpecElHttpRouteEl {
    pub fn build(self) -> AppmeshRouteSpecElHttpRouteEl {
        AppmeshRouteSpecElHttpRouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElHttpRouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElHttpRouteElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElHttpRouteElRef {
        AppmeshRouteSpecElHttpRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElHttpRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<AppmeshRouteSpecElHttpRouteElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<AppmeshRouteSpecElHttpRouteElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<AppmeshRouteSpecElHttpRouteElRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<AppmeshRouteSpecElHttpRouteElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    virtual_node: PrimField<String>,
    weight: PrimField<f64>,
}

impl AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
    type O = BlockAssignable<AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
    #[doc = ""]
    pub virtual_node: PrimField<String>,
    #[doc = ""]
    pub weight: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
    pub fn build(self) -> AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
        AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
            port: core::default::Default::default(),
            virtual_node: self.virtual_node,
            weight: self.weight,
        }
    }
}

pub struct AppmeshRouteSpecElTcpRouteElActionElWeightedTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElTcpRouteElActionElWeightedTargetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshRouteSpecElTcpRouteElActionElWeightedTargetElRef {
        AppmeshRouteSpecElTcpRouteElActionElWeightedTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElTcpRouteElActionElWeightedTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_node` after provisioning.\n"]
    pub fn virtual_node(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_node", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElTcpRouteElActionElDynamic {
    weighted_target: Option<DynamicBlock<AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElTcpRouteElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_target: Option<Vec<AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl>>,
    dynamic: AppmeshRouteSpecElTcpRouteElActionElDynamic,
}

impl AppmeshRouteSpecElTcpRouteElActionEl {
    #[doc = "Set the field `weighted_target`.\n"]
    pub fn set_weighted_target(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weighted_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_target = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElTcpRouteElActionEl {
    type O = BlockAssignable<AppmeshRouteSpecElTcpRouteElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElTcpRouteElActionEl {}

impl BuildAppmeshRouteSpecElTcpRouteElActionEl {
    pub fn build(self) -> AppmeshRouteSpecElTcpRouteElActionEl {
        AppmeshRouteSpecElTcpRouteElActionEl {
            weighted_target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElTcpRouteElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElTcpRouteElActionElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElTcpRouteElActionElRef {
        AppmeshRouteSpecElTcpRouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElTcpRouteElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElTcpRouteElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}

impl AppmeshRouteSpecElTcpRouteElMatchEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElTcpRouteElMatchEl {
    type O = BlockAssignable<AppmeshRouteSpecElTcpRouteElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElTcpRouteElMatchEl {}

impl BuildAppmeshRouteSpecElTcpRouteElMatchEl {
    pub fn build(self) -> AppmeshRouteSpecElTcpRouteElMatchEl {
        AppmeshRouteSpecElTcpRouteElMatchEl {
            port: core::default::Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElTcpRouteElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElTcpRouteElMatchElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElTcpRouteElMatchElRef {
        AppmeshRouteSpecElTcpRouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElTcpRouteElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}

impl AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {}

impl ToListMappable for AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
    type O = BlockAssignable<AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}

impl BuildAppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
    pub fn build(self) -> AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
        AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef {
        AppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElTcpRouteElTimeoutElDynamic {
    idle: Option<DynamicBlock<AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElTcpRouteElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle: Option<Vec<AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl>>,
    dynamic: AppmeshRouteSpecElTcpRouteElTimeoutElDynamic,
}

impl AppmeshRouteSpecElTcpRouteElTimeoutEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElTcpRouteElTimeoutElIdleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.idle = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.idle = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElTcpRouteElTimeoutEl {
    type O = BlockAssignable<AppmeshRouteSpecElTcpRouteElTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElTcpRouteElTimeoutEl {}

impl BuildAppmeshRouteSpecElTcpRouteElTimeoutEl {
    pub fn build(self) -> AppmeshRouteSpecElTcpRouteElTimeoutEl {
        AppmeshRouteSpecElTcpRouteElTimeoutEl {
            idle: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElTcpRouteElTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElTcpRouteElTimeoutElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElTcpRouteElTimeoutElRef {
        AppmeshRouteSpecElTcpRouteElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElTcpRouteElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> ListRef<AppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElTcpRouteElDynamic {
    action: Option<DynamicBlock<AppmeshRouteSpecElTcpRouteElActionEl>>,
    match_: Option<DynamicBlock<AppmeshRouteSpecElTcpRouteElMatchEl>>,
    timeout: Option<DynamicBlock<AppmeshRouteSpecElTcpRouteElTimeoutEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecElTcpRouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<AppmeshRouteSpecElTcpRouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<AppmeshRouteSpecElTcpRouteElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Vec<AppmeshRouteSpecElTcpRouteElTimeoutEl>>,
    dynamic: AppmeshRouteSpecElTcpRouteElDynamic,
}

impl AppmeshRouteSpecElTcpRouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElTcpRouteElActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElTcpRouteElMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElTcpRouteElTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecElTcpRouteEl {
    type O = BlockAssignable<AppmeshRouteSpecElTcpRouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecElTcpRouteEl {}

impl BuildAppmeshRouteSpecElTcpRouteEl {
    pub fn build(self) -> AppmeshRouteSpecElTcpRouteEl {
        AppmeshRouteSpecElTcpRouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            timeout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElTcpRouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElTcpRouteElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElTcpRouteElRef {
        AppmeshRouteSpecElTcpRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElTcpRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<AppmeshRouteSpecElTcpRouteElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<AppmeshRouteSpecElTcpRouteElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<AppmeshRouteSpecElTcpRouteElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteSpecElDynamic {
    grpc_route: Option<DynamicBlock<AppmeshRouteSpecElGrpcRouteEl>>,
    http2_route: Option<DynamicBlock<AppmeshRouteSpecElHttp2RouteEl>>,
    http_route: Option<DynamicBlock<AppmeshRouteSpecElHttpRouteEl>>,
    tcp_route: Option<DynamicBlock<AppmeshRouteSpecElTcpRouteEl>>,
}

#[derive(Serialize)]
pub struct AppmeshRouteSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_route: Option<Vec<AppmeshRouteSpecElGrpcRouteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http2_route: Option<Vec<AppmeshRouteSpecElHttp2RouteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_route: Option<Vec<AppmeshRouteSpecElHttpRouteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_route: Option<Vec<AppmeshRouteSpecElTcpRouteEl>>,
    dynamic: AppmeshRouteSpecElDynamic,
}

impl AppmeshRouteSpecEl {
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }

    #[doc = "Set the field `grpc_route`.\n"]
    pub fn set_grpc_route(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElGrpcRouteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc_route = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc_route = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `http2_route`.\n"]
    pub fn set_http2_route(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttp2RouteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http2_route = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http2_route = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `http_route`.\n"]
    pub fn set_http_route(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElHttpRouteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_route = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_route = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `tcp_route`.\n"]
    pub fn set_tcp_route(
        mut self,
        v: impl Into<BlockAssignable<AppmeshRouteSpecElTcpRouteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tcp_route = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp_route = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for AppmeshRouteSpecEl {
    type O = BlockAssignable<AppmeshRouteSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshRouteSpecEl {}

impl BuildAppmeshRouteSpecEl {
    pub fn build(self) -> AppmeshRouteSpecEl {
        AppmeshRouteSpecEl {
            priority: core::default::Default::default(),
            grpc_route: core::default::Default::default(),
            http2_route: core::default::Default::default(),
            http_route: core::default::Default::default(),
            tcp_route: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshRouteSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshRouteSpecElRef {
    fn new(shared: StackShared, base: String) -> AppmeshRouteSpecElRef {
        AppmeshRouteSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshRouteSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc_route` after provisioning.\n"]
    pub fn grpc_route(&self) -> ListRef<AppmeshRouteSpecElGrpcRouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc_route", self.base))
    }

    #[doc = "Get a reference to the value of field `http2_route` after provisioning.\n"]
    pub fn http2_route(&self) -> ListRef<AppmeshRouteSpecElHttp2RouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http2_route", self.base))
    }

    #[doc = "Get a reference to the value of field `http_route` after provisioning.\n"]
    pub fn http_route(&self) -> ListRef<AppmeshRouteSpecElHttpRouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_route", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_route` after provisioning.\n"]
    pub fn tcp_route(&self) -> ListRef<AppmeshRouteSpecElTcpRouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tcp_route", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshRouteDynamic {
    spec: Option<DynamicBlock<AppmeshRouteSpecEl>>,
}
