use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FirebaserulesRulesetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<FirebaserulesRulesetSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FirebaserulesRulesetTimeoutsEl>,
    dynamic: FirebaserulesRulesetDynamic,
}
struct FirebaserulesRuleset_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FirebaserulesRulesetData>,
}
#[derive(Clone)]
pub struct FirebaserulesRuleset(Rc<FirebaserulesRuleset_>);
impl FirebaserulesRuleset {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl Into<BlockAssignable<FirebaserulesRulesetSourceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FirebaserulesRulesetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time the `Ruleset` was created."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nOutput only. The metadata for this ruleset."]
    pub fn metadata(&self) -> ListRef<FirebaserulesRulesetMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. Name of the `Ruleset`. The ruleset_id is auto generated by the service. Format: `projects/{project_id}/rulesets/{ruleset_id}`"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<FirebaserulesRulesetSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaserulesRulesetTimeoutsElRef {
        FirebaserulesRulesetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for FirebaserulesRuleset {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FirebaserulesRuleset {}
impl ToListMappable for FirebaserulesRuleset {
    type O = ListRef<FirebaserulesRulesetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FirebaserulesRuleset_ {
    fn extract_resource_type(&self) -> String {
        "google_firebaserules_ruleset".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFirebaserulesRuleset {
    pub tf_id: String,
}
impl BuildFirebaserulesRuleset {
    pub fn build(self, stack: &mut Stack) -> FirebaserulesRuleset {
        let out = FirebaserulesRuleset(Rc::new(FirebaserulesRuleset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FirebaserulesRulesetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                source: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FirebaserulesRulesetRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaserulesRulesetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FirebaserulesRulesetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time the `Ruleset` was created."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nOutput only. The metadata for this ruleset."]
    pub fn metadata(&self) -> ListRef<FirebaserulesRulesetMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. Name of the `Ruleset`. The ruleset_id is auto generated by the service. Format: `projects/{project_id}/rulesets/{ruleset_id}`"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<FirebaserulesRulesetSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaserulesRulesetTimeoutsElRef {
        FirebaserulesRulesetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FirebaserulesRulesetMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    services: Option<ListField<PrimField<String>>>,
}
impl FirebaserulesRulesetMetadataEl {
    #[doc = "Set the field `services`.\n"]
    pub fn set_services(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.services = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaserulesRulesetMetadataEl {
    type O = BlockAssignable<FirebaserulesRulesetMetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaserulesRulesetMetadataEl {}
impl BuildFirebaserulesRulesetMetadataEl {
    pub fn build(self) -> FirebaserulesRulesetMetadataEl {
        FirebaserulesRulesetMetadataEl {
            services: core::default::Default::default(),
        }
    }
}
pub struct FirebaserulesRulesetMetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaserulesRulesetMetadataElRef {
    fn new(shared: StackShared, base: String) -> FirebaserulesRulesetMetadataElRef {
        FirebaserulesRulesetMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaserulesRulesetMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `services` after provisioning.\n"]
    pub fn services(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.services", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaserulesRulesetSourceElFilesEl {
    content: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fingerprint: Option<PrimField<String>>,
    name: PrimField<String>,
}
impl FirebaserulesRulesetSourceElFilesEl {
    #[doc = "Set the field `fingerprint`.\nFingerprint (e.g. github sha) associated with the `File`."]
    pub fn set_fingerprint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fingerprint = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaserulesRulesetSourceElFilesEl {
    type O = BlockAssignable<FirebaserulesRulesetSourceElFilesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaserulesRulesetSourceElFilesEl {
    #[doc = "Textual Content."]
    pub content: PrimField<String>,
    #[doc = "File name."]
    pub name: PrimField<String>,
}
impl BuildFirebaserulesRulesetSourceElFilesEl {
    pub fn build(self) -> FirebaserulesRulesetSourceElFilesEl {
        FirebaserulesRulesetSourceElFilesEl {
            content: self.content,
            fingerprint: core::default::Default::default(),
            name: self.name,
        }
    }
}
pub struct FirebaserulesRulesetSourceElFilesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaserulesRulesetSourceElFilesElRef {
    fn new(shared: StackShared, base: String) -> FirebaserulesRulesetSourceElFilesElRef {
        FirebaserulesRulesetSourceElFilesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaserulesRulesetSourceElFilesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content` after provisioning.\nTextual Content."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint (e.g. github sha) associated with the `File`."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nFile name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct FirebaserulesRulesetSourceElDynamic {
    files: Option<DynamicBlock<FirebaserulesRulesetSourceElFilesEl>>,
}
#[derive(Serialize)]
pub struct FirebaserulesRulesetSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<Vec<FirebaserulesRulesetSourceElFilesEl>>,
    dynamic: FirebaserulesRulesetSourceElDynamic,
}
impl FirebaserulesRulesetSourceEl {
    #[doc = "Set the field `language`.\n`Language` of the `Source` bundle. If unspecified, the language will default to `FIREBASE_RULES`. Possible values: LANGUAGE_UNSPECIFIED, FIREBASE_RULES, EVENT_FLOW_TRIGGERS"]
    pub fn set_language(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.language = Some(v.into());
        self
    }
    #[doc = "Set the field `files`.\n"]
    pub fn set_files(
        mut self,
        v: impl Into<BlockAssignable<FirebaserulesRulesetSourceElFilesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.files = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.files = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FirebaserulesRulesetSourceEl {
    type O = BlockAssignable<FirebaserulesRulesetSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaserulesRulesetSourceEl {}
impl BuildFirebaserulesRulesetSourceEl {
    pub fn build(self) -> FirebaserulesRulesetSourceEl {
        FirebaserulesRulesetSourceEl {
            language: core::default::Default::default(),
            files: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FirebaserulesRulesetSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaserulesRulesetSourceElRef {
    fn new(shared: StackShared, base: String) -> FirebaserulesRulesetSourceElRef {
        FirebaserulesRulesetSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaserulesRulesetSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `language` after provisioning.\n`Language` of the `Source` bundle. If unspecified, the language will default to `FIREBASE_RULES`. Possible values: LANGUAGE_UNSPECIFIED, FIREBASE_RULES, EVENT_FLOW_TRIGGERS"]
    pub fn language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.language", self.base))
    }
    #[doc = "Get a reference to the value of field `files` after provisioning.\n"]
    pub fn files(&self) -> ListRef<FirebaserulesRulesetSourceElFilesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.files", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaserulesRulesetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl FirebaserulesRulesetTimeoutsEl {
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
impl ToListMappable for FirebaserulesRulesetTimeoutsEl {
    type O = BlockAssignable<FirebaserulesRulesetTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaserulesRulesetTimeoutsEl {}
impl BuildFirebaserulesRulesetTimeoutsEl {
    pub fn build(self) -> FirebaserulesRulesetTimeoutsEl {
        FirebaserulesRulesetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct FirebaserulesRulesetTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaserulesRulesetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FirebaserulesRulesetTimeoutsElRef {
        FirebaserulesRulesetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaserulesRulesetTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct FirebaserulesRulesetDynamic {
    source: Option<DynamicBlock<FirebaserulesRulesetSourceEl>>,
}
