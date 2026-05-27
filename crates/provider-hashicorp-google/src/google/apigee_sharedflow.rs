use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApigeeSharedflowData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    config_bundle: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detect_md5hash: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    org_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeSharedflowTimeoutsEl>,
}
struct ApigeeSharedflow_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeSharedflowData>,
}
#[derive(Clone)]
pub struct ApigeeSharedflow(Rc<ApigeeSharedflow_>);
impl ApigeeSharedflow {
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
    #[doc = "Set the field `detect_md5hash`.\nA hash of local config bundle in string, user needs to use a Terraform Hash function of their choice. A change in hash will trigger an update."]
    pub fn set_detect_md5hash(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().detect_md5hash = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApigeeSharedflowTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `config_bundle` after provisioning.\nPath to the config zip bundle"]
    pub fn config_bundle(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_bundle", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `detect_md5hash` after provisioning.\nA hash of local config bundle in string, user needs to use a Terraform Hash function of their choice. A change in hash will trigger an update."]
    pub fn detect_md5hash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.detect_md5hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `latest_revision_id` after provisioning.\nThe id of the most recently created revision for this shared flow."]
    pub fn latest_revision_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_revision_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `md5hash` after provisioning.\nBase 64 MD5 hash of the uploaded config bundle."]
    pub fn md5hash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.md5hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `meta_data` after provisioning.\nMetadata describing the shared flow."]
    pub fn meta_data(&self) -> ListRef<ApigeeSharedflowMetaDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.meta_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the shared flow."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization name associated with the Apigee instance."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\nA list of revisions of this shared flow."]
    pub fn revision(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeSharedflowTimeoutsElRef {
        ApigeeSharedflowTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApigeeSharedflow {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApigeeSharedflow {}
impl ToListMappable for ApigeeSharedflow {
    type O = ListRef<ApigeeSharedflowRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApigeeSharedflow_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_sharedflow".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApigeeSharedflow {
    pub tf_id: String,
    #[doc = "Path to the config zip bundle"]
    pub config_bundle: PrimField<String>,
    #[doc = "The ID of the shared flow."]
    pub name: PrimField<String>,
    #[doc = "The Apigee Organization name associated with the Apigee instance."]
    pub org_id: PrimField<String>,
}
impl BuildApigeeSharedflow {
    pub fn build(self, stack: &mut Stack) -> ApigeeSharedflow {
        let out = ApigeeSharedflow(Rc::new(ApigeeSharedflow_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeSharedflowData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                config_bundle: self.config_bundle,
                deletion_policy: core::default::Default::default(),
                detect_md5hash: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                org_id: self.org_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApigeeSharedflowRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeSharedflowRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApigeeSharedflowRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `config_bundle` after provisioning.\nPath to the config zip bundle"]
    pub fn config_bundle(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.config_bundle", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `detect_md5hash` after provisioning.\nA hash of local config bundle in string, user needs to use a Terraform Hash function of their choice. A change in hash will trigger an update."]
    pub fn detect_md5hash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.detect_md5hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `latest_revision_id` after provisioning.\nThe id of the most recently created revision for this shared flow."]
    pub fn latest_revision_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_revision_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `md5hash` after provisioning.\nBase 64 MD5 hash of the uploaded config bundle."]
    pub fn md5hash(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.md5hash", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `meta_data` after provisioning.\nMetadata describing the shared flow."]
    pub fn meta_data(&self) -> ListRef<ApigeeSharedflowMetaDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.meta_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the shared flow."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization name associated with the Apigee instance."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\nA list of revisions of this shared flow."]
    pub fn revision(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeSharedflowTimeoutsElRef {
        ApigeeSharedflowTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeSharedflowMetaDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_type: Option<PrimField<String>>,
}
impl ApigeeSharedflowMetaDataEl {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }
    #[doc = "Set the field `last_modified_at`.\n"]
    pub fn set_last_modified_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_modified_at = Some(v.into());
        self
    }
    #[doc = "Set the field `sub_type`.\n"]
    pub fn set_sub_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sub_type = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeSharedflowMetaDataEl {
    type O = BlockAssignable<ApigeeSharedflowMetaDataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeSharedflowMetaDataEl {}
impl BuildApigeeSharedflowMetaDataEl {
    pub fn build(self) -> ApigeeSharedflowMetaDataEl {
        ApigeeSharedflowMetaDataEl {
            created_at: core::default::Default::default(),
            last_modified_at: core::default::Default::default(),
            sub_type: core::default::Default::default(),
        }
    }
}
pub struct ApigeeSharedflowMetaDataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeSharedflowMetaDataElRef {
    fn new(shared: StackShared, base: String) -> ApigeeSharedflowMetaDataElRef {
        ApigeeSharedflowMetaDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeSharedflowMetaDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }
    #[doc = "Get a reference to the value of field `last_modified_at` after provisioning.\n"]
    pub fn last_modified_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_at", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sub_type` after provisioning.\n"]
    pub fn sub_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sub_type", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeSharedflowTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApigeeSharedflowTimeoutsEl {
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
impl ToListMappable for ApigeeSharedflowTimeoutsEl {
    type O = BlockAssignable<ApigeeSharedflowTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeSharedflowTimeoutsEl {}
impl BuildApigeeSharedflowTimeoutsEl {
    pub fn build(self) -> ApigeeSharedflowTimeoutsEl {
        ApigeeSharedflowTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApigeeSharedflowTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeSharedflowTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeSharedflowTimeoutsElRef {
        ApigeeSharedflowTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeSharedflowTimeoutsElRef {
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
