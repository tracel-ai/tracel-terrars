use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DiscoveryEngineControlData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_id: Option<PrimField<String>>,
    control_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    display_name: PrimField<String>,
    engine_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    solution_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_cases: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost_action: Option<Vec<DiscoveryEngineControlBoostActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<DiscoveryEngineControlConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_action: Option<Vec<DiscoveryEngineControlFilterActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    promote_action: Option<Vec<DiscoveryEngineControlPromoteActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_action: Option<Vec<DiscoveryEngineControlRedirectActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    synonyms_action: Option<Vec<DiscoveryEngineControlSynonymsActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DiscoveryEngineControlTimeoutsEl>,
    dynamic: DiscoveryEngineControlDynamic,
}
struct DiscoveryEngineControl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DiscoveryEngineControlData>,
}
#[derive(Clone)]
pub struct DiscoveryEngineControl(Rc<DiscoveryEngineControl_>);
impl DiscoveryEngineControl {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `collection_id`.\nThe collection ID. Currently only accepts \"default_collection\"."]
    pub fn set_collection_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().collection_id = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `use_cases`.\nThe use cases that the control is used for. Possible values: [\"SEARCH_USE_CASE_SEARCH\", \"SEARCH_USE_CASE_BROWSE\"]"]
    pub fn set_use_cases(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().use_cases = Some(v.into());
        self
    }
    #[doc = "Set the field `boost_action`.\n"]
    pub fn set_boost_action(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineControlBoostActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().boost_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.boost_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineControlConditionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.conditions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `filter_action`.\n"]
    pub fn set_filter_action(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineControlFilterActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `promote_action`.\n"]
    pub fn set_promote_action(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineControlPromoteActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().promote_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.promote_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `redirect_action`.\n"]
    pub fn set_redirect_action(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineControlRedirectActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redirect_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redirect_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `synonyms_action`.\n"]
    pub fn set_synonyms_action(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineControlSynonymsActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().synonyms_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.synonyms_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DiscoveryEngineControlTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe collection ID. Currently only accepts \"default_collection\"."]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_id` after provisioning.\nThe unique id of the control."]
    pub fn control_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the control. This field must be a UTF-8 encoded\nstring with a length limit of 128 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe engine to add the control to."]
    pub fn engine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the control. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}/controls/{control_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `solution_type` after provisioning.\nThe solution type that the control belongs to. Possible values: [\"SOLUTION_TYPE_RECOMMENDATION\", \"SOLUTION_TYPE_SEARCH\", \"SOLUTION_TYPE_CHAT\", \"SOLUTION_TYPE_GENERATIVE_CHAT\"]"]
    pub fn solution_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.solution_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `use_cases` after provisioning.\nThe use cases that the control is used for. Possible values: [\"SEARCH_USE_CASE_SEARCH\", \"SEARCH_USE_CASE_BROWSE\"]"]
    pub fn use_cases(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.use_cases", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `boost_action` after provisioning.\n"]
    pub fn boost_action(&self) -> ListRef<DiscoveryEngineControlBoostActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.boost_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<DiscoveryEngineControlConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter_action` after provisioning.\n"]
    pub fn filter_action(&self) -> ListRef<DiscoveryEngineControlFilterActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `promote_action` after provisioning.\n"]
    pub fn promote_action(&self) -> ListRef<DiscoveryEngineControlPromoteActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.promote_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redirect_action` after provisioning.\n"]
    pub fn redirect_action(&self) -> ListRef<DiscoveryEngineControlRedirectActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redirect_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `synonyms_action` after provisioning.\n"]
    pub fn synonyms_action(&self) -> ListRef<DiscoveryEngineControlSynonymsActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.synonyms_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineControlTimeoutsElRef {
        DiscoveryEngineControlTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DiscoveryEngineControl {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DiscoveryEngineControl {}
impl ToListMappable for DiscoveryEngineControl {
    type O = ListRef<DiscoveryEngineControlRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DiscoveryEngineControl_ {
    fn extract_resource_type(&self) -> String {
        "google_discovery_engine_control".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDiscoveryEngineControl {
    pub tf_id: String,
    #[doc = "The unique id of the control."]
    pub control_id: PrimField<String>,
    #[doc = "The display name of the control. This field must be a UTF-8 encoded\nstring with a length limit of 128 characters."]
    pub display_name: PrimField<String>,
    #[doc = "The engine to add the control to."]
    pub engine_id: PrimField<String>,
    #[doc = "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: PrimField<String>,
    #[doc = "The solution type that the control belongs to. Possible values: [\"SOLUTION_TYPE_RECOMMENDATION\", \"SOLUTION_TYPE_SEARCH\", \"SOLUTION_TYPE_CHAT\", \"SOLUTION_TYPE_GENERATIVE_CHAT\"]"]
    pub solution_type: PrimField<String>,
}
impl BuildDiscoveryEngineControl {
    pub fn build(self, stack: &mut Stack) -> DiscoveryEngineControl {
        let out = DiscoveryEngineControl(Rc::new(DiscoveryEngineControl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DiscoveryEngineControlData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                collection_id: core::default::Default::default(),
                control_id: self.control_id,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                engine_id: self.engine_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                solution_type: self.solution_type,
                use_cases: core::default::Default::default(),
                boost_action: core::default::Default::default(),
                conditions: core::default::Default::default(),
                filter_action: core::default::Default::default(),
                promote_action: core::default::Default::default(),
                redirect_action: core::default::Default::default(),
                synonyms_action: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DiscoveryEngineControlRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineControlRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DiscoveryEngineControlRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe collection ID. Currently only accepts \"default_collection\"."]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `control_id` after provisioning.\nThe unique id of the control."]
    pub fn control_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the control. This field must be a UTF-8 encoded\nstring with a length limit of 128 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe engine to add the control to."]
    pub fn engine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the control. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}/controls/{control_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `solution_type` after provisioning.\nThe solution type that the control belongs to. Possible values: [\"SOLUTION_TYPE_RECOMMENDATION\", \"SOLUTION_TYPE_SEARCH\", \"SOLUTION_TYPE_CHAT\", \"SOLUTION_TYPE_GENERATIVE_CHAT\"]"]
    pub fn solution_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.solution_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `use_cases` after provisioning.\nThe use cases that the control is used for. Possible values: [\"SEARCH_USE_CASE_SEARCH\", \"SEARCH_USE_CASE_BROWSE\"]"]
    pub fn use_cases(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.use_cases", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `boost_action` after provisioning.\n"]
    pub fn boost_action(&self) -> ListRef<DiscoveryEngineControlBoostActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.boost_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<DiscoveryEngineControlConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter_action` after provisioning.\n"]
    pub fn filter_action(&self) -> ListRef<DiscoveryEngineControlFilterActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `promote_action` after provisioning.\n"]
    pub fn promote_action(&self) -> ListRef<DiscoveryEngineControlPromoteActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.promote_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redirect_action` after provisioning.\n"]
    pub fn redirect_action(&self) -> ListRef<DiscoveryEngineControlRedirectActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redirect_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `synonyms_action` after provisioning.\n"]
    pub fn synonyms_action(&self) -> ListRef<DiscoveryEngineControlSynonymsActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.synonyms_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineControlTimeoutsElRef {
        DiscoveryEngineControlTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineControlBoostActionElInterpolationBoostSpecElControlPointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost_amount: Option<PrimField<f64>>,
}
impl DiscoveryEngineControlBoostActionElInterpolationBoostSpecElControlPointEl {
    #[doc = "Set the field `attribute_value`.\nThe attribute value of the control point."]
    pub fn set_attribute_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_value = Some(v.into());
        self
    }
    #[doc = "Set the field `boost_amount`.\nThe value between -1 to 1 by which to boost the score if the attributeValue\nevaluates to the value specified above."]
    pub fn set_boost_amount(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boost_amount = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineControlBoostActionElInterpolationBoostSpecElControlPointEl {
    type O =
        BlockAssignable<DiscoveryEngineControlBoostActionElInterpolationBoostSpecElControlPointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineControlBoostActionElInterpolationBoostSpecElControlPointEl {}
impl BuildDiscoveryEngineControlBoostActionElInterpolationBoostSpecElControlPointEl {
    pub fn build(
        self,
    ) -> DiscoveryEngineControlBoostActionElInterpolationBoostSpecElControlPointEl {
        DiscoveryEngineControlBoostActionElInterpolationBoostSpecElControlPointEl {
            attribute_value: core::default::Default::default(),
            boost_amount: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineControlBoostActionElInterpolationBoostSpecElControlPointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineControlBoostActionElInterpolationBoostSpecElControlPointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineControlBoostActionElInterpolationBoostSpecElControlPointElRef {
        DiscoveryEngineControlBoostActionElInterpolationBoostSpecElControlPointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineControlBoostActionElInterpolationBoostSpecElControlPointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attribute_value` after provisioning.\nThe attribute value of the control point."]
    pub fn attribute_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.attribute_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `boost_amount` after provisioning.\nThe value between -1 to 1 by which to boost the score if the attributeValue\nevaluates to the value specified above."]
    pub fn boost_amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.boost_amount", self.base))
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineControlBoostActionElInterpolationBoostSpecElDynamic {
    control_point: Option<
        DynamicBlock<DiscoveryEngineControlBoostActionElInterpolationBoostSpecElControlPointEl>,
    >,
}
#[derive(Serialize)]
pub struct DiscoveryEngineControlBoostActionElInterpolationBoostSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interpolation_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_point:
        Option<Vec<DiscoveryEngineControlBoostActionElInterpolationBoostSpecElControlPointEl>>,
    dynamic: DiscoveryEngineControlBoostActionElInterpolationBoostSpecElDynamic,
}
impl DiscoveryEngineControlBoostActionElInterpolationBoostSpecEl {
    #[doc = "Set the field `attribute_type`.\nThe attribute type to be used to determine the boost amount. Possible values: [\"NUMERICAL\", \"FRESHNESS\"]"]
    pub fn set_attribute_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_type = Some(v.into());
        self
    }
    #[doc = "Set the field `field_name`.\nThe name of the field whose value will be used to determine the boost amount."]
    pub fn set_field_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field_name = Some(v.into());
        self
    }
    #[doc = "Set the field `interpolation_type`.\nThe interpolation type to be applied to connect the control points. Possible values: [\"LINEAR\"]"]
    pub fn set_interpolation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interpolation_type = Some(v.into());
        self
    }
    #[doc = "Set the field `control_point`.\n"]
    pub fn set_control_point(
        mut self,
        v: impl Into<
            BlockAssignable<
                DiscoveryEngineControlBoostActionElInterpolationBoostSpecElControlPointEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.control_point = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.control_point = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineControlBoostActionElInterpolationBoostSpecEl {
    type O = BlockAssignable<DiscoveryEngineControlBoostActionElInterpolationBoostSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineControlBoostActionElInterpolationBoostSpecEl {}
impl BuildDiscoveryEngineControlBoostActionElInterpolationBoostSpecEl {
    pub fn build(self) -> DiscoveryEngineControlBoostActionElInterpolationBoostSpecEl {
        DiscoveryEngineControlBoostActionElInterpolationBoostSpecEl {
            attribute_type: core::default::Default::default(),
            field_name: core::default::Default::default(),
            interpolation_type: core::default::Default::default(),
            control_point: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineControlBoostActionElInterpolationBoostSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineControlBoostActionElInterpolationBoostSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineControlBoostActionElInterpolationBoostSpecElRef {
        DiscoveryEngineControlBoostActionElInterpolationBoostSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineControlBoostActionElInterpolationBoostSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attribute_type` after provisioning.\nThe attribute type to be used to determine the boost amount. Possible values: [\"NUMERICAL\", \"FRESHNESS\"]"]
    pub fn attribute_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.attribute_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `field_name` after provisioning.\nThe name of the field whose value will be used to determine the boost amount."]
    pub fn field_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field_name", self.base))
    }
    #[doc = "Get a reference to the value of field `interpolation_type` after provisioning.\nThe interpolation type to be applied to connect the control points. Possible values: [\"LINEAR\"]"]
    pub fn interpolation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interpolation_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `control_point` after provisioning.\n"]
    pub fn control_point(
        &self,
    ) -> ListRef<DiscoveryEngineControlBoostActionElInterpolationBoostSpecElControlPointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_point", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineControlBoostActionElDynamic {
    interpolation_boost_spec:
        Option<DynamicBlock<DiscoveryEngineControlBoostActionElInterpolationBoostSpecEl>>,
}
#[derive(Serialize)]
pub struct DiscoveryEngineControlBoostActionEl {
    data_store: PrimField<String>,
    filter: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_boost: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interpolation_boost_spec:
        Option<Vec<DiscoveryEngineControlBoostActionElInterpolationBoostSpecEl>>,
    dynamic: DiscoveryEngineControlBoostActionElDynamic,
}
impl DiscoveryEngineControlBoostActionEl {
    #[doc = "Set the field `fixed_boost`.\nThe fixed boost value to apply to the search results. Positive values will increase the relevance of the results, while negative values will decrease the relevance. The value must be between -100 and 100."]
    pub fn set_fixed_boost(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.fixed_boost = Some(v.into());
        self
    }
    #[doc = "Set the field `interpolation_boost_spec`.\n"]
    pub fn set_interpolation_boost_spec(
        mut self,
        v: impl Into<BlockAssignable<DiscoveryEngineControlBoostActionElInterpolationBoostSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.interpolation_boost_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.interpolation_boost_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineControlBoostActionEl {
    type O = BlockAssignable<DiscoveryEngineControlBoostActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineControlBoostActionEl {
    #[doc = "The data store to boost."]
    pub data_store: PrimField<String>,
    #[doc = "The filter to apply to the search results."]
    pub filter: PrimField<String>,
}
impl BuildDiscoveryEngineControlBoostActionEl {
    pub fn build(self) -> DiscoveryEngineControlBoostActionEl {
        DiscoveryEngineControlBoostActionEl {
            data_store: self.data_store,
            filter: self.filter,
            fixed_boost: core::default::Default::default(),
            interpolation_boost_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineControlBoostActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineControlBoostActionElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineControlBoostActionElRef {
        DiscoveryEngineControlBoostActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineControlBoostActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_store` after provisioning.\nThe data store to boost."]
    pub fn data_store(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_store", self.base))
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nThe filter to apply to the search results."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `fixed_boost` after provisioning.\nThe fixed boost value to apply to the search results. Positive values will increase the relevance of the results, while negative values will decrease the relevance. The value must be between -100 and 100."]
    pub fn fixed_boost(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.fixed_boost", self.base))
    }
    #[doc = "Get a reference to the value of field `interpolation_boost_spec` after provisioning.\n"]
    pub fn interpolation_boost_spec(
        &self,
    ) -> ListRef<DiscoveryEngineControlBoostActionElInterpolationBoostSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.interpolation_boost_spec", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineControlConditionsElActiveTimeRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl DiscoveryEngineControlConditionsElActiveTimeRangeEl {
    #[doc = "Set the field `end_time`.\nThe end time of the active time range."]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\nThe start time of the active time range."]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineControlConditionsElActiveTimeRangeEl {
    type O = BlockAssignable<DiscoveryEngineControlConditionsElActiveTimeRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineControlConditionsElActiveTimeRangeEl {}
impl BuildDiscoveryEngineControlConditionsElActiveTimeRangeEl {
    pub fn build(self) -> DiscoveryEngineControlConditionsElActiveTimeRangeEl {
        DiscoveryEngineControlConditionsElActiveTimeRangeEl {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineControlConditionsElActiveTimeRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineControlConditionsElActiveTimeRangeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineControlConditionsElActiveTimeRangeElRef {
        DiscoveryEngineControlConditionsElActiveTimeRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineControlConditionsElActiveTimeRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\nThe end time of the active time range."]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nThe start time of the active time range."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineControlConditionsElQueryTermsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    full_match: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DiscoveryEngineControlConditionsElQueryTermsEl {
    #[doc = "Set the field `full_match`.\nIf true, the query term must be an exact match. Otherwise, the query term can be a partial match."]
    pub fn set_full_match(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.full_match = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nThe value of the query term."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineControlConditionsElQueryTermsEl {
    type O = BlockAssignable<DiscoveryEngineControlConditionsElQueryTermsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineControlConditionsElQueryTermsEl {}
impl BuildDiscoveryEngineControlConditionsElQueryTermsEl {
    pub fn build(self) -> DiscoveryEngineControlConditionsElQueryTermsEl {
        DiscoveryEngineControlConditionsElQueryTermsEl {
            full_match: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineControlConditionsElQueryTermsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineControlConditionsElQueryTermsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineControlConditionsElQueryTermsElRef {
        DiscoveryEngineControlConditionsElQueryTermsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineControlConditionsElQueryTermsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `full_match` after provisioning.\nIf true, the query term must be an exact match. Otherwise, the query term can be a partial match."]
    pub fn full_match(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_match", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the query term."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineControlConditionsElDynamic {
    active_time_range: Option<DynamicBlock<DiscoveryEngineControlConditionsElActiveTimeRangeEl>>,
    query_terms: Option<DynamicBlock<DiscoveryEngineControlConditionsElQueryTermsEl>>,
}
#[derive(Serialize)]
pub struct DiscoveryEngineControlConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    query_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_time_range: Option<Vec<DiscoveryEngineControlConditionsElActiveTimeRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_terms: Option<Vec<DiscoveryEngineControlConditionsElQueryTermsEl>>,
    dynamic: DiscoveryEngineControlConditionsElDynamic,
}
impl DiscoveryEngineControlConditionsEl {
    #[doc = "Set the field `query_regex`.\nThe regular expression that the query must match for this condition to be met."]
    pub fn set_query_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `active_time_range`.\n"]
    pub fn set_active_time_range(
        mut self,
        v: impl Into<BlockAssignable<DiscoveryEngineControlConditionsElActiveTimeRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.active_time_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.active_time_range = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `query_terms`.\n"]
    pub fn set_query_terms(
        mut self,
        v: impl Into<BlockAssignable<DiscoveryEngineControlConditionsElQueryTermsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_terms = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_terms = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineControlConditionsEl {
    type O = BlockAssignable<DiscoveryEngineControlConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineControlConditionsEl {}
impl BuildDiscoveryEngineControlConditionsEl {
    pub fn build(self) -> DiscoveryEngineControlConditionsEl {
        DiscoveryEngineControlConditionsEl {
            query_regex: core::default::Default::default(),
            active_time_range: core::default::Default::default(),
            query_terms: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineControlConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineControlConditionsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineControlConditionsElRef {
        DiscoveryEngineControlConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineControlConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `query_regex` after provisioning.\nThe regular expression that the query must match for this condition to be met."]
    pub fn query_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query_regex", self.base))
    }
    #[doc = "Get a reference to the value of field `active_time_range` after provisioning.\n"]
    pub fn active_time_range(
        &self,
    ) -> ListRef<DiscoveryEngineControlConditionsElActiveTimeRangeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.active_time_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_terms` after provisioning.\n"]
    pub fn query_terms(&self) -> ListRef<DiscoveryEngineControlConditionsElQueryTermsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.query_terms", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineControlFilterActionEl {
    data_store: PrimField<String>,
    filter: PrimField<String>,
}
impl DiscoveryEngineControlFilterActionEl {}
impl ToListMappable for DiscoveryEngineControlFilterActionEl {
    type O = BlockAssignable<DiscoveryEngineControlFilterActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineControlFilterActionEl {
    #[doc = "The data store to filter."]
    pub data_store: PrimField<String>,
    #[doc = "The filter to apply to the search results."]
    pub filter: PrimField<String>,
}
impl BuildDiscoveryEngineControlFilterActionEl {
    pub fn build(self) -> DiscoveryEngineControlFilterActionEl {
        DiscoveryEngineControlFilterActionEl {
            data_store: self.data_store,
            filter: self.filter,
        }
    }
}
pub struct DiscoveryEngineControlFilterActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineControlFilterActionElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineControlFilterActionElRef {
        DiscoveryEngineControlFilterActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineControlFilterActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_store` after provisioning.\nThe data store to filter."]
    pub fn data_store(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_store", self.base))
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nThe filter to apply to the search results."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineControlPromoteActionElSearchLinkPromotionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_uri: Option<PrimField<String>>,
    title: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl DiscoveryEngineControlPromoteActionElSearchLinkPromotionEl {
    #[doc = "Set the field `description`.\nThe description of the promoted link."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `document`.\nThe document to promote."]
    pub fn set_document(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.document = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\nReturn promotions for basic site search."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `image_uri`.\nThe image URI of the promoted link."]
    pub fn set_image_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `uri`.\nThe URI to promote."]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineControlPromoteActionElSearchLinkPromotionEl {
    type O = BlockAssignable<DiscoveryEngineControlPromoteActionElSearchLinkPromotionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineControlPromoteActionElSearchLinkPromotionEl {
    #[doc = "The title of the promoted link."]
    pub title: PrimField<String>,
}
impl BuildDiscoveryEngineControlPromoteActionElSearchLinkPromotionEl {
    pub fn build(self) -> DiscoveryEngineControlPromoteActionElSearchLinkPromotionEl {
        DiscoveryEngineControlPromoteActionElSearchLinkPromotionEl {
            description: core::default::Default::default(),
            document: core::default::Default::default(),
            enabled: core::default::Default::default(),
            image_uri: core::default::Default::default(),
            title: self.title,
            uri: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineControlPromoteActionElSearchLinkPromotionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineControlPromoteActionElSearchLinkPromotionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineControlPromoteActionElSearchLinkPromotionElRef {
        DiscoveryEngineControlPromoteActionElSearchLinkPromotionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineControlPromoteActionElSearchLinkPromotionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the promoted link."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `document` after provisioning.\nThe document to promote."]
    pub fn document(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.document", self.base))
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nReturn promotions for basic site search."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\nThe image URI of the promoted link."]
    pub fn image_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe title of the promoted link."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe URI to promote."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineControlPromoteActionElDynamic {
    search_link_promotion:
        Option<DynamicBlock<DiscoveryEngineControlPromoteActionElSearchLinkPromotionEl>>,
}
#[derive(Serialize)]
pub struct DiscoveryEngineControlPromoteActionEl {
    data_store: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_link_promotion: Option<Vec<DiscoveryEngineControlPromoteActionElSearchLinkPromotionEl>>,
    dynamic: DiscoveryEngineControlPromoteActionElDynamic,
}
impl DiscoveryEngineControlPromoteActionEl {
    #[doc = "Set the field `search_link_promotion`.\n"]
    pub fn set_search_link_promotion(
        mut self,
        v: impl Into<BlockAssignable<DiscoveryEngineControlPromoteActionElSearchLinkPromotionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.search_link_promotion = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.search_link_promotion = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineControlPromoteActionEl {
    type O = BlockAssignable<DiscoveryEngineControlPromoteActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineControlPromoteActionEl {
    #[doc = "The data store to promote."]
    pub data_store: PrimField<String>,
}
impl BuildDiscoveryEngineControlPromoteActionEl {
    pub fn build(self) -> DiscoveryEngineControlPromoteActionEl {
        DiscoveryEngineControlPromoteActionEl {
            data_store: self.data_store,
            search_link_promotion: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineControlPromoteActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineControlPromoteActionElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineControlPromoteActionElRef {
        DiscoveryEngineControlPromoteActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineControlPromoteActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_store` after provisioning.\nThe data store to promote."]
    pub fn data_store(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_store", self.base))
    }
    #[doc = "Get a reference to the value of field `search_link_promotion` after provisioning.\n"]
    pub fn search_link_promotion(
        &self,
    ) -> ListRef<DiscoveryEngineControlPromoteActionElSearchLinkPromotionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.search_link_promotion", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineControlRedirectActionEl {
    redirect_uri: PrimField<String>,
}
impl DiscoveryEngineControlRedirectActionEl {}
impl ToListMappable for DiscoveryEngineControlRedirectActionEl {
    type O = BlockAssignable<DiscoveryEngineControlRedirectActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineControlRedirectActionEl {
    #[doc = "The URI to redirect to."]
    pub redirect_uri: PrimField<String>,
}
impl BuildDiscoveryEngineControlRedirectActionEl {
    pub fn build(self) -> DiscoveryEngineControlRedirectActionEl {
        DiscoveryEngineControlRedirectActionEl {
            redirect_uri: self.redirect_uri,
        }
    }
}
pub struct DiscoveryEngineControlRedirectActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineControlRedirectActionElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineControlRedirectActionElRef {
        DiscoveryEngineControlRedirectActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineControlRedirectActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `redirect_uri` after provisioning.\nThe URI to redirect to."]
    pub fn redirect_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_uri", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineControlSynonymsActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    synonyms: Option<ListField<PrimField<String>>>,
}
impl DiscoveryEngineControlSynonymsActionEl {
    #[doc = "Set the field `synonyms`.\nThe synonyms to apply to the search results."]
    pub fn set_synonyms(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.synonyms = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineControlSynonymsActionEl {
    type O = BlockAssignable<DiscoveryEngineControlSynonymsActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineControlSynonymsActionEl {}
impl BuildDiscoveryEngineControlSynonymsActionEl {
    pub fn build(self) -> DiscoveryEngineControlSynonymsActionEl {
        DiscoveryEngineControlSynonymsActionEl {
            synonyms: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineControlSynonymsActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineControlSynonymsActionElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineControlSynonymsActionElRef {
        DiscoveryEngineControlSynonymsActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineControlSynonymsActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `synonyms` after provisioning.\nThe synonyms to apply to the search results."]
    pub fn synonyms(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.synonyms", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineControlTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DiscoveryEngineControlTimeoutsEl {
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
impl ToListMappable for DiscoveryEngineControlTimeoutsEl {
    type O = BlockAssignable<DiscoveryEngineControlTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineControlTimeoutsEl {}
impl BuildDiscoveryEngineControlTimeoutsEl {
    pub fn build(self) -> DiscoveryEngineControlTimeoutsEl {
        DiscoveryEngineControlTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineControlTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineControlTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineControlTimeoutsElRef {
        DiscoveryEngineControlTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineControlTimeoutsElRef {
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
struct DiscoveryEngineControlDynamic {
    boost_action: Option<DynamicBlock<DiscoveryEngineControlBoostActionEl>>,
    conditions: Option<DynamicBlock<DiscoveryEngineControlConditionsEl>>,
    filter_action: Option<DynamicBlock<DiscoveryEngineControlFilterActionEl>>,
    promote_action: Option<DynamicBlock<DiscoveryEngineControlPromoteActionEl>>,
    redirect_action: Option<DynamicBlock<DiscoveryEngineControlRedirectActionEl>>,
    synonyms_action: Option<DynamicBlock<DiscoveryEngineControlSynonymsActionEl>>,
}
