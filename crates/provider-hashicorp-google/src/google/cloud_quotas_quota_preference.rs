use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudQuotasQuotaPreferenceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_safety_checks: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    justification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota_config: Option<Vec<CloudQuotasQuotaPreferenceQuotaConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudQuotasQuotaPreferenceTimeoutsEl>,
    dynamic: CloudQuotasQuotaPreferenceDynamic,
}
struct CloudQuotasQuotaPreference_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudQuotasQuotaPreferenceData>,
}
#[derive(Clone)]
pub struct CloudQuotasQuotaPreference(Rc<CloudQuotasQuotaPreference_>);
impl CloudQuotasQuotaPreference {
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
    #[doc = "Set the field `contact_email`.\nAn email address that can be used for quota related communication between the Google Cloud and the user in case the Google Cloud needs further information to make a decision on whether the user preferred quota can be granted.\n\nThe Google account for the email address must have quota update permission for the project, folder or organization this quota preference is for."]
    pub fn set_contact_email(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().contact_email = Some(v.into());
        self
    }
    #[doc = "Set the field `dimensions`.\nThe dimensions that this quota preference applies to. The key of the map entry is the name of a dimension, such as \"region\", \"zone\", \"network_id\", and the value of the map entry is the dimension value. If a dimension is missing from the map of dimensions, the quota preference applies to all the dimension values except for those that have other quota preferences configured for the specific value.\n\nNOTE: QuotaPreferences can only be applied across all values of \"user\" and \"resource\" dimension. Do not set values for \"user\" or \"resource\" in the dimension map.\n\nExample: '{\"provider\": \"Foo Inc\"}' where \"provider\" is a service specific dimension."]
    pub fn set_dimensions(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().dimensions = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_safety_checks`.\nThe list of quota safety checks to be ignored. Default value: \"QUOTA_SAFETY_CHECK_UNSPECIFIED\" Possible values: [\"QUOTA_SAFETY_CHECK_UNSPECIFIED\", \"QUOTA_DECREASE_BELOW_USAGE\", \"QUOTA_DECREASE_PERCENTAGE_TOO_HIGH\"]"]
    pub fn set_ignore_safety_checks(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ignore_safety_checks = Some(v.into());
        self
    }
    #[doc = "Set the field `justification`.\nThe reason / justification for this quota preference."]
    pub fn set_justification(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().justification = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nThe resource name of the quota preference. Required except in the CREATE requests."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `parent`.\nThe parent of the quota preference. Allowed parents are \"projects/[project-id / number]\" or \"folders/[folder-id / number]\" or \"organizations/[org-id / number]\"."]
    pub fn set_parent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent = Some(v.into());
        self
    }
    #[doc = "Set the field `quota_id`.\nThe id of the quota to which the quota preference is applied. A quota id is unique in the service.\nExample: 'CPUS-per-project-region'."]
    pub fn set_quota_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().quota_id = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\nThe name of the service to which the quota preference is applied."]
    pub fn set_service(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service = Some(v.into());
        self
    }
    #[doc = "Set the field `quota_config`.\n"]
    pub fn set_quota_config(
        self,
        v: impl Into<BlockAssignable<CloudQuotasQuotaPreferenceQuotaConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().quota_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.quota_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudQuotasQuotaPreferenceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `contact_email` after provisioning.\nAn email address that can be used for quota related communication between the Google Cloud and the user in case the Google Cloud needs further information to make a decision on whether the user preferred quota can be granted.\n\nThe Google account for the email address must have quota update permission for the project, folder or organization this quota preference is for."]
    pub fn contact_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contact_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time stamp.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: '2014-10-02T15:01:23Z' and '2014-10-02T15:01:23.045123456Z'."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\nThe dimensions that this quota preference applies to. The key of the map entry is the name of a dimension, such as \"region\", \"zone\", \"network_id\", and the value of the map entry is the dimension value. If a dimension is missing from the map of dimensions, the quota preference applies to all the dimension values except for those that have other quota preferences configured for the specific value.\n\nNOTE: QuotaPreferences can only be applied across all values of \"user\" and \"resource\" dimension. Do not set values for \"user\" or \"resource\" in the dimension map.\n\nExample: '{\"provider\": \"Foo Inc\"}' where \"provider\" is a service specific dimension."]
    pub fn dimensions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.dimensions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe current etag of the quota preference. If an etag is provided on update and does not match the current server's etag of the quota preference, the request will be blocked and an ABORTED error will be returned. See https://google.aip.dev/134#etags for more details on etags."]
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
    #[doc = "Get a reference to the value of field `ignore_safety_checks` after provisioning.\nThe list of quota safety checks to be ignored. Default value: \"QUOTA_SAFETY_CHECK_UNSPECIFIED\" Possible values: [\"QUOTA_SAFETY_CHECK_UNSPECIFIED\", \"QUOTA_DECREASE_BELOW_USAGE\", \"QUOTA_DECREASE_PERCENTAGE_TOO_HIGH\"]"]
    pub fn ignore_safety_checks(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_safety_checks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `justification` after provisioning.\nThe reason / justification for this quota preference."]
    pub fn justification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.justification", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the quota preference. Required except in the CREATE requests."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the quota preference. Allowed parents are \"projects/[project-id / number]\" or \"folders/[folder-id / number]\" or \"organizations/[org-id / number]\"."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_id` after provisioning.\nThe id of the quota to which the quota preference is applied. A quota id is unique in the service.\nExample: 'CPUS-per-project-region'."]
    pub fn quota_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIs the quota preference pending Google Cloud approval and fulfillment."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe name of the service to which the quota preference is applied."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nUpdate time stamp.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: '2014-10-02T15:01:23Z' and '2014-10-02T15:01:23.045123456Z'."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_config` after provisioning.\n"]
    pub fn quota_config(&self) -> ListRef<CloudQuotasQuotaPreferenceQuotaConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.quota_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudQuotasQuotaPreferenceTimeoutsElRef {
        CloudQuotasQuotaPreferenceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CloudQuotasQuotaPreference {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudQuotasQuotaPreference {}
impl ToListMappable for CloudQuotasQuotaPreference {
    type O = ListRef<CloudQuotasQuotaPreferenceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudQuotasQuotaPreference_ {
    fn extract_resource_type(&self) -> String {
        "google_cloud_quotas_quota_preference".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudQuotasQuotaPreference {
    pub tf_id: String,
}
impl BuildCloudQuotasQuotaPreference {
    pub fn build(self, stack: &mut Stack) -> CloudQuotasQuotaPreference {
        let out = CloudQuotasQuotaPreference(Rc::new(CloudQuotasQuotaPreference_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudQuotasQuotaPreferenceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                contact_email: core::default::Default::default(),
                dimensions: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_safety_checks: core::default::Default::default(),
                justification: core::default::Default::default(),
                name: core::default::Default::default(),
                parent: core::default::Default::default(),
                quota_id: core::default::Default::default(),
                service: core::default::Default::default(),
                quota_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudQuotasQuotaPreferenceRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudQuotasQuotaPreferenceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudQuotasQuotaPreferenceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contact_email` after provisioning.\nAn email address that can be used for quota related communication between the Google Cloud and the user in case the Google Cloud needs further information to make a decision on whether the user preferred quota can be granted.\n\nThe Google account for the email address must have quota update permission for the project, folder or organization this quota preference is for."]
    pub fn contact_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contact_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time stamp.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: '2014-10-02T15:01:23Z' and '2014-10-02T15:01:23.045123456Z'."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\nThe dimensions that this quota preference applies to. The key of the map entry is the name of a dimension, such as \"region\", \"zone\", \"network_id\", and the value of the map entry is the dimension value. If a dimension is missing from the map of dimensions, the quota preference applies to all the dimension values except for those that have other quota preferences configured for the specific value.\n\nNOTE: QuotaPreferences can only be applied across all values of \"user\" and \"resource\" dimension. Do not set values for \"user\" or \"resource\" in the dimension map.\n\nExample: '{\"provider\": \"Foo Inc\"}' where \"provider\" is a service specific dimension."]
    pub fn dimensions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.dimensions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe current etag of the quota preference. If an etag is provided on update and does not match the current server's etag of the quota preference, the request will be blocked and an ABORTED error will be returned. See https://google.aip.dev/134#etags for more details on etags."]
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
    #[doc = "Get a reference to the value of field `ignore_safety_checks` after provisioning.\nThe list of quota safety checks to be ignored. Default value: \"QUOTA_SAFETY_CHECK_UNSPECIFIED\" Possible values: [\"QUOTA_SAFETY_CHECK_UNSPECIFIED\", \"QUOTA_DECREASE_BELOW_USAGE\", \"QUOTA_DECREASE_PERCENTAGE_TOO_HIGH\"]"]
    pub fn ignore_safety_checks(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_safety_checks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `justification` after provisioning.\nThe reason / justification for this quota preference."]
    pub fn justification(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.justification", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the quota preference. Required except in the CREATE requests."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the quota preference. Allowed parents are \"projects/[project-id / number]\" or \"folders/[folder-id / number]\" or \"organizations/[org-id / number]\"."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_id` after provisioning.\nThe id of the quota to which the quota preference is applied. A quota id is unique in the service.\nExample: 'CPUS-per-project-region'."]
    pub fn quota_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nIs the quota preference pending Google Cloud approval and fulfillment."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe name of the service to which the quota preference is applied."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nUpdate time stamp.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: '2014-10-02T15:01:23Z' and '2014-10-02T15:01:23.045123456Z'."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_config` after provisioning.\n"]
    pub fn quota_config(&self) -> ListRef<CloudQuotasQuotaPreferenceQuotaConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.quota_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudQuotasQuotaPreferenceTimeoutsElRef {
        CloudQuotasQuotaPreferenceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudQuotasQuotaPreferenceQuotaConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    preferred_value: PrimField<String>,
}
impl CloudQuotasQuotaPreferenceQuotaConfigEl {
    #[doc = "Set the field `annotations`.\nThe annotations map for clients to store small amounts of arbitrary data. Do not put PII or other sensitive information here. See https://google.aip.dev/128#annotations.\n\nAn object containing a list of \"key: value\" pairs. Example: '{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }'."]
    pub fn set_annotations(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.annotations = Some(v.into());
        self
    }
}
impl ToListMappable for CloudQuotasQuotaPreferenceQuotaConfigEl {
    type O = BlockAssignable<CloudQuotasQuotaPreferenceQuotaConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudQuotasQuotaPreferenceQuotaConfigEl {
    #[doc = "The preferred value. Must be greater than or equal to -1. If set to -1, it means the value is \"unlimited\"."]
    pub preferred_value: PrimField<String>,
}
impl BuildCloudQuotasQuotaPreferenceQuotaConfigEl {
    pub fn build(self) -> CloudQuotasQuotaPreferenceQuotaConfigEl {
        CloudQuotasQuotaPreferenceQuotaConfigEl {
            annotations: core::default::Default::default(),
            preferred_value: self.preferred_value,
        }
    }
}
pub struct CloudQuotasQuotaPreferenceQuotaConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudQuotasQuotaPreferenceQuotaConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudQuotasQuotaPreferenceQuotaConfigElRef {
        CloudQuotasQuotaPreferenceQuotaConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudQuotasQuotaPreferenceQuotaConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nThe annotations map for clients to store small amounts of arbitrary data. Do not put PII or other sensitive information here. See https://google.aip.dev/128#annotations.\n\nAn object containing a list of \"key: value\" pairs. Example: '{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }'."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }
    #[doc = "Get a reference to the value of field `granted_value` after provisioning.\nGranted quota value."]
    pub fn granted_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.granted_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `preferred_value` after provisioning.\nThe preferred value. Must be greater than or equal to -1. If set to -1, it means the value is \"unlimited\"."]
    pub fn preferred_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preferred_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_origin` after provisioning.\nThe origin of the quota preference request."]
    pub fn request_origin(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_origin", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state_detail` after provisioning.\nOptional details about the state of this quota preference."]
    pub fn state_detail(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state_detail", self.base))
    }
    #[doc = "Get a reference to the value of field `trace_id` after provisioning.\nThe trace id that the Google Cloud uses to provision the requested quota. This trace id may be used by the client to contact Cloud support to track the state of a quota preference request. The trace id is only produced for increase requests and is unique for each request. The quota decrease requests do not have a trace id."]
    pub fn trace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.trace_id", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudQuotasQuotaPreferenceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudQuotasQuotaPreferenceTimeoutsEl {
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
impl ToListMappable for CloudQuotasQuotaPreferenceTimeoutsEl {
    type O = BlockAssignable<CloudQuotasQuotaPreferenceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudQuotasQuotaPreferenceTimeoutsEl {}
impl BuildCloudQuotasQuotaPreferenceTimeoutsEl {
    pub fn build(self) -> CloudQuotasQuotaPreferenceTimeoutsEl {
        CloudQuotasQuotaPreferenceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudQuotasQuotaPreferenceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudQuotasQuotaPreferenceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudQuotasQuotaPreferenceTimeoutsElRef {
        CloudQuotasQuotaPreferenceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudQuotasQuotaPreferenceTimeoutsElRef {
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
struct CloudQuotasQuotaPreferenceDynamic {
    quota_config: Option<DynamicBlock<CloudQuotasQuotaPreferenceQuotaConfigEl>>,
}
