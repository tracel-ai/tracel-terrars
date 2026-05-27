use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CesAppVersionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app: PrimField<String>,
    app_version_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CesAppVersionTimeoutsEl>,
}
struct CesAppVersion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CesAppVersionData>,
}
#[derive(Clone)]
pub struct CesAppVersion(Rc<CesAppVersion_>);
impl CesAppVersion {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe description of the app version."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe display name of the app version."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CesAppVersionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `app_version_id` after provisioning.\nThe ID to use for the app version, which will become the final component\nof the app version's resource name. If not provided, a unique ID will be\nautomatically assigned for the app version."]
    pub fn app_version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_version_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the app version was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creator` after provisioning.\nEmail of the user who created the app version."]
    pub fn creator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creator", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the app version."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the app version."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the app version.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/versions/{version}'"]
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
    #[doc = "Get a reference to the value of field `snapshot` after provisioning.\nA snapshot of the app."]
    pub fn snapshot(&self) -> ListRef<CesAppVersionSnapshotElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesAppVersionTimeoutsElRef {
        CesAppVersionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CesAppVersion {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CesAppVersion {}
impl ToListMappable for CesAppVersion {
    type O = ListRef<CesAppVersionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CesAppVersion_ {
    fn extract_resource_type(&self) -> String {
        "google_ces_app_version".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCesAppVersion {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub app: PrimField<String>,
    #[doc = "The ID to use for the app version, which will become the final component\nof the app version's resource name. If not provided, a unique ID will be\nautomatically assigned for the app version."]
    pub app_version_id: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
}
impl BuildCesAppVersion {
    pub fn build(self, stack: &mut Stack) -> CesAppVersion {
        let out = CesAppVersion(Rc::new(CesAppVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CesAppVersionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app: self.app,
                app_version_id: self.app_version_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CesAppVersionRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CesAppVersionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `app_version_id` after provisioning.\nThe ID to use for the app version, which will become the final component\nof the app version's resource name. If not provided, a unique ID will be\nautomatically assigned for the app version."]
    pub fn app_version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_version_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the app version was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creator` after provisioning.\nEmail of the user who created the app version."]
    pub fn creator(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creator", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the app version."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the app version."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the app version.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/versions/{version}'"]
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
    #[doc = "Get a reference to the value of field `snapshot` after provisioning.\nA snapshot of the app."]
    pub fn snapshot(&self) -> ListRef<CesAppVersionSnapshotElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesAppVersionTimeoutsElRef {
        CesAppVersionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAgentsElAfterAgentCallbacksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_code: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElAgentsElAfterAgentCallbacksEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.python_code = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAgentsElAfterAgentCallbacksEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAgentsElAfterAgentCallbacksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAgentsElAfterAgentCallbacksEl {}
impl BuildCesAppVersionSnapshotElAgentsElAfterAgentCallbacksEl {
    pub fn build(self) -> CesAppVersionSnapshotElAgentsElAfterAgentCallbacksEl {
        CesAppVersionSnapshotElAgentsElAfterAgentCallbacksEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAgentsElAfterAgentCallbacksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAgentsElAfterAgentCallbacksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAgentsElAfterAgentCallbacksElRef {
        CesAppVersionSnapshotElAgentsElAfterAgentCallbacksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAgentsElAfterAgentCallbacksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAgentsElAfterModelCallbacksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_code: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElAgentsElAfterModelCallbacksEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.python_code = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAgentsElAfterModelCallbacksEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAgentsElAfterModelCallbacksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAgentsElAfterModelCallbacksEl {}
impl BuildCesAppVersionSnapshotElAgentsElAfterModelCallbacksEl {
    pub fn build(self) -> CesAppVersionSnapshotElAgentsElAfterModelCallbacksEl {
        CesAppVersionSnapshotElAgentsElAfterModelCallbacksEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAgentsElAfterModelCallbacksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAgentsElAfterModelCallbacksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAgentsElAfterModelCallbacksElRef {
        CesAppVersionSnapshotElAgentsElAfterModelCallbacksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAgentsElAfterModelCallbacksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAgentsElAfterToolCallbacksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_code: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElAgentsElAfterToolCallbacksEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.python_code = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAgentsElAfterToolCallbacksEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAgentsElAfterToolCallbacksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAgentsElAfterToolCallbacksEl {}
impl BuildCesAppVersionSnapshotElAgentsElAfterToolCallbacksEl {
    pub fn build(self) -> CesAppVersionSnapshotElAgentsElAfterToolCallbacksEl {
        CesAppVersionSnapshotElAgentsElAfterToolCallbacksEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAgentsElAfterToolCallbacksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAgentsElAfterToolCallbacksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAgentsElAfterToolCallbacksElRef {
        CesAppVersionSnapshotElAgentsElAfterToolCallbacksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAgentsElAfterToolCallbacksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAgentsElBeforeAgentCallbacksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_code: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElAgentsElBeforeAgentCallbacksEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.python_code = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAgentsElBeforeAgentCallbacksEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAgentsElBeforeAgentCallbacksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAgentsElBeforeAgentCallbacksEl {}
impl BuildCesAppVersionSnapshotElAgentsElBeforeAgentCallbacksEl {
    pub fn build(self) -> CesAppVersionSnapshotElAgentsElBeforeAgentCallbacksEl {
        CesAppVersionSnapshotElAgentsElBeforeAgentCallbacksEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAgentsElBeforeAgentCallbacksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAgentsElBeforeAgentCallbacksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAgentsElBeforeAgentCallbacksElRef {
        CesAppVersionSnapshotElAgentsElBeforeAgentCallbacksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAgentsElBeforeAgentCallbacksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAgentsElBeforeModelCallbacksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_code: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElAgentsElBeforeModelCallbacksEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.python_code = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAgentsElBeforeModelCallbacksEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAgentsElBeforeModelCallbacksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAgentsElBeforeModelCallbacksEl {}
impl BuildCesAppVersionSnapshotElAgentsElBeforeModelCallbacksEl {
    pub fn build(self) -> CesAppVersionSnapshotElAgentsElBeforeModelCallbacksEl {
        CesAppVersionSnapshotElAgentsElBeforeModelCallbacksEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAgentsElBeforeModelCallbacksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAgentsElBeforeModelCallbacksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAgentsElBeforeModelCallbacksElRef {
        CesAppVersionSnapshotElAgentsElBeforeModelCallbacksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAgentsElBeforeModelCallbacksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAgentsElBeforeToolCallbacksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_code: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElAgentsElBeforeToolCallbacksEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.python_code = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAgentsElBeforeToolCallbacksEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAgentsElBeforeToolCallbacksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAgentsElBeforeToolCallbacksEl {}
impl BuildCesAppVersionSnapshotElAgentsElBeforeToolCallbacksEl {
    pub fn build(self) -> CesAppVersionSnapshotElAgentsElBeforeToolCallbacksEl {
        CesAppVersionSnapshotElAgentsElBeforeToolCallbacksEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAgentsElBeforeToolCallbacksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAgentsElBeforeToolCallbacksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAgentsElBeforeToolCallbacksElRef {
        CesAppVersionSnapshotElAgentsElBeforeToolCallbacksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAgentsElBeforeToolCallbacksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAgentsElLlmAgentEl {}
impl CesAppVersionSnapshotElAgentsElLlmAgentEl {}
impl ToListMappable for CesAppVersionSnapshotElAgentsElLlmAgentEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAgentsElLlmAgentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAgentsElLlmAgentEl {}
impl BuildCesAppVersionSnapshotElAgentsElLlmAgentEl {
    pub fn build(self) -> CesAppVersionSnapshotElAgentsElLlmAgentEl {
        CesAppVersionSnapshotElAgentsElLlmAgentEl {}
    }
}
pub struct CesAppVersionSnapshotElAgentsElLlmAgentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAgentsElLlmAgentElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElAgentsElLlmAgentElRef {
        CesAppVersionSnapshotElAgentsElLlmAgentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAgentsElLlmAgentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAgentsElModelSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
}
impl CesAppVersionSnapshotElAgentsElModelSettingsEl {
    #[doc = "Set the field `model`.\n"]
    pub fn set_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model = Some(v.into());
        self
    }
    #[doc = "Set the field `temperature`.\n"]
    pub fn set_temperature(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.temperature = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAgentsElModelSettingsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAgentsElModelSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAgentsElModelSettingsEl {}
impl BuildCesAppVersionSnapshotElAgentsElModelSettingsEl {
    pub fn build(self) -> CesAppVersionSnapshotElAgentsElModelSettingsEl {
        CesAppVersionSnapshotElAgentsElModelSettingsEl {
            model: core::default::Default::default(),
            temperature: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAgentsElModelSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAgentsElModelSettingsElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElAgentsElModelSettingsElRef {
        CesAppVersionSnapshotElAgentsElModelSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAgentsElModelSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }
    #[doc = "Get a reference to the value of field `temperature` after provisioning.\n"]
    pub fn temperature(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAgentsElRemoteDialogflowAgentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    agent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_variable_mapping: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_variable_mapping: Option<RecField<PrimField<String>>>,
}
impl CesAppVersionSnapshotElAgentsElRemoteDialogflowAgentEl {
    #[doc = "Set the field `agent`.\n"]
    pub fn set_agent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.agent = Some(v.into());
        self
    }
    #[doc = "Set the field `environment_id`.\n"]
    pub fn set_environment_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment_id = Some(v.into());
        self
    }
    #[doc = "Set the field `flow_id`.\n"]
    pub fn set_flow_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.flow_id = Some(v.into());
        self
    }
    #[doc = "Set the field `input_variable_mapping`.\n"]
    pub fn set_input_variable_mapping(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.input_variable_mapping = Some(v.into());
        self
    }
    #[doc = "Set the field `output_variable_mapping`.\n"]
    pub fn set_output_variable_mapping(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.output_variable_mapping = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAgentsElRemoteDialogflowAgentEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAgentsElRemoteDialogflowAgentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAgentsElRemoteDialogflowAgentEl {}
impl BuildCesAppVersionSnapshotElAgentsElRemoteDialogflowAgentEl {
    pub fn build(self) -> CesAppVersionSnapshotElAgentsElRemoteDialogflowAgentEl {
        CesAppVersionSnapshotElAgentsElRemoteDialogflowAgentEl {
            agent: core::default::Default::default(),
            environment_id: core::default::Default::default(),
            flow_id: core::default::Default::default(),
            input_variable_mapping: core::default::Default::default(),
            output_variable_mapping: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAgentsElRemoteDialogflowAgentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAgentsElRemoteDialogflowAgentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAgentsElRemoteDialogflowAgentElRef {
        CesAppVersionSnapshotElAgentsElRemoteDialogflowAgentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAgentsElRemoteDialogflowAgentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `agent` after provisioning.\n"]
    pub fn agent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent", self.base))
    }
    #[doc = "Get a reference to the value of field `environment_id` after provisioning.\n"]
    pub fn environment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.environment_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `flow_id` after provisioning.\n"]
    pub fn flow_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flow_id", self.base))
    }
    #[doc = "Get a reference to the value of field `input_variable_mapping` after provisioning.\n"]
    pub fn input_variable_mapping(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.input_variable_mapping", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_variable_mapping` after provisioning.\n"]
    pub fn output_variable_mapping(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.output_variable_mapping", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAgentsElToolsetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toolset: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElAgentsElToolsetsEl {
    #[doc = "Set the field `tool_ids`.\n"]
    pub fn set_tool_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tool_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `toolset`.\n"]
    pub fn set_toolset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.toolset = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAgentsElToolsetsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAgentsElToolsetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAgentsElToolsetsEl {}
impl BuildCesAppVersionSnapshotElAgentsElToolsetsEl {
    pub fn build(self) -> CesAppVersionSnapshotElAgentsElToolsetsEl {
        CesAppVersionSnapshotElAgentsElToolsetsEl {
            tool_ids: core::default::Default::default(),
            toolset: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAgentsElToolsetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAgentsElToolsetsElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElAgentsElToolsetsElRef {
        CesAppVersionSnapshotElAgentsElToolsetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAgentsElToolsetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tool_ids` after provisioning.\n"]
    pub fn tool_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tool_ids", self.base))
    }
    #[doc = "Get a reference to the value of field `toolset` after provisioning.\n"]
    pub fn toolset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.toolset", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAgentsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    after_agent_callbacks: Option<ListField<CesAppVersionSnapshotElAgentsElAfterAgentCallbacksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after_model_callbacks: Option<ListField<CesAppVersionSnapshotElAgentsElAfterModelCallbacksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after_tool_callbacks: Option<ListField<CesAppVersionSnapshotElAgentsElAfterToolCallbacksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before_agent_callbacks:
        Option<ListField<CesAppVersionSnapshotElAgentsElBeforeAgentCallbacksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before_model_callbacks:
        Option<ListField<CesAppVersionSnapshotElAgentsElBeforeModelCallbacksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before_tool_callbacks: Option<ListField<CesAppVersionSnapshotElAgentsElBeforeToolCallbacksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    child_agents: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generated_summary: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guardrails: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instruction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    llm_agent: Option<ListField<CesAppVersionSnapshotElAgentsElLlmAgentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_settings: Option<ListField<CesAppVersionSnapshotElAgentsElModelSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_dialogflow_agent:
        Option<ListField<CesAppVersionSnapshotElAgentsElRemoteDialogflowAgentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toolsets: Option<ListField<CesAppVersionSnapshotElAgentsElToolsetsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElAgentsEl {
    #[doc = "Set the field `after_agent_callbacks`.\n"]
    pub fn set_after_agent_callbacks(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAgentsElAfterAgentCallbacksEl>>,
    ) -> Self {
        self.after_agent_callbacks = Some(v.into());
        self
    }
    #[doc = "Set the field `after_model_callbacks`.\n"]
    pub fn set_after_model_callbacks(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAgentsElAfterModelCallbacksEl>>,
    ) -> Self {
        self.after_model_callbacks = Some(v.into());
        self
    }
    #[doc = "Set the field `after_tool_callbacks`.\n"]
    pub fn set_after_tool_callbacks(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAgentsElAfterToolCallbacksEl>>,
    ) -> Self {
        self.after_tool_callbacks = Some(v.into());
        self
    }
    #[doc = "Set the field `before_agent_callbacks`.\n"]
    pub fn set_before_agent_callbacks(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAgentsElBeforeAgentCallbacksEl>>,
    ) -> Self {
        self.before_agent_callbacks = Some(v.into());
        self
    }
    #[doc = "Set the field `before_model_callbacks`.\n"]
    pub fn set_before_model_callbacks(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAgentsElBeforeModelCallbacksEl>>,
    ) -> Self {
        self.before_model_callbacks = Some(v.into());
        self
    }
    #[doc = "Set the field `before_tool_callbacks`.\n"]
    pub fn set_before_tool_callbacks(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAgentsElBeforeToolCallbacksEl>>,
    ) -> Self {
        self.before_tool_callbacks = Some(v.into());
        self
    }
    #[doc = "Set the field `child_agents`.\n"]
    pub fn set_child_agents(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.child_agents = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.etag = Some(v.into());
        self
    }
    #[doc = "Set the field `generated_summary`.\n"]
    pub fn set_generated_summary(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.generated_summary = Some(v.into());
        self
    }
    #[doc = "Set the field `guardrails`.\n"]
    pub fn set_guardrails(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.guardrails = Some(v.into());
        self
    }
    #[doc = "Set the field `instruction`.\n"]
    pub fn set_instruction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instruction = Some(v.into());
        self
    }
    #[doc = "Set the field `llm_agent`.\n"]
    pub fn set_llm_agent(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAgentsElLlmAgentEl>>,
    ) -> Self {
        self.llm_agent = Some(v.into());
        self
    }
    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAgentsElModelSettingsEl>>,
    ) -> Self {
        self.model_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `remote_dialogflow_agent`.\n"]
    pub fn set_remote_dialogflow_agent(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAgentsElRemoteDialogflowAgentEl>>,
    ) -> Self {
        self.remote_dialogflow_agent = Some(v.into());
        self
    }
    #[doc = "Set the field `tools`.\n"]
    pub fn set_tools(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tools = Some(v.into());
        self
    }
    #[doc = "Set the field `toolsets`.\n"]
    pub fn set_toolsets(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAgentsElToolsetsEl>>,
    ) -> Self {
        self.toolsets = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAgentsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAgentsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAgentsEl {}
impl BuildCesAppVersionSnapshotElAgentsEl {
    pub fn build(self) -> CesAppVersionSnapshotElAgentsEl {
        CesAppVersionSnapshotElAgentsEl {
            after_agent_callbacks: core::default::Default::default(),
            after_model_callbacks: core::default::Default::default(),
            after_tool_callbacks: core::default::Default::default(),
            before_agent_callbacks: core::default::Default::default(),
            before_model_callbacks: core::default::Default::default(),
            before_tool_callbacks: core::default::Default::default(),
            child_agents: core::default::Default::default(),
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            etag: core::default::Default::default(),
            generated_summary: core::default::Default::default(),
            guardrails: core::default::Default::default(),
            instruction: core::default::Default::default(),
            llm_agent: core::default::Default::default(),
            model_settings: core::default::Default::default(),
            name: core::default::Default::default(),
            remote_dialogflow_agent: core::default::Default::default(),
            tools: core::default::Default::default(),
            toolsets: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAgentsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAgentsElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElAgentsElRef {
        CesAppVersionSnapshotElAgentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAgentsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `after_agent_callbacks` after provisioning.\n"]
    pub fn after_agent_callbacks(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAgentsElAfterAgentCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.after_agent_callbacks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `after_model_callbacks` after provisioning.\n"]
    pub fn after_model_callbacks(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAgentsElAfterModelCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.after_model_callbacks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `after_tool_callbacks` after provisioning.\n"]
    pub fn after_tool_callbacks(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAgentsElAfterToolCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.after_tool_callbacks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `before_agent_callbacks` after provisioning.\n"]
    pub fn before_agent_callbacks(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAgentsElBeforeAgentCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.before_agent_callbacks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `before_model_callbacks` after provisioning.\n"]
    pub fn before_model_callbacks(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAgentsElBeforeModelCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.before_model_callbacks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `before_tool_callbacks` after provisioning.\n"]
    pub fn before_tool_callbacks(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAgentsElBeforeToolCallbacksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.before_tool_callbacks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `child_agents` after provisioning.\n"]
    pub fn child_agents(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.child_agents", self.base))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }
    #[doc = "Get a reference to the value of field `generated_summary` after provisioning.\n"]
    pub fn generated_summary(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generated_summary", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `guardrails` after provisioning.\n"]
    pub fn guardrails(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.guardrails", self.base))
    }
    #[doc = "Get a reference to the value of field `instruction` after provisioning.\n"]
    pub fn instruction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instruction", self.base))
    }
    #[doc = "Get a reference to the value of field `llm_agent` after provisioning.\n"]
    pub fn llm_agent(&self) -> ListRef<CesAppVersionSnapshotElAgentsElLlmAgentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.llm_agent", self.base))
    }
    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> ListRef<CesAppVersionSnapshotElAgentsElModelSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `remote_dialogflow_agent` after provisioning.\n"]
    pub fn remote_dialogflow_agent(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAgentsElRemoteDialogflowAgentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.remote_dialogflow_agent", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tools` after provisioning.\n"]
    pub fn tools(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tools", self.base))
    }
    #[doc = "Get a reference to the value of field `toolsets` after provisioning.\n"]
    pub fn toolsets(&self) -> ListRef<CesAppVersionSnapshotElAgentsElToolsetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.toolsets", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElAudioProcessingConfigElAmbientSoundConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prebuilt_ambient_sound: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_gain_db: Option<PrimField<f64>>,
}
impl CesAppVersionSnapshotElAppElAudioProcessingConfigElAmbientSoundConfigEl {
    #[doc = "Set the field `gcs_uri`.\n"]
    pub fn set_gcs_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcs_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `prebuilt_ambient_sound`.\n"]
    pub fn set_prebuilt_ambient_sound(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prebuilt_ambient_sound = Some(v.into());
        self
    }
    #[doc = "Set the field `volume_gain_db`.\n"]
    pub fn set_volume_gain_db(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_gain_db = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElAudioProcessingConfigElAmbientSoundConfigEl {
    type O =
        BlockAssignable<CesAppVersionSnapshotElAppElAudioProcessingConfigElAmbientSoundConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElAudioProcessingConfigElAmbientSoundConfigEl {}
impl BuildCesAppVersionSnapshotElAppElAudioProcessingConfigElAmbientSoundConfigEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElAudioProcessingConfigElAmbientSoundConfigEl {
        CesAppVersionSnapshotElAppElAudioProcessingConfigElAmbientSoundConfigEl {
            gcs_uri: core::default::Default::default(),
            prebuilt_ambient_sound: core::default::Default::default(),
            volume_gain_db: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElAudioProcessingConfigElAmbientSoundConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElAudioProcessingConfigElAmbientSoundConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElAudioProcessingConfigElAmbientSoundConfigElRef {
        CesAppVersionSnapshotElAppElAudioProcessingConfigElAmbientSoundConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElAudioProcessingConfigElAmbientSoundConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcs_uri` after provisioning.\n"]
    pub fn gcs_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gcs_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `prebuilt_ambient_sound` after provisioning.\n"]
    pub fn prebuilt_ambient_sound(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prebuilt_ambient_sound", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `volume_gain_db` after provisioning.\n"]
    pub fn volume_gain_db(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.volume_gain_db", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElAudioProcessingConfigElBargeInConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    barge_in_awareness: Option<PrimField<bool>>,
}
impl CesAppVersionSnapshotElAppElAudioProcessingConfigElBargeInConfigEl {
    #[doc = "Set the field `barge_in_awareness`.\n"]
    pub fn set_barge_in_awareness(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.barge_in_awareness = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElAudioProcessingConfigElBargeInConfigEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElAudioProcessingConfigElBargeInConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElAudioProcessingConfigElBargeInConfigEl {}
impl BuildCesAppVersionSnapshotElAppElAudioProcessingConfigElBargeInConfigEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElAudioProcessingConfigElBargeInConfigEl {
        CesAppVersionSnapshotElAppElAudioProcessingConfigElBargeInConfigEl {
            barge_in_awareness: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElAudioProcessingConfigElBargeInConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElAudioProcessingConfigElBargeInConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElAudioProcessingConfigElBargeInConfigElRef {
        CesAppVersionSnapshotElAppElAudioProcessingConfigElBargeInConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElAudioProcessingConfigElBargeInConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `barge_in_awareness` after provisioning.\n"]
    pub fn barge_in_awareness(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.barge_in_awareness", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElAudioProcessingConfigElSynthesizeSpeechConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    speaking_rate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    voice: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElAppElAudioProcessingConfigElSynthesizeSpeechConfigsEl {
    #[doc = "Set the field `language_code`.\n"]
    pub fn set_language_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `speaking_rate`.\n"]
    pub fn set_speaking_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.speaking_rate = Some(v.into());
        self
    }
    #[doc = "Set the field `voice`.\n"]
    pub fn set_voice(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.voice = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesAppVersionSnapshotElAppElAudioProcessingConfigElSynthesizeSpeechConfigsEl
{
    type O = BlockAssignable<
        CesAppVersionSnapshotElAppElAudioProcessingConfigElSynthesizeSpeechConfigsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElAudioProcessingConfigElSynthesizeSpeechConfigsEl {}
impl BuildCesAppVersionSnapshotElAppElAudioProcessingConfigElSynthesizeSpeechConfigsEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElAppElAudioProcessingConfigElSynthesizeSpeechConfigsEl {
        CesAppVersionSnapshotElAppElAudioProcessingConfigElSynthesizeSpeechConfigsEl {
            language_code: core::default::Default::default(),
            speaking_rate: core::default::Default::default(),
            voice: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElAudioProcessingConfigElSynthesizeSpeechConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElAudioProcessingConfigElSynthesizeSpeechConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElAudioProcessingConfigElSynthesizeSpeechConfigsElRef {
        CesAppVersionSnapshotElAppElAudioProcessingConfigElSynthesizeSpeechConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElAudioProcessingConfigElSynthesizeSpeechConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `speaking_rate` after provisioning.\n"]
    pub fn speaking_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.speaking_rate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `voice` after provisioning.\n"]
    pub fn voice(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.voice", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElAudioProcessingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ambient_sound_config:
        Option<ListField<CesAppVersionSnapshotElAppElAudioProcessingConfigElAmbientSoundConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    barge_in_config:
        Option<ListField<CesAppVersionSnapshotElAppElAudioProcessingConfigElBargeInConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inactivity_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    synthesize_speech_configs: Option<
        SetField<CesAppVersionSnapshotElAppElAudioProcessingConfigElSynthesizeSpeechConfigsEl>,
    >,
}
impl CesAppVersionSnapshotElAppElAudioProcessingConfigEl {
    #[doc = "Set the field `ambient_sound_config`.\n"]
    pub fn set_ambient_sound_config(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElAudioProcessingConfigElAmbientSoundConfigEl>>,
    ) -> Self {
        self.ambient_sound_config = Some(v.into());
        self
    }
    #[doc = "Set the field `barge_in_config`.\n"]
    pub fn set_barge_in_config(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElAudioProcessingConfigElBargeInConfigEl>>,
    ) -> Self {
        self.barge_in_config = Some(v.into());
        self
    }
    #[doc = "Set the field `inactivity_timeout`.\n"]
    pub fn set_inactivity_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.inactivity_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `synthesize_speech_configs`.\n"]
    pub fn set_synthesize_speech_configs(
        mut self,
        v: impl Into<
            SetField<CesAppVersionSnapshotElAppElAudioProcessingConfigElSynthesizeSpeechConfigsEl>,
        >,
    ) -> Self {
        self.synthesize_speech_configs = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElAudioProcessingConfigEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElAudioProcessingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElAudioProcessingConfigEl {}
impl BuildCesAppVersionSnapshotElAppElAudioProcessingConfigEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElAudioProcessingConfigEl {
        CesAppVersionSnapshotElAppElAudioProcessingConfigEl {
            ambient_sound_config: core::default::Default::default(),
            barge_in_config: core::default::Default::default(),
            inactivity_timeout: core::default::Default::default(),
            synthesize_speech_configs: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElAudioProcessingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElAudioProcessingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElAudioProcessingConfigElRef {
        CesAppVersionSnapshotElAppElAudioProcessingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElAudioProcessingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ambient_sound_config` after provisioning.\n"]
    pub fn ambient_sound_config(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAppElAudioProcessingConfigElAmbientSoundConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ambient_sound_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `barge_in_config` after provisioning.\n"]
    pub fn barge_in_config(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAppElAudioProcessingConfigElBargeInConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.barge_in_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inactivity_timeout` after provisioning.\n"]
    pub fn inactivity_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inactivity_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `synthesize_speech_configs` after provisioning.\n"]
    pub fn synthesize_speech_configs(
        &self,
    ) -> SetRef<CesAppVersionSnapshotElAppElAudioProcessingConfigElSynthesizeSpeechConfigsElRef>
    {
        SetRef::new(
            self.shared().clone(),
            format!("{}.synthesize_speech_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElClientCertificateSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    passphrase: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_certificate: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElAppElClientCertificateSettingsEl {
    #[doc = "Set the field `passphrase`.\n"]
    pub fn set_passphrase(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.passphrase = Some(v.into());
        self
    }
    #[doc = "Set the field `private_key`.\n"]
    pub fn set_private_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_key = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_certificate`.\n"]
    pub fn set_tls_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tls_certificate = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElClientCertificateSettingsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElClientCertificateSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElClientCertificateSettingsEl {}
impl BuildCesAppVersionSnapshotElAppElClientCertificateSettingsEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElClientCertificateSettingsEl {
        CesAppVersionSnapshotElAppElClientCertificateSettingsEl {
            passphrase: core::default::Default::default(),
            private_key: core::default::Default::default(),
            tls_certificate: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElClientCertificateSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElClientCertificateSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElClientCertificateSettingsElRef {
        CesAppVersionSnapshotElAppElClientCertificateSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElClientCertificateSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `passphrase` after provisioning.\n"]
    pub fn passphrase(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.passphrase", self.base))
    }
    #[doc = "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }
    #[doc = "Get a reference to the value of field `tls_certificate` after provisioning.\n"]
    pub fn tls_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tls_certificate", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElDataStoreSettingsElEnginesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElAppElDataStoreSettingsElEnginesEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElDataStoreSettingsElEnginesEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElDataStoreSettingsElEnginesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElDataStoreSettingsElEnginesEl {}
impl BuildCesAppVersionSnapshotElAppElDataStoreSettingsElEnginesEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElDataStoreSettingsElEnginesEl {
        CesAppVersionSnapshotElAppElDataStoreSettingsElEnginesEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElDataStoreSettingsElEnginesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElDataStoreSettingsElEnginesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElDataStoreSettingsElEnginesElRef {
        CesAppVersionSnapshotElAppElDataStoreSettingsElEnginesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElDataStoreSettingsElEnginesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElDataStoreSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    engines: Option<ListField<CesAppVersionSnapshotElAppElDataStoreSettingsElEnginesEl>>,
}
impl CesAppVersionSnapshotElAppElDataStoreSettingsEl {
    #[doc = "Set the field `engines`.\n"]
    pub fn set_engines(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElDataStoreSettingsElEnginesEl>>,
    ) -> Self {
        self.engines = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElDataStoreSettingsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElDataStoreSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElDataStoreSettingsEl {}
impl BuildCesAppVersionSnapshotElAppElDataStoreSettingsEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElDataStoreSettingsEl {
        CesAppVersionSnapshotElAppElDataStoreSettingsEl {
            engines: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElDataStoreSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElDataStoreSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElDataStoreSettingsElRef {
        CesAppVersionSnapshotElAppElDataStoreSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElDataStoreSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `engines` after provisioning.\n"]
    pub fn engines(&self) -> ListRef<CesAppVersionSnapshotElAppElDataStoreSettingsElEnginesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.engines", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElDefaultChannelProfileElPersonaPropertyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    persona: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElAppElDefaultChannelProfileElPersonaPropertyEl {
    #[doc = "Set the field `persona`.\n"]
    pub fn set_persona(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.persona = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElDefaultChannelProfileElPersonaPropertyEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElDefaultChannelProfileElPersonaPropertyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElDefaultChannelProfileElPersonaPropertyEl {}
impl BuildCesAppVersionSnapshotElAppElDefaultChannelProfileElPersonaPropertyEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElDefaultChannelProfileElPersonaPropertyEl {
        CesAppVersionSnapshotElAppElDefaultChannelProfileElPersonaPropertyEl {
            persona: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElDefaultChannelProfileElPersonaPropertyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElDefaultChannelProfileElPersonaPropertyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElDefaultChannelProfileElPersonaPropertyElRef {
        CesAppVersionSnapshotElAppElDefaultChannelProfileElPersonaPropertyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElDefaultChannelProfileElPersonaPropertyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `persona` after provisioning.\n"]
    pub fn persona(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.persona", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElDefaultChannelProfileElWebWidgetConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    modality: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    theme: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_widget_title: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElAppElDefaultChannelProfileElWebWidgetConfigEl {
    #[doc = "Set the field `modality`.\n"]
    pub fn set_modality(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.modality = Some(v.into());
        self
    }
    #[doc = "Set the field `theme`.\n"]
    pub fn set_theme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.theme = Some(v.into());
        self
    }
    #[doc = "Set the field `web_widget_title`.\n"]
    pub fn set_web_widget_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_widget_title = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElDefaultChannelProfileElWebWidgetConfigEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElDefaultChannelProfileElWebWidgetConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElDefaultChannelProfileElWebWidgetConfigEl {}
impl BuildCesAppVersionSnapshotElAppElDefaultChannelProfileElWebWidgetConfigEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElDefaultChannelProfileElWebWidgetConfigEl {
        CesAppVersionSnapshotElAppElDefaultChannelProfileElWebWidgetConfigEl {
            modality: core::default::Default::default(),
            theme: core::default::Default::default(),
            web_widget_title: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElDefaultChannelProfileElWebWidgetConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElDefaultChannelProfileElWebWidgetConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElDefaultChannelProfileElWebWidgetConfigElRef {
        CesAppVersionSnapshotElAppElDefaultChannelProfileElWebWidgetConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElDefaultChannelProfileElWebWidgetConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `modality` after provisioning.\n"]
    pub fn modality(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.modality", self.base))
    }
    #[doc = "Get a reference to the value of field `theme` after provisioning.\n"]
    pub fn theme(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.theme", self.base))
    }
    #[doc = "Get a reference to the value of field `web_widget_title` after provisioning.\n"]
    pub fn web_widget_title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.web_widget_title", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElDefaultChannelProfileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_barge_in_control: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_dtmf: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    persona_property:
        Option<ListField<CesAppVersionSnapshotElAppElDefaultChannelProfileElPersonaPropertyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_widget_config:
        Option<ListField<CesAppVersionSnapshotElAppElDefaultChannelProfileElWebWidgetConfigEl>>,
}
impl CesAppVersionSnapshotElAppElDefaultChannelProfileEl {
    #[doc = "Set the field `channel_type`.\n"]
    pub fn set_channel_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.channel_type = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_barge_in_control`.\n"]
    pub fn set_disable_barge_in_control(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_barge_in_control = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_dtmf`.\n"]
    pub fn set_disable_dtmf(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_dtmf = Some(v.into());
        self
    }
    #[doc = "Set the field `persona_property`.\n"]
    pub fn set_persona_property(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElDefaultChannelProfileElPersonaPropertyEl>>,
    ) -> Self {
        self.persona_property = Some(v.into());
        self
    }
    #[doc = "Set the field `profile_id`.\n"]
    pub fn set_profile_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.profile_id = Some(v.into());
        self
    }
    #[doc = "Set the field `web_widget_config`.\n"]
    pub fn set_web_widget_config(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElDefaultChannelProfileElWebWidgetConfigEl>>,
    ) -> Self {
        self.web_widget_config = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElDefaultChannelProfileEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElDefaultChannelProfileEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElDefaultChannelProfileEl {}
impl BuildCesAppVersionSnapshotElAppElDefaultChannelProfileEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElDefaultChannelProfileEl {
        CesAppVersionSnapshotElAppElDefaultChannelProfileEl {
            channel_type: core::default::Default::default(),
            disable_barge_in_control: core::default::Default::default(),
            disable_dtmf: core::default::Default::default(),
            persona_property: core::default::Default::default(),
            profile_id: core::default::Default::default(),
            web_widget_config: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElDefaultChannelProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElDefaultChannelProfileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElDefaultChannelProfileElRef {
        CesAppVersionSnapshotElAppElDefaultChannelProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElDefaultChannelProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `channel_type` after provisioning.\n"]
    pub fn channel_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel_type", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_barge_in_control` after provisioning.\n"]
    pub fn disable_barge_in_control(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_barge_in_control", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_dtmf` after provisioning.\n"]
    pub fn disable_dtmf(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_dtmf", self.base))
    }
    #[doc = "Get a reference to the value of field `persona_property` after provisioning.\n"]
    pub fn persona_property(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAppElDefaultChannelProfileElPersonaPropertyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.persona_property", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `profile_id` after provisioning.\n"]
    pub fn profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.profile_id", self.base))
    }
    #[doc = "Get a reference to the value of field `web_widget_config` after provisioning.\n"]
    pub fn web_widget_config(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAppElDefaultChannelProfileElWebWidgetConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.web_widget_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_invocation_parameter_correctness_threshold: Option<PrimField<f64>>,
}
impl CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl { # [doc = "Set the field `tool_invocation_parameter_correctness_threshold`.\n"] pub fn set_tool_invocation_parameter_correctness_threshold (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . tool_invocation_parameter_correctness_threshold = Some (v . into ()) ; self } }
impl ToListMappable for CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl { type O = BlockAssignable < CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl
{}
impl BuildCesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl { pub fn build (self) -> CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl { CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl { tool_invocation_parameter_correctness_threshold : core :: default :: Default :: default () , } } }
pub struct CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsElRef { fn new (shared : StackShared , base : String) -> CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsElRef { CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsElRef { shared : shared , base : base . to_string () , } } }
impl CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `tool_invocation_parameter_correctness_threshold` after provisioning.\n"] pub fn tool_invocation_parameter_correctness_threshold (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.tool_invocation_parameter_correctness_threshold" , self . base)) } }
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    overall_tool_invocation_correctness_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    semantic_similarity_success_threshold: Option<PrimField<f64>>,
}
impl CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl { # [doc = "Set the field `overall_tool_invocation_correctness_threshold`.\n"] pub fn set_overall_tool_invocation_correctness_threshold (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . overall_tool_invocation_correctness_threshold = Some (v . into ()) ; self } # [doc = "Set the field `semantic_similarity_success_threshold`.\n"] pub fn set_semantic_similarity_success_threshold (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . semantic_similarity_success_threshold = Some (v . into ()) ; self } }
impl ToListMappable for CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl { type O = BlockAssignable < CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl
{}
impl BuildCesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl { pub fn build (self) -> CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl { CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl { overall_tool_invocation_correctness_threshold : core :: default :: Default :: default () , semantic_similarity_success_threshold : core :: default :: Default :: default () , } } }
pub struct CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsElRef { fn new (shared : StackShared , base : String) -> CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsElRef { CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsElRef { shared : shared , base : base . to_string () , } } }
impl CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `overall_tool_invocation_correctness_threshold` after provisioning.\n"] pub fn overall_tool_invocation_correctness_threshold (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.overall_tool_invocation_correctness_threshold" , self . base)) } # [doc = "Get a reference to the value of field `semantic_similarity_success_threshold` after provisioning.\n"] pub fn semantic_similarity_success_threshold (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.semantic_similarity_success_threshold" , self . base)) } }
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl { # [serde (skip_serializing_if = "Option::is_none")] expectation_level_metrics_thresholds : Option < ListField < CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl > > , # [serde (skip_serializing_if = "Option::is_none")] turn_level_metrics_thresholds : Option < ListField < CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl > > , }
impl CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl {
    #[doc = "Set the field `expectation_level_metrics_thresholds`.\n"]
    pub fn set_expectation_level_metrics_thresholds(
        mut self,
        v : impl Into < ListField < CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsEl > >,
    ) -> Self {
        self.expectation_level_metrics_thresholds = Some(v.into());
        self
    }
    #[doc = "Set the field `turn_level_metrics_thresholds`.\n"]
    pub fn set_turn_level_metrics_thresholds(
        mut self,
        v : impl Into < ListField < CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsEl > >,
    ) -> Self {
        self.turn_level_metrics_thresholds = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl
{
    type O = BlockAssignable < CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl
{}
impl BuildCesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl { pub fn build (self) -> CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl { CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl { expectation_level_metrics_thresholds : core :: default :: Default :: default () , turn_level_metrics_thresholds : core :: default :: Default :: default () , } } }
pub struct CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElRef { fn new (shared : StackShared , base : String) -> CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElRef { CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElRef { shared : shared , base : base . to_string () , } } }
impl
    CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expectation_level_metrics_thresholds` after provisioning.\n"]    pub fn expectation_level_metrics_thresholds (& self) -> ListRef < CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElExpectationLevelMetricsThresholdsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.expectation_level_metrics_thresholds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `turn_level_metrics_thresholds` after provisioning.\n"]    pub fn turn_level_metrics_thresholds (& self) -> ListRef < CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElTurnLevelMetricsThresholdsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.turn_level_metrics_thresholds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsEl { # [serde (skip_serializing_if = "Option::is_none")] golden_evaluation_metrics_thresholds : Option < ListField < CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl > > , }
impl CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsEl {
    #[doc = "Set the field `golden_evaluation_metrics_thresholds`.\n"]
    pub fn set_golden_evaluation_metrics_thresholds(
        mut self,
        v : impl Into < ListField < CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsEl > >,
    ) -> Self {
        self.golden_evaluation_metrics_thresholds = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElEvaluationMetricsThresholdsEl {}
impl BuildCesAppVersionSnapshotElAppElEvaluationMetricsThresholdsEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsEl {
        CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsEl {
            golden_evaluation_metrics_thresholds: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElRef {
        CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `golden_evaluation_metrics_thresholds` after provisioning.\n"]    pub fn golden_evaluation_metrics_thresholds (& self) -> ListRef < CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElGoldenEvaluationMetricsThresholdsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.golden_evaluation_metrics_thresholds", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElLanguageSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_language_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_multilingual_support: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fallback_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_language_codes: Option<ListField<PrimField<String>>>,
}
impl CesAppVersionSnapshotElAppElLanguageSettingsEl {
    #[doc = "Set the field `default_language_code`.\n"]
    pub fn set_default_language_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_multilingual_support`.\n"]
    pub fn set_enable_multilingual_support(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_multilingual_support = Some(v.into());
        self
    }
    #[doc = "Set the field `fallback_action`.\n"]
    pub fn set_fallback_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fallback_action = Some(v.into());
        self
    }
    #[doc = "Set the field `supported_language_codes`.\n"]
    pub fn set_supported_language_codes(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.supported_language_codes = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElLanguageSettingsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElLanguageSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElLanguageSettingsEl {}
impl BuildCesAppVersionSnapshotElAppElLanguageSettingsEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElLanguageSettingsEl {
        CesAppVersionSnapshotElAppElLanguageSettingsEl {
            default_language_code: core::default::Default::default(),
            enable_multilingual_support: core::default::Default::default(),
            fallback_action: core::default::Default::default(),
            supported_language_codes: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElLanguageSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElLanguageSettingsElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElAppElLanguageSettingsElRef {
        CesAppVersionSnapshotElAppElLanguageSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElLanguageSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_language_code` after provisioning.\n"]
    pub fn default_language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_language_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_multilingual_support` after provisioning.\n"]
    pub fn enable_multilingual_support(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_multilingual_support", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fallback_action` after provisioning.\n"]
    pub fn fallback_action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fallback_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `supported_language_codes` after provisioning.\n"]
    pub fn supported_language_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_language_codes", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElLoggingSettingsElAudioRecordingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_path_prefix: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElAppElLoggingSettingsElAudioRecordingConfigEl {
    #[doc = "Set the field `gcs_bucket`.\n"]
    pub fn set_gcs_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcs_bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs_path_prefix`.\n"]
    pub fn set_gcs_path_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcs_path_prefix = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElLoggingSettingsElAudioRecordingConfigEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElLoggingSettingsElAudioRecordingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElLoggingSettingsElAudioRecordingConfigEl {}
impl BuildCesAppVersionSnapshotElAppElLoggingSettingsElAudioRecordingConfigEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElLoggingSettingsElAudioRecordingConfigEl {
        CesAppVersionSnapshotElAppElLoggingSettingsElAudioRecordingConfigEl {
            gcs_bucket: core::default::Default::default(),
            gcs_path_prefix: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElLoggingSettingsElAudioRecordingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElLoggingSettingsElAudioRecordingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElLoggingSettingsElAudioRecordingConfigElRef {
        CesAppVersionSnapshotElAppElLoggingSettingsElAudioRecordingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElLoggingSettingsElAudioRecordingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gcs_bucket` after provisioning.\n"]
    pub fn gcs_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gcs_bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `gcs_path_prefix` after provisioning.\n"]
    pub fn gcs_path_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcs_path_prefix", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElLoggingSettingsElBigqueryExportSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElAppElLoggingSettingsElBigqueryExportSettingsEl {
    #[doc = "Set the field `dataset`.\n"]
    pub fn set_dataset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElLoggingSettingsElBigqueryExportSettingsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElLoggingSettingsElBigqueryExportSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElLoggingSettingsElBigqueryExportSettingsEl {}
impl BuildCesAppVersionSnapshotElAppElLoggingSettingsElBigqueryExportSettingsEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElLoggingSettingsElBigqueryExportSettingsEl {
        CesAppVersionSnapshotElAppElLoggingSettingsElBigqueryExportSettingsEl {
            dataset: core::default::Default::default(),
            enabled: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElLoggingSettingsElBigqueryExportSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElLoggingSettingsElBigqueryExportSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElLoggingSettingsElBigqueryExportSettingsElRef {
        CesAppVersionSnapshotElAppElLoggingSettingsElBigqueryExportSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElLoggingSettingsElBigqueryExportSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.base))
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElLoggingSettingsElCloudLoggingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_cloud_logging: Option<PrimField<bool>>,
}
impl CesAppVersionSnapshotElAppElLoggingSettingsElCloudLoggingSettingsEl {
    #[doc = "Set the field `enable_cloud_logging`.\n"]
    pub fn set_enable_cloud_logging(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_cloud_logging = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElLoggingSettingsElCloudLoggingSettingsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElLoggingSettingsElCloudLoggingSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElLoggingSettingsElCloudLoggingSettingsEl {}
impl BuildCesAppVersionSnapshotElAppElLoggingSettingsElCloudLoggingSettingsEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElLoggingSettingsElCloudLoggingSettingsEl {
        CesAppVersionSnapshotElAppElLoggingSettingsElCloudLoggingSettingsEl {
            enable_cloud_logging: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElLoggingSettingsElCloudLoggingSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElLoggingSettingsElCloudLoggingSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElLoggingSettingsElCloudLoggingSettingsElRef {
        CesAppVersionSnapshotElAppElLoggingSettingsElCloudLoggingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElLoggingSettingsElCloudLoggingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_cloud_logging` after provisioning.\n"]
    pub fn enable_cloud_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_cloud_logging", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElLoggingSettingsElConversationLoggingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_conversation_logging: Option<PrimField<bool>>,
}
impl CesAppVersionSnapshotElAppElLoggingSettingsElConversationLoggingSettingsEl {
    #[doc = "Set the field `disable_conversation_logging`.\n"]
    pub fn set_disable_conversation_logging(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_conversation_logging = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElLoggingSettingsElConversationLoggingSettingsEl {
    type O =
        BlockAssignable<CesAppVersionSnapshotElAppElLoggingSettingsElConversationLoggingSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElLoggingSettingsElConversationLoggingSettingsEl {}
impl BuildCesAppVersionSnapshotElAppElLoggingSettingsElConversationLoggingSettingsEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElAppElLoggingSettingsElConversationLoggingSettingsEl {
        CesAppVersionSnapshotElAppElLoggingSettingsElConversationLoggingSettingsEl {
            disable_conversation_logging: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElLoggingSettingsElConversationLoggingSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElLoggingSettingsElConversationLoggingSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElLoggingSettingsElConversationLoggingSettingsElRef {
        CesAppVersionSnapshotElAppElLoggingSettingsElConversationLoggingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElLoggingSettingsElConversationLoggingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_conversation_logging` after provisioning.\n"]
    pub fn disable_conversation_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_conversation_logging", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElLoggingSettingsElRedactionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deidentify_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_redaction: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inspect_template: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElAppElLoggingSettingsElRedactionConfigEl {
    #[doc = "Set the field `deidentify_template`.\n"]
    pub fn set_deidentify_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deidentify_template = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_redaction`.\n"]
    pub fn set_enable_redaction(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_redaction = Some(v.into());
        self
    }
    #[doc = "Set the field `inspect_template`.\n"]
    pub fn set_inspect_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.inspect_template = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElLoggingSettingsElRedactionConfigEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElLoggingSettingsElRedactionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElLoggingSettingsElRedactionConfigEl {}
impl BuildCesAppVersionSnapshotElAppElLoggingSettingsElRedactionConfigEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElLoggingSettingsElRedactionConfigEl {
        CesAppVersionSnapshotElAppElLoggingSettingsElRedactionConfigEl {
            deidentify_template: core::default::Default::default(),
            enable_redaction: core::default::Default::default(),
            inspect_template: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElLoggingSettingsElRedactionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElLoggingSettingsElRedactionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElLoggingSettingsElRedactionConfigElRef {
        CesAppVersionSnapshotElAppElLoggingSettingsElRedactionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElLoggingSettingsElRedactionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deidentify_template` after provisioning.\n"]
    pub fn deidentify_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deidentify_template", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_redaction` after provisioning.\n"]
    pub fn enable_redaction(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_redaction", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inspect_template` after provisioning.\n"]
    pub fn inspect_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inspect_template", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElLoggingSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_recording_config:
        Option<ListField<CesAppVersionSnapshotElAppElLoggingSettingsElAudioRecordingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_export_settings:
        Option<ListField<CesAppVersionSnapshotElAppElLoggingSettingsElBigqueryExportSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_logging_settings:
        Option<ListField<CesAppVersionSnapshotElAppElLoggingSettingsElCloudLoggingSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_logging_settings: Option<
        ListField<CesAppVersionSnapshotElAppElLoggingSettingsElConversationLoggingSettingsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    redaction_config:
        Option<ListField<CesAppVersionSnapshotElAppElLoggingSettingsElRedactionConfigEl>>,
}
impl CesAppVersionSnapshotElAppElLoggingSettingsEl {
    #[doc = "Set the field `audio_recording_config`.\n"]
    pub fn set_audio_recording_config(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElLoggingSettingsElAudioRecordingConfigEl>>,
    ) -> Self {
        self.audio_recording_config = Some(v.into());
        self
    }
    #[doc = "Set the field `bigquery_export_settings`.\n"]
    pub fn set_bigquery_export_settings(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElLoggingSettingsElBigqueryExportSettingsEl>>,
    ) -> Self {
        self.bigquery_export_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `cloud_logging_settings`.\n"]
    pub fn set_cloud_logging_settings(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElLoggingSettingsElCloudLoggingSettingsEl>>,
    ) -> Self {
        self.cloud_logging_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `conversation_logging_settings`.\n"]
    pub fn set_conversation_logging_settings(
        mut self,
        v: impl Into<
            ListField<CesAppVersionSnapshotElAppElLoggingSettingsElConversationLoggingSettingsEl>,
        >,
    ) -> Self {
        self.conversation_logging_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `redaction_config`.\n"]
    pub fn set_redaction_config(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElLoggingSettingsElRedactionConfigEl>>,
    ) -> Self {
        self.redaction_config = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElLoggingSettingsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElLoggingSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElLoggingSettingsEl {}
impl BuildCesAppVersionSnapshotElAppElLoggingSettingsEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElLoggingSettingsEl {
        CesAppVersionSnapshotElAppElLoggingSettingsEl {
            audio_recording_config: core::default::Default::default(),
            bigquery_export_settings: core::default::Default::default(),
            cloud_logging_settings: core::default::Default::default(),
            conversation_logging_settings: core::default::Default::default(),
            redaction_config: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElLoggingSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElLoggingSettingsElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElAppElLoggingSettingsElRef {
        CesAppVersionSnapshotElAppElLoggingSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElLoggingSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audio_recording_config` after provisioning.\n"]
    pub fn audio_recording_config(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAppElLoggingSettingsElAudioRecordingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_recording_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bigquery_export_settings` after provisioning.\n"]
    pub fn bigquery_export_settings(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAppElLoggingSettingsElBigqueryExportSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_export_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_logging_settings` after provisioning.\n"]
    pub fn cloud_logging_settings(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAppElLoggingSettingsElCloudLoggingSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_logging_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `conversation_logging_settings` after provisioning.\n"]
    pub fn conversation_logging_settings(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAppElLoggingSettingsElConversationLoggingSettingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conversation_logging_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `redaction_config` after provisioning.\n"]
    pub fn redaction_config(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAppElLoggingSettingsElRedactionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redaction_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElModelSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
}
impl CesAppVersionSnapshotElAppElModelSettingsEl {
    #[doc = "Set the field `model`.\n"]
    pub fn set_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model = Some(v.into());
        self
    }
    #[doc = "Set the field `temperature`.\n"]
    pub fn set_temperature(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.temperature = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElModelSettingsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElModelSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElModelSettingsEl {}
impl BuildCesAppVersionSnapshotElAppElModelSettingsEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElModelSettingsEl {
        CesAppVersionSnapshotElAppElModelSettingsEl {
            model: core::default::Default::default(),
            temperature: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElModelSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElModelSettingsElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElAppElModelSettingsElRef {
        CesAppVersionSnapshotElAppElModelSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElModelSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }
    #[doc = "Get a reference to the value of field `temperature` after provisioning.\n"]
    pub fn temperature(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElTimeZoneSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElAppElTimeZoneSettingsEl {
    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_zone = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElTimeZoneSettingsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElTimeZoneSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElTimeZoneSettingsEl {}
impl BuildCesAppVersionSnapshotElAppElTimeZoneSettingsEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElTimeZoneSettingsEl {
        CesAppVersionSnapshotElAppElTimeZoneSettingsEl {
            time_zone: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElTimeZoneSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElTimeZoneSettingsElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElAppElTimeZoneSettingsElRef {
        CesAppVersionSnapshotElAppElTimeZoneSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElTimeZoneSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElVariableDeclarationsElSchemaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_properties: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    any_of: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    defs: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    enum_: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nullable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_items: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<PrimField<String>>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    ref_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<ListField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_items: Option<PrimField<bool>>,
}
impl CesAppVersionSnapshotElAppElVariableDeclarationsElSchemaEl {
    #[doc = "Set the field `additional_properties`.\n"]
    pub fn set_additional_properties(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.additional_properties = Some(v.into());
        self
    }
    #[doc = "Set the field `any_of`.\n"]
    pub fn set_any_of(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.any_of = Some(v.into());
        self
    }
    #[doc = "Set the field `default`.\n"]
    pub fn set_default(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default = Some(v.into());
        self
    }
    #[doc = "Set the field `defs`.\n"]
    pub fn set_defs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.defs = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `enum_`.\n"]
    pub fn set_enum(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.enum_ = Some(v.into());
        self
    }
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.items = Some(v.into());
        self
    }
    #[doc = "Set the field `nullable`.\n"]
    pub fn set_nullable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.nullable = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_items`.\n"]
    pub fn set_prefix_items(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_items = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `ref_`.\n"]
    pub fn set_ref(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ref_ = Some(v.into());
        self
    }
    #[doc = "Set the field `required`.\n"]
    pub fn set_required(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.required = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `unique_items`.\n"]
    pub fn set_unique_items(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.unique_items = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElVariableDeclarationsElSchemaEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElVariableDeclarationsElSchemaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElVariableDeclarationsElSchemaEl {}
impl BuildCesAppVersionSnapshotElAppElVariableDeclarationsElSchemaEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElVariableDeclarationsElSchemaEl {
        CesAppVersionSnapshotElAppElVariableDeclarationsElSchemaEl {
            additional_properties: core::default::Default::default(),
            any_of: core::default::Default::default(),
            default: core::default::Default::default(),
            defs: core::default::Default::default(),
            description: core::default::Default::default(),
            enum_: core::default::Default::default(),
            items: core::default::Default::default(),
            nullable: core::default::Default::default(),
            prefix_items: core::default::Default::default(),
            properties: core::default::Default::default(),
            ref_: core::default::Default::default(),
            required: core::default::Default::default(),
            type_: core::default::Default::default(),
            unique_items: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElVariableDeclarationsElSchemaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElVariableDeclarationsElSchemaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElVariableDeclarationsElSchemaElRef {
        CesAppVersionSnapshotElAppElVariableDeclarationsElSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElVariableDeclarationsElSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_properties` after provisioning.\n"]
    pub fn additional_properties(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `any_of` after provisioning.\n"]
    pub fn any_of(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.any_of", self.base))
    }
    #[doc = "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }
    #[doc = "Get a reference to the value of field `defs` after provisioning.\n"]
    pub fn defs(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.defs", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `enum_` after provisioning.\n"]
    pub fn enum_(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.enum", self.base))
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.items", self.base))
    }
    #[doc = "Get a reference to the value of field `nullable` after provisioning.\n"]
    pub fn nullable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix_items` after provisioning.\n"]
    pub fn prefix_items(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_items", self.base))
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `ref_` after provisioning.\n"]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }
    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.required", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `unique_items` after provisioning.\n"]
    pub fn unique_items(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.unique_items", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppElVariableDeclarationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<ListField<CesAppVersionSnapshotElAppElVariableDeclarationsElSchemaEl>>,
}
impl CesAppVersionSnapshotElAppElVariableDeclarationsEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElVariableDeclarationsElSchemaEl>>,
    ) -> Self {
        self.schema = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppElVariableDeclarationsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppElVariableDeclarationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppElVariableDeclarationsEl {}
impl BuildCesAppVersionSnapshotElAppElVariableDeclarationsEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppElVariableDeclarationsEl {
        CesAppVersionSnapshotElAppElVariableDeclarationsEl {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            schema: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElVariableDeclarationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElVariableDeclarationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElAppElVariableDeclarationsElRef {
        CesAppVersionSnapshotElAppElVariableDeclarationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElVariableDeclarationsElRef {
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
    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> ListRef<CesAppVersionSnapshotElAppElVariableDeclarationsElSchemaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElAppEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_processing_config: Option<ListField<CesAppVersionSnapshotElAppElAudioProcessingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate_settings:
        Option<ListField<CesAppVersionSnapshotElAppElClientCertificateSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_settings: Option<ListField<CesAppVersionSnapshotElAppElDataStoreSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_channel_profile: Option<ListField<CesAppVersionSnapshotElAppElDefaultChannelProfileEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluation_metrics_thresholds:
        Option<ListField<CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_instruction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guardrails: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_settings: Option<ListField<CesAppVersionSnapshotElAppElLanguageSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_settings: Option<ListField<CesAppVersionSnapshotElAppElLoggingSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_settings: Option<ListField<CesAppVersionSnapshotElAppElModelSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_agent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone_settings: Option<ListField<CesAppVersionSnapshotElAppElTimeZoneSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variable_declarations: Option<ListField<CesAppVersionSnapshotElAppElVariableDeclarationsEl>>,
}
impl CesAppVersionSnapshotElAppEl {
    #[doc = "Set the field `audio_processing_config`.\n"]
    pub fn set_audio_processing_config(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElAudioProcessingConfigEl>>,
    ) -> Self {
        self.audio_processing_config = Some(v.into());
        self
    }
    #[doc = "Set the field `client_certificate_settings`.\n"]
    pub fn set_client_certificate_settings(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElClientCertificateSettingsEl>>,
    ) -> Self {
        self.client_certificate_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `data_store_settings`.\n"]
    pub fn set_data_store_settings(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElDataStoreSettingsEl>>,
    ) -> Self {
        self.data_store_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `default_channel_profile`.\n"]
    pub fn set_default_channel_profile(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElDefaultChannelProfileEl>>,
    ) -> Self {
        self.default_channel_profile = Some(v.into());
        self
    }
    #[doc = "Set the field `deployment_count`.\n"]
    pub fn set_deployment_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.deployment_count = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.etag = Some(v.into());
        self
    }
    #[doc = "Set the field `evaluation_metrics_thresholds`.\n"]
    pub fn set_evaluation_metrics_thresholds(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsEl>>,
    ) -> Self {
        self.evaluation_metrics_thresholds = Some(v.into());
        self
    }
    #[doc = "Set the field `global_instruction`.\n"]
    pub fn set_global_instruction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.global_instruction = Some(v.into());
        self
    }
    #[doc = "Set the field `guardrails`.\n"]
    pub fn set_guardrails(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.guardrails = Some(v.into());
        self
    }
    #[doc = "Set the field `language_settings`.\n"]
    pub fn set_language_settings(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElLanguageSettingsEl>>,
    ) -> Self {
        self.language_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `logging_settings`.\n"]
    pub fn set_logging_settings(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElLoggingSettingsEl>>,
    ) -> Self {
        self.logging_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElModelSettingsEl>>,
    ) -> Self {
        self.model_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `root_agent`.\n"]
    pub fn set_root_agent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.root_agent = Some(v.into());
        self
    }
    #[doc = "Set the field `time_zone_settings`.\n"]
    pub fn set_time_zone_settings(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElTimeZoneSettingsEl>>,
    ) -> Self {
        self.time_zone_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
    #[doc = "Set the field `variable_declarations`.\n"]
    pub fn set_variable_declarations(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElAppElVariableDeclarationsEl>>,
    ) -> Self {
        self.variable_declarations = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElAppEl {
    type O = BlockAssignable<CesAppVersionSnapshotElAppEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElAppEl {}
impl BuildCesAppVersionSnapshotElAppEl {
    pub fn build(self) -> CesAppVersionSnapshotElAppEl {
        CesAppVersionSnapshotElAppEl {
            audio_processing_config: core::default::Default::default(),
            client_certificate_settings: core::default::Default::default(),
            create_time: core::default::Default::default(),
            data_store_settings: core::default::Default::default(),
            default_channel_profile: core::default::Default::default(),
            deployment_count: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            etag: core::default::Default::default(),
            evaluation_metrics_thresholds: core::default::Default::default(),
            global_instruction: core::default::Default::default(),
            guardrails: core::default::Default::default(),
            language_settings: core::default::Default::default(),
            logging_settings: core::default::Default::default(),
            metadata: core::default::Default::default(),
            model_settings: core::default::Default::default(),
            name: core::default::Default::default(),
            root_agent: core::default::Default::default(),
            time_zone_settings: core::default::Default::default(),
            update_time: core::default::Default::default(),
            variable_declarations: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElAppElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElAppElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElAppElRef {
        CesAppVersionSnapshotElAppElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElAppElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audio_processing_config` after provisioning.\n"]
    pub fn audio_processing_config(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAppElAudioProcessingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.audio_processing_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_certificate_settings` after provisioning.\n"]
    pub fn client_certificate_settings(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAppElClientCertificateSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_certificate_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `data_store_settings` after provisioning.\n"]
    pub fn data_store_settings(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAppElDataStoreSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_channel_profile` after provisioning.\n"]
    pub fn default_channel_profile(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAppElDefaultChannelProfileElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_channel_profile", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_count` after provisioning.\n"]
    pub fn deployment_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }
    #[doc = "Get a reference to the value of field `evaluation_metrics_thresholds` after provisioning.\n"]
    pub fn evaluation_metrics_thresholds(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAppElEvaluationMetricsThresholdsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.evaluation_metrics_thresholds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `global_instruction` after provisioning.\n"]
    pub fn global_instruction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_instruction", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `guardrails` after provisioning.\n"]
    pub fn guardrails(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.guardrails", self.base))
    }
    #[doc = "Get a reference to the value of field `language_settings` after provisioning.\n"]
    pub fn language_settings(&self) -> ListRef<CesAppVersionSnapshotElAppElLanguageSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.language_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logging_settings` after provisioning.\n"]
    pub fn logging_settings(&self) -> ListRef<CesAppVersionSnapshotElAppElLoggingSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> ListRef<CesAppVersionSnapshotElAppElModelSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `root_agent` after provisioning.\n"]
    pub fn root_agent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_agent", self.base))
    }
    #[doc = "Get a reference to the value of field `time_zone_settings` after provisioning.\n"]
    pub fn time_zone_settings(&self) -> ListRef<CesAppVersionSnapshotElAppElTimeZoneSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.time_zone_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
    #[doc = "Get a reference to the value of field `variable_declarations` after provisioning.\n"]
    pub fn variable_declarations(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElAppElVariableDeclarationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.variable_declarations", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElExamplesElMessagesElChunksElAgentTransferEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_agent: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElExamplesElMessagesElChunksElAgentTransferEl {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `target_agent`.\n"]
    pub fn set_target_agent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_agent = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElExamplesElMessagesElChunksElAgentTransferEl {
    type O = BlockAssignable<CesAppVersionSnapshotElExamplesElMessagesElChunksElAgentTransferEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElExamplesElMessagesElChunksElAgentTransferEl {}
impl BuildCesAppVersionSnapshotElExamplesElMessagesElChunksElAgentTransferEl {
    pub fn build(self) -> CesAppVersionSnapshotElExamplesElMessagesElChunksElAgentTransferEl {
        CesAppVersionSnapshotElExamplesElMessagesElChunksElAgentTransferEl {
            display_name: core::default::Default::default(),
            target_agent: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElExamplesElMessagesElChunksElAgentTransferElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElExamplesElMessagesElChunksElAgentTransferElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElExamplesElMessagesElChunksElAgentTransferElRef {
        CesAppVersionSnapshotElExamplesElMessagesElChunksElAgentTransferElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElExamplesElMessagesElChunksElAgentTransferElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `target_agent` after provisioning.\n"]
    pub fn target_agent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_agent", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElExamplesElMessagesElChunksElImageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mime_type: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElExamplesElMessagesElChunksElImageEl {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data = Some(v.into());
        self
    }
    #[doc = "Set the field `mime_type`.\n"]
    pub fn set_mime_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mime_type = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElExamplesElMessagesElChunksElImageEl {
    type O = BlockAssignable<CesAppVersionSnapshotElExamplesElMessagesElChunksElImageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElExamplesElMessagesElChunksElImageEl {}
impl BuildCesAppVersionSnapshotElExamplesElMessagesElChunksElImageEl {
    pub fn build(self) -> CesAppVersionSnapshotElExamplesElMessagesElChunksElImageEl {
        CesAppVersionSnapshotElExamplesElMessagesElChunksElImageEl {
            data: core::default::Default::default(),
            mime_type: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElExamplesElMessagesElChunksElImageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElExamplesElMessagesElChunksElImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElExamplesElMessagesElChunksElImageElRef {
        CesAppVersionSnapshotElExamplesElMessagesElChunksElImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElExamplesElMessagesElChunksElImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }
    #[doc = "Get a reference to the value of field `mime_type` after provisioning.\n"]
    pub fn mime_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mime_type", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElToolsetToolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toolset: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElToolsetToolEl {
    #[doc = "Set the field `tool_id`.\n"]
    pub fn set_tool_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tool_id = Some(v.into());
        self
    }
    #[doc = "Set the field `toolset`.\n"]
    pub fn set_toolset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.toolset = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElToolsetToolEl {
    type O =
        BlockAssignable<CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElToolsetToolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElToolsetToolEl {}
impl BuildCesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElToolsetToolEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElToolsetToolEl {
        CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElToolsetToolEl {
            tool_id: core::default::Default::default(),
            toolset: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElToolsetToolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElToolsetToolElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElToolsetToolElRef {
        CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElToolsetToolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElToolsetToolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tool_id` after provisioning.\n"]
    pub fn tool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tool_id", self.base))
    }
    #[doc = "Get a reference to the value of field `toolset` after provisioning.\n"]
    pub fn toolset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.toolset", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toolset_tool: Option<
        ListField<CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElToolsetToolEl>,
    >,
}
impl CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallEl {
    #[doc = "Set the field `args`.\n"]
    pub fn set_args(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.args = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `tool`.\n"]
    pub fn set_tool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tool = Some(v.into());
        self
    }
    #[doc = "Set the field `toolset_tool`.\n"]
    pub fn set_toolset_tool(
        mut self,
        v: impl Into<
            ListField<CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElToolsetToolEl>,
        >,
    ) -> Self {
        self.toolset_tool = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallEl {
    type O = BlockAssignable<CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallEl {}
impl BuildCesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallEl {
    pub fn build(self) -> CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallEl {
        CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallEl {
            args: core::default::Default::default(),
            display_name: core::default::Default::default(),
            id: core::default::Default::default(),
            tool: core::default::Default::default(),
            toolset_tool: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElRef {
        CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.args", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `tool` after provisioning.\n"]
    pub fn tool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tool", self.base))
    }
    #[doc = "Get a reference to the value of field `toolset_tool` after provisioning.\n"]
    pub fn toolset_tool(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElToolsetToolElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.toolset_tool", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElToolsetToolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toolset: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElToolsetToolEl {
    #[doc = "Set the field `tool_id`.\n"]
    pub fn set_tool_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tool_id = Some(v.into());
        self
    }
    #[doc = "Set the field `toolset`.\n"]
    pub fn set_toolset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.toolset = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElToolsetToolEl
{
    type O = BlockAssignable<
        CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElToolsetToolEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElToolsetToolEl {}
impl BuildCesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElToolsetToolEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElToolsetToolEl {
        CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElToolsetToolEl {
            tool_id: core::default::Default::default(),
            toolset: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElToolsetToolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElToolsetToolElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElToolsetToolElRef {
        CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElToolsetToolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElToolsetToolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tool_id` after provisioning.\n"]
    pub fn tool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tool_id", self.base))
    }
    #[doc = "Get a reference to the value of field `toolset` after provisioning.\n"]
    pub fn toolset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.toolset", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toolset_tool: Option<
        ListField<CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElToolsetToolEl>,
    >,
}
impl CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseEl {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `response`.\n"]
    pub fn set_response(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response = Some(v.into());
        self
    }
    #[doc = "Set the field `tool`.\n"]
    pub fn set_tool(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tool = Some(v.into());
        self
    }
    #[doc = "Set the field `toolset_tool`.\n"]
    pub fn set_toolset_tool(
        mut self,
        v: impl Into<
            ListField<
                CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElToolsetToolEl,
            >,
        >,
    ) -> Self {
        self.toolset_tool = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseEl {
    type O = BlockAssignable<CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseEl {}
impl BuildCesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseEl {
    pub fn build(self) -> CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseEl {
        CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseEl {
            display_name: core::default::Default::default(),
            id: core::default::Default::default(),
            response: core::default::Default::default(),
            tool: core::default::Default::default(),
            toolset_tool: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElRef {
        CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
    #[doc = "Get a reference to the value of field `tool` after provisioning.\n"]
    pub fn tool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tool", self.base))
    }
    #[doc = "Get a reference to the value of field `toolset_tool` after provisioning.\n"]
    pub fn toolset_tool(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElToolsetToolElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.toolset_tool", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElExamplesElMessagesElChunksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_transfer:
        Option<ListField<CesAppVersionSnapshotElExamplesElMessagesElChunksElAgentTransferEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<ListField<CesAppVersionSnapshotElExamplesElMessagesElChunksElImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call: Option<ListField<CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_response:
        Option<ListField<CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_variables: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElExamplesElMessagesElChunksEl {
    #[doc = "Set the field `agent_transfer`.\n"]
    pub fn set_agent_transfer(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElExamplesElMessagesElChunksElAgentTransferEl>>,
    ) -> Self {
        self.agent_transfer = Some(v.into());
        self
    }
    #[doc = "Set the field `image`.\n"]
    pub fn set_image(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElExamplesElMessagesElChunksElImageEl>>,
    ) -> Self {
        self.image = Some(v.into());
        self
    }
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }
    #[doc = "Set the field `tool_call`.\n"]
    pub fn set_tool_call(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallEl>>,
    ) -> Self {
        self.tool_call = Some(v.into());
        self
    }
    #[doc = "Set the field `tool_response`.\n"]
    pub fn set_tool_response(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseEl>>,
    ) -> Self {
        self.tool_response = Some(v.into());
        self
    }
    #[doc = "Set the field `updated_variables`.\n"]
    pub fn set_updated_variables(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.updated_variables = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElExamplesElMessagesElChunksEl {
    type O = BlockAssignable<CesAppVersionSnapshotElExamplesElMessagesElChunksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElExamplesElMessagesElChunksEl {}
impl BuildCesAppVersionSnapshotElExamplesElMessagesElChunksEl {
    pub fn build(self) -> CesAppVersionSnapshotElExamplesElMessagesElChunksEl {
        CesAppVersionSnapshotElExamplesElMessagesElChunksEl {
            agent_transfer: core::default::Default::default(),
            image: core::default::Default::default(),
            text: core::default::Default::default(),
            tool_call: core::default::Default::default(),
            tool_response: core::default::Default::default(),
            updated_variables: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElExamplesElMessagesElChunksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElExamplesElMessagesElChunksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElExamplesElMessagesElChunksElRef {
        CesAppVersionSnapshotElExamplesElMessagesElChunksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElExamplesElMessagesElChunksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `agent_transfer` after provisioning.\n"]
    pub fn agent_transfer(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElExamplesElMessagesElChunksElAgentTransferElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.agent_transfer", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> ListRef<CesAppVersionSnapshotElExamplesElMessagesElChunksElImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
    #[doc = "Get a reference to the value of field `tool_call` after provisioning.\n"]
    pub fn tool_call(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElExamplesElMessagesElChunksElToolCallElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tool_call", self.base))
    }
    #[doc = "Get a reference to the value of field `tool_response` after provisioning.\n"]
    pub fn tool_response(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElExamplesElMessagesElChunksElToolResponseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tool_response", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `updated_variables` after provisioning.\n"]
    pub fn updated_variables(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_variables", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElExamplesElMessagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    chunks: Option<ListField<CesAppVersionSnapshotElExamplesElMessagesElChunksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElExamplesElMessagesEl {
    #[doc = "Set the field `chunks`.\n"]
    pub fn set_chunks(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElExamplesElMessagesElChunksEl>>,
    ) -> Self {
        self.chunks = Some(v.into());
        self
    }
    #[doc = "Set the field `role`.\n"]
    pub fn set_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElExamplesElMessagesEl {
    type O = BlockAssignable<CesAppVersionSnapshotElExamplesElMessagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElExamplesElMessagesEl {}
impl BuildCesAppVersionSnapshotElExamplesElMessagesEl {
    pub fn build(self) -> CesAppVersionSnapshotElExamplesElMessagesEl {
        CesAppVersionSnapshotElExamplesElMessagesEl {
            chunks: core::default::Default::default(),
            role: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElExamplesElMessagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElExamplesElMessagesElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElExamplesElMessagesElRef {
        CesAppVersionSnapshotElExamplesElMessagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElExamplesElMessagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `chunks` after provisioning.\n"]
    pub fn chunks(&self) -> ListRef<CesAppVersionSnapshotElExamplesElMessagesElChunksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.chunks", self.base))
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElExamplesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_agent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invalid: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    messages: Option<ListField<CesAppVersionSnapshotElExamplesElMessagesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElExamplesEl {
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `entry_agent`.\n"]
    pub fn set_entry_agent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entry_agent = Some(v.into());
        self
    }
    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.etag = Some(v.into());
        self
    }
    #[doc = "Set the field `invalid`.\n"]
    pub fn set_invalid(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invalid = Some(v.into());
        self
    }
    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElExamplesElMessagesEl>>,
    ) -> Self {
        self.messages = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElExamplesEl {
    type O = BlockAssignable<CesAppVersionSnapshotElExamplesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElExamplesEl {}
impl BuildCesAppVersionSnapshotElExamplesEl {
    pub fn build(self) -> CesAppVersionSnapshotElExamplesEl {
        CesAppVersionSnapshotElExamplesEl {
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            entry_agent: core::default::Default::default(),
            etag: core::default::Default::default(),
            invalid: core::default::Default::default(),
            messages: core::default::Default::default(),
            name: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElExamplesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElExamplesElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElExamplesElRef {
        CesAppVersionSnapshotElExamplesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElExamplesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `entry_agent` after provisioning.\n"]
    pub fn entry_agent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entry_agent", self.base))
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }
    #[doc = "Get a reference to the value of field `invalid` after provisioning.\n"]
    pub fn invalid(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invalid", self.base))
    }
    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(&self) -> ListRef<CesAppVersionSnapshotElExamplesElMessagesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElActionElGenerativeAnswerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElGuardrailsElActionElGenerativeAnswerEl {
    #[doc = "Set the field `prompt`.\n"]
    pub fn set_prompt(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prompt = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElActionElGenerativeAnswerEl {
    type O = BlockAssignable<CesAppVersionSnapshotElGuardrailsElActionElGenerativeAnswerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElActionElGenerativeAnswerEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElActionElGenerativeAnswerEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsElActionElGenerativeAnswerEl {
        CesAppVersionSnapshotElGuardrailsElActionElGenerativeAnswerEl {
            prompt: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElActionElGenerativeAnswerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElActionElGenerativeAnswerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElGuardrailsElActionElGenerativeAnswerElRef {
        CesAppVersionSnapshotElGuardrailsElActionElGenerativeAnswerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElActionElGenerativeAnswerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElResponsesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElResponsesEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElResponsesEl {
    type O =
        BlockAssignable<CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElResponsesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElResponsesEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElResponsesEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElResponsesEl {
        CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElResponsesEl {
            disabled: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElResponsesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElResponsesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElResponsesElRef {
        CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElResponsesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElResponsesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    responses: Option<
        ListField<CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElResponsesEl>,
    >,
}
impl CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyEl {
    #[doc = "Set the field `responses`.\n"]
    pub fn set_responses(
        mut self,
        v: impl Into<
            ListField<CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElResponsesEl>,
        >,
    ) -> Self {
        self.responses = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyEl {
    type O = BlockAssignable<CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyEl {
        CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyEl {
            responses: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElRef {
        CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `responses` after provisioning.\n"]
    pub fn responses(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElResponsesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.responses", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElActionElTransferAgentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    agent: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElGuardrailsElActionElTransferAgentEl {
    #[doc = "Set the field `agent`.\n"]
    pub fn set_agent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.agent = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElActionElTransferAgentEl {
    type O = BlockAssignable<CesAppVersionSnapshotElGuardrailsElActionElTransferAgentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElActionElTransferAgentEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElActionElTransferAgentEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsElActionElTransferAgentEl {
        CesAppVersionSnapshotElGuardrailsElActionElTransferAgentEl {
            agent: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElActionElTransferAgentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElActionElTransferAgentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElGuardrailsElActionElTransferAgentElRef {
        CesAppVersionSnapshotElGuardrailsElActionElTransferAgentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElActionElTransferAgentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `agent` after provisioning.\n"]
    pub fn agent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    generative_answer:
        Option<ListField<CesAppVersionSnapshotElGuardrailsElActionElGenerativeAnswerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    respond_immediately:
        Option<ListField<CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_agent: Option<ListField<CesAppVersionSnapshotElGuardrailsElActionElTransferAgentEl>>,
}
impl CesAppVersionSnapshotElGuardrailsElActionEl {
    #[doc = "Set the field `generative_answer`.\n"]
    pub fn set_generative_answer(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsElActionElGenerativeAnswerEl>>,
    ) -> Self {
        self.generative_answer = Some(v.into());
        self
    }
    #[doc = "Set the field `respond_immediately`.\n"]
    pub fn set_respond_immediately(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyEl>>,
    ) -> Self {
        self.respond_immediately = Some(v.into());
        self
    }
    #[doc = "Set the field `transfer_agent`.\n"]
    pub fn set_transfer_agent(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsElActionElTransferAgentEl>>,
    ) -> Self {
        self.transfer_agent = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElActionEl {
    type O = BlockAssignable<CesAppVersionSnapshotElGuardrailsElActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElActionEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElActionEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsElActionEl {
        CesAppVersionSnapshotElGuardrailsElActionEl {
            generative_answer: core::default::Default::default(),
            respond_immediately: core::default::Default::default(),
            transfer_agent: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElActionElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElGuardrailsElActionElRef {
        CesAppVersionSnapshotElGuardrailsElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `generative_answer` after provisioning.\n"]
    pub fn generative_answer(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElGuardrailsElActionElGenerativeAnswerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.generative_answer", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `respond_immediately` after provisioning.\n"]
    pub fn respond_immediately(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElGuardrailsElActionElRespondImmediatelyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.respond_immediately", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `transfer_agent` after provisioning.\n"]
    pub fn transfer_agent(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElGuardrailsElActionElTransferAgentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.transfer_agent", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterAgentCallbackEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_code: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterAgentCallbackEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.python_code = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterAgentCallbackEl {
    type O = BlockAssignable<CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterAgentCallbackEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterAgentCallbackEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterAgentCallbackEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterAgentCallbackEl {
        CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterAgentCallbackEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterAgentCallbackElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterAgentCallbackElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterAgentCallbackElRef {
        CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterAgentCallbackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterAgentCallbackElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterModelCallbackEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_code: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterModelCallbackEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.python_code = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterModelCallbackEl {
    type O = BlockAssignable<CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterModelCallbackEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterModelCallbackEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterModelCallbackEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterModelCallbackEl {
        CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterModelCallbackEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterModelCallbackElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterModelCallbackElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterModelCallbackElRef {
        CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterModelCallbackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterModelCallbackElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeAgentCallbackEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_code: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeAgentCallbackEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.python_code = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeAgentCallbackEl {
    type O =
        BlockAssignable<CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeAgentCallbackEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeAgentCallbackEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeAgentCallbackEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeAgentCallbackEl {
        CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeAgentCallbackEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeAgentCallbackElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeAgentCallbackElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeAgentCallbackElRef {
        CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeAgentCallbackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeAgentCallbackElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeModelCallbackEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_code: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeModelCallbackEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.python_code = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeModelCallbackEl {
    type O =
        BlockAssignable<CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeModelCallbackEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeModelCallbackEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeModelCallbackEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeModelCallbackEl {
        CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeModelCallbackEl {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeModelCallbackElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeModelCallbackElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeModelCallbackElRef {
        CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeModelCallbackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeModelCallbackElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElCodeCallbackEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    after_agent_callback:
        Option<ListField<CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterAgentCallbackEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after_model_callback:
        Option<ListField<CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterModelCallbackEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before_agent_callback:
        Option<ListField<CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeAgentCallbackEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before_model_callback:
        Option<ListField<CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeModelCallbackEl>>,
}
impl CesAppVersionSnapshotElGuardrailsElCodeCallbackEl {
    #[doc = "Set the field `after_agent_callback`.\n"]
    pub fn set_after_agent_callback(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterAgentCallbackEl>>,
    ) -> Self {
        self.after_agent_callback = Some(v.into());
        self
    }
    #[doc = "Set the field `after_model_callback`.\n"]
    pub fn set_after_model_callback(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterModelCallbackEl>>,
    ) -> Self {
        self.after_model_callback = Some(v.into());
        self
    }
    #[doc = "Set the field `before_agent_callback`.\n"]
    pub fn set_before_agent_callback(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeAgentCallbackEl>>,
    ) -> Self {
        self.before_agent_callback = Some(v.into());
        self
    }
    #[doc = "Set the field `before_model_callback`.\n"]
    pub fn set_before_model_callback(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeModelCallbackEl>>,
    ) -> Self {
        self.before_model_callback = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElCodeCallbackEl {
    type O = BlockAssignable<CesAppVersionSnapshotElGuardrailsElCodeCallbackEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElCodeCallbackEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElCodeCallbackEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsElCodeCallbackEl {
        CesAppVersionSnapshotElGuardrailsElCodeCallbackEl {
            after_agent_callback: core::default::Default::default(),
            after_model_callback: core::default::Default::default(),
            before_agent_callback: core::default::Default::default(),
            before_model_callback: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElCodeCallbackElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElCodeCallbackElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElGuardrailsElCodeCallbackElRef {
        CesAppVersionSnapshotElGuardrailsElCodeCallbackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElCodeCallbackElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `after_agent_callback` after provisioning.\n"]
    pub fn after_agent_callback(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterAgentCallbackElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.after_agent_callback", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `after_model_callback` after provisioning.\n"]
    pub fn after_model_callback(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElGuardrailsElCodeCallbackElAfterModelCallbackElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.after_model_callback", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `before_agent_callback` after provisioning.\n"]
    pub fn before_agent_callback(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeAgentCallbackElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.before_agent_callback", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `before_model_callback` after provisioning.\n"]
    pub fn before_model_callback(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElGuardrailsElCodeCallbackElBeforeModelCallbackElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.before_model_callback", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElContentFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    banned_contents: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    banned_contents_in_agent_response: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    banned_contents_in_user_input: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disregard_diacritics: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_type: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElGuardrailsElContentFilterEl {
    #[doc = "Set the field `banned_contents`.\n"]
    pub fn set_banned_contents(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.banned_contents = Some(v.into());
        self
    }
    #[doc = "Set the field `banned_contents_in_agent_response`.\n"]
    pub fn set_banned_contents_in_agent_response(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.banned_contents_in_agent_response = Some(v.into());
        self
    }
    #[doc = "Set the field `banned_contents_in_user_input`.\n"]
    pub fn set_banned_contents_in_user_input(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.banned_contents_in_user_input = Some(v.into());
        self
    }
    #[doc = "Set the field `disregard_diacritics`.\n"]
    pub fn set_disregard_diacritics(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disregard_diacritics = Some(v.into());
        self
    }
    #[doc = "Set the field `match_type`.\n"]
    pub fn set_match_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.match_type = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElContentFilterEl {
    type O = BlockAssignable<CesAppVersionSnapshotElGuardrailsElContentFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElContentFilterEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElContentFilterEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsElContentFilterEl {
        CesAppVersionSnapshotElGuardrailsElContentFilterEl {
            banned_contents: core::default::Default::default(),
            banned_contents_in_agent_response: core::default::Default::default(),
            banned_contents_in_user_input: core::default::Default::default(),
            disregard_diacritics: core::default::Default::default(),
            match_type: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElContentFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElContentFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElGuardrailsElContentFilterElRef {
        CesAppVersionSnapshotElGuardrailsElContentFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElContentFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `banned_contents` after provisioning.\n"]
    pub fn banned_contents(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.banned_contents", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `banned_contents_in_agent_response` after provisioning.\n"]
    pub fn banned_contents_in_agent_response(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.banned_contents_in_agent_response", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `banned_contents_in_user_input` after provisioning.\n"]
    pub fn banned_contents_in_user_input(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.banned_contents_in_user_input", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disregard_diacritics` after provisioning.\n"]
    pub fn disregard_diacritics(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disregard_diacritics", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `match_type` after provisioning.\n"]
    pub fn match_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.match_type", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElLlmPolicyElModelSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
}
impl CesAppVersionSnapshotElGuardrailsElLlmPolicyElModelSettingsEl {
    #[doc = "Set the field `model`.\n"]
    pub fn set_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model = Some(v.into());
        self
    }
    #[doc = "Set the field `temperature`.\n"]
    pub fn set_temperature(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.temperature = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElLlmPolicyElModelSettingsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElGuardrailsElLlmPolicyElModelSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElLlmPolicyElModelSettingsEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElLlmPolicyElModelSettingsEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsElLlmPolicyElModelSettingsEl {
        CesAppVersionSnapshotElGuardrailsElLlmPolicyElModelSettingsEl {
            model: core::default::Default::default(),
            temperature: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElLlmPolicyElModelSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElLlmPolicyElModelSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElGuardrailsElLlmPolicyElModelSettingsElRef {
        CesAppVersionSnapshotElGuardrailsElLlmPolicyElModelSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElLlmPolicyElModelSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }
    #[doc = "Get a reference to the value of field `temperature` after provisioning.\n"]
    pub fn temperature(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElLlmPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_open: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_conversation_messages: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_settings:
        Option<ListField<CesAppVersionSnapshotElGuardrailsElLlmPolicyElModelSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElGuardrailsElLlmPolicyEl {
    #[doc = "Set the field `fail_open`.\n"]
    pub fn set_fail_open(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_open = Some(v.into());
        self
    }
    #[doc = "Set the field `max_conversation_messages`.\n"]
    pub fn set_max_conversation_messages(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_conversation_messages = Some(v.into());
        self
    }
    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsElLlmPolicyElModelSettingsEl>>,
    ) -> Self {
        self.model_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_scope`.\n"]
    pub fn set_policy_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy_scope = Some(v.into());
        self
    }
    #[doc = "Set the field `prompt`.\n"]
    pub fn set_prompt(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prompt = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElLlmPolicyEl {
    type O = BlockAssignable<CesAppVersionSnapshotElGuardrailsElLlmPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElLlmPolicyEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElLlmPolicyEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsElLlmPolicyEl {
        CesAppVersionSnapshotElGuardrailsElLlmPolicyEl {
            fail_open: core::default::Default::default(),
            max_conversation_messages: core::default::Default::default(),
            model_settings: core::default::Default::default(),
            policy_scope: core::default::Default::default(),
            prompt: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElLlmPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElLlmPolicyElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElGuardrailsElLlmPolicyElRef {
        CesAppVersionSnapshotElGuardrailsElLlmPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElLlmPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fail_open` after provisioning.\n"]
    pub fn fail_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }
    #[doc = "Get a reference to the value of field `max_conversation_messages` after provisioning.\n"]
    pub fn max_conversation_messages(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_conversation_messages", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElGuardrailsElLlmPolicyElModelSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy_scope` after provisioning.\n"]
    pub fn policy_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_scope", self.base))
    }
    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElModelSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
}
impl CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElModelSettingsEl {
    #[doc = "Set the field `model`.\n"]
    pub fn set_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model = Some(v.into());
        self
    }
    #[doc = "Set the field `temperature`.\n"]
    pub fn set_temperature(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.temperature = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElModelSettingsEl
{
    type O = BlockAssignable<
        CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElModelSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElModelSettingsEl
{}
impl BuildCesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElModelSettingsEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElModelSettingsEl {
        CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElModelSettingsEl {
            model: core::default::Default::default(),
            temperature: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElModelSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElModelSettingsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElModelSettingsElRef
    {
        CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElModelSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElModelSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }
    #[doc = "Get a reference to the value of field `temperature` after provisioning.\n"]
    pub fn temperature(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_open: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_conversation_messages: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_settings: Option<
        ListField<
            CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElModelSettingsEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyEl {
    #[doc = "Set the field `fail_open`.\n"]
    pub fn set_fail_open(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_open = Some(v.into());
        self
    }
    #[doc = "Set the field `max_conversation_messages`.\n"]
    pub fn set_max_conversation_messages(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_conversation_messages = Some(v.into());
        self
    }
    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v: impl Into<
            ListField<
                CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElModelSettingsEl,
            >,
        >,
    ) -> Self {
        self.model_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_scope`.\n"]
    pub fn set_policy_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy_scope = Some(v.into());
        self
    }
    #[doc = "Set the field `prompt`.\n"]
    pub fn set_prompt(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prompt = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyEl {
    type O = BlockAssignable<CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyEl {
        CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyEl {
            fail_open: core::default::Default::default(),
            max_conversation_messages: core::default::Default::default(),
            model_settings: core::default::Default::default(),
            policy_scope: core::default::Default::default(),
            prompt: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElRef {
        CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fail_open` after provisioning.\n"]
    pub fn fail_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }
    #[doc = "Get a reference to the value of field `max_conversation_messages` after provisioning.\n"]
    pub fn max_conversation_messages(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_conversation_messages", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(
        &self,
    ) -> ListRef<
        CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElModelSettingsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `policy_scope` after provisioning.\n"]
    pub fn policy_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_scope", self.base))
    }
    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElDefaultSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_prompt_template: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElDefaultSettingsEl {
    #[doc = "Set the field `default_prompt_template`.\n"]
    pub fn set_default_prompt_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_prompt_template = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElDefaultSettingsEl {
    type O =
        BlockAssignable<CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElDefaultSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElDefaultSettingsEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElDefaultSettingsEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElDefaultSettingsEl {
        CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElDefaultSettingsEl {
            default_prompt_template: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElDefaultSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElDefaultSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElDefaultSettingsElRef {
        CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElDefaultSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElDefaultSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_prompt_template` after provisioning.\n"]
    pub fn default_prompt_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_prompt_template", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_policy:
        Option<ListField<CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_settings:
        Option<ListField<CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElDefaultSettingsEl>>,
}
impl CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityEl {
    #[doc = "Set the field `custom_policy`.\n"]
    pub fn set_custom_policy(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyEl>>,
    ) -> Self {
        self.custom_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `default_settings`.\n"]
    pub fn set_default_settings(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElDefaultSettingsEl>>,
    ) -> Self {
        self.default_settings = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityEl {
    type O = BlockAssignable<CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElLlmPromptSecurityEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElLlmPromptSecurityEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityEl {
        CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityEl {
            custom_policy: core::default::Default::default(),
            default_settings: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElRef {
        CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_policy` after provisioning.\n"]
    pub fn custom_policy(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElCustomPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_settings` after provisioning.\n"]
    pub fn default_settings(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElDefaultSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElModelSafetyElSafetySettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElGuardrailsElModelSafetyElSafetySettingsEl {
    #[doc = "Set the field `category`.\n"]
    pub fn set_category(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.category = Some(v.into());
        self
    }
    #[doc = "Set the field `threshold`.\n"]
    pub fn set_threshold(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.threshold = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElModelSafetyElSafetySettingsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElGuardrailsElModelSafetyElSafetySettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElModelSafetyElSafetySettingsEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElModelSafetyElSafetySettingsEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsElModelSafetyElSafetySettingsEl {
        CesAppVersionSnapshotElGuardrailsElModelSafetyElSafetySettingsEl {
            category: core::default::Default::default(),
            threshold: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElModelSafetyElSafetySettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElModelSafetyElSafetySettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElGuardrailsElModelSafetyElSafetySettingsElRef {
        CesAppVersionSnapshotElGuardrailsElModelSafetyElSafetySettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElModelSafetyElSafetySettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }
    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsElModelSafetyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    safety_settings:
        Option<ListField<CesAppVersionSnapshotElGuardrailsElModelSafetyElSafetySettingsEl>>,
}
impl CesAppVersionSnapshotElGuardrailsElModelSafetyEl {
    #[doc = "Set the field `safety_settings`.\n"]
    pub fn set_safety_settings(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsElModelSafetyElSafetySettingsEl>>,
    ) -> Self {
        self.safety_settings = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsElModelSafetyEl {
    type O = BlockAssignable<CesAppVersionSnapshotElGuardrailsElModelSafetyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsElModelSafetyEl {}
impl BuildCesAppVersionSnapshotElGuardrailsElModelSafetyEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsElModelSafetyEl {
        CesAppVersionSnapshotElGuardrailsElModelSafetyEl {
            safety_settings: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElModelSafetyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElModelSafetyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElGuardrailsElModelSafetyElRef {
        CesAppVersionSnapshotElGuardrailsElModelSafetyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElModelSafetyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `safety_settings` after provisioning.\n"]
    pub fn safety_settings(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElGuardrailsElModelSafetyElSafetySettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.safety_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElGuardrailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<ListField<CesAppVersionSnapshotElGuardrailsElActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_callback: Option<ListField<CesAppVersionSnapshotElGuardrailsElCodeCallbackEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_filter: Option<ListField<CesAppVersionSnapshotElGuardrailsElContentFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    llm_policy: Option<ListField<CesAppVersionSnapshotElGuardrailsElLlmPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    llm_prompt_security: Option<ListField<CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_safety: Option<ListField<CesAppVersionSnapshotElGuardrailsElModelSafetyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElGuardrailsEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsElActionEl>>,
    ) -> Self {
        self.action = Some(v.into());
        self
    }
    #[doc = "Set the field `code_callback`.\n"]
    pub fn set_code_callback(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsElCodeCallbackEl>>,
    ) -> Self {
        self.code_callback = Some(v.into());
        self
    }
    #[doc = "Set the field `content_filter`.\n"]
    pub fn set_content_filter(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsElContentFilterEl>>,
    ) -> Self {
        self.content_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.etag = Some(v.into());
        self
    }
    #[doc = "Set the field `llm_policy`.\n"]
    pub fn set_llm_policy(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsElLlmPolicyEl>>,
    ) -> Self {
        self.llm_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `llm_prompt_security`.\n"]
    pub fn set_llm_prompt_security(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityEl>>,
    ) -> Self {
        self.llm_prompt_security = Some(v.into());
        self
    }
    #[doc = "Set the field `model_safety`.\n"]
    pub fn set_model_safety(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsElModelSafetyEl>>,
    ) -> Self {
        self.model_safety = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElGuardrailsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElGuardrailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElGuardrailsEl {}
impl BuildCesAppVersionSnapshotElGuardrailsEl {
    pub fn build(self) -> CesAppVersionSnapshotElGuardrailsEl {
        CesAppVersionSnapshotElGuardrailsEl {
            action: core::default::Default::default(),
            code_callback: core::default::Default::default(),
            content_filter: core::default::Default::default(),
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            enabled: core::default::Default::default(),
            etag: core::default::Default::default(),
            llm_policy: core::default::Default::default(),
            llm_prompt_security: core::default::Default::default(),
            model_safety: core::default::Default::default(),
            name: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElGuardrailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElGuardrailsElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElGuardrailsElRef {
        CesAppVersionSnapshotElGuardrailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElGuardrailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<CesAppVersionSnapshotElGuardrailsElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `code_callback` after provisioning.\n"]
    pub fn code_callback(&self) -> ListRef<CesAppVersionSnapshotElGuardrailsElCodeCallbackElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.code_callback", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `content_filter` after provisioning.\n"]
    pub fn content_filter(&self) -> ListRef<CesAppVersionSnapshotElGuardrailsElContentFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.content_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }
    #[doc = "Get a reference to the value of field `llm_policy` after provisioning.\n"]
    pub fn llm_policy(&self) -> ListRef<CesAppVersionSnapshotElGuardrailsElLlmPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.llm_policy", self.base))
    }
    #[doc = "Get a reference to the value of field `llm_prompt_security` after provisioning.\n"]
    pub fn llm_prompt_security(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElGuardrailsElLlmPromptSecurityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.llm_prompt_security", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `model_safety` after provisioning.\n"]
    pub fn model_safety(&self) -> ListRef<CesAppVersionSnapshotElGuardrailsElModelSafetyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.model_safety", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElClientFunctionElParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_properties: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    any_of: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    defs: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    enum_: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nullable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_items: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<PrimField<String>>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    ref_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<ListField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_items: Option<PrimField<bool>>,
}
impl CesAppVersionSnapshotElToolsElClientFunctionElParametersEl {
    #[doc = "Set the field `additional_properties`.\n"]
    pub fn set_additional_properties(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.additional_properties = Some(v.into());
        self
    }
    #[doc = "Set the field `any_of`.\n"]
    pub fn set_any_of(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.any_of = Some(v.into());
        self
    }
    #[doc = "Set the field `default`.\n"]
    pub fn set_default(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default = Some(v.into());
        self
    }
    #[doc = "Set the field `defs`.\n"]
    pub fn set_defs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.defs = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `enum_`.\n"]
    pub fn set_enum(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.enum_ = Some(v.into());
        self
    }
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.items = Some(v.into());
        self
    }
    #[doc = "Set the field `nullable`.\n"]
    pub fn set_nullable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.nullable = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_items`.\n"]
    pub fn set_prefix_items(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_items = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `ref_`.\n"]
    pub fn set_ref(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ref_ = Some(v.into());
        self
    }
    #[doc = "Set the field `required`.\n"]
    pub fn set_required(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.required = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `unique_items`.\n"]
    pub fn set_unique_items(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.unique_items = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsElClientFunctionElParametersEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsElClientFunctionElParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElClientFunctionElParametersEl {}
impl BuildCesAppVersionSnapshotElToolsElClientFunctionElParametersEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsElClientFunctionElParametersEl {
        CesAppVersionSnapshotElToolsElClientFunctionElParametersEl {
            additional_properties: core::default::Default::default(),
            any_of: core::default::Default::default(),
            default: core::default::Default::default(),
            defs: core::default::Default::default(),
            description: core::default::Default::default(),
            enum_: core::default::Default::default(),
            items: core::default::Default::default(),
            nullable: core::default::Default::default(),
            prefix_items: core::default::Default::default(),
            properties: core::default::Default::default(),
            ref_: core::default::Default::default(),
            required: core::default::Default::default(),
            type_: core::default::Default::default(),
            unique_items: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElClientFunctionElParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElClientFunctionElParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElClientFunctionElParametersElRef {
        CesAppVersionSnapshotElToolsElClientFunctionElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElClientFunctionElParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_properties` after provisioning.\n"]
    pub fn additional_properties(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `any_of` after provisioning.\n"]
    pub fn any_of(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.any_of", self.base))
    }
    #[doc = "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }
    #[doc = "Get a reference to the value of field `defs` after provisioning.\n"]
    pub fn defs(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.defs", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `enum_` after provisioning.\n"]
    pub fn enum_(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.enum", self.base))
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.items", self.base))
    }
    #[doc = "Get a reference to the value of field `nullable` after provisioning.\n"]
    pub fn nullable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix_items` after provisioning.\n"]
    pub fn prefix_items(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_items", self.base))
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `ref_` after provisioning.\n"]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }
    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.required", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `unique_items` after provisioning.\n"]
    pub fn unique_items(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.unique_items", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElClientFunctionElResponseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_properties: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    any_of: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    defs: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    enum_: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nullable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_items: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<PrimField<String>>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    ref_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<ListField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_items: Option<PrimField<bool>>,
}
impl CesAppVersionSnapshotElToolsElClientFunctionElResponseEl {
    #[doc = "Set the field `additional_properties`.\n"]
    pub fn set_additional_properties(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.additional_properties = Some(v.into());
        self
    }
    #[doc = "Set the field `any_of`.\n"]
    pub fn set_any_of(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.any_of = Some(v.into());
        self
    }
    #[doc = "Set the field `default`.\n"]
    pub fn set_default(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default = Some(v.into());
        self
    }
    #[doc = "Set the field `defs`.\n"]
    pub fn set_defs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.defs = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `enum_`.\n"]
    pub fn set_enum(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.enum_ = Some(v.into());
        self
    }
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.items = Some(v.into());
        self
    }
    #[doc = "Set the field `nullable`.\n"]
    pub fn set_nullable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.nullable = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_items`.\n"]
    pub fn set_prefix_items(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_items = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `ref_`.\n"]
    pub fn set_ref(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ref_ = Some(v.into());
        self
    }
    #[doc = "Set the field `required`.\n"]
    pub fn set_required(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.required = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `unique_items`.\n"]
    pub fn set_unique_items(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.unique_items = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsElClientFunctionElResponseEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsElClientFunctionElResponseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElClientFunctionElResponseEl {}
impl BuildCesAppVersionSnapshotElToolsElClientFunctionElResponseEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsElClientFunctionElResponseEl {
        CesAppVersionSnapshotElToolsElClientFunctionElResponseEl {
            additional_properties: core::default::Default::default(),
            any_of: core::default::Default::default(),
            default: core::default::Default::default(),
            defs: core::default::Default::default(),
            description: core::default::Default::default(),
            enum_: core::default::Default::default(),
            items: core::default::Default::default(),
            nullable: core::default::Default::default(),
            prefix_items: core::default::Default::default(),
            properties: core::default::Default::default(),
            ref_: core::default::Default::default(),
            required: core::default::Default::default(),
            type_: core::default::Default::default(),
            unique_items: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElClientFunctionElResponseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElClientFunctionElResponseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElClientFunctionElResponseElRef {
        CesAppVersionSnapshotElToolsElClientFunctionElResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElClientFunctionElResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_properties` after provisioning.\n"]
    pub fn additional_properties(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `any_of` after provisioning.\n"]
    pub fn any_of(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.any_of", self.base))
    }
    #[doc = "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }
    #[doc = "Get a reference to the value of field `defs` after provisioning.\n"]
    pub fn defs(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.defs", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `enum_` after provisioning.\n"]
    pub fn enum_(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.enum", self.base))
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.items", self.base))
    }
    #[doc = "Get a reference to the value of field `nullable` after provisioning.\n"]
    pub fn nullable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix_items` after provisioning.\n"]
    pub fn prefix_items(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_items", self.base))
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `ref_` after provisioning.\n"]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }
    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.required", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `unique_items` after provisioning.\n"]
    pub fn unique_items(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.unique_items", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElClientFunctionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<ListField<CesAppVersionSnapshotElToolsElClientFunctionElParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<ListField<CesAppVersionSnapshotElToolsElClientFunctionElResponseEl>>,
}
impl CesAppVersionSnapshotElToolsElClientFunctionEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsElClientFunctionElParametersEl>>,
    ) -> Self {
        self.parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `response`.\n"]
    pub fn set_response(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsElClientFunctionElResponseEl>>,
    ) -> Self {
        self.response = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsElClientFunctionEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsElClientFunctionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElClientFunctionEl {}
impl BuildCesAppVersionSnapshotElToolsElClientFunctionEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsElClientFunctionEl {
        CesAppVersionSnapshotElToolsElClientFunctionEl {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            parameters: core::default::Default::default(),
            response: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElClientFunctionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElClientFunctionElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElToolsElClientFunctionElRef {
        CesAppVersionSnapshotElToolsElClientFunctionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElClientFunctionElRef {
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
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsElClientFunctionElParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> ListRef<CesAppVersionSnapshotElToolsElClientFunctionElResponseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.response", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost_amount: Option<PrimField<f64>>,
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl { # [doc = "Set the field `attribute_value`.\n"] pub fn set_attribute_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . attribute_value = Some (v . into ()) ; self } # [doc = "Set the field `boost_amount`.\n"] pub fn set_boost_amount (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . boost_amount = Some (v . into ()) ; self } }
impl ToListMappable for CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl { type O = BlockAssignable < CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl
{}
impl BuildCesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl { pub fn build (self) -> CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl { CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl { attribute_value : core :: default :: Default :: default () , boost_amount : core :: default :: Default :: default () , } } }
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsElRef { fn new (shared : StackShared , base : String) -> CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsElRef { CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsElRef { shared : shared , base : base . to_string () , } } }
impl CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `attribute_value` after provisioning.\n"] pub fn attribute_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.attribute_value" , self . base)) } # [doc = "Get a reference to the value of field `boost_amount` after provisioning.\n"] pub fn boost_amount (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.boost_amount" , self . base)) } }
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl { # [serde (skip_serializing_if = "Option::is_none")] attribute_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] control_points : Option < ListField < CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl > > , # [serde (skip_serializing_if = "Option::is_none")] field_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] interpolation_type : Option < PrimField < String > > , }
impl CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl { # [doc = "Set the field `attribute_type`.\n"] pub fn set_attribute_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . attribute_type = Some (v . into ()) ; self } # [doc = "Set the field `control_points`.\n"] pub fn set_control_points (mut self , v : impl Into < ListField < CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl > >) -> Self { self . control_points = Some (v . into ()) ; self } # [doc = "Set the field `field_name`.\n"] pub fn set_field_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . field_name = Some (v . into ()) ; self } # [doc = "Set the field `interpolation_type`.\n"] pub fn set_interpolation_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . interpolation_type = Some (v . into ()) ; self } }
impl ToListMappable for CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl { type O = BlockAssignable < CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl
{}
impl BuildCesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl { pub fn build (self) -> CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl { CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl { attribute_type : core :: default :: Default :: default () , control_points : core :: default :: Default :: default () , field_name : core :: default :: Default :: default () , interpolation_type : core :: default :: Default :: default () , } } }
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElRef { fn new (shared : StackShared , base : String) -> CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElRef { CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElRef { shared : shared , base : base . to_string () , } } }
impl CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `attribute_type` after provisioning.\n"] pub fn attribute_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.attribute_type" , self . base)) } # [doc = "Get a reference to the value of field `control_points` after provisioning.\n"] pub fn control_points (& self) -> ListRef < CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.control_points" , self . base)) } # [doc = "Get a reference to the value of field `field_name` after provisioning.\n"] pub fn field_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.field_name" , self . base)) } # [doc = "Get a reference to the value of field `interpolation_type` after provisioning.\n"] pub fn interpolation_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.interpolation_type" , self . base)) } }
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl { # [serde (skip_serializing_if = "Option::is_none")] boost : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] boost_control_spec : Option < ListField < CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl > > , # [serde (skip_serializing_if = "Option::is_none")] condition : Option < PrimField < String > > , }
impl CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl {
    #[doc = "Set the field `boost`.\n"]
    pub fn set_boost(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boost = Some(v.into());
        self
    }
    #[doc = "Set the field `boost_control_spec`.\n"]
    pub fn set_boost_control_spec(
        mut self,
        v : impl Into < ListField < CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl > >,
    ) -> Self {
        self.boost_control_spec = Some(v.into());
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.condition = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl
{
    type O = BlockAssignable<
        CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl
{}
impl BuildCesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl {
        CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl {
            boost: core::default::Default::default(),
            boost_control_spec: core::default::Default::default(),
            condition: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElRef
    {
        CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `boost` after provisioning.\n"]
    pub fn boost(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.boost", self.base))
    }
    #[doc = "Get a reference to the value of field `boost_control_spec` after provisioning.\n"]    pub fn boost_control_spec (& self) -> ListRef < CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.boost_control_spec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition_boost_specs: Option<
        ListField<
            CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl,
        >,
    >,
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecEl {
    #[doc = "Set the field `condition_boost_specs`.\n"]
    pub fn set_condition_boost_specs(
        mut self,
        v : impl Into < ListField < CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl > >,
    ) -> Self {
        self.condition_boost_specs = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecEl {}
impl BuildCesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecEl {
        CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecEl {
            condition_boost_specs: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElRef {
        CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `condition_boost_specs` after provisioning.\n"]
    pub fn condition_boost_specs(
        &self,
    ) -> ListRef<
        CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition_boost_specs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_stores: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spec: Option<ListField<CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecEl>>,
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsEl {
    #[doc = "Set the field `data_stores`.\n"]
    pub fn set_data_stores(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.data_stores = Some(v.into());
        self
    }
    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecEl>>,
    ) -> Self {
        self.spec = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsEl {}
impl BuildCesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsEl {
        CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsEl {
            data_stores: core::default::Default::default(),
            spec: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElRef {
        CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_stores` after provisioning.\n"]
    pub fn data_stores(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.data_stores", self.base))
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    collection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl { # [doc = "Set the field `collection`.\n"] pub fn set_collection (mut self , v : impl Into < PrimField < String > >) -> Self { self . collection = Some (v . into ()) ; self } # [doc = "Set the field `collection_display_name`.\n"] pub fn set_collection_display_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . collection_display_name = Some (v . into ()) ; self } # [doc = "Set the field `data_source`.\n"] pub fn set_data_source (mut self , v : impl Into < PrimField < String > >) -> Self { self . data_source = Some (v . into ()) ; self } }
impl ToListMappable for CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl { type O = BlockAssignable < CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl
{}
impl BuildCesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl { pub fn build (self) -> CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl { CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl { collection : core :: default :: Default :: default () , collection_display_name : core :: default :: Default :: default () , data_source : core :: default :: Default :: default () , } } }
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigElRef { fn new (shared : StackShared , base : String) -> CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigElRef { CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigElRef { shared : shared , base : base . to_string () , } } }
impl CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `collection` after provisioning.\n"] pub fn collection (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.collection" , self . base)) } # [doc = "Get a reference to the value of field `collection_display_name` after provisioning.\n"] pub fn collection_display_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.collection_display_name" , self . base)) } # [doc = "Get a reference to the value of field `data_source` after provisioning.\n"] pub fn data_source (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.data_source" , self . base)) } }
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl { # [serde (skip_serializing_if = "Option::is_none")] connector_config : Option < ListField < CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] create_time : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] display_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] document_processing_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] name : Option < PrimField < String > > , # [serde (rename = "type" , skip_serializing_if = "Option::is_none")] type_ : Option < PrimField < String > > , }
impl CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl {
    #[doc = "Set the field `connector_config`.\n"]
    pub fn set_connector_config(
        mut self,
        v : impl Into < ListField < CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl > >,
    ) -> Self {
        self.connector_config = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `document_processing_mode`.\n"]
    pub fn set_document_processing_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.document_processing_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl
{
    type O = BlockAssignable<
        CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl
{}
impl BuildCesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl
    {
        CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl {
            connector_config: core::default::Default::default(),
            create_time: core::default::Default::default(),
            display_name: core::default::Default::default(),
            document_processing_mode: core::default::Default::default(),
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElRef
    {
        CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElRef { shared : shared , base : base . to_string () , }
    }
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connector_config` after provisioning.\n"]    pub fn connector_config (& self) -> ListRef < CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.connector_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `document_processing_mode` after provisioning.\n"]
    pub fn document_processing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.document_processing_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesEl { # [serde (skip_serializing_if = "Option::is_none")] data_store : Option < ListField < CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl > > , # [serde (skip_serializing_if = "Option::is_none")] filter : Option < PrimField < String > > , }
impl CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesEl {
    #[doc = "Set the field `data_store`.\n"]
    pub fn set_data_store(
        mut self,
        v : impl Into < ListField < CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl > >,
    ) -> Self {
        self.data_store = Some(v.into());
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesEl
{
    type O = BlockAssignable<
        CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesEl {}
impl BuildCesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesEl {
        CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesEl {
            data_store: core::default::Default::default(),
            filter: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElRef {
        CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_store` after provisioning.\n"]
    pub fn data_store(
        &self,
    ) -> ListRef<
        CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.data_store", self.base))
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_sources: Option<
        ListField<CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceEl {
    #[doc = "Set the field `data_store_sources`.\n"]
    pub fn set_data_store_sources(
        mut self,
        v: impl Into<
            ListField<
                CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesEl,
            >,
        >,
    ) -> Self {
        self.data_store_sources = Some(v.into());
        self
    }
    #[doc = "Set the field `engine`.\n"]
    pub fn set_engine(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.engine = Some(v.into());
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceEl {}
impl BuildCesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceEl {
        CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceEl {
            data_store_sources: core::default::Default::default(),
            engine: core::default::Default::default(),
            filter: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElRef {
        CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_store_sources` after provisioning.\n"]
    pub fn data_store_sources(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElDataStoreSourcesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_sources", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.base))
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElGroundingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grounding_level: Option<PrimField<f64>>,
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElGroundingConfigEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `grounding_level`.\n"]
    pub fn set_grounding_level(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.grounding_level = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElGroundingConfigEl
{
    type O = BlockAssignable<
        CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElGroundingConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElGroundingConfigEl {}
impl BuildCesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElGroundingConfigEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElGroundingConfigEl {
        CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElGroundingConfigEl {
            disabled: core::default::Default::default(),
            grounding_level: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElGroundingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElGroundingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElGroundingConfigElRef {
        CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElGroundingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElGroundingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `grounding_level` after provisioning.\n"]
    pub fn grounding_level(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.grounding_level", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl {
    #[doc = "Set the field `model`.\n"]
    pub fn set_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model = Some(v.into());
        self
    }
    #[doc = "Set the field `temperature`.\n"]
    pub fn set_temperature(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.temperature = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl { type O = BlockAssignable < CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl
{}
impl BuildCesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl { pub fn build (self) -> CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl { CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl { model : core :: default :: Default :: default () , temperature : core :: default :: Default :: default () , } } }
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsElRef { fn new (shared : StackShared , base : String) -> CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsElRef { CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsElRef { shared : shared , base : base . to_string () , } } }
impl
    CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }
    #[doc = "Get a reference to the value of field `temperature` after provisioning.\n"]
    pub fn temperature(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigEl { # [serde (skip_serializing_if = "Option::is_none")] disabled : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] model_settings : Option < ListField < CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] prompt : Option < PrimField < String > > , }
impl CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v : impl Into < ListField < CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl > >,
    ) -> Self {
        self.model_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `prompt`.\n"]
    pub fn set_prompt(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prompt = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigEl
{
    type O = BlockAssignable<
        CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigEl {}
impl BuildCesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigEl {
        CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigEl {
            disabled: core::default::Default::default(),
            model_settings: core::default::Default::default(),
            prompt: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElRef {
        CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]    pub fn model_settings (& self) -> ListRef < CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl { # [doc = "Set the field `model`.\n"] pub fn set_model (mut self , v : impl Into < PrimField < String > >) -> Self { self . model = Some (v . into ()) ; self } # [doc = "Set the field `temperature`.\n"] pub fn set_temperature (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . temperature = Some (v . into ()) ; self } }
impl ToListMappable for CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl { type O = BlockAssignable < CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl
{}
impl BuildCesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl { pub fn build (self) -> CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl { CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl { model : core :: default :: Default :: default () , temperature : core :: default :: Default :: default () , } } }
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsElRef { fn new (shared : StackShared , base : String) -> CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsElRef { CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsElRef { shared : shared , base : base . to_string () , } } }
impl CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `model` after provisioning.\n"] pub fn model (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.model" , self . base)) } # [doc = "Get a reference to the value of field `temperature` after provisioning.\n"] pub fn temperature (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.temperature" , self . base)) } }
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigEl { # [serde (skip_serializing_if = "Option::is_none")] disabled : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] model_settings : Option < ListField < CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl > > , # [serde (skip_serializing_if = "Option::is_none")] prompt : Option < PrimField < String > > , }
impl CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v : impl Into < ListField < CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl > >,
    ) -> Self {
        self.model_settings = Some(v.into());
        self
    }
    #[doc = "Set the field `prompt`.\n"]
    pub fn set_prompt(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prompt = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigEl
{
    type O = BlockAssignable<
        CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigEl
{}
impl BuildCesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigEl {
        CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigEl {
            disabled: core::default::Default::default(),
            model_settings: core::default::Default::default(),
            prompt: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElRef
    {
        CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]    pub fn model_settings (& self) -> ListRef < CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grounding_config: Option<
        ListField<CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElGroundingConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    modality_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rewriter_config: Option<
        ListField<CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    summarization_config: Option<
        ListField<
            CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigEl,
        >,
    >,
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsEl {
    #[doc = "Set the field `grounding_config`.\n"]
    pub fn set_grounding_config(
        mut self,
        v: impl Into<
            ListField<
                CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElGroundingConfigEl,
            >,
        >,
    ) -> Self {
        self.grounding_config = Some(v.into());
        self
    }
    #[doc = "Set the field `modality_type`.\n"]
    pub fn set_modality_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.modality_type = Some(v.into());
        self
    }
    #[doc = "Set the field `rewriter_config`.\n"]
    pub fn set_rewriter_config(
        mut self,
        v: impl Into<
            ListField<
                CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigEl,
            >,
        >,
    ) -> Self {
        self.rewriter_config = Some(v.into());
        self
    }
    #[doc = "Set the field `summarization_config`.\n"]
    pub fn set_summarization_config(
        mut self,
        v: impl Into<
            ListField<
                CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigEl,
            >,
        >,
    ) -> Self {
        self.summarization_config = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsEl {}
impl BuildCesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsEl {
        CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsEl {
            grounding_config: core::default::Default::default(),
            modality_type: core::default::Default::default(),
            rewriter_config: core::default::Default::default(),
            summarization_config: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRef {
        CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `grounding_config` after provisioning.\n"]
    pub fn grounding_config(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElGroundingConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grounding_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `modality_type` after provisioning.\n"]
    pub fn modality_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.modality_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rewriter_config` after provisioning.\n"]
    pub fn rewriter_config(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRewriterConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rewriter_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `summarization_config` after provisioning.\n"]
    pub fn summarization_config(
        &self,
    ) -> ListRef<
        CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElSummarizationConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.summarization_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElDataStoreToolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    boost_specs: Option<ListField<CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_source: Option<ListField<CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_results: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    modality_configs:
        Option<ListField<CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsElDataStoreToolEl {
    #[doc = "Set the field `boost_specs`.\n"]
    pub fn set_boost_specs(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsEl>>,
    ) -> Self {
        self.boost_specs = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `engine_source`.\n"]
    pub fn set_engine_source(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceEl>>,
    ) -> Self {
        self.engine_source = Some(v.into());
        self
    }
    #[doc = "Set the field `max_results`.\n"]
    pub fn set_max_results(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_results = Some(v.into());
        self
    }
    #[doc = "Set the field `modality_configs`.\n"]
    pub fn set_modality_configs(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsEl>>,
    ) -> Self {
        self.modality_configs = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsElDataStoreToolEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsElDataStoreToolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElDataStoreToolEl {}
impl BuildCesAppVersionSnapshotElToolsElDataStoreToolEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsElDataStoreToolEl {
        CesAppVersionSnapshotElToolsElDataStoreToolEl {
            boost_specs: core::default::Default::default(),
            description: core::default::Default::default(),
            engine_source: core::default::Default::default(),
            max_results: core::default::Default::default(),
            modality_configs: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElDataStoreToolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElDataStoreToolElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElToolsElDataStoreToolElRef {
        CesAppVersionSnapshotElToolsElDataStoreToolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElDataStoreToolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `boost_specs` after provisioning.\n"]
    pub fn boost_specs(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsElDataStoreToolElBoostSpecsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.boost_specs", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `engine_source` after provisioning.\n"]
    pub fn engine_source(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsElDataStoreToolElEngineSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.engine_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_results` after provisioning.\n"]
    pub fn max_results(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_results", self.base))
    }
    #[doc = "Get a reference to the value of field `modality_configs` after provisioning.\n"]
    pub fn modality_configs(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsElDataStoreToolElModalityConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.modality_configs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElGoogleSearchToolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_domains: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsElGoogleSearchToolEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_domains`.\n"]
    pub fn set_exclude_domains(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_domains = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsElGoogleSearchToolEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsElGoogleSearchToolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElGoogleSearchToolEl {}
impl BuildCesAppVersionSnapshotElToolsElGoogleSearchToolEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsElGoogleSearchToolEl {
        CesAppVersionSnapshotElToolsElGoogleSearchToolEl {
            description: core::default::Default::default(),
            exclude_domains: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElGoogleSearchToolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElGoogleSearchToolElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElGoogleSearchToolElRef {
        CesAppVersionSnapshotElToolsElGoogleSearchToolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElGoogleSearchToolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `exclude_domains` after provisioning.\n"]
    pub fn exclude_domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_domains", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElApiKeyConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key_secret_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_location: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElApiKeyConfigEl {
    #[doc = "Set the field `api_key_secret_version`.\n"]
    pub fn set_api_key_secret_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_key_secret_version = Some(v.into());
        self
    }
    #[doc = "Set the field `key_name`.\n"]
    pub fn set_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `request_location`.\n"]
    pub fn set_request_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_location = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElApiKeyConfigEl
{
    type O = BlockAssignable<
        CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElApiKeyConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElApiKeyConfigEl {}
impl BuildCesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElApiKeyConfigEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElApiKeyConfigEl {
        CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElApiKeyConfigEl {
            api_key_secret_version: core::default::Default::default(),
            key_name: core::default::Default::default(),
            request_location: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElApiKeyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElApiKeyConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElApiKeyConfigElRef {
        CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElApiKeyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElApiKeyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key_secret_version` after provisioning.\n"]
    pub fn api_key_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_key_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `request_location` after provisioning.\n"]
    pub fn request_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_location", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElOauthConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_grant_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_endpoint: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElOauthConfigEl {
    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }
    #[doc = "Set the field `client_secret_version`.\n"]
    pub fn set_client_secret_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret_version = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_grant_type`.\n"]
    pub fn set_oauth_grant_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oauth_grant_type = Some(v.into());
        self
    }
    #[doc = "Set the field `scopes`.\n"]
    pub fn set_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token_endpoint = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElOauthConfigEl
{
    type O = BlockAssignable<
        CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElOauthConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElOauthConfigEl {}
impl BuildCesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElOauthConfigEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElOauthConfigEl {
        CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElOauthConfigEl {
            client_id: core::default::Default::default(),
            client_secret_version: core::default::Default::default(),
            oauth_grant_type: core::default::Default::default(),
            scopes: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElOauthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElOauthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElOauthConfigElRef {
        CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElOauthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElOauthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret_version` after provisioning.\n"]
    pub fn client_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_grant_type` after provisioning.\n"]
    pub fn oauth_grant_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth_grant_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl {
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl
{
    type O = BlockAssignable<
        CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl
{}
impl BuildCesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl
    {
        CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl {
            service_account: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAccountAuthConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAccountAuthConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAccountAuthConfigElRef
    {
        CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAccountAuthConfigElRef { shared : shared , base : base . to_string () , }
    }
}
impl CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAccountAuthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl
{}
impl CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {}
impl ToListMappable for CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl { type O = BlockAssignable < CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl
{}
impl BuildCesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl { pub fn build (self) -> CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl { CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl { } } }
pub struct CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef { fn new (shared : StackShared , base : String) -> CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef { CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef { shared : shared , base : base . to_string () , } } }
impl
    CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationEl { # [serde (skip_serializing_if = "Option::is_none")] api_key_config : Option < ListField < CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElApiKeyConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] oauth_config : Option < ListField < CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElOauthConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] service_account_auth_config : Option < ListField < CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] service_agent_id_token_auth_config : Option < ListField < CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl > > , }
impl CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationEl {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v: impl Into<
            ListField<CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElApiKeyConfigEl>,
        >,
    ) -> Self {
        self.api_key_config = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_config`.\n"]
    pub fn set_oauth_config(
        mut self,
        v: impl Into<
            ListField<CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElOauthConfigEl>,
        >,
    ) -> Self {
        self.oauth_config = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_auth_config`.\n"]
    pub fn set_service_account_auth_config(
        mut self,
        v : impl Into < ListField < CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl > >,
    ) -> Self {
        self.service_account_auth_config = Some(v.into());
        self
    }
    #[doc = "Set the field `service_agent_id_token_auth_config`.\n"]
    pub fn set_service_agent_id_token_auth_config(
        mut self,
        v : impl Into < ListField < CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl > >,
    ) -> Self {
        self.service_agent_id_token_auth_config = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationEl {}
impl BuildCesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationEl {
        CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationEl {
            api_key_config: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            service_account_auth_config: core::default::Default::default(),
            service_agent_id_token_auth_config: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElRef {
        CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElApiKeyConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_key_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElOauthConfigElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_auth_config` after provisioning.\n"]
    pub fn service_account_auth_config(
        &self,
    ) -> ListRef<
        CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAccountAuthConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account_auth_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_agent_id_token_auth_config` after provisioning.\n"]    pub fn service_agent_id_token_auth_config (& self) -> ListRef < CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_agent_id_token_auth_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElOpenApiToolElServiceDirectoryConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsElOpenApiToolElServiceDirectoryConfigEl {
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsElOpenApiToolElServiceDirectoryConfigEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsElOpenApiToolElServiceDirectoryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElOpenApiToolElServiceDirectoryConfigEl {}
impl BuildCesAppVersionSnapshotElToolsElOpenApiToolElServiceDirectoryConfigEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsElOpenApiToolElServiceDirectoryConfigEl {
        CesAppVersionSnapshotElToolsElOpenApiToolElServiceDirectoryConfigEl {
            service: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElOpenApiToolElServiceDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElOpenApiToolElServiceDirectoryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElOpenApiToolElServiceDirectoryConfigElRef {
        CesAppVersionSnapshotElToolsElOpenApiToolElServiceDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElOpenApiToolElServiceDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElCaCertsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cert: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElCaCertsEl {
    #[doc = "Set the field `cert`.\n"]
    pub fn set_cert(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cert = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElCaCertsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElCaCertsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElCaCertsEl {}
impl BuildCesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElCaCertsEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElCaCertsEl {
        CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElCaCertsEl {
            cert: core::default::Default::default(),
            display_name: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElCaCertsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElCaCertsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElCaCertsElRef {
        CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElCaCertsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElCaCertsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert` after provisioning.\n"]
    pub fn cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cert", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certs: Option<ListField<CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElCaCertsEl>>,
}
impl CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigEl {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElCaCertsEl>>,
    ) -> Self {
        self.ca_certs = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigEl {}
impl BuildCesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigEl {
        CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigEl {
            ca_certs: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElRef {
        CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElCaCertsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElOpenApiToolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_authentication:
        Option<ListField<CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_unknown_fields: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_api_schema: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_config:
        Option<ListField<CesAppVersionSnapshotElToolsElOpenApiToolElServiceDirectoryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_config: Option<ListField<CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsElOpenApiToolEl {
    #[doc = "Set the field `api_authentication`.\n"]
    pub fn set_api_authentication(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationEl>>,
    ) -> Self {
        self.api_authentication = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_unknown_fields`.\n"]
    pub fn set_ignore_unknown_fields(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_unknown_fields = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `open_api_schema`.\n"]
    pub fn set_open_api_schema(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.open_api_schema = Some(v.into());
        self
    }
    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsElOpenApiToolElServiceDirectoryConfigEl>>,
    ) -> Self {
        self.service_directory_config = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigEl>>,
    ) -> Self {
        self.tls_config = Some(v.into());
        self
    }
    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsElOpenApiToolEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsElOpenApiToolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElOpenApiToolEl {}
impl BuildCesAppVersionSnapshotElToolsElOpenApiToolEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsElOpenApiToolEl {
        CesAppVersionSnapshotElToolsElOpenApiToolEl {
            api_authentication: core::default::Default::default(),
            description: core::default::Default::default(),
            ignore_unknown_fields: core::default::Default::default(),
            name: core::default::Default::default(),
            open_api_schema: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            tls_config: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElOpenApiToolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElOpenApiToolElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElToolsElOpenApiToolElRef {
        CesAppVersionSnapshotElToolsElOpenApiToolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElOpenApiToolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_authentication` after provisioning.\n"]
    pub fn api_authentication(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsElOpenApiToolElApiAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_authentication", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_unknown_fields` after provisioning.\n"]
    pub fn ignore_unknown_fields(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_unknown_fields", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `open_api_schema` after provisioning.\n"]
    pub fn open_api_schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.open_api_schema", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsElOpenApiToolElServiceDirectoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> ListRef<CesAppVersionSnapshotElToolsElOpenApiToolElTlsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls_config", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElPythonFunctionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_code: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsElPythonFunctionEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.python_code = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsElPythonFunctionEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsElPythonFunctionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElPythonFunctionEl {}
impl BuildCesAppVersionSnapshotElToolsElPythonFunctionEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsElPythonFunctionEl {
        CesAppVersionSnapshotElToolsElPythonFunctionEl {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElPythonFunctionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElPythonFunctionElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElToolsElPythonFunctionElRef {
        CesAppVersionSnapshotElToolsElPythonFunctionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElPythonFunctionElRef {
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
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsElSystemToolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsElSystemToolEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsElSystemToolEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsElSystemToolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsElSystemToolEl {}
impl BuildCesAppVersionSnapshotElToolsElSystemToolEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsElSystemToolEl {
        CesAppVersionSnapshotElToolsElSystemToolEl {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElSystemToolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElSystemToolElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElToolsElSystemToolElRef {
        CesAppVersionSnapshotElToolsElSystemToolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElSystemToolElRef {
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
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_function: Option<ListField<CesAppVersionSnapshotElToolsElClientFunctionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_tool: Option<ListField<CesAppVersionSnapshotElToolsElDataStoreToolEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generated_summary: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_search_tool: Option<ListField<CesAppVersionSnapshotElToolsElGoogleSearchToolEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_api_tool: Option<ListField<CesAppVersionSnapshotElToolsElOpenApiToolEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_function: Option<ListField<CesAppVersionSnapshotElToolsElPythonFunctionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_tool: Option<ListField<CesAppVersionSnapshotElToolsElSystemToolEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsEl {
    #[doc = "Set the field `client_function`.\n"]
    pub fn set_client_function(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsElClientFunctionEl>>,
    ) -> Self {
        self.client_function = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `data_store_tool`.\n"]
    pub fn set_data_store_tool(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsElDataStoreToolEl>>,
    ) -> Self {
        self.data_store_tool = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.etag = Some(v.into());
        self
    }
    #[doc = "Set the field `execution_type`.\n"]
    pub fn set_execution_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_type = Some(v.into());
        self
    }
    #[doc = "Set the field `generated_summary`.\n"]
    pub fn set_generated_summary(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.generated_summary = Some(v.into());
        self
    }
    #[doc = "Set the field `google_search_tool`.\n"]
    pub fn set_google_search_tool(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsElGoogleSearchToolEl>>,
    ) -> Self {
        self.google_search_tool = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `open_api_tool`.\n"]
    pub fn set_open_api_tool(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsElOpenApiToolEl>>,
    ) -> Self {
        self.open_api_tool = Some(v.into());
        self
    }
    #[doc = "Set the field `python_function`.\n"]
    pub fn set_python_function(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsElPythonFunctionEl>>,
    ) -> Self {
        self.python_function = Some(v.into());
        self
    }
    #[doc = "Set the field `system_tool`.\n"]
    pub fn set_system_tool(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsElSystemToolEl>>,
    ) -> Self {
        self.system_tool = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsEl {}
impl BuildCesAppVersionSnapshotElToolsEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsEl {
        CesAppVersionSnapshotElToolsEl {
            client_function: core::default::Default::default(),
            create_time: core::default::Default::default(),
            data_store_tool: core::default::Default::default(),
            display_name: core::default::Default::default(),
            etag: core::default::Default::default(),
            execution_type: core::default::Default::default(),
            generated_summary: core::default::Default::default(),
            google_search_tool: core::default::Default::default(),
            name: core::default::Default::default(),
            open_api_tool: core::default::Default::default(),
            python_function: core::default::Default::default(),
            system_tool: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElToolsElRef {
        CesAppVersionSnapshotElToolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_function` after provisioning.\n"]
    pub fn client_function(&self) -> ListRef<CesAppVersionSnapshotElToolsElClientFunctionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_function", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `data_store_tool` after provisioning.\n"]
    pub fn data_store_tool(&self) -> ListRef<CesAppVersionSnapshotElToolsElDataStoreToolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_tool", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }
    #[doc = "Get a reference to the value of field `execution_type` after provisioning.\n"]
    pub fn execution_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `generated_summary` after provisioning.\n"]
    pub fn generated_summary(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generated_summary", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `google_search_tool` after provisioning.\n"]
    pub fn google_search_tool(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsElGoogleSearchToolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_search_tool", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `open_api_tool` after provisioning.\n"]
    pub fn open_api_tool(&self) -> ListRef<CesAppVersionSnapshotElToolsElOpenApiToolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.open_api_tool", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `python_function` after provisioning.\n"]
    pub fn python_function(&self) -> ListRef<CesAppVersionSnapshotElToolsElPythonFunctionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.python_function", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `system_tool` after provisioning.\n"]
    pub fn system_tool(&self) -> ListRef<CesAppVersionSnapshotElToolsElSystemToolElRef> {
        ListRef::new(self.shared().clone(), format!("{}.system_tool", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElApiKeyConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key_secret_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_location: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElApiKeyConfigEl {
    #[doc = "Set the field `api_key_secret_version`.\n"]
    pub fn set_api_key_secret_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_key_secret_version = Some(v.into());
        self
    }
    #[doc = "Set the field `key_name`.\n"]
    pub fn set_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `request_location`.\n"]
    pub fn set_request_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_location = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElApiKeyConfigEl
{
    type O = BlockAssignable<
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElApiKeyConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElApiKeyConfigEl
{}
impl BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElApiKeyConfigEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElApiKeyConfigEl {
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElApiKeyConfigEl {
            api_key_secret_version: core::default::Default::default(),
            key_name: core::default::Default::default(),
            request_location: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElApiKeyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElApiKeyConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElApiKeyConfigElRef {
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElApiKeyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElApiKeyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key_secret_version` after provisioning.\n"]
    pub fn api_key_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_key_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `request_location` after provisioning.\n"]
    pub fn request_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_location", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl {
    #[doc = "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl
{
    type O = BlockAssignable<
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl
{}
impl BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl
    {
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl {
            token: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElBearerTokenConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElBearerTokenConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElBearerTokenConfigElRef
    {
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElBearerTokenConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElBearerTokenConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElOauthConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_grant_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_endpoint: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElOauthConfigEl {
    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }
    #[doc = "Set the field `client_secret_version`.\n"]
    pub fn set_client_secret_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret_version = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_grant_type`.\n"]
    pub fn set_oauth_grant_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oauth_grant_type = Some(v.into());
        self
    }
    #[doc = "Set the field `scopes`.\n"]
    pub fn set_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token_endpoint = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElOauthConfigEl
{
    type O = BlockAssignable<
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElOauthConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElOauthConfigEl {
}
impl BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElOauthConfigEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElOauthConfigEl {
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElOauthConfigEl {
            client_id: core::default::Default::default(),
            client_secret_version: core::default::Default::default(),
            oauth_grant_type: core::default::Default::default(),
            scopes: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElOauthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElOauthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElOauthConfigElRef {
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElOauthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElOauthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret_version` after provisioning.\n"]
    pub fn client_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_grant_type` after provisioning.\n"]
    pub fn oauth_grant_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth_grant_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
}
impl
    CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl
{
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl { type O = BlockAssignable < CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl
{}
impl BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl { pub fn build (self) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl { CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl { service_account : core :: default :: Default :: default () , } } }
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigElRef { fn new (shared : StackShared , base : String) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigElRef { CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigElRef { shared : shared , base : base . to_string () , } } }
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `service_account` after provisioning.\n"] pub fn service_account (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.service_account" , self . base)) } }
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl
{}
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl { }
impl ToListMappable for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl { type O = BlockAssignable < CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl
{}
impl BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl { pub fn build (self) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl { CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl { } } }
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef { fn new (shared : StackShared , base : String) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef { CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef { shared : shared , base : base . to_string () , } } }
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationEl { # [serde (skip_serializing_if = "Option::is_none")] api_key_config : Option < ListField < CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElApiKeyConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] bearer_token_config : Option < ListField < CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] oauth_config : Option < ListField < CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElOauthConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] service_account_auth_config : Option < ListField < CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] service_agent_id_token_auth_config : Option < ListField < CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl > > , }
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationEl {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v: impl Into<
            ListField<
                CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElApiKeyConfigEl,
            >,
        >,
    ) -> Self {
        self.api_key_config = Some(v.into());
        self
    }
    #[doc = "Set the field `bearer_token_config`.\n"]
    pub fn set_bearer_token_config(
        mut self,
        v : impl Into < ListField < CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl > >,
    ) -> Self {
        self.bearer_token_config = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_config`.\n"]
    pub fn set_oauth_config(
        mut self,
        v: impl Into<
            ListField<
                CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElOauthConfigEl,
            >,
        >,
    ) -> Self {
        self.oauth_config = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_auth_config`.\n"]
    pub fn set_service_account_auth_config(
        mut self,
        v : impl Into < ListField < CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl > >,
    ) -> Self {
        self.service_account_auth_config = Some(v.into());
        self
    }
    #[doc = "Set the field `service_agent_id_token_auth_config`.\n"]
    pub fn set_service_agent_id_token_auth_config(
        mut self,
        v : impl Into < ListField < CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl > >,
    ) -> Self {
        self.service_agent_id_token_auth_config = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationEl {}
impl BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationEl {
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationEl {
            api_key_config: core::default::Default::default(),
            bearer_token_config: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            service_account_auth_config: core::default::Default::default(),
            service_agent_id_token_auth_config: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElRef {
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(
        &self,
    ) -> ListRef<
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElApiKeyConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_key_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bearer_token_config` after provisioning.\n"]
    pub fn bearer_token_config(
        &self,
    ) -> ListRef<
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElBearerTokenConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bearer_token_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElOauthConfigElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_auth_config` after provisioning.\n"]    pub fn service_account_auth_config (& self) -> ListRef < CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account_auth_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_agent_id_token_auth_config` after provisioning.\n"]    pub fn service_agent_id_token_auth_config (& self) -> ListRef < CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_agent_id_token_auth_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElServiceDirectoryConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElServiceDirectoryConfigEl {
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElServiceDirectoryConfigEl {
    type O =
        BlockAssignable<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElServiceDirectoryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElServiceDirectoryConfigEl {}
impl BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElServiceDirectoryConfigEl {
    pub fn build(
        self,
    ) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElServiceDirectoryConfigEl {
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElServiceDirectoryConfigEl {
            service: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElServiceDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElServiceDirectoryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElServiceDirectoryConfigElRef {
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElServiceDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElServiceDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElCaCertsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cert: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElCaCertsEl {
    #[doc = "Set the field `cert`.\n"]
    pub fn set_cert(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cert = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElCaCertsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElCaCertsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElCaCertsEl {}
impl BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElCaCertsEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElCaCertsEl {
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElCaCertsEl {
            cert: core::default::Default::default(),
            display_name: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElCaCertsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElCaCertsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElCaCertsElRef {
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElCaCertsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElCaCertsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert` after provisioning.\n"]
    pub fn cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cert", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certs:
        Option<ListField<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElCaCertsEl>>,
}
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigEl {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElCaCertsEl>>,
    ) -> Self {
        self.ca_certs = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigEl {}
impl BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigEl {
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigEl {
            ca_certs: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElRef {
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElCaCertsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_authentication:
        Option<ListField<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_unknown_fields: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_api_schema: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_config: Option<
        ListField<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElServiceDirectoryConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_config: Option<ListField<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetEl {
    #[doc = "Set the field `api_authentication`.\n"]
    pub fn set_api_authentication(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationEl>>,
    ) -> Self {
        self.api_authentication = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_unknown_fields`.\n"]
    pub fn set_ignore_unknown_fields(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_unknown_fields = Some(v.into());
        self
    }
    #[doc = "Set the field `open_api_schema`.\n"]
    pub fn set_open_api_schema(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.open_api_schema = Some(v.into());
        self
    }
    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl Into<
            ListField<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElServiceDirectoryConfigEl>,
        >,
    ) -> Self {
        self.service_directory_config = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigEl>>,
    ) -> Self {
        self.tls_config = Some(v.into());
        self
    }
    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsetsElOpenApiToolsetEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsetsElOpenApiToolsetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetEl {}
impl BuildCesAppVersionSnapshotElToolsetsElOpenApiToolsetEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetEl {
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetEl {
            api_authentication: core::default::Default::default(),
            ignore_unknown_fields: core::default::Default::default(),
            open_api_schema: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            tls_config: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsetsElOpenApiToolsetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsetsElOpenApiToolsetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesAppVersionSnapshotElToolsetsElOpenApiToolsetElRef {
        CesAppVersionSnapshotElToolsetsElOpenApiToolsetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsetsElOpenApiToolsetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_authentication` after provisioning.\n"]
    pub fn api_authentication(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElApiAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_authentication", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ignore_unknown_fields` after provisioning.\n"]
    pub fn ignore_unknown_fields(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_unknown_fields", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `open_api_schema` after provisioning.\n"]
    pub fn open_api_schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.open_api_schema", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElServiceDirectoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElTlsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls_config", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotElToolsetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_api_toolset: Option<ListField<CesAppVersionSnapshotElToolsetsElOpenApiToolsetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl CesAppVersionSnapshotElToolsetsEl {
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.etag = Some(v.into());
        self
    }
    #[doc = "Set the field `execution_type`.\n"]
    pub fn set_execution_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.execution_type = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `open_api_toolset`.\n"]
    pub fn set_open_api_toolset(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsetsElOpenApiToolsetEl>>,
    ) -> Self {
        self.open_api_toolset = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotElToolsetsEl {
    type O = BlockAssignable<CesAppVersionSnapshotElToolsetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotElToolsetsEl {}
impl BuildCesAppVersionSnapshotElToolsetsEl {
    pub fn build(self) -> CesAppVersionSnapshotElToolsetsEl {
        CesAppVersionSnapshotElToolsetsEl {
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            etag: core::default::Default::default(),
            execution_type: core::default::Default::default(),
            name: core::default::Default::default(),
            open_api_toolset: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElToolsetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElToolsetsElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElToolsetsElRef {
        CesAppVersionSnapshotElToolsetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElToolsetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }
    #[doc = "Get a reference to the value of field `execution_type` after provisioning.\n"]
    pub fn execution_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `open_api_toolset` after provisioning.\n"]
    pub fn open_api_toolset(
        &self,
    ) -> ListRef<CesAppVersionSnapshotElToolsetsElOpenApiToolsetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.open_api_toolset", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionSnapshotEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    agents: Option<ListField<CesAppVersionSnapshotElAgentsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app: Option<ListField<CesAppVersionSnapshotElAppEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    examples: Option<ListField<CesAppVersionSnapshotElExamplesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guardrails: Option<ListField<CesAppVersionSnapshotElGuardrailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<ListField<CesAppVersionSnapshotElToolsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toolsets: Option<ListField<CesAppVersionSnapshotElToolsetsEl>>,
}
impl CesAppVersionSnapshotEl {
    #[doc = "Set the field `agents`.\n"]
    pub fn set_agents(mut self, v: impl Into<ListField<CesAppVersionSnapshotElAgentsEl>>) -> Self {
        self.agents = Some(v.into());
        self
    }
    #[doc = "Set the field `app`.\n"]
    pub fn set_app(mut self, v: impl Into<ListField<CesAppVersionSnapshotElAppEl>>) -> Self {
        self.app = Some(v.into());
        self
    }
    #[doc = "Set the field `examples`.\n"]
    pub fn set_examples(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElExamplesEl>>,
    ) -> Self {
        self.examples = Some(v.into());
        self
    }
    #[doc = "Set the field `guardrails`.\n"]
    pub fn set_guardrails(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElGuardrailsEl>>,
    ) -> Self {
        self.guardrails = Some(v.into());
        self
    }
    #[doc = "Set the field `tools`.\n"]
    pub fn set_tools(mut self, v: impl Into<ListField<CesAppVersionSnapshotElToolsEl>>) -> Self {
        self.tools = Some(v.into());
        self
    }
    #[doc = "Set the field `toolsets`.\n"]
    pub fn set_toolsets(
        mut self,
        v: impl Into<ListField<CesAppVersionSnapshotElToolsetsEl>>,
    ) -> Self {
        self.toolsets = Some(v.into());
        self
    }
}
impl ToListMappable for CesAppVersionSnapshotEl {
    type O = BlockAssignable<CesAppVersionSnapshotEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionSnapshotEl {}
impl BuildCesAppVersionSnapshotEl {
    pub fn build(self) -> CesAppVersionSnapshotEl {
        CesAppVersionSnapshotEl {
            agents: core::default::Default::default(),
            app: core::default::Default::default(),
            examples: core::default::Default::default(),
            guardrails: core::default::Default::default(),
            tools: core::default::Default::default(),
            toolsets: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionSnapshotElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionSnapshotElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionSnapshotElRef {
        CesAppVersionSnapshotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionSnapshotElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `agents` after provisioning.\n"]
    pub fn agents(&self) -> ListRef<CesAppVersionSnapshotElAgentsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.agents", self.base))
    }
    #[doc = "Get a reference to the value of field `app` after provisioning.\n"]
    pub fn app(&self) -> ListRef<CesAppVersionSnapshotElAppElRef> {
        ListRef::new(self.shared().clone(), format!("{}.app", self.base))
    }
    #[doc = "Get a reference to the value of field `examples` after provisioning.\n"]
    pub fn examples(&self) -> ListRef<CesAppVersionSnapshotElExamplesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.examples", self.base))
    }
    #[doc = "Get a reference to the value of field `guardrails` after provisioning.\n"]
    pub fn guardrails(&self) -> ListRef<CesAppVersionSnapshotElGuardrailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.guardrails", self.base))
    }
    #[doc = "Get a reference to the value of field `tools` after provisioning.\n"]
    pub fn tools(&self) -> ListRef<CesAppVersionSnapshotElToolsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tools", self.base))
    }
    #[doc = "Get a reference to the value of field `toolsets` after provisioning.\n"]
    pub fn toolsets(&self) -> ListRef<CesAppVersionSnapshotElToolsetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.toolsets", self.base))
    }
}
#[derive(Serialize)]
pub struct CesAppVersionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl CesAppVersionTimeoutsEl {
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
}
impl ToListMappable for CesAppVersionTimeoutsEl {
    type O = BlockAssignable<CesAppVersionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesAppVersionTimeoutsEl {}
impl BuildCesAppVersionTimeoutsEl {
    pub fn build(self) -> CesAppVersionTimeoutsEl {
        CesAppVersionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct CesAppVersionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesAppVersionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CesAppVersionTimeoutsElRef {
        CesAppVersionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesAppVersionTimeoutsElRef {
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
}
