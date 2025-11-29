use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2NetworkInsightsPathData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    protocol: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    source: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_at_destination: Option<Vec<Ec2NetworkInsightsPathFilterAtDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_at_source: Option<Vec<Ec2NetworkInsightsPathFilterAtSourceEl>>,
    dynamic: Ec2NetworkInsightsPathDynamic,
}

struct Ec2NetworkInsightsPath_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Ec2NetworkInsightsPathData>,
}

#[derive(Clone)]
pub struct Ec2NetworkInsightsPath(Rc<Ec2NetworkInsightsPath_>);

impl Ec2NetworkInsightsPath {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().destination = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_ip`.\n"]
    pub fn set_destination_ip(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().destination_ip = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port`.\n"]
    pub fn set_destination_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().destination_port = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `source_ip`.\n"]
    pub fn set_source_ip(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_ip = Some(v.into());
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

    #[doc = "Set the field `filter_at_destination`.\n"]
    pub fn set_filter_at_destination(
        self,
        v: impl Into<BlockAssignable<Ec2NetworkInsightsPathFilterAtDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_at_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_at_destination = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter_at_source`.\n"]
    pub fn set_filter_at_source(self, v: impl Into<BlockAssignable<Ec2NetworkInsightsPathFilterAtSourceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_at_source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_at_source = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_ip` after provisioning.\n"]
    pub fn destination_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_port` after provisioning.\n"]
    pub fn destination_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_at_destination` after provisioning.\n"]
    pub fn filter_at_destination(&self) -> ListRef<Ec2NetworkInsightsPathFilterAtDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter_at_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_at_source` after provisioning.\n"]
    pub fn filter_at_source(&self) -> ListRef<Ec2NetworkInsightsPathFilterAtSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter_at_source", self.extract_ref()))
    }
}

impl Referable for Ec2NetworkInsightsPath {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Ec2NetworkInsightsPath { }

impl ToListMappable for Ec2NetworkInsightsPath {
    type O = ListRef<Ec2NetworkInsightsPathRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Ec2NetworkInsightsPath_ {
    fn extract_resource_type(&self) -> String {
        "aws_ec2_network_insights_path".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2NetworkInsightsPath {
    pub tf_id: String,
    #[doc = ""]
    pub protocol: PrimField<String>,
    #[doc = ""]
    pub source: PrimField<String>,
}

impl BuildEc2NetworkInsightsPath {
    pub fn build(self, stack: &mut Stack) -> Ec2NetworkInsightsPath {
        let out = Ec2NetworkInsightsPath(Rc::new(Ec2NetworkInsightsPath_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Ec2NetworkInsightsPathData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                destination: core::default::Default::default(),
                destination_ip: core::default::Default::default(),
                destination_port: core::default::Default::default(),
                id: core::default::Default::default(),
                protocol: self.protocol,
                region: core::default::Default::default(),
                source: self.source,
                source_ip: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                filter_at_destination: core::default::Default::default(),
                filter_at_source: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2NetworkInsightsPathRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsPathRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Ec2NetworkInsightsPathRef {
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

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_ip` after provisioning.\n"]
    pub fn destination_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_port` after provisioning.\n"]
    pub fn destination_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_at_destination` after provisioning.\n"]
    pub fn filter_at_destination(&self) -> ListRef<Ec2NetworkInsightsPathFilterAtDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter_at_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_at_source` after provisioning.\n"]
    pub fn filter_at_source(&self) -> ListRef<Ec2NetworkInsightsPathFilterAtSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter_at_source", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_port: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl {
    #[doc = "Set the field `from_port`.\n"]
    pub fn set_from_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from_port = Some(v.into());
        self
    }

    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to_port = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl {
    type O = BlockAssignable<Ec2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl {}

impl BuildEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl {
    pub fn build(self) -> Ec2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl {
        Ec2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl {
            from_port: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeElRef {
        Ec2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_port: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl {
    #[doc = "Set the field `from_port`.\n"]
    pub fn set_from_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from_port = Some(v.into());
        self
    }

    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to_port = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl {
    type O = BlockAssignable<Ec2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl {}

impl BuildEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl {
    pub fn build(self) -> Ec2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl {
        Ec2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl {
            from_port: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsPathFilterAtDestinationElSourcePortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsPathFilterAtDestinationElSourcePortRangeElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsPathFilterAtDestinationElSourcePortRangeElRef {
        Ec2NetworkInsightsPathFilterAtDestinationElSourcePortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsPathFilterAtDestinationElSourcePortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2NetworkInsightsPathFilterAtDestinationElDynamic {
    destination_port_range: Option<DynamicBlock<Ec2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl>>,
    source_port_range: Option<DynamicBlock<Ec2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl>>,
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsPathFilterAtDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_port_range: Option<Vec<Ec2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_port_range: Option<Vec<Ec2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl>>,
    dynamic: Ec2NetworkInsightsPathFilterAtDestinationElDynamic,
}

impl Ec2NetworkInsightsPathFilterAtDestinationEl {
    #[doc = "Set the field `destination_address`.\n"]
    pub fn set_destination_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_address = Some(v.into());
        self
    }

    #[doc = "Set the field `source_address`.\n"]
    pub fn set_source_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_address = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port_range`.\n"]
    pub fn set_destination_port_range(
        mut self,
        v: impl Into<BlockAssignable<Ec2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_port_range = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_port_range = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_port_range`.\n"]
    pub fn set_source_port_range(
        mut self,
        v: impl Into<BlockAssignable<Ec2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_port_range = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_port_range = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsPathFilterAtDestinationEl {
    type O = BlockAssignable<Ec2NetworkInsightsPathFilterAtDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsPathFilterAtDestinationEl {}

impl BuildEc2NetworkInsightsPathFilterAtDestinationEl {
    pub fn build(self) -> Ec2NetworkInsightsPathFilterAtDestinationEl {
        Ec2NetworkInsightsPathFilterAtDestinationEl {
            destination_address: core::default::Default::default(),
            source_address: core::default::Default::default(),
            destination_port_range: core::default::Default::default(),
            source_port_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsPathFilterAtDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsPathFilterAtDestinationElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsPathFilterAtDestinationElRef {
        Ec2NetworkInsightsPathFilterAtDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsPathFilterAtDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_address` after provisioning.\n"]
    pub fn destination_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_address", self.base))
    }

    #[doc = "Get a reference to the value of field `source_address` after provisioning.\n"]
    pub fn source_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_address", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_port_range` after provisioning.\n"]
    pub fn destination_port_range(
        &self,
    ) -> ListRef<Ec2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination_port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `source_port_range` after provisioning.\n"]
    pub fn source_port_range(&self) -> ListRef<Ec2NetworkInsightsPathFilterAtDestinationElSourcePortRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_port_range", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_port: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl {
    #[doc = "Set the field `from_port`.\n"]
    pub fn set_from_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from_port = Some(v.into());
        self
    }

    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to_port = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl {
    type O = BlockAssignable<Ec2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl {}

impl BuildEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl {
    pub fn build(self) -> Ec2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl {
        Ec2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl {
            from_port: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsPathFilterAtSourceElDestinationPortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsPathFilterAtSourceElDestinationPortRangeElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsPathFilterAtSourceElDestinationPortRangeElRef {
        Ec2NetworkInsightsPathFilterAtSourceElDestinationPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsPathFilterAtSourceElDestinationPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_port: Option<PrimField<f64>>,
}

impl Ec2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl {
    #[doc = "Set the field `from_port`.\n"]
    pub fn set_from_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from_port = Some(v.into());
        self
    }

    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to_port = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl {
    type O = BlockAssignable<Ec2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl {}

impl BuildEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl {
    pub fn build(self) -> Ec2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl {
        Ec2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl {
            from_port: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsPathFilterAtSourceElSourcePortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsPathFilterAtSourceElSourcePortRangeElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsPathFilterAtSourceElSourcePortRangeElRef {
        Ec2NetworkInsightsPathFilterAtSourceElSourcePortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsPathFilterAtSourceElSourcePortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2NetworkInsightsPathFilterAtSourceElDynamic {
    destination_port_range: Option<DynamicBlock<Ec2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl>>,
    source_port_range: Option<DynamicBlock<Ec2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl>>,
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsPathFilterAtSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_port_range: Option<Vec<Ec2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_port_range: Option<Vec<Ec2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl>>,
    dynamic: Ec2NetworkInsightsPathFilterAtSourceElDynamic,
}

impl Ec2NetworkInsightsPathFilterAtSourceEl {
    #[doc = "Set the field `destination_address`.\n"]
    pub fn set_destination_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_address = Some(v.into());
        self
    }

    #[doc = "Set the field `source_address`.\n"]
    pub fn set_source_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_address = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port_range`.\n"]
    pub fn set_destination_port_range(
        mut self,
        v: impl Into<BlockAssignable<Ec2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_port_range = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_port_range = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_port_range`.\n"]
    pub fn set_source_port_range(
        mut self,
        v: impl Into<BlockAssignable<Ec2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_port_range = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_port_range = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Ec2NetworkInsightsPathFilterAtSourceEl {
    type O = BlockAssignable<Ec2NetworkInsightsPathFilterAtSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsPathFilterAtSourceEl {}

impl BuildEc2NetworkInsightsPathFilterAtSourceEl {
    pub fn build(self) -> Ec2NetworkInsightsPathFilterAtSourceEl {
        Ec2NetworkInsightsPathFilterAtSourceEl {
            destination_address: core::default::Default::default(),
            source_address: core::default::Default::default(),
            destination_port_range: core::default::Default::default(),
            source_port_range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsPathFilterAtSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2NetworkInsightsPathFilterAtSourceElRef {
    fn new(shared: StackShared, base: String) -> Ec2NetworkInsightsPathFilterAtSourceElRef {
        Ec2NetworkInsightsPathFilterAtSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsPathFilterAtSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_address` after provisioning.\n"]
    pub fn destination_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_address", self.base))
    }

    #[doc = "Get a reference to the value of field `source_address` after provisioning.\n"]
    pub fn source_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_address", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_port_range` after provisioning.\n"]
    pub fn destination_port_range(&self) -> ListRef<Ec2NetworkInsightsPathFilterAtSourceElDestinationPortRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination_port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `source_port_range` after provisioning.\n"]
    pub fn source_port_range(&self) -> ListRef<Ec2NetworkInsightsPathFilterAtSourceElSourcePortRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_port_range", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2NetworkInsightsPathDynamic {
    filter_at_destination: Option<DynamicBlock<Ec2NetworkInsightsPathFilterAtDestinationEl>>,
    filter_at_source: Option<DynamicBlock<Ec2NetworkInsightsPathFilterAtSourceEl>>,
}
