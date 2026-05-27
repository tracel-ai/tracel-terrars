use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BigqueryAnalyticsHubDataExchangeData {
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
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_linked_dataset_query_user_email: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_contact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sharing_environment_config:
        Option<Vec<BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigqueryAnalyticsHubDataExchangeTimeoutsEl>,
    dynamic: BigqueryAnalyticsHubDataExchangeDynamic,
}
struct BigqueryAnalyticsHubDataExchange_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryAnalyticsHubDataExchangeData>,
}
#[derive(Clone)]
pub struct BigqueryAnalyticsHubDataExchange(Rc<BigqueryAnalyticsHubDataExchange_>);
impl BigqueryAnalyticsHubDataExchange {
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
    #[doc = "Set the field `description`.\nDescription of the data exchange."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `discovery_type`.\nType of discovery on the discovery page for all the listings under this exchange. Cannot be set for a Data Clean Room. Updating this field also updates (overwrites) the discoveryType field for all the listings under this exchange. Possible values: [\"DISCOVERY_TYPE_PRIVATE\", \"DISCOVERY_TYPE_PUBLIC\"]"]
    pub fn set_discovery_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().discovery_type = Some(v.into());
        self
    }
    #[doc = "Set the field `documentation`.\nDocumentation describing the data exchange."]
    pub fn set_documentation(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().documentation = Some(v.into());
        self
    }
    #[doc = "Set the field `icon`.\nBase64 encoded image representing the data exchange."]
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
    #[doc = "Set the field `primary_contact`.\nEmail or URL of the primary point of contact of the data exchange."]
    pub fn set_primary_contact(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().primary_contact = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `sharing_environment_config`.\n"]
    pub fn set_sharing_environment_config(
        self,
        v: impl Into<BlockAssignable<BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sharing_environment_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sharing_environment_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigqueryAnalyticsHubDataExchangeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the data exchange."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_type` after provisioning.\nType of discovery on the discovery page for all the listings under this exchange. Cannot be set for a Data Clean Room. Updating this field also updates (overwrites) the discoveryType field for all the listings under this exchange. Possible values: [\"DISCOVERY_TYPE_PRIVATE\", \"DISCOVERY_TYPE_PUBLIC\"]"]
    pub fn discovery_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.discovery_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of the data exchange. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), and must not start or end with spaces."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `documentation` after provisioning.\nDocumentation describing the data exchange."]
    pub fn documentation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.documentation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `icon` after provisioning.\nBase64 encoded image representing the data exchange."]
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
    #[doc = "Get a reference to the value of field `listing_count` after provisioning.\nNumber of listings contained in the data exchange."]
    pub fn listing_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listing_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location this data exchange."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the data exchange, for example:\n\"projects/myproject/locations/US/dataExchanges/123\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the primary point of contact of the data exchange."]
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
    #[doc = "Get a reference to the value of field `sharing_environment_config` after provisioning.\n"]
    pub fn sharing_environment_config(
        &self,
    ) -> ListRef<BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sharing_environment_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryAnalyticsHubDataExchangeTimeoutsElRef {
        BigqueryAnalyticsHubDataExchangeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BigqueryAnalyticsHubDataExchange {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BigqueryAnalyticsHubDataExchange {}
impl ToListMappable for BigqueryAnalyticsHubDataExchange {
    type O = ListRef<BigqueryAnalyticsHubDataExchangeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BigqueryAnalyticsHubDataExchange_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_analytics_hub_data_exchange".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBigqueryAnalyticsHubDataExchange {
    pub tf_id: String,
    #[doc = "The ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub data_exchange_id: PrimField<String>,
    #[doc = "Human-readable display name of the data exchange. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), and must not start or end with spaces."]
    pub display_name: PrimField<String>,
    #[doc = "The name of the location this data exchange."]
    pub location: PrimField<String>,
}
impl BuildBigqueryAnalyticsHubDataExchange {
    pub fn build(self, stack: &mut Stack) -> BigqueryAnalyticsHubDataExchange {
        let out = BigqueryAnalyticsHubDataExchange(Rc::new(BigqueryAnalyticsHubDataExchange_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigqueryAnalyticsHubDataExchangeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                data_exchange_id: self.data_exchange_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                discovery_type: core::default::Default::default(),
                display_name: self.display_name,
                documentation: core::default::Default::default(),
                icon: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                log_linked_dataset_query_user_email: core::default::Default::default(),
                primary_contact: core::default::Default::default(),
                project: core::default::Default::default(),
                sharing_environment_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BigqueryAnalyticsHubDataExchangeRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubDataExchangeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BigqueryAnalyticsHubDataExchangeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the data exchange."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_type` after provisioning.\nType of discovery on the discovery page for all the listings under this exchange. Cannot be set for a Data Clean Room. Updating this field also updates (overwrites) the discoveryType field for all the listings under this exchange. Possible values: [\"DISCOVERY_TYPE_PRIVATE\", \"DISCOVERY_TYPE_PUBLIC\"]"]
    pub fn discovery_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.discovery_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of the data exchange. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), and must not start or end with spaces."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `documentation` after provisioning.\nDocumentation describing the data exchange."]
    pub fn documentation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.documentation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `icon` after provisioning.\nBase64 encoded image representing the data exchange."]
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
    #[doc = "Get a reference to the value of field `listing_count` after provisioning.\nNumber of listings contained in the data exchange."]
    pub fn listing_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listing_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location this data exchange."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the data exchange, for example:\n\"projects/myproject/locations/US/dataExchanges/123\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the primary point of contact of the data exchange."]
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
    #[doc = "Get a reference to the value of field `sharing_environment_config` after provisioning.\n"]
    pub fn sharing_environment_config(
        &self,
    ) -> ListRef<BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sharing_environment_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryAnalyticsHubDataExchangeTimeoutsElRef {
        BigqueryAnalyticsHubDataExchangeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDcrExchangeConfigEl {}
impl BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDcrExchangeConfigEl {}
impl ToListMappable
    for BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDcrExchangeConfigEl
{
    type O = BlockAssignable<
        BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDcrExchangeConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDcrExchangeConfigEl {}
impl BuildBigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDcrExchangeConfigEl {
    pub fn build(
        self,
    ) -> BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDcrExchangeConfigEl {
        BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDcrExchangeConfigEl {}
    }
}
pub struct BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDcrExchangeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDcrExchangeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDcrExchangeConfigElRef {
        BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDcrExchangeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDcrExchangeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDefaultExchangeConfigEl {}
impl BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDefaultExchangeConfigEl {}
impl ToListMappable
    for BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDefaultExchangeConfigEl
{
    type O = BlockAssignable<
        BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDefaultExchangeConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDefaultExchangeConfigEl {
}
impl BuildBigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDefaultExchangeConfigEl {
    pub fn build(
        self,
    ) -> BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDefaultExchangeConfigEl {
        BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDefaultExchangeConfigEl {}
    }
}
pub struct BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDefaultExchangeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDefaultExchangeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDefaultExchangeConfigElRef {
        BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDefaultExchangeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDefaultExchangeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDynamic {
    dcr_exchange_config: Option<
        DynamicBlock<BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDcrExchangeConfigEl>,
    >,
    default_exchange_config: Option<
        DynamicBlock<
            BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDefaultExchangeConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dcr_exchange_config:
        Option<Vec<BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDcrExchangeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_exchange_config: Option<
        Vec<BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDefaultExchangeConfigEl>,
    >,
    dynamic: BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDynamic,
}
impl BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigEl {
    #[doc = "Set the field `dcr_exchange_config`.\n"]
    pub fn set_dcr_exchange_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDcrExchangeConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dcr_exchange_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dcr_exchange_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_exchange_config`.\n"]
    pub fn set_default_exchange_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDefaultExchangeConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_exchange_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_exchange_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigEl {
    type O = BlockAssignable<BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigEl {}
impl BuildBigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigEl {
    pub fn build(self) -> BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigEl {
        BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigEl {
            dcr_exchange_config: core::default::Default::default(),
            default_exchange_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElRef {
        BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dcr_exchange_config` after provisioning.\n"]
    pub fn dcr_exchange_config(
        &self,
    ) -> ListRef<BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDcrExchangeConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dcr_exchange_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_exchange_config` after provisioning.\n"]
    pub fn default_exchange_config(
        &self,
    ) -> ListRef<BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigElDefaultExchangeConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_exchange_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryAnalyticsHubDataExchangeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BigqueryAnalyticsHubDataExchangeTimeoutsEl {
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
impl ToListMappable for BigqueryAnalyticsHubDataExchangeTimeoutsEl {
    type O = BlockAssignable<BigqueryAnalyticsHubDataExchangeTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryAnalyticsHubDataExchangeTimeoutsEl {}
impl BuildBigqueryAnalyticsHubDataExchangeTimeoutsEl {
    pub fn build(self) -> BigqueryAnalyticsHubDataExchangeTimeoutsEl {
        BigqueryAnalyticsHubDataExchangeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BigqueryAnalyticsHubDataExchangeTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryAnalyticsHubDataExchangeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryAnalyticsHubDataExchangeTimeoutsElRef {
        BigqueryAnalyticsHubDataExchangeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryAnalyticsHubDataExchangeTimeoutsElRef {
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
struct BigqueryAnalyticsHubDataExchangeDynamic {
    sharing_environment_config:
        Option<DynamicBlock<BigqueryAnalyticsHubDataExchangeSharingEnvironmentConfigEl>>,
}
