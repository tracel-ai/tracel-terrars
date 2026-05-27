use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FirebaseRemoteConfigRemoteConfigData {
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
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<FirebaseRemoteConfigRemoteConfigConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_groups: Option<Vec<FirebaseRemoteConfigRemoteConfigParameterGroupsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<FirebaseRemoteConfigRemoteConfigParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FirebaseRemoteConfigRemoteConfigTimeoutsEl>,
    dynamic: FirebaseRemoteConfigRemoteConfigDynamic,
}
struct FirebaseRemoteConfigRemoteConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FirebaseRemoteConfigRemoteConfigData>,
}
#[derive(Clone)]
pub struct FirebaseRemoteConfigRemoteConfig(Rc<FirebaseRemoteConfigRemoteConfig_>);
impl FirebaseRemoteConfigRemoteConfig {
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
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        self,
        v: impl Into<BlockAssignable<FirebaseRemoteConfigRemoteConfigConditionsEl>>,
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
    #[doc = "Set the field `parameter_groups`.\n"]
    pub fn set_parameter_groups(
        self,
        v: impl Into<BlockAssignable<FirebaseRemoteConfigRemoteConfigParameterGroupsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameter_groups = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameter_groups = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        self,
        v: impl Into<BlockAssignable<FirebaseRemoteConfigRemoteConfigParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FirebaseRemoteConfigRemoteConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the RemoteConfig.\nFormat: projects/{project}/namespaces/{namespace}/remoteConfig\nProject is a Firebase project ID or project number.\nNamespace is the namespace ID (e.g.: firebase or firebase-server)"]
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
    #[doc = "Get a reference to the value of field `version` after provisioning.\nContains all metadata about a particular version of the Remote Config\ntemplate.\n\nAll fields are set at the time the specified Remote Config template was\nwritten."]
    pub fn version(&self) -> ListRef<FirebaseRemoteConfigRemoteConfigVersionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<FirebaseRemoteConfigRemoteConfigConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseRemoteConfigRemoteConfigTimeoutsElRef {
        FirebaseRemoteConfigRemoteConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for FirebaseRemoteConfigRemoteConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FirebaseRemoteConfigRemoteConfig {}
impl ToListMappable for FirebaseRemoteConfigRemoteConfig {
    type O = ListRef<FirebaseRemoteConfigRemoteConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FirebaseRemoteConfigRemoteConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_firebase_remote_config_remote_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFirebaseRemoteConfigRemoteConfig {
    pub tf_id: String,
}
impl BuildFirebaseRemoteConfigRemoteConfig {
    pub fn build(self, stack: &mut Stack) -> FirebaseRemoteConfigRemoteConfig {
        let out = FirebaseRemoteConfigRemoteConfig(Rc::new(FirebaseRemoteConfigRemoteConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FirebaseRemoteConfigRemoteConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                conditions: core::default::Default::default(),
                parameter_groups: core::default::Default::default(),
                parameters: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FirebaseRemoteConfigRemoteConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseRemoteConfigRemoteConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FirebaseRemoteConfigRemoteConfigRef {
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the RemoteConfig.\nFormat: projects/{project}/namespaces/{namespace}/remoteConfig\nProject is a Firebase project ID or project number.\nNamespace is the namespace ID (e.g.: firebase or firebase-server)"]
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
    #[doc = "Get a reference to the value of field `version` after provisioning.\nContains all metadata about a particular version of the Remote Config\ntemplate.\n\nAll fields are set at the time the specified Remote Config template was\nwritten."]
    pub fn version(&self) -> ListRef<FirebaseRemoteConfigRemoteConfigVersionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<FirebaseRemoteConfigRemoteConfigConditionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conditions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseRemoteConfigRemoteConfigTimeoutsElRef {
        FirebaseRemoteConfigRemoteConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfigVersionElUpdateUserEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl FirebaseRemoteConfigRemoteConfigVersionElUpdateUserEl {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
    #[doc = "Set the field `image_url`.\n"]
    pub fn set_image_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_url = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseRemoteConfigRemoteConfigVersionElUpdateUserEl {
    type O = BlockAssignable<FirebaseRemoteConfigRemoteConfigVersionElUpdateUserEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseRemoteConfigRemoteConfigVersionElUpdateUserEl {}
impl BuildFirebaseRemoteConfigRemoteConfigVersionElUpdateUserEl {
    pub fn build(self) -> FirebaseRemoteConfigRemoteConfigVersionElUpdateUserEl {
        FirebaseRemoteConfigRemoteConfigVersionElUpdateUserEl {
            email: core::default::Default::default(),
            image_url: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct FirebaseRemoteConfigRemoteConfigVersionElUpdateUserElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseRemoteConfigRemoteConfigVersionElUpdateUserElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirebaseRemoteConfigRemoteConfigVersionElUpdateUserElRef {
        FirebaseRemoteConfigRemoteConfigVersionElUpdateUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseRemoteConfigRemoteConfigVersionElUpdateUserElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
    #[doc = "Get a reference to the value of field `image_url` after provisioning.\n"]
    pub fn image_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_url", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfigVersionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_legacy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollback_source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_origin: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_user: Option<ListField<FirebaseRemoteConfigRemoteConfigVersionElUpdateUserEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_number: Option<PrimField<String>>,
}
impl FirebaseRemoteConfigRemoteConfigVersionEl {
    #[doc = "Set the field `is_legacy`.\n"]
    pub fn set_is_legacy(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_legacy = Some(v.into());
        self
    }
    #[doc = "Set the field `rollback_source`.\n"]
    pub fn set_rollback_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rollback_source = Some(v.into());
        self
    }
    #[doc = "Set the field `update_origin`.\n"]
    pub fn set_update_origin(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_origin = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
    #[doc = "Set the field `update_type`.\n"]
    pub fn set_update_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_type = Some(v.into());
        self
    }
    #[doc = "Set the field `update_user`.\n"]
    pub fn set_update_user(
        mut self,
        v: impl Into<ListField<FirebaseRemoteConfigRemoteConfigVersionElUpdateUserEl>>,
    ) -> Self {
        self.update_user = Some(v.into());
        self
    }
    #[doc = "Set the field `version_number`.\n"]
    pub fn set_version_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version_number = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseRemoteConfigRemoteConfigVersionEl {
    type O = BlockAssignable<FirebaseRemoteConfigRemoteConfigVersionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseRemoteConfigRemoteConfigVersionEl {}
impl BuildFirebaseRemoteConfigRemoteConfigVersionEl {
    pub fn build(self) -> FirebaseRemoteConfigRemoteConfigVersionEl {
        FirebaseRemoteConfigRemoteConfigVersionEl {
            is_legacy: core::default::Default::default(),
            rollback_source: core::default::Default::default(),
            update_origin: core::default::Default::default(),
            update_time: core::default::Default::default(),
            update_type: core::default::Default::default(),
            update_user: core::default::Default::default(),
            version_number: core::default::Default::default(),
        }
    }
}
pub struct FirebaseRemoteConfigRemoteConfigVersionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseRemoteConfigRemoteConfigVersionElRef {
    fn new(shared: StackShared, base: String) -> FirebaseRemoteConfigRemoteConfigVersionElRef {
        FirebaseRemoteConfigRemoteConfigVersionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseRemoteConfigRemoteConfigVersionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_legacy` after provisioning.\n"]
    pub fn is_legacy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_legacy", self.base))
    }
    #[doc = "Get a reference to the value of field `rollback_source` after provisioning.\n"]
    pub fn rollback_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rollback_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `update_origin` after provisioning.\n"]
    pub fn update_origin(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_origin", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `update_type` after provisioning.\n"]
    pub fn update_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_type", self.base))
    }
    #[doc = "Get a reference to the value of field `update_user` after provisioning.\n"]
    pub fn update_user(&self) -> ListRef<FirebaseRemoteConfigRemoteConfigVersionElUpdateUserElRef> {
        ListRef::new(self.shared().clone(), format!("{}.update_user", self.base))
    }
    #[doc = "Get a reference to the value of field `version_number` after provisioning.\n"]
    pub fn version_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_number", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfigConditionsEl {
    expression: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_color: Option<PrimField<String>>,
}
impl FirebaseRemoteConfigRemoteConfigConditionsEl {
    #[doc = "Set the field `tag_color`.\nThe color associated with this condition for display purposes in the Firebase Console.\nNot specifying this value results in the Console picking an arbitrary color to associate with the condition. Possible values: [\"BLUE\", \"BROWN\", \"CYAN\", \"DEEP_ORANGE\", \"GREEN\", \"INDIGO\", \"LIME\", \"ORANGE\", \"PINK\", \"PURPLE\", \"TEAL\"]"]
    pub fn set_tag_color(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag_color = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseRemoteConfigRemoteConfigConditionsEl {
    type O = BlockAssignable<FirebaseRemoteConfigRemoteConfigConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseRemoteConfigRemoteConfigConditionsEl {
    #[doc = "The logic of this condition.\n\nSee the documentation regarding\n[Condition\nExpressions](https://firebase.google.com/docs/remote-config/condition-reference)\nfor the expected syntax of this field."]
    pub expression: PrimField<String>,
    #[doc = "A non-empty and unique name of this condition."]
    pub name: PrimField<String>,
}
impl BuildFirebaseRemoteConfigRemoteConfigConditionsEl {
    pub fn build(self) -> FirebaseRemoteConfigRemoteConfigConditionsEl {
        FirebaseRemoteConfigRemoteConfigConditionsEl {
            expression: self.expression,
            name: self.name,
            tag_color: core::default::Default::default(),
        }
    }
}
pub struct FirebaseRemoteConfigRemoteConfigConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseRemoteConfigRemoteConfigConditionsElRef {
    fn new(shared: StackShared, base: String) -> FirebaseRemoteConfigRemoteConfigConditionsElRef {
        FirebaseRemoteConfigRemoteConfigConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseRemoteConfigRemoteConfigConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nThe logic of this condition.\n\nSee the documentation regarding\n[Condition\nExpressions](https://firebase.google.com/docs/remote-config/condition-reference)\nfor the expected syntax of this field."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA non-empty and unique name of this condition."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `tag_color` after provisioning.\nThe color associated with this condition for display purposes in the Firebase Console.\nNot specifying this value results in the Console picking an arbitrary color to associate with the condition. Possible values: [\"BLUE\", \"BROWN\", \"CYAN\", \"DEEP_ORANGE\", \"GREEN\", \"INDIGO\", \"LIME\", \"ORANGE\", \"PINK\", \"PURPLE\", \"TEAL\"]"]
    pub fn tag_color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_color", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElConditionalValuesEl {
    condition_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_in_app_default: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElConditionalValuesEl {
    #[doc = "Set the field `use_in_app_default`.\nIf true, the parameter is omitted from the parameter values returned\nto a client."]
    pub fn set_use_in_app_default(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_in_app_default = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nThe string value that the parameter is set to."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElConditionalValuesEl
{
    type O = BlockAssignable<
        FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElConditionalValuesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElConditionalValuesEl {
    #[doc = ""]
    pub condition_name: PrimField<String>,
}
impl BuildFirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElConditionalValuesEl {
    pub fn build(
        self,
    ) -> FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElConditionalValuesEl {
        FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElConditionalValuesEl {
            condition_name: self.condition_name,
            use_in_app_default: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElConditionalValuesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElConditionalValuesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElConditionalValuesElRef {
        FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElConditionalValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElConditionalValuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `condition_name` after provisioning.\n"]
    pub fn condition_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.condition_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_in_app_default` after provisioning.\nIf true, the parameter is omitted from the parameter values returned\nto a client."]
    pub fn use_in_app_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_in_app_default", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe string value that the parameter is set to."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDefaultValueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    use_in_app_default: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDefaultValueEl {
    #[doc = "Set the field `use_in_app_default`.\nIf true, the parameter is omitted from the parameter values returned\nto a client."]
    pub fn set_use_in_app_default(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_in_app_default = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nThe string value that the parameter is set to."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDefaultValueEl
{
    type O = BlockAssignable<
        FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDefaultValueEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDefaultValueEl {}
impl BuildFirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDefaultValueEl {
    pub fn build(
        self,
    ) -> FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDefaultValueEl {
        FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDefaultValueEl {
            use_in_app_default: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDefaultValueElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDefaultValueElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDefaultValueElRef {
        FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDefaultValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDefaultValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `use_in_app_default` after provisioning.\nIf true, the parameter is omitted from the parameter values returned\nto a client."]
    pub fn use_in_app_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_in_app_default", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe string value that the parameter is set to."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDynamic {
    conditional_values: Option<
        DynamicBlock<
            FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElConditionalValuesEl,
        >,
    >,
    default_value: Option<
        DynamicBlock<FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDefaultValueEl>,
    >,
}
#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    parameter_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditional_values: Option<
        Vec<FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElConditionalValuesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value:
        Option<Vec<FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDefaultValueEl>>,
    dynamic: FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDynamic,
}
impl FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersEl {
    #[doc = "Set the field `description`.\nA description for this Parameter. Its length must be less than or equal to\n256 characters . A description may contain any Unicode characters."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `value_type`.\nThe data type for all values of this parameter in the current version of\nthe template. Default value: \"STRING\" Possible values: [\"STRING\", \"BOOLEAN\", \"NUMBER\", \"JSON\"]"]
    pub fn set_value_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value_type = Some(v.into());
        self
    }
    #[doc = "Set the field `conditional_values`.\n"]
    pub fn set_conditional_values(
        mut self,
        v: impl Into<
            BlockAssignable<
                FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElConditionalValuesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conditional_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_values = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(
        mut self,
        v: impl Into<
            BlockAssignable<
                FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDefaultValueEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_value = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersEl {
    type O = BlockAssignable<FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseRemoteConfigRemoteConfigParameterGroupsElParametersEl {
    #[doc = ""]
    pub parameter_name: PrimField<String>,
}
impl BuildFirebaseRemoteConfigRemoteConfigParameterGroupsElParametersEl {
    pub fn build(self) -> FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersEl {
        FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersEl {
            description: core::default::Default::default(),
            parameter_name: self.parameter_name,
            value_type: core::default::Default::default(),
            conditional_values: core::default::Default::default(),
            default_value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElRef {
        FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description for this Parameter. Its length must be less than or equal to\n256 characters . A description may contain any Unicode characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `parameter_name` after provisioning.\n"]
    pub fn parameter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `value_type` after provisioning.\nThe data type for all values of this parameter in the current version of\nthe template. Default value: \"STRING\" Possible values: [\"STRING\", \"BOOLEAN\", \"NUMBER\", \"JSON\"]"]
    pub fn value_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }
    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(
        &self,
    ) -> ListRef<FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersElDefaultValueElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_value", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct FirebaseRemoteConfigRemoteConfigParameterGroupsElDynamic {
    parameters: Option<DynamicBlock<FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersEl>>,
}
#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfigParameterGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    parameter_group_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersEl>>,
    dynamic: FirebaseRemoteConfigRemoteConfigParameterGroupsElDynamic,
}
impl FirebaseRemoteConfigRemoteConfigParameterGroupsEl {
    #[doc = "Set the field `description`.\nA description for the group. Its length must be less than or equal to 256\ncharacters. A description may contain any Unicode characters."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl Into<BlockAssignable<FirebaseRemoteConfigRemoteConfigParameterGroupsElParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FirebaseRemoteConfigRemoteConfigParameterGroupsEl {
    type O = BlockAssignable<FirebaseRemoteConfigRemoteConfigParameterGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseRemoteConfigRemoteConfigParameterGroupsEl {
    #[doc = ""]
    pub parameter_group_name: PrimField<String>,
}
impl BuildFirebaseRemoteConfigRemoteConfigParameterGroupsEl {
    pub fn build(self) -> FirebaseRemoteConfigRemoteConfigParameterGroupsEl {
        FirebaseRemoteConfigRemoteConfigParameterGroupsEl {
            description: core::default::Default::default(),
            parameter_group_name: self.parameter_group_name,
            parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FirebaseRemoteConfigRemoteConfigParameterGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseRemoteConfigRemoteConfigParameterGroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirebaseRemoteConfigRemoteConfigParameterGroupsElRef {
        FirebaseRemoteConfigRemoteConfigParameterGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseRemoteConfigRemoteConfigParameterGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description for the group. Its length must be less than or equal to 256\ncharacters. A description may contain any Unicode characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `parameter_group_name` after provisioning.\n"]
    pub fn parameter_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_group_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfigParametersElConditionalValuesEl {
    condition_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_in_app_default: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl FirebaseRemoteConfigRemoteConfigParametersElConditionalValuesEl {
    #[doc = "Set the field `use_in_app_default`.\nIf true, the parameter is omitted from the parameter values returned\nto a client."]
    pub fn set_use_in_app_default(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_in_app_default = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nThe string value that the parameter is set to."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseRemoteConfigRemoteConfigParametersElConditionalValuesEl {
    type O = BlockAssignable<FirebaseRemoteConfigRemoteConfigParametersElConditionalValuesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseRemoteConfigRemoteConfigParametersElConditionalValuesEl {
    #[doc = ""]
    pub condition_name: PrimField<String>,
}
impl BuildFirebaseRemoteConfigRemoteConfigParametersElConditionalValuesEl {
    pub fn build(self) -> FirebaseRemoteConfigRemoteConfigParametersElConditionalValuesEl {
        FirebaseRemoteConfigRemoteConfigParametersElConditionalValuesEl {
            condition_name: self.condition_name,
            use_in_app_default: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct FirebaseRemoteConfigRemoteConfigParametersElConditionalValuesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseRemoteConfigRemoteConfigParametersElConditionalValuesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirebaseRemoteConfigRemoteConfigParametersElConditionalValuesElRef {
        FirebaseRemoteConfigRemoteConfigParametersElConditionalValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseRemoteConfigRemoteConfigParametersElConditionalValuesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `condition_name` after provisioning.\n"]
    pub fn condition_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.condition_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `use_in_app_default` after provisioning.\nIf true, the parameter is omitted from the parameter values returned\nto a client."]
    pub fn use_in_app_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_in_app_default", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe string value that the parameter is set to."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfigParametersElDefaultValueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    use_in_app_default: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl FirebaseRemoteConfigRemoteConfigParametersElDefaultValueEl {
    #[doc = "Set the field `use_in_app_default`.\nIf true, the parameter is omitted from the parameter values returned\nto a client."]
    pub fn set_use_in_app_default(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_in_app_default = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nThe string value that the parameter is set to."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseRemoteConfigRemoteConfigParametersElDefaultValueEl {
    type O = BlockAssignable<FirebaseRemoteConfigRemoteConfigParametersElDefaultValueEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseRemoteConfigRemoteConfigParametersElDefaultValueEl {}
impl BuildFirebaseRemoteConfigRemoteConfigParametersElDefaultValueEl {
    pub fn build(self) -> FirebaseRemoteConfigRemoteConfigParametersElDefaultValueEl {
        FirebaseRemoteConfigRemoteConfigParametersElDefaultValueEl {
            use_in_app_default: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct FirebaseRemoteConfigRemoteConfigParametersElDefaultValueElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseRemoteConfigRemoteConfigParametersElDefaultValueElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> FirebaseRemoteConfigRemoteConfigParametersElDefaultValueElRef {
        FirebaseRemoteConfigRemoteConfigParametersElDefaultValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseRemoteConfigRemoteConfigParametersElDefaultValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `use_in_app_default` after provisioning.\nIf true, the parameter is omitted from the parameter values returned\nto a client."]
    pub fn use_in_app_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_in_app_default", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe string value that the parameter is set to."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct FirebaseRemoteConfigRemoteConfigParametersElDynamic {
    conditional_values:
        Option<DynamicBlock<FirebaseRemoteConfigRemoteConfigParametersElConditionalValuesEl>>,
    default_value: Option<DynamicBlock<FirebaseRemoteConfigRemoteConfigParametersElDefaultValueEl>>,
}
#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfigParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    parameter_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditional_values:
        Option<Vec<FirebaseRemoteConfigRemoteConfigParametersElConditionalValuesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value: Option<Vec<FirebaseRemoteConfigRemoteConfigParametersElDefaultValueEl>>,
    dynamic: FirebaseRemoteConfigRemoteConfigParametersElDynamic,
}
impl FirebaseRemoteConfigRemoteConfigParametersEl {
    #[doc = "Set the field `description`.\nA description for this Parameter. Its length must be less than or equal to\n256 characters . A description may contain any Unicode characters."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `value_type`.\nThe data type for all values of this parameter in the current version of\nthe template. Default value: \"STRING\" Possible values: [\"STRING\", \"BOOLEAN\", \"NUMBER\", \"JSON\"]"]
    pub fn set_value_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value_type = Some(v.into());
        self
    }
    #[doc = "Set the field `conditional_values`.\n"]
    pub fn set_conditional_values(
        mut self,
        v: impl Into<BlockAssignable<FirebaseRemoteConfigRemoteConfigParametersElConditionalValuesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conditional_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_values = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(
        mut self,
        v: impl Into<BlockAssignable<FirebaseRemoteConfigRemoteConfigParametersElDefaultValueEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_value = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FirebaseRemoteConfigRemoteConfigParametersEl {
    type O = BlockAssignable<FirebaseRemoteConfigRemoteConfigParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseRemoteConfigRemoteConfigParametersEl {
    #[doc = ""]
    pub parameter_name: PrimField<String>,
}
impl BuildFirebaseRemoteConfigRemoteConfigParametersEl {
    pub fn build(self) -> FirebaseRemoteConfigRemoteConfigParametersEl {
        FirebaseRemoteConfigRemoteConfigParametersEl {
            description: core::default::Default::default(),
            parameter_name: self.parameter_name,
            value_type: core::default::Default::default(),
            conditional_values: core::default::Default::default(),
            default_value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FirebaseRemoteConfigRemoteConfigParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseRemoteConfigRemoteConfigParametersElRef {
    fn new(shared: StackShared, base: String) -> FirebaseRemoteConfigRemoteConfigParametersElRef {
        FirebaseRemoteConfigRemoteConfigParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseRemoteConfigRemoteConfigParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description for this Parameter. Its length must be less than or equal to\n256 characters . A description may contain any Unicode characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `parameter_name` after provisioning.\n"]
    pub fn parameter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `value_type` after provisioning.\nThe data type for all values of this parameter in the current version of\nthe template. Default value: \"STRING\" Possible values: [\"STRING\", \"BOOLEAN\", \"NUMBER\", \"JSON\"]"]
    pub fn value_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }
    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(
        &self,
    ) -> ListRef<FirebaseRemoteConfigRemoteConfigParametersElDefaultValueElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_value", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FirebaseRemoteConfigRemoteConfigTimeoutsEl {
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
impl ToListMappable for FirebaseRemoteConfigRemoteConfigTimeoutsEl {
    type O = BlockAssignable<FirebaseRemoteConfigRemoteConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseRemoteConfigRemoteConfigTimeoutsEl {}
impl BuildFirebaseRemoteConfigRemoteConfigTimeoutsEl {
    pub fn build(self) -> FirebaseRemoteConfigRemoteConfigTimeoutsEl {
        FirebaseRemoteConfigRemoteConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FirebaseRemoteConfigRemoteConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseRemoteConfigRemoteConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FirebaseRemoteConfigRemoteConfigTimeoutsElRef {
        FirebaseRemoteConfigRemoteConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseRemoteConfigRemoteConfigTimeoutsElRef {
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
struct FirebaseRemoteConfigRemoteConfigDynamic {
    conditions: Option<DynamicBlock<FirebaseRemoteConfigRemoteConfigConditionsEl>>,
    parameter_groups: Option<DynamicBlock<FirebaseRemoteConfigRemoteConfigParameterGroupsEl>>,
    parameters: Option<DynamicBlock<FirebaseRemoteConfigRemoteConfigParametersEl>>,
}
