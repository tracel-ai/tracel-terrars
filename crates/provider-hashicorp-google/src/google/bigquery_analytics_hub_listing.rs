use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BigqueryAnalyticsHubListingData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_only_metadata_sharing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<ListField<PrimField<String>>>,
    data_exchange_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_commercial: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discovery_type: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documentation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    listing_id: PrimField<String>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_linked_dataset_query_user_email: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_contact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_access: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_dataset: Option<Vec<BigqueryAnalyticsHubListingBigqueryDatasetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_provider: Option<Vec<BigqueryAnalyticsHubListingDataProviderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publisher: Option<Vec<BigqueryAnalyticsHubListingPublisherEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub_topic: Option<Vec<BigqueryAnalyticsHubListingPubsubTopicEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_export_config: Option<Vec<BigqueryAnalyticsHubListingRestrictedExportConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigqueryAnalyticsHubListingTimeoutsEl>,
    dynamic: BigqueryAnalyticsHubListingDynamic,
}
struct BigqueryAnalyticsHubListing_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryAnalyticsHubListingData>,
}
#[derive(Clone)]
pub struct BigqueryAnalyticsHubListing(Rc<BigqueryAnalyticsHubListing_>);
impl BigqueryAnalyticsHubListing {
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
    #[doc = "Set the field `allow_only_metadata_sharing`.\nIf true, the listing is only available to get the resource metadata. Listing is non subscribable."]
    pub fn set_allow_only_metadata_sharing(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_only_metadata_sharing = Some(v.into());
        self
    }
    #[doc = "Set the field `categories`.\nCategories of the listing. Up to two categories are allowed."]
    pub fn set_categories(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().categories = Some(v.into());
        self
    }
    #[doc = "Set the field `delete_commercial`.\nIf the listing is commercial then this field must be set to true, otherwise a failure is thrown. This acts as a safety guard to avoid deleting commercial listings accidentally."]
    pub fn set_delete_commercial(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_commercial = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nShort description of the listing. The description must not contain Unicode non-characters and C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF)."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `discovery_type`.\nSpecifies the type of discovery on the discovery page. Cannot be set for a restricted listing. Note that this does not control the visibility of the exchange/listing which is defined by IAM permission. Possible values: [\"DISCOVERY_TYPE_PRIVATE\", \"DISCOVERY_TYPE_PUBLIC\"]"]
    pub fn set_discovery_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().discovery_type = Some(v.into());
        self
    }
    #[doc = "Set the field `documentation`.\nDocumentation describing the listing."]
    pub fn set_documentation(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().documentation = Some(v.into());
        self
    }
    #[doc = "Set the field `icon`.\nBase64 encoded image representing the listing."]
    pub fn set_icon(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().icon = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `log_linked_dataset_query_user_email`.\nIf true, subscriber email logging is enabled and all queries on the linked dataset will log the email address of the querying user. Once enabled, this setting cannot be turned off."]
    pub fn set_log_linked_dataset_query_user_email(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().log_linked_dataset_query_user_email = Some(v.into());
        self
    }
    #[doc = "Set the field `primary_contact`.\nEmail or URL of the primary point of contact of the listing."]
    pub fn set_primary_contact(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().primary_contact = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `request_access`.\nEmail or URL of the request access of the listing. Subscribers can use this reference to request access."]
    pub fn set_request_access(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().request_access = Some(v.into());
        self
    }
    #[doc = "Set the field `bigquery_dataset`.\n"]
    pub fn set_bigquery_dataset(
        self,
        v: impl Into<BlockAssignable<BigqueryAnalyticsHubListingBigqueryDatasetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bigquery_dataset = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bigquery_dataset = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `data_provider`.\n"]
    pub fn set_data_provider(
        self,
        v: impl Into<BlockAssignable<BigqueryAnalyticsHubListingDataProviderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_provider = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_provider = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `publisher`.\n"]
    pub fn set_publisher(
        self,
        v: impl Into<BlockAssignable<BigqueryAnalyticsHubListingPublisherEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().publisher = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.publisher = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pubsub_topic`.\n"]
    pub fn set_pubsub_topic(
        self,
        v: impl Into<BlockAssignable<BigqueryAnalyticsHubListingPubsubTopicEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pubsub_topic = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pubsub_topic = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `restricted_export_config`.\n"]
    pub fn set_restricted_export_config(
        self,
        v: impl Into<BlockAssignable<BigqueryAnalyticsHubListingRestrictedExportConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restricted_export_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restricted_export_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigqueryAnalyticsHubListingTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `allow_only_metadata_sharing` after provisioning.\nIf true, the listing is only available to get the resource metadata. Listing is non subscribable."]
    pub fn allow_only_metadata_sharing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_only_metadata_sharing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `categories` after provisioning.\nCategories of the listing. Up to two categories are allowed."]
    pub fn categories(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.categories", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `commercial_info` after provisioning.\nCommercial info contains the information about the commercial data products associated with the listing."]
    pub fn commercial_info(&self) -> ListRef<BigqueryAnalyticsHubListingCommercialInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.commercial_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_exchange_id` after provisioning.\nThe ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn data_exchange_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_exchange_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_commercial` after provisioning.\nIf the listing is commercial then this field must be set to true, otherwise a failure is thrown. This acts as a safety guard to avoid deleting commercial listings accidentally."]
    pub fn delete_commercial(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_commercial", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nShort description of the listing. The description must not contain Unicode non-characters and C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF)."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_type` after provisioning.\nSpecifies the type of discovery on the discovery page. Cannot be set for a restricted listing. Note that this does not control the visibility of the exchange/listing which is defined by IAM permission. Possible values: [\"DISCOVERY_TYPE_PRIVATE\", \"DISCOVERY_TYPE_PUBLIC\"]"]
    pub fn discovery_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.discovery_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of the listing. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and can't start or end with spaces."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `documentation` after provisioning.\nDocumentation describing the listing."]
    pub fn documentation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.documentation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `icon` after provisioning.\nBase64 encoded image representing the listing."]
    pub fn icon(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.icon", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `listing_id` after provisioning.\nThe ID of the listing. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn listing_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listing_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location this data exchange listing."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_linked_dataset_query_user_email` after provisioning.\nIf true, subscriber email logging is enabled and all queries on the linked dataset will log the email address of the querying user. Once enabled, this setting cannot be turned off."]
    pub fn log_linked_dataset_query_user_email(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_linked_dataset_query_user_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the listing. e.g. \"projects/myproject/locations/US/dataExchanges/123/listings/456\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the primary point of contact of the listing."]
    pub fn primary_contact(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_contact", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `request_access` after provisioning.\nEmail or URL of the request access of the listing. Subscribers can use this reference to request access."]
    pub fn request_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the listing."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bigquery_dataset` after provisioning.\n"]
    pub fn bigquery_dataset(&self) -> ListRef<BigqueryAnalyticsHubListingBigqueryDatasetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_dataset", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_provider` after provisioning.\n"]
    pub fn data_provider(&self) -> ListRef<BigqueryAnalyticsHubListingDataProviderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_provider", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `publisher` after provisioning.\n"]
    pub fn publisher(&self) -> ListRef<BigqueryAnalyticsHubListingPublisherElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.publisher", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pubsub_topic` after provisioning.\n"]
    pub fn pubsub_topic(&self) -> ListRef<BigqueryAnalyticsHubListingPubsubTopicElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pubsub_topic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restricted_export_config` after provisioning.\n"]
    pub fn restricted_export_config(
        &self,
    ) -> ListRef<BigqueryAnalyticsHubListingRestrictedExportConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restricted_export_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryAnalyticsHubListingTimeoutsElRef {
        BigqueryAnalyticsHubListingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BigqueryAnalyticsHubListing {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BigqueryAnalyticsHubListing {}
impl ToListMappable for BigqueryAnalyticsHubListing {
    type O = ListRef<BigqueryAnalyticsHubListingRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BigqueryAnalyticsHubListing_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_analytics_hub_listing".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBigqueryAnalyticsHubListing {
    pub tf_id: String,
    #[doc = "The ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub data_exchange_id: PrimField<String>,
    #[doc = "Human-readable display name of the listing. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and can't start or end with spaces."]
    pub display_name: PrimField<String>,
    #[doc = "The ID of the listing. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub listing_id: PrimField<String>,
    #[doc = "The name of the location this data exchange listing."]
    pub location: PrimField<String>,
}
impl BuildBigqueryAnalyticsHubListing {
    pub fn build(self, stack: &mut Stack) -> BigqueryAnalyticsHubListing {
        let out = BigqueryAnalyticsHubListing(Rc::new(BigqueryAnalyticsHubListing_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigqueryAnalyticsHubListingData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allow_only_metadata_sharing: core::default::Default::default(),
                categories: core::default::Default::default(),
                data_exchange_id: self.data_exchange_id,
                delete_commercial: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                discovery_type: core::default::Default::default(),
                display_name: self.display_name,
                documentation: core::default::Default::default(),
                icon: core::default::Default::default(),
                id: core::default::Default::default(),
                listing_id: self.listing_id,
                location: self.location,
                log_linked_dataset_query_user_email: core::default::Default::default(),
                primary_contact: core::default::Default::default(),
                project: core::default::Default::default(),
                request_access: core::default::Default::default(),
                bigquery_dataset: core::default::Default::default(),
                data_provider: core::default::Default::default(),
                publisher: core::default::Default::default(),
                pubsub_topic: core::default::Default::default(),
                restricted_export_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BigqueryAnalyticsHubListingRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BigqueryAnalyticsHubListingRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_only_metadata_sharing` after provisioning.\nIf true, the listing is only available to get the resource metadata. Listing is non subscribable."]
    pub fn allow_only_metadata_sharing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_only_metadata_sharing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `categories` after provisioning.\nCategories of the listing. Up to two categories are allowed."]
    pub fn categories(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.categories", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `commercial_info` after provisioning.\nCommercial info contains the information about the commercial data products associated with the listing."]
    pub fn commercial_info(&self) -> ListRef<BigqueryAnalyticsHubListingCommercialInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.commercial_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_exchange_id` after provisioning.\nThe ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn data_exchange_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_exchange_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_commercial` after provisioning.\nIf the listing is commercial then this field must be set to true, otherwise a failure is thrown. This acts as a safety guard to avoid deleting commercial listings accidentally."]
    pub fn delete_commercial(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_commercial", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nShort description of the listing. The description must not contain Unicode non-characters and C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF)."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_type` after provisioning.\nSpecifies the type of discovery on the discovery page. Cannot be set for a restricted listing. Note that this does not control the visibility of the exchange/listing which is defined by IAM permission. Possible values: [\"DISCOVERY_TYPE_PRIVATE\", \"DISCOVERY_TYPE_PUBLIC\"]"]
    pub fn discovery_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.discovery_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of the listing. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and can't start or end with spaces."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `documentation` after provisioning.\nDocumentation describing the listing."]
    pub fn documentation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.documentation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `icon` after provisioning.\nBase64 encoded image representing the listing."]
    pub fn icon(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.icon", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `listing_id` after provisioning.\nThe ID of the listing. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn listing_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listing_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location this data exchange listing."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_linked_dataset_query_user_email` after provisioning.\nIf true, subscriber email logging is enabled and all queries on the linked dataset will log the email address of the querying user. Once enabled, this setting cannot be turned off."]
    pub fn log_linked_dataset_query_user_email(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_linked_dataset_query_user_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the listing. e.g. \"projects/myproject/locations/US/dataExchanges/123/listings/456\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the primary point of contact of the listing."]
    pub fn primary_contact(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_contact", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `request_access` after provisioning.\nEmail or URL of the request access of the listing. Subscribers can use this reference to request access."]
    pub fn request_access(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_access", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the listing."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bigquery_dataset` after provisioning.\n"]
    pub fn bigquery_dataset(&self) -> ListRef<BigqueryAnalyticsHubListingBigqueryDatasetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_dataset", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_provider` after provisioning.\n"]
    pub fn data_provider(&self) -> ListRef<BigqueryAnalyticsHubListingDataProviderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_provider", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `publisher` after provisioning.\n"]
    pub fn publisher(&self) -> ListRef<BigqueryAnalyticsHubListingPublisherElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.publisher", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `pubsub_topic` after provisioning.\n"]
    pub fn pubsub_topic(&self) -> ListRef<BigqueryAnalyticsHubListingPubsubTopicElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pubsub_topic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `restricted_export_config` after provisioning.\n"]
    pub fn restricted_export_config(
        &self,
    ) -> ListRef<BigqueryAnalyticsHubListingRestrictedExportConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.restricted_export_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryAnalyticsHubListingTimeoutsElRef {
        BigqueryAnalyticsHubListingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingCommercialInfoElCloudMarketplaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    commercial_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl BigqueryAnalyticsHubListingCommercialInfoElCloudMarketplaceEl {
    #[doc = "Set the field `commercial_state`.\n"]
    pub fn set_commercial_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.commercial_state = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryAnalyticsHubListingCommercialInfoElCloudMarketplaceEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingCommercialInfoElCloudMarketplaceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubListingCommercialInfoElCloudMarketplaceEl {}
impl BuildBigqueryAnalyticsHubListingCommercialInfoElCloudMarketplaceEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingCommercialInfoElCloudMarketplaceEl {
        BigqueryAnalyticsHubListingCommercialInfoElCloudMarketplaceEl {
            commercial_state: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct BigqueryAnalyticsHubListingCommercialInfoElCloudMarketplaceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingCommercialInfoElCloudMarketplaceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryAnalyticsHubListingCommercialInfoElCloudMarketplaceElRef {
        BigqueryAnalyticsHubListingCommercialInfoElCloudMarketplaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubListingCommercialInfoElCloudMarketplaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `commercial_state` after provisioning.\n"]
    pub fn commercial_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.commercial_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingCommercialInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_marketplace:
        Option<ListField<BigqueryAnalyticsHubListingCommercialInfoElCloudMarketplaceEl>>,
}
impl BigqueryAnalyticsHubListingCommercialInfoEl {
    #[doc = "Set the field `cloud_marketplace`.\n"]
    pub fn set_cloud_marketplace(
        mut self,
        v: impl Into<ListField<BigqueryAnalyticsHubListingCommercialInfoElCloudMarketplaceEl>>,
    ) -> Self {
        self.cloud_marketplace = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryAnalyticsHubListingCommercialInfoEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingCommercialInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubListingCommercialInfoEl {}
impl BuildBigqueryAnalyticsHubListingCommercialInfoEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingCommercialInfoEl {
        BigqueryAnalyticsHubListingCommercialInfoEl {
            cloud_marketplace: core::default::Default::default(),
        }
    }
}
pub struct BigqueryAnalyticsHubListingCommercialInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingCommercialInfoElRef {
    fn new(shared: StackShared, base: String) -> BigqueryAnalyticsHubListingCommercialInfoElRef {
        BigqueryAnalyticsHubListingCommercialInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubListingCommercialInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_marketplace` after provisioning.\n"]
    pub fn cloud_marketplace(
        &self,
    ) -> ListRef<BigqueryAnalyticsHubListingCommercialInfoElCloudMarketplaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_marketplace", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingBigqueryDatasetElEffectiveReplicasEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_state: Option<PrimField<String>>,
}
impl BigqueryAnalyticsHubListingBigqueryDatasetElEffectiveReplicasEl {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `primary_state`.\n"]
    pub fn set_primary_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.primary_state = Some(v.into());
        self
    }
    #[doc = "Set the field `replica_state`.\n"]
    pub fn set_replica_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.replica_state = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryAnalyticsHubListingBigqueryDatasetElEffectiveReplicasEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingBigqueryDatasetElEffectiveReplicasEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubListingBigqueryDatasetElEffectiveReplicasEl {}
impl BuildBigqueryAnalyticsHubListingBigqueryDatasetElEffectiveReplicasEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingBigqueryDatasetElEffectiveReplicasEl {
        BigqueryAnalyticsHubListingBigqueryDatasetElEffectiveReplicasEl {
            location: core::default::Default::default(),
            primary_state: core::default::Default::default(),
            replica_state: core::default::Default::default(),
        }
    }
}
pub struct BigqueryAnalyticsHubListingBigqueryDatasetElEffectiveReplicasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingBigqueryDatasetElEffectiveReplicasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryAnalyticsHubListingBigqueryDatasetElEffectiveReplicasElRef {
        BigqueryAnalyticsHubListingBigqueryDatasetElEffectiveReplicasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubListingBigqueryDatasetElEffectiveReplicasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `primary_state` after provisioning.\n"]
    pub fn primary_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `replica_state` after provisioning.\n"]
    pub fn replica_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replica_state", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingBigqueryDatasetElSelectedResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    routine: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table: Option<PrimField<String>>,
}
impl BigqueryAnalyticsHubListingBigqueryDatasetElSelectedResourcesEl {
    #[doc = "Set the field `routine`.\nFormat: For routine: projects/{projectId}/datasets/{datasetId}/routines/{routineId} Example:\"projects/test_project/datasets/test_dataset/routines/test_routine\""]
    pub fn set_routine(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.routine = Some(v.into());
        self
    }
    #[doc = "Set the field `table`.\nFormat: For table: projects/{projectId}/datasets/{datasetId}/tables/{tableId} Example:\"projects/test_project/datasets/test_dataset/tables/test_table\""]
    pub fn set_table(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryAnalyticsHubListingBigqueryDatasetElSelectedResourcesEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingBigqueryDatasetElSelectedResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubListingBigqueryDatasetElSelectedResourcesEl {}
impl BuildBigqueryAnalyticsHubListingBigqueryDatasetElSelectedResourcesEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingBigqueryDatasetElSelectedResourcesEl {
        BigqueryAnalyticsHubListingBigqueryDatasetElSelectedResourcesEl {
            routine: core::default::Default::default(),
            table: core::default::Default::default(),
        }
    }
}
pub struct BigqueryAnalyticsHubListingBigqueryDatasetElSelectedResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingBigqueryDatasetElSelectedResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryAnalyticsHubListingBigqueryDatasetElSelectedResourcesElRef {
        BigqueryAnalyticsHubListingBigqueryDatasetElSelectedResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubListingBigqueryDatasetElSelectedResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `routine` after provisioning.\nFormat: For routine: projects/{projectId}/datasets/{datasetId}/routines/{routineId} Example:\"projects/test_project/datasets/test_dataset/routines/test_routine\""]
    pub fn routine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.routine", self.base))
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\nFormat: For table: projects/{projectId}/datasets/{datasetId}/tables/{tableId} Example:\"projects/test_project/datasets/test_dataset/tables/test_table\""]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
}
#[derive(Serialize, Default)]
struct BigqueryAnalyticsHubListingBigqueryDatasetElDynamic {
    selected_resources:
        Option<DynamicBlock<BigqueryAnalyticsHubListingBigqueryDatasetElSelectedResourcesEl>>,
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingBigqueryDatasetEl {
    dataset: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_locations: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_resources:
        Option<Vec<BigqueryAnalyticsHubListingBigqueryDatasetElSelectedResourcesEl>>,
    dynamic: BigqueryAnalyticsHubListingBigqueryDatasetElDynamic,
}
impl BigqueryAnalyticsHubListingBigqueryDatasetEl {
    #[doc = "Set the field `replica_locations`.\nA list of regions where the publisher has created shared dataset replicas."]
    pub fn set_replica_locations(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.replica_locations = Some(v.into());
        self
    }
    #[doc = "Set the field `selected_resources`.\n"]
    pub fn set_selected_resources(
        mut self,
        v: impl Into<BlockAssignable<BigqueryAnalyticsHubListingBigqueryDatasetElSelectedResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.selected_resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.selected_resources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BigqueryAnalyticsHubListingBigqueryDatasetEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingBigqueryDatasetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubListingBigqueryDatasetEl {
    #[doc = "Resource name of the dataset source for this listing. e.g. projects/myproject/datasets/123"]
    pub dataset: PrimField<String>,
}
impl BuildBigqueryAnalyticsHubListingBigqueryDatasetEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingBigqueryDatasetEl {
        BigqueryAnalyticsHubListingBigqueryDatasetEl {
            dataset: self.dataset,
            replica_locations: core::default::Default::default(),
            selected_resources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BigqueryAnalyticsHubListingBigqueryDatasetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingBigqueryDatasetElRef {
    fn new(shared: StackShared, base: String) -> BigqueryAnalyticsHubListingBigqueryDatasetElRef {
        BigqueryAnalyticsHubListingBigqueryDatasetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubListingBigqueryDatasetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\nResource name of the dataset source for this listing. e.g. projects/myproject/datasets/123"]
    pub fn dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.base))
    }
    #[doc = "Get a reference to the value of field `effective_replicas` after provisioning.\nServer owned effective state of replicas. Contains both primary and secondary replicas.\nEach replica includes a system-computed (output-only) state and primary designation."]
    pub fn effective_replicas(
        &self,
    ) -> ListRef<BigqueryAnalyticsHubListingBigqueryDatasetElEffectiveReplicasElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.effective_replicas", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `replica_locations` after provisioning.\nA list of regions where the publisher has created shared dataset replicas."]
    pub fn replica_locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.replica_locations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `selected_resources` after provisioning.\n"]
    pub fn selected_resources(
        &self,
    ) -> ListRef<BigqueryAnalyticsHubListingBigqueryDatasetElSelectedResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.selected_resources", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingDataProviderEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_contact: Option<PrimField<String>>,
}
impl BigqueryAnalyticsHubListingDataProviderEl {
    #[doc = "Set the field `primary_contact`.\nEmail or URL of the data provider."]
    pub fn set_primary_contact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.primary_contact = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryAnalyticsHubListingDataProviderEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingDataProviderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubListingDataProviderEl {
    #[doc = "Name of the data provider."]
    pub name: PrimField<String>,
}
impl BuildBigqueryAnalyticsHubListingDataProviderEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingDataProviderEl {
        BigqueryAnalyticsHubListingDataProviderEl {
            name: self.name,
            primary_contact: core::default::Default::default(),
        }
    }
}
pub struct BigqueryAnalyticsHubListingDataProviderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingDataProviderElRef {
    fn new(shared: StackShared, base: String) -> BigqueryAnalyticsHubListingDataProviderElRef {
        BigqueryAnalyticsHubListingDataProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubListingDataProviderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the data provider."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the data provider."]
    pub fn primary_contact(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_contact", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingPublisherEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_contact: Option<PrimField<String>>,
}
impl BigqueryAnalyticsHubListingPublisherEl {
    #[doc = "Set the field `primary_contact`.\nEmail or URL of the listing publisher."]
    pub fn set_primary_contact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.primary_contact = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryAnalyticsHubListingPublisherEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingPublisherEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubListingPublisherEl {
    #[doc = "Name of the listing publisher."]
    pub name: PrimField<String>,
}
impl BuildBigqueryAnalyticsHubListingPublisherEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingPublisherEl {
        BigqueryAnalyticsHubListingPublisherEl {
            name: self.name,
            primary_contact: core::default::Default::default(),
        }
    }
}
pub struct BigqueryAnalyticsHubListingPublisherElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingPublisherElRef {
    fn new(shared: StackShared, base: String) -> BigqueryAnalyticsHubListingPublisherElRef {
        BigqueryAnalyticsHubListingPublisherElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubListingPublisherElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the listing publisher."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the listing publisher."]
    pub fn primary_contact(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_contact", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingPubsubTopicEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_affinity_regions: Option<SetField<PrimField<String>>>,
    topic: PrimField<String>,
}
impl BigqueryAnalyticsHubListingPubsubTopicEl {
    #[doc = "Set the field `data_affinity_regions`.\nRegion hint on where the data might be published. Data affinity regions are modifiable.\nSee https://cloud.google.com/about/locations for full listing of possible Cloud regions."]
    pub fn set_data_affinity_regions(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.data_affinity_regions = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryAnalyticsHubListingPubsubTopicEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingPubsubTopicEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubListingPubsubTopicEl {
    #[doc = "Resource name of the Pub/Sub topic source for this listing. e.g. projects/myproject/topics/topicId"]
    pub topic: PrimField<String>,
}
impl BuildBigqueryAnalyticsHubListingPubsubTopicEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingPubsubTopicEl {
        BigqueryAnalyticsHubListingPubsubTopicEl {
            data_affinity_regions: core::default::Default::default(),
            topic: self.topic,
        }
    }
}
pub struct BigqueryAnalyticsHubListingPubsubTopicElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingPubsubTopicElRef {
    fn new(shared: StackShared, base: String) -> BigqueryAnalyticsHubListingPubsubTopicElRef {
        BigqueryAnalyticsHubListingPubsubTopicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubListingPubsubTopicElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_affinity_regions` after provisioning.\nRegion hint on where the data might be published. Data affinity regions are modifiable.\nSee https://cloud.google.com/about/locations for full listing of possible Cloud regions."]
    pub fn data_affinity_regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.data_affinity_regions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `topic` after provisioning.\nResource name of the Pub/Sub topic source for this listing. e.g. projects/myproject/topics/topicId"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingRestrictedExportConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrict_query_result: Option<PrimField<bool>>,
}
impl BigqueryAnalyticsHubListingRestrictedExportConfigEl {
    #[doc = "Set the field `enabled`.\nIf true, enable restricted export."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `restrict_query_result`.\nIf true, restrict export of query result derived from restricted linked dataset table."]
    pub fn set_restrict_query_result(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.restrict_query_result = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryAnalyticsHubListingRestrictedExportConfigEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingRestrictedExportConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubListingRestrictedExportConfigEl {}
impl BuildBigqueryAnalyticsHubListingRestrictedExportConfigEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingRestrictedExportConfigEl {
        BigqueryAnalyticsHubListingRestrictedExportConfigEl {
            enabled: core::default::Default::default(),
            restrict_query_result: core::default::Default::default(),
        }
    }
}
pub struct BigqueryAnalyticsHubListingRestrictedExportConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingRestrictedExportConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryAnalyticsHubListingRestrictedExportConfigElRef {
        BigqueryAnalyticsHubListingRestrictedExportConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubListingRestrictedExportConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nIf true, enable restricted export."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `restrict_direct_table_access` after provisioning.\nIf true, restrict direct table access(read api/tabledata.list) on linked table."]
    pub fn restrict_direct_table_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.restrict_direct_table_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `restrict_query_result` after provisioning.\nIf true, restrict export of query result derived from restricted linked dataset table."]
    pub fn restrict_query_result(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.restrict_query_result", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BigqueryAnalyticsHubListingTimeoutsEl {
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
impl ToListMappable for BigqueryAnalyticsHubListingTimeoutsEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubListingTimeoutsEl {}
impl BuildBigqueryAnalyticsHubListingTimeoutsEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingTimeoutsEl {
        BigqueryAnalyticsHubListingTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BigqueryAnalyticsHubListingTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryAnalyticsHubListingTimeoutsElRef {
        BigqueryAnalyticsHubListingTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubListingTimeoutsElRef {
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
struct BigqueryAnalyticsHubListingDynamic {
    bigquery_dataset: Option<DynamicBlock<BigqueryAnalyticsHubListingBigqueryDatasetEl>>,
    data_provider: Option<DynamicBlock<BigqueryAnalyticsHubListingDataProviderEl>>,
    publisher: Option<DynamicBlock<BigqueryAnalyticsHubListingPublisherEl>>,
    pubsub_topic: Option<DynamicBlock<BigqueryAnalyticsHubListingPubsubTopicEl>>,
    restricted_export_config:
        Option<DynamicBlock<BigqueryAnalyticsHubListingRestrictedExportConfigEl>>,
}
