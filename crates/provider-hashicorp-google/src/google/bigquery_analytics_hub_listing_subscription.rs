use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BigqueryAnalyticsHubListingSubscriptionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    data_exchange_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    listing_id: PrimField<String>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_dataset: Option<Vec<BigqueryAnalyticsHubListingSubscriptionDestinationDatasetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigqueryAnalyticsHubListingSubscriptionTimeoutsEl>,
    dynamic: BigqueryAnalyticsHubListingSubscriptionDynamic,
}
struct BigqueryAnalyticsHubListingSubscription_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryAnalyticsHubListingSubscriptionData>,
}
#[derive(Clone)]
pub struct BigqueryAnalyticsHubListingSubscription(Rc<BigqueryAnalyticsHubListingSubscription_>);
impl BigqueryAnalyticsHubListingSubscription {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `destination_dataset`.\n"]
    pub fn set_destination_dataset(
        self,
        v: impl Into<BlockAssignable<BigqueryAnalyticsHubListingSubscriptionDestinationDatasetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_dataset = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_dataset = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<BigqueryAnalyticsHubListingSubscriptionTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `commercial_info` after provisioning.\nCommercial info metadata for this subscription. This is set if this is a commercial subscription i.e. if this subscription was created from subscribing to a commercial listing."]
    pub fn commercial_info(
        &self,
    ) -> ListRef<BigqueryAnalyticsHubListingSubscriptionCommercialInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.commercial_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\nTimestamp when the subscription was created."]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_exchange_id` after provisioning.\nThe ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn data_exchange_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_exchange_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `last_modify_time` after provisioning.\nTimestamp when the subscription was last modified."]
    pub fn last_modify_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modify_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_dataset_map` after provisioning.\nOutput only. Map of listing resource names to associated linked resource,\ne.g. projects/123/locations/US/dataExchanges/456/listings/789 -> projects/123/datasets/my_dataset"]
    pub fn linked_dataset_map(
        &self,
    ) -> SetRef<BigqueryAnalyticsHubListingSubscriptionLinkedDatasetMapElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.linked_dataset_map", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_resources` after provisioning.\nOutput only. Linked resources created in the subscription. Only contains values if state = STATE_ACTIVE."]
    pub fn linked_resources(
        &self,
    ) -> ListRef<BigqueryAnalyticsHubListingSubscriptionLinkedResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linked_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `listing_id` after provisioning.\nThe ID of the listing. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn listing_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listing_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location of the data exchange. Distinct from the location of the destination data set."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_linked_dataset_query_user_email` after provisioning.\nOutput only. By default, false. If true, the Subscriber agreed to the email sharing mandate that is enabled for Listing."]
    pub fn log_linked_dataset_query_user_email(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_linked_dataset_query_user_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the subscription. e.g. \"projects/myproject/locations/US/subscriptions/123\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_display_name` after provisioning.\nDisplay name of the project of this subscription."]
    pub fn organization_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\nOrganization of the project this subscription belongs to."]
    pub fn organization_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nListing shared asset type."]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the subscription."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subscriber_contact` after provisioning.\nEmail of the subscriber."]
    pub fn subscriber_contact(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscriber_contact", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subscription_id` after provisioning.\nThe subscription id used to reference the subscription."]
    pub fn subscription_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_dataset` after provisioning.\n"]
    pub fn destination_dataset(
        &self,
    ) -> ListRef<BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_dataset", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryAnalyticsHubListingSubscriptionTimeoutsElRef {
        BigqueryAnalyticsHubListingSubscriptionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BigqueryAnalyticsHubListingSubscription {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BigqueryAnalyticsHubListingSubscription {}
impl ToListMappable for BigqueryAnalyticsHubListingSubscription {
    type O = ListRef<BigqueryAnalyticsHubListingSubscriptionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BigqueryAnalyticsHubListingSubscription_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_analytics_hub_listing_subscription".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBigqueryAnalyticsHubListingSubscription {
    pub tf_id: String,
    #[doc = "The ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub data_exchange_id: PrimField<String>,
    #[doc = "The ID of the listing. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub listing_id: PrimField<String>,
    #[doc = "The name of the location of the data exchange. Distinct from the location of the destination data set."]
    pub location: PrimField<String>,
}
impl BuildBigqueryAnalyticsHubListingSubscription {
    pub fn build(self, stack: &mut Stack) -> BigqueryAnalyticsHubListingSubscription {
        let out = BigqueryAnalyticsHubListingSubscription(Rc::new(
            BigqueryAnalyticsHubListingSubscription_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(BigqueryAnalyticsHubListingSubscriptionData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    data_exchange_id: self.data_exchange_id,
                    deletion_policy: core::default::Default::default(),
                    id: core::default::Default::default(),
                    listing_id: self.listing_id,
                    location: self.location,
                    project: core::default::Default::default(),
                    destination_dataset: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BigqueryAnalyticsHubListingSubscriptionRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingSubscriptionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BigqueryAnalyticsHubListingSubscriptionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `commercial_info` after provisioning.\nCommercial info metadata for this subscription. This is set if this is a commercial subscription i.e. if this subscription was created from subscribing to a commercial listing."]
    pub fn commercial_info(
        &self,
    ) -> ListRef<BigqueryAnalyticsHubListingSubscriptionCommercialInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.commercial_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\nTimestamp when the subscription was created."]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_exchange_id` after provisioning.\nThe ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn data_exchange_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_exchange_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `last_modify_time` after provisioning.\nTimestamp when the subscription was last modified."]
    pub fn last_modify_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modify_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_dataset_map` after provisioning.\nOutput only. Map of listing resource names to associated linked resource,\ne.g. projects/123/locations/US/dataExchanges/456/listings/789 -> projects/123/datasets/my_dataset"]
    pub fn linked_dataset_map(
        &self,
    ) -> SetRef<BigqueryAnalyticsHubListingSubscriptionLinkedDatasetMapElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.linked_dataset_map", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_resources` after provisioning.\nOutput only. Linked resources created in the subscription. Only contains values if state = STATE_ACTIVE."]
    pub fn linked_resources(
        &self,
    ) -> ListRef<BigqueryAnalyticsHubListingSubscriptionLinkedResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.linked_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `listing_id` after provisioning.\nThe ID of the listing. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn listing_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listing_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location of the data exchange. Distinct from the location of the destination data set."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_linked_dataset_query_user_email` after provisioning.\nOutput only. By default, false. If true, the Subscriber agreed to the email sharing mandate that is enabled for Listing."]
    pub fn log_linked_dataset_query_user_email(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_linked_dataset_query_user_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the subscription. e.g. \"projects/myproject/locations/US/subscriptions/123\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_display_name` after provisioning.\nDisplay name of the project of this subscription."]
    pub fn organization_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization_id` after provisioning.\nOrganization of the project this subscription belongs to."]
    pub fn organization_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nListing shared asset type."]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the subscription."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subscriber_contact` after provisioning.\nEmail of the subscriber."]
    pub fn subscriber_contact(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscriber_contact", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subscription_id` after provisioning.\nThe subscription id used to reference the subscription."]
    pub fn subscription_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_dataset` after provisioning.\n"]
    pub fn destination_dataset(
        &self,
    ) -> ListRef<BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_dataset", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryAnalyticsHubListingSubscriptionTimeoutsElRef {
        BigqueryAnalyticsHubListingSubscriptionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscriptionCommercialInfoElCloudMarketplaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<PrimField<String>>,
}
impl BigqueryAnalyticsHubListingSubscriptionCommercialInfoElCloudMarketplaceEl {
    #[doc = "Set the field `order`.\n"]
    pub fn set_order(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.order = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryAnalyticsHubListingSubscriptionCommercialInfoElCloudMarketplaceEl {
    type O =
        BlockAssignable<BigqueryAnalyticsHubListingSubscriptionCommercialInfoElCloudMarketplaceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubListingSubscriptionCommercialInfoElCloudMarketplaceEl {}
impl BuildBigqueryAnalyticsHubListingSubscriptionCommercialInfoElCloudMarketplaceEl {
    pub fn build(
        self,
    ) -> BigqueryAnalyticsHubListingSubscriptionCommercialInfoElCloudMarketplaceEl {
        BigqueryAnalyticsHubListingSubscriptionCommercialInfoElCloudMarketplaceEl {
            order: core::default::Default::default(),
        }
    }
}
pub struct BigqueryAnalyticsHubListingSubscriptionCommercialInfoElCloudMarketplaceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingSubscriptionCommercialInfoElCloudMarketplaceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryAnalyticsHubListingSubscriptionCommercialInfoElCloudMarketplaceElRef {
        BigqueryAnalyticsHubListingSubscriptionCommercialInfoElCloudMarketplaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubListingSubscriptionCommercialInfoElCloudMarketplaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `order` after provisioning.\n"]
    pub fn order(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscriptionCommercialInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_marketplace: Option<
        ListField<BigqueryAnalyticsHubListingSubscriptionCommercialInfoElCloudMarketplaceEl>,
    >,
}
impl BigqueryAnalyticsHubListingSubscriptionCommercialInfoEl {
    #[doc = "Set the field `cloud_marketplace`.\n"]
    pub fn set_cloud_marketplace(
        mut self,
        v: impl Into<
            ListField<BigqueryAnalyticsHubListingSubscriptionCommercialInfoElCloudMarketplaceEl>,
        >,
    ) -> Self {
        self.cloud_marketplace = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryAnalyticsHubListingSubscriptionCommercialInfoEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingSubscriptionCommercialInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubListingSubscriptionCommercialInfoEl {}
impl BuildBigqueryAnalyticsHubListingSubscriptionCommercialInfoEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingSubscriptionCommercialInfoEl {
        BigqueryAnalyticsHubListingSubscriptionCommercialInfoEl {
            cloud_marketplace: core::default::Default::default(),
        }
    }
}
pub struct BigqueryAnalyticsHubListingSubscriptionCommercialInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingSubscriptionCommercialInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryAnalyticsHubListingSubscriptionCommercialInfoElRef {
        BigqueryAnalyticsHubListingSubscriptionCommercialInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubListingSubscriptionCommercialInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_marketplace` after provisioning.\n"]
    pub fn cloud_marketplace(
        &self,
    ) -> ListRef<BigqueryAnalyticsHubListingSubscriptionCommercialInfoElCloudMarketplaceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_marketplace", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscriptionLinkedDatasetMapEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    linked_dataset: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    listing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_name: Option<PrimField<String>>,
}
impl BigqueryAnalyticsHubListingSubscriptionLinkedDatasetMapEl {
    #[doc = "Set the field `linked_dataset`.\n"]
    pub fn set_linked_dataset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.linked_dataset = Some(v.into());
        self
    }
    #[doc = "Set the field `listing`.\n"]
    pub fn set_listing(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.listing = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_name`.\n"]
    pub fn set_resource_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_name = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryAnalyticsHubListingSubscriptionLinkedDatasetMapEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingSubscriptionLinkedDatasetMapEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubListingSubscriptionLinkedDatasetMapEl {}
impl BuildBigqueryAnalyticsHubListingSubscriptionLinkedDatasetMapEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingSubscriptionLinkedDatasetMapEl {
        BigqueryAnalyticsHubListingSubscriptionLinkedDatasetMapEl {
            linked_dataset: core::default::Default::default(),
            listing: core::default::Default::default(),
            resource_name: core::default::Default::default(),
        }
    }
}
pub struct BigqueryAnalyticsHubListingSubscriptionLinkedDatasetMapElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingSubscriptionLinkedDatasetMapElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryAnalyticsHubListingSubscriptionLinkedDatasetMapElRef {
        BigqueryAnalyticsHubListingSubscriptionLinkedDatasetMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubListingSubscriptionLinkedDatasetMapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `linked_dataset` after provisioning.\n"]
    pub fn linked_dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.linked_dataset", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `listing` after provisioning.\n"]
    pub fn listing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.listing", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_name` after provisioning.\n"]
    pub fn resource_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscriptionLinkedResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    linked_dataset: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    listing: Option<PrimField<String>>,
}
impl BigqueryAnalyticsHubListingSubscriptionLinkedResourcesEl {
    #[doc = "Set the field `linked_dataset`.\n"]
    pub fn set_linked_dataset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.linked_dataset = Some(v.into());
        self
    }
    #[doc = "Set the field `listing`.\n"]
    pub fn set_listing(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.listing = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryAnalyticsHubListingSubscriptionLinkedResourcesEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingSubscriptionLinkedResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubListingSubscriptionLinkedResourcesEl {}
impl BuildBigqueryAnalyticsHubListingSubscriptionLinkedResourcesEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingSubscriptionLinkedResourcesEl {
        BigqueryAnalyticsHubListingSubscriptionLinkedResourcesEl {
            linked_dataset: core::default::Default::default(),
            listing: core::default::Default::default(),
        }
    }
}
pub struct BigqueryAnalyticsHubListingSubscriptionLinkedResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingSubscriptionLinkedResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryAnalyticsHubListingSubscriptionLinkedResourcesElRef {
        BigqueryAnalyticsHubListingSubscriptionLinkedResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubListingSubscriptionLinkedResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `linked_dataset` after provisioning.\n"]
    pub fn linked_dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.linked_dataset", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `listing` after provisioning.\n"]
    pub fn listing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.listing", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDatasetReferenceEl {
    dataset_id: PrimField<String>,
    project_id: PrimField<String>,
}
impl BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDatasetReferenceEl {}
impl ToListMappable
    for BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDatasetReferenceEl
{
    type O = BlockAssignable<
        BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDatasetReferenceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDatasetReferenceEl {
    #[doc = "A unique ID for this dataset, without the project name. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters."]
    pub dataset_id: PrimField<String>,
    #[doc = "The ID of the project containing this dataset."]
    pub project_id: PrimField<String>,
}
impl BuildBigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDatasetReferenceEl {
    pub fn build(
        self,
    ) -> BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDatasetReferenceEl {
        BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDatasetReferenceEl {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
        }
    }
}
pub struct BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDatasetReferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDatasetReferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDatasetReferenceElRef {
        BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDatasetReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDatasetReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nA unique ID for this dataset, without the project name. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this dataset."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDynamic {
    dataset_reference: Option<
        DynamicBlock<BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDatasetReferenceEl>,
    >,
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscriptionDestinationDatasetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    friendly_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_locations: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_reference:
        Option<Vec<BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDatasetReferenceEl>>,
    dynamic: BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDynamic,
}
impl BigqueryAnalyticsHubListingSubscriptionDestinationDatasetEl {
    #[doc = "Set the field `description`.\nA user-friendly description of the dataset."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `friendly_name`.\nA descriptive name for the dataset."]
    pub fn set_friendly_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.friendly_name = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe labels associated with this dataset. You can use these to\norganize and group your datasets."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `replica_locations`.\nList of regions where the subscriber wants dataset replicas."]
    pub fn set_replica_locations(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.replica_locations = Some(v.into());
        self
    }
    #[doc = "Set the field `dataset_reference`.\n"]
    pub fn set_dataset_reference(
        mut self,
        v: impl Into<
            BlockAssignable<
                BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDatasetReferenceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dataset_reference = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dataset_reference = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BigqueryAnalyticsHubListingSubscriptionDestinationDatasetEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingSubscriptionDestinationDatasetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubListingSubscriptionDestinationDatasetEl {
    #[doc = "The geographic location where the dataset should reside.\nSee https://cloud.google.com/bigquery/docs/locations for supported locations."]
    pub location: PrimField<String>,
}
impl BuildBigqueryAnalyticsHubListingSubscriptionDestinationDatasetEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingSubscriptionDestinationDatasetEl {
        BigqueryAnalyticsHubListingSubscriptionDestinationDatasetEl {
            description: core::default::Default::default(),
            friendly_name: core::default::Default::default(),
            labels: core::default::Default::default(),
            location: self.location,
            replica_locations: core::default::Default::default(),
            dataset_reference: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElRef {
        BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA user-friendly description of the dataset."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the dataset."]
    pub fn friendly_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.friendly_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this dataset. You can use these to\norganize and group your datasets."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the dataset should reside.\nSee https://cloud.google.com/bigquery/docs/locations for supported locations."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `replica_locations` after provisioning.\nList of regions where the subscriber wants dataset replicas."]
    pub fn replica_locations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.replica_locations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dataset_reference` after provisioning.\n"]
    pub fn dataset_reference(
        &self,
    ) -> ListRef<BigqueryAnalyticsHubListingSubscriptionDestinationDatasetElDatasetReferenceElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dataset_reference", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscriptionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl BigqueryAnalyticsHubListingSubscriptionTimeoutsEl {
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
impl ToListMappable for BigqueryAnalyticsHubListingSubscriptionTimeoutsEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingSubscriptionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubListingSubscriptionTimeoutsEl {}
impl BuildBigqueryAnalyticsHubListingSubscriptionTimeoutsEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingSubscriptionTimeoutsEl {
        BigqueryAnalyticsHubListingSubscriptionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct BigqueryAnalyticsHubListingSubscriptionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubListingSubscriptionTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryAnalyticsHubListingSubscriptionTimeoutsElRef {
        BigqueryAnalyticsHubListingSubscriptionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubListingSubscriptionTimeoutsElRef {
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
struct BigqueryAnalyticsHubListingSubscriptionDynamic {
    destination_dataset:
        Option<DynamicBlock<BigqueryAnalyticsHubListingSubscriptionDestinationDatasetEl>>,
}
