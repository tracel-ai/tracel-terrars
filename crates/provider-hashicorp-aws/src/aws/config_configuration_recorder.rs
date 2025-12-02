use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ConfigConfigurationRecorderData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recording_group: Option<Vec<ConfigConfigurationRecorderRecordingGroupEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recording_mode: Option<Vec<ConfigConfigurationRecorderRecordingModeEl>>,
    dynamic: ConfigConfigurationRecorderDynamic,
}
struct ConfigConfigurationRecorder_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConfigConfigurationRecorderData>,
}
#[derive(Clone)]
pub struct ConfigConfigurationRecorder(Rc<ConfigConfigurationRecorder_>);
impl ConfigConfigurationRecorder {
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
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `recording_group`.\n"]
    pub fn set_recording_group(
        self,
        v: impl Into<BlockAssignable<ConfigConfigurationRecorderRecordingGroupEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recording_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recording_group = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `recording_mode`.\n"]
    pub fn set_recording_mode(
        self,
        v: impl Into<BlockAssignable<ConfigConfigurationRecorderRecordingModeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recording_mode = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recording_mode = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `recording_group` after provisioning.\n"]
    pub fn recording_group(&self) -> ListRef<ConfigConfigurationRecorderRecordingGroupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recording_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `recording_mode` after provisioning.\n"]
    pub fn recording_mode(&self) -> ListRef<ConfigConfigurationRecorderRecordingModeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recording_mode", self.extract_ref()),
        )
    }
}
impl Referable for ConfigConfigurationRecorder {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ConfigConfigurationRecorder {}
impl ToListMappable for ConfigConfigurationRecorder {
    type O = ListRef<ConfigConfigurationRecorderRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ConfigConfigurationRecorder_ {
    fn extract_resource_type(&self) -> String {
        "aws_config_configuration_recorder".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildConfigConfigurationRecorder {
    pub tf_id: String,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}
impl BuildConfigConfigurationRecorder {
    pub fn build(self, stack: &mut Stack) -> ConfigConfigurationRecorder {
        let out = ConfigConfigurationRecorder(Rc::new(ConfigConfigurationRecorder_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConfigConfigurationRecorderData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                recording_group: core::default::Default::default(),
                recording_mode: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ConfigConfigurationRecorderRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigConfigurationRecorderRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ConfigConfigurationRecorderRef {
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
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `recording_group` after provisioning.\n"]
    pub fn recording_group(&self) -> ListRef<ConfigConfigurationRecorderRecordingGroupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recording_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `recording_mode` after provisioning.\n"]
    pub fn recording_mode(&self) -> ListRef<ConfigConfigurationRecorderRecordingModeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recording_mode", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ConfigConfigurationRecorderRecordingGroupElExclusionByResourceTypesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_types: Option<SetField<PrimField<String>>>,
}
impl ConfigConfigurationRecorderRecordingGroupElExclusionByResourceTypesEl {
    #[doc = "Set the field `resource_types`.\n"]
    pub fn set_resource_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resource_types = Some(v.into());
        self
    }
}
impl ToListMappable for ConfigConfigurationRecorderRecordingGroupElExclusionByResourceTypesEl {
    type O = BlockAssignable<ConfigConfigurationRecorderRecordingGroupElExclusionByResourceTypesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConfigConfigurationRecorderRecordingGroupElExclusionByResourceTypesEl {}
impl BuildConfigConfigurationRecorderRecordingGroupElExclusionByResourceTypesEl {
    pub fn build(self) -> ConfigConfigurationRecorderRecordingGroupElExclusionByResourceTypesEl {
        ConfigConfigurationRecorderRecordingGroupElExclusionByResourceTypesEl {
            resource_types: core::default::Default::default(),
        }
    }
}
pub struct ConfigConfigurationRecorderRecordingGroupElExclusionByResourceTypesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigConfigurationRecorderRecordingGroupElExclusionByResourceTypesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConfigConfigurationRecorderRecordingGroupElExclusionByResourceTypesElRef {
        ConfigConfigurationRecorderRecordingGroupElExclusionByResourceTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConfigConfigurationRecorderRecordingGroupElExclusionByResourceTypesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.resource_types", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ConfigConfigurationRecorderRecordingGroupElRecordingStrategyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    use_only: Option<PrimField<String>>,
}
impl ConfigConfigurationRecorderRecordingGroupElRecordingStrategyEl {
    #[doc = "Set the field `use_only`.\n"]
    pub fn set_use_only(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.use_only = Some(v.into());
        self
    }
}
impl ToListMappable for ConfigConfigurationRecorderRecordingGroupElRecordingStrategyEl {
    type O = BlockAssignable<ConfigConfigurationRecorderRecordingGroupElRecordingStrategyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConfigConfigurationRecorderRecordingGroupElRecordingStrategyEl {}
impl BuildConfigConfigurationRecorderRecordingGroupElRecordingStrategyEl {
    pub fn build(self) -> ConfigConfigurationRecorderRecordingGroupElRecordingStrategyEl {
        ConfigConfigurationRecorderRecordingGroupElRecordingStrategyEl {
            use_only: core::default::Default::default(),
        }
    }
}
pub struct ConfigConfigurationRecorderRecordingGroupElRecordingStrategyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigConfigurationRecorderRecordingGroupElRecordingStrategyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConfigConfigurationRecorderRecordingGroupElRecordingStrategyElRef {
        ConfigConfigurationRecorderRecordingGroupElRecordingStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConfigConfigurationRecorderRecordingGroupElRecordingStrategyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `use_only` after provisioning.\n"]
    pub fn use_only(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_only", self.base))
    }
}
#[derive(Serialize, Default)]
struct ConfigConfigurationRecorderRecordingGroupElDynamic {
    exclusion_by_resource_types:
        Option<DynamicBlock<ConfigConfigurationRecorderRecordingGroupElExclusionByResourceTypesEl>>,
    recording_strategy:
        Option<DynamicBlock<ConfigConfigurationRecorderRecordingGroupElRecordingStrategyEl>>,
}
#[derive(Serialize)]
pub struct ConfigConfigurationRecorderRecordingGroupEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all_supported: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_global_resource_types: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion_by_resource_types:
        Option<Vec<ConfigConfigurationRecorderRecordingGroupElExclusionByResourceTypesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recording_strategy: Option<Vec<ConfigConfigurationRecorderRecordingGroupElRecordingStrategyEl>>,
    dynamic: ConfigConfigurationRecorderRecordingGroupElDynamic,
}
impl ConfigConfigurationRecorderRecordingGroupEl {
    #[doc = "Set the field `all_supported`.\n"]
    pub fn set_all_supported(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all_supported = Some(v.into());
        self
    }
    #[doc = "Set the field `include_global_resource_types`.\n"]
    pub fn set_include_global_resource_types(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_global_resource_types = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_types`.\n"]
    pub fn set_resource_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resource_types = Some(v.into());
        self
    }
    #[doc = "Set the field `exclusion_by_resource_types`.\n"]
    pub fn set_exclusion_by_resource_types(
        mut self,
        v: impl Into<
            BlockAssignable<ConfigConfigurationRecorderRecordingGroupElExclusionByResourceTypesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclusion_by_resource_types = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion_by_resource_types = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `recording_strategy`.\n"]
    pub fn set_recording_strategy(
        mut self,
        v: impl Into<BlockAssignable<ConfigConfigurationRecorderRecordingGroupElRecordingStrategyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.recording_strategy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.recording_strategy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ConfigConfigurationRecorderRecordingGroupEl {
    type O = BlockAssignable<ConfigConfigurationRecorderRecordingGroupEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConfigConfigurationRecorderRecordingGroupEl {}
impl BuildConfigConfigurationRecorderRecordingGroupEl {
    pub fn build(self) -> ConfigConfigurationRecorderRecordingGroupEl {
        ConfigConfigurationRecorderRecordingGroupEl {
            all_supported: core::default::Default::default(),
            include_global_resource_types: core::default::Default::default(),
            resource_types: core::default::Default::default(),
            exclusion_by_resource_types: core::default::Default::default(),
            recording_strategy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ConfigConfigurationRecorderRecordingGroupElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigConfigurationRecorderRecordingGroupElRef {
    fn new(shared: StackShared, base: String) -> ConfigConfigurationRecorderRecordingGroupElRef {
        ConfigConfigurationRecorderRecordingGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConfigConfigurationRecorderRecordingGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all_supported` after provisioning.\n"]
    pub fn all_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.all_supported", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_global_resource_types` after provisioning.\n"]
    pub fn include_global_resource_types(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_global_resource_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.resource_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclusion_by_resource_types` after provisioning.\n"]
    pub fn exclusion_by_resource_types(
        &self,
    ) -> ListRef<ConfigConfigurationRecorderRecordingGroupElExclusionByResourceTypesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclusion_by_resource_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `recording_strategy` after provisioning.\n"]
    pub fn recording_strategy(
        &self,
    ) -> ListRef<ConfigConfigurationRecorderRecordingGroupElRecordingStrategyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recording_strategy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ConfigConfigurationRecorderRecordingModeElRecordingModeOverrideEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    recording_frequency: PrimField<String>,
    resource_types: SetField<PrimField<String>>,
}
impl ConfigConfigurationRecorderRecordingModeElRecordingModeOverrideEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}
impl ToListMappable for ConfigConfigurationRecorderRecordingModeElRecordingModeOverrideEl {
    type O = BlockAssignable<ConfigConfigurationRecorderRecordingModeElRecordingModeOverrideEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConfigConfigurationRecorderRecordingModeElRecordingModeOverrideEl {
    #[doc = ""]
    pub recording_frequency: PrimField<String>,
    #[doc = ""]
    pub resource_types: SetField<PrimField<String>>,
}
impl BuildConfigConfigurationRecorderRecordingModeElRecordingModeOverrideEl {
    pub fn build(self) -> ConfigConfigurationRecorderRecordingModeElRecordingModeOverrideEl {
        ConfigConfigurationRecorderRecordingModeElRecordingModeOverrideEl {
            description: core::default::Default::default(),
            recording_frequency: self.recording_frequency,
            resource_types: self.resource_types,
        }
    }
}
pub struct ConfigConfigurationRecorderRecordingModeElRecordingModeOverrideElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigConfigurationRecorderRecordingModeElRecordingModeOverrideElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConfigConfigurationRecorderRecordingModeElRecordingModeOverrideElRef {
        ConfigConfigurationRecorderRecordingModeElRecordingModeOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConfigConfigurationRecorderRecordingModeElRecordingModeOverrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `recording_frequency` after provisioning.\n"]
    pub fn recording_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.recording_frequency", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.resource_types", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ConfigConfigurationRecorderRecordingModeElDynamic {
    recording_mode_override:
        Option<DynamicBlock<ConfigConfigurationRecorderRecordingModeElRecordingModeOverrideEl>>,
}
#[derive(Serialize)]
pub struct ConfigConfigurationRecorderRecordingModeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    recording_frequency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recording_mode_override:
        Option<Vec<ConfigConfigurationRecorderRecordingModeElRecordingModeOverrideEl>>,
    dynamic: ConfigConfigurationRecorderRecordingModeElDynamic,
}
impl ConfigConfigurationRecorderRecordingModeEl {
    #[doc = "Set the field `recording_frequency`.\n"]
    pub fn set_recording_frequency(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.recording_frequency = Some(v.into());
        self
    }
    #[doc = "Set the field `recording_mode_override`.\n"]
    pub fn set_recording_mode_override(
        mut self,
        v: impl Into<BlockAssignable<ConfigConfigurationRecorderRecordingModeElRecordingModeOverrideEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.recording_mode_override = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.recording_mode_override = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ConfigConfigurationRecorderRecordingModeEl {
    type O = BlockAssignable<ConfigConfigurationRecorderRecordingModeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConfigConfigurationRecorderRecordingModeEl {}
impl BuildConfigConfigurationRecorderRecordingModeEl {
    pub fn build(self) -> ConfigConfigurationRecorderRecordingModeEl {
        ConfigConfigurationRecorderRecordingModeEl {
            recording_frequency: core::default::Default::default(),
            recording_mode_override: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ConfigConfigurationRecorderRecordingModeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigConfigurationRecorderRecordingModeElRef {
    fn new(shared: StackShared, base: String) -> ConfigConfigurationRecorderRecordingModeElRef {
        ConfigConfigurationRecorderRecordingModeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConfigConfigurationRecorderRecordingModeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `recording_frequency` after provisioning.\n"]
    pub fn recording_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.recording_frequency", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `recording_mode_override` after provisioning.\n"]
    pub fn recording_mode_override(
        &self,
    ) -> ListRef<ConfigConfigurationRecorderRecordingModeElRecordingModeOverrideElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recording_mode_override", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ConfigConfigurationRecorderDynamic {
    recording_group: Option<DynamicBlock<ConfigConfigurationRecorderRecordingGroupEl>>,
    recording_mode: Option<DynamicBlock<ConfigConfigurationRecorderRecordingModeEl>>,
}
