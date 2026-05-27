use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApigeeSecurityMonitoringConditionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    condition_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    org_id: PrimField<String>,
    profile: PrimField<String>,
    scope: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_all_resources: Option<Vec<ApigeeSecurityMonitoringConditionIncludeAllResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeSecurityMonitoringConditionTimeoutsEl>,
    dynamic: ApigeeSecurityMonitoringConditionDynamic,
}
struct ApigeeSecurityMonitoringCondition_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeSecurityMonitoringConditionData>,
}
#[derive(Clone)]
pub struct ApigeeSecurityMonitoringCondition(Rc<ApigeeSecurityMonitoringCondition_>);
impl ApigeeSecurityMonitoringCondition {
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
    #[doc = "Set the field `include_all_resources`.\n"]
    pub fn set_include_all_resources(
        self,
        v: impl Into<BlockAssignable<ApigeeSecurityMonitoringConditionIncludeAllResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().include_all_resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.include_all_resources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApigeeSecurityMonitoringConditionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `condition_id` after provisioning.\nResource ID of the security monitoring condition."]
    pub fn condition_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.condition_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp at which this profile was created."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the security monitoring condition resource,\nin the format 'organizations/{{org_name}}/securityMonitoringConditions/{{condition_id}}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee Security Monitoring Condition,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `profile` after provisioning.\nID of security profile of the security monitoring condition."]
    pub fn profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\nID of security profile of the security monitoring condition."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_deployed_resources` after provisioning.\nTotal number of deployed resources within scope."]
    pub fn total_deployed_resources(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_deployed_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_monitored_resources` after provisioning.\nTotal number of monitored resources within this condition."]
    pub fn total_monitored_resources(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_monitored_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp at which this profile was most recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_all_resources` after provisioning.\n"]
    pub fn include_all_resources(
        &self,
    ) -> ListRef<ApigeeSecurityMonitoringConditionIncludeAllResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_all_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeSecurityMonitoringConditionTimeoutsElRef {
        ApigeeSecurityMonitoringConditionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApigeeSecurityMonitoringCondition {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApigeeSecurityMonitoringCondition {}
impl ToListMappable for ApigeeSecurityMonitoringCondition {
    type O = ListRef<ApigeeSecurityMonitoringConditionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApigeeSecurityMonitoringCondition_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_security_monitoring_condition".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApigeeSecurityMonitoringCondition {
    pub tf_id: String,
    #[doc = "Resource ID of the security monitoring condition."]
    pub condition_id: PrimField<String>,
    #[doc = "The Apigee Organization associated with the Apigee Security Monitoring Condition,\nin the format 'organizations/{{org_name}}'."]
    pub org_id: PrimField<String>,
    #[doc = "ID of security profile of the security monitoring condition."]
    pub profile: PrimField<String>,
    #[doc = "ID of security profile of the security monitoring condition."]
    pub scope: PrimField<String>,
}
impl BuildApigeeSecurityMonitoringCondition {
    pub fn build(self, stack: &mut Stack) -> ApigeeSecurityMonitoringCondition {
        let out = ApigeeSecurityMonitoringCondition(Rc::new(ApigeeSecurityMonitoringCondition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeSecurityMonitoringConditionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                condition_id: self.condition_id,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                org_id: self.org_id,
                profile: self.profile,
                scope: self.scope,
                include_all_resources: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApigeeSecurityMonitoringConditionRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeSecurityMonitoringConditionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApigeeSecurityMonitoringConditionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `condition_id` after provisioning.\nResource ID of the security monitoring condition."]
    pub fn condition_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.condition_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp at which this profile was created."]
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the security monitoring condition resource,\nin the format 'organizations/{{org_name}}/securityMonitoringConditions/{{condition_id}}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee Security Monitoring Condition,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `profile` after provisioning.\nID of security profile of the security monitoring condition."]
    pub fn profile(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profile", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\nID of security profile of the security monitoring condition."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_deployed_resources` after provisioning.\nTotal number of deployed resources within scope."]
    pub fn total_deployed_resources(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_deployed_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `total_monitored_resources` after provisioning.\nTotal number of monitored resources within this condition."]
    pub fn total_monitored_resources(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_monitored_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp at which this profile was most recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `include_all_resources` after provisioning.\n"]
    pub fn include_all_resources(
        &self,
    ) -> ListRef<ApigeeSecurityMonitoringConditionIncludeAllResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_all_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeSecurityMonitoringConditionTimeoutsElRef {
        ApigeeSecurityMonitoringConditionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeSecurityMonitoringConditionIncludeAllResourcesEl {}
impl ApigeeSecurityMonitoringConditionIncludeAllResourcesEl {}
impl ToListMappable for ApigeeSecurityMonitoringConditionIncludeAllResourcesEl {
    type O = BlockAssignable<ApigeeSecurityMonitoringConditionIncludeAllResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeSecurityMonitoringConditionIncludeAllResourcesEl {}
impl BuildApigeeSecurityMonitoringConditionIncludeAllResourcesEl {
    pub fn build(self) -> ApigeeSecurityMonitoringConditionIncludeAllResourcesEl {
        ApigeeSecurityMonitoringConditionIncludeAllResourcesEl {}
    }
}
pub struct ApigeeSecurityMonitoringConditionIncludeAllResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeSecurityMonitoringConditionIncludeAllResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeSecurityMonitoringConditionIncludeAllResourcesElRef {
        ApigeeSecurityMonitoringConditionIncludeAllResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeSecurityMonitoringConditionIncludeAllResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct ApigeeSecurityMonitoringConditionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApigeeSecurityMonitoringConditionTimeoutsEl {
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
impl ToListMappable for ApigeeSecurityMonitoringConditionTimeoutsEl {
    type O = BlockAssignable<ApigeeSecurityMonitoringConditionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeSecurityMonitoringConditionTimeoutsEl {}
impl BuildApigeeSecurityMonitoringConditionTimeoutsEl {
    pub fn build(self) -> ApigeeSecurityMonitoringConditionTimeoutsEl {
        ApigeeSecurityMonitoringConditionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApigeeSecurityMonitoringConditionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeSecurityMonitoringConditionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeSecurityMonitoringConditionTimeoutsElRef {
        ApigeeSecurityMonitoringConditionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeSecurityMonitoringConditionTimeoutsElRef {
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
struct ApigeeSecurityMonitoringConditionDynamic {
    include_all_resources:
        Option<DynamicBlock<ApigeeSecurityMonitoringConditionIncludeAllResourcesEl>>,
}
