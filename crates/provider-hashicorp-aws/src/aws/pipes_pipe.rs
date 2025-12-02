use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct PipesPipeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enrichment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    source: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    target: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enrichment_parameters: Option<Vec<PipesPipeEnrichmentParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_configuration: Option<Vec<PipesPipeLogConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_parameters: Option<Vec<PipesPipeSourceParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_parameters: Option<Vec<PipesPipeTargetParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<PipesPipeTimeoutsEl>,
    dynamic: PipesPipeDynamic,
}
struct PipesPipe_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PipesPipeData>,
}
#[derive(Clone)]
pub struct PipesPipe(Rc<PipesPipe_>);
impl PipesPipe {
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
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `desired_state`.\n"]
    pub fn set_desired_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().desired_state = Some(v.into());
        self
    }
    #[doc = "Set the field `enrichment`.\n"]
    pub fn set_enrichment(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().enrichment = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_identifier`.\n"]
    pub fn set_kms_key_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
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
    #[doc = "Set the field `enrichment_parameters`.\n"]
    pub fn set_enrichment_parameters(
        self,
        v: impl Into<BlockAssignable<PipesPipeEnrichmentParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().enrichment_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.enrichment_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `log_configuration`.\n"]
    pub fn set_log_configuration(
        self,
        v: impl Into<BlockAssignable<PipesPipeLogConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_parameters`.\n"]
    pub fn set_source_parameters(
        self,
        v: impl Into<BlockAssignable<PipesPipeSourceParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target_parameters`.\n"]
    pub fn set_target_parameters(
        self,
        v: impl Into<BlockAssignable<PipesPipeTargetParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<PipesPipeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\n"]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enrichment` after provisioning.\n"]
    pub fn enrichment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enrichment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_identifier` after provisioning.\n"]
    pub fn kms_key_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enrichment_parameters` after provisioning.\n"]
    pub fn enrichment_parameters(&self) -> ListRef<PipesPipeEnrichmentParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enrichment_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> ListRef<PipesPipeLogConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_parameters` after provisioning.\n"]
    pub fn source_parameters(&self) -> ListRef<PipesPipeSourceParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_parameters` after provisioning.\n"]
    pub fn target_parameters(&self) -> ListRef<PipesPipeTargetParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PipesPipeTimeoutsElRef {
        PipesPipeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for PipesPipe {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for PipesPipe {}
impl ToListMappable for PipesPipe {
    type O = ListRef<PipesPipeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for PipesPipe_ {
    fn extract_resource_type(&self) -> String {
        "aws_pipes_pipe".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildPipesPipe {
    pub tf_id: String,
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub source: PrimField<String>,
    #[doc = ""]
    pub target: PrimField<String>,
}
impl BuildPipesPipe {
    pub fn build(self, stack: &mut Stack) -> PipesPipe {
        let out = PipesPipe(Rc::new(PipesPipe_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PipesPipeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                desired_state: core::default::Default::default(),
                enrichment: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_identifier: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                source: self.source,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                target: self.target,
                enrichment_parameters: core::default::Default::default(),
                log_configuration: core::default::Default::default(),
                source_parameters: core::default::Default::default(),
                target_parameters: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct PipesPipeRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl PipesPipeRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\n"]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enrichment` after provisioning.\n"]
    pub fn enrichment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enrichment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_identifier` after provisioning.\n"]
    pub fn kms_key_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enrichment_parameters` after provisioning.\n"]
    pub fn enrichment_parameters(&self) -> ListRef<PipesPipeEnrichmentParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enrichment_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> ListRef<PipesPipeLogConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_parameters` after provisioning.\n"]
    pub fn source_parameters(&self) -> ListRef<PipesPipeSourceParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_parameters` after provisioning.\n"]
    pub fn target_parameters(&self) -> ListRef<PipesPipeTargetParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PipesPipeTimeoutsElRef {
        PipesPipeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeEnrichmentParametersElHttpParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_parameter_values: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string_parameters: Option<RecField<PrimField<String>>>,
}
impl PipesPipeEnrichmentParametersElHttpParametersEl {
    #[doc = "Set the field `header_parameters`.\n"]
    pub fn set_header_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.header_parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `path_parameter_values`.\n"]
    pub fn set_path_parameter_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.path_parameter_values = Some(v.into());
        self
    }
    #[doc = "Set the field `query_string_parameters`.\n"]
    pub fn set_query_string_parameters(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.query_string_parameters = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeEnrichmentParametersElHttpParametersEl {
    type O = BlockAssignable<PipesPipeEnrichmentParametersElHttpParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeEnrichmentParametersElHttpParametersEl {}
impl BuildPipesPipeEnrichmentParametersElHttpParametersEl {
    pub fn build(self) -> PipesPipeEnrichmentParametersElHttpParametersEl {
        PipesPipeEnrichmentParametersElHttpParametersEl {
            header_parameters: core::default::Default::default(),
            path_parameter_values: core::default::Default::default(),
            query_string_parameters: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeEnrichmentParametersElHttpParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeEnrichmentParametersElHttpParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeEnrichmentParametersElHttpParametersElRef {
        PipesPipeEnrichmentParametersElHttpParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeEnrichmentParametersElHttpParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_parameters` after provisioning.\n"]
    pub fn header_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.header_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path_parameter_values` after provisioning.\n"]
    pub fn path_parameter_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.path_parameter_values", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_string_parameters` after provisioning.\n"]
    pub fn query_string_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.query_string_parameters", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PipesPipeEnrichmentParametersElDynamic {
    http_parameters: Option<DynamicBlock<PipesPipeEnrichmentParametersElHttpParametersEl>>,
}
#[derive(Serialize)]
pub struct PipesPipeEnrichmentParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_parameters: Option<Vec<PipesPipeEnrichmentParametersElHttpParametersEl>>,
    dynamic: PipesPipeEnrichmentParametersElDynamic,
}
impl PipesPipeEnrichmentParametersEl {
    #[doc = "Set the field `input_template`.\n"]
    pub fn set_input_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_template = Some(v.into());
        self
    }
    #[doc = "Set the field `http_parameters`.\n"]
    pub fn set_http_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeEnrichmentParametersElHttpParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_parameters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PipesPipeEnrichmentParametersEl {
    type O = BlockAssignable<PipesPipeEnrichmentParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeEnrichmentParametersEl {}
impl BuildPipesPipeEnrichmentParametersEl {
    pub fn build(self) -> PipesPipeEnrichmentParametersEl {
        PipesPipeEnrichmentParametersEl {
            input_template: core::default::Default::default(),
            http_parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeEnrichmentParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeEnrichmentParametersElRef {
    fn new(shared: StackShared, base: String) -> PipesPipeEnrichmentParametersElRef {
        PipesPipeEnrichmentParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeEnrichmentParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `input_template` after provisioning.\n"]
    pub fn input_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_template", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `http_parameters` after provisioning.\n"]
    pub fn http_parameters(&self) -> ListRef<PipesPipeEnrichmentParametersElHttpParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_parameters", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeLogConfigurationElCloudwatchLogsLogDestinationEl {
    log_group_arn: PrimField<String>,
}
impl PipesPipeLogConfigurationElCloudwatchLogsLogDestinationEl {}
impl ToListMappable for PipesPipeLogConfigurationElCloudwatchLogsLogDestinationEl {
    type O = BlockAssignable<PipesPipeLogConfigurationElCloudwatchLogsLogDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeLogConfigurationElCloudwatchLogsLogDestinationEl {
    #[doc = ""]
    pub log_group_arn: PrimField<String>,
}
impl BuildPipesPipeLogConfigurationElCloudwatchLogsLogDestinationEl {
    pub fn build(self) -> PipesPipeLogConfigurationElCloudwatchLogsLogDestinationEl {
        PipesPipeLogConfigurationElCloudwatchLogsLogDestinationEl {
            log_group_arn: self.log_group_arn,
        }
    }
}
pub struct PipesPipeLogConfigurationElCloudwatchLogsLogDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeLogConfigurationElCloudwatchLogsLogDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeLogConfigurationElCloudwatchLogsLogDestinationElRef {
        PipesPipeLogConfigurationElCloudwatchLogsLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeLogConfigurationElCloudwatchLogsLogDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `log_group_arn` after provisioning.\n"]
    pub fn log_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_group_arn", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeLogConfigurationElFirehoseLogDestinationEl {
    delivery_stream_arn: PrimField<String>,
}
impl PipesPipeLogConfigurationElFirehoseLogDestinationEl {}
impl ToListMappable for PipesPipeLogConfigurationElFirehoseLogDestinationEl {
    type O = BlockAssignable<PipesPipeLogConfigurationElFirehoseLogDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeLogConfigurationElFirehoseLogDestinationEl {
    #[doc = ""]
    pub delivery_stream_arn: PrimField<String>,
}
impl BuildPipesPipeLogConfigurationElFirehoseLogDestinationEl {
    pub fn build(self) -> PipesPipeLogConfigurationElFirehoseLogDestinationEl {
        PipesPipeLogConfigurationElFirehoseLogDestinationEl {
            delivery_stream_arn: self.delivery_stream_arn,
        }
    }
}
pub struct PipesPipeLogConfigurationElFirehoseLogDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeLogConfigurationElFirehoseLogDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeLogConfigurationElFirehoseLogDestinationElRef {
        PipesPipeLogConfigurationElFirehoseLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeLogConfigurationElFirehoseLogDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delivery_stream_arn` after provisioning.\n"]
    pub fn delivery_stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_stream_arn", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeLogConfigurationElS3LogDestinationEl {
    bucket_name: PrimField<String>,
    bucket_owner: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}
impl PipesPipeLogConfigurationElS3LogDestinationEl {
    #[doc = "Set the field `output_format`.\n"]
    pub fn set_output_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_format = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeLogConfigurationElS3LogDestinationEl {
    type O = BlockAssignable<PipesPipeLogConfigurationElS3LogDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeLogConfigurationElS3LogDestinationEl {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
    #[doc = ""]
    pub bucket_owner: PrimField<String>,
}
impl BuildPipesPipeLogConfigurationElS3LogDestinationEl {
    pub fn build(self) -> PipesPipeLogConfigurationElS3LogDestinationEl {
        PipesPipeLogConfigurationElS3LogDestinationEl {
            bucket_name: self.bucket_name,
            bucket_owner: self.bucket_owner,
            output_format: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeLogConfigurationElS3LogDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeLogConfigurationElS3LogDestinationElRef {
    fn new(shared: StackShared, base: String) -> PipesPipeLogConfigurationElS3LogDestinationElRef {
        PipesPipeLogConfigurationElS3LogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeLogConfigurationElS3LogDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket_owner` after provisioning.\n"]
    pub fn bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_owner", self.base))
    }
    #[doc = "Get a reference to the value of field `output_format` after provisioning.\n"]
    pub fn output_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}
#[derive(Serialize, Default)]
struct PipesPipeLogConfigurationElDynamic {
    cloudwatch_logs_log_destination:
        Option<DynamicBlock<PipesPipeLogConfigurationElCloudwatchLogsLogDestinationEl>>,
    firehose_log_destination:
        Option<DynamicBlock<PipesPipeLogConfigurationElFirehoseLogDestinationEl>>,
    s3_log_destination: Option<DynamicBlock<PipesPipeLogConfigurationElS3LogDestinationEl>>,
}
#[derive(Serialize)]
pub struct PipesPipeLogConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_execution_data: Option<SetField<PrimField<String>>>,
    level: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logs_log_destination:
        Option<Vec<PipesPipeLogConfigurationElCloudwatchLogsLogDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firehose_log_destination: Option<Vec<PipesPipeLogConfigurationElFirehoseLogDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_log_destination: Option<Vec<PipesPipeLogConfigurationElS3LogDestinationEl>>,
    dynamic: PipesPipeLogConfigurationElDynamic,
}
impl PipesPipeLogConfigurationEl {
    #[doc = "Set the field `include_execution_data`.\n"]
    pub fn set_include_execution_data(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.include_execution_data = Some(v.into());
        self
    }
    #[doc = "Set the field `cloudwatch_logs_log_destination`.\n"]
    pub fn set_cloudwatch_logs_log_destination(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeLogConfigurationElCloudwatchLogsLogDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logs_log_destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs_log_destination = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `firehose_log_destination`.\n"]
    pub fn set_firehose_log_destination(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeLogConfigurationElFirehoseLogDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.firehose_log_destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.firehose_log_destination = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3_log_destination`.\n"]
    pub fn set_s3_log_destination(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeLogConfigurationElS3LogDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_log_destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_log_destination = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PipesPipeLogConfigurationEl {
    type O = BlockAssignable<PipesPipeLogConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeLogConfigurationEl {
    #[doc = ""]
    pub level: PrimField<String>,
}
impl BuildPipesPipeLogConfigurationEl {
    pub fn build(self) -> PipesPipeLogConfigurationEl {
        PipesPipeLogConfigurationEl {
            include_execution_data: core::default::Default::default(),
            level: self.level,
            cloudwatch_logs_log_destination: core::default::Default::default(),
            firehose_log_destination: core::default::Default::default(),
            s3_log_destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeLogConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeLogConfigurationElRef {
    fn new(shared: StackShared, base: String) -> PipesPipeLogConfigurationElRef {
        PipesPipeLogConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeLogConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `include_execution_data` after provisioning.\n"]
    pub fn include_execution_data(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.include_execution_data", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `level` after provisioning.\n"]
    pub fn level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.level", self.base))
    }
    #[doc = "Get a reference to the value of field `cloudwatch_logs_log_destination` after provisioning.\n"]
    pub fn cloudwatch_logs_log_destination(
        &self,
    ) -> ListRef<PipesPipeLogConfigurationElCloudwatchLogsLogDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_logs_log_destination", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `firehose_log_destination` after provisioning.\n"]
    pub fn firehose_log_destination(
        &self,
    ) -> ListRef<PipesPipeLogConfigurationElFirehoseLogDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.firehose_log_destination", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_log_destination` after provisioning.\n"]
    pub fn s3_log_destination(&self) -> ListRef<PipesPipeLogConfigurationElS3LogDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_log_destination", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeSourceParametersElActivemqBrokerParametersElCredentialsEl {
    basic_auth: PrimField<String>,
}
impl PipesPipeSourceParametersElActivemqBrokerParametersElCredentialsEl {}
impl ToListMappable for PipesPipeSourceParametersElActivemqBrokerParametersElCredentialsEl {
    type O = BlockAssignable<PipesPipeSourceParametersElActivemqBrokerParametersElCredentialsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeSourceParametersElActivemqBrokerParametersElCredentialsEl {
    #[doc = ""]
    pub basic_auth: PrimField<String>,
}
impl BuildPipesPipeSourceParametersElActivemqBrokerParametersElCredentialsEl {
    pub fn build(self) -> PipesPipeSourceParametersElActivemqBrokerParametersElCredentialsEl {
        PipesPipeSourceParametersElActivemqBrokerParametersElCredentialsEl {
            basic_auth: self.basic_auth,
        }
    }
}
pub struct PipesPipeSourceParametersElActivemqBrokerParametersElCredentialsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeSourceParametersElActivemqBrokerParametersElCredentialsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeSourceParametersElActivemqBrokerParametersElCredentialsElRef {
        PipesPipeSourceParametersElActivemqBrokerParametersElCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeSourceParametersElActivemqBrokerParametersElCredentialsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `basic_auth` after provisioning.\n"]
    pub fn basic_auth(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.basic_auth", self.base))
    }
}
#[derive(Serialize, Default)]
struct PipesPipeSourceParametersElActivemqBrokerParametersElDynamic {
    credentials:
        Option<DynamicBlock<PipesPipeSourceParametersElActivemqBrokerParametersElCredentialsEl>>,
}
#[derive(Serialize)]
pub struct PipesPipeSourceParametersElActivemqBrokerParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_batching_window_in_seconds: Option<PrimField<f64>>,
    queue_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials: Option<Vec<PipesPipeSourceParametersElActivemqBrokerParametersElCredentialsEl>>,
    dynamic: PipesPipeSourceParametersElActivemqBrokerParametersElDynamic,
}
impl PipesPipeSourceParametersElActivemqBrokerParametersEl {
    #[doc = "Set the field `batch_size`.\n"]
    pub fn set_batch_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_size = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_batching_window_in_seconds`.\n"]
    pub fn set_maximum_batching_window_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_batching_window_in_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(
        mut self,
        v: impl Into<
            BlockAssignable<PipesPipeSourceParametersElActivemqBrokerParametersElCredentialsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.credentials = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.credentials = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PipesPipeSourceParametersElActivemqBrokerParametersEl {
    type O = BlockAssignable<PipesPipeSourceParametersElActivemqBrokerParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeSourceParametersElActivemqBrokerParametersEl {
    #[doc = ""]
    pub queue_name: PrimField<String>,
}
impl BuildPipesPipeSourceParametersElActivemqBrokerParametersEl {
    pub fn build(self) -> PipesPipeSourceParametersElActivemqBrokerParametersEl {
        PipesPipeSourceParametersElActivemqBrokerParametersEl {
            batch_size: core::default::Default::default(),
            maximum_batching_window_in_seconds: core::default::Default::default(),
            queue_name: self.queue_name,
            credentials: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeSourceParametersElActivemqBrokerParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeSourceParametersElActivemqBrokerParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeSourceParametersElActivemqBrokerParametersElRef {
        PipesPipeSourceParametersElActivemqBrokerParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeSourceParametersElActivemqBrokerParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }
    #[doc = "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_batching_window_in_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `queue_name` after provisioning.\n"]
    pub fn queue_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_name", self.base))
    }
    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(
        &self,
    ) -> ListRef<PipesPipeSourceParametersElActivemqBrokerParametersElCredentialsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.credentials", self.base))
    }
}
#[derive(Serialize)]
pub struct PipesPipeSourceParametersElDynamodbStreamParametersElDeadLetterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
}
impl PipesPipeSourceParametersElDynamodbStreamParametersElDeadLetterConfigEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeSourceParametersElDynamodbStreamParametersElDeadLetterConfigEl {
    type O =
        BlockAssignable<PipesPipeSourceParametersElDynamodbStreamParametersElDeadLetterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeSourceParametersElDynamodbStreamParametersElDeadLetterConfigEl {}
impl BuildPipesPipeSourceParametersElDynamodbStreamParametersElDeadLetterConfigEl {
    pub fn build(self) -> PipesPipeSourceParametersElDynamodbStreamParametersElDeadLetterConfigEl {
        PipesPipeSourceParametersElDynamodbStreamParametersElDeadLetterConfigEl {
            arn: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeSourceParametersElDynamodbStreamParametersElDeadLetterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeSourceParametersElDynamodbStreamParametersElDeadLetterConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeSourceParametersElDynamodbStreamParametersElDeadLetterConfigElRef {
        PipesPipeSourceParametersElDynamodbStreamParametersElDeadLetterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeSourceParametersElDynamodbStreamParametersElDeadLetterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}
#[derive(Serialize, Default)]
struct PipesPipeSourceParametersElDynamodbStreamParametersElDynamic {
    dead_letter_config: Option<
        DynamicBlock<PipesPipeSourceParametersElDynamodbStreamParametersElDeadLetterConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct PipesPipeSourceParametersElDynamodbStreamParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_batching_window_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_record_age_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_retry_attempts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_partial_batch_item_failure: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallelization_factor: Option<PrimField<f64>>,
    starting_position: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_letter_config:
        Option<Vec<PipesPipeSourceParametersElDynamodbStreamParametersElDeadLetterConfigEl>>,
    dynamic: PipesPipeSourceParametersElDynamodbStreamParametersElDynamic,
}
impl PipesPipeSourceParametersElDynamodbStreamParametersEl {
    #[doc = "Set the field `batch_size`.\n"]
    pub fn set_batch_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_size = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_batching_window_in_seconds`.\n"]
    pub fn set_maximum_batching_window_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_batching_window_in_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_record_age_in_seconds`.\n"]
    pub fn set_maximum_record_age_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_record_age_in_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_retry_attempts`.\n"]
    pub fn set_maximum_retry_attempts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_retry_attempts = Some(v.into());
        self
    }
    #[doc = "Set the field `on_partial_batch_item_failure`.\n"]
    pub fn set_on_partial_batch_item_failure(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_partial_batch_item_failure = Some(v.into());
        self
    }
    #[doc = "Set the field `parallelization_factor`.\n"]
    pub fn set_parallelization_factor(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.parallelization_factor = Some(v.into());
        self
    }
    #[doc = "Set the field `dead_letter_config`.\n"]
    pub fn set_dead_letter_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                PipesPipeSourceParametersElDynamodbStreamParametersElDeadLetterConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dead_letter_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dead_letter_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PipesPipeSourceParametersElDynamodbStreamParametersEl {
    type O = BlockAssignable<PipesPipeSourceParametersElDynamodbStreamParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeSourceParametersElDynamodbStreamParametersEl {
    #[doc = ""]
    pub starting_position: PrimField<String>,
}
impl BuildPipesPipeSourceParametersElDynamodbStreamParametersEl {
    pub fn build(self) -> PipesPipeSourceParametersElDynamodbStreamParametersEl {
        PipesPipeSourceParametersElDynamodbStreamParametersEl {
            batch_size: core::default::Default::default(),
            maximum_batching_window_in_seconds: core::default::Default::default(),
            maximum_record_age_in_seconds: core::default::Default::default(),
            maximum_retry_attempts: core::default::Default::default(),
            on_partial_batch_item_failure: core::default::Default::default(),
            parallelization_factor: core::default::Default::default(),
            starting_position: self.starting_position,
            dead_letter_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeSourceParametersElDynamodbStreamParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeSourceParametersElDynamodbStreamParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeSourceParametersElDynamodbStreamParametersElRef {
        PipesPipeSourceParametersElDynamodbStreamParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeSourceParametersElDynamodbStreamParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }
    #[doc = "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_batching_window_in_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_record_age_in_seconds` after provisioning.\n"]
    pub fn maximum_record_age_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_record_age_in_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_retry_attempts` after provisioning.\n"]
    pub fn maximum_retry_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_retry_attempts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `on_partial_batch_item_failure` after provisioning.\n"]
    pub fn on_partial_batch_item_failure(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_partial_batch_item_failure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `parallelization_factor` after provisioning.\n"]
    pub fn parallelization_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parallelization_factor", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `starting_position` after provisioning.\n"]
    pub fn starting_position(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.starting_position", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(
        &self,
    ) -> ListRef<PipesPipeSourceParametersElDynamodbStreamParametersElDeadLetterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dead_letter_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeSourceParametersElFilterCriteriaElFilterEl {
    pattern: PrimField<String>,
}
impl PipesPipeSourceParametersElFilterCriteriaElFilterEl {}
impl ToListMappable for PipesPipeSourceParametersElFilterCriteriaElFilterEl {
    type O = BlockAssignable<PipesPipeSourceParametersElFilterCriteriaElFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeSourceParametersElFilterCriteriaElFilterEl {
    #[doc = ""]
    pub pattern: PrimField<String>,
}
impl BuildPipesPipeSourceParametersElFilterCriteriaElFilterEl {
    pub fn build(self) -> PipesPipeSourceParametersElFilterCriteriaElFilterEl {
        PipesPipeSourceParametersElFilterCriteriaElFilterEl {
            pattern: self.pattern,
        }
    }
}
pub struct PipesPipeSourceParametersElFilterCriteriaElFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeSourceParametersElFilterCriteriaElFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeSourceParametersElFilterCriteriaElFilterElRef {
        PipesPipeSourceParametersElFilterCriteriaElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeSourceParametersElFilterCriteriaElFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}
#[derive(Serialize, Default)]
struct PipesPipeSourceParametersElFilterCriteriaElDynamic {
    filter: Option<DynamicBlock<PipesPipeSourceParametersElFilterCriteriaElFilterEl>>,
}
#[derive(Serialize)]
pub struct PipesPipeSourceParametersElFilterCriteriaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<PipesPipeSourceParametersElFilterCriteriaElFilterEl>>,
    dynamic: PipesPipeSourceParametersElFilterCriteriaElDynamic,
}
impl PipesPipeSourceParametersElFilterCriteriaEl {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeSourceParametersElFilterCriteriaElFilterEl>>,
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
impl ToListMappable for PipesPipeSourceParametersElFilterCriteriaEl {
    type O = BlockAssignable<PipesPipeSourceParametersElFilterCriteriaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeSourceParametersElFilterCriteriaEl {}
impl BuildPipesPipeSourceParametersElFilterCriteriaEl {
    pub fn build(self) -> PipesPipeSourceParametersElFilterCriteriaEl {
        PipesPipeSourceParametersElFilterCriteriaEl {
            filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeSourceParametersElFilterCriteriaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeSourceParametersElFilterCriteriaElRef {
    fn new(shared: StackShared, base: String) -> PipesPipeSourceParametersElFilterCriteriaElRef {
        PipesPipeSourceParametersElFilterCriteriaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeSourceParametersElFilterCriteriaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<PipesPipeSourceParametersElFilterCriteriaElFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}
#[derive(Serialize)]
pub struct PipesPipeSourceParametersElKinesisStreamParametersElDeadLetterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
}
impl PipesPipeSourceParametersElKinesisStreamParametersElDeadLetterConfigEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeSourceParametersElKinesisStreamParametersElDeadLetterConfigEl {
    type O =
        BlockAssignable<PipesPipeSourceParametersElKinesisStreamParametersElDeadLetterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeSourceParametersElKinesisStreamParametersElDeadLetterConfigEl {}
impl BuildPipesPipeSourceParametersElKinesisStreamParametersElDeadLetterConfigEl {
    pub fn build(self) -> PipesPipeSourceParametersElKinesisStreamParametersElDeadLetterConfigEl {
        PipesPipeSourceParametersElKinesisStreamParametersElDeadLetterConfigEl {
            arn: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeSourceParametersElKinesisStreamParametersElDeadLetterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeSourceParametersElKinesisStreamParametersElDeadLetterConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeSourceParametersElKinesisStreamParametersElDeadLetterConfigElRef {
        PipesPipeSourceParametersElKinesisStreamParametersElDeadLetterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeSourceParametersElKinesisStreamParametersElDeadLetterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}
#[derive(Serialize, Default)]
struct PipesPipeSourceParametersElKinesisStreamParametersElDynamic {
    dead_letter_config: Option<
        DynamicBlock<PipesPipeSourceParametersElKinesisStreamParametersElDeadLetterConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct PipesPipeSourceParametersElKinesisStreamParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_batching_window_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_record_age_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_retry_attempts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_partial_batch_item_failure: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallelization_factor: Option<PrimField<f64>>,
    starting_position: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_position_timestamp: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_letter_config:
        Option<Vec<PipesPipeSourceParametersElKinesisStreamParametersElDeadLetterConfigEl>>,
    dynamic: PipesPipeSourceParametersElKinesisStreamParametersElDynamic,
}
impl PipesPipeSourceParametersElKinesisStreamParametersEl {
    #[doc = "Set the field `batch_size`.\n"]
    pub fn set_batch_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_size = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_batching_window_in_seconds`.\n"]
    pub fn set_maximum_batching_window_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_batching_window_in_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_record_age_in_seconds`.\n"]
    pub fn set_maximum_record_age_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_record_age_in_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_retry_attempts`.\n"]
    pub fn set_maximum_retry_attempts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_retry_attempts = Some(v.into());
        self
    }
    #[doc = "Set the field `on_partial_batch_item_failure`.\n"]
    pub fn set_on_partial_batch_item_failure(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_partial_batch_item_failure = Some(v.into());
        self
    }
    #[doc = "Set the field `parallelization_factor`.\n"]
    pub fn set_parallelization_factor(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.parallelization_factor = Some(v.into());
        self
    }
    #[doc = "Set the field `starting_position_timestamp`.\n"]
    pub fn set_starting_position_timestamp(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.starting_position_timestamp = Some(v.into());
        self
    }
    #[doc = "Set the field `dead_letter_config`.\n"]
    pub fn set_dead_letter_config(
        mut self,
        v: impl Into<
            BlockAssignable<PipesPipeSourceParametersElKinesisStreamParametersElDeadLetterConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dead_letter_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dead_letter_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PipesPipeSourceParametersElKinesisStreamParametersEl {
    type O = BlockAssignable<PipesPipeSourceParametersElKinesisStreamParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeSourceParametersElKinesisStreamParametersEl {
    #[doc = ""]
    pub starting_position: PrimField<String>,
}
impl BuildPipesPipeSourceParametersElKinesisStreamParametersEl {
    pub fn build(self) -> PipesPipeSourceParametersElKinesisStreamParametersEl {
        PipesPipeSourceParametersElKinesisStreamParametersEl {
            batch_size: core::default::Default::default(),
            maximum_batching_window_in_seconds: core::default::Default::default(),
            maximum_record_age_in_seconds: core::default::Default::default(),
            maximum_retry_attempts: core::default::Default::default(),
            on_partial_batch_item_failure: core::default::Default::default(),
            parallelization_factor: core::default::Default::default(),
            starting_position: self.starting_position,
            starting_position_timestamp: core::default::Default::default(),
            dead_letter_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeSourceParametersElKinesisStreamParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeSourceParametersElKinesisStreamParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeSourceParametersElKinesisStreamParametersElRef {
        PipesPipeSourceParametersElKinesisStreamParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeSourceParametersElKinesisStreamParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }
    #[doc = "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_batching_window_in_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_record_age_in_seconds` after provisioning.\n"]
    pub fn maximum_record_age_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_record_age_in_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_retry_attempts` after provisioning.\n"]
    pub fn maximum_retry_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_retry_attempts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `on_partial_batch_item_failure` after provisioning.\n"]
    pub fn on_partial_batch_item_failure(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_partial_batch_item_failure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `parallelization_factor` after provisioning.\n"]
    pub fn parallelization_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parallelization_factor", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `starting_position` after provisioning.\n"]
    pub fn starting_position(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.starting_position", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `starting_position_timestamp` after provisioning.\n"]
    pub fn starting_position_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.starting_position_timestamp", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(
        &self,
    ) -> ListRef<PipesPipeSourceParametersElKinesisStreamParametersElDeadLetterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dead_letter_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeSourceParametersElManagedStreamingKafkaParametersElCredentialsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate_tls_auth: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sasl_scram_512_auth: Option<PrimField<String>>,
}
impl PipesPipeSourceParametersElManagedStreamingKafkaParametersElCredentialsEl {
    #[doc = "Set the field `client_certificate_tls_auth`.\n"]
    pub fn set_client_certificate_tls_auth(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_certificate_tls_auth = Some(v.into());
        self
    }
    #[doc = "Set the field `sasl_scram_512_auth`.\n"]
    pub fn set_sasl_scram_512_auth(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sasl_scram_512_auth = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeSourceParametersElManagedStreamingKafkaParametersElCredentialsEl {
    type O =
        BlockAssignable<PipesPipeSourceParametersElManagedStreamingKafkaParametersElCredentialsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeSourceParametersElManagedStreamingKafkaParametersElCredentialsEl {}
impl BuildPipesPipeSourceParametersElManagedStreamingKafkaParametersElCredentialsEl {
    pub fn build(
        self,
    ) -> PipesPipeSourceParametersElManagedStreamingKafkaParametersElCredentialsEl {
        PipesPipeSourceParametersElManagedStreamingKafkaParametersElCredentialsEl {
            client_certificate_tls_auth: core::default::Default::default(),
            sasl_scram_512_auth: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeSourceParametersElManagedStreamingKafkaParametersElCredentialsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeSourceParametersElManagedStreamingKafkaParametersElCredentialsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeSourceParametersElManagedStreamingKafkaParametersElCredentialsElRef {
        PipesPipeSourceParametersElManagedStreamingKafkaParametersElCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeSourceParametersElManagedStreamingKafkaParametersElCredentialsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_certificate_tls_auth` after provisioning.\n"]
    pub fn client_certificate_tls_auth(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_certificate_tls_auth", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sasl_scram_512_auth` after provisioning.\n"]
    pub fn sasl_scram_512_auth(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sasl_scram_512_auth", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PipesPipeSourceParametersElManagedStreamingKafkaParametersElDynamic {
    credentials: Option<
        DynamicBlock<PipesPipeSourceParametersElManagedStreamingKafkaParametersElCredentialsEl>,
    >,
}
#[derive(Serialize)]
pub struct PipesPipeSourceParametersElManagedStreamingKafkaParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_batching_window_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_position: Option<PrimField<String>>,
    topic_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials:
        Option<Vec<PipesPipeSourceParametersElManagedStreamingKafkaParametersElCredentialsEl>>,
    dynamic: PipesPipeSourceParametersElManagedStreamingKafkaParametersElDynamic,
}
impl PipesPipeSourceParametersElManagedStreamingKafkaParametersEl {
    #[doc = "Set the field `batch_size`.\n"]
    pub fn set_batch_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_size = Some(v.into());
        self
    }
    #[doc = "Set the field `consumer_group_id`.\n"]
    pub fn set_consumer_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consumer_group_id = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_batching_window_in_seconds`.\n"]
    pub fn set_maximum_batching_window_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_batching_window_in_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `starting_position`.\n"]
    pub fn set_starting_position(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.starting_position = Some(v.into());
        self
    }
    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(
        mut self,
        v: impl Into<
            BlockAssignable<
                PipesPipeSourceParametersElManagedStreamingKafkaParametersElCredentialsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.credentials = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.credentials = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PipesPipeSourceParametersElManagedStreamingKafkaParametersEl {
    type O = BlockAssignable<PipesPipeSourceParametersElManagedStreamingKafkaParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeSourceParametersElManagedStreamingKafkaParametersEl {
    #[doc = ""]
    pub topic_name: PrimField<String>,
}
impl BuildPipesPipeSourceParametersElManagedStreamingKafkaParametersEl {
    pub fn build(self) -> PipesPipeSourceParametersElManagedStreamingKafkaParametersEl {
        PipesPipeSourceParametersElManagedStreamingKafkaParametersEl {
            batch_size: core::default::Default::default(),
            consumer_group_id: core::default::Default::default(),
            maximum_batching_window_in_seconds: core::default::Default::default(),
            starting_position: core::default::Default::default(),
            topic_name: self.topic_name,
            credentials: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeSourceParametersElManagedStreamingKafkaParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeSourceParametersElManagedStreamingKafkaParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeSourceParametersElManagedStreamingKafkaParametersElRef {
        PipesPipeSourceParametersElManagedStreamingKafkaParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeSourceParametersElManagedStreamingKafkaParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }
    #[doc = "Get a reference to the value of field `consumer_group_id` after provisioning.\n"]
    pub fn consumer_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consumer_group_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_batching_window_in_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `starting_position` after provisioning.\n"]
    pub fn starting_position(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.starting_position", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topic_name` after provisioning.\n"]
    pub fn topic_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic_name", self.base))
    }
    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(
        &self,
    ) -> ListRef<PipesPipeSourceParametersElManagedStreamingKafkaParametersElCredentialsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.credentials", self.base))
    }
}
#[derive(Serialize)]
pub struct PipesPipeSourceParametersElRabbitmqBrokerParametersElCredentialsEl {
    basic_auth: PrimField<String>,
}
impl PipesPipeSourceParametersElRabbitmqBrokerParametersElCredentialsEl {}
impl ToListMappable for PipesPipeSourceParametersElRabbitmqBrokerParametersElCredentialsEl {
    type O = BlockAssignable<PipesPipeSourceParametersElRabbitmqBrokerParametersElCredentialsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeSourceParametersElRabbitmqBrokerParametersElCredentialsEl {
    #[doc = ""]
    pub basic_auth: PrimField<String>,
}
impl BuildPipesPipeSourceParametersElRabbitmqBrokerParametersElCredentialsEl {
    pub fn build(self) -> PipesPipeSourceParametersElRabbitmqBrokerParametersElCredentialsEl {
        PipesPipeSourceParametersElRabbitmqBrokerParametersElCredentialsEl {
            basic_auth: self.basic_auth,
        }
    }
}
pub struct PipesPipeSourceParametersElRabbitmqBrokerParametersElCredentialsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeSourceParametersElRabbitmqBrokerParametersElCredentialsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeSourceParametersElRabbitmqBrokerParametersElCredentialsElRef {
        PipesPipeSourceParametersElRabbitmqBrokerParametersElCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeSourceParametersElRabbitmqBrokerParametersElCredentialsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `basic_auth` after provisioning.\n"]
    pub fn basic_auth(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.basic_auth", self.base))
    }
}
#[derive(Serialize, Default)]
struct PipesPipeSourceParametersElRabbitmqBrokerParametersElDynamic {
    credentials:
        Option<DynamicBlock<PipesPipeSourceParametersElRabbitmqBrokerParametersElCredentialsEl>>,
}
#[derive(Serialize)]
pub struct PipesPipeSourceParametersElRabbitmqBrokerParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_batching_window_in_seconds: Option<PrimField<f64>>,
    queue_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials: Option<Vec<PipesPipeSourceParametersElRabbitmqBrokerParametersElCredentialsEl>>,
    dynamic: PipesPipeSourceParametersElRabbitmqBrokerParametersElDynamic,
}
impl PipesPipeSourceParametersElRabbitmqBrokerParametersEl {
    #[doc = "Set the field `batch_size`.\n"]
    pub fn set_batch_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_size = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_batching_window_in_seconds`.\n"]
    pub fn set_maximum_batching_window_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_batching_window_in_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `virtual_host`.\n"]
    pub fn set_virtual_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.virtual_host = Some(v.into());
        self
    }
    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(
        mut self,
        v: impl Into<
            BlockAssignable<PipesPipeSourceParametersElRabbitmqBrokerParametersElCredentialsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.credentials = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.credentials = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PipesPipeSourceParametersElRabbitmqBrokerParametersEl {
    type O = BlockAssignable<PipesPipeSourceParametersElRabbitmqBrokerParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeSourceParametersElRabbitmqBrokerParametersEl {
    #[doc = ""]
    pub queue_name: PrimField<String>,
}
impl BuildPipesPipeSourceParametersElRabbitmqBrokerParametersEl {
    pub fn build(self) -> PipesPipeSourceParametersElRabbitmqBrokerParametersEl {
        PipesPipeSourceParametersElRabbitmqBrokerParametersEl {
            batch_size: core::default::Default::default(),
            maximum_batching_window_in_seconds: core::default::Default::default(),
            queue_name: self.queue_name,
            virtual_host: core::default::Default::default(),
            credentials: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeSourceParametersElRabbitmqBrokerParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeSourceParametersElRabbitmqBrokerParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeSourceParametersElRabbitmqBrokerParametersElRef {
        PipesPipeSourceParametersElRabbitmqBrokerParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeSourceParametersElRabbitmqBrokerParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }
    #[doc = "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_batching_window_in_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `queue_name` after provisioning.\n"]
    pub fn queue_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_name", self.base))
    }
    #[doc = "Get a reference to the value of field `virtual_host` after provisioning.\n"]
    pub fn virtual_host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_host", self.base))
    }
    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(
        &self,
    ) -> ListRef<PipesPipeSourceParametersElRabbitmqBrokerParametersElCredentialsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.credentials", self.base))
    }
}
#[derive(Serialize)]
pub struct PipesPipeSourceParametersElSelfManagedKafkaParametersElCredentialsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_auth: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate_tls_auth: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sasl_scram_256_auth: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sasl_scram_512_auth: Option<PrimField<String>>,
}
impl PipesPipeSourceParametersElSelfManagedKafkaParametersElCredentialsEl {
    #[doc = "Set the field `basic_auth`.\n"]
    pub fn set_basic_auth(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.basic_auth = Some(v.into());
        self
    }
    #[doc = "Set the field `client_certificate_tls_auth`.\n"]
    pub fn set_client_certificate_tls_auth(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_certificate_tls_auth = Some(v.into());
        self
    }
    #[doc = "Set the field `sasl_scram_256_auth`.\n"]
    pub fn set_sasl_scram_256_auth(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sasl_scram_256_auth = Some(v.into());
        self
    }
    #[doc = "Set the field `sasl_scram_512_auth`.\n"]
    pub fn set_sasl_scram_512_auth(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sasl_scram_512_auth = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeSourceParametersElSelfManagedKafkaParametersElCredentialsEl {
    type O = BlockAssignable<PipesPipeSourceParametersElSelfManagedKafkaParametersElCredentialsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeSourceParametersElSelfManagedKafkaParametersElCredentialsEl {}
impl BuildPipesPipeSourceParametersElSelfManagedKafkaParametersElCredentialsEl {
    pub fn build(self) -> PipesPipeSourceParametersElSelfManagedKafkaParametersElCredentialsEl {
        PipesPipeSourceParametersElSelfManagedKafkaParametersElCredentialsEl {
            basic_auth: core::default::Default::default(),
            client_certificate_tls_auth: core::default::Default::default(),
            sasl_scram_256_auth: core::default::Default::default(),
            sasl_scram_512_auth: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeSourceParametersElSelfManagedKafkaParametersElCredentialsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeSourceParametersElSelfManagedKafkaParametersElCredentialsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeSourceParametersElSelfManagedKafkaParametersElCredentialsElRef {
        PipesPipeSourceParametersElSelfManagedKafkaParametersElCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeSourceParametersElSelfManagedKafkaParametersElCredentialsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `basic_auth` after provisioning.\n"]
    pub fn basic_auth(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.basic_auth", self.base))
    }
    #[doc = "Get a reference to the value of field `client_certificate_tls_auth` after provisioning.\n"]
    pub fn client_certificate_tls_auth(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_certificate_tls_auth", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sasl_scram_256_auth` after provisioning.\n"]
    pub fn sasl_scram_256_auth(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sasl_scram_256_auth", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sasl_scram_512_auth` after provisioning.\n"]
    pub fn sasl_scram_512_auth(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sasl_scram_512_auth", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeSourceParametersElSelfManagedKafkaParametersElVpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnets: Option<SetField<PrimField<String>>>,
}
impl PipesPipeSourceParametersElSelfManagedKafkaParametersElVpcEl {
    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `subnets`.\n"]
    pub fn set_subnets(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnets = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeSourceParametersElSelfManagedKafkaParametersElVpcEl {
    type O = BlockAssignable<PipesPipeSourceParametersElSelfManagedKafkaParametersElVpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeSourceParametersElSelfManagedKafkaParametersElVpcEl {}
impl BuildPipesPipeSourceParametersElSelfManagedKafkaParametersElVpcEl {
    pub fn build(self) -> PipesPipeSourceParametersElSelfManagedKafkaParametersElVpcEl {
        PipesPipeSourceParametersElSelfManagedKafkaParametersElVpcEl {
            security_groups: core::default::Default::default(),
            subnets: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeSourceParametersElSelfManagedKafkaParametersElVpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeSourceParametersElSelfManagedKafkaParametersElVpcElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeSourceParametersElSelfManagedKafkaParametersElVpcElRef {
        PipesPipeSourceParametersElSelfManagedKafkaParametersElVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeSourceParametersElSelfManagedKafkaParametersElVpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}
#[derive(Serialize, Default)]
struct PipesPipeSourceParametersElSelfManagedKafkaParametersElDynamic {
    credentials:
        Option<DynamicBlock<PipesPipeSourceParametersElSelfManagedKafkaParametersElCredentialsEl>>,
    vpc: Option<DynamicBlock<PipesPipeSourceParametersElSelfManagedKafkaParametersElVpcEl>>,
}
#[derive(Serialize)]
pub struct PipesPipeSourceParametersElSelfManagedKafkaParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_bootstrap_servers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_batching_window_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_root_ca_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_position: Option<PrimField<String>>,
    topic_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials: Option<Vec<PipesPipeSourceParametersElSelfManagedKafkaParametersElCredentialsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc: Option<Vec<PipesPipeSourceParametersElSelfManagedKafkaParametersElVpcEl>>,
    dynamic: PipesPipeSourceParametersElSelfManagedKafkaParametersElDynamic,
}
impl PipesPipeSourceParametersElSelfManagedKafkaParametersEl {
    #[doc = "Set the field `additional_bootstrap_servers`.\n"]
    pub fn set_additional_bootstrap_servers(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.additional_bootstrap_servers = Some(v.into());
        self
    }
    #[doc = "Set the field `batch_size`.\n"]
    pub fn set_batch_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_size = Some(v.into());
        self
    }
    #[doc = "Set the field `consumer_group_id`.\n"]
    pub fn set_consumer_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consumer_group_id = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_batching_window_in_seconds`.\n"]
    pub fn set_maximum_batching_window_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_batching_window_in_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `server_root_ca_certificate`.\n"]
    pub fn set_server_root_ca_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.server_root_ca_certificate = Some(v.into());
        self
    }
    #[doc = "Set the field `starting_position`.\n"]
    pub fn set_starting_position(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.starting_position = Some(v.into());
        self
    }
    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(
        mut self,
        v: impl Into<
            BlockAssignable<PipesPipeSourceParametersElSelfManagedKafkaParametersElCredentialsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.credentials = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.credentials = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeSourceParametersElSelfManagedKafkaParametersElVpcEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PipesPipeSourceParametersElSelfManagedKafkaParametersEl {
    type O = BlockAssignable<PipesPipeSourceParametersElSelfManagedKafkaParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeSourceParametersElSelfManagedKafkaParametersEl {
    #[doc = ""]
    pub topic_name: PrimField<String>,
}
impl BuildPipesPipeSourceParametersElSelfManagedKafkaParametersEl {
    pub fn build(self) -> PipesPipeSourceParametersElSelfManagedKafkaParametersEl {
        PipesPipeSourceParametersElSelfManagedKafkaParametersEl {
            additional_bootstrap_servers: core::default::Default::default(),
            batch_size: core::default::Default::default(),
            consumer_group_id: core::default::Default::default(),
            maximum_batching_window_in_seconds: core::default::Default::default(),
            server_root_ca_certificate: core::default::Default::default(),
            starting_position: core::default::Default::default(),
            topic_name: self.topic_name,
            credentials: core::default::Default::default(),
            vpc: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeSourceParametersElSelfManagedKafkaParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeSourceParametersElSelfManagedKafkaParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeSourceParametersElSelfManagedKafkaParametersElRef {
        PipesPipeSourceParametersElSelfManagedKafkaParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeSourceParametersElSelfManagedKafkaParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_bootstrap_servers` after provisioning.\n"]
    pub fn additional_bootstrap_servers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.additional_bootstrap_servers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }
    #[doc = "Get a reference to the value of field `consumer_group_id` after provisioning.\n"]
    pub fn consumer_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consumer_group_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_batching_window_in_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `server_root_ca_certificate` after provisioning.\n"]
    pub fn server_root_ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_root_ca_certificate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `starting_position` after provisioning.\n"]
    pub fn starting_position(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.starting_position", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topic_name` after provisioning.\n"]
    pub fn topic_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic_name", self.base))
    }
    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(
        &self,
    ) -> ListRef<PipesPipeSourceParametersElSelfManagedKafkaParametersElCredentialsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.credentials", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> ListRef<PipesPipeSourceParametersElSelfManagedKafkaParametersElVpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}
#[derive(Serialize)]
pub struct PipesPipeSourceParametersElSqsQueueParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_batching_window_in_seconds: Option<PrimField<f64>>,
}
impl PipesPipeSourceParametersElSqsQueueParametersEl {
    #[doc = "Set the field `batch_size`.\n"]
    pub fn set_batch_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_size = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum_batching_window_in_seconds`.\n"]
    pub fn set_maximum_batching_window_in_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_batching_window_in_seconds = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeSourceParametersElSqsQueueParametersEl {
    type O = BlockAssignable<PipesPipeSourceParametersElSqsQueueParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeSourceParametersElSqsQueueParametersEl {}
impl BuildPipesPipeSourceParametersElSqsQueueParametersEl {
    pub fn build(self) -> PipesPipeSourceParametersElSqsQueueParametersEl {
        PipesPipeSourceParametersElSqsQueueParametersEl {
            batch_size: core::default::Default::default(),
            maximum_batching_window_in_seconds: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeSourceParametersElSqsQueueParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeSourceParametersElSqsQueueParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeSourceParametersElSqsQueueParametersElRef {
        PipesPipeSourceParametersElSqsQueueParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeSourceParametersElSqsQueueParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }
    #[doc = "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_batching_window_in_seconds", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PipesPipeSourceParametersElDynamic {
    activemq_broker_parameters:
        Option<DynamicBlock<PipesPipeSourceParametersElActivemqBrokerParametersEl>>,
    dynamodb_stream_parameters:
        Option<DynamicBlock<PipesPipeSourceParametersElDynamodbStreamParametersEl>>,
    filter_criteria: Option<DynamicBlock<PipesPipeSourceParametersElFilterCriteriaEl>>,
    kinesis_stream_parameters:
        Option<DynamicBlock<PipesPipeSourceParametersElKinesisStreamParametersEl>>,
    managed_streaming_kafka_parameters:
        Option<DynamicBlock<PipesPipeSourceParametersElManagedStreamingKafkaParametersEl>>,
    rabbitmq_broker_parameters:
        Option<DynamicBlock<PipesPipeSourceParametersElRabbitmqBrokerParametersEl>>,
    self_managed_kafka_parameters:
        Option<DynamicBlock<PipesPipeSourceParametersElSelfManagedKafkaParametersEl>>,
    sqs_queue_parameters: Option<DynamicBlock<PipesPipeSourceParametersElSqsQueueParametersEl>>,
}
#[derive(Serialize)]
pub struct PipesPipeSourceParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    activemq_broker_parameters: Option<Vec<PipesPipeSourceParametersElActivemqBrokerParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynamodb_stream_parameters: Option<Vec<PipesPipeSourceParametersElDynamodbStreamParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_criteria: Option<Vec<PipesPipeSourceParametersElFilterCriteriaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_stream_parameters: Option<Vec<PipesPipeSourceParametersElKinesisStreamParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_streaming_kafka_parameters:
        Option<Vec<PipesPipeSourceParametersElManagedStreamingKafkaParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rabbitmq_broker_parameters: Option<Vec<PipesPipeSourceParametersElRabbitmqBrokerParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_managed_kafka_parameters:
        Option<Vec<PipesPipeSourceParametersElSelfManagedKafkaParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sqs_queue_parameters: Option<Vec<PipesPipeSourceParametersElSqsQueueParametersEl>>,
    dynamic: PipesPipeSourceParametersElDynamic,
}
impl PipesPipeSourceParametersEl {
    #[doc = "Set the field `activemq_broker_parameters`.\n"]
    pub fn set_activemq_broker_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeSourceParametersElActivemqBrokerParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.activemq_broker_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.activemq_broker_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dynamodb_stream_parameters`.\n"]
    pub fn set_dynamodb_stream_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeSourceParametersElDynamodbStreamParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dynamodb_stream_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dynamodb_stream_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `filter_criteria`.\n"]
    pub fn set_filter_criteria(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeSourceParametersElFilterCriteriaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter_criteria = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_criteria = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kinesis_stream_parameters`.\n"]
    pub fn set_kinesis_stream_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeSourceParametersElKinesisStreamParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_stream_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_stream_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `managed_streaming_kafka_parameters`.\n"]
    pub fn set_managed_streaming_kafka_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeSourceParametersElManagedStreamingKafkaParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.managed_streaming_kafka_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_streaming_kafka_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rabbitmq_broker_parameters`.\n"]
    pub fn set_rabbitmq_broker_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeSourceParametersElRabbitmqBrokerParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rabbitmq_broker_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rabbitmq_broker_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `self_managed_kafka_parameters`.\n"]
    pub fn set_self_managed_kafka_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeSourceParametersElSelfManagedKafkaParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.self_managed_kafka_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.self_managed_kafka_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sqs_queue_parameters`.\n"]
    pub fn set_sqs_queue_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeSourceParametersElSqsQueueParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sqs_queue_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sqs_queue_parameters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PipesPipeSourceParametersEl {
    type O = BlockAssignable<PipesPipeSourceParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeSourceParametersEl {}
impl BuildPipesPipeSourceParametersEl {
    pub fn build(self) -> PipesPipeSourceParametersEl {
        PipesPipeSourceParametersEl {
            activemq_broker_parameters: core::default::Default::default(),
            dynamodb_stream_parameters: core::default::Default::default(),
            filter_criteria: core::default::Default::default(),
            kinesis_stream_parameters: core::default::Default::default(),
            managed_streaming_kafka_parameters: core::default::Default::default(),
            rabbitmq_broker_parameters: core::default::Default::default(),
            self_managed_kafka_parameters: core::default::Default::default(),
            sqs_queue_parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeSourceParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeSourceParametersElRef {
    fn new(shared: StackShared, base: String) -> PipesPipeSourceParametersElRef {
        PipesPipeSourceParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeSourceParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `activemq_broker_parameters` after provisioning.\n"]
    pub fn activemq_broker_parameters(
        &self,
    ) -> ListRef<PipesPipeSourceParametersElActivemqBrokerParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.activemq_broker_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dynamodb_stream_parameters` after provisioning.\n"]
    pub fn dynamodb_stream_parameters(
        &self,
    ) -> ListRef<PipesPipeSourceParametersElDynamodbStreamParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dynamodb_stream_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter_criteria` after provisioning.\n"]
    pub fn filter_criteria(&self) -> ListRef<PipesPipeSourceParametersElFilterCriteriaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_criteria", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kinesis_stream_parameters` after provisioning.\n"]
    pub fn kinesis_stream_parameters(
        &self,
    ) -> ListRef<PipesPipeSourceParametersElKinesisStreamParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_stream_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `managed_streaming_kafka_parameters` after provisioning.\n"]
    pub fn managed_streaming_kafka_parameters(
        &self,
    ) -> ListRef<PipesPipeSourceParametersElManagedStreamingKafkaParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.managed_streaming_kafka_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rabbitmq_broker_parameters` after provisioning.\n"]
    pub fn rabbitmq_broker_parameters(
        &self,
    ) -> ListRef<PipesPipeSourceParametersElRabbitmqBrokerParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rabbitmq_broker_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `self_managed_kafka_parameters` after provisioning.\n"]
    pub fn self_managed_kafka_parameters(
        &self,
    ) -> ListRef<PipesPipeSourceParametersElSelfManagedKafkaParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.self_managed_kafka_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sqs_queue_parameters` after provisioning.\n"]
    pub fn sqs_queue_parameters(
        &self,
    ) -> ListRef<PipesPipeSourceParametersElSqsQueueParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sqs_queue_parameters", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElBatchJobParametersElArrayPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
}
impl PipesPipeTargetParametersElBatchJobParametersElArrayPropertiesEl {
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersElBatchJobParametersElArrayPropertiesEl {
    type O = BlockAssignable<PipesPipeTargetParametersElBatchJobParametersElArrayPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElBatchJobParametersElArrayPropertiesEl {}
impl BuildPipesPipeTargetParametersElBatchJobParametersElArrayPropertiesEl {
    pub fn build(self) -> PipesPipeTargetParametersElBatchJobParametersElArrayPropertiesEl {
        PipesPipeTargetParametersElBatchJobParametersElArrayPropertiesEl {
            size: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElBatchJobParametersElArrayPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElBatchJobParametersElArrayPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElBatchJobParametersElArrayPropertiesElRef {
        PipesPipeTargetParametersElBatchJobParametersElArrayPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElBatchJobParametersElArrayPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElEnvironmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElEnvironmentEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElEnvironmentEl
{
    type O = BlockAssignable<
        PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElEnvironmentEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElBatchJobParametersElContainerOverridesElEnvironmentEl {
}
impl BuildPipesPipeTargetParametersElBatchJobParametersElContainerOverridesElEnvironmentEl {
    pub fn build(
        self,
    ) -> PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElEnvironmentEl {
        PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElEnvironmentEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElEnvironmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElEnvironmentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElEnvironmentElRef {
        PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElEnvironmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElResourceRequirementEl
{
    #[serde(rename = "type")]
    type_: PrimField<String>,
    value: PrimField<String>,
}
impl PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElResourceRequirementEl {}
impl ToListMappable
    for PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElResourceRequirementEl
{
    type O = BlockAssignable<
        PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElResourceRequirementEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElBatchJobParametersElContainerOverridesElResourceRequirementEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildPipesPipeTargetParametersElBatchJobParametersElContainerOverridesElResourceRequirementEl {
    pub fn build(
        self,
    ) -> PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElResourceRequirementEl
    {
        PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElResourceRequirementEl {
            type_: self.type_,
            value: self.value,
        }
    }
}
pub struct PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElResourceRequirementElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElResourceRequirementElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElResourceRequirementElRef
    {
        PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElResourceRequirementElRef { shared : shared , base : base . to_string () , }
    }
}
impl PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElResourceRequirementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElDynamic { environment : Option < DynamicBlock < PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElEnvironmentEl >> , resource_requirement : Option < DynamicBlock < PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElResourceRequirementEl >> , }
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElBatchJobParametersElContainerOverridesEl { # [serde (skip_serializing_if = "Option::is_none")] command : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] instance_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] environment : Option < Vec < PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElEnvironmentEl > > , # [serde (skip_serializing_if = "Option::is_none")] resource_requirement : Option < Vec < PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElResourceRequirementEl > > , dynamic : PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElDynamic , }
impl PipesPipeTargetParametersElBatchJobParametersElContainerOverridesEl {
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl Into<
            BlockAssignable<
                PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElEnvironmentEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.environment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.environment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_requirement`.\n"]
    pub fn set_resource_requirement(
        mut self,
        v : impl Into < BlockAssignable < PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElResourceRequirementEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_requirement = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_requirement = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersElBatchJobParametersElContainerOverridesEl {
    type O = BlockAssignable<PipesPipeTargetParametersElBatchJobParametersElContainerOverridesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElBatchJobParametersElContainerOverridesEl {}
impl BuildPipesPipeTargetParametersElBatchJobParametersElContainerOverridesEl {
    pub fn build(self) -> PipesPipeTargetParametersElBatchJobParametersElContainerOverridesEl {
        PipesPipeTargetParametersElBatchJobParametersElContainerOverridesEl {
            command: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            environment: core::default::Default::default(),
            resource_requirement: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElRef {
        PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElEnvironmentElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_requirement` after provisioning.\n"]
    pub fn resource_requirement(
        &self,
    ) -> ListRef<
        PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElResourceRequirementElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_requirement", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElBatchJobParametersElDependsOnEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    job_id: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl PipesPipeTargetParametersElBatchJobParametersElDependsOnEl {
    #[doc = "Set the field `job_id`.\n"]
    pub fn set_job_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.job_id = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersElBatchJobParametersElDependsOnEl {
    type O = BlockAssignable<PipesPipeTargetParametersElBatchJobParametersElDependsOnEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElBatchJobParametersElDependsOnEl {}
impl BuildPipesPipeTargetParametersElBatchJobParametersElDependsOnEl {
    pub fn build(self) -> PipesPipeTargetParametersElBatchJobParametersElDependsOnEl {
        PipesPipeTargetParametersElBatchJobParametersElDependsOnEl {
            job_id: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElBatchJobParametersElDependsOnElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElBatchJobParametersElDependsOnElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElBatchJobParametersElDependsOnElRef {
        PipesPipeTargetParametersElBatchJobParametersElDependsOnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElBatchJobParametersElDependsOnElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `job_id` after provisioning.\n"]
    pub fn job_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_id", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElBatchJobParametersElRetryStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attempts: Option<PrimField<f64>>,
}
impl PipesPipeTargetParametersElBatchJobParametersElRetryStrategyEl {
    #[doc = "Set the field `attempts`.\n"]
    pub fn set_attempts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.attempts = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersElBatchJobParametersElRetryStrategyEl {
    type O = BlockAssignable<PipesPipeTargetParametersElBatchJobParametersElRetryStrategyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElBatchJobParametersElRetryStrategyEl {}
impl BuildPipesPipeTargetParametersElBatchJobParametersElRetryStrategyEl {
    pub fn build(self) -> PipesPipeTargetParametersElBatchJobParametersElRetryStrategyEl {
        PipesPipeTargetParametersElBatchJobParametersElRetryStrategyEl {
            attempts: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElBatchJobParametersElRetryStrategyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElBatchJobParametersElRetryStrategyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElBatchJobParametersElRetryStrategyElRef {
        PipesPipeTargetParametersElBatchJobParametersElRetryStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElBatchJobParametersElRetryStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attempts` after provisioning.\n"]
    pub fn attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.attempts", self.base))
    }
}
#[derive(Serialize, Default)]
struct PipesPipeTargetParametersElBatchJobParametersElDynamic {
    array_properties:
        Option<DynamicBlock<PipesPipeTargetParametersElBatchJobParametersElArrayPropertiesEl>>,
    container_overrides:
        Option<DynamicBlock<PipesPipeTargetParametersElBatchJobParametersElContainerOverridesEl>>,
    depends_on: Option<DynamicBlock<PipesPipeTargetParametersElBatchJobParametersElDependsOnEl>>,
    retry_strategy:
        Option<DynamicBlock<PipesPipeTargetParametersElBatchJobParametersElRetryStrategyEl>>,
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElBatchJobParametersEl {
    job_definition: PrimField<String>,
    job_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    array_properties: Option<Vec<PipesPipeTargetParametersElBatchJobParametersElArrayPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_overrides:
        Option<Vec<PipesPipeTargetParametersElBatchJobParametersElContainerOverridesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    depends_on: Option<Vec<PipesPipeTargetParametersElBatchJobParametersElDependsOnEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_strategy: Option<Vec<PipesPipeTargetParametersElBatchJobParametersElRetryStrategyEl>>,
    dynamic: PipesPipeTargetParametersElBatchJobParametersElDynamic,
}
impl PipesPipeTargetParametersElBatchJobParametersEl {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `array_properties`.\n"]
    pub fn set_array_properties(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeTargetParametersElBatchJobParametersElArrayPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.array_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.array_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `container_overrides`.\n"]
    pub fn set_container_overrides(
        mut self,
        v: impl Into<
            BlockAssignable<PipesPipeTargetParametersElBatchJobParametersElContainerOverridesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container_overrides = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container_overrides = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `depends_on`.\n"]
    pub fn set_depends_on(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeTargetParametersElBatchJobParametersElDependsOnEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.depends_on = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.depends_on = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `retry_strategy`.\n"]
    pub fn set_retry_strategy(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeTargetParametersElBatchJobParametersElRetryStrategyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retry_strategy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_strategy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersElBatchJobParametersEl {
    type O = BlockAssignable<PipesPipeTargetParametersElBatchJobParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElBatchJobParametersEl {
    #[doc = ""]
    pub job_definition: PrimField<String>,
    #[doc = ""]
    pub job_name: PrimField<String>,
}
impl BuildPipesPipeTargetParametersElBatchJobParametersEl {
    pub fn build(self) -> PipesPipeTargetParametersElBatchJobParametersEl {
        PipesPipeTargetParametersElBatchJobParametersEl {
            job_definition: self.job_definition,
            job_name: self.job_name,
            parameters: core::default::Default::default(),
            array_properties: core::default::Default::default(),
            container_overrides: core::default::Default::default(),
            depends_on: core::default::Default::default(),
            retry_strategy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElBatchJobParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElBatchJobParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElBatchJobParametersElRef {
        PipesPipeTargetParametersElBatchJobParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElBatchJobParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `job_definition` after provisioning.\n"]
    pub fn job_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.job_definition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.base))
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
    #[doc = "Get a reference to the value of field `array_properties` after provisioning.\n"]
    pub fn array_properties(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElBatchJobParametersElArrayPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.array_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_overrides` after provisioning.\n"]
    pub fn container_overrides(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElBatchJobParametersElContainerOverridesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_overrides", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `depends_on` after provisioning.\n"]
    pub fn depends_on(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElBatchJobParametersElDependsOnElRef> {
        ListRef::new(self.shared().clone(), format!("{}.depends_on", self.base))
    }
    #[doc = "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElBatchJobParametersElRetryStrategyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_strategy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElCloudwatchLogsParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    log_stream_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<PrimField<String>>,
}
impl PipesPipeTargetParametersElCloudwatchLogsParametersEl {
    #[doc = "Set the field `log_stream_name`.\n"]
    pub fn set_log_stream_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_stream_name = Some(v.into());
        self
    }
    #[doc = "Set the field `timestamp`.\n"]
    pub fn set_timestamp(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timestamp = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersElCloudwatchLogsParametersEl {
    type O = BlockAssignable<PipesPipeTargetParametersElCloudwatchLogsParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElCloudwatchLogsParametersEl {}
impl BuildPipesPipeTargetParametersElCloudwatchLogsParametersEl {
    pub fn build(self) -> PipesPipeTargetParametersElCloudwatchLogsParametersEl {
        PipesPipeTargetParametersElCloudwatchLogsParametersEl {
            log_stream_name: core::default::Default::default(),
            timestamp: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElCloudwatchLogsParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElCloudwatchLogsParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElCloudwatchLogsParametersElRef {
        PipesPipeTargetParametersElCloudwatchLogsParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElCloudwatchLogsParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `log_stream_name` after provisioning.\n"]
    pub fn log_stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_stream_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timestamp` after provisioning.\n"]
    pub fn timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timestamp", self.base))
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElEcsTaskParametersElCapacityProviderStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<PrimField<f64>>,
    capacity_provider: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}
impl PipesPipeTargetParametersElEcsTaskParametersElCapacityProviderStrategyEl {
    #[doc = "Set the field `base`.\n"]
    pub fn set_base(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.base = Some(v.into());
        self
    }
    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersElEcsTaskParametersElCapacityProviderStrategyEl {
    type O =
        BlockAssignable<PipesPipeTargetParametersElEcsTaskParametersElCapacityProviderStrategyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElEcsTaskParametersElCapacityProviderStrategyEl {
    #[doc = ""]
    pub capacity_provider: PrimField<String>,
}
impl BuildPipesPipeTargetParametersElEcsTaskParametersElCapacityProviderStrategyEl {
    pub fn build(self) -> PipesPipeTargetParametersElEcsTaskParametersElCapacityProviderStrategyEl {
        PipesPipeTargetParametersElEcsTaskParametersElCapacityProviderStrategyEl {
            base: core::default::Default::default(),
            capacity_provider: self.capacity_provider,
            weight: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElEcsTaskParametersElCapacityProviderStrategyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElEcsTaskParametersElCapacityProviderStrategyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElEcsTaskParametersElCapacityProviderStrategyElRef {
        PipesPipeTargetParametersElEcsTaskParametersElCapacityProviderStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElEcsTaskParametersElCapacityProviderStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `base` after provisioning.\n"]
    pub fn base(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }
    #[doc = "Get a reference to the value of field `capacity_provider` after provisioning.\n"]
    pub fn capacity_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_provider", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElAwsVpcConfigurationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    assign_public_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnets: Option<SetField<PrimField<String>>>,
}
impl PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElAwsVpcConfigurationEl {
    #[doc = "Set the field `assign_public_ip`.\n"]
    pub fn set_assign_public_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.assign_public_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `subnets`.\n"]
    pub fn set_subnets(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnets = Some(v.into());
        self
    }
}
impl ToListMappable
    for PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElAwsVpcConfigurationEl
{
    type O = BlockAssignable<
        PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElAwsVpcConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElAwsVpcConfigurationEl
{}
impl
    BuildPipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElAwsVpcConfigurationEl
{
    pub fn build(
        self,
    ) -> PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElAwsVpcConfigurationEl
    {
        PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElAwsVpcConfigurationEl {
            assign_public_ip: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnets: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElAwsVpcConfigurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElAwsVpcConfigurationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElAwsVpcConfigurationElRef
    {
        PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElAwsVpcConfigurationElRef { shared : shared , base : base . to_string () , }
    }
}
impl PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElAwsVpcConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `assign_public_ip` after provisioning.\n"]
    pub fn assign_public_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assign_public_ip", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}
#[derive(Serialize, Default)]
struct PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElDynamic { aws_vpc_configuration : Option < DynamicBlock < PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElAwsVpcConfigurationEl >> , }
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationEl { # [serde (skip_serializing_if = "Option::is_none")] aws_vpc_configuration : Option < Vec < PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElAwsVpcConfigurationEl > > , dynamic : PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElDynamic , }
impl PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationEl {
    #[doc = "Set the field `aws_vpc_configuration`.\n"]
    pub fn set_aws_vpc_configuration(
        mut self,
        v : impl Into < BlockAssignable < PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElAwsVpcConfigurationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_vpc_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_vpc_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationEl {
    type O = BlockAssignable<PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationEl {}
impl BuildPipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationEl {
    pub fn build(self) -> PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationEl {
        PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationEl {
            aws_vpc_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElRef {
        PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aws_vpc_configuration` after provisioning.\n"]    pub fn aws_vpc_configuration (& self) -> ListRef < PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElAwsVpcConfigurationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.aws_vpc_configuration", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentEl
{
    type O = BlockAssignable<
        PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentEl
{}
impl
    BuildPipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentEl
{
    pub fn build(
        self,
    ) -> PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentEl
    {
        PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentElRef
    {
        PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentElRef { shared : shared , base : base . to_string () , }
    }
}
impl PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentFileEl
{
    #[serde(rename = "type")]
    type_: PrimField<String>,
    value: PrimField<String>,
}
impl PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentFileEl {}
impl ToListMappable for PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentFileEl { type O = BlockAssignable < PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentFileEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildPipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentFileEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildPipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentFileEl { pub fn build (self) -> PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentFileEl { PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentFileEl { type_ : self . type_ , value : self . value , } } }
pub struct PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentFileElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentFileElRef { fn new (shared : StackShared , base : String) -> PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentFileElRef { PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentFileElRef { shared : shared , base : base . to_string () , } } }
impl
    PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentFileElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElResourceRequirementEl
{
    #[serde(rename = "type")]
    type_: PrimField<String>,
    value: PrimField<String>,
}
impl PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElResourceRequirementEl { }
impl ToListMappable for PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElResourceRequirementEl { type O = BlockAssignable < PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElResourceRequirementEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildPipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElResourceRequirementEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildPipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElResourceRequirementEl { pub fn build (self) -> PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElResourceRequirementEl { PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElResourceRequirementEl { type_ : self . type_ , value : self . value , } } }
pub struct PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElResourceRequirementElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElResourceRequirementElRef { fn new (shared : StackShared , base : String) -> PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElResourceRequirementElRef { PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElResourceRequirementElRef { shared : shared , base : base . to_string () , } } }
impl PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElResourceRequirementElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `type_` after provisioning.\n"] pub fn type_ (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.type" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize, Default)]
struct PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElDynamic { environment : Option < DynamicBlock < PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentEl >> , environment_file : Option < DynamicBlock < PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentFileEl >> , resource_requirement : Option < DynamicBlock < PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElResourceRequirementEl >> , }
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideEl { # [serde (skip_serializing_if = "Option::is_none")] command : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] cpu : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] memory : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] memory_reservation : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] environment : Option < Vec < PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentEl > > , # [serde (skip_serializing_if = "Option::is_none")] environment_file : Option < Vec < PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentFileEl > > , # [serde (skip_serializing_if = "Option::is_none")] resource_requirement : Option < Vec < PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElResourceRequirementEl > > , dynamic : PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElDynamic , }
impl PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideEl {
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu = Some(v.into());
        self
    }
    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_reservation`.\n"]
    pub fn set_memory_reservation(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_reservation = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v : impl Into < BlockAssignable < PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.environment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.environment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `environment_file`.\n"]
    pub fn set_environment_file(
        mut self,
        v : impl Into < BlockAssignable < PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentFileEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.environment_file = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.environment_file = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_requirement`.\n"]
    pub fn set_resource_requirement(
        mut self,
        v : impl Into < BlockAssignable < PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElResourceRequirementEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_requirement = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_requirement = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideEl
{
    type O = BlockAssignable<
        PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideEl {}
impl BuildPipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideEl {
    pub fn build(
        self,
    ) -> PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideEl {
        PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideEl {
            command: core::default::Default::default(),
            cpu: core::default::Default::default(),
            memory: core::default::Default::default(),
            memory_reservation: core::default::Default::default(),
            name: core::default::Default::default(),
            environment: core::default::Default::default(),
            environment_file: core::default::Default::default(),
            resource_requirement: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElRef {
        PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }
    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
    #[doc = "Get a reference to the value of field `memory_reservation` after provisioning.\n"]
    pub fn memory_reservation(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_reservation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]    pub fn environment (& self) -> ListRef < PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentElRef >{
        ListRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }
    #[doc = "Get a reference to the value of field `environment_file` after provisioning.\n"]    pub fn environment_file (& self) -> ListRef < PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElEnvironmentFileElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.environment_file", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_requirement` after provisioning.\n"]    pub fn resource_requirement (& self) -> ListRef < PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElResourceRequirementElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_requirement", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElEcsTaskParametersElOverridesElEphemeralStorageEl {
    size_in_gib: PrimField<f64>,
}
impl PipesPipeTargetParametersElEcsTaskParametersElOverridesElEphemeralStorageEl {}
impl ToListMappable
    for PipesPipeTargetParametersElEcsTaskParametersElOverridesElEphemeralStorageEl
{
    type O = BlockAssignable<
        PipesPipeTargetParametersElEcsTaskParametersElOverridesElEphemeralStorageEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElEcsTaskParametersElOverridesElEphemeralStorageEl {
    #[doc = ""]
    pub size_in_gib: PrimField<f64>,
}
impl BuildPipesPipeTargetParametersElEcsTaskParametersElOverridesElEphemeralStorageEl {
    pub fn build(
        self,
    ) -> PipesPipeTargetParametersElEcsTaskParametersElOverridesElEphemeralStorageEl {
        PipesPipeTargetParametersElEcsTaskParametersElOverridesElEphemeralStorageEl {
            size_in_gib: self.size_in_gib,
        }
    }
}
pub struct PipesPipeTargetParametersElEcsTaskParametersElOverridesElEphemeralStorageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElEcsTaskParametersElOverridesElEphemeralStorageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElEcsTaskParametersElOverridesElEphemeralStorageElRef {
        PipesPipeTargetParametersElEcsTaskParametersElOverridesElEphemeralStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElEcsTaskParametersElOverridesElEphemeralStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `size_in_gib` after provisioning.\n"]
    pub fn size_in_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_in_gib", self.base))
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElEcsTaskParametersElOverridesElInferenceAcceleratorOverrideEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_type: Option<PrimField<String>>,
}
impl PipesPipeTargetParametersElEcsTaskParametersElOverridesElInferenceAcceleratorOverrideEl {
    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_name = Some(v.into());
        self
    }
    #[doc = "Set the field `device_type`.\n"]
    pub fn set_device_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_type = Some(v.into());
        self
    }
}
impl ToListMappable
    for PipesPipeTargetParametersElEcsTaskParametersElOverridesElInferenceAcceleratorOverrideEl
{
    type O = BlockAssignable<
        PipesPipeTargetParametersElEcsTaskParametersElOverridesElInferenceAcceleratorOverrideEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElEcsTaskParametersElOverridesElInferenceAcceleratorOverrideEl
{}
impl BuildPipesPipeTargetParametersElEcsTaskParametersElOverridesElInferenceAcceleratorOverrideEl {
    pub fn build(
        self,
    ) -> PipesPipeTargetParametersElEcsTaskParametersElOverridesElInferenceAcceleratorOverrideEl
    {
        PipesPipeTargetParametersElEcsTaskParametersElOverridesElInferenceAcceleratorOverrideEl {
            device_name: core::default::Default::default(),
            device_type: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElEcsTaskParametersElOverridesElInferenceAcceleratorOverrideElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for PipesPipeTargetParametersElEcsTaskParametersElOverridesElInferenceAcceleratorOverrideElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElEcsTaskParametersElOverridesElInferenceAcceleratorOverrideElRef
    {
        PipesPipeTargetParametersElEcsTaskParametersElOverridesElInferenceAcceleratorOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElEcsTaskParametersElOverridesElInferenceAcceleratorOverrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }
    #[doc = "Get a reference to the value of field `device_type` after provisioning.\n"]
    pub fn device_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct PipesPipeTargetParametersElEcsTaskParametersElOverridesElDynamic {
    container_override: Option<
        DynamicBlock<PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideEl>,
    >,
    ephemeral_storage: Option<
        DynamicBlock<PipesPipeTargetParametersElEcsTaskParametersElOverridesElEphemeralStorageEl>,
    >,
    inference_accelerator_override: Option<
        DynamicBlock<
            PipesPipeTargetParametersElEcsTaskParametersElOverridesElInferenceAcceleratorOverrideEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElEcsTaskParametersElOverridesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_override:
        Option<Vec<PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ephemeral_storage:
        Option<Vec<PipesPipeTargetParametersElEcsTaskParametersElOverridesElEphemeralStorageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inference_accelerator_override: Option<
        Vec<
            PipesPipeTargetParametersElEcsTaskParametersElOverridesElInferenceAcceleratorOverrideEl,
        >,
    >,
    dynamic: PipesPipeTargetParametersElEcsTaskParametersElOverridesElDynamic,
}
impl PipesPipeTargetParametersElEcsTaskParametersElOverridesEl {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu = Some(v.into());
        self
    }
    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.memory = Some(v.into());
        self
    }
    #[doc = "Set the field `task_role_arn`.\n"]
    pub fn set_task_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.task_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `container_override`.\n"]
    pub fn set_container_override(
        mut self,
        v: impl Into<
            BlockAssignable<
                PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container_override = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container_override = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ephemeral_storage`.\n"]
    pub fn set_ephemeral_storage(
        mut self,
        v: impl Into<
            BlockAssignable<
                PipesPipeTargetParametersElEcsTaskParametersElOverridesElEphemeralStorageEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ephemeral_storage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ephemeral_storage = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `inference_accelerator_override`.\n"]
    pub fn set_inference_accelerator_override(
        mut self,
        v : impl Into < BlockAssignable < PipesPipeTargetParametersElEcsTaskParametersElOverridesElInferenceAcceleratorOverrideEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inference_accelerator_override = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inference_accelerator_override = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersElEcsTaskParametersElOverridesEl {
    type O = BlockAssignable<PipesPipeTargetParametersElEcsTaskParametersElOverridesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElEcsTaskParametersElOverridesEl {}
impl BuildPipesPipeTargetParametersElEcsTaskParametersElOverridesEl {
    pub fn build(self) -> PipesPipeTargetParametersElEcsTaskParametersElOverridesEl {
        PipesPipeTargetParametersElEcsTaskParametersElOverridesEl {
            cpu: core::default::Default::default(),
            execution_role_arn: core::default::Default::default(),
            memory: core::default::Default::default(),
            task_role_arn: core::default::Default::default(),
            container_override: core::default::Default::default(),
            ephemeral_storage: core::default::Default::default(),
            inference_accelerator_override: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElEcsTaskParametersElOverridesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElEcsTaskParametersElOverridesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElEcsTaskParametersElOverridesElRef {
        PipesPipeTargetParametersElEcsTaskParametersElOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElEcsTaskParametersElOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }
    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_role_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
    #[doc = "Get a reference to the value of field `task_role_arn` after provisioning.\n"]
    pub fn task_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_role_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_override` after provisioning.\n"]
    pub fn container_override(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElEcsTaskParametersElOverridesElContainerOverrideElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_override", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElEcsTaskParametersElOverridesElEphemeralStorageElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ephemeral_storage", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inference_accelerator_override` after provisioning.\n"]
    pub fn inference_accelerator_override(
        &self,
    ) -> ListRef<
        PipesPipeTargetParametersElEcsTaskParametersElOverridesElInferenceAcceleratorOverrideElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inference_accelerator_override", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElEcsTaskParametersElPlacementConstraintEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl PipesPipeTargetParametersElEcsTaskParametersElPlacementConstraintEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersElEcsTaskParametersElPlacementConstraintEl {
    type O = BlockAssignable<PipesPipeTargetParametersElEcsTaskParametersElPlacementConstraintEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElEcsTaskParametersElPlacementConstraintEl {}
impl BuildPipesPipeTargetParametersElEcsTaskParametersElPlacementConstraintEl {
    pub fn build(self) -> PipesPipeTargetParametersElEcsTaskParametersElPlacementConstraintEl {
        PipesPipeTargetParametersElEcsTaskParametersElPlacementConstraintEl {
            expression: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElEcsTaskParametersElPlacementConstraintElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElEcsTaskParametersElPlacementConstraintElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElEcsTaskParametersElPlacementConstraintElRef {
        PipesPipeTargetParametersElEcsTaskParametersElPlacementConstraintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElEcsTaskParametersElPlacementConstraintElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElEcsTaskParametersElPlacementStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl PipesPipeTargetParametersElEcsTaskParametersElPlacementStrategyEl {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersElEcsTaskParametersElPlacementStrategyEl {
    type O = BlockAssignable<PipesPipeTargetParametersElEcsTaskParametersElPlacementStrategyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElEcsTaskParametersElPlacementStrategyEl {}
impl BuildPipesPipeTargetParametersElEcsTaskParametersElPlacementStrategyEl {
    pub fn build(self) -> PipesPipeTargetParametersElEcsTaskParametersElPlacementStrategyEl {
        PipesPipeTargetParametersElEcsTaskParametersElPlacementStrategyEl {
            field: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElEcsTaskParametersElPlacementStrategyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElEcsTaskParametersElPlacementStrategyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElEcsTaskParametersElPlacementStrategyElRef {
        PipesPipeTargetParametersElEcsTaskParametersElPlacementStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElEcsTaskParametersElPlacementStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct PipesPipeTargetParametersElEcsTaskParametersElDynamic {
    capacity_provider_strategy: Option<
        DynamicBlock<PipesPipeTargetParametersElEcsTaskParametersElCapacityProviderStrategyEl>,
    >,
    network_configuration:
        Option<DynamicBlock<PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationEl>>,
    overrides: Option<DynamicBlock<PipesPipeTargetParametersElEcsTaskParametersElOverridesEl>>,
    placement_constraint:
        Option<DynamicBlock<PipesPipeTargetParametersElEcsTaskParametersElPlacementConstraintEl>>,
    placement_strategy:
        Option<DynamicBlock<PipesPipeTargetParametersElEcsTaskParametersElPlacementStrategyEl>>,
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElEcsTaskParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ecs_managed_tags: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_execute_command: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    propagate_tags: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_count: Option<PrimField<f64>>,
    task_definition_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_provider_strategy:
        Option<Vec<PipesPipeTargetParametersElEcsTaskParametersElCapacityProviderStrategyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_configuration:
        Option<Vec<PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overrides: Option<Vec<PipesPipeTargetParametersElEcsTaskParametersElOverridesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_constraint:
        Option<Vec<PipesPipeTargetParametersElEcsTaskParametersElPlacementConstraintEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_strategy:
        Option<Vec<PipesPipeTargetParametersElEcsTaskParametersElPlacementStrategyEl>>,
    dynamic: PipesPipeTargetParametersElEcsTaskParametersElDynamic,
}
impl PipesPipeTargetParametersElEcsTaskParametersEl {
    #[doc = "Set the field `enable_ecs_managed_tags`.\n"]
    pub fn set_enable_ecs_managed_tags(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_ecs_managed_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_execute_command`.\n"]
    pub fn set_enable_execute_command(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_execute_command = Some(v.into());
        self
    }
    #[doc = "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group = Some(v.into());
        self
    }
    #[doc = "Set the field `launch_type`.\n"]
    pub fn set_launch_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.launch_type = Some(v.into());
        self
    }
    #[doc = "Set the field `platform_version`.\n"]
    pub fn set_platform_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.platform_version = Some(v.into());
        self
    }
    #[doc = "Set the field `propagate_tags`.\n"]
    pub fn set_propagate_tags(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.propagate_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `reference_id`.\n"]
    pub fn set_reference_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reference_id = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `task_count`.\n"]
    pub fn set_task_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.task_count = Some(v.into());
        self
    }
    #[doc = "Set the field `capacity_provider_strategy`.\n"]
    pub fn set_capacity_provider_strategy(
        mut self,
        v: impl Into<
            BlockAssignable<
                PipesPipeTargetParametersElEcsTaskParametersElCapacityProviderStrategyEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.capacity_provider_strategy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity_provider_strategy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `overrides`.\n"]
    pub fn set_overrides(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeTargetParametersElEcsTaskParametersElOverridesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.overrides = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.overrides = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `placement_constraint`.\n"]
    pub fn set_placement_constraint(
        mut self,
        v: impl Into<
            BlockAssignable<PipesPipeTargetParametersElEcsTaskParametersElPlacementConstraintEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.placement_constraint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.placement_constraint = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `placement_strategy`.\n"]
    pub fn set_placement_strategy(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeTargetParametersElEcsTaskParametersElPlacementStrategyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.placement_strategy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.placement_strategy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersElEcsTaskParametersEl {
    type O = BlockAssignable<PipesPipeTargetParametersElEcsTaskParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElEcsTaskParametersEl {
    #[doc = ""]
    pub task_definition_arn: PrimField<String>,
}
impl BuildPipesPipeTargetParametersElEcsTaskParametersEl {
    pub fn build(self) -> PipesPipeTargetParametersElEcsTaskParametersEl {
        PipesPipeTargetParametersElEcsTaskParametersEl {
            enable_ecs_managed_tags: core::default::Default::default(),
            enable_execute_command: core::default::Default::default(),
            group: core::default::Default::default(),
            launch_type: core::default::Default::default(),
            platform_version: core::default::Default::default(),
            propagate_tags: core::default::Default::default(),
            reference_id: core::default::Default::default(),
            tags: core::default::Default::default(),
            task_count: core::default::Default::default(),
            task_definition_arn: self.task_definition_arn,
            capacity_provider_strategy: core::default::Default::default(),
            network_configuration: core::default::Default::default(),
            overrides: core::default::Default::default(),
            placement_constraint: core::default::Default::default(),
            placement_strategy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElEcsTaskParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElEcsTaskParametersElRef {
    fn new(shared: StackShared, base: String) -> PipesPipeTargetParametersElEcsTaskParametersElRef {
        PipesPipeTargetParametersElEcsTaskParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElEcsTaskParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_ecs_managed_tags` after provisioning.\n"]
    pub fn enable_ecs_managed_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_ecs_managed_tags", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_execute_command` after provisioning.\n"]
    pub fn enable_execute_command(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_execute_command", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }
    #[doc = "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.launch_type", self.base))
    }
    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.platform_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.propagate_tags", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `reference_id` after provisioning.\n"]
    pub fn reference_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reference_id", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `task_count` after provisioning.\n"]
    pub fn task_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_count", self.base))
    }
    #[doc = "Get a reference to the value of field `task_definition_arn` after provisioning.\n"]
    pub fn task_definition_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.task_definition_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `capacity_provider_strategy` after provisioning.\n"]
    pub fn capacity_provider_strategy(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElEcsTaskParametersElCapacityProviderStrategyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_provider_strategy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElEcsTaskParametersElNetworkConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `overrides` after provisioning.\n"]
    pub fn overrides(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElEcsTaskParametersElOverridesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.overrides", self.base))
    }
    #[doc = "Get a reference to the value of field `placement_constraint` after provisioning.\n"]
    pub fn placement_constraint(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElEcsTaskParametersElPlacementConstraintElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.placement_constraint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `placement_strategy` after provisioning.\n"]
    pub fn placement_strategy(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElEcsTaskParametersElPlacementStrategyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.placement_strategy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElEventbridgeEventBusParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    detail_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<PrimField<String>>,
}
impl PipesPipeTargetParametersElEventbridgeEventBusParametersEl {
    #[doc = "Set the field `detail_type`.\n"]
    pub fn set_detail_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.detail_type = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoint_id`.\n"]
    pub fn set_endpoint_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint_id = Some(v.into());
        self
    }
    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }
    #[doc = "Set the field `time`.\n"]
    pub fn set_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersElEventbridgeEventBusParametersEl {
    type O = BlockAssignable<PipesPipeTargetParametersElEventbridgeEventBusParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElEventbridgeEventBusParametersEl {}
impl BuildPipesPipeTargetParametersElEventbridgeEventBusParametersEl {
    pub fn build(self) -> PipesPipeTargetParametersElEventbridgeEventBusParametersEl {
        PipesPipeTargetParametersElEventbridgeEventBusParametersEl {
            detail_type: core::default::Default::default(),
            endpoint_id: core::default::Default::default(),
            resources: core::default::Default::default(),
            source: core::default::Default::default(),
            time: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElEventbridgeEventBusParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElEventbridgeEventBusParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElEventbridgeEventBusParametersElRef {
        PipesPipeTargetParametersElEventbridgeEventBusParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElEventbridgeEventBusParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `detail_type` after provisioning.\n"]
    pub fn detail_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.detail_type", self.base))
    }
    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.base))
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
    #[doc = "Get a reference to the value of field `time` after provisioning.\n"]
    pub fn time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time", self.base))
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElHttpParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_parameter_values: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string_parameters: Option<RecField<PrimField<String>>>,
}
impl PipesPipeTargetParametersElHttpParametersEl {
    #[doc = "Set the field `header_parameters`.\n"]
    pub fn set_header_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.header_parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `path_parameter_values`.\n"]
    pub fn set_path_parameter_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.path_parameter_values = Some(v.into());
        self
    }
    #[doc = "Set the field `query_string_parameters`.\n"]
    pub fn set_query_string_parameters(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.query_string_parameters = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersElHttpParametersEl {
    type O = BlockAssignable<PipesPipeTargetParametersElHttpParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElHttpParametersEl {}
impl BuildPipesPipeTargetParametersElHttpParametersEl {
    pub fn build(self) -> PipesPipeTargetParametersElHttpParametersEl {
        PipesPipeTargetParametersElHttpParametersEl {
            header_parameters: core::default::Default::default(),
            path_parameter_values: core::default::Default::default(),
            query_string_parameters: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElHttpParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElHttpParametersElRef {
    fn new(shared: StackShared, base: String) -> PipesPipeTargetParametersElHttpParametersElRef {
        PipesPipeTargetParametersElHttpParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElHttpParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_parameters` after provisioning.\n"]
    pub fn header_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.header_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path_parameter_values` after provisioning.\n"]
    pub fn path_parameter_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.path_parameter_values", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_string_parameters` after provisioning.\n"]
    pub fn query_string_parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.query_string_parameters", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElKinesisStreamParametersEl {
    partition_key: PrimField<String>,
}
impl PipesPipeTargetParametersElKinesisStreamParametersEl {}
impl ToListMappable for PipesPipeTargetParametersElKinesisStreamParametersEl {
    type O = BlockAssignable<PipesPipeTargetParametersElKinesisStreamParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElKinesisStreamParametersEl {
    #[doc = ""]
    pub partition_key: PrimField<String>,
}
impl BuildPipesPipeTargetParametersElKinesisStreamParametersEl {
    pub fn build(self) -> PipesPipeTargetParametersElKinesisStreamParametersEl {
        PipesPipeTargetParametersElKinesisStreamParametersEl {
            partition_key: self.partition_key,
        }
    }
}
pub struct PipesPipeTargetParametersElKinesisStreamParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElKinesisStreamParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElKinesisStreamParametersElRef {
        PipesPipeTargetParametersElKinesisStreamParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElKinesisStreamParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `partition_key` after provisioning.\n"]
    pub fn partition_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.partition_key", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElLambdaFunctionParametersEl {
    invocation_type: PrimField<String>,
}
impl PipesPipeTargetParametersElLambdaFunctionParametersEl {}
impl ToListMappable for PipesPipeTargetParametersElLambdaFunctionParametersEl {
    type O = BlockAssignable<PipesPipeTargetParametersElLambdaFunctionParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElLambdaFunctionParametersEl {
    #[doc = ""]
    pub invocation_type: PrimField<String>,
}
impl BuildPipesPipeTargetParametersElLambdaFunctionParametersEl {
    pub fn build(self) -> PipesPipeTargetParametersElLambdaFunctionParametersEl {
        PipesPipeTargetParametersElLambdaFunctionParametersEl {
            invocation_type: self.invocation_type,
        }
    }
}
pub struct PipesPipeTargetParametersElLambdaFunctionParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElLambdaFunctionParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElLambdaFunctionParametersElRef {
        PipesPipeTargetParametersElLambdaFunctionParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElLambdaFunctionParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `invocation_type` after provisioning.\n"]
    pub fn invocation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.invocation_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElRedshiftDataParametersEl {
    database: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_user: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_manager_arn: Option<PrimField<String>>,
    sqls: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_event: Option<PrimField<bool>>,
}
impl PipesPipeTargetParametersElRedshiftDataParametersEl {
    #[doc = "Set the field `db_user`.\n"]
    pub fn set_db_user(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_user = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_manager_arn`.\n"]
    pub fn set_secret_manager_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_manager_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `statement_name`.\n"]
    pub fn set_statement_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.statement_name = Some(v.into());
        self
    }
    #[doc = "Set the field `with_event`.\n"]
    pub fn set_with_event(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.with_event = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersElRedshiftDataParametersEl {
    type O = BlockAssignable<PipesPipeTargetParametersElRedshiftDataParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElRedshiftDataParametersEl {
    #[doc = ""]
    pub database: PrimField<String>,
    #[doc = ""]
    pub sqls: SetField<PrimField<String>>,
}
impl BuildPipesPipeTargetParametersElRedshiftDataParametersEl {
    pub fn build(self) -> PipesPipeTargetParametersElRedshiftDataParametersEl {
        PipesPipeTargetParametersElRedshiftDataParametersEl {
            database: self.database,
            db_user: core::default::Default::default(),
            secret_manager_arn: core::default::Default::default(),
            sqls: self.sqls,
            statement_name: core::default::Default::default(),
            with_event: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElRedshiftDataParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElRedshiftDataParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElRedshiftDataParametersElRef {
        PipesPipeTargetParametersElRedshiftDataParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElRedshiftDataParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `db_user` after provisioning.\n"]
    pub fn db_user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_user", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_manager_arn` after provisioning.\n"]
    pub fn secret_manager_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_manager_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sqls` after provisioning.\n"]
    pub fn sqls(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.sqls", self.base))
    }
    #[doc = "Get a reference to the value of field `statement_name` after provisioning.\n"]
    pub fn statement_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.statement_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `with_event` after provisioning.\n"]
    pub fn with_event(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.with_event", self.base))
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElSagemakerPipelineParametersElPipelineParameterEl {
    name: PrimField<String>,
    value: PrimField<String>,
}
impl PipesPipeTargetParametersElSagemakerPipelineParametersElPipelineParameterEl {}
impl ToListMappable
    for PipesPipeTargetParametersElSagemakerPipelineParametersElPipelineParameterEl
{
    type O = BlockAssignable<
        PipesPipeTargetParametersElSagemakerPipelineParametersElPipelineParameterEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElSagemakerPipelineParametersElPipelineParameterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildPipesPipeTargetParametersElSagemakerPipelineParametersElPipelineParameterEl {
    pub fn build(
        self,
    ) -> PipesPipeTargetParametersElSagemakerPipelineParametersElPipelineParameterEl {
        PipesPipeTargetParametersElSagemakerPipelineParametersElPipelineParameterEl {
            name: self.name,
            value: self.value,
        }
    }
}
pub struct PipesPipeTargetParametersElSagemakerPipelineParametersElPipelineParameterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElSagemakerPipelineParametersElPipelineParameterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElSagemakerPipelineParametersElPipelineParameterElRef {
        PipesPipeTargetParametersElSagemakerPipelineParametersElPipelineParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElSagemakerPipelineParametersElPipelineParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct PipesPipeTargetParametersElSagemakerPipelineParametersElDynamic {
    pipeline_parameter: Option<
        DynamicBlock<PipesPipeTargetParametersElSagemakerPipelineParametersElPipelineParameterEl>,
    >,
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElSagemakerPipelineParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pipeline_parameter:
        Option<Vec<PipesPipeTargetParametersElSagemakerPipelineParametersElPipelineParameterEl>>,
    dynamic: PipesPipeTargetParametersElSagemakerPipelineParametersElDynamic,
}
impl PipesPipeTargetParametersElSagemakerPipelineParametersEl {
    #[doc = "Set the field `pipeline_parameter`.\n"]
    pub fn set_pipeline_parameter(
        mut self,
        v: impl Into<
            BlockAssignable<
                PipesPipeTargetParametersElSagemakerPipelineParametersElPipelineParameterEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pipeline_parameter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pipeline_parameter = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersElSagemakerPipelineParametersEl {
    type O = BlockAssignable<PipesPipeTargetParametersElSagemakerPipelineParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElSagemakerPipelineParametersEl {}
impl BuildPipesPipeTargetParametersElSagemakerPipelineParametersEl {
    pub fn build(self) -> PipesPipeTargetParametersElSagemakerPipelineParametersEl {
        PipesPipeTargetParametersElSagemakerPipelineParametersEl {
            pipeline_parameter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElSagemakerPipelineParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElSagemakerPipelineParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElSagemakerPipelineParametersElRef {
        PipesPipeTargetParametersElSagemakerPipelineParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElSagemakerPipelineParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pipeline_parameter` after provisioning.\n"]
    pub fn pipeline_parameter(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElSagemakerPipelineParametersElPipelineParameterElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pipeline_parameter", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElSqsQueueParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_deduplication_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_group_id: Option<PrimField<String>>,
}
impl PipesPipeTargetParametersElSqsQueueParametersEl {
    #[doc = "Set the field `message_deduplication_id`.\n"]
    pub fn set_message_deduplication_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_deduplication_id = Some(v.into());
        self
    }
    #[doc = "Set the field `message_group_id`.\n"]
    pub fn set_message_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_group_id = Some(v.into());
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersElSqsQueueParametersEl {
    type O = BlockAssignable<PipesPipeTargetParametersElSqsQueueParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElSqsQueueParametersEl {}
impl BuildPipesPipeTargetParametersElSqsQueueParametersEl {
    pub fn build(self) -> PipesPipeTargetParametersElSqsQueueParametersEl {
        PipesPipeTargetParametersElSqsQueueParametersEl {
            message_deduplication_id: core::default::Default::default(),
            message_group_id: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElSqsQueueParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElSqsQueueParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElSqsQueueParametersElRef {
        PipesPipeTargetParametersElSqsQueueParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElSqsQueueParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message_deduplication_id` after provisioning.\n"]
    pub fn message_deduplication_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_deduplication_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message_group_id` after provisioning.\n"]
    pub fn message_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_group_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersElStepFunctionStateMachineParametersEl {
    invocation_type: PrimField<String>,
}
impl PipesPipeTargetParametersElStepFunctionStateMachineParametersEl {}
impl ToListMappable for PipesPipeTargetParametersElStepFunctionStateMachineParametersEl {
    type O = BlockAssignable<PipesPipeTargetParametersElStepFunctionStateMachineParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersElStepFunctionStateMachineParametersEl {
    #[doc = ""]
    pub invocation_type: PrimField<String>,
}
impl BuildPipesPipeTargetParametersElStepFunctionStateMachineParametersEl {
    pub fn build(self) -> PipesPipeTargetParametersElStepFunctionStateMachineParametersEl {
        PipesPipeTargetParametersElStepFunctionStateMachineParametersEl {
            invocation_type: self.invocation_type,
        }
    }
}
pub struct PipesPipeTargetParametersElStepFunctionStateMachineParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElStepFunctionStateMachineParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> PipesPipeTargetParametersElStepFunctionStateMachineParametersElRef {
        PipesPipeTargetParametersElStepFunctionStateMachineParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElStepFunctionStateMachineParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `invocation_type` after provisioning.\n"]
    pub fn invocation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.invocation_type", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct PipesPipeTargetParametersElDynamic {
    batch_job_parameters: Option<DynamicBlock<PipesPipeTargetParametersElBatchJobParametersEl>>,
    cloudwatch_logs_parameters:
        Option<DynamicBlock<PipesPipeTargetParametersElCloudwatchLogsParametersEl>>,
    ecs_task_parameters: Option<DynamicBlock<PipesPipeTargetParametersElEcsTaskParametersEl>>,
    eventbridge_event_bus_parameters:
        Option<DynamicBlock<PipesPipeTargetParametersElEventbridgeEventBusParametersEl>>,
    http_parameters: Option<DynamicBlock<PipesPipeTargetParametersElHttpParametersEl>>,
    kinesis_stream_parameters:
        Option<DynamicBlock<PipesPipeTargetParametersElKinesisStreamParametersEl>>,
    lambda_function_parameters:
        Option<DynamicBlock<PipesPipeTargetParametersElLambdaFunctionParametersEl>>,
    redshift_data_parameters:
        Option<DynamicBlock<PipesPipeTargetParametersElRedshiftDataParametersEl>>,
    sagemaker_pipeline_parameters:
        Option<DynamicBlock<PipesPipeTargetParametersElSagemakerPipelineParametersEl>>,
    sqs_queue_parameters: Option<DynamicBlock<PipesPipeTargetParametersElSqsQueueParametersEl>>,
    step_function_state_machine_parameters:
        Option<DynamicBlock<PipesPipeTargetParametersElStepFunctionStateMachineParametersEl>>,
}
#[derive(Serialize)]
pub struct PipesPipeTargetParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_job_parameters: Option<Vec<PipesPipeTargetParametersElBatchJobParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logs_parameters: Option<Vec<PipesPipeTargetParametersElCloudwatchLogsParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecs_task_parameters: Option<Vec<PipesPipeTargetParametersElEcsTaskParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eventbridge_event_bus_parameters:
        Option<Vec<PipesPipeTargetParametersElEventbridgeEventBusParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_parameters: Option<Vec<PipesPipeTargetParametersElHttpParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_stream_parameters: Option<Vec<PipesPipeTargetParametersElKinesisStreamParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_function_parameters: Option<Vec<PipesPipeTargetParametersElLambdaFunctionParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redshift_data_parameters: Option<Vec<PipesPipeTargetParametersElRedshiftDataParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sagemaker_pipeline_parameters:
        Option<Vec<PipesPipeTargetParametersElSagemakerPipelineParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sqs_queue_parameters: Option<Vec<PipesPipeTargetParametersElSqsQueueParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    step_function_state_machine_parameters:
        Option<Vec<PipesPipeTargetParametersElStepFunctionStateMachineParametersEl>>,
    dynamic: PipesPipeTargetParametersElDynamic,
}
impl PipesPipeTargetParametersEl {
    #[doc = "Set the field `input_template`.\n"]
    pub fn set_input_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_template = Some(v.into());
        self
    }
    #[doc = "Set the field `batch_job_parameters`.\n"]
    pub fn set_batch_job_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeTargetParametersElBatchJobParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.batch_job_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.batch_job_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cloudwatch_logs_parameters`.\n"]
    pub fn set_cloudwatch_logs_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeTargetParametersElCloudwatchLogsParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logs_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ecs_task_parameters`.\n"]
    pub fn set_ecs_task_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeTargetParametersElEcsTaskParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ecs_task_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ecs_task_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `eventbridge_event_bus_parameters`.\n"]
    pub fn set_eventbridge_event_bus_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeTargetParametersElEventbridgeEventBusParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.eventbridge_event_bus_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.eventbridge_event_bus_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `http_parameters`.\n"]
    pub fn set_http_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeTargetParametersElHttpParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kinesis_stream_parameters`.\n"]
    pub fn set_kinesis_stream_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeTargetParametersElKinesisStreamParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_stream_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_stream_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lambda_function_parameters`.\n"]
    pub fn set_lambda_function_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeTargetParametersElLambdaFunctionParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda_function_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `redshift_data_parameters`.\n"]
    pub fn set_redshift_data_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeTargetParametersElRedshiftDataParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redshift_data_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redshift_data_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sagemaker_pipeline_parameters`.\n"]
    pub fn set_sagemaker_pipeline_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeTargetParametersElSagemakerPipelineParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sagemaker_pipeline_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sagemaker_pipeline_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sqs_queue_parameters`.\n"]
    pub fn set_sqs_queue_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeTargetParametersElSqsQueueParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sqs_queue_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sqs_queue_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `step_function_state_machine_parameters`.\n"]
    pub fn set_step_function_state_machine_parameters(
        mut self,
        v: impl Into<BlockAssignable<PipesPipeTargetParametersElStepFunctionStateMachineParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.step_function_state_machine_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.step_function_state_machine_parameters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for PipesPipeTargetParametersEl {
    type O = BlockAssignable<PipesPipeTargetParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTargetParametersEl {}
impl BuildPipesPipeTargetParametersEl {
    pub fn build(self) -> PipesPipeTargetParametersEl {
        PipesPipeTargetParametersEl {
            input_template: core::default::Default::default(),
            batch_job_parameters: core::default::Default::default(),
            cloudwatch_logs_parameters: core::default::Default::default(),
            ecs_task_parameters: core::default::Default::default(),
            eventbridge_event_bus_parameters: core::default::Default::default(),
            http_parameters: core::default::Default::default(),
            kinesis_stream_parameters: core::default::Default::default(),
            lambda_function_parameters: core::default::Default::default(),
            redshift_data_parameters: core::default::Default::default(),
            sagemaker_pipeline_parameters: core::default::Default::default(),
            sqs_queue_parameters: core::default::Default::default(),
            step_function_state_machine_parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct PipesPipeTargetParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTargetParametersElRef {
    fn new(shared: StackShared, base: String) -> PipesPipeTargetParametersElRef {
        PipesPipeTargetParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTargetParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `input_template` after provisioning.\n"]
    pub fn input_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_template", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `batch_job_parameters` after provisioning.\n"]
    pub fn batch_job_parameters(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElBatchJobParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.batch_job_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloudwatch_logs_parameters` after provisioning.\n"]
    pub fn cloudwatch_logs_parameters(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElCloudwatchLogsParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_logs_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ecs_task_parameters` after provisioning.\n"]
    pub fn ecs_task_parameters(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElEcsTaskParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ecs_task_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `eventbridge_event_bus_parameters` after provisioning.\n"]
    pub fn eventbridge_event_bus_parameters(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElEventbridgeEventBusParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.eventbridge_event_bus_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `http_parameters` after provisioning.\n"]
    pub fn http_parameters(&self) -> ListRef<PipesPipeTargetParametersElHttpParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.http_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `kinesis_stream_parameters` after provisioning.\n"]
    pub fn kinesis_stream_parameters(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElKinesisStreamParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_stream_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lambda_function_parameters` after provisioning.\n"]
    pub fn lambda_function_parameters(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElLambdaFunctionParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lambda_function_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `redshift_data_parameters` after provisioning.\n"]
    pub fn redshift_data_parameters(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElRedshiftDataParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redshift_data_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sagemaker_pipeline_parameters` after provisioning.\n"]
    pub fn sagemaker_pipeline_parameters(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElSagemakerPipelineParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sagemaker_pipeline_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sqs_queue_parameters` after provisioning.\n"]
    pub fn sqs_queue_parameters(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElSqsQueueParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sqs_queue_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `step_function_state_machine_parameters` after provisioning.\n"]
    pub fn step_function_state_machine_parameters(
        &self,
    ) -> ListRef<PipesPipeTargetParametersElStepFunctionStateMachineParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.step_function_state_machine_parameters", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct PipesPipeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl PipesPipeTimeoutsEl {
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
impl ToListMappable for PipesPipeTimeoutsEl {
    type O = BlockAssignable<PipesPipeTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPipesPipeTimeoutsEl {}
impl BuildPipesPipeTimeoutsEl {
    pub fn build(self) -> PipesPipeTimeoutsEl {
        PipesPipeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct PipesPipeTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for PipesPipeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> PipesPipeTimeoutsElRef {
        PipesPipeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl PipesPipeTimeoutsElRef {
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
struct PipesPipeDynamic {
    enrichment_parameters: Option<DynamicBlock<PipesPipeEnrichmentParametersEl>>,
    log_configuration: Option<DynamicBlock<PipesPipeLogConfigurationEl>>,
    source_parameters: Option<DynamicBlock<PipesPipeSourceParametersEl>>,
    target_parameters: Option<DynamicBlock<PipesPipeTargetParametersEl>>,
}
