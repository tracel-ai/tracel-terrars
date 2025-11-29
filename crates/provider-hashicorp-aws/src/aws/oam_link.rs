use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OamLinkData {
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
    label_template: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    resource_types: SetField<PrimField<String>>,
    sink_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_configuration: Option<Vec<OamLinkLinkConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OamLinkTimeoutsEl>,
    dynamic: OamLinkDynamic,
}

struct OamLink_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OamLinkData>,
}

#[derive(Clone)]
pub struct OamLink(Rc<OamLink_>);

impl OamLink {
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

    #[doc = "Set the field `link_configuration`.\n"]
    pub fn set_link_configuration(self, v: impl Into<BlockAssignable<OamLinkLinkConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().link_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.link_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<OamLinkTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label_template` after provisioning.\n"]
    pub fn label_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `link_id` after provisioning.\n"]
    pub fn link_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.link_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resource_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sink_arn` after provisioning.\n"]
    pub fn sink_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sink_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sink_identifier` after provisioning.\n"]
    pub fn sink_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sink_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `link_configuration` after provisioning.\n"]
    pub fn link_configuration(&self) -> ListRef<OamLinkLinkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.link_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OamLinkTimeoutsElRef {
        OamLinkTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for OamLink {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for OamLink { }

impl ToListMappable for OamLink {
    type O = ListRef<OamLinkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for OamLink_ {
    fn extract_resource_type(&self) -> String {
        "aws_oam_link".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOamLink {
    pub tf_id: String,
    #[doc = ""]
    pub label_template: PrimField<String>,
    #[doc = ""]
    pub resource_types: SetField<PrimField<String>>,
    #[doc = ""]
    pub sink_identifier: PrimField<String>,
}

impl BuildOamLink {
    pub fn build(self, stack: &mut Stack) -> OamLink {
        let out = OamLink(Rc::new(OamLink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OamLinkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                label_template: self.label_template,
                region: core::default::Default::default(),
                resource_types: self.resource_types,
                sink_identifier: self.sink_identifier,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                link_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OamLinkRef {
    shared: StackShared,
    base: String,
}

impl Ref for OamLinkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl OamLinkRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label_template` after provisioning.\n"]
    pub fn label_template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `link_id` after provisioning.\n"]
    pub fn link_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.link_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resource_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sink_arn` after provisioning.\n"]
    pub fn sink_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sink_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sink_identifier` after provisioning.\n"]
    pub fn sink_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sink_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `link_configuration` after provisioning.\n"]
    pub fn link_configuration(&self) -> ListRef<OamLinkLinkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.link_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OamLinkTimeoutsElRef {
        OamLinkTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OamLinkLinkConfigurationElLogGroupConfigurationEl {
    filter: PrimField<String>,
}

impl OamLinkLinkConfigurationElLogGroupConfigurationEl { }

impl ToListMappable for OamLinkLinkConfigurationElLogGroupConfigurationEl {
    type O = BlockAssignable<OamLinkLinkConfigurationElLogGroupConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOamLinkLinkConfigurationElLogGroupConfigurationEl {
    #[doc = ""]
    pub filter: PrimField<String>,
}

impl BuildOamLinkLinkConfigurationElLogGroupConfigurationEl {
    pub fn build(self) -> OamLinkLinkConfigurationElLogGroupConfigurationEl {
        OamLinkLinkConfigurationElLogGroupConfigurationEl { filter: self.filter }
    }
}

pub struct OamLinkLinkConfigurationElLogGroupConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OamLinkLinkConfigurationElLogGroupConfigurationElRef {
    fn new(shared: StackShared, base: String) -> OamLinkLinkConfigurationElLogGroupConfigurationElRef {
        OamLinkLinkConfigurationElLogGroupConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OamLinkLinkConfigurationElLogGroupConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize)]
pub struct OamLinkLinkConfigurationElMetricConfigurationEl {
    filter: PrimField<String>,
}

impl OamLinkLinkConfigurationElMetricConfigurationEl { }

impl ToListMappable for OamLinkLinkConfigurationElMetricConfigurationEl {
    type O = BlockAssignable<OamLinkLinkConfigurationElMetricConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOamLinkLinkConfigurationElMetricConfigurationEl {
    #[doc = ""]
    pub filter: PrimField<String>,
}

impl BuildOamLinkLinkConfigurationElMetricConfigurationEl {
    pub fn build(self) -> OamLinkLinkConfigurationElMetricConfigurationEl {
        OamLinkLinkConfigurationElMetricConfigurationEl { filter: self.filter }
    }
}

pub struct OamLinkLinkConfigurationElMetricConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OamLinkLinkConfigurationElMetricConfigurationElRef {
    fn new(shared: StackShared, base: String) -> OamLinkLinkConfigurationElMetricConfigurationElRef {
        OamLinkLinkConfigurationElMetricConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OamLinkLinkConfigurationElMetricConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct OamLinkLinkConfigurationElDynamic {
    log_group_configuration: Option<DynamicBlock<OamLinkLinkConfigurationElLogGroupConfigurationEl>>,
    metric_configuration: Option<DynamicBlock<OamLinkLinkConfigurationElMetricConfigurationEl>>,
}

#[derive(Serialize)]
pub struct OamLinkLinkConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group_configuration: Option<Vec<OamLinkLinkConfigurationElLogGroupConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_configuration: Option<Vec<OamLinkLinkConfigurationElMetricConfigurationEl>>,
    dynamic: OamLinkLinkConfigurationElDynamic,
}

impl OamLinkLinkConfigurationEl {
    #[doc = "Set the field `log_group_configuration`.\n"]
    pub fn set_log_group_configuration(
        mut self,
        v: impl Into<BlockAssignable<OamLinkLinkConfigurationElLogGroupConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.log_group_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.log_group_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metric_configuration`.\n"]
    pub fn set_metric_configuration(
        mut self,
        v: impl Into<BlockAssignable<OamLinkLinkConfigurationElMetricConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metric_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for OamLinkLinkConfigurationEl {
    type O = BlockAssignable<OamLinkLinkConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOamLinkLinkConfigurationEl {}

impl BuildOamLinkLinkConfigurationEl {
    pub fn build(self) -> OamLinkLinkConfigurationEl {
        OamLinkLinkConfigurationEl {
            log_group_configuration: core::default::Default::default(),
            metric_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct OamLinkLinkConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OamLinkLinkConfigurationElRef {
    fn new(shared: StackShared, base: String) -> OamLinkLinkConfigurationElRef {
        OamLinkLinkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OamLinkLinkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_configuration` after provisioning.\n"]
    pub fn log_group_configuration(&self) -> ListRef<OamLinkLinkConfigurationElLogGroupConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_group_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_configuration` after provisioning.\n"]
    pub fn metric_configuration(&self) -> ListRef<OamLinkLinkConfigurationElMetricConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metric_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct OamLinkTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl OamLinkTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for OamLinkTimeoutsEl {
    type O = BlockAssignable<OamLinkTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOamLinkTimeoutsEl {}

impl BuildOamLinkTimeoutsEl {
    pub fn build(self) -> OamLinkTimeoutsEl {
        OamLinkTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct OamLinkTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OamLinkTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OamLinkTimeoutsElRef {
        OamLinkTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OamLinkTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct OamLinkDynamic {
    link_configuration: Option<DynamicBlock<OamLinkLinkConfigurationEl>>,
}
