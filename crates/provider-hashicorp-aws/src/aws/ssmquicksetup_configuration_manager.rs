use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SsmquicksetupConfigurationManagerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_definition:
        Option<Vec<SsmquicksetupConfigurationManagerConfigurationDefinitionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SsmquicksetupConfigurationManagerTimeoutsEl>,
    dynamic: SsmquicksetupConfigurationManagerDynamic,
}

struct SsmquicksetupConfigurationManager_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SsmquicksetupConfigurationManagerData>,
}

#[derive(Clone)]
pub struct SsmquicksetupConfigurationManager(Rc<SsmquicksetupConfigurationManager_>);

impl SsmquicksetupConfigurationManager {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `configuration_definition`.\n"]
    pub fn set_configuration_definition(
        self,
        v: impl Into<BlockAssignable<SsmquicksetupConfigurationManagerConfigurationDefinitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration_definition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration_definition = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SsmquicksetupConfigurationManagerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `manager_arn` after provisioning.\n"]
    pub fn manager_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manager_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status_summaries` after provisioning.\n"]
    pub fn status_summaries(
        &self,
    ) -> ListRef<SsmquicksetupConfigurationManagerStatusSummariesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status_summaries", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `configuration_definition` after provisioning.\n"]
    pub fn configuration_definition(
        &self,
    ) -> ListRef<SsmquicksetupConfigurationManagerConfigurationDefinitionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration_definition", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SsmquicksetupConfigurationManagerTimeoutsElRef {
        SsmquicksetupConfigurationManagerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for SsmquicksetupConfigurationManager {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SsmquicksetupConfigurationManager {}

impl ToListMappable for SsmquicksetupConfigurationManager {
    type O = ListRef<SsmquicksetupConfigurationManagerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SsmquicksetupConfigurationManager_ {
    fn extract_resource_type(&self) -> String {
        "aws_ssmquicksetup_configuration_manager".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsmquicksetupConfigurationManager {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildSsmquicksetupConfigurationManager {
    pub fn build(self, stack: &mut Stack) -> SsmquicksetupConfigurationManager {
        let out = SsmquicksetupConfigurationManager(Rc::new(SsmquicksetupConfigurationManager_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SsmquicksetupConfigurationManagerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                configuration_definition: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsmquicksetupConfigurationManagerRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmquicksetupConfigurationManagerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SsmquicksetupConfigurationManagerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `manager_arn` after provisioning.\n"]
    pub fn manager_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manager_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status_summaries` after provisioning.\n"]
    pub fn status_summaries(
        &self,
    ) -> ListRef<SsmquicksetupConfigurationManagerStatusSummariesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status_summaries", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `configuration_definition` after provisioning.\n"]
    pub fn configuration_definition(
        &self,
    ) -> ListRef<SsmquicksetupConfigurationManagerConfigurationDefinitionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration_definition", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SsmquicksetupConfigurationManagerTimeoutsElRef {
        SsmquicksetupConfigurationManagerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SsmquicksetupConfigurationManagerStatusSummariesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_type: Option<PrimField<String>>,
}

impl SsmquicksetupConfigurationManagerStatusSummariesEl {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc = "Set the field `status_message`.\n"]
    pub fn set_status_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_message = Some(v.into());
        self
    }

    #[doc = "Set the field `status_type`.\n"]
    pub fn set_status_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status_type = Some(v.into());
        self
    }
}

impl ToListMappable for SsmquicksetupConfigurationManagerStatusSummariesEl {
    type O = BlockAssignable<SsmquicksetupConfigurationManagerStatusSummariesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmquicksetupConfigurationManagerStatusSummariesEl {}

impl BuildSsmquicksetupConfigurationManagerStatusSummariesEl {
    pub fn build(self) -> SsmquicksetupConfigurationManagerStatusSummariesEl {
        SsmquicksetupConfigurationManagerStatusSummariesEl {
            status: core::default::Default::default(),
            status_message: core::default::Default::default(),
            status_type: core::default::Default::default(),
        }
    }
}

pub struct SsmquicksetupConfigurationManagerStatusSummariesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmquicksetupConfigurationManagerStatusSummariesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmquicksetupConfigurationManagerStatusSummariesElRef {
        SsmquicksetupConfigurationManagerStatusSummariesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmquicksetupConfigurationManagerStatusSummariesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_message", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `status_type` after provisioning.\n"]
    pub fn status_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_type", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmquicksetupConfigurationManagerConfigurationDefinitionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    local_deployment_administration_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_deployment_execution_role_name: Option<PrimField<String>>,
    parameters: RecField<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    type_version: Option<PrimField<String>>,
}

impl SsmquicksetupConfigurationManagerConfigurationDefinitionEl {
    #[doc = "Set the field `local_deployment_administration_role_arn`.\n"]
    pub fn set_local_deployment_administration_role_arn(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.local_deployment_administration_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `local_deployment_execution_role_name`.\n"]
    pub fn set_local_deployment_execution_role_name(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.local_deployment_execution_role_name = Some(v.into());
        self
    }

    #[doc = "Set the field `type_version`.\n"]
    pub fn set_type_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_version = Some(v.into());
        self
    }
}

impl ToListMappable for SsmquicksetupConfigurationManagerConfigurationDefinitionEl {
    type O = BlockAssignable<SsmquicksetupConfigurationManagerConfigurationDefinitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmquicksetupConfigurationManagerConfigurationDefinitionEl {
    #[doc = ""]
    pub parameters: RecField<PrimField<String>>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildSsmquicksetupConfigurationManagerConfigurationDefinitionEl {
    pub fn build(self) -> SsmquicksetupConfigurationManagerConfigurationDefinitionEl {
        SsmquicksetupConfigurationManagerConfigurationDefinitionEl {
            local_deployment_administration_role_arn: core::default::Default::default(),
            local_deployment_execution_role_name: core::default::Default::default(),
            parameters: self.parameters,
            type_: self.type_,
            type_version: core::default::Default::default(),
        }
    }
}

pub struct SsmquicksetupConfigurationManagerConfigurationDefinitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmquicksetupConfigurationManagerConfigurationDefinitionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsmquicksetupConfigurationManagerConfigurationDefinitionElRef {
        SsmquicksetupConfigurationManagerConfigurationDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmquicksetupConfigurationManagerConfigurationDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `local_deployment_administration_role_arn` after provisioning.\n"]
    pub fn local_deployment_administration_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_deployment_administration_role_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `local_deployment_execution_role_name` after provisioning.\n"]
    pub fn local_deployment_execution_role_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_deployment_execution_role_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `type_version` after provisioning.\n"]
    pub fn type_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type_version", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmquicksetupConfigurationManagerTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl SsmquicksetupConfigurationManagerTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for SsmquicksetupConfigurationManagerTimeoutsEl {
    type O = BlockAssignable<SsmquicksetupConfigurationManagerTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmquicksetupConfigurationManagerTimeoutsEl {}

impl BuildSsmquicksetupConfigurationManagerTimeoutsEl {
    pub fn build(self) -> SsmquicksetupConfigurationManagerTimeoutsEl {
        SsmquicksetupConfigurationManagerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct SsmquicksetupConfigurationManagerTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsmquicksetupConfigurationManagerTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SsmquicksetupConfigurationManagerTimeoutsElRef {
        SsmquicksetupConfigurationManagerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmquicksetupConfigurationManagerTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmquicksetupConfigurationManagerDynamic {
    configuration_definition:
        Option<DynamicBlock<SsmquicksetupConfigurationManagerConfigurationDefinitionEl>>,
}
