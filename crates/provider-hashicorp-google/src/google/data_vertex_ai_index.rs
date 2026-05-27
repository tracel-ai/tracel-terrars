use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataVertexAiIndexData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    region: PrimField<String>,
}
struct DataVertexAiIndex_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVertexAiIndexData>,
}
#[derive(Clone)]
pub struct DataVertexAiIndex(Rc<DataVertexAiIndex_>);
impl DataVertexAiIndex {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
    pub fn deployed_indexes(&self) -> ListRef<DataVertexAiIndexDeployedIndexesElRef> {
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
    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\nCustomer-managed encryption key spec for an Index. If set, this Index and all sub-resources of this Index will be secured by this key."]
    pub fn encryption_spec(&self) -> ListRef<DataVertexAiIndexEncryptionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_spec", self.extract_ref()),
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
    pub fn index_stats(&self) -> ListRef<DataVertexAiIndexIndexStatsElRef> {
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
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nAdditional information about the Index.\nAlthough this field is not marked as required in the API specification, it is currently required when creating an Index and must be provided.\nAttempts to create an Index without this field will result in an API error."]
    pub fn metadata(&self) -> ListRef<DataVertexAiIndexMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
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
}
impl Referable for DataVertexAiIndex {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataVertexAiIndex {}
impl ToListMappable for DataVertexAiIndex {
    type O = ListRef<DataVertexAiIndexRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataVertexAiIndex_ {
    fn extract_datasource_type(&self) -> String {
        "google_vertex_ai_index".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataVertexAiIndex {
    pub tf_id: String,
    #[doc = "The resource name of the Index."]
    pub name: PrimField<String>,
    #[doc = "The region of the index. eg us-central1"]
    pub region: PrimField<String>,
}
impl BuildDataVertexAiIndex {
    pub fn build(self, stack: &mut Stack) -> DataVertexAiIndex {
        let out = DataVertexAiIndex(Rc::new(DataVertexAiIndex_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVertexAiIndexData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: self.region,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataVertexAiIndexRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVertexAiIndexRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataVertexAiIndexRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
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
    pub fn deployed_indexes(&self) -> ListRef<DataVertexAiIndexDeployedIndexesElRef> {
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
    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\nCustomer-managed encryption key spec for an Index. If set, this Index and all sub-resources of this Index will be secured by this key."]
    pub fn encryption_spec(&self) -> ListRef<DataVertexAiIndexEncryptionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_spec", self.extract_ref()),
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
    pub fn index_stats(&self) -> ListRef<DataVertexAiIndexIndexStatsElRef> {
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
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nAdditional information about the Index.\nAlthough this field is not marked as required in the API specification, it is currently required when creating an Index and must be provided.\nAttempts to create an Index without this field will result in an API error."]
    pub fn metadata(&self) -> ListRef<DataVertexAiIndexMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct DataVertexAiIndexDeployedIndexesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deployed_index_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index_endpoint: Option<PrimField<String>>,
}
impl DataVertexAiIndexDeployedIndexesEl {
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
impl ToListMappable for DataVertexAiIndexDeployedIndexesEl {
    type O = BlockAssignable<DataVertexAiIndexDeployedIndexesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVertexAiIndexDeployedIndexesEl {}
impl BuildDataVertexAiIndexDeployedIndexesEl {
    pub fn build(self) -> DataVertexAiIndexDeployedIndexesEl {
        DataVertexAiIndexDeployedIndexesEl {
            deployed_index_id: core::default::Default::default(),
            index_endpoint: core::default::Default::default(),
        }
    }
}
pub struct DataVertexAiIndexDeployedIndexesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVertexAiIndexDeployedIndexesElRef {
    fn new(shared: StackShared, base: String) -> DataVertexAiIndexDeployedIndexesElRef {
        DataVertexAiIndexDeployedIndexesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVertexAiIndexDeployedIndexesElRef {
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
pub struct DataVertexAiIndexEncryptionSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
}
impl DataVertexAiIndexEncryptionSpecEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataVertexAiIndexEncryptionSpecEl {
    type O = BlockAssignable<DataVertexAiIndexEncryptionSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVertexAiIndexEncryptionSpecEl {}
impl BuildDataVertexAiIndexEncryptionSpecEl {
    pub fn build(self) -> DataVertexAiIndexEncryptionSpecEl {
        DataVertexAiIndexEncryptionSpecEl {
            kms_key_name: core::default::Default::default(),
        }
    }
}
pub struct DataVertexAiIndexEncryptionSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVertexAiIndexEncryptionSpecElRef {
    fn new(shared: StackShared, base: String) -> DataVertexAiIndexEncryptionSpecElRef {
        DataVertexAiIndexEncryptionSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVertexAiIndexEncryptionSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVertexAiIndexIndexStatsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    shards_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vectors_count: Option<PrimField<String>>,
}
impl DataVertexAiIndexIndexStatsEl {
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
impl ToListMappable for DataVertexAiIndexIndexStatsEl {
    type O = BlockAssignable<DataVertexAiIndexIndexStatsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVertexAiIndexIndexStatsEl {}
impl BuildDataVertexAiIndexIndexStatsEl {
    pub fn build(self) -> DataVertexAiIndexIndexStatsEl {
        DataVertexAiIndexIndexStatsEl {
            shards_count: core::default::Default::default(),
            vectors_count: core::default::Default::default(),
        }
    }
}
pub struct DataVertexAiIndexIndexStatsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVertexAiIndexIndexStatsElRef {
    fn new(shared: StackShared, base: String) -> DataVertexAiIndexIndexStatsElRef {
        DataVertexAiIndexIndexStatsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVertexAiIndexIndexStatsElRef {
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
pub struct DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {}
impl DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {}
impl ToListMappable for DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {
    type O =
        BlockAssignable<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {}
impl BuildDataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {
    pub fn build(self) -> DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {
        DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {}
    }
}
pub struct DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef {
        DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    leaf_node_embedding_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    leaf_nodes_to_search_percent: Option<PrimField<f64>>,
}
impl DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
    #[doc = "Set the field `leaf_node_embedding_count`.\n"]
    pub fn set_leaf_node_embedding_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.leaf_node_embedding_count = Some(v.into());
        self
    }
    #[doc = "Set the field `leaf_nodes_to_search_percent`.\n"]
    pub fn set_leaf_nodes_to_search_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.leaf_nodes_to_search_percent = Some(v.into());
        self
    }
}
impl ToListMappable for DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
    type O = BlockAssignable<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {}
impl BuildDataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
    pub fn build(self) -> DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
        DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
            leaf_node_embedding_count: core::default::Default::default(),
            leaf_nodes_to_search_percent: core::default::Default::default(),
        }
    }
}
pub struct DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef {
        DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `leaf_node_embedding_count` after provisioning.\n"]
    pub fn leaf_node_embedding_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.leaf_node_embedding_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `leaf_nodes_to_search_percent` after provisioning.\n"]
    pub fn leaf_nodes_to_search_percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.leaf_nodes_to_search_percent", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataVertexAiIndexMetadataElConfigElAlgorithmConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    brute_force_config:
        Option<ListField<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tree_ah_config:
        Option<ListField<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl>>,
}
impl DataVertexAiIndexMetadataElConfigElAlgorithmConfigEl {
    #[doc = "Set the field `brute_force_config`.\n"]
    pub fn set_brute_force_config(
        mut self,
        v: impl Into<ListField<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl>>,
    ) -> Self {
        self.brute_force_config = Some(v.into());
        self
    }
    #[doc = "Set the field `tree_ah_config`.\n"]
    pub fn set_tree_ah_config(
        mut self,
        v: impl Into<ListField<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl>>,
    ) -> Self {
        self.tree_ah_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataVertexAiIndexMetadataElConfigElAlgorithmConfigEl {
    type O = BlockAssignable<DataVertexAiIndexMetadataElConfigElAlgorithmConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVertexAiIndexMetadataElConfigElAlgorithmConfigEl {}
impl BuildDataVertexAiIndexMetadataElConfigElAlgorithmConfigEl {
    pub fn build(self) -> DataVertexAiIndexMetadataElConfigElAlgorithmConfigEl {
        DataVertexAiIndexMetadataElConfigElAlgorithmConfigEl {
            brute_force_config: core::default::Default::default(),
            tree_ah_config: core::default::Default::default(),
        }
    }
}
pub struct DataVertexAiIndexMetadataElConfigElAlgorithmConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVertexAiIndexMetadataElConfigElAlgorithmConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVertexAiIndexMetadataElConfigElAlgorithmConfigElRef {
        DataVertexAiIndexMetadataElConfigElAlgorithmConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVertexAiIndexMetadataElConfigElAlgorithmConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `brute_force_config` after provisioning.\n"]
    pub fn brute_force_config(
        &self,
    ) -> ListRef<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.brute_force_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tree_ah_config` after provisioning.\n"]
    pub fn tree_ah_config(
        &self,
    ) -> ListRef<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tree_ah_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataVertexAiIndexMetadataElConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm_config: Option<ListField<DataVertexAiIndexMetadataElConfigElAlgorithmConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approximate_neighbors_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    distance_measure_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feature_norm_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shard_size: Option<PrimField<String>>,
}
impl DataVertexAiIndexMetadataElConfigEl {
    #[doc = "Set the field `algorithm_config`.\n"]
    pub fn set_algorithm_config(
        mut self,
        v: impl Into<ListField<DataVertexAiIndexMetadataElConfigElAlgorithmConfigEl>>,
    ) -> Self {
        self.algorithm_config = Some(v.into());
        self
    }
    #[doc = "Set the field `approximate_neighbors_count`.\n"]
    pub fn set_approximate_neighbors_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.approximate_neighbors_count = Some(v.into());
        self
    }
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.dimensions = Some(v.into());
        self
    }
    #[doc = "Set the field `distance_measure_type`.\n"]
    pub fn set_distance_measure_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.distance_measure_type = Some(v.into());
        self
    }
    #[doc = "Set the field `feature_norm_type`.\n"]
    pub fn set_feature_norm_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.feature_norm_type = Some(v.into());
        self
    }
    #[doc = "Set the field `shard_size`.\n"]
    pub fn set_shard_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.shard_size = Some(v.into());
        self
    }
}
impl ToListMappable for DataVertexAiIndexMetadataElConfigEl {
    type O = BlockAssignable<DataVertexAiIndexMetadataElConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVertexAiIndexMetadataElConfigEl {}
impl BuildDataVertexAiIndexMetadataElConfigEl {
    pub fn build(self) -> DataVertexAiIndexMetadataElConfigEl {
        DataVertexAiIndexMetadataElConfigEl {
            algorithm_config: core::default::Default::default(),
            approximate_neighbors_count: core::default::Default::default(),
            dimensions: core::default::Default::default(),
            distance_measure_type: core::default::Default::default(),
            feature_norm_type: core::default::Default::default(),
            shard_size: core::default::Default::default(),
        }
    }
}
pub struct DataVertexAiIndexMetadataElConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVertexAiIndexMetadataElConfigElRef {
    fn new(shared: StackShared, base: String) -> DataVertexAiIndexMetadataElConfigElRef {
        DataVertexAiIndexMetadataElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVertexAiIndexMetadataElConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `algorithm_config` after provisioning.\n"]
    pub fn algorithm_config(
        &self,
    ) -> ListRef<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.algorithm_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `approximate_neighbors_count` after provisioning.\n"]
    pub fn approximate_neighbors_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approximate_neighbors_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }
    #[doc = "Get a reference to the value of field `distance_measure_type` after provisioning.\n"]
    pub fn distance_measure_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.distance_measure_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `feature_norm_type` after provisioning.\n"]
    pub fn feature_norm_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.feature_norm_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shard_size` after provisioning.\n"]
    pub fn shard_size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shard_size", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVertexAiIndexMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<ListField<DataVertexAiIndexMetadataElConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contents_delta_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_complete_overwrite: Option<PrimField<bool>>,
}
impl DataVertexAiIndexMetadataEl {
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl Into<ListField<DataVertexAiIndexMetadataElConfigEl>>,
    ) -> Self {
        self.config = Some(v.into());
        self
    }
    #[doc = "Set the field `contents_delta_uri`.\n"]
    pub fn set_contents_delta_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contents_delta_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `is_complete_overwrite`.\n"]
    pub fn set_is_complete_overwrite(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_complete_overwrite = Some(v.into());
        self
    }
}
impl ToListMappable for DataVertexAiIndexMetadataEl {
    type O = BlockAssignable<DataVertexAiIndexMetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVertexAiIndexMetadataEl {}
impl BuildDataVertexAiIndexMetadataEl {
    pub fn build(self) -> DataVertexAiIndexMetadataEl {
        DataVertexAiIndexMetadataEl {
            config: core::default::Default::default(),
            contents_delta_uri: core::default::Default::default(),
            is_complete_overwrite: core::default::Default::default(),
        }
    }
}
pub struct DataVertexAiIndexMetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVertexAiIndexMetadataElRef {
    fn new(shared: StackShared, base: String) -> DataVertexAiIndexMetadataElRef {
        DataVertexAiIndexMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVertexAiIndexMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<DataVertexAiIndexMetadataElConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
    #[doc = "Get a reference to the value of field `contents_delta_uri` after provisioning.\n"]
    pub fn contents_delta_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contents_delta_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_complete_overwrite` after provisioning.\n"]
    pub fn is_complete_overwrite(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_complete_overwrite", self.base),
        )
    }
}
