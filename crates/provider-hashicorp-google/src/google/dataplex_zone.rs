use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataplexZoneData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    lake: PrimField<String>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discovery_spec: Option<Vec<DataplexZoneDiscoverySpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_spec: Option<Vec<DataplexZoneResourceSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataplexZoneTimeoutsEl>,
    dynamic: DataplexZoneDynamic,
}
struct DataplexZone_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataplexZoneData>,
}
#[derive(Clone)]
pub struct DataplexZone(Rc<DataplexZone_>);
impl DataplexZone {
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
    #[doc = "Set the field `description`.\nOptional. Description of the zone."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nOptional. User friendly display name."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional. User defined labels for the zone.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `discovery_spec`.\n"]
    pub fn set_discovery_spec(
        self,
        v: impl Into<BlockAssignable<DataplexZoneDiscoverySpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().discovery_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.discovery_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_spec`.\n"]
    pub fn set_resource_spec(
        self,
        v: impl Into<BlockAssignable<DataplexZoneResourceSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataplexZoneTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `asset_status` after provisioning.\nOutput only. Aggregated status of the underlying assets of the zone."]
    pub fn asset_status(&self) -> ListRef<DataplexZoneAssetStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.asset_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the zone was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the zone."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOptional. User friendly display name."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. User defined labels for the zone.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lake` after provisioning.\nThe lake for the resource"]
    pub fn lake(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lake", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the zone."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. Current state of the zone. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nRequired. Immutable. The type of the zone. Possible values: TYPE_UNSPECIFIED, RAW, CURATED"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. System generated globally unique ID for the zone. This ID will be different if the zone is deleted and re-created with the same name."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the zone was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_spec` after provisioning.\n"]
    pub fn discovery_spec(&self) -> ListRef<DataplexZoneDiscoverySpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.discovery_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_spec` after provisioning.\n"]
    pub fn resource_spec(&self) -> ListRef<DataplexZoneResourceSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexZoneTimeoutsElRef {
        DataplexZoneTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataplexZone {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataplexZone {}
impl ToListMappable for DataplexZone {
    type O = ListRef<DataplexZoneRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataplexZone_ {
    fn extract_resource_type(&self) -> String {
        "google_dataplex_zone".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataplexZone {
    pub tf_id: String,
    #[doc = "The lake for the resource"]
    pub lake: PrimField<String>,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "The name of the zone."]
    pub name: PrimField<String>,
    #[doc = "Required. Immutable. The type of the zone. Possible values: TYPE_UNSPECIFIED, RAW, CURATED"]
    pub type_: PrimField<String>,
}
impl BuildDataplexZone {
    pub fn build(self, stack: &mut Stack) -> DataplexZone {
        let out = DataplexZone(Rc::new(DataplexZone_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataplexZoneData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                lake: self.lake,
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                type_: self.type_,
                discovery_spec: core::default::Default::default(),
                resource_spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataplexZoneRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexZoneRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataplexZoneRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `asset_status` after provisioning.\nOutput only. Aggregated status of the underlying assets of the zone."]
    pub fn asset_status(&self) -> ListRef<DataplexZoneAssetStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.asset_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the zone was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the zone."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOptional. User friendly display name."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. User defined labels for the zone.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lake` after provisioning.\nThe lake for the resource"]
    pub fn lake(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lake", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the zone."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. Current state of the zone. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nRequired. Immutable. The type of the zone. Possible values: TYPE_UNSPECIFIED, RAW, CURATED"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. System generated globally unique ID for the zone. This ID will be different if the zone is deleted and re-created with the same name."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the zone was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_spec` after provisioning.\n"]
    pub fn discovery_spec(&self) -> ListRef<DataplexZoneDiscoverySpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.discovery_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_spec` after provisioning.\n"]
    pub fn resource_spec(&self) -> ListRef<DataplexZoneResourceSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexZoneTimeoutsElRef {
        DataplexZoneTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexZoneAssetStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    active_assets: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_policy_applying_assets: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl DataplexZoneAssetStatusEl {
    #[doc = "Set the field `active_assets`.\n"]
    pub fn set_active_assets(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.active_assets = Some(v.into());
        self
    }
    #[doc = "Set the field `security_policy_applying_assets`.\n"]
    pub fn set_security_policy_applying_assets(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.security_policy_applying_assets = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexZoneAssetStatusEl {
    type O = BlockAssignable<DataplexZoneAssetStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexZoneAssetStatusEl {}
impl BuildDataplexZoneAssetStatusEl {
    pub fn build(self) -> DataplexZoneAssetStatusEl {
        DataplexZoneAssetStatusEl {
            active_assets: core::default::Default::default(),
            security_policy_applying_assets: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct DataplexZoneAssetStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexZoneAssetStatusElRef {
    fn new(shared: StackShared, base: String) -> DataplexZoneAssetStatusElRef {
        DataplexZoneAssetStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexZoneAssetStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `active_assets` after provisioning.\n"]
    pub fn active_assets(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active_assets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `security_policy_applying_assets` after provisioning.\n"]
    pub fn security_policy_applying_assets(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_policy_applying_assets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexZoneDiscoverySpecElCsvOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_type_inference: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_rows: Option<PrimField<f64>>,
}
impl DataplexZoneDiscoverySpecElCsvOptionsEl {
    #[doc = "Set the field `delimiter`.\nOptional. The delimiter being used to separate values. This defaults to ','."]
    pub fn set_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delimiter = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_type_inference`.\nOptional. Whether to disable the inference of data type for CSV data. If true, all columns will be registered as strings."]
    pub fn set_disable_type_inference(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_type_inference = Some(v.into());
        self
    }
    #[doc = "Set the field `encoding`.\nOptional. The character encoding of the data. The default is UTF-8."]
    pub fn set_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding = Some(v.into());
        self
    }
    #[doc = "Set the field `header_rows`.\nOptional. The number of rows to interpret as header rows that should be skipped when reading data rows."]
    pub fn set_header_rows(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.header_rows = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexZoneDiscoverySpecElCsvOptionsEl {
    type O = BlockAssignable<DataplexZoneDiscoverySpecElCsvOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexZoneDiscoverySpecElCsvOptionsEl {}
impl BuildDataplexZoneDiscoverySpecElCsvOptionsEl {
    pub fn build(self) -> DataplexZoneDiscoverySpecElCsvOptionsEl {
        DataplexZoneDiscoverySpecElCsvOptionsEl {
            delimiter: core::default::Default::default(),
            disable_type_inference: core::default::Default::default(),
            encoding: core::default::Default::default(),
            header_rows: core::default::Default::default(),
        }
    }
}
pub struct DataplexZoneDiscoverySpecElCsvOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexZoneDiscoverySpecElCsvOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataplexZoneDiscoverySpecElCsvOptionsElRef {
        DataplexZoneDiscoverySpecElCsvOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexZoneDiscoverySpecElCsvOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delimiter` after provisioning.\nOptional. The delimiter being used to separate values. This defaults to ','."]
    pub fn delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }
    #[doc = "Get a reference to the value of field `disable_type_inference` after provisioning.\nOptional. Whether to disable the inference of data type for CSV data. If true, all columns will be registered as strings."]
    pub fn disable_type_inference(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_type_inference", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encoding` after provisioning.\nOptional. The character encoding of the data. The default is UTF-8."]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }
    #[doc = "Get a reference to the value of field `header_rows` after provisioning.\nOptional. The number of rows to interpret as header rows that should be skipped when reading data rows."]
    pub fn header_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_rows", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexZoneDiscoverySpecElJsonOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_type_inference: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
}
impl DataplexZoneDiscoverySpecElJsonOptionsEl {
    #[doc = "Set the field `disable_type_inference`.\nOptional. Whether to disable the inference of data type for Json data. If true, all columns will be registered as their primitive types (strings, number or boolean)."]
    pub fn set_disable_type_inference(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_type_inference = Some(v.into());
        self
    }
    #[doc = "Set the field `encoding`.\nOptional. The character encoding of the data. The default is UTF-8."]
    pub fn set_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexZoneDiscoverySpecElJsonOptionsEl {
    type O = BlockAssignable<DataplexZoneDiscoverySpecElJsonOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexZoneDiscoverySpecElJsonOptionsEl {}
impl BuildDataplexZoneDiscoverySpecElJsonOptionsEl {
    pub fn build(self) -> DataplexZoneDiscoverySpecElJsonOptionsEl {
        DataplexZoneDiscoverySpecElJsonOptionsEl {
            disable_type_inference: core::default::Default::default(),
            encoding: core::default::Default::default(),
        }
    }
}
pub struct DataplexZoneDiscoverySpecElJsonOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexZoneDiscoverySpecElJsonOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataplexZoneDiscoverySpecElJsonOptionsElRef {
        DataplexZoneDiscoverySpecElJsonOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexZoneDiscoverySpecElJsonOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_type_inference` after provisioning.\nOptional. Whether to disable the inference of data type for Json data. If true, all columns will be registered as their primitive types (strings, number or boolean)."]
    pub fn disable_type_inference(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_type_inference", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encoding` after provisioning.\nOptional. The character encoding of the data. The default is UTF-8."]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataplexZoneDiscoverySpecElDynamic {
    csv_options: Option<DynamicBlock<DataplexZoneDiscoverySpecElCsvOptionsEl>>,
    json_options: Option<DynamicBlock<DataplexZoneDiscoverySpecElJsonOptionsEl>>,
}
#[derive(Serialize)]
pub struct DataplexZoneDiscoverySpecEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_patterns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_patterns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_options: Option<Vec<DataplexZoneDiscoverySpecElCsvOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_options: Option<Vec<DataplexZoneDiscoverySpecElJsonOptionsEl>>,
    dynamic: DataplexZoneDiscoverySpecElDynamic,
}
impl DataplexZoneDiscoverySpecEl {
    #[doc = "Set the field `exclude_patterns`.\nOptional. The list of patterns to apply for selecting data to exclude during discovery. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names."]
    pub fn set_exclude_patterns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_patterns = Some(v.into());
        self
    }
    #[doc = "Set the field `include_patterns`.\nOptional. The list of patterns to apply for selecting data to include during discovery if only a subset of the data should considered. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names."]
    pub fn set_include_patterns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_patterns = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule`.\nOptional. Cron schedule (https://en.wikipedia.org/wiki/Cron) for running discovery periodically. Successive discovery runs must be scheduled at least 60 minutes apart. The default value is to run discovery every 60 minutes. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: \"CRON_TZ=${IANA_TIME_ZONE}\" or TZ=${IANA_TIME_ZONE}\". The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, \"CRON_TZ=America/New_York 1 * * * *\", or \"TZ=America/New_York 1 * * * *\"."]
    pub fn set_schedule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `csv_options`.\n"]
    pub fn set_csv_options(
        mut self,
        v: impl Into<BlockAssignable<DataplexZoneDiscoverySpecElCsvOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.csv_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.csv_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `json_options`.\n"]
    pub fn set_json_options(
        mut self,
        v: impl Into<BlockAssignable<DataplexZoneDiscoverySpecElJsonOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.json_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.json_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataplexZoneDiscoverySpecEl {
    type O = BlockAssignable<DataplexZoneDiscoverySpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexZoneDiscoverySpecEl {
    #[doc = "Required. Whether discovery is enabled."]
    pub enabled: PrimField<bool>,
}
impl BuildDataplexZoneDiscoverySpecEl {
    pub fn build(self) -> DataplexZoneDiscoverySpecEl {
        DataplexZoneDiscoverySpecEl {
            enabled: self.enabled,
            exclude_patterns: core::default::Default::default(),
            include_patterns: core::default::Default::default(),
            schedule: core::default::Default::default(),
            csv_options: core::default::Default::default(),
            json_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataplexZoneDiscoverySpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexZoneDiscoverySpecElRef {
    fn new(shared: StackShared, base: String) -> DataplexZoneDiscoverySpecElRef {
        DataplexZoneDiscoverySpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexZoneDiscoverySpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nRequired. Whether discovery is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `exclude_patterns` after provisioning.\nOptional. The list of patterns to apply for selecting data to exclude during discovery. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names."]
    pub fn exclude_patterns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_patterns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_patterns` after provisioning.\nOptional. The list of patterns to apply for selecting data to include during discovery if only a subset of the data should considered. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names."]
    pub fn include_patterns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_patterns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\nOptional. Cron schedule (https://en.wikipedia.org/wiki/Cron) for running discovery periodically. Successive discovery runs must be scheduled at least 60 minutes apart. The default value is to run discovery every 60 minutes. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: \"CRON_TZ=${IANA_TIME_ZONE}\" or TZ=${IANA_TIME_ZONE}\". The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, \"CRON_TZ=America/New_York 1 * * * *\", or \"TZ=America/New_York 1 * * * *\"."]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
    #[doc = "Get a reference to the value of field `csv_options` after provisioning.\n"]
    pub fn csv_options(&self) -> ListRef<DataplexZoneDiscoverySpecElCsvOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.csv_options", self.base))
    }
    #[doc = "Get a reference to the value of field `json_options` after provisioning.\n"]
    pub fn json_options(&self) -> ListRef<DataplexZoneDiscoverySpecElJsonOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.json_options", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexZoneResourceSpecEl {
    location_type: PrimField<String>,
}
impl DataplexZoneResourceSpecEl {}
impl ToListMappable for DataplexZoneResourceSpecEl {
    type O = BlockAssignable<DataplexZoneResourceSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexZoneResourceSpecEl {
    #[doc = "Required. Immutable. The location type of the resources that are allowed to be attached to the assets within this zone. Possible values: LOCATION_TYPE_UNSPECIFIED, SINGLE_REGION, MULTI_REGION"]
    pub location_type: PrimField<String>,
}
impl BuildDataplexZoneResourceSpecEl {
    pub fn build(self) -> DataplexZoneResourceSpecEl {
        DataplexZoneResourceSpecEl {
            location_type: self.location_type,
        }
    }
}
pub struct DataplexZoneResourceSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexZoneResourceSpecElRef {
    fn new(shared: StackShared, base: String) -> DataplexZoneResourceSpecElRef {
        DataplexZoneResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexZoneResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location_type` after provisioning.\nRequired. Immutable. The location type of the resources that are allowed to be attached to the assets within this zone. Possible values: LOCATION_TYPE_UNSPECIFIED, SINGLE_REGION, MULTI_REGION"]
    pub fn location_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexZoneTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataplexZoneTimeoutsEl {
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
impl ToListMappable for DataplexZoneTimeoutsEl {
    type O = BlockAssignable<DataplexZoneTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexZoneTimeoutsEl {}
impl BuildDataplexZoneTimeoutsEl {
    pub fn build(self) -> DataplexZoneTimeoutsEl {
        DataplexZoneTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataplexZoneTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexZoneTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataplexZoneTimeoutsElRef {
        DataplexZoneTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexZoneTimeoutsElRef {
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
struct DataplexZoneDynamic {
    discovery_spec: Option<DynamicBlock<DataplexZoneDiscoverySpecEl>>,
    resource_spec: Option<DynamicBlock<DataplexZoneResourceSpecEl>>,
}
