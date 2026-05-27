use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ChronicleWatchlistData {
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
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance: PrimField<String>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiplying_factor: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    watchlist_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_population_mechanism: Option<Vec<ChronicleWatchlistEntityPopulationMechanismEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ChronicleWatchlistTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    watchlist_user_preferences: Option<Vec<ChronicleWatchlistWatchlistUserPreferencesEl>>,
    dynamic: ChronicleWatchlistDynamic,
}
struct ChronicleWatchlist_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ChronicleWatchlistData>,
}
#[derive(Clone)]
pub struct ChronicleWatchlist(Rc<ChronicleWatchlist_>);
impl ChronicleWatchlist {
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
    #[doc = "Set the field `description`.\nOptional. Description of the watchlist."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `multiplying_factor`.\nOptional. Weight applied to the risk score for entities\nin this watchlist.\nThe default is 1.0 if it is not specified."]
    pub fn set_multiplying_factor(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().multiplying_factor = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `watchlist_id`.\nOptional. The ID to use for the watchlist,\nwhich will become the final component of the watchlist's resource name.\nThis value should be 4-63 characters, and valid characters\nare /a-z-/."]
    pub fn set_watchlist_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().watchlist_id = Some(v.into());
        self
    }
    #[doc = "Set the field `entity_population_mechanism`.\n"]
    pub fn set_entity_population_mechanism(
        self,
        v: impl Into<BlockAssignable<ChronicleWatchlistEntityPopulationMechanismEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entity_population_mechanism = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entity_population_mechanism = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ChronicleWatchlistTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `watchlist_user_preferences`.\n"]
    pub fn set_watchlist_user_preferences(
        self,
        v: impl Into<BlockAssignable<ChronicleWatchlistWatchlistUserPreferencesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().watchlist_user_preferences = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.watchlist_user_preferences = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time the watchlist was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the watchlist."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. Display name of the watchlist.\nNote that it must be at least one character and less than 63 characters\n(https://google.aip.dev/148)."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entity_count` after provisioning.\nCount of different types of entities in the watchlist."]
    pub fn entity_count(&self) -> ListRef<ChronicleWatchlistEntityCountElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entity_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multiplying_factor` after provisioning.\nOptional. Weight applied to the risk score for entities\nin this watchlist.\nThe default is 1.0 if it is not specified."]
    pub fn multiplying_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multiplying_factor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Resource name of the watchlist. This unique identifier is generated using values provided for the URL parameters.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/watchlists/{watchlist}"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Time the watchlist was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `watchlist_id` after provisioning.\nOptional. The ID to use for the watchlist,\nwhich will become the final component of the watchlist's resource name.\nThis value should be 4-63 characters, and valid characters\nare /a-z-/."]
    pub fn watchlist_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.watchlist_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entity_population_mechanism` after provisioning.\n"]
    pub fn entity_population_mechanism(
        &self,
    ) -> ListRef<ChronicleWatchlistEntityPopulationMechanismElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entity_population_mechanism", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleWatchlistTimeoutsElRef {
        ChronicleWatchlistTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `watchlist_user_preferences` after provisioning.\n"]
    pub fn watchlist_user_preferences(
        &self,
    ) -> ListRef<ChronicleWatchlistWatchlistUserPreferencesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.watchlist_user_preferences", self.extract_ref()),
        )
    }
}
impl Referable for ChronicleWatchlist {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ChronicleWatchlist {}
impl ToListMappable for ChronicleWatchlist {
    type O = ListRef<ChronicleWatchlistRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ChronicleWatchlist_ {
    fn extract_resource_type(&self) -> String {
        "google_chronicle_watchlist".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildChronicleWatchlist {
    pub tf_id: String,
    #[doc = "Required. Display name of the watchlist.\nNote that it must be at least one character and less than 63 characters\n(https://google.aip.dev/148)."]
    pub display_name: PrimField<String>,
    #[doc = "The unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub instance: PrimField<String>,
    #[doc = "The location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub location: PrimField<String>,
}
impl BuildChronicleWatchlist {
    pub fn build(self, stack: &mut Stack) -> ChronicleWatchlist {
        let out = ChronicleWatchlist(Rc::new(ChronicleWatchlist_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ChronicleWatchlistData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                multiplying_factor: core::default::Default::default(),
                project: core::default::Default::default(),
                watchlist_id: core::default::Default::default(),
                entity_population_mechanism: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                watchlist_user_preferences: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ChronicleWatchlistRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleWatchlistRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ChronicleWatchlistRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time the watchlist was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the watchlist."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. Display name of the watchlist.\nNote that it must be at least one character and less than 63 characters\n(https://google.aip.dev/148)."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entity_count` after provisioning.\nCount of different types of entities in the watchlist."]
    pub fn entity_count(&self) -> ListRef<ChronicleWatchlistEntityCountElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entity_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multiplying_factor` after provisioning.\nOptional. Weight applied to the risk score for entities\nin this watchlist.\nThe default is 1.0 if it is not specified."]
    pub fn multiplying_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multiplying_factor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Resource name of the watchlist. This unique identifier is generated using values provided for the URL parameters.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/watchlists/{watchlist}"]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Time the watchlist was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `watchlist_id` after provisioning.\nOptional. The ID to use for the watchlist,\nwhich will become the final component of the watchlist's resource name.\nThis value should be 4-63 characters, and valid characters\nare /a-z-/."]
    pub fn watchlist_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.watchlist_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entity_population_mechanism` after provisioning.\n"]
    pub fn entity_population_mechanism(
        &self,
    ) -> ListRef<ChronicleWatchlistEntityPopulationMechanismElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.entity_population_mechanism", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleWatchlistTimeoutsElRef {
        ChronicleWatchlistTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `watchlist_user_preferences` after provisioning.\n"]
    pub fn watchlist_user_preferences(
        &self,
    ) -> ListRef<ChronicleWatchlistWatchlistUserPreferencesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.watchlist_user_preferences", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleWatchlistEntityCountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    asset: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<PrimField<f64>>,
}
impl ChronicleWatchlistEntityCountEl {
    #[doc = "Set the field `asset`.\n"]
    pub fn set_asset(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.asset = Some(v.into());
        self
    }
    #[doc = "Set the field `user`.\n"]
    pub fn set_user(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.user = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleWatchlistEntityCountEl {
    type O = BlockAssignable<ChronicleWatchlistEntityCountEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleWatchlistEntityCountEl {}
impl BuildChronicleWatchlistEntityCountEl {
    pub fn build(self) -> ChronicleWatchlistEntityCountEl {
        ChronicleWatchlistEntityCountEl {
            asset: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}
pub struct ChronicleWatchlistEntityCountElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleWatchlistEntityCountElRef {
    fn new(shared: StackShared, base: String) -> ChronicleWatchlistEntityCountElRef {
        ChronicleWatchlistEntityCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleWatchlistEntityCountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `asset` after provisioning.\n"]
    pub fn asset(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.asset", self.base))
    }
    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleWatchlistEntityPopulationMechanismElManualEl {}
impl ChronicleWatchlistEntityPopulationMechanismElManualEl {}
impl ToListMappable for ChronicleWatchlistEntityPopulationMechanismElManualEl {
    type O = BlockAssignable<ChronicleWatchlistEntityPopulationMechanismElManualEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleWatchlistEntityPopulationMechanismElManualEl {}
impl BuildChronicleWatchlistEntityPopulationMechanismElManualEl {
    pub fn build(self) -> ChronicleWatchlistEntityPopulationMechanismElManualEl {
        ChronicleWatchlistEntityPopulationMechanismElManualEl {}
    }
}
pub struct ChronicleWatchlistEntityPopulationMechanismElManualElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleWatchlistEntityPopulationMechanismElManualElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ChronicleWatchlistEntityPopulationMechanismElManualElRef {
        ChronicleWatchlistEntityPopulationMechanismElManualElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleWatchlistEntityPopulationMechanismElManualElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct ChronicleWatchlistEntityPopulationMechanismElDynamic {
    manual: Option<DynamicBlock<ChronicleWatchlistEntityPopulationMechanismElManualEl>>,
}
#[derive(Serialize)]
pub struct ChronicleWatchlistEntityPopulationMechanismEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    manual: Option<Vec<ChronicleWatchlistEntityPopulationMechanismElManualEl>>,
    dynamic: ChronicleWatchlistEntityPopulationMechanismElDynamic,
}
impl ChronicleWatchlistEntityPopulationMechanismEl {
    #[doc = "Set the field `manual`.\n"]
    pub fn set_manual(
        mut self,
        v: impl Into<BlockAssignable<ChronicleWatchlistEntityPopulationMechanismElManualEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.manual = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.manual = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ChronicleWatchlistEntityPopulationMechanismEl {
    type O = BlockAssignable<ChronicleWatchlistEntityPopulationMechanismEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleWatchlistEntityPopulationMechanismEl {}
impl BuildChronicleWatchlistEntityPopulationMechanismEl {
    pub fn build(self) -> ChronicleWatchlistEntityPopulationMechanismEl {
        ChronicleWatchlistEntityPopulationMechanismEl {
            manual: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ChronicleWatchlistEntityPopulationMechanismElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleWatchlistEntityPopulationMechanismElRef {
    fn new(shared: StackShared, base: String) -> ChronicleWatchlistEntityPopulationMechanismElRef {
        ChronicleWatchlistEntityPopulationMechanismElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleWatchlistEntityPopulationMechanismElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `manual` after provisioning.\n"]
    pub fn manual(&self) -> ListRef<ChronicleWatchlistEntityPopulationMechanismElManualElRef> {
        ListRef::new(self.shared().clone(), format!("{}.manual", self.base))
    }
}
#[derive(Serialize)]
pub struct ChronicleWatchlistTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ChronicleWatchlistTimeoutsEl {
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
impl ToListMappable for ChronicleWatchlistTimeoutsEl {
    type O = BlockAssignable<ChronicleWatchlistTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleWatchlistTimeoutsEl {}
impl BuildChronicleWatchlistTimeoutsEl {
    pub fn build(self) -> ChronicleWatchlistTimeoutsEl {
        ChronicleWatchlistTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ChronicleWatchlistTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleWatchlistTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ChronicleWatchlistTimeoutsElRef {
        ChronicleWatchlistTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleWatchlistTimeoutsElRef {
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
#[derive(Serialize)]
pub struct ChronicleWatchlistWatchlistUserPreferencesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pinned: Option<PrimField<bool>>,
}
impl ChronicleWatchlistWatchlistUserPreferencesEl {
    #[doc = "Set the field `pinned`.\nOptional. Whether the watchlist is pinned on the dashboard."]
    pub fn set_pinned(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.pinned = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleWatchlistWatchlistUserPreferencesEl {
    type O = BlockAssignable<ChronicleWatchlistWatchlistUserPreferencesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleWatchlistWatchlistUserPreferencesEl {}
impl BuildChronicleWatchlistWatchlistUserPreferencesEl {
    pub fn build(self) -> ChronicleWatchlistWatchlistUserPreferencesEl {
        ChronicleWatchlistWatchlistUserPreferencesEl {
            pinned: core::default::Default::default(),
        }
    }
}
pub struct ChronicleWatchlistWatchlistUserPreferencesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleWatchlistWatchlistUserPreferencesElRef {
    fn new(shared: StackShared, base: String) -> ChronicleWatchlistWatchlistUserPreferencesElRef {
        ChronicleWatchlistWatchlistUserPreferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleWatchlistWatchlistUserPreferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `pinned` after provisioning.\nOptional. Whether the watchlist is pinned on the dashboard."]
    pub fn pinned(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.pinned", self.base))
    }
}
#[derive(Serialize, Default)]
struct ChronicleWatchlistDynamic {
    entity_population_mechanism:
        Option<DynamicBlock<ChronicleWatchlistEntityPopulationMechanismEl>>,
    watchlist_user_preferences: Option<DynamicBlock<ChronicleWatchlistWatchlistUserPreferencesEl>>,
}
