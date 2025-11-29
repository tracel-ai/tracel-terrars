use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataKendraIndexData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataKendraIndex_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKendraIndexData>,
}

#[derive(Clone)]
pub struct DataKendraIndex(Rc<DataKendraIndex_>);

impl DataKendraIndex {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_units` after provisioning.\n"]
    pub fn capacity_units(&self) -> ListRef<DataKendraIndexCapacityUnitsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.capacity_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `document_metadata_configuration_updates` after provisioning.\n"]
    pub fn document_metadata_configuration_updates(
        &self,
    ) -> SetRef<DataKendraIndexDocumentMetadataConfigurationUpdatesElRef> {
        SetRef::new(self.shared().clone(), format!("{}.document_metadata_configuration_updates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_statistics` after provisioning.\n"]
    pub fn index_statistics(&self) -> ListRef<DataKendraIndexIndexStatisticsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.index_statistics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(&self) -> ListRef<DataKendraIndexServerSideEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.server_side_encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_context_policy` after provisioning.\n"]
    pub fn user_context_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_context_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_group_resolution_configuration` after provisioning.\n"]
    pub fn user_group_resolution_configuration(&self) -> ListRef<DataKendraIndexUserGroupResolutionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_group_resolution_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_token_configurations` after provisioning.\n"]
    pub fn user_token_configurations(&self) -> ListRef<DataKendraIndexUserTokenConfigurationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_token_configurations", self.extract_ref()))
    }
}

impl Referable for DataKendraIndex {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataKendraIndex { }

impl ToListMappable for DataKendraIndex {
    type O = ListRef<DataKendraIndexRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataKendraIndex_ {
    fn extract_datasource_type(&self) -> String {
        "aws_kendra_index".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataKendraIndex {
    pub tf_id: String,
    #[doc = ""]
    pub id: PrimField<String>,
}

impl BuildDataKendraIndex {
    pub fn build(self, stack: &mut Stack) -> DataKendraIndex {
        let out = DataKendraIndex(Rc::new(DataKendraIndex_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKendraIndexData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: self.id,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataKendraIndexRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraIndexRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataKendraIndexRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_units` after provisioning.\n"]
    pub fn capacity_units(&self) -> ListRef<DataKendraIndexCapacityUnitsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.capacity_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `document_metadata_configuration_updates` after provisioning.\n"]
    pub fn document_metadata_configuration_updates(
        &self,
    ) -> SetRef<DataKendraIndexDocumentMetadataConfigurationUpdatesElRef> {
        SetRef::new(self.shared().clone(), format!("{}.document_metadata_configuration_updates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_statistics` after provisioning.\n"]
    pub fn index_statistics(&self) -> ListRef<DataKendraIndexIndexStatisticsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.index_statistics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(&self) -> ListRef<DataKendraIndexServerSideEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.server_side_encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_context_policy` after provisioning.\n"]
    pub fn user_context_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_context_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_group_resolution_configuration` after provisioning.\n"]
    pub fn user_group_resolution_configuration(&self) -> ListRef<DataKendraIndexUserGroupResolutionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_group_resolution_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_token_configurations` after provisioning.\n"]
    pub fn user_token_configurations(&self) -> ListRef<DataKendraIndexUserTokenConfigurationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_token_configurations", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexCapacityUnitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    query_capacity_units: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_capacity_units: Option<PrimField<f64>>,
}

impl DataKendraIndexCapacityUnitsEl {
    #[doc = "Set the field `query_capacity_units`.\n"]
    pub fn set_query_capacity_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.query_capacity_units = Some(v.into());
        self
    }

    #[doc = "Set the field `storage_capacity_units`.\n"]
    pub fn set_storage_capacity_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_capacity_units = Some(v.into());
        self
    }
}

impl ToListMappable for DataKendraIndexCapacityUnitsEl {
    type O = BlockAssignable<DataKendraIndexCapacityUnitsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexCapacityUnitsEl {}

impl BuildDataKendraIndexCapacityUnitsEl {
    pub fn build(self) -> DataKendraIndexCapacityUnitsEl {
        DataKendraIndexCapacityUnitsEl {
            query_capacity_units: core::default::Default::default(),
            storage_capacity_units: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexCapacityUnitsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraIndexCapacityUnitsElRef {
    fn new(shared: StackShared, base: String) -> DataKendraIndexCapacityUnitsElRef {
        DataKendraIndexCapacityUnitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexCapacityUnitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `query_capacity_units` after provisioning.\n"]
    pub fn query_capacity_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.query_capacity_units", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_capacity_units` after provisioning.\n"]
    pub fn storage_capacity_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity_units", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    freshness: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    importance: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rank_order: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values_importance_map: Option<RecField<PrimField<f64>>>,
}

impl DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.duration = Some(v.into());
        self
    }

    #[doc = "Set the field `freshness`.\n"]
    pub fn set_freshness(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.freshness = Some(v.into());
        self
    }

    #[doc = "Set the field `importance`.\n"]
    pub fn set_importance(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.importance = Some(v.into());
        self
    }

    #[doc = "Set the field `rank_order`.\n"]
    pub fn set_rank_order(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rank_order = Some(v.into());
        self
    }

    #[doc = "Set the field `values_importance_map`.\n"]
    pub fn set_values_importance_map(mut self, v: impl Into<RecField<PrimField<f64>>>) -> Self {
        self.values_importance_map = Some(v.into());
        self
    }
}

impl ToListMappable for DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
    type O = BlockAssignable<DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {}

impl BuildDataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
    pub fn build(self) -> DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
        DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
            duration: core::default::Default::default(),
            freshness: core::default::Default::default(),
            importance: core::default::Default::default(),
            rank_order: core::default::Default::default(),
            values_importance_map: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
    fn new(shared: StackShared, base: String) -> DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
        DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `freshness` after provisioning.\n"]
    pub fn freshness(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.freshness", self.base))
    }

    #[doc = "Get a reference to the value of field `importance` after provisioning.\n"]
    pub fn importance(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.importance", self.base))
    }

    #[doc = "Get a reference to the value of field `rank_order` after provisioning.\n"]
    pub fn rank_order(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rank_order", self.base))
    }

    #[doc = "Get a reference to the value of field `values_importance_map` after provisioning.\n"]
    pub fn values_importance_map(&self) -> RecRef<PrimExpr<f64>> {
        RecRef::new(self.shared().clone(), format!("{}.values_importance_map", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    displayable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facetable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    searchable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sortable: Option<PrimField<bool>>,
}

impl DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
    #[doc = "Set the field `displayable`.\n"]
    pub fn set_displayable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.displayable = Some(v.into());
        self
    }

    #[doc = "Set the field `facetable`.\n"]
    pub fn set_facetable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.facetable = Some(v.into());
        self
    }

    #[doc = "Set the field `searchable`.\n"]
    pub fn set_searchable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.searchable = Some(v.into());
        self
    }

    #[doc = "Set the field `sortable`.\n"]
    pub fn set_sortable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.sortable = Some(v.into());
        self
    }
}

impl ToListMappable for DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
    type O = BlockAssignable<DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {}

impl BuildDataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
    pub fn build(self) -> DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
        DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
            displayable: core::default::Default::default(),
            facetable: core::default::Default::default(),
            searchable: core::default::Default::default(),
            sortable: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
    fn new(shared: StackShared, base: String) -> DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
        DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `displayable` after provisioning.\n"]
    pub fn displayable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.displayable", self.base))
    }

    #[doc = "Get a reference to the value of field `facetable` after provisioning.\n"]
    pub fn facetable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.facetable", self.base))
    }

    #[doc = "Get a reference to the value of field `searchable` after provisioning.\n"]
    pub fn searchable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.searchable", self.base))
    }

    #[doc = "Get a reference to the value of field `sortable` after provisioning.\n"]
    pub fn sortable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.sortable", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexDocumentMetadataConfigurationUpdatesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relevance: Option<ListField<DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<ListField<DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataKendraIndexDocumentMetadataConfigurationUpdatesEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `relevance`.\n"]
    pub fn set_relevance(
        mut self,
        v: impl Into<ListField<DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl>>,
    ) -> Self {
        self.relevance = Some(v.into());
        self
    }

    #[doc = "Set the field `search`.\n"]
    pub fn set_search(
        mut self,
        v: impl Into<ListField<DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl>>,
    ) -> Self {
        self.search = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataKendraIndexDocumentMetadataConfigurationUpdatesEl {
    type O = BlockAssignable<DataKendraIndexDocumentMetadataConfigurationUpdatesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexDocumentMetadataConfigurationUpdatesEl {}

impl BuildDataKendraIndexDocumentMetadataConfigurationUpdatesEl {
    pub fn build(self) -> DataKendraIndexDocumentMetadataConfigurationUpdatesEl {
        DataKendraIndexDocumentMetadataConfigurationUpdatesEl {
            name: core::default::Default::default(),
            relevance: core::default::Default::default(),
            search: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexDocumentMetadataConfigurationUpdatesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraIndexDocumentMetadataConfigurationUpdatesElRef {
    fn new(shared: StackShared, base: String) -> DataKendraIndexDocumentMetadataConfigurationUpdatesElRef {
        DataKendraIndexDocumentMetadataConfigurationUpdatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexDocumentMetadataConfigurationUpdatesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `relevance` after provisioning.\n"]
    pub fn relevance(&self) -> ListRef<DataKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.relevance", self.base))
    }

    #[doc = "Get a reference to the value of field `search` after provisioning.\n"]
    pub fn search(&self) -> ListRef<DataKendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.search", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexIndexStatisticsElFaqStatisticsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    indexed_question_answers_count: Option<PrimField<f64>>,
}

impl DataKendraIndexIndexStatisticsElFaqStatisticsEl {
    #[doc = "Set the field `indexed_question_answers_count`.\n"]
    pub fn set_indexed_question_answers_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.indexed_question_answers_count = Some(v.into());
        self
    }
}

impl ToListMappable for DataKendraIndexIndexStatisticsElFaqStatisticsEl {
    type O = BlockAssignable<DataKendraIndexIndexStatisticsElFaqStatisticsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexIndexStatisticsElFaqStatisticsEl {}

impl BuildDataKendraIndexIndexStatisticsElFaqStatisticsEl {
    pub fn build(self) -> DataKendraIndexIndexStatisticsElFaqStatisticsEl {
        DataKendraIndexIndexStatisticsElFaqStatisticsEl {
            indexed_question_answers_count: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexIndexStatisticsElFaqStatisticsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraIndexIndexStatisticsElFaqStatisticsElRef {
    fn new(shared: StackShared, base: String) -> DataKendraIndexIndexStatisticsElFaqStatisticsElRef {
        DataKendraIndexIndexStatisticsElFaqStatisticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexIndexStatisticsElFaqStatisticsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `indexed_question_answers_count` after provisioning.\n"]
    pub fn indexed_question_answers_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.indexed_question_answers_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexIndexStatisticsElTextDocumentStatisticsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    indexed_text_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indexed_text_documents_count: Option<PrimField<f64>>,
}

impl DataKendraIndexIndexStatisticsElTextDocumentStatisticsEl {
    #[doc = "Set the field `indexed_text_bytes`.\n"]
    pub fn set_indexed_text_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.indexed_text_bytes = Some(v.into());
        self
    }

    #[doc = "Set the field `indexed_text_documents_count`.\n"]
    pub fn set_indexed_text_documents_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.indexed_text_documents_count = Some(v.into());
        self
    }
}

impl ToListMappable for DataKendraIndexIndexStatisticsElTextDocumentStatisticsEl {
    type O = BlockAssignable<DataKendraIndexIndexStatisticsElTextDocumentStatisticsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexIndexStatisticsElTextDocumentStatisticsEl {}

impl BuildDataKendraIndexIndexStatisticsElTextDocumentStatisticsEl {
    pub fn build(self) -> DataKendraIndexIndexStatisticsElTextDocumentStatisticsEl {
        DataKendraIndexIndexStatisticsElTextDocumentStatisticsEl {
            indexed_text_bytes: core::default::Default::default(),
            indexed_text_documents_count: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
    fn new(shared: StackShared, base: String) -> DataKendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
        DataKendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `indexed_text_bytes` after provisioning.\n"]
    pub fn indexed_text_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.indexed_text_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `indexed_text_documents_count` after provisioning.\n"]
    pub fn indexed_text_documents_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.indexed_text_documents_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexIndexStatisticsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    faq_statistics: Option<ListField<DataKendraIndexIndexStatisticsElFaqStatisticsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_document_statistics: Option<ListField<DataKendraIndexIndexStatisticsElTextDocumentStatisticsEl>>,
}

impl DataKendraIndexIndexStatisticsEl {
    #[doc = "Set the field `faq_statistics`.\n"]
    pub fn set_faq_statistics(
        mut self,
        v: impl Into<ListField<DataKendraIndexIndexStatisticsElFaqStatisticsEl>>,
    ) -> Self {
        self.faq_statistics = Some(v.into());
        self
    }

    #[doc = "Set the field `text_document_statistics`.\n"]
    pub fn set_text_document_statistics(
        mut self,
        v: impl Into<ListField<DataKendraIndexIndexStatisticsElTextDocumentStatisticsEl>>,
    ) -> Self {
        self.text_document_statistics = Some(v.into());
        self
    }
}

impl ToListMappable for DataKendraIndexIndexStatisticsEl {
    type O = BlockAssignable<DataKendraIndexIndexStatisticsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexIndexStatisticsEl {}

impl BuildDataKendraIndexIndexStatisticsEl {
    pub fn build(self) -> DataKendraIndexIndexStatisticsEl {
        DataKendraIndexIndexStatisticsEl {
            faq_statistics: core::default::Default::default(),
            text_document_statistics: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexIndexStatisticsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraIndexIndexStatisticsElRef {
    fn new(shared: StackShared, base: String) -> DataKendraIndexIndexStatisticsElRef {
        DataKendraIndexIndexStatisticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexIndexStatisticsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `faq_statistics` after provisioning.\n"]
    pub fn faq_statistics(&self) -> ListRef<DataKendraIndexIndexStatisticsElFaqStatisticsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.faq_statistics", self.base))
    }

    #[doc = "Get a reference to the value of field `text_document_statistics` after provisioning.\n"]
    pub fn text_document_statistics(&self) -> ListRef<DataKendraIndexIndexStatisticsElTextDocumentStatisticsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.text_document_statistics", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexServerSideEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
}

impl DataKendraIndexServerSideEncryptionConfigurationEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataKendraIndexServerSideEncryptionConfigurationEl {
    type O = BlockAssignable<DataKendraIndexServerSideEncryptionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexServerSideEncryptionConfigurationEl {}

impl BuildDataKendraIndexServerSideEncryptionConfigurationEl {
    pub fn build(self) -> DataKendraIndexServerSideEncryptionConfigurationEl {
        DataKendraIndexServerSideEncryptionConfigurationEl { kms_key_id: core::default::Default::default() }
    }
}

pub struct DataKendraIndexServerSideEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraIndexServerSideEncryptionConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataKendraIndexServerSideEncryptionConfigurationElRef {
        DataKendraIndexServerSideEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexServerSideEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexUserGroupResolutionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    user_group_resolution_mode: Option<PrimField<String>>,
}

impl DataKendraIndexUserGroupResolutionConfigurationEl {
    #[doc = "Set the field `user_group_resolution_mode`.\n"]
    pub fn set_user_group_resolution_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_group_resolution_mode = Some(v.into());
        self
    }
}

impl ToListMappable for DataKendraIndexUserGroupResolutionConfigurationEl {
    type O = BlockAssignable<DataKendraIndexUserGroupResolutionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexUserGroupResolutionConfigurationEl {}

impl BuildDataKendraIndexUserGroupResolutionConfigurationEl {
    pub fn build(self) -> DataKendraIndexUserGroupResolutionConfigurationEl {
        DataKendraIndexUserGroupResolutionConfigurationEl {
            user_group_resolution_mode: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexUserGroupResolutionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraIndexUserGroupResolutionConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataKendraIndexUserGroupResolutionConfigurationElRef {
        DataKendraIndexUserGroupResolutionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexUserGroupResolutionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `user_group_resolution_mode` after provisioning.\n"]
    pub fn user_group_resolution_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_group_resolution_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_attribute_field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_name_attribute_field: Option<PrimField<String>>,
}

impl DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
    #[doc = "Set the field `group_attribute_field`.\n"]
    pub fn set_group_attribute_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_attribute_field = Some(v.into());
        self
    }

    #[doc = "Set the field `user_name_attribute_field`.\n"]
    pub fn set_user_name_attribute_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_name_attribute_field = Some(v.into());
        self
    }
}

impl ToListMappable for DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
    type O = BlockAssignable<DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {}

impl BuildDataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
    pub fn build(self) -> DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
        DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
            group_attribute_field: core::default::Default::default(),
            user_name_attribute_field: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
        DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_attribute_field` after provisioning.\n"]
    pub fn group_attribute_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_attribute_field", self.base))
    }

    #[doc = "Get a reference to the value of field `user_name_attribute_field` after provisioning.\n"]
    pub fn user_name_attribute_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_name_attribute_field", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    claim_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_attribute_field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets_manager_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_name_attribute_field: Option<PrimField<String>>,
}

impl DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    #[doc = "Set the field `claim_regex`.\n"]
    pub fn set_claim_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.claim_regex = Some(v.into());
        self
    }

    #[doc = "Set the field `group_attribute_field`.\n"]
    pub fn set_group_attribute_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_attribute_field = Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }

    #[doc = "Set the field `key_location`.\n"]
    pub fn set_key_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_location = Some(v.into());
        self
    }

    #[doc = "Set the field `secrets_manager_arn`.\n"]
    pub fn set_secrets_manager_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secrets_manager_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }

    #[doc = "Set the field `user_name_attribute_field`.\n"]
    pub fn set_user_name_attribute_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_name_attribute_field = Some(v.into());
        self
    }
}

impl ToListMappable for DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    type O = BlockAssignable<DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {}

impl BuildDataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    pub fn build(self) -> DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
        DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
            claim_regex: core::default::Default::default(),
            group_attribute_field: core::default::Default::default(),
            issuer: core::default::Default::default(),
            key_location: core::default::Default::default(),
            secrets_manager_arn: core::default::Default::default(),
            url: core::default::Default::default(),
            user_name_attribute_field: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
        DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `claim_regex` after provisioning.\n"]
    pub fn claim_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.claim_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `group_attribute_field` after provisioning.\n"]
    pub fn group_attribute_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_attribute_field", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `key_location` after provisioning.\n"]
    pub fn key_location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_location", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_arn` after provisioning.\n"]
    pub fn secrets_manager_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secrets_manager_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `user_name_attribute_field` after provisioning.\n"]
    pub fn user_name_attribute_field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_name_attribute_field", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraIndexUserTokenConfigurationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    json_token_type_configuration: Option<
        ListField<DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    jwt_token_type_configuration: Option<ListField<DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl>>,
}

impl DataKendraIndexUserTokenConfigurationsEl {
    #[doc = "Set the field `json_token_type_configuration`.\n"]
    pub fn set_json_token_type_configuration(
        mut self,
        v: impl Into<ListField<DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl>>,
    ) -> Self {
        self.json_token_type_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `jwt_token_type_configuration`.\n"]
    pub fn set_jwt_token_type_configuration(
        mut self,
        v: impl Into<ListField<DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl>>,
    ) -> Self {
        self.jwt_token_type_configuration = Some(v.into());
        self
    }
}

impl ToListMappable for DataKendraIndexUserTokenConfigurationsEl {
    type O = BlockAssignable<DataKendraIndexUserTokenConfigurationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraIndexUserTokenConfigurationsEl {}

impl BuildDataKendraIndexUserTokenConfigurationsEl {
    pub fn build(self) -> DataKendraIndexUserTokenConfigurationsEl {
        DataKendraIndexUserTokenConfigurationsEl {
            json_token_type_configuration: core::default::Default::default(),
            jwt_token_type_configuration: core::default::Default::default(),
        }
    }
}

pub struct DataKendraIndexUserTokenConfigurationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraIndexUserTokenConfigurationsElRef {
    fn new(shared: StackShared, base: String) -> DataKendraIndexUserTokenConfigurationsElRef {
        DataKendraIndexUserTokenConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraIndexUserTokenConfigurationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json_token_type_configuration` after provisioning.\n"]
    pub fn json_token_type_configuration(
        &self,
    ) -> ListRef<DataKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.json_token_type_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_token_type_configuration` after provisioning.\n"]
    pub fn jwt_token_type_configuration(
        &self,
    ) -> ListRef<DataKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.jwt_token_type_configuration", self.base))
    }
}
