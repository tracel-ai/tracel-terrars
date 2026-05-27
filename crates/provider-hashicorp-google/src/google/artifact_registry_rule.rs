use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ArtifactRegistryRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    repository_id: PrimField<String>,
    rule_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<ArtifactRegistryRuleConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ArtifactRegistryRuleTimeoutsEl>,
    dynamic: ArtifactRegistryRuleDynamic,
}
struct ArtifactRegistryRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ArtifactRegistryRuleData>,
}
#[derive(Clone)]
pub struct ArtifactRegistryRule(Rc<ArtifactRegistryRule_>);
impl ArtifactRegistryRule {
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
    #[doc = "Set the field `action`.\nThe action this rule takes. Possible values: [\"ACTION_UNSPECIFIED\", \"ALLOW\", \"DENY\"]"]
    pub fn set_action(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().action = Some(v.into());
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
    #[doc = "Set the field `location`.\nThe name of the repository's location. In addition to specific regions,\nspecial values for multi-region locations are 'asia', 'europe', and 'us'.\nSee [here](https://cloud.google.com/artifact-registry/docs/repositories/repo-locations),\nor use the\n[google_artifact_registry_locations](https://registry.terraform.io/providers/hashicorp/google/latest/docs/data-sources/artifact_registry_locations)\ndata source for possible values."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `operation`.\nThe operation the rule applies to. Possible values: [\"OPERATION_UNSPECIFIED\", \"DOWNLOAD\"]"]
    pub fn set_operation(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().operation = Some(v.into());
        self
    }
    #[doc = "Set the field `package_id`.\nThe package ID the rule applies to. If empty, this rule applies to all\npackages inside the repository."]
    pub fn set_package_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().package_id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        self,
        v: impl Into<BlockAssignable<ArtifactRegistryRuleConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ArtifactRegistryRuleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe action this rule takes. Possible values: [\"ACTION_UNSPECIFIED\", \"ALLOW\", \"DENY\"]"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the repository's location. In addition to specific regions,\nspecial values for multi-region locations are 'asia', 'europe', and 'us'.\nSee [here](https://cloud.google.com/artifact-registry/docs/repositories/repo-locations),\nor use the\n[google_artifact_registry_locations](https://registry.terraform.io/providers/hashicorp/google/latest/docs/data-sources/artifact_registry_locations)\ndata source for possible values."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the rule, for example:\n\"projects/p1/locations/us-central1/repositories/repo1/rules/rule1\"."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `operation` after provisioning.\nThe operation the rule applies to. Possible values: [\"OPERATION_UNSPECIFIED\", \"DOWNLOAD\"]"]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `package_id` after provisioning.\nThe package ID the rule applies to. If empty, this rule applies to all\npackages inside the repository."]
    pub fn package_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_id` after provisioning.\nThe last part of the repository name, for example:\n\"repo1\""]
    pub fn repository_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\nThe rule id to use for this repository."]
    pub fn rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<ArtifactRegistryRuleConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ArtifactRegistryRuleTimeoutsElRef {
        ArtifactRegistryRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ArtifactRegistryRule {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ArtifactRegistryRule {}
impl ToListMappable for ArtifactRegistryRule {
    type O = ListRef<ArtifactRegistryRuleRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ArtifactRegistryRule_ {
    fn extract_resource_type(&self) -> String {
        "google_artifact_registry_rule".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildArtifactRegistryRule {
    pub tf_id: String,
    #[doc = "The last part of the repository name, for example:\n\"repo1\""]
    pub repository_id: PrimField<String>,
    #[doc = "The rule id to use for this repository."]
    pub rule_id: PrimField<String>,
}
impl BuildArtifactRegistryRule {
    pub fn build(self, stack: &mut Stack) -> ArtifactRegistryRule {
        let out = ArtifactRegistryRule(Rc::new(ArtifactRegistryRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ArtifactRegistryRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                action: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                operation: core::default::Default::default(),
                package_id: core::default::Default::default(),
                project: core::default::Default::default(),
                repository_id: self.repository_id,
                rule_id: self.rule_id,
                condition: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ArtifactRegistryRuleRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ArtifactRegistryRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe action this rule takes. Possible values: [\"ACTION_UNSPECIFIED\", \"ALLOW\", \"DENY\"]"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the repository's location. In addition to specific regions,\nspecial values for multi-region locations are 'asia', 'europe', and 'us'.\nSee [here](https://cloud.google.com/artifact-registry/docs/repositories/repo-locations),\nor use the\n[google_artifact_registry_locations](https://registry.terraform.io/providers/hashicorp/google/latest/docs/data-sources/artifact_registry_locations)\ndata source for possible values."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the rule, for example:\n\"projects/p1/locations/us-central1/repositories/repo1/rules/rule1\"."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `operation` after provisioning.\nThe operation the rule applies to. Possible values: [\"OPERATION_UNSPECIFIED\", \"DOWNLOAD\"]"]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `package_id` after provisioning.\nThe package ID the rule applies to. If empty, this rule applies to all\npackages inside the repository."]
    pub fn package_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_id` after provisioning.\nThe last part of the repository name, for example:\n\"repo1\""]
    pub fn repository_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\nThe rule id to use for this repository."]
    pub fn rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<ArtifactRegistryRuleConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ArtifactRegistryRuleTimeoutsElRef {
        ArtifactRegistryRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ArtifactRegistryRuleConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl ArtifactRegistryRuleConditionEl {
    #[doc = "Set the field `description`.\nOptional. Description of the expression. This is a longer text which\ndescribes the expression, e.g. when hovered over it in a UI."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nOptional. String indicating the location of the expression for error\nreporting, e.g. a file name and a position in the file."]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nOptional. Title for the expression, i.e. a short string describing its purpose."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for ArtifactRegistryRuleConditionEl {
    type O = BlockAssignable<ArtifactRegistryRuleConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRuleConditionEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}
impl BuildArtifactRegistryRuleConditionEl {
    pub fn build(self) -> ArtifactRegistryRuleConditionEl {
        ArtifactRegistryRuleConditionEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct ArtifactRegistryRuleConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRuleConditionElRef {
    fn new(shared: StackShared, base: String) -> ArtifactRegistryRuleConditionElRef {
        ArtifactRegistryRuleConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRuleConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the expression. This is a longer text which\ndescribes the expression, e.g. when hovered over it in a UI."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nOptional. String indicating the location of the expression for error\nreporting, e.g. a file name and a position in the file."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nOptional. Title for the expression, i.e. a short string describing its purpose."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
#[derive(Serialize)]
pub struct ArtifactRegistryRuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ArtifactRegistryRuleTimeoutsEl {
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
impl ToListMappable for ArtifactRegistryRuleTimeoutsEl {
    type O = BlockAssignable<ArtifactRegistryRuleTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildArtifactRegistryRuleTimeoutsEl {}
impl BuildArtifactRegistryRuleTimeoutsEl {
    pub fn build(self) -> ArtifactRegistryRuleTimeoutsEl {
        ArtifactRegistryRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ArtifactRegistryRuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ArtifactRegistryRuleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ArtifactRegistryRuleTimeoutsElRef {
        ArtifactRegistryRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ArtifactRegistryRuleTimeoutsElRef {
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
struct ArtifactRegistryRuleDynamic {
    condition: Option<DynamicBlock<ArtifactRegistryRuleConditionEl>>,
}
