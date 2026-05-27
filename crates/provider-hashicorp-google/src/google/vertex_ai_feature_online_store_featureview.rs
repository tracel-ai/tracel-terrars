use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VertexAiFeatureOnlineStoreFeatureviewData {
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
    feature_online_store: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    big_query_source: Option<Vec<VertexAiFeatureOnlineStoreFeatureviewBigQuerySourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feature_registry_source:
        Option<Vec<VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_config: Option<Vec<VertexAiFeatureOnlineStoreFeatureviewSyncConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VertexAiFeatureOnlineStoreFeatureviewTimeoutsEl>,
    dynamic: VertexAiFeatureOnlineStoreFeatureviewDynamic,
}
struct VertexAiFeatureOnlineStoreFeatureview_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VertexAiFeatureOnlineStoreFeatureviewData>,
}
#[derive(Clone)]
pub struct VertexAiFeatureOnlineStoreFeatureview(Rc<VertexAiFeatureOnlineStoreFeatureview_>);
impl VertexAiFeatureOnlineStoreFeatureview {
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
    #[doc = "Set the field `labels`.\nA set of key/value label pairs to assign to this FeatureView.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nName of the FeatureView. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region for the resource. It should be the same as the featureonlinestore region."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `big_query_source`.\n"]
    pub fn set_big_query_source(
        self,
        v: impl Into<BlockAssignable<VertexAiFeatureOnlineStoreFeatureviewBigQuerySourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().big_query_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.big_query_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `feature_registry_source`.\n"]
    pub fn set_feature_registry_source(
        self,
        v: impl Into<BlockAssignable<VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().feature_registry_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.feature_registry_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sync_config`.\n"]
    pub fn set_sync_config(
        self,
        v: impl Into<BlockAssignable<VertexAiFeatureOnlineStoreFeatureviewSyncConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sync_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sync_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<VertexAiFeatureOnlineStoreFeatureviewTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the featureOnlinestore was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `feature_online_store` after provisioning.\nThe name of the FeatureOnlineStore to use for the featureview."]
    pub fn feature_online_store(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.feature_online_store", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this FeatureView.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the FeatureView. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region for the resource. It should be the same as the featureonlinestore region."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the featureOnlinestore was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `big_query_source` after provisioning.\n"]
    pub fn big_query_source(
        &self,
    ) -> ListRef<VertexAiFeatureOnlineStoreFeatureviewBigQuerySourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.big_query_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `feature_registry_source` after provisioning.\n"]
    pub fn feature_registry_source(
        &self,
    ) -> ListRef<VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.feature_registry_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sync_config` after provisioning.\n"]
    pub fn sync_config(&self) -> ListRef<VertexAiFeatureOnlineStoreFeatureviewSyncConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sync_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiFeatureOnlineStoreFeatureviewTimeoutsElRef {
        VertexAiFeatureOnlineStoreFeatureviewTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VertexAiFeatureOnlineStoreFeatureview {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VertexAiFeatureOnlineStoreFeatureview {}
impl ToListMappable for VertexAiFeatureOnlineStoreFeatureview {
    type O = ListRef<VertexAiFeatureOnlineStoreFeatureviewRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VertexAiFeatureOnlineStoreFeatureview_ {
    fn extract_resource_type(&self) -> String {
        "google_vertex_ai_feature_online_store_featureview".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVertexAiFeatureOnlineStoreFeatureview {
    pub tf_id: String,
    #[doc = "The name of the FeatureOnlineStore to use for the featureview."]
    pub feature_online_store: PrimField<String>,
}
impl BuildVertexAiFeatureOnlineStoreFeatureview {
    pub fn build(self, stack: &mut Stack) -> VertexAiFeatureOnlineStoreFeatureview {
        let out = VertexAiFeatureOnlineStoreFeatureview(Rc::new(
            VertexAiFeatureOnlineStoreFeatureview_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(VertexAiFeatureOnlineStoreFeatureviewData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    feature_online_store: self.feature_online_store,
                    id: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    name: core::default::Default::default(),
                    project: core::default::Default::default(),
                    region: core::default::Default::default(),
                    big_query_source: core::default::Default::default(),
                    feature_registry_source: core::default::Default::default(),
                    sync_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VertexAiFeatureOnlineStoreFeatureviewRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeatureOnlineStoreFeatureviewRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VertexAiFeatureOnlineStoreFeatureviewRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the featureOnlinestore was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `feature_online_store` after provisioning.\nThe name of the FeatureOnlineStore to use for the featureview."]
    pub fn feature_online_store(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.feature_online_store", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this FeatureView.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the FeatureView. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region for the resource. It should be the same as the featureonlinestore region."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the featureOnlinestore was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `big_query_source` after provisioning.\n"]
    pub fn big_query_source(
        &self,
    ) -> ListRef<VertexAiFeatureOnlineStoreFeatureviewBigQuerySourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.big_query_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `feature_registry_source` after provisioning.\n"]
    pub fn feature_registry_source(
        &self,
    ) -> ListRef<VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.feature_registry_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sync_config` after provisioning.\n"]
    pub fn sync_config(&self) -> ListRef<VertexAiFeatureOnlineStoreFeatureviewSyncConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sync_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiFeatureOnlineStoreFeatureviewTimeoutsElRef {
        VertexAiFeatureOnlineStoreFeatureviewTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStoreFeatureviewBigQuerySourceEl {
    entity_id_columns: ListField<PrimField<String>>,
    uri: PrimField<String>,
}
impl VertexAiFeatureOnlineStoreFeatureviewBigQuerySourceEl {}
impl ToListMappable for VertexAiFeatureOnlineStoreFeatureviewBigQuerySourceEl {
    type O = BlockAssignable<VertexAiFeatureOnlineStoreFeatureviewBigQuerySourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeatureOnlineStoreFeatureviewBigQuerySourceEl {
    #[doc = "Columns to construct entityId / row keys. Start by supporting 1 only."]
    pub entity_id_columns: ListField<PrimField<String>>,
    #[doc = "The BigQuery view URI that will be materialized on each sync trigger based on FeatureView.SyncConfig."]
    pub uri: PrimField<String>,
}
impl BuildVertexAiFeatureOnlineStoreFeatureviewBigQuerySourceEl {
    pub fn build(self) -> VertexAiFeatureOnlineStoreFeatureviewBigQuerySourceEl {
        VertexAiFeatureOnlineStoreFeatureviewBigQuerySourceEl {
            entity_id_columns: self.entity_id_columns,
            uri: self.uri,
        }
    }
}
pub struct VertexAiFeatureOnlineStoreFeatureviewBigQuerySourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeatureOnlineStoreFeatureviewBigQuerySourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiFeatureOnlineStoreFeatureviewBigQuerySourceElRef {
        VertexAiFeatureOnlineStoreFeatureviewBigQuerySourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeatureOnlineStoreFeatureviewBigQuerySourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `entity_id_columns` after provisioning.\nColumns to construct entityId / row keys. Start by supporting 1 only."]
    pub fn entity_id_columns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entity_id_columns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe BigQuery view URI that will be materialized on each sync trigger based on FeatureView.SyncConfig."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElFeatureGroupsEl {
    feature_group_id: PrimField<String>,
    feature_ids: ListField<PrimField<String>>,
}
impl VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElFeatureGroupsEl {}
impl ToListMappable
    for VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElFeatureGroupsEl
{
    type O = BlockAssignable<
        VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElFeatureGroupsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElFeatureGroupsEl {
    #[doc = "Identifier of the feature group."]
    pub feature_group_id: PrimField<String>,
    #[doc = "Identifiers of features under the feature group."]
    pub feature_ids: ListField<PrimField<String>>,
}
impl BuildVertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElFeatureGroupsEl {
    pub fn build(
        self,
    ) -> VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElFeatureGroupsEl {
        VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElFeatureGroupsEl {
            feature_group_id: self.feature_group_id,
            feature_ids: self.feature_ids,
        }
    }
}
pub struct VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElFeatureGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElFeatureGroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElFeatureGroupsElRef {
        VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElFeatureGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElFeatureGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `feature_group_id` after provisioning.\nIdentifier of the feature group."]
    pub fn feature_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.feature_group_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `feature_ids` after provisioning.\nIdentifiers of features under the feature group."]
    pub fn feature_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.feature_ids", self.base))
    }
}
#[derive(Serialize, Default)]
struct VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElDynamic {
    feature_groups: Option<
        DynamicBlock<VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElFeatureGroupsEl>,
    >,
}
#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    project_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feature_groups:
        Option<Vec<VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElFeatureGroupsEl>>,
    dynamic: VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElDynamic,
}
impl VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceEl {
    #[doc = "Set the field `project_number`.\nThe project number of the parent project of the feature Groups."]
    pub fn set_project_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_number = Some(v.into());
        self
    }
    #[doc = "Set the field `feature_groups`.\n"]
    pub fn set_feature_groups(
        mut self,
        v: impl Into<
            BlockAssignable<
                VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElFeatureGroupsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.feature_groups = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.feature_groups = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceEl {
    type O = BlockAssignable<VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceEl {}
impl BuildVertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceEl {
    pub fn build(self) -> VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceEl {
        VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceEl {
            project_number: core::default::Default::default(),
            feature_groups: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElRef {
        VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `project_number` after provisioning.\nThe project number of the parent project of the feature Groups."]
    pub fn project_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_number", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `feature_groups` after provisioning.\n"]
    pub fn feature_groups(
        &self,
    ) -> ListRef<VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceElFeatureGroupsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.feature_groups", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStoreFeatureviewSyncConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    continuous: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cron: Option<PrimField<String>>,
}
impl VertexAiFeatureOnlineStoreFeatureviewSyncConfigEl {
    #[doc = "Set the field `continuous`.\nIf true, syncs the FeatureView in a continuous manner to Online Store."]
    pub fn set_continuous(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.continuous = Some(v.into());
        self
    }
    #[doc = "Set the field `cron`.\nCron schedule (https://en.wikipedia.org/wiki/Cron) to launch scheduled runs.\nTo explicitly set a timezone to the cron tab, apply a prefix in the cron tab: \"CRON_TZ=${IANA_TIME_ZONE}\" or \"TZ=${IANA_TIME_ZONE}\"."]
    pub fn set_cron(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cron = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiFeatureOnlineStoreFeatureviewSyncConfigEl {
    type O = BlockAssignable<VertexAiFeatureOnlineStoreFeatureviewSyncConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeatureOnlineStoreFeatureviewSyncConfigEl {}
impl BuildVertexAiFeatureOnlineStoreFeatureviewSyncConfigEl {
    pub fn build(self) -> VertexAiFeatureOnlineStoreFeatureviewSyncConfigEl {
        VertexAiFeatureOnlineStoreFeatureviewSyncConfigEl {
            continuous: core::default::Default::default(),
            cron: core::default::Default::default(),
        }
    }
}
pub struct VertexAiFeatureOnlineStoreFeatureviewSyncConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeatureOnlineStoreFeatureviewSyncConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiFeatureOnlineStoreFeatureviewSyncConfigElRef {
        VertexAiFeatureOnlineStoreFeatureviewSyncConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeatureOnlineStoreFeatureviewSyncConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `continuous` after provisioning.\nIf true, syncs the FeatureView in a continuous manner to Online Store."]
    pub fn continuous(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.continuous", self.base))
    }
    #[doc = "Get a reference to the value of field `cron` after provisioning.\nCron schedule (https://en.wikipedia.org/wiki/Cron) to launch scheduled runs.\nTo explicitly set a timezone to the cron tab, apply a prefix in the cron tab: \"CRON_TZ=${IANA_TIME_ZONE}\" or \"TZ=${IANA_TIME_ZONE}\"."]
    pub fn cron(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cron", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStoreFeatureviewTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VertexAiFeatureOnlineStoreFeatureviewTimeoutsEl {
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
impl ToListMappable for VertexAiFeatureOnlineStoreFeatureviewTimeoutsEl {
    type O = BlockAssignable<VertexAiFeatureOnlineStoreFeatureviewTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiFeatureOnlineStoreFeatureviewTimeoutsEl {}
impl BuildVertexAiFeatureOnlineStoreFeatureviewTimeoutsEl {
    pub fn build(self) -> VertexAiFeatureOnlineStoreFeatureviewTimeoutsEl {
        VertexAiFeatureOnlineStoreFeatureviewTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VertexAiFeatureOnlineStoreFeatureviewTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiFeatureOnlineStoreFeatureviewTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiFeatureOnlineStoreFeatureviewTimeoutsElRef {
        VertexAiFeatureOnlineStoreFeatureviewTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiFeatureOnlineStoreFeatureviewTimeoutsElRef {
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
struct VertexAiFeatureOnlineStoreFeatureviewDynamic {
    big_query_source: Option<DynamicBlock<VertexAiFeatureOnlineStoreFeatureviewBigQuerySourceEl>>,
    feature_registry_source:
        Option<DynamicBlock<VertexAiFeatureOnlineStoreFeatureviewFeatureRegistrySourceEl>>,
    sync_config: Option<DynamicBlock<VertexAiFeatureOnlineStoreFeatureviewSyncConfigEl>>,
}
