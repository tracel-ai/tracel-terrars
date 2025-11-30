use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataOamLinkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    link_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataOamLink_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOamLinkData>,
}

#[derive(Clone)]
pub struct DataOamLink(Rc<DataOamLink_>);

impl DataOamLink {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `label_template` after provisioning.\n"]
    pub fn label_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `link_configuration` after provisioning.\n"]
    pub fn link_configuration(&self) -> ListRef<DataOamLinkLinkConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.link_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `link_id` after provisioning.\n"]
    pub fn link_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.link_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `link_identifier` after provisioning.\n"]
    pub fn link_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.link_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.resource_types", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sink_arn` after provisioning.\n"]
    pub fn sink_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sink_arn", self.extract_ref()),
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

impl Referable for DataOamLink {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataOamLink {}

impl ToListMappable for DataOamLink {
    type O = ListRef<DataOamLinkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOamLink_ {
    fn extract_datasource_type(&self) -> String {
        "aws_oam_link".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOamLink {
    pub tf_id: String,
    #[doc = ""]
    pub link_identifier: PrimField<String>,
}

impl BuildDataOamLink {
    pub fn build(self, stack: &mut Stack) -> DataOamLink {
        let out = DataOamLink(Rc::new(DataOamLink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOamLinkData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                link_identifier: self.link_identifier,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOamLinkRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOamLinkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataOamLinkRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `label_template` after provisioning.\n"]
    pub fn label_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `link_configuration` after provisioning.\n"]
    pub fn link_configuration(&self) -> ListRef<DataOamLinkLinkConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.link_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `link_id` after provisioning.\n"]
    pub fn link_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.link_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `link_identifier` after provisioning.\n"]
    pub fn link_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.link_identifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.resource_types", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sink_arn` after provisioning.\n"]
    pub fn sink_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sink_arn", self.extract_ref()),
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
pub struct DataOamLinkLinkConfigurationElLogGroupConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
}

impl DataOamLinkLinkConfigurationElLogGroupConfigurationEl {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter = Some(v.into());
        self
    }
}

impl ToListMappable for DataOamLinkLinkConfigurationElLogGroupConfigurationEl {
    type O = BlockAssignable<DataOamLinkLinkConfigurationElLogGroupConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOamLinkLinkConfigurationElLogGroupConfigurationEl {}

impl BuildDataOamLinkLinkConfigurationElLogGroupConfigurationEl {
    pub fn build(self) -> DataOamLinkLinkConfigurationElLogGroupConfigurationEl {
        DataOamLinkLinkConfigurationElLogGroupConfigurationEl {
            filter: core::default::Default::default(),
        }
    }
}

pub struct DataOamLinkLinkConfigurationElLogGroupConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOamLinkLinkConfigurationElLogGroupConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOamLinkLinkConfigurationElLogGroupConfigurationElRef {
        DataOamLinkLinkConfigurationElLogGroupConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOamLinkLinkConfigurationElLogGroupConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOamLinkLinkConfigurationElMetricConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
}

impl DataOamLinkLinkConfigurationElMetricConfigurationEl {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter = Some(v.into());
        self
    }
}

impl ToListMappable for DataOamLinkLinkConfigurationElMetricConfigurationEl {
    type O = BlockAssignable<DataOamLinkLinkConfigurationElMetricConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOamLinkLinkConfigurationElMetricConfigurationEl {}

impl BuildDataOamLinkLinkConfigurationElMetricConfigurationEl {
    pub fn build(self) -> DataOamLinkLinkConfigurationElMetricConfigurationEl {
        DataOamLinkLinkConfigurationElMetricConfigurationEl {
            filter: core::default::Default::default(),
        }
    }
}

pub struct DataOamLinkLinkConfigurationElMetricConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOamLinkLinkConfigurationElMetricConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOamLinkLinkConfigurationElMetricConfigurationElRef {
        DataOamLinkLinkConfigurationElMetricConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOamLinkLinkConfigurationElMetricConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOamLinkLinkConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group_configuration:
        Option<ListField<DataOamLinkLinkConfigurationElLogGroupConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_configuration: Option<ListField<DataOamLinkLinkConfigurationElMetricConfigurationEl>>,
}

impl DataOamLinkLinkConfigurationEl {
    #[doc = "Set the field `log_group_configuration`.\n"]
    pub fn set_log_group_configuration(
        mut self,
        v: impl Into<ListField<DataOamLinkLinkConfigurationElLogGroupConfigurationEl>>,
    ) -> Self {
        self.log_group_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `metric_configuration`.\n"]
    pub fn set_metric_configuration(
        mut self,
        v: impl Into<ListField<DataOamLinkLinkConfigurationElMetricConfigurationEl>>,
    ) -> Self {
        self.metric_configuration = Some(v.into());
        self
    }
}

impl ToListMappable for DataOamLinkLinkConfigurationEl {
    type O = BlockAssignable<DataOamLinkLinkConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOamLinkLinkConfigurationEl {}

impl BuildDataOamLinkLinkConfigurationEl {
    pub fn build(self) -> DataOamLinkLinkConfigurationEl {
        DataOamLinkLinkConfigurationEl {
            log_group_configuration: core::default::Default::default(),
            metric_configuration: core::default::Default::default(),
        }
    }
}

pub struct DataOamLinkLinkConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOamLinkLinkConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataOamLinkLinkConfigurationElRef {
        DataOamLinkLinkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOamLinkLinkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_configuration` after provisioning.\n"]
    pub fn log_group_configuration(
        &self,
    ) -> ListRef<DataOamLinkLinkConfigurationElLogGroupConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_group_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `metric_configuration` after provisioning.\n"]
    pub fn metric_configuration(
        &self,
    ) -> ListRef<DataOamLinkLinkConfigurationElMetricConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metric_configuration", self.base),
        )
    }
}
