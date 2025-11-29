use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct KmsGrantData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grant_creation_tokens: Option<SetField<PrimField<String>>>,
    grantee_principal: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    key_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    operations: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retire_on_delete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retiring_principal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    constraints: Option<Vec<KmsGrantConstraintsEl>>,
    dynamic: KmsGrantDynamic,
}

struct KmsGrant_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<KmsGrantData>,
}

#[derive(Clone)]
pub struct KmsGrant(Rc<KmsGrant_>);

impl KmsGrant {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `grant_creation_tokens`.\n"]
    pub fn set_grant_creation_tokens(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().grant_creation_tokens = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `retire_on_delete`.\n"]
    pub fn set_retire_on_delete(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().retire_on_delete = Some(v.into());
        self
    }

    #[doc = "Set the field `retiring_principal`.\n"]
    pub fn set_retiring_principal(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().retiring_principal = Some(v.into());
        self
    }

    #[doc = "Set the field `constraints`.\n"]
    pub fn set_constraints(self, v: impl Into<BlockAssignable<KmsGrantConstraintsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().constraints = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.constraints = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `grant_creation_tokens` after provisioning.\n"]
    pub fn grant_creation_tokens(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.grant_creation_tokens", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_id` after provisioning.\n"]
    pub fn grant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grant_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_token` after provisioning.\n"]
    pub fn grant_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grant_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grantee_principal` after provisioning.\n"]
    pub fn grantee_principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grantee_principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.operations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retire_on_delete` after provisioning.\n"]
    pub fn retire_on_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.retire_on_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retiring_principal` after provisioning.\n"]
    pub fn retiring_principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.retiring_principal", self.extract_ref()))
    }
}

impl Referable for KmsGrant {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for KmsGrant { }

impl ToListMappable for KmsGrant {
    type O = ListRef<KmsGrantRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for KmsGrant_ {
    fn extract_resource_type(&self) -> String {
        "aws_kms_grant".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKmsGrant {
    pub tf_id: String,
    #[doc = ""]
    pub grantee_principal: PrimField<String>,
    #[doc = ""]
    pub key_id: PrimField<String>,
    #[doc = ""]
    pub operations: SetField<PrimField<String>>,
}

impl BuildKmsGrant {
    pub fn build(self, stack: &mut Stack) -> KmsGrant {
        let out = KmsGrant(Rc::new(KmsGrant_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(KmsGrantData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                grant_creation_tokens: core::default::Default::default(),
                grantee_principal: self.grantee_principal,
                id: core::default::Default::default(),
                key_id: self.key_id,
                name: core::default::Default::default(),
                operations: self.operations,
                region: core::default::Default::default(),
                retire_on_delete: core::default::Default::default(),
                retiring_principal: core::default::Default::default(),
                constraints: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KmsGrantRef {
    shared: StackShared,
    base: String,
}

impl Ref for KmsGrantRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl KmsGrantRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grant_creation_tokens` after provisioning.\n"]
    pub fn grant_creation_tokens(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.grant_creation_tokens", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_id` after provisioning.\n"]
    pub fn grant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grant_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_token` after provisioning.\n"]
    pub fn grant_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grant_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grantee_principal` after provisioning.\n"]
    pub fn grantee_principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grantee_principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.operations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retire_on_delete` after provisioning.\n"]
    pub fn retire_on_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.retire_on_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retiring_principal` after provisioning.\n"]
    pub fn retiring_principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.retiring_principal", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct KmsGrantConstraintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_context_equals: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_context_subset: Option<RecField<PrimField<String>>>,
}

impl KmsGrantConstraintsEl {
    #[doc = "Set the field `encryption_context_equals`.\n"]
    pub fn set_encryption_context_equals(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.encryption_context_equals = Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_context_subset`.\n"]
    pub fn set_encryption_context_subset(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.encryption_context_subset = Some(v.into());
        self
    }
}

impl ToListMappable for KmsGrantConstraintsEl {
    type O = BlockAssignable<KmsGrantConstraintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKmsGrantConstraintsEl {}

impl BuildKmsGrantConstraintsEl {
    pub fn build(self) -> KmsGrantConstraintsEl {
        KmsGrantConstraintsEl {
            encryption_context_equals: core::default::Default::default(),
            encryption_context_subset: core::default::Default::default(),
        }
    }
}

pub struct KmsGrantConstraintsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KmsGrantConstraintsElRef {
    fn new(shared: StackShared, base: String) -> KmsGrantConstraintsElRef {
        KmsGrantConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KmsGrantConstraintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_context_equals` after provisioning.\n"]
    pub fn encryption_context_equals(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.encryption_context_equals", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_context_subset` after provisioning.\n"]
    pub fn encryption_context_subset(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.encryption_context_subset", self.base))
    }
}

#[derive(Serialize, Default)]
struct KmsGrantDynamic {
    constraints: Option<DynamicBlock<KmsGrantConstraintsEl>>,
}
