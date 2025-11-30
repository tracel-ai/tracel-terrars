use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct VerifiedpermissionsPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    policy_store_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    definition: Option<Vec<VerifiedpermissionsPolicyDefinitionEl>>,
    dynamic: VerifiedpermissionsPolicyDynamic,
}

struct VerifiedpermissionsPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VerifiedpermissionsPolicyData>,
}

#[derive(Clone)]
pub struct VerifiedpermissionsPolicy(Rc<VerifiedpermissionsPolicy_>);

impl VerifiedpermissionsPolicy {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `definition`.\n"]
    pub fn set_definition(
        self,
        v: impl Into<BlockAssignable<VerifiedpermissionsPolicyDefinitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().definition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.definition = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\n"]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `policy_store_id` after provisioning.\n"]
    pub fn policy_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_store_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> ListRef<VerifiedpermissionsPolicyDefinitionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.definition", self.extract_ref()),
        )
    }
}

impl Referable for VerifiedpermissionsPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for VerifiedpermissionsPolicy {}

impl ToListMappable for VerifiedpermissionsPolicy {
    type O = ListRef<VerifiedpermissionsPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VerifiedpermissionsPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_verifiedpermissions_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVerifiedpermissionsPolicy {
    pub tf_id: String,
    #[doc = ""]
    pub policy_store_id: PrimField<String>,
}

impl BuildVerifiedpermissionsPolicy {
    pub fn build(self, stack: &mut Stack) -> VerifiedpermissionsPolicy {
        let out = VerifiedpermissionsPolicy(Rc::new(VerifiedpermissionsPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VerifiedpermissionsPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                policy_store_id: self.policy_store_id,
                region: core::default::Default::default(),
                definition: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VerifiedpermissionsPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedpermissionsPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl VerifiedpermissionsPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\n"]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `policy_store_id` after provisioning.\n"]
    pub fn policy_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_store_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> ListRef<VerifiedpermissionsPolicyDefinitionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.definition", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct VerifiedpermissionsPolicyDefinitionElStaticEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    statement: PrimField<String>,
}

impl VerifiedpermissionsPolicyDefinitionElStaticEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}

impl ToListMappable for VerifiedpermissionsPolicyDefinitionElStaticEl {
    type O = BlockAssignable<VerifiedpermissionsPolicyDefinitionElStaticEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsPolicyDefinitionElStaticEl {
    #[doc = ""]
    pub statement: PrimField<String>,
}

impl BuildVerifiedpermissionsPolicyDefinitionElStaticEl {
    pub fn build(self) -> VerifiedpermissionsPolicyDefinitionElStaticEl {
        VerifiedpermissionsPolicyDefinitionElStaticEl {
            description: core::default::Default::default(),
            statement: self.statement,
        }
    }
}

pub struct VerifiedpermissionsPolicyDefinitionElStaticElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedpermissionsPolicyDefinitionElStaticElRef {
    fn new(shared: StackShared, base: String) -> VerifiedpermissionsPolicyDefinitionElStaticElRef {
        VerifiedpermissionsPolicyDefinitionElStaticElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsPolicyDefinitionElStaticElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.statement", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedpermissionsPolicyDefinitionElTemplateLinkedElPrincipalEl {
    entity_id: PrimField<String>,
    entity_type: PrimField<String>,
}

impl VerifiedpermissionsPolicyDefinitionElTemplateLinkedElPrincipalEl {}

impl ToListMappable for VerifiedpermissionsPolicyDefinitionElTemplateLinkedElPrincipalEl {
    type O = BlockAssignable<VerifiedpermissionsPolicyDefinitionElTemplateLinkedElPrincipalEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsPolicyDefinitionElTemplateLinkedElPrincipalEl {
    #[doc = ""]
    pub entity_id: PrimField<String>,
    #[doc = ""]
    pub entity_type: PrimField<String>,
}

impl BuildVerifiedpermissionsPolicyDefinitionElTemplateLinkedElPrincipalEl {
    pub fn build(self) -> VerifiedpermissionsPolicyDefinitionElTemplateLinkedElPrincipalEl {
        VerifiedpermissionsPolicyDefinitionElTemplateLinkedElPrincipalEl {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
        }
    }
}

pub struct VerifiedpermissionsPolicyDefinitionElTemplateLinkedElPrincipalElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedpermissionsPolicyDefinitionElTemplateLinkedElPrincipalElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VerifiedpermissionsPolicyDefinitionElTemplateLinkedElPrincipalElRef {
        VerifiedpermissionsPolicyDefinitionElTemplateLinkedElPrincipalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsPolicyDefinitionElTemplateLinkedElPrincipalElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entity_id` after provisioning.\n"]
    pub fn entity_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entity_id", self.base))
    }

    #[doc = "Get a reference to the value of field `entity_type` after provisioning.\n"]
    pub fn entity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entity_type", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedpermissionsPolicyDefinitionElTemplateLinkedElResourceEl {
    entity_id: PrimField<String>,
    entity_type: PrimField<String>,
}

impl VerifiedpermissionsPolicyDefinitionElTemplateLinkedElResourceEl {}

impl ToListMappable for VerifiedpermissionsPolicyDefinitionElTemplateLinkedElResourceEl {
    type O = BlockAssignable<VerifiedpermissionsPolicyDefinitionElTemplateLinkedElResourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsPolicyDefinitionElTemplateLinkedElResourceEl {
    #[doc = ""]
    pub entity_id: PrimField<String>,
    #[doc = ""]
    pub entity_type: PrimField<String>,
}

impl BuildVerifiedpermissionsPolicyDefinitionElTemplateLinkedElResourceEl {
    pub fn build(self) -> VerifiedpermissionsPolicyDefinitionElTemplateLinkedElResourceEl {
        VerifiedpermissionsPolicyDefinitionElTemplateLinkedElResourceEl {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
        }
    }
}

pub struct VerifiedpermissionsPolicyDefinitionElTemplateLinkedElResourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedpermissionsPolicyDefinitionElTemplateLinkedElResourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VerifiedpermissionsPolicyDefinitionElTemplateLinkedElResourceElRef {
        VerifiedpermissionsPolicyDefinitionElTemplateLinkedElResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsPolicyDefinitionElTemplateLinkedElResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entity_id` after provisioning.\n"]
    pub fn entity_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entity_id", self.base))
    }

    #[doc = "Get a reference to the value of field `entity_type` after provisioning.\n"]
    pub fn entity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entity_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedpermissionsPolicyDefinitionElTemplateLinkedElDynamic {
    principal:
        Option<DynamicBlock<VerifiedpermissionsPolicyDefinitionElTemplateLinkedElPrincipalEl>>,
    resource: Option<DynamicBlock<VerifiedpermissionsPolicyDefinitionElTemplateLinkedElResourceEl>>,
}

#[derive(Serialize)]
pub struct VerifiedpermissionsPolicyDefinitionElTemplateLinkedEl {
    policy_template_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal: Option<Vec<VerifiedpermissionsPolicyDefinitionElTemplateLinkedElPrincipalEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<Vec<VerifiedpermissionsPolicyDefinitionElTemplateLinkedElResourceEl>>,
    dynamic: VerifiedpermissionsPolicyDefinitionElTemplateLinkedElDynamic,
}

impl VerifiedpermissionsPolicyDefinitionElTemplateLinkedEl {
    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        mut self,
        v: impl Into<BlockAssignable<VerifiedpermissionsPolicyDefinitionElTemplateLinkedElPrincipalEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.principal = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.principal = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(
        mut self,
        v: impl Into<BlockAssignable<VerifiedpermissionsPolicyDefinitionElTemplateLinkedElResourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for VerifiedpermissionsPolicyDefinitionElTemplateLinkedEl {
    type O = BlockAssignable<VerifiedpermissionsPolicyDefinitionElTemplateLinkedEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsPolicyDefinitionElTemplateLinkedEl {
    #[doc = ""]
    pub policy_template_id: PrimField<String>,
}

impl BuildVerifiedpermissionsPolicyDefinitionElTemplateLinkedEl {
    pub fn build(self) -> VerifiedpermissionsPolicyDefinitionElTemplateLinkedEl {
        VerifiedpermissionsPolicyDefinitionElTemplateLinkedEl {
            policy_template_id: self.policy_template_id,
            principal: core::default::Default::default(),
            resource: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct VerifiedpermissionsPolicyDefinitionElTemplateLinkedElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedpermissionsPolicyDefinitionElTemplateLinkedElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VerifiedpermissionsPolicyDefinitionElTemplateLinkedElRef {
        VerifiedpermissionsPolicyDefinitionElTemplateLinkedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsPolicyDefinitionElTemplateLinkedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `policy_template_id` after provisioning.\n"]
    pub fn policy_template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_template_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(
        &self,
    ) -> ListRef<VerifiedpermissionsPolicyDefinitionElTemplateLinkedElPrincipalElRef> {
        ListRef::new(self.shared().clone(), format!("{}.principal", self.base))
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(
        &self,
    ) -> ListRef<VerifiedpermissionsPolicyDefinitionElTemplateLinkedElResourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedpermissionsPolicyDefinitionElDynamic {
    static_: Option<DynamicBlock<VerifiedpermissionsPolicyDefinitionElStaticEl>>,
    template_linked: Option<DynamicBlock<VerifiedpermissionsPolicyDefinitionElTemplateLinkedEl>>,
}

#[derive(Serialize)]
pub struct VerifiedpermissionsPolicyDefinitionEl {
    #[serde(rename = "static", skip_serializing_if = "Option::is_none")]
    static_: Option<Vec<VerifiedpermissionsPolicyDefinitionElStaticEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_linked: Option<Vec<VerifiedpermissionsPolicyDefinitionElTemplateLinkedEl>>,
    dynamic: VerifiedpermissionsPolicyDefinitionElDynamic,
}

impl VerifiedpermissionsPolicyDefinitionEl {
    #[doc = "Set the field `static_`.\n"]
    pub fn set_static(
        mut self,
        v: impl Into<BlockAssignable<VerifiedpermissionsPolicyDefinitionElStaticEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.static_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.static_ = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `template_linked`.\n"]
    pub fn set_template_linked(
        mut self,
        v: impl Into<BlockAssignable<VerifiedpermissionsPolicyDefinitionElTemplateLinkedEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.template_linked = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.template_linked = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for VerifiedpermissionsPolicyDefinitionEl {
    type O = BlockAssignable<VerifiedpermissionsPolicyDefinitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsPolicyDefinitionEl {}

impl BuildVerifiedpermissionsPolicyDefinitionEl {
    pub fn build(self) -> VerifiedpermissionsPolicyDefinitionEl {
        VerifiedpermissionsPolicyDefinitionEl {
            static_: core::default::Default::default(),
            template_linked: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct VerifiedpermissionsPolicyDefinitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedpermissionsPolicyDefinitionElRef {
    fn new(shared: StackShared, base: String) -> VerifiedpermissionsPolicyDefinitionElRef {
        VerifiedpermissionsPolicyDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsPolicyDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `static_` after provisioning.\n"]
    pub fn static_(&self) -> ListRef<VerifiedpermissionsPolicyDefinitionElStaticElRef> {
        ListRef::new(self.shared().clone(), format!("{}.static", self.base))
    }

    #[doc = "Get a reference to the value of field `template_linked` after provisioning.\n"]
    pub fn template_linked(
        &self,
    ) -> ListRef<VerifiedpermissionsPolicyDefinitionElTemplateLinkedElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.template_linked", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct VerifiedpermissionsPolicyDynamic {
    definition: Option<DynamicBlock<VerifiedpermissionsPolicyDefinitionEl>>,
}
