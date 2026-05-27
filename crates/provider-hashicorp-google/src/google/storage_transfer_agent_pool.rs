use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct StorageTransferAgentPoolData {
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
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bandwidth_limit: Option<Vec<StorageTransferAgentPoolBandwidthLimitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<StorageTransferAgentPoolTimeoutsEl>,
    dynamic: StorageTransferAgentPoolDynamic,
}
struct StorageTransferAgentPool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<StorageTransferAgentPoolData>,
}
#[derive(Clone)]
pub struct StorageTransferAgentPool(Rc<StorageTransferAgentPool_>);
impl StorageTransferAgentPool {
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
    #[doc = "Set the field `display_name`.\nSpecifies the client-specified AgentPool description."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
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
    #[doc = "Set the field `bandwidth_limit`.\n"]
    pub fn set_bandwidth_limit(
        self,
        v: impl Into<BlockAssignable<StorageTransferAgentPoolBandwidthLimitEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bandwidth_limit = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bandwidth_limit = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<StorageTransferAgentPoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nSpecifies the client-specified AgentPool description."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the agent pool to create.\n\nThe agentPoolId must meet the following requirements:\n* Length of 128 characters or less.\n* Not start with the string goog.\n* Start with a lowercase ASCII character, followed by:\n  * Zero or more: lowercase Latin alphabet characters, numerals, hyphens (-), periods (.), underscores (_), or tildes (~).\n  * One or more numerals or lowercase ASCII characters.\n\nAs expressed by the regular expression: ^(?!goog)[a-z]([a-z0-9-._~]*[a-z0-9])?$."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nSpecifies the state of the AgentPool."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bandwidth_limit` after provisioning.\n"]
    pub fn bandwidth_limit(&self) -> ListRef<StorageTransferAgentPoolBandwidthLimitElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bandwidth_limit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageTransferAgentPoolTimeoutsElRef {
        StorageTransferAgentPoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for StorageTransferAgentPool {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for StorageTransferAgentPool {}
impl ToListMappable for StorageTransferAgentPool {
    type O = ListRef<StorageTransferAgentPoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for StorageTransferAgentPool_ {
    fn extract_resource_type(&self) -> String {
        "google_storage_transfer_agent_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildStorageTransferAgentPool {
    pub tf_id: String,
    #[doc = "The ID of the agent pool to create.\n\nThe agentPoolId must meet the following requirements:\n* Length of 128 characters or less.\n* Not start with the string goog.\n* Start with a lowercase ASCII character, followed by:\n  * Zero or more: lowercase Latin alphabet characters, numerals, hyphens (-), periods (.), underscores (_), or tildes (~).\n  * One or more numerals or lowercase ASCII characters.\n\nAs expressed by the regular expression: ^(?!goog)[a-z]([a-z0-9-._~]*[a-z0-9])?$."]
    pub name: PrimField<String>,
}
impl BuildStorageTransferAgentPool {
    pub fn build(self, stack: &mut Stack) -> StorageTransferAgentPool {
        let out = StorageTransferAgentPool(Rc::new(StorageTransferAgentPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(StorageTransferAgentPoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                bandwidth_limit: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct StorageTransferAgentPoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferAgentPoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl StorageTransferAgentPoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nSpecifies the client-specified AgentPool description."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the agent pool to create.\n\nThe agentPoolId must meet the following requirements:\n* Length of 128 characters or less.\n* Not start with the string goog.\n* Start with a lowercase ASCII character, followed by:\n  * Zero or more: lowercase Latin alphabet characters, numerals, hyphens (-), periods (.), underscores (_), or tildes (~).\n  * One or more numerals or lowercase ASCII characters.\n\nAs expressed by the regular expression: ^(?!goog)[a-z]([a-z0-9-._~]*[a-z0-9])?$."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nSpecifies the state of the AgentPool."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bandwidth_limit` after provisioning.\n"]
    pub fn bandwidth_limit(&self) -> ListRef<StorageTransferAgentPoolBandwidthLimitElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bandwidth_limit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageTransferAgentPoolTimeoutsElRef {
        StorageTransferAgentPoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct StorageTransferAgentPoolBandwidthLimitEl {
    limit_mbps: PrimField<String>,
}
impl StorageTransferAgentPoolBandwidthLimitEl {}
impl ToListMappable for StorageTransferAgentPoolBandwidthLimitEl {
    type O = BlockAssignable<StorageTransferAgentPoolBandwidthLimitEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferAgentPoolBandwidthLimitEl {
    #[doc = "Bandwidth rate in megabytes per second, distributed across all the agents in the pool."]
    pub limit_mbps: PrimField<String>,
}
impl BuildStorageTransferAgentPoolBandwidthLimitEl {
    pub fn build(self) -> StorageTransferAgentPoolBandwidthLimitEl {
        StorageTransferAgentPoolBandwidthLimitEl {
            limit_mbps: self.limit_mbps,
        }
    }
}
pub struct StorageTransferAgentPoolBandwidthLimitElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferAgentPoolBandwidthLimitElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferAgentPoolBandwidthLimitElRef {
        StorageTransferAgentPoolBandwidthLimitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferAgentPoolBandwidthLimitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `limit_mbps` after provisioning.\nBandwidth rate in megabytes per second, distributed across all the agents in the pool."]
    pub fn limit_mbps(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.limit_mbps", self.base))
    }
}
#[derive(Serialize)]
pub struct StorageTransferAgentPoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl StorageTransferAgentPoolTimeoutsEl {
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
impl ToListMappable for StorageTransferAgentPoolTimeoutsEl {
    type O = BlockAssignable<StorageTransferAgentPoolTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildStorageTransferAgentPoolTimeoutsEl {}
impl BuildStorageTransferAgentPoolTimeoutsEl {
    pub fn build(self) -> StorageTransferAgentPoolTimeoutsEl {
        StorageTransferAgentPoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct StorageTransferAgentPoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for StorageTransferAgentPoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> StorageTransferAgentPoolTimeoutsElRef {
        StorageTransferAgentPoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl StorageTransferAgentPoolTimeoutsElRef {
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
struct StorageTransferAgentPoolDynamic {
    bandwidth_limit: Option<DynamicBlock<StorageTransferAgentPoolBandwidthLimitEl>>,
}
