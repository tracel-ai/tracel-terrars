use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DiscoveryEngineRecommendationEngineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    data_store_ids: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    display_name: PrimField<String>,
    engine_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    industry_vertical: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    common_config: Option<Vec<DiscoveryEngineRecommendationEngineCommonConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_recommendation_engine_config:
        Option<Vec<DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DiscoveryEngineRecommendationEngineTimeoutsEl>,
    dynamic: DiscoveryEngineRecommendationEngineDynamic,
}
struct DiscoveryEngineRecommendationEngine_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DiscoveryEngineRecommendationEngineData>,
}
#[derive(Clone)]
pub struct DiscoveryEngineRecommendationEngine(Rc<DiscoveryEngineRecommendationEngine_>);
impl DiscoveryEngineRecommendationEngine {
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
    #[doc = "Set the field `industry_vertical`.\nThe industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: If unspecified, default to GENERIC. Vertical on Engine has to match vertical of the DataStore liniked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\", \"MEDIA\"]"]
    pub fn set_industry_vertical(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().industry_vertical = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `common_config`.\n"]
    pub fn set_common_config(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineRecommendationEngineCommonConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().common_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.common_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `media_recommendation_engine_config`.\n"]
    pub fn set_media_recommendation_engine_config(
        self,
        v: impl Into<
            BlockAssignable<DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().media_recommendation_engine_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .media_recommendation_engine_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DiscoveryEngineRecommendationEngineTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp the Engine was created at."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_store_ids` after provisioning.\nThe data stores associated with this engine. For SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store."]
    pub fn data_store_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nUnique ID to use for Recommendation Engine."]
    pub fn engine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `industry_vertical` after provisioning.\nThe industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: If unspecified, default to GENERIC. Vertical on Engine has to match vertical of the DataStore liniked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\", \"MEDIA\"]"]
    pub fn industry_vertical(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.industry_vertical", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the recommendation engine. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection}/engines/{engine_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024 characters."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp the Engine was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `common_config` after provisioning.\n"]
    pub fn common_config(&self) -> ListRef<DiscoveryEngineRecommendationEngineCommonConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.common_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `media_recommendation_engine_config` after provisioning.\n"]
    pub fn media_recommendation_engine_config(
        &self,
    ) -> ListRef<DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.media_recommendation_engine_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineRecommendationEngineTimeoutsElRef {
        DiscoveryEngineRecommendationEngineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DiscoveryEngineRecommendationEngine {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DiscoveryEngineRecommendationEngine {}
impl ToListMappable for DiscoveryEngineRecommendationEngine {
    type O = ListRef<DiscoveryEngineRecommendationEngineRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DiscoveryEngineRecommendationEngine_ {
    fn extract_resource_type(&self) -> String {
        "google_discovery_engine_recommendation_engine".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDiscoveryEngineRecommendationEngine {
    pub tf_id: String,
    #[doc = "The data stores associated with this engine. For SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store."]
    pub data_store_ids: ListField<PrimField<String>>,
    #[doc = "Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub display_name: PrimField<String>,
    #[doc = "Unique ID to use for Recommendation Engine."]
    pub engine_id: PrimField<String>,
    #[doc = "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: PrimField<String>,
}
impl BuildDiscoveryEngineRecommendationEngine {
    pub fn build(self, stack: &mut Stack) -> DiscoveryEngineRecommendationEngine {
        let out =
            DiscoveryEngineRecommendationEngine(Rc::new(DiscoveryEngineRecommendationEngine_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DiscoveryEngineRecommendationEngineData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    data_store_ids: self.data_store_ids,
                    deletion_policy: core::default::Default::default(),
                    display_name: self.display_name,
                    engine_id: self.engine_id,
                    id: core::default::Default::default(),
                    industry_vertical: core::default::Default::default(),
                    location: self.location,
                    project: core::default::Default::default(),
                    common_config: core::default::Default::default(),
                    media_recommendation_engine_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DiscoveryEngineRecommendationEngineRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineRecommendationEngineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DiscoveryEngineRecommendationEngineRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp the Engine was created at."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_store_ids` after provisioning.\nThe data stores associated with this engine. For SOLUTION_TYPE_RECOMMENDATION type of engines, they can only associate with at most one data store."]
    pub fn data_store_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nUnique ID to use for Recommendation Engine."]
    pub fn engine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `industry_vertical` after provisioning.\nThe industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: If unspecified, default to GENERIC. Vertical on Engine has to match vertical of the DataStore liniked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\", \"MEDIA\"]"]
    pub fn industry_vertical(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.industry_vertical", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the recommendation engine. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection}/engines/{engine_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024 characters."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp the Engine was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `common_config` after provisioning.\n"]
    pub fn common_config(&self) -> ListRef<DiscoveryEngineRecommendationEngineCommonConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.common_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `media_recommendation_engine_config` after provisioning.\n"]
    pub fn media_recommendation_engine_config(
        &self,
    ) -> ListRef<DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.media_recommendation_engine_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineRecommendationEngineTimeoutsElRef {
        DiscoveryEngineRecommendationEngineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineRecommendationEngineCommonConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    company_name: Option<PrimField<String>>,
}
impl DiscoveryEngineRecommendationEngineCommonConfigEl {
    #[doc = "Set the field `company_name`.\nThe name of the company, business or entity that is associated with the engine. Setting this may help improve LLM related features.cd"]
    pub fn set_company_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.company_name = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineRecommendationEngineCommonConfigEl {
    type O = BlockAssignable<DiscoveryEngineRecommendationEngineCommonConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineRecommendationEngineCommonConfigEl {}
impl BuildDiscoveryEngineRecommendationEngineCommonConfigEl {
    pub fn build(self) -> DiscoveryEngineRecommendationEngineCommonConfigEl {
        DiscoveryEngineRecommendationEngineCommonConfigEl {
            company_name: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineRecommendationEngineCommonConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineRecommendationEngineCommonConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineRecommendationEngineCommonConfigElRef {
        DiscoveryEngineRecommendationEngineCommonConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineRecommendationEngineCommonConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `company_name` after provisioning.\nThe name of the company, business or entity that is associated with the engine. Setting this may help improve LLM related features.cd"]
    pub fn company_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.company_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElMostPopularConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    time_window_days: Option<PrimField<f64>>,
}
impl DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElMostPopularConfigEl { # [doc = "Set the field `time_window_days`.\nThe time window of which the engine is queried at training and\nprediction time. Positive integers only. The value translates to the\nlast X days of events. Currently required for the 'most-popular-items'\nengine."] pub fn set_time_window_days (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . time_window_days = Some (v . into ()) ; self } }
impl ToListMappable for DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElMostPopularConfigEl { type O = BlockAssignable < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElMostPopularConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElMostPopularConfigEl
{}
impl BuildDiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElMostPopularConfigEl { pub fn build (self) -> DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElMostPopularConfigEl { DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElMostPopularConfigEl { time_window_days : core :: default :: Default :: default () , } } }
pub struct DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElMostPopularConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElMostPopularConfigElRef { fn new (shared : StackShared , base : String) -> DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElMostPopularConfigElRef { DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElMostPopularConfigElRef { shared : shared , base : base . to_string () , } } }
impl DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElMostPopularConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `time_window_days` after provisioning.\nThe time window of which the engine is queried at training and\nprediction time. Positive integers only. The value translates to the\nlast X days of events. Currently required for the 'most-popular-items'\nengine."] pub fn time_window_days (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.time_window_days" , self . base)) } }
#[derive(Serialize)]
pub struct DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRecommendedForYouConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    context_event_type: Option<PrimField<String>>,
}
impl DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRecommendedForYouConfigEl { # [doc = "Set the field `context_event_type`.\nThe type of event with which the engine is queried at prediction time.\nIf set to 'generic', only 'view-item', 'media-play',and\n'media-complete' will be used as 'context-event' in engine training. If\nset to 'view-home-page', 'view-home-page' will also be used as\n'context-events' in addition to 'view-item', 'media-play', and\n'media-complete'. Currently supported for the 'recommended-for-you'\nengine. Currently supported values: 'view-home-page', 'generic'."] pub fn set_context_event_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . context_event_type = Some (v . into ()) ; self } }
impl ToListMappable for DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRecommendedForYouConfigEl { type O = BlockAssignable < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRecommendedForYouConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRecommendedForYouConfigEl
{}
impl BuildDiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRecommendedForYouConfigEl { pub fn build (self) -> DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRecommendedForYouConfigEl { DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRecommendedForYouConfigEl { context_event_type : core :: default :: Default :: default () , } } }
pub struct DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRecommendedForYouConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRecommendedForYouConfigElRef { fn new (shared : StackShared , base : String) -> DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRecommendedForYouConfigElRef { DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRecommendedForYouConfigElRef { shared : shared , base : base . to_string () , } } }
impl DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRecommendedForYouConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `context_event_type` after provisioning.\nThe type of event with which the engine is queried at prediction time.\nIf set to 'generic', only 'view-item', 'media-play',and\n'media-complete' will be used as 'context-event' in engine training. If\nset to 'view-home-page', 'view-home-page' will also be used as\n'context-events' in addition to 'view-item', 'media-play', and\n'media-complete'. Currently supported for the 'recommended-for-you'\nengine. Currently supported values: 'view-home-page', 'generic'."] pub fn context_event_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.context_event_type" , self . base)) } }
#[derive(Serialize, Default)]
struct DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElDynamic { most_popular_config : Option < DynamicBlock < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElMostPopularConfigEl >> , recommended_for_you_config : Option < DynamicBlock < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRecommendedForYouConfigEl >> , }
#[derive(Serialize)]
pub struct DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigEl { # [serde (skip_serializing_if = "Option::is_none")] most_popular_config : Option < Vec < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElMostPopularConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] recommended_for_you_config : Option < Vec < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRecommendedForYouConfigEl > > , dynamic : DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElDynamic , }
impl DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigEl {
    #[doc = "Set the field `most_popular_config`.\n"]
    pub fn set_most_popular_config(
        mut self,
        v : impl Into < BlockAssignable < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElMostPopularConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.most_popular_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.most_popular_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `recommended_for_you_config`.\n"]
    pub fn set_recommended_for_you_config(
        mut self,
        v : impl Into < BlockAssignable < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRecommendedForYouConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.recommended_for_you_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.recommended_for_you_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigEl
{
    type O = BlockAssignable<
        DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigEl
{}
impl
    BuildDiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigEl
{
    pub fn build(
        self,
    ) -> DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigEl
    {
        DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigEl {
            most_popular_config: core::default::Default::default(),
            recommended_for_you_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRef { fn new (shared : StackShared , base : String) -> DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRef { DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRef { shared : shared , base : base . to_string () , } } }
impl DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `most_popular_config` after provisioning.\n"]    pub fn most_popular_config (& self) -> ListRef < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElMostPopularConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.most_popular_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `recommended_for_you_config` after provisioning.\n"]    pub fn recommended_for_you_config (& self) -> ListRef < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRecommendedForYouConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.recommended_for_you_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElOptimizationObjectiveConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    target_field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_field_value_float: Option<PrimField<f64>>,
}
impl DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElOptimizationObjectiveConfigEl { # [doc = "Set the field `target_field`.\nThe name of the field to target. Currently supported values: 'watch-percentage', 'watch-time'."] pub fn set_target_field (mut self , v : impl Into < PrimField < String > >) -> Self { self . target_field = Some (v . into ()) ; self } # [doc = "Set the field `target_field_value_float`.\nThe threshold to be applied to the target (e.g., 0.5)."] pub fn set_target_field_value_float (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . target_field_value_float = Some (v . into ()) ; self } }
impl ToListMappable for DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElOptimizationObjectiveConfigEl { type O = BlockAssignable < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElOptimizationObjectiveConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElOptimizationObjectiveConfigEl
{}
impl BuildDiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElOptimizationObjectiveConfigEl { pub fn build (self) -> DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElOptimizationObjectiveConfigEl { DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElOptimizationObjectiveConfigEl { target_field : core :: default :: Default :: default () , target_field_value_float : core :: default :: Default :: default () , } } }
pub struct DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElOptimizationObjectiveConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElOptimizationObjectiveConfigElRef { fn new (shared : StackShared , base : String) -> DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElOptimizationObjectiveConfigElRef { DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElOptimizationObjectiveConfigElRef { shared : shared , base : base . to_string () , } } }
impl DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElOptimizationObjectiveConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `target_field` after provisioning.\nThe name of the field to target. Currently supported values: 'watch-percentage', 'watch-time'."] pub fn target_field (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.target_field" , self . base)) } # [doc = "Get a reference to the value of field `target_field_value_float` after provisioning.\nThe threshold to be applied to the target (e.g., 0.5)."] pub fn target_field_value_float (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.target_field_value_float" , self . base)) } }
#[derive(Serialize, Default)]
struct DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElDynamic { engine_features_config : Option < DynamicBlock < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigEl >> , optimization_objective_config : Option < DynamicBlock < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElOptimizationObjectiveConfigEl >> , }
#[derive(Serialize)]
pub struct DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigEl { # [serde (skip_serializing_if = "Option::is_none")] optimization_objective : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] training_state : Option < PrimField < String > > , # [serde (rename = "type" , skip_serializing_if = "Option::is_none")] type_ : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] engine_features_config : Option < Vec < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] optimization_objective_config : Option < Vec < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElOptimizationObjectiveConfigEl > > , dynamic : DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElDynamic , }
impl DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigEl {
    #[doc = "Set the field `optimization_objective`.\nThe optimization objective. e.g., 'cvr'.\nThis field together with MediaRecommendationEngineConfig.type describes\nengine metadata to use to control engine training and serving.\nCurrently supported values: 'ctr', 'cvr'.\nIf not specified, we choose default based on engine type. Default depends on type of recommendation:\n'recommended-for-you' => 'ctr'\n'others-you-may-like' => 'ctr'"]
    pub fn set_optimization_objective(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.optimization_objective = Some(v.into());
        self
    }
    #[doc = "Set the field `training_state`.\nThe training state that the engine is in (e.g. 'TRAINING' or 'PAUSED').\nSince part of the cost of running the service\nis frequency of training - this can be used to determine when to train\nengine in order to control cost. If not specified: the default value for\n'CreateEngine' method is 'TRAINING'. The default value for\n'UpdateEngine' method is to keep the state the same as before. Possible values: [\"PAUSED\", \"TRAINING\"]"]
    pub fn set_training_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.training_state = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThe type of engine. e.g., 'recommended-for-you'.\nThis field together with MediaRecommendationEngineConfig.optimizationObjective describes\nengine metadata to use to control engine training and serving.\nCurrently supported values: 'recommended-for-you', 'others-you-may-like',\n'more-like-this', 'most-popular-items'."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `engine_features_config`.\n"]
    pub fn set_engine_features_config(
        mut self,
        v : impl Into < BlockAssignable < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.engine_features_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.engine_features_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `optimization_objective_config`.\n"]
    pub fn set_optimization_objective_config(
        mut self,
        v : impl Into < BlockAssignable < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElOptimizationObjectiveConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.optimization_objective_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.optimization_objective_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigEl {
    type O = BlockAssignable<DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigEl {}
impl BuildDiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigEl {
    pub fn build(self) -> DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigEl {
        DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigEl {
            optimization_objective: core::default::Default::default(),
            training_state: core::default::Default::default(),
            type_: core::default::Default::default(),
            engine_features_config: core::default::Default::default(),
            optimization_objective_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElRef {
        DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `optimization_objective` after provisioning.\nThe optimization objective. e.g., 'cvr'.\nThis field together with MediaRecommendationEngineConfig.type describes\nengine metadata to use to control engine training and serving.\nCurrently supported values: 'ctr', 'cvr'.\nIf not specified, we choose default based on engine type. Default depends on type of recommendation:\n'recommended-for-you' => 'ctr'\n'others-you-may-like' => 'ctr'"]
    pub fn optimization_objective(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.optimization_objective", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `training_state` after provisioning.\nThe training state that the engine is in (e.g. 'TRAINING' or 'PAUSED').\nSince part of the cost of running the service\nis frequency of training - this can be used to determine when to train\nengine in order to control cost. If not specified: the default value for\n'CreateEngine' method is 'TRAINING'. The default value for\n'UpdateEngine' method is to keep the state the same as before. Possible values: [\"PAUSED\", \"TRAINING\"]"]
    pub fn training_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.training_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of engine. e.g., 'recommended-for-you'.\nThis field together with MediaRecommendationEngineConfig.optimizationObjective describes\nengine metadata to use to control engine training and serving.\nCurrently supported values: 'recommended-for-you', 'others-you-may-like',\n'more-like-this', 'most-popular-items'."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `engine_features_config` after provisioning.\n"]    pub fn engine_features_config (& self) -> ListRef < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElEngineFeaturesConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.engine_features_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `optimization_objective_config` after provisioning.\n"]    pub fn optimization_objective_config (& self) -> ListRef < DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigElOptimizationObjectiveConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.optimization_objective_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineRecommendationEngineTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DiscoveryEngineRecommendationEngineTimeoutsEl {
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
impl ToListMappable for DiscoveryEngineRecommendationEngineTimeoutsEl {
    type O = BlockAssignable<DiscoveryEngineRecommendationEngineTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineRecommendationEngineTimeoutsEl {}
impl BuildDiscoveryEngineRecommendationEngineTimeoutsEl {
    pub fn build(self) -> DiscoveryEngineRecommendationEngineTimeoutsEl {
        DiscoveryEngineRecommendationEngineTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineRecommendationEngineTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineRecommendationEngineTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineRecommendationEngineTimeoutsElRef {
        DiscoveryEngineRecommendationEngineTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineRecommendationEngineTimeoutsElRef {
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
struct DiscoveryEngineRecommendationEngineDynamic {
    common_config: Option<DynamicBlock<DiscoveryEngineRecommendationEngineCommonConfigEl>>,
    media_recommendation_engine_config:
        Option<DynamicBlock<DiscoveryEngineRecommendationEngineMediaRecommendationEngineConfigEl>>,
}
