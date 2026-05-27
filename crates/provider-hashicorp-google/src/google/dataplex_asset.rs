use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataplexAssetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    dataplex_zone: PrimField<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    discovery_spec: Option<Vec<DataplexAssetDiscoverySpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_spec: Option<Vec<DataplexAssetResourceSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataplexAssetTimeoutsEl>,
    dynamic: DataplexAssetDynamic,
}
struct DataplexAsset_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataplexAssetData>,
}
#[derive(Clone)]
pub struct DataplexAsset(Rc<DataplexAsset_>);
impl DataplexAsset {
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
    #[doc = "Set the field `description`.\nOptional. Description of the asset."]
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
    #[doc = "Set the field `labels`.\nOptional. User defined labels for the asset.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
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
        v: impl Into<BlockAssignable<DataplexAssetDiscoverySpecEl>>,
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
        v: impl Into<BlockAssignable<DataplexAssetResourceSpecEl>>,
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
    pub fn set_timeouts(self, v: impl Into<DataplexAssetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the asset was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataplex_zone` after provisioning.\nThe zone for the resource"]
    pub fn dataplex_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataplex_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the asset."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_status` after provisioning.\nOutput only. Status of the discovery feature applied to data referenced by this asset."]
    pub fn discovery_status(&self) -> ListRef<DataplexAssetDiscoveryStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.discovery_status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. User defined labels for the asset.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the asset."]
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
    #[doc = "Get a reference to the value of field `resource_status` after provisioning.\nOutput only. Status of the resource referenced by this asset."]
    pub fn resource_status(&self) -> ListRef<DataplexAssetResourceStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_status` after provisioning.\nOutput only. Status of the security policy applied to resource referenced by this asset."]
    pub fn security_status(&self) -> ListRef<DataplexAssetSecurityStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. Current state of the asset. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED"]
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
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. System generated globally unique ID for the asset. This ID will be different if the asset is deleted and re-created with the same name."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the asset was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_spec` after provisioning.\n"]
    pub fn discovery_spec(&self) -> ListRef<DataplexAssetDiscoverySpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.discovery_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_spec` after provisioning.\n"]
    pub fn resource_spec(&self) -> ListRef<DataplexAssetResourceSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexAssetTimeoutsElRef {
        DataplexAssetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataplexAsset {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataplexAsset {}
impl ToListMappable for DataplexAsset {
    type O = ListRef<DataplexAssetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataplexAsset_ {
    fn extract_resource_type(&self) -> String {
        "google_dataplex_asset".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataplexAsset {
    pub tf_id: String,
    #[doc = "The zone for the resource"]
    pub dataplex_zone: PrimField<String>,
    #[doc = "The lake for the resource"]
    pub lake: PrimField<String>,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "The name of the asset."]
    pub name: PrimField<String>,
}
impl BuildDataplexAsset {
    pub fn build(self, stack: &mut Stack) -> DataplexAsset {
        let out = DataplexAsset(Rc::new(DataplexAsset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataplexAssetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                dataplex_zone: self.dataplex_zone,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                lake: self.lake,
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
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
pub struct DataplexAssetRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexAssetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataplexAssetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the asset was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataplex_zone` after provisioning.\nThe zone for the resource"]
    pub fn dataplex_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataplex_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the asset."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_status` after provisioning.\nOutput only. Status of the discovery feature applied to data referenced by this asset."]
    pub fn discovery_status(&self) -> ListRef<DataplexAssetDiscoveryStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.discovery_status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. User defined labels for the asset.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the asset."]
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
    #[doc = "Get a reference to the value of field `resource_status` after provisioning.\nOutput only. Status of the resource referenced by this asset."]
    pub fn resource_status(&self) -> ListRef<DataplexAssetResourceStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_status` after provisioning.\nOutput only. Status of the security policy applied to resource referenced by this asset."]
    pub fn security_status(&self) -> ListRef<DataplexAssetSecurityStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. Current state of the asset. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED"]
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
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. System generated globally unique ID for the asset. This ID will be different if the asset is deleted and re-created with the same name."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the asset was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovery_spec` after provisioning.\n"]
    pub fn discovery_spec(&self) -> ListRef<DataplexAssetDiscoverySpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.discovery_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_spec` after provisioning.\n"]
    pub fn resource_spec(&self) -> ListRef<DataplexAssetResourceSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexAssetTimeoutsElRef {
        DataplexAssetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataplexAssetDiscoveryStatusElStatsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_items: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filesets: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tables: Option<PrimField<f64>>,
}
impl DataplexAssetDiscoveryStatusElStatsEl {
    #[doc = "Set the field `data_items`.\n"]
    pub fn set_data_items(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.data_items = Some(v.into());
        self
    }
    #[doc = "Set the field `data_size`.\n"]
    pub fn set_data_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.data_size = Some(v.into());
        self
    }
    #[doc = "Set the field `filesets`.\n"]
    pub fn set_filesets(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.filesets = Some(v.into());
        self
    }
    #[doc = "Set the field `tables`.\n"]
    pub fn set_tables(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.tables = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexAssetDiscoveryStatusElStatsEl {
    type O = BlockAssignable<DataplexAssetDiscoveryStatusElStatsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexAssetDiscoveryStatusElStatsEl {}
impl BuildDataplexAssetDiscoveryStatusElStatsEl {
    pub fn build(self) -> DataplexAssetDiscoveryStatusElStatsEl {
        DataplexAssetDiscoveryStatusElStatsEl {
            data_items: core::default::Default::default(),
            data_size: core::default::Default::default(),
            filesets: core::default::Default::default(),
            tables: core::default::Default::default(),
        }
    }
}
pub struct DataplexAssetDiscoveryStatusElStatsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexAssetDiscoveryStatusElStatsElRef {
    fn new(shared: StackShared, base: String) -> DataplexAssetDiscoveryStatusElStatsElRef {
        DataplexAssetDiscoveryStatusElStatsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexAssetDiscoveryStatusElStatsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_items` after provisioning.\n"]
    pub fn data_items(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_items", self.base))
    }
    #[doc = "Get a reference to the value of field `data_size` after provisioning.\n"]
    pub fn data_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_size", self.base))
    }
    #[doc = "Get a reference to the value of field `filesets` after provisioning.\n"]
    pub fn filesets(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.filesets", self.base))
    }
    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tables", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexAssetDiscoveryStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    last_run_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_run_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stats: Option<ListField<DataplexAssetDiscoveryStatusElStatsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl DataplexAssetDiscoveryStatusEl {
    #[doc = "Set the field `last_run_duration`.\n"]
    pub fn set_last_run_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_run_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `last_run_time`.\n"]
    pub fn set_last_run_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_run_time = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `stats`.\n"]
    pub fn set_stats(
        mut self,
        v: impl Into<ListField<DataplexAssetDiscoveryStatusElStatsEl>>,
    ) -> Self {
        self.stats = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexAssetDiscoveryStatusEl {
    type O = BlockAssignable<DataplexAssetDiscoveryStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexAssetDiscoveryStatusEl {}
impl BuildDataplexAssetDiscoveryStatusEl {
    pub fn build(self) -> DataplexAssetDiscoveryStatusEl {
        DataplexAssetDiscoveryStatusEl {
            last_run_duration: core::default::Default::default(),
            last_run_time: core::default::Default::default(),
            message: core::default::Default::default(),
            state: core::default::Default::default(),
            stats: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct DataplexAssetDiscoveryStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexAssetDiscoveryStatusElRef {
    fn new(shared: StackShared, base: String) -> DataplexAssetDiscoveryStatusElRef {
        DataplexAssetDiscoveryStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexAssetDiscoveryStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `last_run_duration` after provisioning.\n"]
    pub fn last_run_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_run_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `last_run_time` after provisioning.\n"]
    pub fn last_run_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_run_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `stats` after provisioning.\n"]
    pub fn stats(&self) -> ListRef<DataplexAssetDiscoveryStatusElStatsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stats", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexAssetResourceStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl DataplexAssetResourceStatusEl {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexAssetResourceStatusEl {
    type O = BlockAssignable<DataplexAssetResourceStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexAssetResourceStatusEl {}
impl BuildDataplexAssetResourceStatusEl {
    pub fn build(self) -> DataplexAssetResourceStatusEl {
        DataplexAssetResourceStatusEl {
            message: core::default::Default::default(),
            state: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct DataplexAssetResourceStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexAssetResourceStatusElRef {
    fn new(shared: StackShared, base: String) -> DataplexAssetResourceStatusElRef {
        DataplexAssetResourceStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexAssetResourceStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexAssetSecurityStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl DataplexAssetSecurityStatusEl {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexAssetSecurityStatusEl {
    type O = BlockAssignable<DataplexAssetSecurityStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexAssetSecurityStatusEl {}
impl BuildDataplexAssetSecurityStatusEl {
    pub fn build(self) -> DataplexAssetSecurityStatusEl {
        DataplexAssetSecurityStatusEl {
            message: core::default::Default::default(),
            state: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct DataplexAssetSecurityStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexAssetSecurityStatusElRef {
    fn new(shared: StackShared, base: String) -> DataplexAssetSecurityStatusElRef {
        DataplexAssetSecurityStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexAssetSecurityStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexAssetDiscoverySpecElCsvOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_type_inference: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_rows: Option<PrimField<f64>>,
}
impl DataplexAssetDiscoverySpecElCsvOptionsEl {
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
impl ToListMappable for DataplexAssetDiscoverySpecElCsvOptionsEl {
    type O = BlockAssignable<DataplexAssetDiscoverySpecElCsvOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexAssetDiscoverySpecElCsvOptionsEl {}
impl BuildDataplexAssetDiscoverySpecElCsvOptionsEl {
    pub fn build(self) -> DataplexAssetDiscoverySpecElCsvOptionsEl {
        DataplexAssetDiscoverySpecElCsvOptionsEl {
            delimiter: core::default::Default::default(),
            disable_type_inference: core::default::Default::default(),
            encoding: core::default::Default::default(),
            header_rows: core::default::Default::default(),
        }
    }
}
pub struct DataplexAssetDiscoverySpecElCsvOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexAssetDiscoverySpecElCsvOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataplexAssetDiscoverySpecElCsvOptionsElRef {
        DataplexAssetDiscoverySpecElCsvOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexAssetDiscoverySpecElCsvOptionsElRef {
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
pub struct DataplexAssetDiscoverySpecElJsonOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_type_inference: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
}
impl DataplexAssetDiscoverySpecElJsonOptionsEl {
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
impl ToListMappable for DataplexAssetDiscoverySpecElJsonOptionsEl {
    type O = BlockAssignable<DataplexAssetDiscoverySpecElJsonOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexAssetDiscoverySpecElJsonOptionsEl {}
impl BuildDataplexAssetDiscoverySpecElJsonOptionsEl {
    pub fn build(self) -> DataplexAssetDiscoverySpecElJsonOptionsEl {
        DataplexAssetDiscoverySpecElJsonOptionsEl {
            disable_type_inference: core::default::Default::default(),
            encoding: core::default::Default::default(),
        }
    }
}
pub struct DataplexAssetDiscoverySpecElJsonOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexAssetDiscoverySpecElJsonOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataplexAssetDiscoverySpecElJsonOptionsElRef {
        DataplexAssetDiscoverySpecElJsonOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexAssetDiscoverySpecElJsonOptionsElRef {
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
struct DataplexAssetDiscoverySpecElDynamic {
    csv_options: Option<DynamicBlock<DataplexAssetDiscoverySpecElCsvOptionsEl>>,
    json_options: Option<DynamicBlock<DataplexAssetDiscoverySpecElJsonOptionsEl>>,
}
#[derive(Serialize)]
pub struct DataplexAssetDiscoverySpecEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_patterns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_patterns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_options: Option<Vec<DataplexAssetDiscoverySpecElCsvOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_options: Option<Vec<DataplexAssetDiscoverySpecElJsonOptionsEl>>,
    dynamic: DataplexAssetDiscoverySpecElDynamic,
}
impl DataplexAssetDiscoverySpecEl {
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
        v: impl Into<BlockAssignable<DataplexAssetDiscoverySpecElCsvOptionsEl>>,
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
        v: impl Into<BlockAssignable<DataplexAssetDiscoverySpecElJsonOptionsEl>>,
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
impl ToListMappable for DataplexAssetDiscoverySpecEl {
    type O = BlockAssignable<DataplexAssetDiscoverySpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexAssetDiscoverySpecEl {
    #[doc = "Required. Whether discovery is enabled."]
    pub enabled: PrimField<bool>,
}
impl BuildDataplexAssetDiscoverySpecEl {
    pub fn build(self) -> DataplexAssetDiscoverySpecEl {
        DataplexAssetDiscoverySpecEl {
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
pub struct DataplexAssetDiscoverySpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexAssetDiscoverySpecElRef {
    fn new(shared: StackShared, base: String) -> DataplexAssetDiscoverySpecElRef {
        DataplexAssetDiscoverySpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexAssetDiscoverySpecElRef {
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
    pub fn csv_options(&self) -> ListRef<DataplexAssetDiscoverySpecElCsvOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.csv_options", self.base))
    }
    #[doc = "Get a reference to the value of field `json_options` after provisioning.\n"]
    pub fn json_options(&self) -> ListRef<DataplexAssetDiscoverySpecElJsonOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.json_options", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexAssetResourceSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_access_mode: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl DataplexAssetResourceSpecEl {
    #[doc = "Set the field `name`.\nImmutable. Relative name of the cloud resource that contains the data that is being managed within a lake. For example: `projects/{project_number}/buckets/{bucket_id}` `projects/{project_number}/datasets/{dataset_id}`"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `read_access_mode`.\nOptional. Determines how read permissions are handled for each asset and their associated tables. Only available to storage buckets assets. Possible values: DIRECT, MANAGED"]
    pub fn set_read_access_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read_access_mode = Some(v.into());
        self
    }
}
impl ToListMappable for DataplexAssetResourceSpecEl {
    type O = BlockAssignable<DataplexAssetResourceSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexAssetResourceSpecEl {
    #[doc = "Required. Immutable. Type of resource. Possible values: STORAGE_BUCKET, BIGQUERY_DATASET"]
    pub type_: PrimField<String>,
}
impl BuildDataplexAssetResourceSpecEl {
    pub fn build(self) -> DataplexAssetResourceSpecEl {
        DataplexAssetResourceSpecEl {
            name: core::default::Default::default(),
            read_access_mode: core::default::Default::default(),
            type_: self.type_,
        }
    }
}
pub struct DataplexAssetResourceSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexAssetResourceSpecElRef {
    fn new(shared: StackShared, base: String) -> DataplexAssetResourceSpecElRef {
        DataplexAssetResourceSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexAssetResourceSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nImmutable. Relative name of the cloud resource that contains the data that is being managed within a lake. For example: `projects/{project_number}/buckets/{bucket_id}` `projects/{project_number}/datasets/{dataset_id}`"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `read_access_mode` after provisioning.\nOptional. Determines how read permissions are handled for each asset and their associated tables. Only available to storage buckets assets. Possible values: DIRECT, MANAGED"]
    pub fn read_access_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_access_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nRequired. Immutable. Type of resource. Possible values: STORAGE_BUCKET, BIGQUERY_DATASET"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataplexAssetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataplexAssetTimeoutsEl {
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
impl ToListMappable for DataplexAssetTimeoutsEl {
    type O = BlockAssignable<DataplexAssetTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataplexAssetTimeoutsEl {}
impl BuildDataplexAssetTimeoutsEl {
    pub fn build(self) -> DataplexAssetTimeoutsEl {
        DataplexAssetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataplexAssetTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataplexAssetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataplexAssetTimeoutsElRef {
        DataplexAssetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataplexAssetTimeoutsElRef {
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
struct DataplexAssetDynamic {
    discovery_spec: Option<DynamicBlock<DataplexAssetDiscoverySpecEl>>,
    resource_spec: Option<DynamicBlock<DataplexAssetResourceSpecEl>>,
}
