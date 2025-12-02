use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SsmAssociationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apply_only_at_cron_interval: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    association_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automation_target_parameter_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compliance_severity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_errors: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule_expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_compliance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_success_timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_location: Option<Vec<SsmAssociationOutputLocationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    targets: Option<Vec<SsmAssociationTargetsEl>>,
    dynamic: SsmAssociationDynamic,
}
struct SsmAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SsmAssociationData>,
}
#[derive(Clone)]
pub struct SsmAssociation(Rc<SsmAssociation_>);
impl SsmAssociation {
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
    #[doc = "Set the field `apply_only_at_cron_interval`.\n"]
    pub fn set_apply_only_at_cron_interval(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().apply_only_at_cron_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `association_name`.\n"]
    pub fn set_association_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().association_name = Some(v.into());
        self
    }
    #[doc = "Set the field `automation_target_parameter_name`.\n"]
    pub fn set_automation_target_parameter_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().automation_target_parameter_name = Some(v.into());
        self
    }
    #[doc = "Set the field `compliance_severity`.\n"]
    pub fn set_compliance_severity(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().compliance_severity = Some(v.into());
        self
    }
    #[doc = "Set the field `document_version`.\n"]
    pub fn set_document_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().document_version = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `max_concurrency`.\n"]
    pub fn set_max_concurrency(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().max_concurrency = Some(v.into());
        self
    }
    #[doc = "Set the field `max_errors`.\n"]
    pub fn set_max_errors(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().max_errors = Some(v.into());
        self
    }
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule_expression`.\n"]
    pub fn set_schedule_expression(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().schedule_expression = Some(v.into());
        self
    }
    #[doc = "Set the field `sync_compliance`.\n"]
    pub fn set_sync_compliance(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sync_compliance = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `wait_for_success_timeout_seconds`.\n"]
    pub fn set_wait_for_success_timeout_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().wait_for_success_timeout_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `output_location`.\n"]
    pub fn set_output_location(
        self,
        v: impl Into<BlockAssignable<SsmAssociationOutputLocationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().output_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.output_location = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `targets`.\n"]
    pub fn set_targets(self, v: impl Into<BlockAssignable<SsmAssociationTargetsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().targets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.targets = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `apply_only_at_cron_interval` after provisioning.\n"]
    pub fn apply_only_at_cron_interval(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.apply_only_at_cron_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `association_name` after provisioning.\n"]
    pub fn association_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automation_target_parameter_name` after provisioning.\n"]
    pub fn automation_target_parameter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.automation_target_parameter_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compliance_severity` after provisioning.\n"]
    pub fn compliance_severity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compliance_severity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `document_version` after provisioning.\n"]
    pub fn document_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.document_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `max_concurrency` after provisioning.\n"]
    pub fn max_concurrency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrency", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_errors` after provisioning.\n"]
    pub fn max_errors(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_errors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule_expression", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sync_compliance` after provisioning.\n"]
    pub fn sync_compliance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sync_compliance", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `wait_for_success_timeout_seconds` after provisioning.\n"]
    pub fn wait_for_success_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_success_timeout_seconds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_location` after provisioning.\n"]
    pub fn output_location(&self) -> ListRef<SsmAssociationOutputLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> ListRef<SsmAssociationTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.targets", self.extract_ref()),
        )
    }
}
impl Referable for SsmAssociation {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SsmAssociation {}
impl ToListMappable for SsmAssociation {
    type O = ListRef<SsmAssociationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SsmAssociation_ {
    fn extract_resource_type(&self) -> String {
        "aws_ssm_association".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSsmAssociation {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildSsmAssociation {
    pub fn build(self, stack: &mut Stack) -> SsmAssociation {
        let out = SsmAssociation(Rc::new(SsmAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SsmAssociationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                apply_only_at_cron_interval: core::default::Default::default(),
                association_name: core::default::Default::default(),
                automation_target_parameter_name: core::default::Default::default(),
                compliance_severity: core::default::Default::default(),
                document_version: core::default::Default::default(),
                id: core::default::Default::default(),
                max_concurrency: core::default::Default::default(),
                max_errors: core::default::Default::default(),
                name: self.name,
                parameters: core::default::Default::default(),
                region: core::default::Default::default(),
                schedule_expression: core::default::Default::default(),
                sync_compliance: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                wait_for_success_timeout_seconds: core::default::Default::default(),
                output_location: core::default::Default::default(),
                targets: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SsmAssociationRef {
    shared: StackShared,
    base: String,
}
impl Ref for SsmAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SsmAssociationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `apply_only_at_cron_interval` after provisioning.\n"]
    pub fn apply_only_at_cron_interval(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.apply_only_at_cron_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `association_name` after provisioning.\n"]
    pub fn association_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automation_target_parameter_name` after provisioning.\n"]
    pub fn automation_target_parameter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.automation_target_parameter_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compliance_severity` after provisioning.\n"]
    pub fn compliance_severity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compliance_severity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `document_version` after provisioning.\n"]
    pub fn document_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.document_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `max_concurrency` after provisioning.\n"]
    pub fn max_concurrency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrency", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_errors` after provisioning.\n"]
    pub fn max_errors(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_errors", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule_expression", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sync_compliance` after provisioning.\n"]
    pub fn sync_compliance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sync_compliance", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `wait_for_success_timeout_seconds` after provisioning.\n"]
    pub fn wait_for_success_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_success_timeout_seconds", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_location` after provisioning.\n"]
    pub fn output_location(&self) -> ListRef<SsmAssociationOutputLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> ListRef<SsmAssociationTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.targets", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SsmAssociationOutputLocationEl {
    s3_bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_key_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_region: Option<PrimField<String>>,
}
impl SsmAssociationOutputLocationEl {
    #[doc = "Set the field `s3_key_prefix`.\n"]
    pub fn set_s3_key_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_key_prefix = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_region`.\n"]
    pub fn set_s3_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_region = Some(v.into());
        self
    }
}
impl ToListMappable for SsmAssociationOutputLocationEl {
    type O = BlockAssignable<SsmAssociationOutputLocationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSsmAssociationOutputLocationEl {
    #[doc = ""]
    pub s3_bucket_name: PrimField<String>,
}
impl BuildSsmAssociationOutputLocationEl {
    pub fn build(self) -> SsmAssociationOutputLocationEl {
        SsmAssociationOutputLocationEl {
            s3_bucket_name: self.s3_bucket_name,
            s3_key_prefix: core::default::Default::default(),
            s3_region: core::default::Default::default(),
        }
    }
}
pub struct SsmAssociationOutputLocationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SsmAssociationOutputLocationElRef {
    fn new(shared: StackShared, base: String) -> SsmAssociationOutputLocationElRef {
        SsmAssociationOutputLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SsmAssociationOutputLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_key_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_region` after provisioning.\n"]
    pub fn s3_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_region", self.base))
    }
}
#[derive(Serialize)]
pub struct SsmAssociationTargetsEl {
    key: PrimField<String>,
    values: ListField<PrimField<String>>,
}
impl SsmAssociationTargetsEl {}
impl ToListMappable for SsmAssociationTargetsEl {
    type O = BlockAssignable<SsmAssociationTargetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSsmAssociationTargetsEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub values: ListField<PrimField<String>>,
}
impl BuildSsmAssociationTargetsEl {
    pub fn build(self) -> SsmAssociationTargetsEl {
        SsmAssociationTargetsEl {
            key: self.key,
            values: self.values,
        }
    }
}
pub struct SsmAssociationTargetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SsmAssociationTargetsElRef {
    fn new(shared: StackShared, base: String) -> SsmAssociationTargetsElRef {
        SsmAssociationTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SsmAssociationTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize, Default)]
struct SsmAssociationDynamic {
    output_location: Option<DynamicBlock<SsmAssociationOutputLocationEl>>,
    targets: Option<DynamicBlock<SsmAssociationTargetsEl>>,
}
