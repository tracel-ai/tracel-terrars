use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApphubApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    application_id: PrimField<String>,
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
    attributes: Option<Vec<ApphubApplicationAttributesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<Vec<ApphubApplicationScopeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApphubApplicationTimeoutsEl>,
    dynamic: ApphubApplicationDynamic,
}
struct ApphubApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApphubApplicationData>,
}
#[derive(Clone)]
pub struct ApphubApplication(Rc<ApphubApplication_>);
impl ApphubApplication {
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
    #[doc = "Set the field `description`.\nOptional. User-defined description of an Application."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nOptional. User-defined name for the Application."]
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
    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        self,
        v: impl Into<BlockAssignable<ApphubApplicationAttributesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attributes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attributes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(self, v: impl Into<BlockAssignable<ApphubApplicationScopeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scope = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scope = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApphubApplicationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `application_id` after provisioning.\nRequired. The Application identifier."]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Create time."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. User-defined description of an Application."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOptional. User-defined name for the Application."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'. See documentation of 'projectsId'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of an Application. Format:\n\"projects/{host-project-id}/locations/{location}/applications/{application-id}\""]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. Application state. \n Possible values:\n STATE_UNSPECIFIED\nCREATING\nACTIVE\nDELETING"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. A universally unique identifier (in UUID4 format) for the 'Application'."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Update time."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> ListRef<ApphubApplicationAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> ListRef<ApphubApplicationScopeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApphubApplicationTimeoutsElRef {
        ApphubApplicationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApphubApplication {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApphubApplication {}
impl ToListMappable for ApphubApplication {
    type O = ListRef<ApphubApplicationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApphubApplication_ {
    fn extract_resource_type(&self) -> String {
        "google_apphub_application".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApphubApplication {
    pub tf_id: String,
    #[doc = "Required. The Application identifier."]
    pub application_id: PrimField<String>,
    #[doc = "Part of 'parent'. See documentation of 'projectsId'."]
    pub location: PrimField<String>,
}
impl BuildApphubApplication {
    pub fn build(self, stack: &mut Stack) -> ApphubApplication {
        let out = ApphubApplication(Rc::new(ApphubApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApphubApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                application_id: self.application_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                attributes: core::default::Default::default(),
                scope: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApphubApplicationRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApphubApplicationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `application_id` after provisioning.\nRequired. The Application identifier."]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Create time."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. User-defined description of an Application."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOptional. User-defined name for the Application."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'. See documentation of 'projectsId'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of an Application. Format:\n\"projects/{host-project-id}/locations/{location}/applications/{application-id}\""]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. Application state. \n Possible values:\n STATE_UNSPECIFIED\nCREATING\nACTIVE\nDELETING"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. A universally unique identifier (in UUID4 format) for the 'Application'."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Update time."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> ListRef<ApphubApplicationAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> ListRef<ApphubApplicationScopeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApphubApplicationTimeoutsElRef {
        ApphubApplicationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApphubApplicationAttributesElBusinessOwnersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    email: PrimField<String>,
}
impl ApphubApplicationAttributesElBusinessOwnersEl {
    #[doc = "Set the field `display_name`.\nOptional. Contact's name."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubApplicationAttributesElBusinessOwnersEl {
    type O = BlockAssignable<ApphubApplicationAttributesElBusinessOwnersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubApplicationAttributesElBusinessOwnersEl {
    #[doc = "Required. Email address of the contacts."]
    pub email: PrimField<String>,
}
impl BuildApphubApplicationAttributesElBusinessOwnersEl {
    pub fn build(self) -> ApphubApplicationAttributesElBusinessOwnersEl {
        ApphubApplicationAttributesElBusinessOwnersEl {
            display_name: core::default::Default::default(),
            email: self.email,
        }
    }
}
pub struct ApphubApplicationAttributesElBusinessOwnersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubApplicationAttributesElBusinessOwnersElRef {
    fn new(shared: StackShared, base: String) -> ApphubApplicationAttributesElBusinessOwnersElRef {
        ApphubApplicationAttributesElBusinessOwnersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubApplicationAttributesElBusinessOwnersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOptional. Contact's name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nRequired. Email address of the contacts."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubApplicationAttributesElCriticalityEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl ApphubApplicationAttributesElCriticalityEl {}
impl ToListMappable for ApphubApplicationAttributesElCriticalityEl {
    type O = BlockAssignable<ApphubApplicationAttributesElCriticalityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubApplicationAttributesElCriticalityEl {
    #[doc = "Criticality type. Possible values: [\"MISSION_CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"]
    pub type_: PrimField<String>,
}
impl BuildApphubApplicationAttributesElCriticalityEl {
    pub fn build(self) -> ApphubApplicationAttributesElCriticalityEl {
        ApphubApplicationAttributesElCriticalityEl { type_: self.type_ }
    }
}
pub struct ApphubApplicationAttributesElCriticalityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubApplicationAttributesElCriticalityElRef {
    fn new(shared: StackShared, base: String) -> ApphubApplicationAttributesElCriticalityElRef {
        ApphubApplicationAttributesElCriticalityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubApplicationAttributesElCriticalityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nCriticality type. Possible values: [\"MISSION_CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubApplicationAttributesElDeveloperOwnersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    email: PrimField<String>,
}
impl ApphubApplicationAttributesElDeveloperOwnersEl {
    #[doc = "Set the field `display_name`.\nOptional. Contact's name."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubApplicationAttributesElDeveloperOwnersEl {
    type O = BlockAssignable<ApphubApplicationAttributesElDeveloperOwnersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubApplicationAttributesElDeveloperOwnersEl {
    #[doc = "Required. Email address of the contacts."]
    pub email: PrimField<String>,
}
impl BuildApphubApplicationAttributesElDeveloperOwnersEl {
    pub fn build(self) -> ApphubApplicationAttributesElDeveloperOwnersEl {
        ApphubApplicationAttributesElDeveloperOwnersEl {
            display_name: core::default::Default::default(),
            email: self.email,
        }
    }
}
pub struct ApphubApplicationAttributesElDeveloperOwnersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubApplicationAttributesElDeveloperOwnersElRef {
    fn new(shared: StackShared, base: String) -> ApphubApplicationAttributesElDeveloperOwnersElRef {
        ApphubApplicationAttributesElDeveloperOwnersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubApplicationAttributesElDeveloperOwnersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOptional. Contact's name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nRequired. Email address of the contacts."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubApplicationAttributesElEnvironmentEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl ApphubApplicationAttributesElEnvironmentEl {}
impl ToListMappable for ApphubApplicationAttributesElEnvironmentEl {
    type O = BlockAssignable<ApphubApplicationAttributesElEnvironmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubApplicationAttributesElEnvironmentEl {
    #[doc = "Environment type. Possible values: [\"PRODUCTION\", \"STAGING\", \"TEST\", \"DEVELOPMENT\"]"]
    pub type_: PrimField<String>,
}
impl BuildApphubApplicationAttributesElEnvironmentEl {
    pub fn build(self) -> ApphubApplicationAttributesElEnvironmentEl {
        ApphubApplicationAttributesElEnvironmentEl { type_: self.type_ }
    }
}
pub struct ApphubApplicationAttributesElEnvironmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubApplicationAttributesElEnvironmentElRef {
    fn new(shared: StackShared, base: String) -> ApphubApplicationAttributesElEnvironmentElRef {
        ApphubApplicationAttributesElEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubApplicationAttributesElEnvironmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nEnvironment type. Possible values: [\"PRODUCTION\", \"STAGING\", \"TEST\", \"DEVELOPMENT\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubApplicationAttributesElOperatorOwnersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    email: PrimField<String>,
}
impl ApphubApplicationAttributesElOperatorOwnersEl {
    #[doc = "Set the field `display_name`.\nOptional. Contact's name."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubApplicationAttributesElOperatorOwnersEl {
    type O = BlockAssignable<ApphubApplicationAttributesElOperatorOwnersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubApplicationAttributesElOperatorOwnersEl {
    #[doc = "Required. Email address of the contacts."]
    pub email: PrimField<String>,
}
impl BuildApphubApplicationAttributesElOperatorOwnersEl {
    pub fn build(self) -> ApphubApplicationAttributesElOperatorOwnersEl {
        ApphubApplicationAttributesElOperatorOwnersEl {
            display_name: core::default::Default::default(),
            email: self.email,
        }
    }
}
pub struct ApphubApplicationAttributesElOperatorOwnersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubApplicationAttributesElOperatorOwnersElRef {
    fn new(shared: StackShared, base: String) -> ApphubApplicationAttributesElOperatorOwnersElRef {
        ApphubApplicationAttributesElOperatorOwnersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubApplicationAttributesElOperatorOwnersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOptional. Contact's name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nRequired. Email address of the contacts."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApphubApplicationAttributesElDynamic {
    business_owners: Option<DynamicBlock<ApphubApplicationAttributesElBusinessOwnersEl>>,
    criticality: Option<DynamicBlock<ApphubApplicationAttributesElCriticalityEl>>,
    developer_owners: Option<DynamicBlock<ApphubApplicationAttributesElDeveloperOwnersEl>>,
    environment: Option<DynamicBlock<ApphubApplicationAttributesElEnvironmentEl>>,
    operator_owners: Option<DynamicBlock<ApphubApplicationAttributesElOperatorOwnersEl>>,
}
#[derive(Serialize)]
pub struct ApphubApplicationAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    business_owners: Option<Vec<ApphubApplicationAttributesElBusinessOwnersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    criticality: Option<Vec<ApphubApplicationAttributesElCriticalityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    developer_owners: Option<Vec<ApphubApplicationAttributesElDeveloperOwnersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<Vec<ApphubApplicationAttributesElEnvironmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator_owners: Option<Vec<ApphubApplicationAttributesElOperatorOwnersEl>>,
    dynamic: ApphubApplicationAttributesElDynamic,
}
impl ApphubApplicationAttributesEl {
    #[doc = "Set the field `business_owners`.\n"]
    pub fn set_business_owners(
        mut self,
        v: impl Into<BlockAssignable<ApphubApplicationAttributesElBusinessOwnersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.business_owners = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.business_owners = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `criticality`.\n"]
    pub fn set_criticality(
        mut self,
        v: impl Into<BlockAssignable<ApphubApplicationAttributesElCriticalityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.criticality = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.criticality = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `developer_owners`.\n"]
    pub fn set_developer_owners(
        mut self,
        v: impl Into<BlockAssignable<ApphubApplicationAttributesElDeveloperOwnersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.developer_owners = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.developer_owners = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl Into<BlockAssignable<ApphubApplicationAttributesElEnvironmentEl>>,
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
    #[doc = "Set the field `operator_owners`.\n"]
    pub fn set_operator_owners(
        mut self,
        v: impl Into<BlockAssignable<ApphubApplicationAttributesElOperatorOwnersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operator_owners = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operator_owners = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApphubApplicationAttributesEl {
    type O = BlockAssignable<ApphubApplicationAttributesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubApplicationAttributesEl {}
impl BuildApphubApplicationAttributesEl {
    pub fn build(self) -> ApphubApplicationAttributesEl {
        ApphubApplicationAttributesEl {
            business_owners: core::default::Default::default(),
            criticality: core::default::Default::default(),
            developer_owners: core::default::Default::default(),
            environment: core::default::Default::default(),
            operator_owners: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApphubApplicationAttributesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubApplicationAttributesElRef {
    fn new(shared: StackShared, base: String) -> ApphubApplicationAttributesElRef {
        ApphubApplicationAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubApplicationAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `business_owners` after provisioning.\n"]
    pub fn business_owners(&self) -> ListRef<ApphubApplicationAttributesElBusinessOwnersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.business_owners", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `criticality` after provisioning.\n"]
    pub fn criticality(&self) -> ListRef<ApphubApplicationAttributesElCriticalityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.criticality", self.base))
    }
    #[doc = "Get a reference to the value of field `developer_owners` after provisioning.\n"]
    pub fn developer_owners(&self) -> ListRef<ApphubApplicationAttributesElDeveloperOwnersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.developer_owners", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> ListRef<ApphubApplicationAttributesElEnvironmentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }
    #[doc = "Get a reference to the value of field `operator_owners` after provisioning.\n"]
    pub fn operator_owners(&self) -> ListRef<ApphubApplicationAttributesElOperatorOwnersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.operator_owners", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApphubApplicationScopeEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl ApphubApplicationScopeEl {}
impl ToListMappable for ApphubApplicationScopeEl {
    type O = BlockAssignable<ApphubApplicationScopeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubApplicationScopeEl {
    #[doc = "Required. Scope Type. \n Possible values:\nREGIONAL\nGLOBAL Possible values: [\"REGIONAL\", \"GLOBAL\"]"]
    pub type_: PrimField<String>,
}
impl BuildApphubApplicationScopeEl {
    pub fn build(self) -> ApphubApplicationScopeEl {
        ApphubApplicationScopeEl { type_: self.type_ }
    }
}
pub struct ApphubApplicationScopeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubApplicationScopeElRef {
    fn new(shared: StackShared, base: String) -> ApphubApplicationScopeElRef {
        ApphubApplicationScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubApplicationScopeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nRequired. Scope Type. \n Possible values:\nREGIONAL\nGLOBAL Possible values: [\"REGIONAL\", \"GLOBAL\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubApplicationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApphubApplicationTimeoutsEl {
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
impl ToListMappable for ApphubApplicationTimeoutsEl {
    type O = BlockAssignable<ApphubApplicationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubApplicationTimeoutsEl {}
impl BuildApphubApplicationTimeoutsEl {
    pub fn build(self) -> ApphubApplicationTimeoutsEl {
        ApphubApplicationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApphubApplicationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubApplicationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApphubApplicationTimeoutsElRef {
        ApphubApplicationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubApplicationTimeoutsElRef {
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
struct ApphubApplicationDynamic {
    attributes: Option<DynamicBlock<ApphubApplicationAttributesEl>>,
    scope: Option<DynamicBlock<ApphubApplicationScopeEl>>,
}
