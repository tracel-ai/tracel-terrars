use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VertexAiIndexData {
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
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index_update_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_spec: Option<Vec<VertexAiIndexEncryptionSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Vec<VertexAiIndexMetadataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VertexAiIndexTimeoutsEl>,
    dynamic: VertexAiIndexDynamic,
}
struct VertexAiIndex_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VertexAiIndexData>,
}
#[derive(Clone)]
pub struct VertexAiIndex(Rc<VertexAiIndex_>);
impl VertexAiIndex {
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
    #[doc = "Set the field `description`.\nThe description of the Index."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `index_update_method`.\nThe update method to use with this Index. The value must be the followings. If not set, BATCH_UPDATE will be used by default.\n* BATCH_UPDATE: user can call indexes.patch with files on Cloud Storage of datapoints to update.\n* STREAM_UPDATE: user can call indexes.upsertDatapoints/DeleteDatapoints to update the Index and the updates will be applied in corresponding DeployedIndexes in nearly real-time."]
    pub fn set_index_update_method(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().index_update_method = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe labels with user-defined metadata to organize your Indexes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region of the index. eg us-central1"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        self,
        v: impl Into<BlockAssignable<VertexAiIndexEncryptionSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(self, v: impl Into<BlockAssignable<VertexAiIndexMetadataEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VertexAiIndexTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the Index was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
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
    #[doc = "Get a reference to the value of field `deployed_indexes` after provisioning.\nThe pointers to DeployedIndexes created from this Index. An Index can be only deleted if all its DeployedIndexes had been undeployed first."]
    pub fn deployed_indexes(&self) -> ListRef<VertexAiIndexDeployedIndexesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployed_indexes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the Index."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates."]
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
    #[doc = "Get a reference to the value of field `index_stats` after provisioning.\nStats of the index resource."]
    pub fn index_stats(&self) -> ListRef<VertexAiIndexIndexStatsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.index_stats", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `index_update_method` after provisioning.\nThe update method to use with this Index. The value must be the followings. If not set, BATCH_UPDATE will be used by default.\n* BATCH_UPDATE: user can call indexes.patch with files on Cloud Storage of datapoints to update.\n* STREAM_UPDATE: user can call indexes.upsertDatapoints/DeleteDatapoints to update the Index and the updates will be applied in corresponding DeployedIndexes in nearly real-time."]
    pub fn index_update_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.index_update_method", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize your Indexes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_schema_uri` after provisioning.\nPoints to a YAML file stored on Google Cloud Storage describing additional information about the Index, that is specific to it. Unset if the Index does not have any additional information."]
    pub fn metadata_schema_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata_schema_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Index."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the index. eg us-central1"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the Index was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> ListRef<VertexAiIndexEncryptionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> ListRef<VertexAiIndexMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiIndexTimeoutsElRef {
        VertexAiIndexTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VertexAiIndex {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VertexAiIndex {}
impl ToListMappable for VertexAiIndex {
    type O = ListRef<VertexAiIndexRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VertexAiIndex_ {
    fn extract_resource_type(&self) -> String {
        "google_vertex_ai_index".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVertexAiIndex {
    pub tf_id: String,
    #[doc = "The display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub display_name: PrimField<String>,
}
impl BuildVertexAiIndex {
    pub fn build(self, stack: &mut Stack) -> VertexAiIndex {
        let out = VertexAiIndex(Rc::new(VertexAiIndex_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VertexAiIndexData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                index_update_method: core::default::Default::default(),
                labels: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                encryption_spec: core::default::Default::default(),
                metadata: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VertexAiIndexRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VertexAiIndexRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the Index was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
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
    #[doc = "Get a reference to the value of field `deployed_indexes` after provisioning.\nThe pointers to DeployedIndexes created from this Index. An Index can be only deleted if all its DeployedIndexes had been undeployed first."]
    pub fn deployed_indexes(&self) -> ListRef<VertexAiIndexDeployedIndexesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployed_indexes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the Index."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates."]
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
    #[doc = "Get a reference to the value of field `index_stats` after provisioning.\nStats of the index resource."]
    pub fn index_stats(&self) -> ListRef<VertexAiIndexIndexStatsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.index_stats", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `index_update_method` after provisioning.\nThe update method to use with this Index. The value must be the followings. If not set, BATCH_UPDATE will be used by default.\n* BATCH_UPDATE: user can call indexes.patch with files on Cloud Storage of datapoints to update.\n* STREAM_UPDATE: user can call indexes.upsertDatapoints/DeleteDatapoints to update the Index and the updates will be applied in corresponding DeployedIndexes in nearly real-time."]
    pub fn index_update_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.index_update_method", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize your Indexes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_schema_uri` after provisioning.\nPoints to a YAML file stored on Google Cloud Storage describing additional information about the Index, that is specific to it. Unset if the Index does not have any additional information."]
    pub fn metadata_schema_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata_schema_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Index."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the index. eg us-central1"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the Index was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> ListRef<VertexAiIndexEncryptionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> ListRef<VertexAiIndexMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiIndexTimeoutsElRef {
        VertexAiIndexTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiIndexDeployedIndexesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deployed_index_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index_endpoint: Option<PrimField<String>>,
}
impl VertexAiIndexDeployedIndexesEl {
    #[doc = "Set the field `deployed_index_id`.\n"]
    pub fn set_deployed_index_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deployed_index_id = Some(v.into());
        self
    }
    #[doc = "Set the field `index_endpoint`.\n"]
    pub fn set_index_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.index_endpoint = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiIndexDeployedIndexesEl {
    type O = BlockAssignable<VertexAiIndexDeployedIndexesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexDeployedIndexesEl {}
impl BuildVertexAiIndexDeployedIndexesEl {
    pub fn build(self) -> VertexAiIndexDeployedIndexesEl {
        VertexAiIndexDeployedIndexesEl {
            deployed_index_id: core::default::Default::default(),
            index_endpoint: core::default::Default::default(),
        }
    }
}
pub struct VertexAiIndexDeployedIndexesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexDeployedIndexesElRef {
    fn new(shared: StackShared, base: String) -> VertexAiIndexDeployedIndexesElRef {
        VertexAiIndexDeployedIndexesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexDeployedIndexesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deployed_index_id` after provisioning.\n"]
    pub fn deployed_index_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployed_index_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `index_endpoint` after provisioning.\n"]
    pub fn index_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.index_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiIndexIndexStatsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    shards_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vectors_count: Option<PrimField<String>>,
}
impl VertexAiIndexIndexStatsEl {
    #[doc = "Set the field `shards_count`.\n"]
    pub fn set_shards_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.shards_count = Some(v.into());
        self
    }
    #[doc = "Set the field `vectors_count`.\n"]
    pub fn set_vectors_count(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vectors_count = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiIndexIndexStatsEl {
    type O = BlockAssignable<VertexAiIndexIndexStatsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexIndexStatsEl {}
impl BuildVertexAiIndexIndexStatsEl {
    pub fn build(self) -> VertexAiIndexIndexStatsEl {
        VertexAiIndexIndexStatsEl {
            shards_count: core::default::Default::default(),
            vectors_count: core::default::Default::default(),
        }
    }
}
pub struct VertexAiIndexIndexStatsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexIndexStatsElRef {
    fn new(shared: StackShared, base: String) -> VertexAiIndexIndexStatsElRef {
        VertexAiIndexIndexStatsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexIndexStatsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `shards_count` after provisioning.\n"]
    pub fn shards_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.shards_count", self.base))
    }
    #[doc = "Get a reference to the value of field `vectors_count` after provisioning.\n"]
    pub fn vectors_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vectors_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiIndexEncryptionSpecEl {
    kms_key_name: PrimField<String>,
}
impl VertexAiIndexEncryptionSpecEl {}
impl ToListMappable for VertexAiIndexEncryptionSpecEl {
    type O = BlockAssignable<VertexAiIndexEncryptionSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexEncryptionSpecEl {
    #[doc = "Required. The Cloud KMS resource identifier of the customer managed encryption key used to protect a resource. Has the form: 'projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key'. The key needs to be in the same region as where the compute resource is created."]
    pub kms_key_name: PrimField<String>,
}
impl BuildVertexAiIndexEncryptionSpecEl {
    pub fn build(self) -> VertexAiIndexEncryptionSpecEl {
        VertexAiIndexEncryptionSpecEl {
            kms_key_name: self.kms_key_name,
        }
    }
}
pub struct VertexAiIndexEncryptionSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexEncryptionSpecElRef {
    fn new(shared: StackShared, base: String) -> VertexAiIndexEncryptionSpecElRef {
        VertexAiIndexEncryptionSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexEncryptionSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nRequired. The Cloud KMS resource identifier of the customer managed encryption key used to protect a resource. Has the form: 'projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key'. The key needs to be in the same region as where the compute resource is created."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {}
impl VertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {}
impl ToListMappable for VertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {
    type O = BlockAssignable<VertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {}
impl BuildVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {
    pub fn build(self) -> VertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {
        VertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {}
    }
}
pub struct VertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef {
        VertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct VertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    leaf_node_embedding_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    leaf_nodes_to_search_percent: Option<PrimField<f64>>,
}
impl VertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
    #[doc = "Set the field `leaf_node_embedding_count`.\nNumber of embeddings on each leaf node. The default value is 1000 if not set."]
    pub fn set_leaf_node_embedding_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.leaf_node_embedding_count = Some(v.into());
        self
    }
    #[doc = "Set the field `leaf_nodes_to_search_percent`.\nThe default percentage of leaf nodes that any query may be searched. Must be in\nrange 1-100, inclusive. The default value is 10 (means 10%) if not set."]
    pub fn set_leaf_nodes_to_search_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.leaf_nodes_to_search_percent = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
    type O = BlockAssignable<VertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {}
impl BuildVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
    pub fn build(self) -> VertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
        VertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
            leaf_node_embedding_count: core::default::Default::default(),
            leaf_nodes_to_search_percent: core::default::Default::default(),
        }
    }
}
pub struct VertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef {
        VertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `leaf_node_embedding_count` after provisioning.\nNumber of embeddings on each leaf node. The default value is 1000 if not set."]
    pub fn leaf_node_embedding_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.leaf_node_embedding_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `leaf_nodes_to_search_percent` after provisioning.\nThe default percentage of leaf nodes that any query may be searched. Must be in\nrange 1-100, inclusive. The default value is 10 (means 10%) if not set."]
    pub fn leaf_nodes_to_search_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.leaf_nodes_to_search_percent", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct VertexAiIndexMetadataElConfigElAlgorithmConfigElDynamic {
    brute_force_config:
        Option<DynamicBlock<VertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl>>,
    tree_ah_config:
        Option<DynamicBlock<VertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl>>,
}
#[derive(Serialize)]
pub struct VertexAiIndexMetadataElConfigElAlgorithmConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    brute_force_config:
        Option<Vec<VertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tree_ah_config: Option<Vec<VertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl>>,
    dynamic: VertexAiIndexMetadataElConfigElAlgorithmConfigElDynamic,
}
impl VertexAiIndexMetadataElConfigElAlgorithmConfigEl {
    #[doc = "Set the field `brute_force_config`.\n"]
    pub fn set_brute_force_config(
        mut self,
        v: impl Into<
            BlockAssignable<VertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.brute_force_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.brute_force_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tree_ah_config`.\n"]
    pub fn set_tree_ah_config(
        mut self,
        v: impl Into<BlockAssignable<VertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tree_ah_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tree_ah_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiIndexMetadataElConfigElAlgorithmConfigEl {
    type O = BlockAssignable<VertexAiIndexMetadataElConfigElAlgorithmConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexMetadataElConfigElAlgorithmConfigEl {}
impl BuildVertexAiIndexMetadataElConfigElAlgorithmConfigEl {
    pub fn build(self) -> VertexAiIndexMetadataElConfigElAlgorithmConfigEl {
        VertexAiIndexMetadataElConfigElAlgorithmConfigEl {
            brute_force_config: core::default::Default::default(),
            tree_ah_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiIndexMetadataElConfigElAlgorithmConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexMetadataElConfigElAlgorithmConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiIndexMetadataElConfigElAlgorithmConfigElRef {
        VertexAiIndexMetadataElConfigElAlgorithmConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexMetadataElConfigElAlgorithmConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `brute_force_config` after provisioning.\n"]
    pub fn brute_force_config(
        &self,
    ) -> ListRef<VertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.brute_force_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tree_ah_config` after provisioning.\n"]
    pub fn tree_ah_config(
        &self,
    ) -> ListRef<VertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tree_ah_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct VertexAiIndexMetadataElConfigElDynamic {
    algorithm_config: Option<DynamicBlock<VertexAiIndexMetadataElConfigElAlgorithmConfigEl>>,
}
#[derive(Serialize)]
pub struct VertexAiIndexMetadataElConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    approximate_neighbors_count: Option<PrimField<f64>>,
    dimensions: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    distance_measure_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feature_norm_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shard_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm_config: Option<Vec<VertexAiIndexMetadataElConfigElAlgorithmConfigEl>>,
    dynamic: VertexAiIndexMetadataElConfigElDynamic,
}
impl VertexAiIndexMetadataElConfigEl {
    #[doc = "Set the field `approximate_neighbors_count`.\nThe default number of neighbors to find via approximate search before exact reordering is\nperformed. Exact reordering is a procedure where results returned by an\napproximate search algorithm are reordered via a more expensive distance computation.\nRequired if tree-AH algorithm is used."]
    pub fn set_approximate_neighbors_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.approximate_neighbors_count = Some(v.into());
        self
    }
    #[doc = "Set the field `distance_measure_type`.\nThe distance measure used in nearest neighbor search. The value must be one of the followings:\n* SQUARED_L2_DISTANCE: Euclidean (L_2) Distance\n* L1_DISTANCE: Manhattan (L_1) Distance\n* COSINE_DISTANCE: Cosine Distance. Defined as 1 - cosine similarity.\n* DOT_PRODUCT_DISTANCE: Dot Product Distance. Defined as a negative of the dot product"]
    pub fn set_distance_measure_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.distance_measure_type = Some(v.into());
        self
    }
    #[doc = "Set the field `feature_norm_type`.\nType of normalization to be carried out on each vector. The value must be one of the followings:\n* UNIT_L2_NORM: Unit L2 normalization type\n* NONE: No normalization type is specified."]
    pub fn set_feature_norm_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.feature_norm_type = Some(v.into());
        self
    }
    #[doc = "Set the field `shard_size`.\nIndex data is split into equal parts to be processed. These are called \"shards\".\nThe shard size must be specified when creating an index. The value must be one of the followings:\n* SHARD_SIZE_SMALL: Small (2GB)\n* SHARD_SIZE_MEDIUM: Medium (20GB)\n* SHARD_SIZE_LARGE: Large (50GB)"]
    pub fn set_shard_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.shard_size = Some(v.into());
        self
    }
    #[doc = "Set the field `algorithm_config`.\n"]
    pub fn set_algorithm_config(
        mut self,
        v: impl Into<BlockAssignable<VertexAiIndexMetadataElConfigElAlgorithmConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.algorithm_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.algorithm_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiIndexMetadataElConfigEl {
    type O = BlockAssignable<VertexAiIndexMetadataElConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexMetadataElConfigEl {
    #[doc = "The number of dimensions of the input vectors."]
    pub dimensions: PrimField<f64>,
}
impl BuildVertexAiIndexMetadataElConfigEl {
    pub fn build(self) -> VertexAiIndexMetadataElConfigEl {
        VertexAiIndexMetadataElConfigEl {
            approximate_neighbors_count: core::default::Default::default(),
            dimensions: self.dimensions,
            distance_measure_type: core::default::Default::default(),
            feature_norm_type: core::default::Default::default(),
            shard_size: core::default::Default::default(),
            algorithm_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiIndexMetadataElConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexMetadataElConfigElRef {
    fn new(shared: StackShared, base: String) -> VertexAiIndexMetadataElConfigElRef {
        VertexAiIndexMetadataElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexMetadataElConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `approximate_neighbors_count` after provisioning.\nThe default number of neighbors to find via approximate search before exact reordering is\nperformed. Exact reordering is a procedure where results returned by an\napproximate search algorithm are reordered via a more expensive distance computation.\nRequired if tree-AH algorithm is used."]
    pub fn approximate_neighbors_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approximate_neighbors_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\nThe number of dimensions of the input vectors."]
    pub fn dimensions(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }
    #[doc = "Get a reference to the value of field `distance_measure_type` after provisioning.\nThe distance measure used in nearest neighbor search. The value must be one of the followings:\n* SQUARED_L2_DISTANCE: Euclidean (L_2) Distance\n* L1_DISTANCE: Manhattan (L_1) Distance\n* COSINE_DISTANCE: Cosine Distance. Defined as 1 - cosine similarity.\n* DOT_PRODUCT_DISTANCE: Dot Product Distance. Defined as a negative of the dot product"]
    pub fn distance_measure_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.distance_measure_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `feature_norm_type` after provisioning.\nType of normalization to be carried out on each vector. The value must be one of the followings:\n* UNIT_L2_NORM: Unit L2 normalization type\n* NONE: No normalization type is specified."]
    pub fn feature_norm_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.feature_norm_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shard_size` after provisioning.\nIndex data is split into equal parts to be processed. These are called \"shards\".\nThe shard size must be specified when creating an index. The value must be one of the followings:\n* SHARD_SIZE_SMALL: Small (2GB)\n* SHARD_SIZE_MEDIUM: Medium (20GB)\n* SHARD_SIZE_LARGE: Large (50GB)"]
    pub fn shard_size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shard_size", self.base))
    }
    #[doc = "Get a reference to the value of field `algorithm_config` after provisioning.\n"]
    pub fn algorithm_config(&self) -> ListRef<VertexAiIndexMetadataElConfigElAlgorithmConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.algorithm_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct VertexAiIndexMetadataElDynamic {
    config: Option<DynamicBlock<VertexAiIndexMetadataElConfigEl>>,
}
#[derive(Serialize)]
pub struct VertexAiIndexMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contents_delta_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_complete_overwrite: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<VertexAiIndexMetadataElConfigEl>>,
    dynamic: VertexAiIndexMetadataElDynamic,
}
impl VertexAiIndexMetadataEl {
    #[doc = "Set the field `contents_delta_uri`.\nAllows inserting, updating  or deleting the contents of the Matching Engine Index.\nThe string must be a valid Cloud Storage directory path. If this\nfield is set when calling IndexService.UpdateIndex, then no other\nIndex field can be also updated as part of the same call.\nThe expected structure and format of the files this URI points to is\ndescribed at https://cloud.google.com/vertex-ai/docs/matching-engine/using-matching-engine#input-data-format"]
    pub fn set_contents_delta_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contents_delta_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `is_complete_overwrite`.\nIf this field is set together with contentsDeltaUri when calling IndexService.UpdateIndex,\nthen existing content of the Index will be replaced by the data from the contentsDeltaUri."]
    pub fn set_is_complete_overwrite(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_complete_overwrite = Some(v.into());
        self
    }
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl Into<BlockAssignable<VertexAiIndexMetadataElConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiIndexMetadataEl {
    type O = BlockAssignable<VertexAiIndexMetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexMetadataEl {}
impl BuildVertexAiIndexMetadataEl {
    pub fn build(self) -> VertexAiIndexMetadataEl {
        VertexAiIndexMetadataEl {
            contents_delta_uri: core::default::Default::default(),
            is_complete_overwrite: core::default::Default::default(),
            config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiIndexMetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexMetadataElRef {
    fn new(shared: StackShared, base: String) -> VertexAiIndexMetadataElRef {
        VertexAiIndexMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contents_delta_uri` after provisioning.\nAllows inserting, updating  or deleting the contents of the Matching Engine Index.\nThe string must be a valid Cloud Storage directory path. If this\nfield is set when calling IndexService.UpdateIndex, then no other\nIndex field can be also updated as part of the same call.\nThe expected structure and format of the files this URI points to is\ndescribed at https://cloud.google.com/vertex-ai/docs/matching-engine/using-matching-engine#input-data-format"]
    pub fn contents_delta_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contents_delta_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_complete_overwrite` after provisioning.\nIf this field is set together with contentsDeltaUri when calling IndexService.UpdateIndex,\nthen existing content of the Index will be replaced by the data from the contentsDeltaUri."]
    pub fn is_complete_overwrite(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_complete_overwrite", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<VertexAiIndexMetadataElConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiIndexTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VertexAiIndexTimeoutsEl {
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
impl ToListMappable for VertexAiIndexTimeoutsEl {
    type O = BlockAssignable<VertexAiIndexTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexTimeoutsEl {}
impl BuildVertexAiIndexTimeoutsEl {
    pub fn build(self) -> VertexAiIndexTimeoutsEl {
        VertexAiIndexTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VertexAiIndexTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VertexAiIndexTimeoutsElRef {
        VertexAiIndexTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexTimeoutsElRef {
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
struct VertexAiIndexDynamic {
    encryption_spec: Option<DynamicBlock<VertexAiIndexEncryptionSpecEl>>,
    metadata: Option<DynamicBlock<VertexAiIndexMetadataEl>>,
}
