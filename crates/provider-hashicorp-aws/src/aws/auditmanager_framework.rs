use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AuditmanagerFrameworkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compliance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_sets: Option<Vec<AuditmanagerFrameworkControlSetsEl>>,
    dynamic: AuditmanagerFrameworkDynamic,
}
struct AuditmanagerFramework_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AuditmanagerFrameworkData>,
}
#[derive(Clone)]
pub struct AuditmanagerFramework(Rc<AuditmanagerFramework_>);
impl AuditmanagerFramework {
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
    #[doc = "Set the field `compliance_type`.\n"]
    pub fn set_compliance_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().compliance_type = Some(v.into());
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
    #[doc = "Set the field `control_sets`.\n"]
    pub fn set_control_sets(
        self,
        v: impl Into<BlockAssignable<AuditmanagerFrameworkControlSetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().control_sets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.control_sets = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `compliance_type` after provisioning.\n"]
    pub fn compliance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compliance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `framework_type` after provisioning.\n"]
    pub fn framework_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.framework_type", self.extract_ref()),
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
}
impl Referable for AuditmanagerFramework {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AuditmanagerFramework {}
impl ToListMappable for AuditmanagerFramework {
    type O = ListRef<AuditmanagerFrameworkRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AuditmanagerFramework_ {
    fn extract_resource_type(&self) -> String {
        "aws_auditmanager_framework".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAuditmanagerFramework {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildAuditmanagerFramework {
    pub fn build(self, stack: &mut Stack) -> AuditmanagerFramework {
        let out = AuditmanagerFramework(Rc::new(AuditmanagerFramework_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AuditmanagerFrameworkData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                compliance_type: core::default::Default::default(),
                description: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                control_sets: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AuditmanagerFrameworkRef {
    shared: StackShared,
    base: String,
}
impl Ref for AuditmanagerFrameworkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AuditmanagerFrameworkRef {
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
    #[doc = "Get a reference to the value of field `compliance_type` after provisioning.\n"]
    pub fn compliance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compliance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `framework_type` after provisioning.\n"]
    pub fn framework_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.framework_type", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct AuditmanagerFrameworkControlSetsElControlsEl {
    id: PrimField<String>,
}
impl AuditmanagerFrameworkControlSetsElControlsEl {}
impl ToListMappable for AuditmanagerFrameworkControlSetsElControlsEl {
    type O = BlockAssignable<AuditmanagerFrameworkControlSetsElControlsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAuditmanagerFrameworkControlSetsElControlsEl {
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildAuditmanagerFrameworkControlSetsElControlsEl {
    pub fn build(self) -> AuditmanagerFrameworkControlSetsElControlsEl {
        AuditmanagerFrameworkControlSetsElControlsEl { id: self.id }
    }
}
pub struct AuditmanagerFrameworkControlSetsElControlsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AuditmanagerFrameworkControlSetsElControlsElRef {
    fn new(shared: StackShared, base: String) -> AuditmanagerFrameworkControlSetsElControlsElRef {
        AuditmanagerFrameworkControlSetsElControlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AuditmanagerFrameworkControlSetsElControlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}
#[derive(Serialize, Default)]
struct AuditmanagerFrameworkControlSetsElDynamic {
    controls: Option<DynamicBlock<AuditmanagerFrameworkControlSetsElControlsEl>>,
}
#[derive(Serialize)]
pub struct AuditmanagerFrameworkControlSetsEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    controls: Option<Vec<AuditmanagerFrameworkControlSetsElControlsEl>>,
    dynamic: AuditmanagerFrameworkControlSetsElDynamic,
}
impl AuditmanagerFrameworkControlSetsEl {
    #[doc = "Set the field `controls`.\n"]
    pub fn set_controls(
        mut self,
        v: impl Into<BlockAssignable<AuditmanagerFrameworkControlSetsElControlsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.controls = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.controls = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AuditmanagerFrameworkControlSetsEl {
    type O = BlockAssignable<AuditmanagerFrameworkControlSetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAuditmanagerFrameworkControlSetsEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildAuditmanagerFrameworkControlSetsEl {
    pub fn build(self) -> AuditmanagerFrameworkControlSetsEl {
        AuditmanagerFrameworkControlSetsEl {
            name: self.name,
            controls: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AuditmanagerFrameworkControlSetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AuditmanagerFrameworkControlSetsElRef {
    fn new(shared: StackShared, base: String) -> AuditmanagerFrameworkControlSetsElRef {
        AuditmanagerFrameworkControlSetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AuditmanagerFrameworkControlSetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct AuditmanagerFrameworkDynamic {
    control_sets: Option<DynamicBlock<AuditmanagerFrameworkControlSetsEl>>,
}
