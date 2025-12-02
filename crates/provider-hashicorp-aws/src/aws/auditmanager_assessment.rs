use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AuditmanagerAssessmentData {
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
    framework_id: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assessment_reports_destination:
        Option<Vec<AuditmanagerAssessmentAssessmentReportsDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<Vec<AuditmanagerAssessmentRolesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<Vec<AuditmanagerAssessmentScopeEl>>,
    dynamic: AuditmanagerAssessmentDynamic,
}
struct AuditmanagerAssessment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AuditmanagerAssessmentData>,
}
#[derive(Clone)]
pub struct AuditmanagerAssessment(Rc<AuditmanagerAssessment_>);
impl AuditmanagerAssessment {
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
    #[doc = "Set the field `assessment_reports_destination`.\n"]
    pub fn set_assessment_reports_destination(
        self,
        v: impl Into<BlockAssignable<AuditmanagerAssessmentAssessmentReportsDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().assessment_reports_destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .assessment_reports_destination = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `roles`.\n"]
    pub fn set_roles(self, v: impl Into<BlockAssignable<AuditmanagerAssessmentRolesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().roles = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.roles = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(self, v: impl Into<BlockAssignable<AuditmanagerAssessmentScopeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scope = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scope = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `framework_id` after provisioning.\n"]
    pub fn framework_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.framework_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `roles_all` after provisioning.\n"]
    pub fn roles_all(&self) -> ListRef<AuditmanagerAssessmentRolesAllElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.roles_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `assessment_reports_destination` after provisioning.\n"]
    pub fn assessment_reports_destination(
        &self,
    ) -> ListRef<AuditmanagerAssessmentAssessmentReportsDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.assessment_reports_destination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> ListRef<AuditmanagerAssessmentScopeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
}
impl Referable for AuditmanagerAssessment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AuditmanagerAssessment {}
impl ToListMappable for AuditmanagerAssessment {
    type O = ListRef<AuditmanagerAssessmentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AuditmanagerAssessment_ {
    fn extract_resource_type(&self) -> String {
        "aws_auditmanager_assessment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAuditmanagerAssessment {
    pub tf_id: String,
    #[doc = ""]
    pub framework_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildAuditmanagerAssessment {
    pub fn build(self, stack: &mut Stack) -> AuditmanagerAssessment {
        let out = AuditmanagerAssessment(Rc::new(AuditmanagerAssessment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AuditmanagerAssessmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                framework_id: self.framework_id,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                assessment_reports_destination: core::default::Default::default(),
                roles: core::default::Default::default(),
                scope: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AuditmanagerAssessmentRef {
    shared: StackShared,
    base: String,
}
impl Ref for AuditmanagerAssessmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AuditmanagerAssessmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `framework_id` after provisioning.\n"]
    pub fn framework_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.framework_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `roles_all` after provisioning.\n"]
    pub fn roles_all(&self) -> ListRef<AuditmanagerAssessmentRolesAllElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.roles_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `assessment_reports_destination` after provisioning.\n"]
    pub fn assessment_reports_destination(
        &self,
    ) -> ListRef<AuditmanagerAssessmentAssessmentReportsDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.assessment_reports_destination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> ListRef<AuditmanagerAssessmentScopeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AuditmanagerAssessmentRolesAllEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_type: Option<PrimField<String>>,
}
impl AuditmanagerAssessmentRolesAllEl {
    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `role_type`.\n"]
    pub fn set_role_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_type = Some(v.into());
        self
    }
}
impl ToListMappable for AuditmanagerAssessmentRolesAllEl {
    type O = BlockAssignable<AuditmanagerAssessmentRolesAllEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAuditmanagerAssessmentRolesAllEl {}
impl BuildAuditmanagerAssessmentRolesAllEl {
    pub fn build(self) -> AuditmanagerAssessmentRolesAllEl {
        AuditmanagerAssessmentRolesAllEl {
            role_arn: core::default::Default::default(),
            role_type: core::default::Default::default(),
        }
    }
}
pub struct AuditmanagerAssessmentRolesAllElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AuditmanagerAssessmentRolesAllElRef {
    fn new(shared: StackShared, base: String) -> AuditmanagerAssessmentRolesAllElRef {
        AuditmanagerAssessmentRolesAllElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AuditmanagerAssessmentRolesAllElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `role_type` after provisioning.\n"]
    pub fn role_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_type", self.base))
    }
}
#[derive(Serialize)]
pub struct AuditmanagerAssessmentAssessmentReportsDestinationEl {
    destination: PrimField<String>,
    destination_type: PrimField<String>,
}
impl AuditmanagerAssessmentAssessmentReportsDestinationEl {}
impl ToListMappable for AuditmanagerAssessmentAssessmentReportsDestinationEl {
    type O = BlockAssignable<AuditmanagerAssessmentAssessmentReportsDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAuditmanagerAssessmentAssessmentReportsDestinationEl {
    #[doc = ""]
    pub destination: PrimField<String>,
    #[doc = ""]
    pub destination_type: PrimField<String>,
}
impl BuildAuditmanagerAssessmentAssessmentReportsDestinationEl {
    pub fn build(self) -> AuditmanagerAssessmentAssessmentReportsDestinationEl {
        AuditmanagerAssessmentAssessmentReportsDestinationEl {
            destination: self.destination,
            destination_type: self.destination_type,
        }
    }
}
pub struct AuditmanagerAssessmentAssessmentReportsDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AuditmanagerAssessmentAssessmentReportsDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AuditmanagerAssessmentAssessmentReportsDestinationElRef {
        AuditmanagerAssessmentAssessmentReportsDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AuditmanagerAssessmentAssessmentReportsDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }
    #[doc = "Get a reference to the value of field `destination_type` after provisioning.\n"]
    pub fn destination_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.destination_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AuditmanagerAssessmentRolesEl {
    role_arn: PrimField<String>,
    role_type: PrimField<String>,
}
impl AuditmanagerAssessmentRolesEl {}
impl ToListMappable for AuditmanagerAssessmentRolesEl {
    type O = BlockAssignable<AuditmanagerAssessmentRolesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAuditmanagerAssessmentRolesEl {
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub role_type: PrimField<String>,
}
impl BuildAuditmanagerAssessmentRolesEl {
    pub fn build(self) -> AuditmanagerAssessmentRolesEl {
        AuditmanagerAssessmentRolesEl {
            role_arn: self.role_arn,
            role_type: self.role_type,
        }
    }
}
pub struct AuditmanagerAssessmentRolesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AuditmanagerAssessmentRolesElRef {
    fn new(shared: StackShared, base: String) -> AuditmanagerAssessmentRolesElRef {
        AuditmanagerAssessmentRolesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AuditmanagerAssessmentRolesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `role_type` after provisioning.\n"]
    pub fn role_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_type", self.base))
    }
}
#[derive(Serialize)]
pub struct AuditmanagerAssessmentScopeElAwsAccountsEl {
    id: PrimField<String>,
}
impl AuditmanagerAssessmentScopeElAwsAccountsEl {}
impl ToListMappable for AuditmanagerAssessmentScopeElAwsAccountsEl {
    type O = BlockAssignable<AuditmanagerAssessmentScopeElAwsAccountsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAuditmanagerAssessmentScopeElAwsAccountsEl {
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildAuditmanagerAssessmentScopeElAwsAccountsEl {
    pub fn build(self) -> AuditmanagerAssessmentScopeElAwsAccountsEl {
        AuditmanagerAssessmentScopeElAwsAccountsEl { id: self.id }
    }
}
pub struct AuditmanagerAssessmentScopeElAwsAccountsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AuditmanagerAssessmentScopeElAwsAccountsElRef {
    fn new(shared: StackShared, base: String) -> AuditmanagerAssessmentScopeElAwsAccountsElRef {
        AuditmanagerAssessmentScopeElAwsAccountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AuditmanagerAssessmentScopeElAwsAccountsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}
#[derive(Serialize)]
pub struct AuditmanagerAssessmentScopeElAwsServicesEl {
    service_name: PrimField<String>,
}
impl AuditmanagerAssessmentScopeElAwsServicesEl {}
impl ToListMappable for AuditmanagerAssessmentScopeElAwsServicesEl {
    type O = BlockAssignable<AuditmanagerAssessmentScopeElAwsServicesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAuditmanagerAssessmentScopeElAwsServicesEl {
    #[doc = ""]
    pub service_name: PrimField<String>,
}
impl BuildAuditmanagerAssessmentScopeElAwsServicesEl {
    pub fn build(self) -> AuditmanagerAssessmentScopeElAwsServicesEl {
        AuditmanagerAssessmentScopeElAwsServicesEl {
            service_name: self.service_name,
        }
    }
}
pub struct AuditmanagerAssessmentScopeElAwsServicesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AuditmanagerAssessmentScopeElAwsServicesElRef {
    fn new(shared: StackShared, base: String) -> AuditmanagerAssessmentScopeElAwsServicesElRef {
        AuditmanagerAssessmentScopeElAwsServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AuditmanagerAssessmentScopeElAwsServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct AuditmanagerAssessmentScopeElDynamic {
    aws_accounts: Option<DynamicBlock<AuditmanagerAssessmentScopeElAwsAccountsEl>>,
    aws_services: Option<DynamicBlock<AuditmanagerAssessmentScopeElAwsServicesEl>>,
}
#[derive(Serialize)]
pub struct AuditmanagerAssessmentScopeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_accounts: Option<Vec<AuditmanagerAssessmentScopeElAwsAccountsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_services: Option<Vec<AuditmanagerAssessmentScopeElAwsServicesEl>>,
    dynamic: AuditmanagerAssessmentScopeElDynamic,
}
impl AuditmanagerAssessmentScopeEl {
    #[doc = "Set the field `aws_accounts`.\n"]
    pub fn set_aws_accounts(
        mut self,
        v: impl Into<BlockAssignable<AuditmanagerAssessmentScopeElAwsAccountsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_accounts = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_accounts = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `aws_services`.\n"]
    pub fn set_aws_services(
        mut self,
        v: impl Into<BlockAssignable<AuditmanagerAssessmentScopeElAwsServicesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_services = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_services = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AuditmanagerAssessmentScopeEl {
    type O = BlockAssignable<AuditmanagerAssessmentScopeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAuditmanagerAssessmentScopeEl {}
impl BuildAuditmanagerAssessmentScopeEl {
    pub fn build(self) -> AuditmanagerAssessmentScopeEl {
        AuditmanagerAssessmentScopeEl {
            aws_accounts: core::default::Default::default(),
            aws_services: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AuditmanagerAssessmentScopeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AuditmanagerAssessmentScopeElRef {
    fn new(shared: StackShared, base: String) -> AuditmanagerAssessmentScopeElRef {
        AuditmanagerAssessmentScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AuditmanagerAssessmentScopeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct AuditmanagerAssessmentDynamic {
    assessment_reports_destination:
        Option<DynamicBlock<AuditmanagerAssessmentAssessmentReportsDestinationEl>>,
    roles: Option<DynamicBlock<AuditmanagerAssessmentRolesEl>>,
    scope: Option<DynamicBlock<AuditmanagerAssessmentScopeEl>>,
}
