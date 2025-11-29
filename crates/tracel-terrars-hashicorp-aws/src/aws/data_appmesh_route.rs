use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAppmeshRouteData {
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
    virtual_router_name: PrimField<String>,
}

struct DataAppmeshRoute_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAppmeshRouteData>,
}

#[derive(Clone)]
pub struct DataAppmeshRoute(Rc<DataAppmeshRoute_>);

impl DataAppmeshRoute {
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
    pub fn spec(&self) -> ListRef<DataAppmeshRouteSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_router_name` after provisioning.\n"]
    pub fn virtual_router_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_router_name", self.extract_ref()))
    }
}

impl Referable for DataAppmeshRoute {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataAppmeshRoute { }

impl ToListMappable for DataAppmeshRoute {
    type O = ListRef<DataAppmeshRouteRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAppmeshRoute_ {
    fn extract_datasource_type(&self) -> String {
        "aws_appmesh_route".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAppmeshRoute {
    pub tf_id: String,
    #[doc = ""]
    pub mesh_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub virtual_router_name: PrimField<String>,
}

impl BuildDataAppmeshRoute {
    pub fn build(self, stack: &mut Stack) -> DataAppmeshRoute {
        let out = DataAppmeshRoute(Rc::new(DataAppmeshRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAppmeshRouteData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                mesh_name: self.mesh_name,
                mesh_owner: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                virtual_router_name: self.virtual_router_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAppmeshRouteRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataAppmeshRouteRef {
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
    pub fn spec(&self) -> ListRef<DataAppmeshRouteSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_router_name` after provisioning.\n"]
    pub fn virtual_router_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_router_name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_node: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_node`.\n"]
    pub fn set_virtual_node(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.virtual_node = Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {}

impl BuildDataAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
    pub fn build(self) -> DataAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
        DataAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl {
            port: core::default::Default::default(),
            virtual_node: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetElRef {
        DataAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetElRef {
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

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElGrpcRouteElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_target: Option<SetField<DataAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl>>,
}

impl DataAppmeshRouteSpecElGrpcRouteElActionEl {
    #[doc = "Set the field `weighted_target`.\n"]
    pub fn set_weighted_target(
        mut self,
        v: impl Into<SetField<DataAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetEl>>,
    ) -> Self {
        self.weighted_target = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElGrpcRouteElActionEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElGrpcRouteElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElGrpcRouteElActionEl {}

impl BuildDataAppmeshRouteSpecElGrpcRouteElActionEl {
    pub fn build(self) -> DataAppmeshRouteSpecElGrpcRouteElActionEl {
        DataAppmeshRouteSpecElGrpcRouteElActionEl { weighted_target: core::default::Default::default() }
    }
}

pub struct DataAppmeshRouteSpecElGrpcRouteElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElGrpcRouteElActionElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElGrpcRouteElActionElRef {
        DataAppmeshRouteSpecElGrpcRouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElGrpcRouteElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `weighted_target` after provisioning.\n"]
    pub fn weighted_target(&self) -> SetRef<DataAppmeshRouteSpecElGrpcRouteElActionElWeightedTargetElRef> {
        SetRef::new(self.shared().clone(), format!("{}.weighted_target", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
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

impl ToListMappable for DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {}

impl BuildDataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
    pub fn build(self) -> DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
        DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef {
        DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef {
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
pub struct DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<ListField<DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}

impl DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
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
        v: impl Into<ListField<DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeEl>>,
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

impl ToListMappable for DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {}

impl BuildDataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
    pub fn build(self) -> DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
        DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            range: core::default::Default::default(),
            regex: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef {
        DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef {
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
    pub fn range(&self) -> ListRef<DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRangeElRef> {
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
pub struct DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    invert: Option<PrimField<bool>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
    #[doc = "Set the field `invert`.\n"]
    pub fn set_invert(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert = Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<ListField<DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchEl>>,
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

impl ToListMappable for DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {}

impl BuildDataAppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
    pub fn build(self) -> DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
        DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataEl {
            invert: core::default::Default::default(),
            match_: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElRef {
        DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invert` after provisioning.\n"]
    pub fn invert(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElGrpcRouteElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<SetField<DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_name: Option<PrimField<String>>,
}

impl DataAppmeshRouteSpecElGrpcRouteElMatchEl {
    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(mut self, v: impl Into<SetField<DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataEl>>) -> Self {
        self.metadata = Some(v.into());
        self
    }

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
}

impl ToListMappable for DataAppmeshRouteSpecElGrpcRouteElMatchEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElGrpcRouteElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElGrpcRouteElMatchEl {}

impl BuildDataAppmeshRouteSpecElGrpcRouteElMatchEl {
    pub fn build(self) -> DataAppmeshRouteSpecElGrpcRouteElMatchEl {
        DataAppmeshRouteSpecElGrpcRouteElMatchEl {
            metadata: core::default::Default::default(),
            method_name: core::default::Default::default(),
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            service_name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElGrpcRouteElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElGrpcRouteElMatchElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElGrpcRouteElMatchElRef {
        DataAppmeshRouteSpecElGrpcRouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElGrpcRouteElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> SetRef<DataAppmeshRouteSpecElGrpcRouteElMatchElMetadataElRef> {
        SetRef::new(self.shared().clone(), format!("{}.metadata", self.base))
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
pub struct DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {}

impl BuildDataAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
    pub fn build(self) -> DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
        DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef {
        DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef {
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
pub struct DataAppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_retry_events: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_retry_events: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_retry_timeout: Option<ListField<DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_retry_events: Option<SetField<PrimField<String>>>,
}

impl DataAppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
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

    #[doc = "Set the field `max_retries`.\n"]
    pub fn set_max_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_retries = Some(v.into());
        self
    }

    #[doc = "Set the field `per_retry_timeout`.\n"]
    pub fn set_per_retry_timeout(
        mut self,
        v: impl Into<ListField<DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutEl>>,
    ) -> Self {
        self.per_retry_timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_retry_events`.\n"]
    pub fn set_tcp_retry_events(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.tcp_retry_events = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElGrpcRouteElRetryPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElGrpcRouteElRetryPolicyEl {}

impl BuildDataAppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
    pub fn build(self) -> DataAppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
        DataAppmeshRouteSpecElGrpcRouteElRetryPolicyEl {
            grpc_retry_events: core::default::Default::default(),
            http_retry_events: core::default::Default::default(),
            max_retries: core::default::Default::default(),
            per_retry_timeout: core::default::Default::default(),
            tcp_retry_events: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElRef {
        DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc_retry_events` after provisioning.\n"]
    pub fn grpc_retry_events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.grpc_retry_events", self.base))
    }

    #[doc = "Get a reference to the value of field `http_retry_events` after provisioning.\n"]
    pub fn http_retry_events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.http_retry_events", self.base))
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `per_retry_timeout` after provisioning.\n"]
    pub fn per_retry_timeout(&self) -> ListRef<DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElPerRetryTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_retry_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_retry_events` after provisioning.\n"]
    pub fn tcp_retry_events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tcp_retry_events", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {}

impl BuildDataAppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
    pub fn build(self) -> DataAppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
        DataAppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef {
        DataAppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef {
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
pub struct DataAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {}

impl BuildDataAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
    pub fn build(self) -> DataAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
        DataAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef {
        DataAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef {
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
pub struct DataAppmeshRouteSpecElGrpcRouteElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle: Option<ListField<DataAppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_request: Option<ListField<DataAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl>>,
}

impl DataAppmeshRouteSpecElGrpcRouteElTimeoutEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElGrpcRouteElTimeoutElIdleEl>>) -> Self {
        self.idle = Some(v.into());
        self
    }

    #[doc = "Set the field `per_request`.\n"]
    pub fn set_per_request(
        mut self,
        v: impl Into<ListField<DataAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestEl>>,
    ) -> Self {
        self.per_request = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElGrpcRouteElTimeoutEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElGrpcRouteElTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElGrpcRouteElTimeoutEl {}

impl BuildDataAppmeshRouteSpecElGrpcRouteElTimeoutEl {
    pub fn build(self) -> DataAppmeshRouteSpecElGrpcRouteElTimeoutEl {
        DataAppmeshRouteSpecElGrpcRouteElTimeoutEl {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElGrpcRouteElTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElGrpcRouteElTimeoutElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElGrpcRouteElTimeoutElRef {
        DataAppmeshRouteSpecElGrpcRouteElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElGrpcRouteElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> ListRef<DataAppmeshRouteSpecElGrpcRouteElTimeoutElIdleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> ListRef<DataAppmeshRouteSpecElGrpcRouteElTimeoutElPerRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElGrpcRouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<ListField<DataAppmeshRouteSpecElGrpcRouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<DataAppmeshRouteSpecElGrpcRouteElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<ListField<DataAppmeshRouteSpecElGrpcRouteElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<ListField<DataAppmeshRouteSpecElGrpcRouteElTimeoutEl>>,
}

impl DataAppmeshRouteSpecElGrpcRouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElGrpcRouteElActionEl>>) -> Self {
        self.action = Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElGrpcRouteElMatchEl>>) -> Self {
        self.match_ = Some(v.into());
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElGrpcRouteElRetryPolicyEl>>) -> Self {
        self.retry_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElGrpcRouteElTimeoutEl>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElGrpcRouteEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElGrpcRouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElGrpcRouteEl {}

impl BuildDataAppmeshRouteSpecElGrpcRouteEl {
    pub fn build(self) -> DataAppmeshRouteSpecElGrpcRouteEl {
        DataAppmeshRouteSpecElGrpcRouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElGrpcRouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElGrpcRouteElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElGrpcRouteElRef {
        DataAppmeshRouteSpecElGrpcRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElGrpcRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<DataAppmeshRouteSpecElGrpcRouteElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<DataAppmeshRouteSpecElGrpcRouteElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<DataAppmeshRouteSpecElGrpcRouteElRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<DataAppmeshRouteSpecElGrpcRouteElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_node: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_node`.\n"]
    pub fn set_virtual_node(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.virtual_node = Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {}

impl BuildDataAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
        DataAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl {
            port: core::default::Default::default(),
            virtual_node: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetElRef {
        DataAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetElRef {
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

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttp2RouteElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_target: Option<SetField<DataAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl>>,
}

impl DataAppmeshRouteSpecElHttp2RouteElActionEl {
    #[doc = "Set the field `weighted_target`.\n"]
    pub fn set_weighted_target(
        mut self,
        v: impl Into<SetField<DataAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetEl>>,
    ) -> Self {
        self.weighted_target = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttp2RouteElActionEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttp2RouteElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttp2RouteElActionEl {}

impl BuildDataAppmeshRouteSpecElHttp2RouteElActionEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttp2RouteElActionEl {
        DataAppmeshRouteSpecElHttp2RouteElActionEl { weighted_target: core::default::Default::default() }
    }
}

pub struct DataAppmeshRouteSpecElHttp2RouteElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttp2RouteElActionElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttp2RouteElActionElRef {
        DataAppmeshRouteSpecElHttp2RouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttp2RouteElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `weighted_target` after provisioning.\n"]
    pub fn weighted_target(&self) -> SetRef<DataAppmeshRouteSpecElHttp2RouteElActionElWeightedTargetElRef> {
        SetRef::new(self.shared().clone(), format!("{}.weighted_target", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
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

impl ToListMappable for DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {}

impl BuildDataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
        DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
        DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef {
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
pub struct DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<ListField<DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}

impl DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
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
        v: impl Into<ListField<DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeEl>>,
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

impl ToListMappable for DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {}

impl BuildDataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
        DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            range: core::default::Default::default(),
            regex: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
        DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef {
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
    pub fn range(&self) -> ListRef<DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRangeElRef> {
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
pub struct DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    invert: Option<PrimField<bool>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
    #[doc = "Set the field `invert`.\n"]
    pub fn set_invert(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert = Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<ListField<DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchEl>>,
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

impl ToListMappable for DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {}

impl BuildDataAppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
        DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderEl {
            invert: core::default::Default::default(),
            match_: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElRef {
        DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invert` after provisioning.\n"]
    pub fn invert(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttp2RouteElMatchElPathEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
}

impl DataAppmeshRouteSpecElHttp2RouteElMatchElPathEl {
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

impl ToListMappable for DataAppmeshRouteSpecElHttp2RouteElMatchElPathEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttp2RouteElMatchElPathEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttp2RouteElMatchElPathEl {}

impl BuildDataAppmeshRouteSpecElHttp2RouteElMatchElPathEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttp2RouteElMatchElPathEl {
        DataAppmeshRouteSpecElHttp2RouteElMatchElPathEl {
            exact: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttp2RouteElMatchElPathElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttp2RouteElMatchElPathElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttp2RouteElMatchElPathElRef {
        DataAppmeshRouteSpecElHttp2RouteElMatchElPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttp2RouteElMatchElPathElRef {
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
pub struct DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
}

impl DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {}

impl BuildDataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl {
        DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl { exact: core::default::Default::default() }
    }
}

pub struct DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef {
        DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl {
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<ListField<DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchEl>>,
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

impl ToListMappable for DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl {}

impl BuildDataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl {
        DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl {
            match_: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElRef {
        DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttp2RouteElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<SetField<DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<ListField<DataAppmeshRouteSpecElHttp2RouteElMatchElPathEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_parameter: Option<SetField<DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheme: Option<PrimField<String>>,
}

impl DataAppmeshRouteSpecElHttp2RouteElMatchEl {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(mut self, v: impl Into<SetField<DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderEl>>) -> Self {
        self.header = Some(v.into());
        self
    }

    #[doc = "Set the field `method`.\n"]
    pub fn set_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.method = Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElHttp2RouteElMatchElPathEl>>) -> Self {
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
        v: impl Into<SetField<DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterEl>>,
    ) -> Self {
        self.query_parameter = Some(v.into());
        self
    }

    #[doc = "Set the field `scheme`.\n"]
    pub fn set_scheme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scheme = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttp2RouteElMatchEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttp2RouteElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttp2RouteElMatchEl {}

impl BuildDataAppmeshRouteSpecElHttp2RouteElMatchEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttp2RouteElMatchEl {
        DataAppmeshRouteSpecElHttp2RouteElMatchEl {
            header: core::default::Default::default(),
            method: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            query_parameter: core::default::Default::default(),
            scheme: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttp2RouteElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttp2RouteElMatchElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttp2RouteElMatchElRef {
        DataAppmeshRouteSpecElHttp2RouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttp2RouteElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> SetRef<DataAppmeshRouteSpecElHttp2RouteElMatchElHeaderElRef> {
        SetRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> ListRef<DataAppmeshRouteSpecElHttp2RouteElMatchElPathElRef> {
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
    pub fn query_parameter(&self) -> SetRef<DataAppmeshRouteSpecElHttp2RouteElMatchElQueryParameterElRef> {
        SetRef::new(self.shared().clone(), format!("{}.query_parameter", self.base))
    }

    #[doc = "Get a reference to the value of field `scheme` after provisioning.\n"]
    pub fn scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {}

impl BuildDataAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
        DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef {
        DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef {
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
pub struct DataAppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_retry_events: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_retry_timeout: Option<ListField<DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_retry_events: Option<SetField<PrimField<String>>>,
}

impl DataAppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
    #[doc = "Set the field `http_retry_events`.\n"]
    pub fn set_http_retry_events(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.http_retry_events = Some(v.into());
        self
    }

    #[doc = "Set the field `max_retries`.\n"]
    pub fn set_max_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_retries = Some(v.into());
        self
    }

    #[doc = "Set the field `per_retry_timeout`.\n"]
    pub fn set_per_retry_timeout(
        mut self,
        v: impl Into<ListField<DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutEl>>,
    ) -> Self {
        self.per_retry_timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_retry_events`.\n"]
    pub fn set_tcp_retry_events(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.tcp_retry_events = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttp2RouteElRetryPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttp2RouteElRetryPolicyEl {}

impl BuildDataAppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
        DataAppmeshRouteSpecElHttp2RouteElRetryPolicyEl {
            http_retry_events: core::default::Default::default(),
            max_retries: core::default::Default::default(),
            per_retry_timeout: core::default::Default::default(),
            tcp_retry_events: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElRef {
        DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_retry_events` after provisioning.\n"]
    pub fn http_retry_events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.http_retry_events", self.base))
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `per_retry_timeout` after provisioning.\n"]
    pub fn per_retry_timeout(&self) -> ListRef<DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElPerRetryTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_retry_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_retry_events` after provisioning.\n"]
    pub fn tcp_retry_events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tcp_retry_events", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {}

impl BuildDataAppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
        DataAppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef {
        DataAppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef {
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
pub struct DataAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {}

impl BuildDataAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
        DataAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef {
        DataAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef {
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
pub struct DataAppmeshRouteSpecElHttp2RouteElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle: Option<ListField<DataAppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_request: Option<ListField<DataAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl>>,
}

impl DataAppmeshRouteSpecElHttp2RouteElTimeoutEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElHttp2RouteElTimeoutElIdleEl>>) -> Self {
        self.idle = Some(v.into());
        self
    }

    #[doc = "Set the field `per_request`.\n"]
    pub fn set_per_request(
        mut self,
        v: impl Into<ListField<DataAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestEl>>,
    ) -> Self {
        self.per_request = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttp2RouteElTimeoutEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttp2RouteElTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttp2RouteElTimeoutEl {}

impl BuildDataAppmeshRouteSpecElHttp2RouteElTimeoutEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttp2RouteElTimeoutEl {
        DataAppmeshRouteSpecElHttp2RouteElTimeoutEl {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttp2RouteElTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttp2RouteElTimeoutElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttp2RouteElTimeoutElRef {
        DataAppmeshRouteSpecElHttp2RouteElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttp2RouteElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> ListRef<DataAppmeshRouteSpecElHttp2RouteElTimeoutElIdleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> ListRef<DataAppmeshRouteSpecElHttp2RouteElTimeoutElPerRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttp2RouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<ListField<DataAppmeshRouteSpecElHttp2RouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<DataAppmeshRouteSpecElHttp2RouteElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<ListField<DataAppmeshRouteSpecElHttp2RouteElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<ListField<DataAppmeshRouteSpecElHttp2RouteElTimeoutEl>>,
}

impl DataAppmeshRouteSpecElHttp2RouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElHttp2RouteElActionEl>>) -> Self {
        self.action = Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElHttp2RouteElMatchEl>>) -> Self {
        self.match_ = Some(v.into());
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElHttp2RouteElRetryPolicyEl>>) -> Self {
        self.retry_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElHttp2RouteElTimeoutEl>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttp2RouteEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttp2RouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttp2RouteEl {}

impl BuildDataAppmeshRouteSpecElHttp2RouteEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttp2RouteEl {
        DataAppmeshRouteSpecElHttp2RouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttp2RouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttp2RouteElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttp2RouteElRef {
        DataAppmeshRouteSpecElHttp2RouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttp2RouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<DataAppmeshRouteSpecElHttp2RouteElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<DataAppmeshRouteSpecElHttp2RouteElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<DataAppmeshRouteSpecElHttp2RouteElRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<DataAppmeshRouteSpecElHttp2RouteElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_node: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_node`.\n"]
    pub fn set_virtual_node(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.virtual_node = Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {}

impl BuildDataAppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
        DataAppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl {
            port: core::default::Default::default(),
            virtual_node: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttpRouteElActionElWeightedTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttpRouteElActionElWeightedTargetElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttpRouteElActionElWeightedTargetElRef {
        DataAppmeshRouteSpecElHttpRouteElActionElWeightedTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttpRouteElActionElWeightedTargetElRef {
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

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttpRouteElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_target: Option<SetField<DataAppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl>>,
}

impl DataAppmeshRouteSpecElHttpRouteElActionEl {
    #[doc = "Set the field `weighted_target`.\n"]
    pub fn set_weighted_target(
        mut self,
        v: impl Into<SetField<DataAppmeshRouteSpecElHttpRouteElActionElWeightedTargetEl>>,
    ) -> Self {
        self.weighted_target = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttpRouteElActionEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttpRouteElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttpRouteElActionEl {}

impl BuildDataAppmeshRouteSpecElHttpRouteElActionEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttpRouteElActionEl {
        DataAppmeshRouteSpecElHttpRouteElActionEl { weighted_target: core::default::Default::default() }
    }
}

pub struct DataAppmeshRouteSpecElHttpRouteElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttpRouteElActionElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttpRouteElActionElRef {
        DataAppmeshRouteSpecElHttpRouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttpRouteElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `weighted_target` after provisioning.\n"]
    pub fn weighted_target(&self) -> SetRef<DataAppmeshRouteSpecElHttpRouteElActionElWeightedTargetElRef> {
        SetRef::new(self.shared().clone(), format!("{}.weighted_target", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
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

impl ToListMappable for DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {}

impl BuildDataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
        DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
        DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef {
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
pub struct DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<ListField<DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}

impl DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
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
        v: impl Into<ListField<DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeEl>>,
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

impl ToListMappable for DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {}

impl BuildDataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
        DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            range: core::default::Default::default(),
            regex: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
        DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef {
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
    pub fn range(&self) -> ListRef<DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRangeElRef> {
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
pub struct DataAppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    invert: Option<PrimField<bool>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataAppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
    #[doc = "Set the field `invert`.\n"]
    pub fn set_invert(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert = Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<ListField<DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchEl>>,
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

impl ToListMappable for DataAppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttpRouteElMatchElHeaderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttpRouteElMatchElHeaderEl {}

impl BuildDataAppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
        DataAppmeshRouteSpecElHttpRouteElMatchElHeaderEl {
            invert: core::default::Default::default(),
            match_: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElRef {
        DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invert` after provisioning.\n"]
    pub fn invert(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttpRouteElMatchElPathEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
}

impl DataAppmeshRouteSpecElHttpRouteElMatchElPathEl {
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

impl ToListMappable for DataAppmeshRouteSpecElHttpRouteElMatchElPathEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttpRouteElMatchElPathEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttpRouteElMatchElPathEl {}

impl BuildDataAppmeshRouteSpecElHttpRouteElMatchElPathEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttpRouteElMatchElPathEl {
        DataAppmeshRouteSpecElHttpRouteElMatchElPathEl {
            exact: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttpRouteElMatchElPathElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttpRouteElMatchElPathElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttpRouteElMatchElPathElRef {
        DataAppmeshRouteSpecElHttpRouteElMatchElPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttpRouteElMatchElPathElRef {
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
pub struct DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
}

impl DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {}

impl BuildDataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl {
        DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl { exact: core::default::Default::default() }
    }
}

pub struct DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef {
        DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl {
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<ListField<DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchEl>>,
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

impl ToListMappable for DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl {}

impl BuildDataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl {
        DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl {
            match_: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElRef {
        DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttpRouteElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<SetField<DataAppmeshRouteSpecElHttpRouteElMatchElHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<ListField<DataAppmeshRouteSpecElHttpRouteElMatchElPathEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_parameter: Option<SetField<DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheme: Option<PrimField<String>>,
}

impl DataAppmeshRouteSpecElHttpRouteElMatchEl {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(mut self, v: impl Into<SetField<DataAppmeshRouteSpecElHttpRouteElMatchElHeaderEl>>) -> Self {
        self.header = Some(v.into());
        self
    }

    #[doc = "Set the field `method`.\n"]
    pub fn set_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.method = Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElHttpRouteElMatchElPathEl>>) -> Self {
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
        v: impl Into<SetField<DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterEl>>,
    ) -> Self {
        self.query_parameter = Some(v.into());
        self
    }

    #[doc = "Set the field `scheme`.\n"]
    pub fn set_scheme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scheme = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttpRouteElMatchEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttpRouteElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttpRouteElMatchEl {}

impl BuildDataAppmeshRouteSpecElHttpRouteElMatchEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttpRouteElMatchEl {
        DataAppmeshRouteSpecElHttpRouteElMatchEl {
            header: core::default::Default::default(),
            method: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            query_parameter: core::default::Default::default(),
            scheme: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttpRouteElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttpRouteElMatchElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttpRouteElMatchElRef {
        DataAppmeshRouteSpecElHttpRouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttpRouteElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> SetRef<DataAppmeshRouteSpecElHttpRouteElMatchElHeaderElRef> {
        SetRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> ListRef<DataAppmeshRouteSpecElHttpRouteElMatchElPathElRef> {
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
    pub fn query_parameter(&self) -> SetRef<DataAppmeshRouteSpecElHttpRouteElMatchElQueryParameterElRef> {
        SetRef::new(self.shared().clone(), format!("{}.query_parameter", self.base))
    }

    #[doc = "Get a reference to the value of field `scheme` after provisioning.\n"]
    pub fn scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {}

impl BuildDataAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
        DataAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef {
        DataAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef {
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
pub struct DataAppmeshRouteSpecElHttpRouteElRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_retry_events: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_retry_timeout: Option<ListField<DataAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_retry_events: Option<SetField<PrimField<String>>>,
}

impl DataAppmeshRouteSpecElHttpRouteElRetryPolicyEl {
    #[doc = "Set the field `http_retry_events`.\n"]
    pub fn set_http_retry_events(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.http_retry_events = Some(v.into());
        self
    }

    #[doc = "Set the field `max_retries`.\n"]
    pub fn set_max_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_retries = Some(v.into());
        self
    }

    #[doc = "Set the field `per_retry_timeout`.\n"]
    pub fn set_per_retry_timeout(
        mut self,
        v: impl Into<ListField<DataAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutEl>>,
    ) -> Self {
        self.per_retry_timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_retry_events`.\n"]
    pub fn set_tcp_retry_events(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.tcp_retry_events = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttpRouteElRetryPolicyEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttpRouteElRetryPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttpRouteElRetryPolicyEl {}

impl BuildDataAppmeshRouteSpecElHttpRouteElRetryPolicyEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttpRouteElRetryPolicyEl {
        DataAppmeshRouteSpecElHttpRouteElRetryPolicyEl {
            http_retry_events: core::default::Default::default(),
            max_retries: core::default::Default::default(),
            per_retry_timeout: core::default::Default::default(),
            tcp_retry_events: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttpRouteElRetryPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttpRouteElRetryPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttpRouteElRetryPolicyElRef {
        DataAppmeshRouteSpecElHttpRouteElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttpRouteElRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_retry_events` after provisioning.\n"]
    pub fn http_retry_events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.http_retry_events", self.base))
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `per_retry_timeout` after provisioning.\n"]
    pub fn per_retry_timeout(&self) -> ListRef<DataAppmeshRouteSpecElHttpRouteElRetryPolicyElPerRetryTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_retry_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_retry_events` after provisioning.\n"]
    pub fn tcp_retry_events(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tcp_retry_events", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttpRouteElTimeoutElIdleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {}

impl BuildDataAppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
        DataAppmeshRouteSpecElHttpRouteElTimeoutElIdleEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef {
        DataAppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef {
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
pub struct DataAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {}

impl BuildDataAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
        DataAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef {
        DataAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef {
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
pub struct DataAppmeshRouteSpecElHttpRouteElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle: Option<ListField<DataAppmeshRouteSpecElHttpRouteElTimeoutElIdleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_request: Option<ListField<DataAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl>>,
}

impl DataAppmeshRouteSpecElHttpRouteElTimeoutEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElHttpRouteElTimeoutElIdleEl>>) -> Self {
        self.idle = Some(v.into());
        self
    }

    #[doc = "Set the field `per_request`.\n"]
    pub fn set_per_request(
        mut self,
        v: impl Into<ListField<DataAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestEl>>,
    ) -> Self {
        self.per_request = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttpRouteElTimeoutEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttpRouteElTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttpRouteElTimeoutEl {}

impl BuildDataAppmeshRouteSpecElHttpRouteElTimeoutEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttpRouteElTimeoutEl {
        DataAppmeshRouteSpecElHttpRouteElTimeoutEl {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttpRouteElTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttpRouteElTimeoutElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttpRouteElTimeoutElRef {
        DataAppmeshRouteSpecElHttpRouteElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttpRouteElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> ListRef<DataAppmeshRouteSpecElHttpRouteElTimeoutElIdleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> ListRef<DataAppmeshRouteSpecElHttpRouteElTimeoutElPerRequestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElHttpRouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<ListField<DataAppmeshRouteSpecElHttpRouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<DataAppmeshRouteSpecElHttpRouteElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<ListField<DataAppmeshRouteSpecElHttpRouteElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<ListField<DataAppmeshRouteSpecElHttpRouteElTimeoutEl>>,
}

impl DataAppmeshRouteSpecElHttpRouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElHttpRouteElActionEl>>) -> Self {
        self.action = Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElHttpRouteElMatchEl>>) -> Self {
        self.match_ = Some(v.into());
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElHttpRouteElRetryPolicyEl>>) -> Self {
        self.retry_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElHttpRouteElTimeoutEl>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElHttpRouteEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElHttpRouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElHttpRouteEl {}

impl BuildDataAppmeshRouteSpecElHttpRouteEl {
    pub fn build(self) -> DataAppmeshRouteSpecElHttpRouteEl {
        DataAppmeshRouteSpecElHttpRouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElHttpRouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElHttpRouteElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElHttpRouteElRef {
        DataAppmeshRouteSpecElHttpRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElHttpRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<DataAppmeshRouteSpecElHttpRouteElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<DataAppmeshRouteSpecElHttpRouteElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<DataAppmeshRouteSpecElHttpRouteElRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<DataAppmeshRouteSpecElHttpRouteElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_node: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_node`.\n"]
    pub fn set_virtual_node(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.virtual_node = Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {}

impl BuildDataAppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
    pub fn build(self) -> DataAppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
        DataAppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl {
            port: core::default::Default::default(),
            virtual_node: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElTcpRouteElActionElWeightedTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElTcpRouteElActionElWeightedTargetElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElTcpRouteElActionElWeightedTargetElRef {
        DataAppmeshRouteSpecElTcpRouteElActionElWeightedTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElTcpRouteElActionElWeightedTargetElRef {
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

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElTcpRouteElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_target: Option<SetField<DataAppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl>>,
}

impl DataAppmeshRouteSpecElTcpRouteElActionEl {
    #[doc = "Set the field `weighted_target`.\n"]
    pub fn set_weighted_target(
        mut self,
        v: impl Into<SetField<DataAppmeshRouteSpecElTcpRouteElActionElWeightedTargetEl>>,
    ) -> Self {
        self.weighted_target = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElTcpRouteElActionEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElTcpRouteElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElTcpRouteElActionEl {}

impl BuildDataAppmeshRouteSpecElTcpRouteElActionEl {
    pub fn build(self) -> DataAppmeshRouteSpecElTcpRouteElActionEl {
        DataAppmeshRouteSpecElTcpRouteElActionEl { weighted_target: core::default::Default::default() }
    }
}

pub struct DataAppmeshRouteSpecElTcpRouteElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElTcpRouteElActionElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElTcpRouteElActionElRef {
        DataAppmeshRouteSpecElTcpRouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElTcpRouteElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `weighted_target` after provisioning.\n"]
    pub fn weighted_target(&self) -> SetRef<DataAppmeshRouteSpecElTcpRouteElActionElWeightedTargetElRef> {
        SetRef::new(self.shared().clone(), format!("{}.weighted_target", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElTcpRouteElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElTcpRouteElMatchEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElTcpRouteElMatchEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElTcpRouteElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElTcpRouteElMatchEl {}

impl BuildDataAppmeshRouteSpecElTcpRouteElMatchEl {
    pub fn build(self) -> DataAppmeshRouteSpecElTcpRouteElMatchEl {
        DataAppmeshRouteSpecElTcpRouteElMatchEl { port: core::default::Default::default() }
    }
}

pub struct DataAppmeshRouteSpecElTcpRouteElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElTcpRouteElMatchElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElTcpRouteElMatchElRef {
        DataAppmeshRouteSpecElTcpRouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElTcpRouteElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataAppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElTcpRouteElTimeoutElIdleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {}

impl BuildDataAppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
    pub fn build(self) -> DataAppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
        DataAppmeshRouteSpecElTcpRouteElTimeoutElIdleEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef {
        DataAppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef {
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
pub struct DataAppmeshRouteSpecElTcpRouteElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle: Option<ListField<DataAppmeshRouteSpecElTcpRouteElTimeoutElIdleEl>>,
}

impl DataAppmeshRouteSpecElTcpRouteElTimeoutEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElTcpRouteElTimeoutElIdleEl>>) -> Self {
        self.idle = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElTcpRouteElTimeoutEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElTcpRouteElTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElTcpRouteElTimeoutEl {}

impl BuildDataAppmeshRouteSpecElTcpRouteElTimeoutEl {
    pub fn build(self) -> DataAppmeshRouteSpecElTcpRouteElTimeoutEl {
        DataAppmeshRouteSpecElTcpRouteElTimeoutEl { idle: core::default::Default::default() }
    }
}

pub struct DataAppmeshRouteSpecElTcpRouteElTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElTcpRouteElTimeoutElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElTcpRouteElTimeoutElRef {
        DataAppmeshRouteSpecElTcpRouteElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElTcpRouteElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> ListRef<DataAppmeshRouteSpecElTcpRouteElTimeoutElIdleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecElTcpRouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<ListField<DataAppmeshRouteSpecElTcpRouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<ListField<DataAppmeshRouteSpecElTcpRouteElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<ListField<DataAppmeshRouteSpecElTcpRouteElTimeoutEl>>,
}

impl DataAppmeshRouteSpecElTcpRouteEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElTcpRouteElActionEl>>) -> Self {
        self.action = Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElTcpRouteElMatchEl>>) -> Self {
        self.match_ = Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElTcpRouteElTimeoutEl>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecElTcpRouteEl {
    type O = BlockAssignable<DataAppmeshRouteSpecElTcpRouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecElTcpRouteEl {}

impl BuildDataAppmeshRouteSpecElTcpRouteEl {
    pub fn build(self) -> DataAppmeshRouteSpecElTcpRouteEl {
        DataAppmeshRouteSpecElTcpRouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElTcpRouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElTcpRouteElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElTcpRouteElRef {
        DataAppmeshRouteSpecElTcpRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElTcpRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<DataAppmeshRouteSpecElTcpRouteElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<DataAppmeshRouteSpecElTcpRouteElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<DataAppmeshRouteSpecElTcpRouteElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRouteSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_route: Option<ListField<DataAppmeshRouteSpecElGrpcRouteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http2_route: Option<ListField<DataAppmeshRouteSpecElHttp2RouteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_route: Option<ListField<DataAppmeshRouteSpecElHttpRouteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_route: Option<ListField<DataAppmeshRouteSpecElTcpRouteEl>>,
}

impl DataAppmeshRouteSpecEl {
    #[doc = "Set the field `grpc_route`.\n"]
    pub fn set_grpc_route(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElGrpcRouteEl>>) -> Self {
        self.grpc_route = Some(v.into());
        self
    }

    #[doc = "Set the field `http2_route`.\n"]
    pub fn set_http2_route(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElHttp2RouteEl>>) -> Self {
        self.http2_route = Some(v.into());
        self
    }

    #[doc = "Set the field `http_route`.\n"]
    pub fn set_http_route(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElHttpRouteEl>>) -> Self {
        self.http_route = Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_route`.\n"]
    pub fn set_tcp_route(mut self, v: impl Into<ListField<DataAppmeshRouteSpecElTcpRouteEl>>) -> Self {
        self.tcp_route = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppmeshRouteSpecEl {
    type O = BlockAssignable<DataAppmeshRouteSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRouteSpecEl {}

impl BuildDataAppmeshRouteSpecEl {
    pub fn build(self) -> DataAppmeshRouteSpecEl {
        DataAppmeshRouteSpecEl {
            grpc_route: core::default::Default::default(),
            http2_route: core::default::Default::default(),
            http_route: core::default::Default::default(),
            priority: core::default::Default::default(),
            tcp_route: core::default::Default::default(),
        }
    }
}

pub struct DataAppmeshRouteSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppmeshRouteSpecElRef {
    fn new(shared: StackShared, base: String) -> DataAppmeshRouteSpecElRef {
        DataAppmeshRouteSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRouteSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc_route` after provisioning.\n"]
    pub fn grpc_route(&self) -> ListRef<DataAppmeshRouteSpecElGrpcRouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc_route", self.base))
    }

    #[doc = "Get a reference to the value of field `http2_route` after provisioning.\n"]
    pub fn http2_route(&self) -> ListRef<DataAppmeshRouteSpecElHttp2RouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http2_route", self.base))
    }

    #[doc = "Get a reference to the value of field `http_route` after provisioning.\n"]
    pub fn http_route(&self) -> ListRef<DataAppmeshRouteSpecElHttpRouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_route", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_route` after provisioning.\n"]
    pub fn tcp_route(&self) -> ListRef<DataAppmeshRouteSpecElTcpRouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tcp_route", self.base))
    }
}
