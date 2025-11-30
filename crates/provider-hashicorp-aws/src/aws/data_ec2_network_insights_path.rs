use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataEc2NetworkInsightsPathData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_insights_path_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataEc2NetworkInsightsPathFilterEl>>,
    dynamic: DataEc2NetworkInsightsPathDynamic,
}

struct DataEc2NetworkInsightsPath_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEc2NetworkInsightsPathData>,
}

#[derive(Clone)]
pub struct DataEc2NetworkInsightsPath(Rc<DataEc2NetworkInsightsPath_>);

impl DataEc2NetworkInsightsPath {
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

    #[doc = "Set the field `network_insights_path_id`.\n"]
    pub fn set_network_insights_path_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network_insights_path_id = Some(v.into());
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl Into<BlockAssignable<DataEc2NetworkInsightsPathFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_ip` after provisioning.\n"]
    pub fn destination_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_ip", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_port` after provisioning.\n"]
    pub fn destination_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_port", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter_at_destination` after provisioning.\n"]
    pub fn filter_at_destination(
        &self,
    ) -> ListRef<DataEc2NetworkInsightsPathFilterAtDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_at_destination", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter_at_source` after provisioning.\n"]
    pub fn filter_at_source(&self) -> ListRef<DataEc2NetworkInsightsPathFilterAtSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_at_source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_insights_path_id` after provisioning.\n"]
    pub fn network_insights_path_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_insights_path_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_ip", self.extract_ref()),
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

impl Referable for DataEc2NetworkInsightsPath {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataEc2NetworkInsightsPath {}

impl ToListMappable for DataEc2NetworkInsightsPath {
    type O = ListRef<DataEc2NetworkInsightsPathRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEc2NetworkInsightsPath_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ec2_network_insights_path".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2NetworkInsightsPath {
    pub tf_id: String,
}

impl BuildDataEc2NetworkInsightsPath {
    pub fn build(self, stack: &mut Stack) -> DataEc2NetworkInsightsPath {
        let out = DataEc2NetworkInsightsPath(Rc::new(DataEc2NetworkInsightsPath_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEc2NetworkInsightsPathData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                network_insights_path_id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2NetworkInsightsPathRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2NetworkInsightsPathRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataEc2NetworkInsightsPathRef {
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

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_ip` after provisioning.\n"]
    pub fn destination_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_ip", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_port` after provisioning.\n"]
    pub fn destination_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_port", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter_at_destination` after provisioning.\n"]
    pub fn filter_at_destination(
        &self,
    ) -> ListRef<DataEc2NetworkInsightsPathFilterAtDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_at_destination", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filter_at_source` after provisioning.\n"]
    pub fn filter_at_source(&self) -> ListRef<DataEc2NetworkInsightsPathFilterAtSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_at_source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_insights_path_id` after provisioning.\n"]
    pub fn network_insights_path_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_insights_path_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_ip", self.extract_ref()),
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
pub struct DataEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_port: Option<PrimField<f64>>,
}

impl DataEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl {
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

impl ToListMappable for DataEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl {
    type O = BlockAssignable<DataEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl {}

impl BuildDataEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl {
    pub fn build(self) -> DataEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl {
        DataEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl {
            from_port: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeElRef {
        DataEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeElRef {
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
pub struct DataEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_port: Option<PrimField<f64>>,
}

impl DataEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl {
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

impl ToListMappable for DataEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl {
    type O = BlockAssignable<DataEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl {}

impl BuildDataEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl {
    pub fn build(self) -> DataEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl {
        DataEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl {
            from_port: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeElRef {
        DataEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeElRef {
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
pub struct DataEc2NetworkInsightsPathFilterAtDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_port_range:
        Option<ListField<DataEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_port_range:
        Option<ListField<DataEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl>>,
}

impl DataEc2NetworkInsightsPathFilterAtDestinationEl {
    #[doc = "Set the field `destination_address`.\n"]
    pub fn set_destination_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_address = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port_range`.\n"]
    pub fn set_destination_port_range(
        mut self,
        v: impl Into<ListField<DataEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeEl>>,
    ) -> Self {
        self.destination_port_range = Some(v.into());
        self
    }

    #[doc = "Set the field `source_address`.\n"]
    pub fn set_source_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_address = Some(v.into());
        self
    }

    #[doc = "Set the field `source_port_range`.\n"]
    pub fn set_source_port_range(
        mut self,
        v: impl Into<ListField<DataEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeEl>>,
    ) -> Self {
        self.source_port_range = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2NetworkInsightsPathFilterAtDestinationEl {
    type O = BlockAssignable<DataEc2NetworkInsightsPathFilterAtDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsPathFilterAtDestinationEl {}

impl BuildDataEc2NetworkInsightsPathFilterAtDestinationEl {
    pub fn build(self) -> DataEc2NetworkInsightsPathFilterAtDestinationEl {
        DataEc2NetworkInsightsPathFilterAtDestinationEl {
            destination_address: core::default::Default::default(),
            destination_port_range: core::default::Default::default(),
            source_address: core::default::Default::default(),
            source_port_range: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsPathFilterAtDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2NetworkInsightsPathFilterAtDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEc2NetworkInsightsPathFilterAtDestinationElRef {
        DataEc2NetworkInsightsPathFilterAtDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsPathFilterAtDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_address` after provisioning.\n"]
    pub fn destination_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_address", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `destination_port_range` after provisioning.\n"]
    pub fn destination_port_range(
        &self,
    ) -> ListRef<DataEc2NetworkInsightsPathFilterAtDestinationElDestinationPortRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_port_range", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_address` after provisioning.\n"]
    pub fn source_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_address", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_port_range` after provisioning.\n"]
    pub fn source_port_range(
        &self,
    ) -> ListRef<DataEc2NetworkInsightsPathFilterAtDestinationElSourcePortRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_port_range", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_port: Option<PrimField<f64>>,
}

impl DataEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl {
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

impl ToListMappable for DataEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl {
    type O = BlockAssignable<DataEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl {}

impl BuildDataEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl {
    pub fn build(self) -> DataEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl {
        DataEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl {
            from_port: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeElRef {
        DataEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeElRef {
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
pub struct DataEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    from_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_port: Option<PrimField<f64>>,
}

impl DataEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl {
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

impl ToListMappable for DataEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl {
    type O = BlockAssignable<DataEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl {}

impl BuildDataEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl {
    pub fn build(self) -> DataEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl {
        DataEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl {
            from_port: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeElRef {
        DataEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeElRef {
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
pub struct DataEc2NetworkInsightsPathFilterAtSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_port_range:
        Option<ListField<DataEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_port_range:
        Option<ListField<DataEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl>>,
}

impl DataEc2NetworkInsightsPathFilterAtSourceEl {
    #[doc = "Set the field `destination_address`.\n"]
    pub fn set_destination_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.destination_address = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port_range`.\n"]
    pub fn set_destination_port_range(
        mut self,
        v: impl Into<ListField<DataEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeEl>>,
    ) -> Self {
        self.destination_port_range = Some(v.into());
        self
    }

    #[doc = "Set the field `source_address`.\n"]
    pub fn set_source_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_address = Some(v.into());
        self
    }

    #[doc = "Set the field `source_port_range`.\n"]
    pub fn set_source_port_range(
        mut self,
        v: impl Into<ListField<DataEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeEl>>,
    ) -> Self {
        self.source_port_range = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2NetworkInsightsPathFilterAtSourceEl {
    type O = BlockAssignable<DataEc2NetworkInsightsPathFilterAtSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsPathFilterAtSourceEl {}

impl BuildDataEc2NetworkInsightsPathFilterAtSourceEl {
    pub fn build(self) -> DataEc2NetworkInsightsPathFilterAtSourceEl {
        DataEc2NetworkInsightsPathFilterAtSourceEl {
            destination_address: core::default::Default::default(),
            destination_port_range: core::default::Default::default(),
            source_address: core::default::Default::default(),
            source_port_range: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsPathFilterAtSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2NetworkInsightsPathFilterAtSourceElRef {
    fn new(shared: StackShared, base: String) -> DataEc2NetworkInsightsPathFilterAtSourceElRef {
        DataEc2NetworkInsightsPathFilterAtSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsPathFilterAtSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_address` after provisioning.\n"]
    pub fn destination_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_address", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `destination_port_range` after provisioning.\n"]
    pub fn destination_port_range(
        &self,
    ) -> ListRef<DataEc2NetworkInsightsPathFilterAtSourceElDestinationPortRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_port_range", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_address` after provisioning.\n"]
    pub fn source_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_address", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_port_range` after provisioning.\n"]
    pub fn source_port_range(
        &self,
    ) -> ListRef<DataEc2NetworkInsightsPathFilterAtSourceElSourcePortRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_port_range", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2NetworkInsightsPathFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataEc2NetworkInsightsPathFilterEl {}

impl ToListMappable for DataEc2NetworkInsightsPathFilterEl {
    type O = BlockAssignable<DataEc2NetworkInsightsPathFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsPathFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataEc2NetworkInsightsPathFilterEl {
    pub fn build(self) -> DataEc2NetworkInsightsPathFilterEl {
        DataEc2NetworkInsightsPathFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2NetworkInsightsPathFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2NetworkInsightsPathFilterElRef {
    fn new(shared: StackShared, base: String) -> DataEc2NetworkInsightsPathFilterElRef {
        DataEc2NetworkInsightsPathFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsPathFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEc2NetworkInsightsPathDynamic {
    filter: Option<DynamicBlock<DataEc2NetworkInsightsPathFilterEl>>,
}
