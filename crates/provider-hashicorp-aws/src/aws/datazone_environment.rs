use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DatazoneEnvironmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blueprint_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    domain_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    glossary_terms: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    profile_identifier: PrimField<String>,
    project_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DatazoneEnvironmentTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_parameters: Option<Vec<DatazoneEnvironmentUserParametersEl>>,
    dynamic: DatazoneEnvironmentDynamic,
}
struct DatazoneEnvironment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatazoneEnvironmentData>,
}
#[derive(Clone)]
pub struct DatazoneEnvironment(Rc<DatazoneEnvironment_>);
impl DatazoneEnvironment {
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
    #[doc = "Set the field `account_identifier`.\n"]
    pub fn set_account_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `account_region`.\n"]
    pub fn set_account_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().account_region = Some(v.into());
        self
    }
    #[doc = "Set the field `blueprint_identifier`.\n"]
    pub fn set_blueprint_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().blueprint_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `glossary_terms`.\n"]
    pub fn set_glossary_terms(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().glossary_terms = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DatazoneEnvironmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `user_parameters`.\n"]
    pub fn set_user_parameters(
        self,
        v: impl Into<BlockAssignable<DatazoneEnvironmentUserParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `account_identifier` after provisioning.\n"]
    pub fn account_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `account_region` after provisioning.\n"]
    pub fn account_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `blueprint_identifier` after provisioning.\n"]
    pub fn blueprint_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.blueprint_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_by", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_identifier` after provisioning.\n"]
    pub fn domain_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `glossary_terms` after provisioning.\n"]
    pub fn glossary_terms(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.glossary_terms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_deployment` after provisioning.\n"]
    pub fn last_deployment(&self) -> ListRef<DatazoneEnvironmentLastDeploymentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.last_deployment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `profile_identifier` after provisioning.\n"]
    pub fn profile_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profile_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_identifier` after provisioning.\n"]
    pub fn project_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provider_environment` after provisioning.\n"]
    pub fn provider_environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provider_environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_resources` after provisioning.\n"]
    pub fn provisioned_resources(&self) -> ListRef<DatazoneEnvironmentProvisionedResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provisioned_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatazoneEnvironmentTimeoutsElRef {
        DatazoneEnvironmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_parameters` after provisioning.\n"]
    pub fn user_parameters(&self) -> ListRef<DatazoneEnvironmentUserParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_parameters", self.extract_ref()),
        )
    }
}
impl Referable for DatazoneEnvironment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DatazoneEnvironment {}
impl ToListMappable for DatazoneEnvironment {
    type O = ListRef<DatazoneEnvironmentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DatazoneEnvironment_ {
    fn extract_resource_type(&self) -> String {
        "aws_datazone_environment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDatazoneEnvironment {
    pub tf_id: String,
    #[doc = ""]
    pub domain_identifier: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub profile_identifier: PrimField<String>,
    #[doc = ""]
    pub project_identifier: PrimField<String>,
}
impl BuildDatazoneEnvironment {
    pub fn build(self, stack: &mut Stack) -> DatazoneEnvironment {
        let out = DatazoneEnvironment(Rc::new(DatazoneEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatazoneEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_identifier: core::default::Default::default(),
                account_region: core::default::Default::default(),
                blueprint_identifier: core::default::Default::default(),
                description: core::default::Default::default(),
                domain_identifier: self.domain_identifier,
                glossary_terms: core::default::Default::default(),
                name: self.name,
                profile_identifier: self.profile_identifier,
                project_identifier: self.project_identifier,
                region: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                user_parameters: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DatazoneEnvironmentRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneEnvironmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DatazoneEnvironmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_identifier` after provisioning.\n"]
    pub fn account_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `account_region` after provisioning.\n"]
    pub fn account_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `blueprint_identifier` after provisioning.\n"]
    pub fn blueprint_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.blueprint_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_by", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_identifier` after provisioning.\n"]
    pub fn domain_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `glossary_terms` after provisioning.\n"]
    pub fn glossary_terms(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.glossary_terms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_deployment` after provisioning.\n"]
    pub fn last_deployment(&self) -> ListRef<DatazoneEnvironmentLastDeploymentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.last_deployment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `profile_identifier` after provisioning.\n"]
    pub fn profile_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profile_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project_identifier` after provisioning.\n"]
    pub fn project_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provider_environment` after provisioning.\n"]
    pub fn provider_environment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provider_environment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_resources` after provisioning.\n"]
    pub fn provisioned_resources(&self) -> ListRef<DatazoneEnvironmentProvisionedResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provisioned_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatazoneEnvironmentTimeoutsElRef {
        DatazoneEnvironmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_parameters` after provisioning.\n"]
    pub fn user_parameters(&self) -> ListRef<DatazoneEnvironmentUserParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_parameters", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DatazoneEnvironmentLastDeploymentElFailureReasonsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl DatazoneEnvironmentLastDeploymentElFailureReasonsEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for DatazoneEnvironmentLastDeploymentElFailureReasonsEl {
    type O = BlockAssignable<DatazoneEnvironmentLastDeploymentElFailureReasonsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatazoneEnvironmentLastDeploymentElFailureReasonsEl {}
impl BuildDatazoneEnvironmentLastDeploymentElFailureReasonsEl {
    pub fn build(self) -> DatazoneEnvironmentLastDeploymentElFailureReasonsEl {
        DatazoneEnvironmentLastDeploymentElFailureReasonsEl {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct DatazoneEnvironmentLastDeploymentElFailureReasonsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneEnvironmentLastDeploymentElFailureReasonsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DatazoneEnvironmentLastDeploymentElFailureReasonsElRef {
        DatazoneEnvironmentLastDeploymentElFailureReasonsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatazoneEnvironmentLastDeploymentElFailureReasonsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct DatazoneEnvironmentLastDeploymentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_reasons: Option<ListField<DatazoneEnvironmentLastDeploymentElFailureReasonsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_deployment_complete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    messages: Option<ListField<PrimField<String>>>,
}
impl DatazoneEnvironmentLastDeploymentEl {
    #[doc = "Set the field `deployment_id`.\n"]
    pub fn set_deployment_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deployment_id = Some(v.into());
        self
    }
    #[doc = "Set the field `deployment_status`.\n"]
    pub fn set_deployment_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deployment_status = Some(v.into());
        self
    }
    #[doc = "Set the field `deployment_type`.\n"]
    pub fn set_deployment_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deployment_type = Some(v.into());
        self
    }
    #[doc = "Set the field `failure_reasons`.\n"]
    pub fn set_failure_reasons(
        mut self,
        v: impl Into<ListField<DatazoneEnvironmentLastDeploymentElFailureReasonsEl>>,
    ) -> Self {
        self.failure_reasons = Some(v.into());
        self
    }
    #[doc = "Set the field `is_deployment_complete`.\n"]
    pub fn set_is_deployment_complete(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_deployment_complete = Some(v.into());
        self
    }
    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.messages = Some(v.into());
        self
    }
}
impl ToListMappable for DatazoneEnvironmentLastDeploymentEl {
    type O = BlockAssignable<DatazoneEnvironmentLastDeploymentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatazoneEnvironmentLastDeploymentEl {}
impl BuildDatazoneEnvironmentLastDeploymentEl {
    pub fn build(self) -> DatazoneEnvironmentLastDeploymentEl {
        DatazoneEnvironmentLastDeploymentEl {
            deployment_id: core::default::Default::default(),
            deployment_status: core::default::Default::default(),
            deployment_type: core::default::Default::default(),
            failure_reasons: core::default::Default::default(),
            is_deployment_complete: core::default::Default::default(),
            messages: core::default::Default::default(),
        }
    }
}
pub struct DatazoneEnvironmentLastDeploymentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneEnvironmentLastDeploymentElRef {
    fn new(shared: StackShared, base: String) -> DatazoneEnvironmentLastDeploymentElRef {
        DatazoneEnvironmentLastDeploymentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatazoneEnvironmentLastDeploymentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deployment_id` after provisioning.\n"]
    pub fn deployment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_status` after provisioning.\n"]
    pub fn deployment_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_status", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `failure_reasons` after provisioning.\n"]
    pub fn failure_reasons(
        &self,
    ) -> ListRef<DatazoneEnvironmentLastDeploymentElFailureReasonsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.failure_reasons", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_deployment_complete` after provisioning.\n"]
    pub fn is_deployment_complete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_deployment_complete", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }
}
#[derive(Serialize)]
pub struct DatazoneEnvironmentProvisionedResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DatazoneEnvironmentProvisionedResourcesEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `provider`.\n"]
    pub fn set_provider(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.provider = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DatazoneEnvironmentProvisionedResourcesEl {
    type O = BlockAssignable<DatazoneEnvironmentProvisionedResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatazoneEnvironmentProvisionedResourcesEl {}
impl BuildDatazoneEnvironmentProvisionedResourcesEl {
    pub fn build(self) -> DatazoneEnvironmentProvisionedResourcesEl {
        DatazoneEnvironmentProvisionedResourcesEl {
            name: core::default::Default::default(),
            provider: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DatazoneEnvironmentProvisionedResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneEnvironmentProvisionedResourcesElRef {
    fn new(shared: StackShared, base: String) -> DatazoneEnvironmentProvisionedResourcesElRef {
        DatazoneEnvironmentProvisionedResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatazoneEnvironmentProvisionedResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DatazoneEnvironmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DatazoneEnvironmentTimeoutsEl {
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
impl ToListMappable for DatazoneEnvironmentTimeoutsEl {
    type O = BlockAssignable<DatazoneEnvironmentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatazoneEnvironmentTimeoutsEl {}
impl BuildDatazoneEnvironmentTimeoutsEl {
    pub fn build(self) -> DatazoneEnvironmentTimeoutsEl {
        DatazoneEnvironmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DatazoneEnvironmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneEnvironmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DatazoneEnvironmentTimeoutsElRef {
        DatazoneEnvironmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatazoneEnvironmentTimeoutsElRef {
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
#[derive(Serialize)]
pub struct DatazoneEnvironmentUserParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DatazoneEnvironmentUserParametersEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DatazoneEnvironmentUserParametersEl {
    type O = BlockAssignable<DatazoneEnvironmentUserParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatazoneEnvironmentUserParametersEl {}
impl BuildDatazoneEnvironmentUserParametersEl {
    pub fn build(self) -> DatazoneEnvironmentUserParametersEl {
        DatazoneEnvironmentUserParametersEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DatazoneEnvironmentUserParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneEnvironmentUserParametersElRef {
    fn new(shared: StackShared, base: String) -> DatazoneEnvironmentUserParametersElRef {
        DatazoneEnvironmentUserParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatazoneEnvironmentUserParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatazoneEnvironmentDynamic {
    user_parameters: Option<DynamicBlock<DatazoneEnvironmentUserParametersEl>>,
}
