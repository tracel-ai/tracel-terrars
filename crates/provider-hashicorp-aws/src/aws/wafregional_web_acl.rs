use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WafregionalWebAclData {
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
    metric_name: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_action: Option<Vec<WafregionalWebAclDefaultActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_configuration: Option<Vec<WafregionalWebAclLoggingConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<WafregionalWebAclRuleEl>>,
    dynamic: WafregionalWebAclDynamic,
}

struct WafregionalWebAcl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WafregionalWebAclData>,
}

#[derive(Clone)]
pub struct WafregionalWebAcl(Rc<WafregionalWebAcl_>);

impl WafregionalWebAcl {
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

    #[doc = "Set the field `default_action`.\n"]
    pub fn set_default_action(self, v: impl Into<BlockAssignable<WafregionalWebAclDefaultActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_action = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging_configuration`.\n"]
    pub fn set_logging_configuration(
        self,
        v: impl Into<BlockAssignable<WafregionalWebAclLoggingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl Into<BlockAssignable<WafregionalWebAclRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> ListRef<WafregionalWebAclDefaultActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> ListRef<WafregionalWebAclLoggingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }
}

impl Referable for WafregionalWebAcl {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for WafregionalWebAcl { }

impl ToListMappable for WafregionalWebAcl {
    type O = ListRef<WafregionalWebAclRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for WafregionalWebAcl_ {
    fn extract_resource_type(&self) -> String {
        "aws_wafregional_web_acl".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWafregionalWebAcl {
    pub tf_id: String,
    #[doc = ""]
    pub metric_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildWafregionalWebAcl {
    pub fn build(self, stack: &mut Stack) -> WafregionalWebAcl {
        let out = WafregionalWebAcl(Rc::new(WafregionalWebAcl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WafregionalWebAclData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                metric_name: self.metric_name,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                default_action: core::default::Default::default(),
                logging_configuration: core::default::Default::default(),
                rule: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WafregionalWebAclRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafregionalWebAclRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl WafregionalWebAclRef {
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

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> ListRef<WafregionalWebAclDefaultActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> ListRef<WafregionalWebAclLoggingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WafregionalWebAclDefaultActionEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl WafregionalWebAclDefaultActionEl { }

impl ToListMappable for WafregionalWebAclDefaultActionEl {
    type O = BlockAssignable<WafregionalWebAclDefaultActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAclDefaultActionEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildWafregionalWebAclDefaultActionEl {
    pub fn build(self) -> WafregionalWebAclDefaultActionEl {
        WafregionalWebAclDefaultActionEl { type_: self.type_ }
    }
}

pub struct WafregionalWebAclDefaultActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafregionalWebAclDefaultActionElRef {
    fn new(shared: StackShared, base: String) -> WafregionalWebAclDefaultActionElRef {
        WafregionalWebAclDefaultActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAclDefaultActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data = Some(v.into());
        self
    }
}

impl ToListMappable for WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    type O = BlockAssignable<WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildWafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    pub fn build(self) -> WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
        WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
            data: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
        WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafregionalWebAclLoggingConfigurationElRedactedFieldsElDynamic {
    field_to_match: Option<DynamicBlock<WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl>>,
}

#[derive(Serialize)]
pub struct WafregionalWebAclLoggingConfigurationElRedactedFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field_to_match: Option<Vec<WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl>>,
    dynamic: WafregionalWebAclLoggingConfigurationElRedactedFieldsElDynamic,
}

impl WafregionalWebAclLoggingConfigurationElRedactedFieldsEl {
    #[doc = "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v: impl Into<BlockAssignable<WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.field_to_match = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.field_to_match = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for WafregionalWebAclLoggingConfigurationElRedactedFieldsEl {
    type O = BlockAssignable<WafregionalWebAclLoggingConfigurationElRedactedFieldsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAclLoggingConfigurationElRedactedFieldsEl {}

impl BuildWafregionalWebAclLoggingConfigurationElRedactedFieldsEl {
    pub fn build(self) -> WafregionalWebAclLoggingConfigurationElRedactedFieldsEl {
        WafregionalWebAclLoggingConfigurationElRedactedFieldsEl {
            field_to_match: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct WafregionalWebAclLoggingConfigurationElRedactedFieldsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafregionalWebAclLoggingConfigurationElRedactedFieldsElRef {
    fn new(shared: StackShared, base: String) -> WafregionalWebAclLoggingConfigurationElRedactedFieldsElRef {
        WafregionalWebAclLoggingConfigurationElRedactedFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAclLoggingConfigurationElRedactedFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct WafregionalWebAclLoggingConfigurationElDynamic {
    redacted_fields: Option<DynamicBlock<WafregionalWebAclLoggingConfigurationElRedactedFieldsEl>>,
}

#[derive(Serialize)]
pub struct WafregionalWebAclLoggingConfigurationEl {
    log_destination: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redacted_fields: Option<Vec<WafregionalWebAclLoggingConfigurationElRedactedFieldsEl>>,
    dynamic: WafregionalWebAclLoggingConfigurationElDynamic,
}

impl WafregionalWebAclLoggingConfigurationEl {
    #[doc = "Set the field `redacted_fields`.\n"]
    pub fn set_redacted_fields(
        mut self,
        v: impl Into<BlockAssignable<WafregionalWebAclLoggingConfigurationElRedactedFieldsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redacted_fields = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redacted_fields = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for WafregionalWebAclLoggingConfigurationEl {
    type O = BlockAssignable<WafregionalWebAclLoggingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAclLoggingConfigurationEl {
    #[doc = ""]
    pub log_destination: PrimField<String>,
}

impl BuildWafregionalWebAclLoggingConfigurationEl {
    pub fn build(self) -> WafregionalWebAclLoggingConfigurationEl {
        WafregionalWebAclLoggingConfigurationEl {
            log_destination: self.log_destination,
            redacted_fields: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct WafregionalWebAclLoggingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafregionalWebAclLoggingConfigurationElRef {
    fn new(shared: StackShared, base: String) -> WafregionalWebAclLoggingConfigurationElRef {
        WafregionalWebAclLoggingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAclLoggingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `redacted_fields` after provisioning.\n"]
    pub fn redacted_fields(&self) -> ListRef<WafregionalWebAclLoggingConfigurationElRedactedFieldsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redacted_fields", self.base))
    }
}

#[derive(Serialize)]
pub struct WafregionalWebAclRuleElActionEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl WafregionalWebAclRuleElActionEl { }

impl ToListMappable for WafregionalWebAclRuleElActionEl {
    type O = BlockAssignable<WafregionalWebAclRuleElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAclRuleElActionEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildWafregionalWebAclRuleElActionEl {
    pub fn build(self) -> WafregionalWebAclRuleElActionEl {
        WafregionalWebAclRuleElActionEl { type_: self.type_ }
    }
}

pub struct WafregionalWebAclRuleElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafregionalWebAclRuleElActionElRef {
    fn new(shared: StackShared, base: String) -> WafregionalWebAclRuleElActionElRef {
        WafregionalWebAclRuleElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAclRuleElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct WafregionalWebAclRuleElOverrideActionEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl WafregionalWebAclRuleElOverrideActionEl { }

impl ToListMappable for WafregionalWebAclRuleElOverrideActionEl {
    type O = BlockAssignable<WafregionalWebAclRuleElOverrideActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAclRuleElOverrideActionEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildWafregionalWebAclRuleElOverrideActionEl {
    pub fn build(self) -> WafregionalWebAclRuleElOverrideActionEl {
        WafregionalWebAclRuleElOverrideActionEl { type_: self.type_ }
    }
}

pub struct WafregionalWebAclRuleElOverrideActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafregionalWebAclRuleElOverrideActionElRef {
    fn new(shared: StackShared, base: String) -> WafregionalWebAclRuleElOverrideActionElRef {
        WafregionalWebAclRuleElOverrideActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAclRuleElOverrideActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafregionalWebAclRuleElDynamic {
    action: Option<DynamicBlock<WafregionalWebAclRuleElActionEl>>,
    override_action: Option<DynamicBlock<WafregionalWebAclRuleElOverrideActionEl>>,
}

#[derive(Serialize)]
pub struct WafregionalWebAclRuleEl {
    priority: PrimField<f64>,
    rule_id: PrimField<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<WafregionalWebAclRuleElActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_action: Option<Vec<WafregionalWebAclRuleElOverrideActionEl>>,
    dynamic: WafregionalWebAclRuleElDynamic,
}

impl WafregionalWebAclRuleEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<BlockAssignable<WafregionalWebAclRuleElActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `override_action`.\n"]
    pub fn set_override_action(
        mut self,
        v: impl Into<BlockAssignable<WafregionalWebAclRuleElOverrideActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.override_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.override_action = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for WafregionalWebAclRuleEl {
    type O = BlockAssignable<WafregionalWebAclRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAclRuleEl {
    #[doc = ""]
    pub priority: PrimField<f64>,
    #[doc = ""]
    pub rule_id: PrimField<String>,
}

impl BuildWafregionalWebAclRuleEl {
    pub fn build(self) -> WafregionalWebAclRuleEl {
        WafregionalWebAclRuleEl {
            priority: self.priority,
            rule_id: self.rule_id,
            type_: core::default::Default::default(),
            action: core::default::Default::default(),
            override_action: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct WafregionalWebAclRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafregionalWebAclRuleElRef {
    fn new(shared: StackShared, base: String) -> WafregionalWebAclRuleElRef {
        WafregionalWebAclRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAclRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<WafregionalWebAclRuleElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `override_action` after provisioning.\n"]
    pub fn override_action(&self) -> ListRef<WafregionalWebAclRuleElOverrideActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.override_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafregionalWebAclDynamic {
    default_action: Option<DynamicBlock<WafregionalWebAclDefaultActionEl>>,
    logging_configuration: Option<DynamicBlock<WafregionalWebAclLoggingConfigurationEl>>,
    rule: Option<DynamicBlock<WafregionalWebAclRuleEl>>,
}
