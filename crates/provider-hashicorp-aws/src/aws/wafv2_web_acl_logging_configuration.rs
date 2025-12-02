use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct Wafv2WebAclLoggingConfigurationData {
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
    log_destination_configs: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    resource_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_filter: Option<Vec<Wafv2WebAclLoggingConfigurationLoggingFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redacted_fields: Option<Vec<Wafv2WebAclLoggingConfigurationRedactedFieldsEl>>,
    dynamic: Wafv2WebAclLoggingConfigurationDynamic,
}
struct Wafv2WebAclLoggingConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Wafv2WebAclLoggingConfigurationData>,
}
#[derive(Clone)]
pub struct Wafv2WebAclLoggingConfiguration(Rc<Wafv2WebAclLoggingConfiguration_>);
impl Wafv2WebAclLoggingConfiguration {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `logging_filter`.\n"]
    pub fn set_logging_filter(
        self,
        v: impl Into<BlockAssignable<Wafv2WebAclLoggingConfigurationLoggingFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `redacted_fields`.\n"]
    pub fn set_redacted_fields(
        self,
        v: impl Into<BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redacted_fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redacted_fields = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `log_destination_configs` after provisioning.\nAWS Kinesis Firehose Delivery Stream ARNs"]
    pub fn log_destination_configs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.log_destination_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\nAWS WebACL ARN"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_filter` after provisioning.\n"]
    pub fn logging_filter(&self) -> ListRef<Wafv2WebAclLoggingConfigurationLoggingFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redacted_fields` after provisioning.\n"]
    pub fn redacted_fields(&self) -> ListRef<Wafv2WebAclLoggingConfigurationRedactedFieldsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redacted_fields", self.extract_ref()),
        )
    }
}
impl Referable for Wafv2WebAclLoggingConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Wafv2WebAclLoggingConfiguration {}
impl ToListMappable for Wafv2WebAclLoggingConfiguration {
    type O = ListRef<Wafv2WebAclLoggingConfigurationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Wafv2WebAclLoggingConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_wafv2_web_acl_logging_configuration".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildWafv2WebAclLoggingConfiguration {
    pub tf_id: String,
    #[doc = "AWS Kinesis Firehose Delivery Stream ARNs"]
    pub log_destination_configs: SetField<PrimField<String>>,
    #[doc = "AWS WebACL ARN"]
    pub resource_arn: PrimField<String>,
}
impl BuildWafv2WebAclLoggingConfiguration {
    pub fn build(self, stack: &mut Stack) -> Wafv2WebAclLoggingConfiguration {
        let out = Wafv2WebAclLoggingConfiguration(Rc::new(Wafv2WebAclLoggingConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Wafv2WebAclLoggingConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                log_destination_configs: self.log_destination_configs,
                region: core::default::Default::default(),
                resource_arn: self.resource_arn,
                logging_filter: core::default::Default::default(),
                redacted_fields: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct Wafv2WebAclLoggingConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclLoggingConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl Wafv2WebAclLoggingConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `log_destination_configs` after provisioning.\nAWS Kinesis Firehose Delivery Stream ARNs"]
    pub fn log_destination_configs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.log_destination_configs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\nAWS WebACL ARN"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging_filter` after provisioning.\n"]
    pub fn logging_filter(&self) -> ListRef<Wafv2WebAclLoggingConfigurationLoggingFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redacted_fields` after provisioning.\n"]
    pub fn redacted_fields(&self) -> ListRef<Wafv2WebAclLoggingConfigurationRedactedFieldsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redacted_fields", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl {
    action: PrimField<String>,
}
impl Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl {}
impl ToListMappable
    for Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl
{
    type O = BlockAssignable<
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl {
    #[doc = ""]
    pub action: PrimField<String>,
}
impl BuildWafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl {
    pub fn build(
        self,
    ) -> Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl {
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl {
            action: self.action,
        }
    }
}
pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionElRef {
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
}
#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl {
    label_name: PrimField<String>,
}
impl Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl {}
impl ToListMappable
    for Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl
{
    type O = BlockAssignable<
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl
{
    #[doc = ""]
    pub label_name: PrimField<String>,
}
impl BuildWafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl {
    pub fn build(
        self,
    ) -> Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl {
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl {
            label_name: self.label_name,
        }
    }
}
pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionElRef
    {
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `label_name` after provisioning.\n"]
    pub fn label_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElDynamic {
    action_condition: Option<
        DynamicBlock<
            Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl,
        >,
    >,
    label_name_condition: Option<
        DynamicBlock<
            Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action_condition: Option<
        Vec<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    label_name_condition: Option<
        Vec<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl>,
    >,
    dynamic: Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElDynamic,
}
impl Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl {
    #[doc = "Set the field `action_condition`.\n"]
    pub fn set_action_condition(
        mut self,
        v: impl Into<
            BlockAssignable<
                Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action_condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action_condition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `label_name_condition`.\n"]
    pub fn set_label_name_condition(
        mut self,
        v : impl Into < BlockAssignable < Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.label_name_condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.label_name_condition = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl {
    type O = BlockAssignable<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl {}
impl BuildWafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl {
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl {
            action_condition: core::default::Default::default(),
            label_name_condition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElRef {
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action_condition` after provisioning.\n"]
    pub fn action_condition(
        &self,
    ) -> ListRef<
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action_condition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `label_name_condition` after provisioning.\n"]
    pub fn label_name_condition(
        &self,
    ) -> ListRef<
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.label_name_condition", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElDynamic {
    condition:
        Option<DynamicBlock<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl>>,
}
#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl {
    behavior: PrimField<String>,
    requirement: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl>>,
    dynamic: Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElDynamic,
}
impl Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl {
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl Into<BlockAssignable<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl {
    type O = BlockAssignable<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclLoggingConfigurationLoggingFilterElFilterEl {
    #[doc = ""]
    pub behavior: PrimField<String>,
    #[doc = ""]
    pub requirement: PrimField<String>,
}
impl BuildWafv2WebAclLoggingConfigurationLoggingFilterElFilterEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl {
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl {
            behavior: self.behavior,
            requirement: self.requirement,
            condition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElRef {
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `behavior` after provisioning.\n"]
    pub fn behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.behavior", self.base))
    }
    #[doc = "Get a reference to the value of field `requirement` after provisioning.\n"]
    pub fn requirement(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.requirement", self.base))
    }
}
#[derive(Serialize, Default)]
struct Wafv2WebAclLoggingConfigurationLoggingFilterElDynamic {
    filter: Option<DynamicBlock<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl>>,
}
#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationLoggingFilterEl {
    default_behavior: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl>>,
    dynamic: Wafv2WebAclLoggingConfigurationLoggingFilterElDynamic,
}
impl Wafv2WebAclLoggingConfigurationLoggingFilterEl {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl Into<BlockAssignable<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Wafv2WebAclLoggingConfigurationLoggingFilterEl {
    type O = BlockAssignable<Wafv2WebAclLoggingConfigurationLoggingFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclLoggingConfigurationLoggingFilterEl {
    #[doc = ""]
    pub default_behavior: PrimField<String>,
}
impl BuildWafv2WebAclLoggingConfigurationLoggingFilterEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationLoggingFilterEl {
        Wafv2WebAclLoggingConfigurationLoggingFilterEl {
            default_behavior: self.default_behavior,
            filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclLoggingConfigurationLoggingFilterElRef {
    fn new(shared: StackShared, base: String) -> Wafv2WebAclLoggingConfigurationLoggingFilterElRef {
        Wafv2WebAclLoggingConfigurationLoggingFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Wafv2WebAclLoggingConfigurationLoggingFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_behavior` after provisioning.\n"]
    pub fn default_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_behavior", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl {}
impl Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl {}
impl ToListMappable for Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl {
    type O = BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl {}
impl BuildWafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl {}
    }
}
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodElRef {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl {}
impl Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl {}
impl ToListMappable for Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl {
    type O = BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl {}
impl BuildWafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl {}
    }
}
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringElRef {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl {
    name: PrimField<String>,
}
impl Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl {}
impl ToListMappable for Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl {
    type O = BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildWafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl { name: self.name }
    }
}
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderElRef {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl {}
impl Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl {}
impl ToListMappable for Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl {
    type O = BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl {}
impl BuildWafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl {}
    }
}
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathElRef {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct Wafv2WebAclLoggingConfigurationRedactedFieldsElDynamic {
    method: Option<DynamicBlock<Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl>>,
    query_string:
        Option<DynamicBlock<Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl>>,
    single_header:
        Option<DynamicBlock<Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl>>,
    uri_path: Option<DynamicBlock<Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl>>,
}
#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<Vec<Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string: Option<Vec<Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_header: Option<Vec<Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri_path: Option<Vec<Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl>>,
    dynamic: Wafv2WebAclLoggingConfigurationRedactedFieldsElDynamic,
}
impl Wafv2WebAclLoggingConfigurationRedactedFieldsEl {
    #[doc = "Set the field `method`.\n"]
    pub fn set_method(
        mut self,
        v: impl Into<BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.method = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.method = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `query_string`.\n"]
    pub fn set_query_string(
        mut self,
        v: impl Into<BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_string = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_string = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `single_header`.\n"]
    pub fn set_single_header(
        mut self,
        v: impl Into<BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.single_header = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.single_header = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `uri_path`.\n"]
    pub fn set_uri_path(
        mut self,
        v: impl Into<BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.uri_path = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.uri_path = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Wafv2WebAclLoggingConfigurationRedactedFieldsEl {
    type O = BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWafv2WebAclLoggingConfigurationRedactedFieldsEl {}
impl BuildWafv2WebAclLoggingConfigurationRedactedFieldsEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationRedactedFieldsEl {
        Wafv2WebAclLoggingConfigurationRedactedFieldsEl {
            method: core::default::Default::default(),
            query_string: core::default::Default::default(),
            single_header: core::default::Default::default(),
            uri_path: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Wafv2WebAclLoggingConfigurationRedactedFieldsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElRef {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Wafv2WebAclLoggingConfigurationRedactedFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> ListRef<Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodElRef> {
        ListRef::new(self.shared().clone(), format!("{}.method", self.base))
    }
    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(
        &self,
    ) -> ListRef<Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringElRef> {
        ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }
    #[doc = "Get a reference to the value of field `single_header` after provisioning.\n"]
    pub fn single_header(
        &self,
    ) -> ListRef<Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.single_header", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri_path` after provisioning.\n"]
    pub fn uri_path(&self) -> ListRef<Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathElRef> {
        ListRef::new(self.shared().clone(), format!("{}.uri_path", self.base))
    }
}
#[derive(Serialize, Default)]
struct Wafv2WebAclLoggingConfigurationDynamic {
    logging_filter: Option<DynamicBlock<Wafv2WebAclLoggingConfigurationLoggingFilterEl>>,
    redacted_fields: Option<DynamicBlock<Wafv2WebAclLoggingConfigurationRedactedFieldsEl>>,
}
