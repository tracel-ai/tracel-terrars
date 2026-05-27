use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BigtableAppProfileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app_profile_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_warnings: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_cluster_routing_cluster_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_cluster_routing_use_any: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    row_affinity: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_boost_isolation_read_only: Option<Vec<BigtableAppProfileDataBoostIsolationReadOnlyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    single_cluster_routing: Option<Vec<BigtableAppProfileSingleClusterRoutingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standard_isolation: Option<Vec<BigtableAppProfileStandardIsolationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigtableAppProfileTimeoutsEl>,
    dynamic: BigtableAppProfileDynamic,
}
struct BigtableAppProfile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigtableAppProfileData>,
}
#[derive(Clone)]
pub struct BigtableAppProfile(Rc<BigtableAppProfile_>);
impl BigtableAppProfile {
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
    #[doc = "Set the field `description`.\nLong form description of the use case for this app profile."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_warnings`.\nIf true, ignore safety checks when deleting/updating the app profile."]
    pub fn set_ignore_warnings(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_warnings = Some(v.into());
        self
    }
    #[doc = "Set the field `instance`.\nThe name of the instance to create the app profile within."]
    pub fn set_instance(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance = Some(v.into());
        self
    }
    #[doc = "Set the field `multi_cluster_routing_cluster_ids`.\nThe set of clusters to route to. The order is ignored; clusters will be tried in order of distance. If left empty, all clusters are eligible."]
    pub fn set_multi_cluster_routing_cluster_ids(
        self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().multi_cluster_routing_cluster_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `multi_cluster_routing_use_any`.\nIf true, read/write requests are routed to the nearest cluster in the instance, and will fail over to the nearest cluster that is available\nin the event of transient errors or delays. Clusters in a region are considered equidistant. Choosing this option sacrifices read-your-writes\nconsistency to improve availability."]
    pub fn set_multi_cluster_routing_use_any(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multi_cluster_routing_use_any = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `row_affinity`.\nMust be used with multi-cluster routing. If true, then this app profile will use row affinity sticky routing. With row affinity, Bigtable will route single row key requests based on the row key, rather than randomly. Instead, each row key will be assigned to a cluster by Cloud Bigtable, and will stick to that cluster. Choosing this option improves read-your-writes consistency for most requests under most circumstances, without sacrificing availability. Consistency is not guaranteed, as requests may still fail over between clusters in the event of errors or latency."]
    pub fn set_row_affinity(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().row_affinity = Some(v.into());
        self
    }
    #[doc = "Set the field `data_boost_isolation_read_only`.\n"]
    pub fn set_data_boost_isolation_read_only(
        self,
        v: impl Into<BlockAssignable<BigtableAppProfileDataBoostIsolationReadOnlyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_boost_isolation_read_only = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .data_boost_isolation_read_only = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `single_cluster_routing`.\n"]
    pub fn set_single_cluster_routing(
        self,
        v: impl Into<BlockAssignable<BigtableAppProfileSingleClusterRoutingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().single_cluster_routing = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.single_cluster_routing = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `standard_isolation`.\n"]
    pub fn set_standard_isolation(
        self,
        v: impl Into<BlockAssignable<BigtableAppProfileStandardIsolationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().standard_isolation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.standard_isolation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigtableAppProfileTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `app_profile_id` after provisioning.\nThe unique name of the app profile in the form '[_a-zA-Z0-9][-_.a-zA-Z0-9]*'."]
    pub fn app_profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_profile_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nLong form description of the use case for this app profile."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignore_warnings` after provisioning.\nIf true, ignore safety checks when deleting/updating the app profile."]
    pub fn ignore_warnings(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_warnings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe name of the instance to create the app profile within."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multi_cluster_routing_cluster_ids` after provisioning.\nThe set of clusters to route to. The order is ignored; clusters will be tried in order of distance. If left empty, all clusters are eligible."]
    pub fn multi_cluster_routing_cluster_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multi_cluster_routing_cluster_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multi_cluster_routing_use_any` after provisioning.\nIf true, read/write requests are routed to the nearest cluster in the instance, and will fail over to the nearest cluster that is available\nin the event of transient errors or delays. Clusters in a region are considered equidistant. Choosing this option sacrifices read-your-writes\nconsistency to improve availability."]
    pub fn multi_cluster_routing_use_any(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multi_cluster_routing_use_any", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique name of the requested app profile. Values are of the form 'projects/<project>/instances/<instance>/appProfiles/<appProfileId>'."]
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
    #[doc = "Get a reference to the value of field `row_affinity` after provisioning.\nMust be used with multi-cluster routing. If true, then this app profile will use row affinity sticky routing. With row affinity, Bigtable will route single row key requests based on the row key, rather than randomly. Instead, each row key will be assigned to a cluster by Cloud Bigtable, and will stick to that cluster. Choosing this option improves read-your-writes consistency for most requests under most circumstances, without sacrificing availability. Consistency is not guaranteed, as requests may still fail over between clusters in the event of errors or latency."]
    pub fn row_affinity(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.row_affinity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_boost_isolation_read_only` after provisioning.\n"]
    pub fn data_boost_isolation_read_only(
        &self,
    ) -> ListRef<BigtableAppProfileDataBoostIsolationReadOnlyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_boost_isolation_read_only", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `single_cluster_routing` after provisioning.\n"]
    pub fn single_cluster_routing(&self) -> ListRef<BigtableAppProfileSingleClusterRoutingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.single_cluster_routing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `standard_isolation` after provisioning.\n"]
    pub fn standard_isolation(&self) -> ListRef<BigtableAppProfileStandardIsolationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.standard_isolation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableAppProfileTimeoutsElRef {
        BigtableAppProfileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BigtableAppProfile {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BigtableAppProfile {}
impl ToListMappable for BigtableAppProfile {
    type O = ListRef<BigtableAppProfileRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BigtableAppProfile_ {
    fn extract_resource_type(&self) -> String {
        "google_bigtable_app_profile".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBigtableAppProfile {
    pub tf_id: String,
    #[doc = "The unique name of the app profile in the form '[_a-zA-Z0-9][-_.a-zA-Z0-9]*'."]
    pub app_profile_id: PrimField<String>,
}
impl BuildBigtableAppProfile {
    pub fn build(self, stack: &mut Stack) -> BigtableAppProfile {
        let out = BigtableAppProfile(Rc::new(BigtableAppProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigtableAppProfileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app_profile_id: self.app_profile_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_warnings: core::default::Default::default(),
                instance: core::default::Default::default(),
                multi_cluster_routing_cluster_ids: core::default::Default::default(),
                multi_cluster_routing_use_any: core::default::Default::default(),
                project: core::default::Default::default(),
                row_affinity: core::default::Default::default(),
                data_boost_isolation_read_only: core::default::Default::default(),
                single_cluster_routing: core::default::Default::default(),
                standard_isolation: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BigtableAppProfileRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigtableAppProfileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BigtableAppProfileRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_profile_id` after provisioning.\nThe unique name of the app profile in the form '[_a-zA-Z0-9][-_.a-zA-Z0-9]*'."]
    pub fn app_profile_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_profile_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nLong form description of the use case for this app profile."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignore_warnings` after provisioning.\nIf true, ignore safety checks when deleting/updating the app profile."]
    pub fn ignore_warnings(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_warnings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe name of the instance to create the app profile within."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multi_cluster_routing_cluster_ids` after provisioning.\nThe set of clusters to route to. The order is ignored; clusters will be tried in order of distance. If left empty, all clusters are eligible."]
    pub fn multi_cluster_routing_cluster_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multi_cluster_routing_cluster_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multi_cluster_routing_use_any` after provisioning.\nIf true, read/write requests are routed to the nearest cluster in the instance, and will fail over to the nearest cluster that is available\nin the event of transient errors or delays. Clusters in a region are considered equidistant. Choosing this option sacrifices read-your-writes\nconsistency to improve availability."]
    pub fn multi_cluster_routing_use_any(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multi_cluster_routing_use_any", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique name of the requested app profile. Values are of the form 'projects/<project>/instances/<instance>/appProfiles/<appProfileId>'."]
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
    #[doc = "Get a reference to the value of field `row_affinity` after provisioning.\nMust be used with multi-cluster routing. If true, then this app profile will use row affinity sticky routing. With row affinity, Bigtable will route single row key requests based on the row key, rather than randomly. Instead, each row key will be assigned to a cluster by Cloud Bigtable, and will stick to that cluster. Choosing this option improves read-your-writes consistency for most requests under most circumstances, without sacrificing availability. Consistency is not guaranteed, as requests may still fail over between clusters in the event of errors or latency."]
    pub fn row_affinity(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.row_affinity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_boost_isolation_read_only` after provisioning.\n"]
    pub fn data_boost_isolation_read_only(
        &self,
    ) -> ListRef<BigtableAppProfileDataBoostIsolationReadOnlyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_boost_isolation_read_only", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `single_cluster_routing` after provisioning.\n"]
    pub fn single_cluster_routing(&self) -> ListRef<BigtableAppProfileSingleClusterRoutingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.single_cluster_routing", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `standard_isolation` after provisioning.\n"]
    pub fn standard_isolation(&self) -> ListRef<BigtableAppProfileStandardIsolationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.standard_isolation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableAppProfileTimeoutsElRef {
        BigtableAppProfileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BigtableAppProfileDataBoostIsolationReadOnlyEl {
    compute_billing_owner: PrimField<String>,
}
impl BigtableAppProfileDataBoostIsolationReadOnlyEl {}
impl ToListMappable for BigtableAppProfileDataBoostIsolationReadOnlyEl {
    type O = BlockAssignable<BigtableAppProfileDataBoostIsolationReadOnlyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigtableAppProfileDataBoostIsolationReadOnlyEl {
    #[doc = "The Compute Billing Owner for this Data Boost App Profile. Possible values: [\"HOST_PAYS\"]"]
    pub compute_billing_owner: PrimField<String>,
}
impl BuildBigtableAppProfileDataBoostIsolationReadOnlyEl {
    pub fn build(self) -> BigtableAppProfileDataBoostIsolationReadOnlyEl {
        BigtableAppProfileDataBoostIsolationReadOnlyEl {
            compute_billing_owner: self.compute_billing_owner,
        }
    }
}
pub struct BigtableAppProfileDataBoostIsolationReadOnlyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigtableAppProfileDataBoostIsolationReadOnlyElRef {
    fn new(shared: StackShared, base: String) -> BigtableAppProfileDataBoostIsolationReadOnlyElRef {
        BigtableAppProfileDataBoostIsolationReadOnlyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigtableAppProfileDataBoostIsolationReadOnlyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `compute_billing_owner` after provisioning.\nThe Compute Billing Owner for this Data Boost App Profile. Possible values: [\"HOST_PAYS\"]"]
    pub fn compute_billing_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_billing_owner", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigtableAppProfileSingleClusterRoutingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_transactional_writes: Option<PrimField<bool>>,
    cluster_id: PrimField<String>,
}
impl BigtableAppProfileSingleClusterRoutingEl {
    #[doc = "Set the field `allow_transactional_writes`.\nIf true, CheckAndMutateRow and ReadModifyWriteRow requests are allowed by this app profile.\nIt is unsafe to send these requests to the same table/row/column in multiple clusters."]
    pub fn set_allow_transactional_writes(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_transactional_writes = Some(v.into());
        self
    }
}
impl ToListMappable for BigtableAppProfileSingleClusterRoutingEl {
    type O = BlockAssignable<BigtableAppProfileSingleClusterRoutingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigtableAppProfileSingleClusterRoutingEl {
    #[doc = "The cluster to which read/write requests should be routed."]
    pub cluster_id: PrimField<String>,
}
impl BuildBigtableAppProfileSingleClusterRoutingEl {
    pub fn build(self) -> BigtableAppProfileSingleClusterRoutingEl {
        BigtableAppProfileSingleClusterRoutingEl {
            allow_transactional_writes: core::default::Default::default(),
            cluster_id: self.cluster_id,
        }
    }
}
pub struct BigtableAppProfileSingleClusterRoutingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigtableAppProfileSingleClusterRoutingElRef {
    fn new(shared: StackShared, base: String) -> BigtableAppProfileSingleClusterRoutingElRef {
        BigtableAppProfileSingleClusterRoutingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigtableAppProfileSingleClusterRoutingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_transactional_writes` after provisioning.\nIf true, CheckAndMutateRow and ReadModifyWriteRow requests are allowed by this app profile.\nIt is unsafe to send these requests to the same table/row/column in multiple clusters."]
    pub fn allow_transactional_writes(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_transactional_writes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\nThe cluster to which read/write requests should be routed."]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }
}
#[derive(Serialize)]
pub struct BigtableAppProfileStandardIsolationEl {
    priority: PrimField<String>,
}
impl BigtableAppProfileStandardIsolationEl {}
impl ToListMappable for BigtableAppProfileStandardIsolationEl {
    type O = BlockAssignable<BigtableAppProfileStandardIsolationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigtableAppProfileStandardIsolationEl {
    #[doc = "The priority of requests sent using this app profile. Possible values: [\"PRIORITY_LOW\", \"PRIORITY_MEDIUM\", \"PRIORITY_HIGH\"]"]
    pub priority: PrimField<String>,
}
impl BuildBigtableAppProfileStandardIsolationEl {
    pub fn build(self) -> BigtableAppProfileStandardIsolationEl {
        BigtableAppProfileStandardIsolationEl {
            priority: self.priority,
        }
    }
}
pub struct BigtableAppProfileStandardIsolationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigtableAppProfileStandardIsolationElRef {
    fn new(shared: StackShared, base: String) -> BigtableAppProfileStandardIsolationElRef {
        BigtableAppProfileStandardIsolationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigtableAppProfileStandardIsolationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nThe priority of requests sent using this app profile. Possible values: [\"PRIORITY_LOW\", \"PRIORITY_MEDIUM\", \"PRIORITY_HIGH\"]"]
    pub fn priority(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
}
#[derive(Serialize)]
pub struct BigtableAppProfileTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BigtableAppProfileTimeoutsEl {
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
impl ToListMappable for BigtableAppProfileTimeoutsEl {
    type O = BlockAssignable<BigtableAppProfileTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigtableAppProfileTimeoutsEl {}
impl BuildBigtableAppProfileTimeoutsEl {
    pub fn build(self) -> BigtableAppProfileTimeoutsEl {
        BigtableAppProfileTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BigtableAppProfileTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigtableAppProfileTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigtableAppProfileTimeoutsElRef {
        BigtableAppProfileTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigtableAppProfileTimeoutsElRef {
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
struct BigtableAppProfileDynamic {
    data_boost_isolation_read_only:
        Option<DynamicBlock<BigtableAppProfileDataBoostIsolationReadOnlyEl>>,
    single_cluster_routing: Option<DynamicBlock<BigtableAppProfileSingleClusterRoutingEl>>,
    standard_isolation: Option<DynamicBlock<BigtableAppProfileStandardIsolationEl>>,
}
