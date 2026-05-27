use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ChronicleDataAccessScopeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_all: Option<PrimField<bool>>,
    data_access_scope_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance: PrimField<String>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_data_access_labels: Option<Vec<ChronicleDataAccessScopeAllowedDataAccessLabelsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    denied_data_access_labels: Option<Vec<ChronicleDataAccessScopeDeniedDataAccessLabelsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ChronicleDataAccessScopeTimeoutsEl>,
    dynamic: ChronicleDataAccessScopeDynamic,
}
struct ChronicleDataAccessScope_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ChronicleDataAccessScopeData>,
}
#[derive(Clone)]
pub struct ChronicleDataAccessScope(Rc<ChronicleDataAccessScope_>);
impl ChronicleDataAccessScope {
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
    #[doc = "Set the field `allow_all`.\nOptional. Whether or not the scope allows all labels, allow_all and\nallowed_data_access_labels are mutually exclusive and one of them must be\npresent. denied_data_access_labels can still be used along with allow_all.\nWhen combined with denied_data_access_labels, access will be granted to all\ndata that doesn't have labels mentioned in denied_data_access_labels. E.g.:\nA customer with scope with denied labels A and B and allow_all will be able\nto see all data except data labeled with A and data labeled with B and data\nwith labels A and B."]
    pub fn set_allow_all(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_all = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nOptional. A description of the data access scope for a human reader."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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
    #[doc = "Set the field `allowed_data_access_labels`.\n"]
    pub fn set_allowed_data_access_labels(
        self,
        v: impl Into<BlockAssignable<ChronicleDataAccessScopeAllowedDataAccessLabelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().allowed_data_access_labels = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.allowed_data_access_labels = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `denied_data_access_labels`.\n"]
    pub fn set_denied_data_access_labels(
        self,
        v: impl Into<BlockAssignable<ChronicleDataAccessScopeDeniedDataAccessLabelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().denied_data_access_labels = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.denied_data_access_labels = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ChronicleDataAccessScopeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `allow_all` after provisioning.\nOptional. Whether or not the scope allows all labels, allow_all and\nallowed_data_access_labels are mutually exclusive and one of them must be\npresent. denied_data_access_labels can still be used along with allow_all.\nWhen combined with denied_data_access_labels, access will be granted to all\ndata that doesn't have labels mentioned in denied_data_access_labels. E.g.:\nA customer with scope with denied labels A and B and allow_all will be able\nto see all data except data labeled with A and data labeled with B and data\nwith labels A and B."]
    pub fn allow_all(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `author` after provisioning.\nOutput only. The user who created the data access scope."]
    pub fn author(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.author", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which the data access scope was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_access_scope_id` after provisioning.\nRequired. The user provided scope id which will become the last part of the name\nof the scope resource.\nNeeds to be compliant with https://google.aip.dev/122"]
    pub fn data_access_scope_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_access_scope_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. A description of the data access scope for a human reader."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOutput only. The name to be used for display to customers of the data access scope."]
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
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_editor` after provisioning.\nOutput only. The user who last updated the data access scope."]
    pub fn last_editor(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_editor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full name of the data access scope. This unique identifier is generated using values provided for the URL parameters.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{data_access_scope_id}"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which the data access scope was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_data_access_labels` after provisioning.\n"]
    pub fn allowed_data_access_labels(
        &self,
    ) -> ListRef<ChronicleDataAccessScopeAllowedDataAccessLabelsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_data_access_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `denied_data_access_labels` after provisioning.\n"]
    pub fn denied_data_access_labels(
        &self,
    ) -> ListRef<ChronicleDataAccessScopeDeniedDataAccessLabelsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.denied_data_access_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleDataAccessScopeTimeoutsElRef {
        ChronicleDataAccessScopeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ChronicleDataAccessScope {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ChronicleDataAccessScope {}
impl ToListMappable for ChronicleDataAccessScope {
    type O = ListRef<ChronicleDataAccessScopeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ChronicleDataAccessScope_ {
    fn extract_resource_type(&self) -> String {
        "google_chronicle_data_access_scope".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildChronicleDataAccessScope {
    pub tf_id: String,
    #[doc = "Required. The user provided scope id which will become the last part of the name\nof the scope resource.\nNeeds to be compliant with https://google.aip.dev/122"]
    pub data_access_scope_id: PrimField<String>,
    #[doc = "The unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub instance: PrimField<String>,
    #[doc = "The location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub location: PrimField<String>,
}
impl BuildChronicleDataAccessScope {
    pub fn build(self, stack: &mut Stack) -> ChronicleDataAccessScope {
        let out = ChronicleDataAccessScope(Rc::new(ChronicleDataAccessScope_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ChronicleDataAccessScopeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allow_all: core::default::Default::default(),
                data_access_scope_id: self.data_access_scope_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                project: core::default::Default::default(),
                allowed_data_access_labels: core::default::Default::default(),
                denied_data_access_labels: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ChronicleDataAccessScopeRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDataAccessScopeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ChronicleDataAccessScopeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_all` after provisioning.\nOptional. Whether or not the scope allows all labels, allow_all and\nallowed_data_access_labels are mutually exclusive and one of them must be\npresent. denied_data_access_labels can still be used along with allow_all.\nWhen combined with denied_data_access_labels, access will be granted to all\ndata that doesn't have labels mentioned in denied_data_access_labels. E.g.:\nA customer with scope with denied labels A and B and allow_all will be able\nto see all data except data labeled with A and data labeled with B and data\nwith labels A and B."]
    pub fn allow_all(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `author` after provisioning.\nOutput only. The user who created the data access scope."]
    pub fn author(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.author", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which the data access scope was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_access_scope_id` after provisioning.\nRequired. The user provided scope id which will become the last part of the name\nof the scope resource.\nNeeds to be compliant with https://google.aip.dev/122"]
    pub fn data_access_scope_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_access_scope_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. A description of the data access scope for a human reader."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOutput only. The name to be used for display to customers of the data access scope."]
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
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_editor` after provisioning.\nOutput only. The user who last updated the data access scope."]
    pub fn last_editor(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_editor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full name of the data access scope. This unique identifier is generated using values provided for the URL parameters.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{data_access_scope_id}"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which the data access scope was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_data_access_labels` after provisioning.\n"]
    pub fn allowed_data_access_labels(
        &self,
    ) -> ListRef<ChronicleDataAccessScopeAllowedDataAccessLabelsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_data_access_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `denied_data_access_labels` after provisioning.\n"]
    pub fn denied_data_access_labels(
        &self,
    ) -> ListRef<ChronicleDataAccessScopeDeniedDataAccessLabelsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.denied_data_access_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleDataAccessScopeTimeoutsElRef {
        ChronicleDataAccessScopeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDataAccessScopeAllowedDataAccessLabelsElIngestionLabelEl {
    ingestion_label_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingestion_label_value: Option<PrimField<String>>,
}
impl ChronicleDataAccessScopeAllowedDataAccessLabelsElIngestionLabelEl {
    #[doc = "Set the field `ingestion_label_value`.\nOptional. The value of the ingestion label. Optional. An object\nwith no provided value and some key provided would match\nagainst the given key and ANY value."]
    pub fn set_ingestion_label_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ingestion_label_value = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDataAccessScopeAllowedDataAccessLabelsElIngestionLabelEl {
    type O = BlockAssignable<ChronicleDataAccessScopeAllowedDataAccessLabelsElIngestionLabelEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDataAccessScopeAllowedDataAccessLabelsElIngestionLabelEl {
    #[doc = "Required. The key of the ingestion label. Always required."]
    pub ingestion_label_key: PrimField<String>,
}
impl BuildChronicleDataAccessScopeAllowedDataAccessLabelsElIngestionLabelEl {
    pub fn build(self) -> ChronicleDataAccessScopeAllowedDataAccessLabelsElIngestionLabelEl {
        ChronicleDataAccessScopeAllowedDataAccessLabelsElIngestionLabelEl {
            ingestion_label_key: self.ingestion_label_key,
            ingestion_label_value: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDataAccessScopeAllowedDataAccessLabelsElIngestionLabelElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDataAccessScopeAllowedDataAccessLabelsElIngestionLabelElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDataAccessScopeAllowedDataAccessLabelsElIngestionLabelElRef {
        ChronicleDataAccessScopeAllowedDataAccessLabelsElIngestionLabelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDataAccessScopeAllowedDataAccessLabelsElIngestionLabelElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ingestion_label_key` after provisioning.\nRequired. The key of the ingestion label. Always required."]
    pub fn ingestion_label_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingestion_label_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ingestion_label_value` after provisioning.\nOptional. The value of the ingestion label. Optional. An object\nwith no provided value and some key provided would match\nagainst the given key and ANY value."]
    pub fn ingestion_label_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingestion_label_value", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ChronicleDataAccessScopeAllowedDataAccessLabelsElDynamic {
    ingestion_label:
        Option<DynamicBlock<ChronicleDataAccessScopeAllowedDataAccessLabelsElIngestionLabelEl>>,
}
#[derive(Serialize)]
pub struct ChronicleDataAccessScopeAllowedDataAccessLabelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_access_label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingestion_label: Option<Vec<ChronicleDataAccessScopeAllowedDataAccessLabelsElIngestionLabelEl>>,
    dynamic: ChronicleDataAccessScopeAllowedDataAccessLabelsElDynamic,
}
impl ChronicleDataAccessScopeAllowedDataAccessLabelsEl {
    #[doc = "Set the field `asset_namespace`.\nThe asset namespace configured in the forwarder\nof the customer's events."]
    pub fn set_asset_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.asset_namespace = Some(v.into());
        self
    }
    #[doc = "Set the field `data_access_label`.\nThe name of the data access label."]
    pub fn set_data_access_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_access_label = Some(v.into());
        self
    }
    #[doc = "Set the field `log_type`.\nThe name of the log type."]
    pub fn set_log_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_type = Some(v.into());
        self
    }
    #[doc = "Set the field `ingestion_label`.\n"]
    pub fn set_ingestion_label(
        mut self,
        v: impl Into<BlockAssignable<ChronicleDataAccessScopeAllowedDataAccessLabelsElIngestionLabelEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ingestion_label = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ingestion_label = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleDataAccessScopeAllowedDataAccessLabelsEl {
    type O = BlockAssignable<ChronicleDataAccessScopeAllowedDataAccessLabelsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDataAccessScopeAllowedDataAccessLabelsEl {}
impl BuildChronicleDataAccessScopeAllowedDataAccessLabelsEl {
    pub fn build(self) -> ChronicleDataAccessScopeAllowedDataAccessLabelsEl {
        ChronicleDataAccessScopeAllowedDataAccessLabelsEl {
            asset_namespace: core::default::Default::default(),
            data_access_label: core::default::Default::default(),
            log_type: core::default::Default::default(),
            ingestion_label: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDataAccessScopeAllowedDataAccessLabelsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDataAccessScopeAllowedDataAccessLabelsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDataAccessScopeAllowedDataAccessLabelsElRef {
        ChronicleDataAccessScopeAllowedDataAccessLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDataAccessScopeAllowedDataAccessLabelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `asset_namespace` after provisioning.\nThe asset namespace configured in the forwarder\nof the customer's events."]
    pub fn asset_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.asset_namespace", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_access_label` after provisioning.\nThe name of the data access label."]
    pub fn data_access_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_access_label", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOutput only. The display name of the label.\nData access label and log types's name\nwill match the display name of the resource.\nThe asset namespace will match the namespace itself.\nThe ingestion key value pair will match the key of the tuple."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `log_type` after provisioning.\nThe name of the log type."]
    pub fn log_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }
    #[doc = "Get a reference to the value of field `ingestion_label` after provisioning.\n"]
    pub fn ingestion_label(
        &self,
    ) -> ListRef<ChronicleDataAccessScopeAllowedDataAccessLabelsElIngestionLabelElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingestion_label", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDataAccessScopeDeniedDataAccessLabelsElIngestionLabelEl {
    ingestion_label_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingestion_label_value: Option<PrimField<String>>,
}
impl ChronicleDataAccessScopeDeniedDataAccessLabelsElIngestionLabelEl {
    #[doc = "Set the field `ingestion_label_value`.\nOptional. The value of the ingestion label. Optional. An object\nwith no provided value and some key provided would match\nagainst the given key and ANY value."]
    pub fn set_ingestion_label_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ingestion_label_value = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDataAccessScopeDeniedDataAccessLabelsElIngestionLabelEl {
    type O = BlockAssignable<ChronicleDataAccessScopeDeniedDataAccessLabelsElIngestionLabelEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDataAccessScopeDeniedDataAccessLabelsElIngestionLabelEl {
    #[doc = "Required. The key of the ingestion label. Always required."]
    pub ingestion_label_key: PrimField<String>,
}
impl BuildChronicleDataAccessScopeDeniedDataAccessLabelsElIngestionLabelEl {
    pub fn build(self) -> ChronicleDataAccessScopeDeniedDataAccessLabelsElIngestionLabelEl {
        ChronicleDataAccessScopeDeniedDataAccessLabelsElIngestionLabelEl {
            ingestion_label_key: self.ingestion_label_key,
            ingestion_label_value: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDataAccessScopeDeniedDataAccessLabelsElIngestionLabelElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDataAccessScopeDeniedDataAccessLabelsElIngestionLabelElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDataAccessScopeDeniedDataAccessLabelsElIngestionLabelElRef {
        ChronicleDataAccessScopeDeniedDataAccessLabelsElIngestionLabelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDataAccessScopeDeniedDataAccessLabelsElIngestionLabelElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ingestion_label_key` after provisioning.\nRequired. The key of the ingestion label. Always required."]
    pub fn ingestion_label_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingestion_label_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ingestion_label_value` after provisioning.\nOptional. The value of the ingestion label. Optional. An object\nwith no provided value and some key provided would match\nagainst the given key and ANY value."]
    pub fn ingestion_label_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ingestion_label_value", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ChronicleDataAccessScopeDeniedDataAccessLabelsElDynamic {
    ingestion_label:
        Option<DynamicBlock<ChronicleDataAccessScopeDeniedDataAccessLabelsElIngestionLabelEl>>,
}
#[derive(Serialize)]
pub struct ChronicleDataAccessScopeDeniedDataAccessLabelsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_access_label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingestion_label: Option<Vec<ChronicleDataAccessScopeDeniedDataAccessLabelsElIngestionLabelEl>>,
    dynamic: ChronicleDataAccessScopeDeniedDataAccessLabelsElDynamic,
}
impl ChronicleDataAccessScopeDeniedDataAccessLabelsEl {
    #[doc = "Set the field `asset_namespace`.\nThe asset namespace configured in the forwarder\nof the customer's events."]
    pub fn set_asset_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.asset_namespace = Some(v.into());
        self
    }
    #[doc = "Set the field `data_access_label`.\nThe name of the data access label."]
    pub fn set_data_access_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_access_label = Some(v.into());
        self
    }
    #[doc = "Set the field `log_type`.\nThe name of the log type."]
    pub fn set_log_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_type = Some(v.into());
        self
    }
    #[doc = "Set the field `ingestion_label`.\n"]
    pub fn set_ingestion_label(
        mut self,
        v: impl Into<BlockAssignable<ChronicleDataAccessScopeDeniedDataAccessLabelsElIngestionLabelEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ingestion_label = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ingestion_label = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleDataAccessScopeDeniedDataAccessLabelsEl {
    type O = BlockAssignable<ChronicleDataAccessScopeDeniedDataAccessLabelsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDataAccessScopeDeniedDataAccessLabelsEl {}
impl BuildChronicleDataAccessScopeDeniedDataAccessLabelsEl {
    pub fn build(self) -> ChronicleDataAccessScopeDeniedDataAccessLabelsEl {
        ChronicleDataAccessScopeDeniedDataAccessLabelsEl {
            asset_namespace: core::default::Default::default(),
            data_access_label: core::default::Default::default(),
            log_type: core::default::Default::default(),
            ingestion_label: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleDataAccessScopeDeniedDataAccessLabelsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDataAccessScopeDeniedDataAccessLabelsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleDataAccessScopeDeniedDataAccessLabelsElRef {
        ChronicleDataAccessScopeDeniedDataAccessLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDataAccessScopeDeniedDataAccessLabelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `asset_namespace` after provisioning.\nThe asset namespace configured in the forwarder\nof the customer's events."]
    pub fn asset_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.asset_namespace", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_access_label` after provisioning.\nThe name of the data access label."]
    pub fn data_access_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_access_label", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOutput only. The display name of the label.\nData access label and log types's name\nwill match the display name of the resource.\nThe asset namespace will match the namespace itself.\nThe ingestion key value pair will match the key of the tuple."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `log_type` after provisioning.\nThe name of the log type."]
    pub fn log_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }
    #[doc = "Get a reference to the value of field `ingestion_label` after provisioning.\n"]
    pub fn ingestion_label(
        &self,
    ) -> ListRef<ChronicleDataAccessScopeDeniedDataAccessLabelsElIngestionLabelElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingestion_label", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDataAccessScopeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ChronicleDataAccessScopeTimeoutsEl {
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
impl ToListMappable for ChronicleDataAccessScopeTimeoutsEl {
    type O = BlockAssignable<ChronicleDataAccessScopeTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDataAccessScopeTimeoutsEl {}
impl BuildChronicleDataAccessScopeTimeoutsEl {
    pub fn build(self) -> ChronicleDataAccessScopeTimeoutsEl {
        ChronicleDataAccessScopeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDataAccessScopeTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDataAccessScopeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ChronicleDataAccessScopeTimeoutsElRef {
        ChronicleDataAccessScopeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDataAccessScopeTimeoutsElRef {
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
struct ChronicleDataAccessScopeDynamic {
    allowed_data_access_labels:
        Option<DynamicBlock<ChronicleDataAccessScopeAllowedDataAccessLabelsEl>>,
    denied_data_access_labels:
        Option<DynamicBlock<ChronicleDataAccessScopeDeniedDataAccessLabelsEl>>,
}
