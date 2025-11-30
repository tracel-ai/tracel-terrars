use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataNetworkmanagerCoreNetworkPolicyDocumentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment_policies:
        Option<Vec<DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    core_network_configuration:
        Option<Vec<DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_function_groups:
        Option<Vec<DataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    segment_actions: Option<Vec<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    segments: Option<Vec<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentsEl>>,
    dynamic: DataNetworkmanagerCoreNetworkPolicyDocumentDynamic,
}

struct DataNetworkmanagerCoreNetworkPolicyDocument_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataNetworkmanagerCoreNetworkPolicyDocumentData>,
}

#[derive(Clone)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument(
    Rc<DataNetworkmanagerCoreNetworkPolicyDocument_>,
);

impl DataNetworkmanagerCoreNetworkPolicyDocument {
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

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version = Some(v.into());
        self
    }

    #[doc = "Set the field `attachment_policies`.\n"]
    pub fn set_attachment_policies(
        self,
        v: impl Into<BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attachment_policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attachment_policies = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `core_network_configuration`.\n"]
    pub fn set_core_network_configuration(
        self,
        v: impl Into<
            BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().core_network_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.core_network_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `network_function_groups`.\n"]
    pub fn set_network_function_groups(
        self,
        v: impl Into<
            BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_function_groups = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_function_groups = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `segment_actions`.\n"]
    pub fn set_segment_actions(
        self,
        v: impl Into<BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().segment_actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.segment_actions = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `segments`.\n"]
    pub fn set_segments(
        self,
        v: impl Into<BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().segments = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.segments = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `attachment_policies` after provisioning.\n"]
    pub fn attachment_policies(
        &self,
    ) -> ListRef<DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attachment_policies", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `core_network_configuration` after provisioning.\n"]
    pub fn core_network_configuration(
        &self,
    ) -> ListRef<DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.core_network_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `network_function_groups` after provisioning.\n"]
    pub fn network_function_groups(
        &self,
    ) -> ListRef<DataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_function_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `segment_actions` after provisioning.\n"]
    pub fn segment_actions(
        &self,
    ) -> ListRef<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.segment_actions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `segments` after provisioning.\n"]
    pub fn segments(&self) -> ListRef<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.segments", self.extract_ref()),
        )
    }
}

impl Referable for DataNetworkmanagerCoreNetworkPolicyDocument {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataNetworkmanagerCoreNetworkPolicyDocument {}

impl ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument {
    type O = ListRef<DataNetworkmanagerCoreNetworkPolicyDocumentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataNetworkmanagerCoreNetworkPolicyDocument_ {
    fn extract_datasource_type(&self) -> String {
        "aws_networkmanager_core_network_policy_document".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument {
    pub tf_id: String,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument {
    pub fn build(self, stack: &mut Stack) -> DataNetworkmanagerCoreNetworkPolicyDocument {
        let out = DataNetworkmanagerCoreNetworkPolicyDocument(Rc::new(
            DataNetworkmanagerCoreNetworkPolicyDocument_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataNetworkmanagerCoreNetworkPolicyDocumentData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    version: core::default::Default::default(),
                    attachment_policies: core::default::Default::default(),
                    core_network_configuration: core::default::Default::default(),
                    network_function_groups: core::default::Default::default(),
                    segment_actions: core::default::Default::default(),
                    segments: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataNetworkmanagerCoreNetworkPolicyDocumentRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkmanagerCoreNetworkPolicyDocumentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `attachment_policies` after provisioning.\n"]
    pub fn attachment_policies(
        &self,
    ) -> ListRef<DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attachment_policies", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `core_network_configuration` after provisioning.\n"]
    pub fn core_network_configuration(
        &self,
    ) -> ListRef<DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.core_network_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `network_function_groups` after provisioning.\n"]
    pub fn network_function_groups(
        &self,
    ) -> ListRef<DataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_function_groups", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `segment_actions` after provisioning.\n"]
    pub fn segment_actions(
        &self,
    ) -> ListRef<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.segment_actions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `segments` after provisioning.\n"]
    pub fn segments(&self) -> ListRef<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.segments", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    add_to_network_function_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    association_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_acceptance: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    segment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_value_of_key: Option<PrimField<String>>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElActionEl {
    #[doc = "Set the field `add_to_network_function_group`.\n"]
    pub fn set_add_to_network_function_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.add_to_network_function_group = Some(v.into());
        self
    }

    #[doc = "Set the field `association_method`.\n"]
    pub fn set_association_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.association_method = Some(v.into());
        self
    }

    #[doc = "Set the field `require_acceptance`.\n"]
    pub fn set_require_acceptance(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_acceptance = Some(v.into());
        self
    }

    #[doc = "Set the field `segment`.\n"]
    pub fn set_segment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.segment = Some(v.into());
        self
    }

    #[doc = "Set the field `tag_value_of_key`.\n"]
    pub fn set_tag_value_of_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag_value_of_key = Some(v.into());
        self
    }
}

impl ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElActionEl {
    type O =
        BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElActionEl {}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElActionEl {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElActionEl {
        DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElActionEl {
            add_to_network_function_group: core::default::Default::default(),
            association_method: core::default::Default::default(),
            require_acceptance: core::default::Default::default(),
            segment: core::default::Default::default(),
            tag_value_of_key: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElActionElRef {
        DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `add_to_network_function_group` after provisioning.\n"]
    pub fn add_to_network_function_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.add_to_network_function_group", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `association_method` after provisioning.\n"]
    pub fn association_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `require_acceptance` after provisioning.\n"]
    pub fn require_acceptance(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_acceptance", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `segment` after provisioning.\n"]
    pub fn segment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.segment", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_value_of_key` after provisioning.\n"]
    pub fn tag_value_of_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tag_value_of_key", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElConditionsEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable
    for DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElConditionsEl
{
    type O = BlockAssignable<
        DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElConditionsEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElConditionsEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElConditionsEl {
    pub fn build(
        self,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElConditionsEl {
        DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElConditionsEl {
            key: core::default::Default::default(),
            operator: core::default::Default::default(),
            type_: self.type_,
            value: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElConditionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElConditionsElRef {
        DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
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
struct DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElDynamic {
    action: Option<
        DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElActionEl>,
    >,
    conditions: Option<
        DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElConditionsEl>,
    >,
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition_logic: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    rule_number: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions:
        Option<Vec<DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElConditionsEl>>,
    dynamic: DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElDynamic,
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesEl {
    #[doc = "Set the field `condition_logic`.\n"]
    pub fn set_condition_logic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.condition_logic = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElActionEl,
            >,
        >,
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

    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElConditionsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesEl {
    type O = BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesEl {
    #[doc = ""]
    pub rule_number: PrimField<f64>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesEl {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesEl {
        DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesEl {
            condition_logic: core::default::Default::default(),
            description: core::default::Default::default(),
            rule_number: self.rule_number,
            action: core::default::Default::default(),
            conditions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElRef {
        DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition_logic` after provisioning.\n"]
    pub fn condition_logic(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.condition_logic", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_number` after provisioning.\n"]
    pub fn rule_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_number", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(
        &self,
    ) -> ListRef<DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(
        &self,
    ) -> ListRef<DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesElConditionsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElEdgeLocationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    asn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inside_cidr_blocks: Option<ListField<PrimField<String>>>,
    location: PrimField<String>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElEdgeLocationsEl {
    #[doc = "Set the field `asn`.\n"]
    pub fn set_asn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.asn = Some(v.into());
        self
    }

    #[doc = "Set the field `inside_cidr_blocks`.\n"]
    pub fn set_inside_cidr_blocks(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.inside_cidr_blocks = Some(v.into());
        self
    }
}

impl ToListMappable
    for DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElEdgeLocationsEl
{
    type O = BlockAssignable<
        DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElEdgeLocationsEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElEdgeLocationsEl
{
    #[doc = ""]
    pub location: PrimField<String>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElEdgeLocationsEl {
    pub fn build(
        self,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElEdgeLocationsEl {
        DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElEdgeLocationsEl {
            asn: core::default::Default::default(),
            inside_cidr_blocks: core::default::Default::default(),
            location: self.location,
        }
    }
}

pub struct DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElEdgeLocationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref
    for DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElEdgeLocationsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElEdgeLocationsElRef
    {
        DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElEdgeLocationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElEdgeLocationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `asn` after provisioning.\n"]
    pub fn asn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }

    #[doc = "Get a reference to the value of field `inside_cidr_blocks` after provisioning.\n"]
    pub fn inside_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inside_cidr_blocks", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElDynamic {
    edge_locations: Option<
        DynamicBlock<
            DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElEdgeLocationsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationEl {
    asn_ranges: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_support: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inside_cidr_blocks: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_referencing_support: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpn_ecmp_support: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edge_locations: Option<
        Vec<DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElEdgeLocationsEl>,
    >,
    dynamic: DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElDynamic,
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationEl {
    #[doc = "Set the field `dns_support`.\n"]
    pub fn set_dns_support(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.dns_support = Some(v.into());
        self
    }

    #[doc = "Set the field `inside_cidr_blocks`.\n"]
    pub fn set_inside_cidr_blocks(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.inside_cidr_blocks = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_referencing_support`.\n"]
    pub fn set_security_group_referencing_support(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.security_group_referencing_support = Some(v.into());
        self
    }

    #[doc = "Set the field `vpn_ecmp_support`.\n"]
    pub fn set_vpn_ecmp_support(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.vpn_ecmp_support = Some(v.into());
        self
    }

    #[doc = "Set the field `edge_locations`.\n"]
    pub fn set_edge_locations(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElEdgeLocationsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.edge_locations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.edge_locations = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationEl {
    type O = BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationEl {
    #[doc = ""]
    pub asn_ranges: SetField<PrimField<String>>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationEl {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationEl {
        DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationEl {
            asn_ranges: self.asn_ranges,
            dns_support: core::default::Default::default(),
            inside_cidr_blocks: core::default::Default::default(),
            security_group_referencing_support: core::default::Default::default(),
            vpn_ecmp_support: core::default::Default::default(),
            edge_locations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElRef {
        DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `asn_ranges` after provisioning.\n"]
    pub fn asn_ranges(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.asn_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_support` after provisioning.\n"]
    pub fn dns_support(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_support", self.base))
    }

    #[doc = "Get a reference to the value of field `inside_cidr_blocks` after provisioning.\n"]
    pub fn inside_cidr_blocks(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.inside_cidr_blocks", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `security_group_referencing_support` after provisioning.\n"]
    pub fn security_group_referencing_support(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_group_referencing_support", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `vpn_ecmp_support` after provisioning.\n"]
    pub fn vpn_ecmp_support(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpn_ecmp_support", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `edge_locations` after provisioning.\n"]
    pub fn edge_locations(
        &self,
    ) -> ListRef<
        DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationElEdgeLocationsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.edge_locations", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    require_attachment_acceptance: PrimField<bool>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}

impl ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsEl {
    type O = BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub require_attachment_acceptance: PrimField<bool>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsEl {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsEl {
        DataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsEl {
            description: core::default::Default::default(),
            name: self.name,
            require_attachment_acceptance: self.require_attachment_acceptance,
        }
    }
}

pub struct DataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsElRef {
        DataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `require_attachment_acceptance` after provisioning.\n"]
    pub fn require_attachment_acceptance(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_attachment_acceptance", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElWithEdgeOverrideEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    edge_sets: Option<SetField<SetField<PrimField<String>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_edge: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_edge_location: Option<PrimField<String>>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElWithEdgeOverrideEl {
    #[doc = "Set the field `edge_sets`.\n"]
    pub fn set_edge_sets(mut self, v: impl Into<SetField<SetField<PrimField<String>>>>) -> Self {
        self.edge_sets = Some(v.into());
        self
    }

    #[doc = "Set the field `use_edge`.\n"]
    pub fn set_use_edge(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.use_edge = Some(v.into());
        self
    }

    #[doc = "Set the field `use_edge_location`.\n"]
    pub fn set_use_edge_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.use_edge_location = Some(v.into());
        self
    }
}

impl ToListMappable
    for DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElWithEdgeOverrideEl
{
    type O = BlockAssignable<
        DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElWithEdgeOverrideEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElWithEdgeOverrideEl
{}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElWithEdgeOverrideEl {
    pub fn build(
        self,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElWithEdgeOverrideEl {
        DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElWithEdgeOverrideEl {
            edge_sets: core::default::Default::default(),
            use_edge: core::default::Default::default(),
            use_edge_location: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElWithEdgeOverrideElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElWithEdgeOverrideElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElWithEdgeOverrideElRef {
        DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElWithEdgeOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElWithEdgeOverrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `edge_sets` after provisioning.\n"]
    pub fn edge_sets(&self) -> SetRef<SetRef<PrimExpr<String>>> {
        SetRef::new(self.shared().clone(), format!("{}.edge_sets", self.base))
    }

    #[doc = "Get a reference to the value of field `use_edge` after provisioning.\n"]
    pub fn use_edge(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_edge", self.base))
    }

    #[doc = "Get a reference to the value of field `use_edge_location` after provisioning.\n"]
    pub fn use_edge_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_edge_location", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElDynamic {
    with_edge_override: Option<
        DynamicBlock<
            DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElWithEdgeOverrideEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network_function_groups: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_edge_override: Option<
        Vec<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElWithEdgeOverrideEl>,
    >,
    dynamic: DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElDynamic,
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaEl {
    #[doc = "Set the field `network_function_groups`.\n"]
    pub fn set_network_function_groups(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.network_function_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `with_edge_override`.\n"]
    pub fn set_with_edge_override(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElWithEdgeOverrideEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.with_edge_override = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.with_edge_override = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaEl {
    type O = BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaEl {}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaEl {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaEl {
        DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaEl {
            network_function_groups: core::default::Default::default(),
            with_edge_override: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElRef {
        DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_function_groups` after provisioning.\n"]
    pub fn network_function_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.network_function_groups", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `with_edge_override` after provisioning.\n"]
    pub fn with_edge_override(
        &self,
    ) -> ListRef<
        DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElWithEdgeOverrideElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.with_edge_override", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElWhenSentToEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    segments: Option<SetField<PrimField<String>>>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElWhenSentToEl {
    #[doc = "Set the field `segments`.\n"]
    pub fn set_segments(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.segments = Some(v.into());
        self
    }
}

impl ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElWhenSentToEl {
    type O =
        BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElWhenSentToEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElWhenSentToEl {}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElWhenSentToEl {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElWhenSentToEl {
        DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElWhenSentToEl {
            segments: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElWhenSentToElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElWhenSentToElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElWhenSentToElRef {
        DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElWhenSentToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElWhenSentToElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `segments` after provisioning.\n"]
    pub fn segments(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.segments", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElDynamic {
    via: Option<DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaEl>>,
    when_sent_to: Option<
        DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElWhenSentToEl>,
    >,
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsEl {
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_cidr_blocks: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destinations: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    segment: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_with: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_with_except: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    via: Option<Vec<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    when_sent_to:
        Option<Vec<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElWhenSentToEl>>,
    dynamic: DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElDynamic,
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_cidr_blocks`.\n"]
    pub fn set_destination_cidr_blocks(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.destination_cidr_blocks = Some(v.into());
        self
    }

    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.destinations = Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc = "Set the field `share_with`.\n"]
    pub fn set_share_with(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.share_with = Some(v.into());
        self
    }

    #[doc = "Set the field `share_with_except`.\n"]
    pub fn set_share_with_except(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.share_with_except = Some(v.into());
        self
    }

    #[doc = "Set the field `via`.\n"]
    pub fn set_via(
        mut self,
        v: impl Into<BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.via = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.via = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `when_sent_to`.\n"]
    pub fn set_when_sent_to(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElWhenSentToEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.when_sent_to = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.when_sent_to = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsEl {
    type O = BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsEl {
    #[doc = ""]
    pub action: PrimField<String>,
    #[doc = ""]
    pub segment: PrimField<String>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsEl {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsEl {
        DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsEl {
            action: self.action,
            description: core::default::Default::default(),
            destination_cidr_blocks: core::default::Default::default(),
            destinations: core::default::Default::default(),
            mode: core::default::Default::default(),
            segment: self.segment,
            share_with: core::default::Default::default(),
            share_with_except: core::default::Default::default(),
            via: core::default::Default::default(),
            when_sent_to: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElRef {
        DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_cidr_blocks` after provisioning.\n"]
    pub fn destination_cidr_blocks(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.destination_cidr_blocks", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.destinations", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `segment` after provisioning.\n"]
    pub fn segment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.segment", self.base))
    }

    #[doc = "Get a reference to the value of field `share_with` after provisioning.\n"]
    pub fn share_with(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.share_with", self.base))
    }

    #[doc = "Get a reference to the value of field `share_with_except` after provisioning.\n"]
    pub fn share_with_except(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.share_with_except", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `via` after provisioning.\n"]
    pub fn via(
        &self,
    ) -> ListRef<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElViaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.via", self.base))
    }

    #[doc = "Get a reference to the value of field `when_sent_to` after provisioning.\n"]
    pub fn when_sent_to(
        &self,
    ) -> ListRef<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsElWhenSentToElRef> {
        ListRef::new(self.shared().clone(), format!("{}.when_sent_to", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocumentSegmentsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_filter: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deny_filter: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edge_locations: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isolate_attachments: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_attachment_acceptance: Option<PrimField<bool>>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentSegmentsEl {
    #[doc = "Set the field `allow_filter`.\n"]
    pub fn set_allow_filter(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allow_filter = Some(v.into());
        self
    }

    #[doc = "Set the field `deny_filter`.\n"]
    pub fn set_deny_filter(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.deny_filter = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `edge_locations`.\n"]
    pub fn set_edge_locations(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.edge_locations = Some(v.into());
        self
    }

    #[doc = "Set the field `isolate_attachments`.\n"]
    pub fn set_isolate_attachments(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.isolate_attachments = Some(v.into());
        self
    }

    #[doc = "Set the field `require_attachment_acceptance`.\n"]
    pub fn set_require_attachment_acceptance(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_attachment_acceptance = Some(v.into());
        self
    }
}

impl ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocumentSegmentsEl {
    type O = BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocumentSegmentsEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocumentSegmentsEl {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocumentSegmentsEl {
        DataNetworkmanagerCoreNetworkPolicyDocumentSegmentsEl {
            allow_filter: core::default::Default::default(),
            deny_filter: core::default::Default::default(),
            description: core::default::Default::default(),
            edge_locations: core::default::Default::default(),
            isolate_attachments: core::default::Default::default(),
            name: self.name,
            require_attachment_acceptance: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkmanagerCoreNetworkPolicyDocumentSegmentsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkmanagerCoreNetworkPolicyDocumentSegmentsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocumentSegmentsElRef {
        DataNetworkmanagerCoreNetworkPolicyDocumentSegmentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentSegmentsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_filter` after provisioning.\n"]
    pub fn allow_filter(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allow_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `deny_filter` after provisioning.\n"]
    pub fn deny_filter(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.deny_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `edge_locations` after provisioning.\n"]
    pub fn edge_locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.edge_locations", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `isolate_attachments` after provisioning.\n"]
    pub fn isolate_attachments(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.isolate_attachments", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `require_attachment_acceptance` after provisioning.\n"]
    pub fn require_attachment_acceptance(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_attachment_acceptance", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct DataNetworkmanagerCoreNetworkPolicyDocumentDynamic {
    attachment_policies:
        Option<DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocumentAttachmentPoliciesEl>>,
    core_network_configuration:
        Option<DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocumentCoreNetworkConfigurationEl>>,
    network_function_groups:
        Option<DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocumentNetworkFunctionGroupsEl>>,
    segment_actions:
        Option<DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentActionsEl>>,
    segments: Option<DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocumentSegmentsEl>>,
}
