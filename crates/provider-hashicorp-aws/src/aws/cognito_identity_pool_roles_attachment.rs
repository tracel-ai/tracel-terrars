use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CognitoIdentityPoolRolesAttachmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    identity_pool_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    roles: RecField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_mapping: Option<Vec<CognitoIdentityPoolRolesAttachmentRoleMappingEl>>,
    dynamic: CognitoIdentityPoolRolesAttachmentDynamic,
}

struct CognitoIdentityPoolRolesAttachment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CognitoIdentityPoolRolesAttachmentData>,
}

#[derive(Clone)]
pub struct CognitoIdentityPoolRolesAttachment(Rc<CognitoIdentityPoolRolesAttachment_>);

impl CognitoIdentityPoolRolesAttachment {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `role_mapping`.\n"]
    pub fn set_role_mapping(
        self,
        v: impl Into<BlockAssignable<CognitoIdentityPoolRolesAttachmentRoleMappingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().role_mapping = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.role_mapping = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_pool_id` after provisioning.\n"]
    pub fn identity_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_pool_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `roles` after provisioning.\n"]
    pub fn roles(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.roles", self.extract_ref()))
    }
}

impl Referable for CognitoIdentityPoolRolesAttachment {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CognitoIdentityPoolRolesAttachment { }

impl ToListMappable for CognitoIdentityPoolRolesAttachment {
    type O = ListRef<CognitoIdentityPoolRolesAttachmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CognitoIdentityPoolRolesAttachment_ {
    fn extract_resource_type(&self) -> String {
        "aws_cognito_identity_pool_roles_attachment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCognitoIdentityPoolRolesAttachment {
    pub tf_id: String,
    #[doc = ""]
    pub identity_pool_id: PrimField<String>,
    #[doc = ""]
    pub roles: RecField<PrimField<String>>,
}

impl BuildCognitoIdentityPoolRolesAttachment {
    pub fn build(self, stack: &mut Stack) -> CognitoIdentityPoolRolesAttachment {
        let out = CognitoIdentityPoolRolesAttachment(Rc::new(CognitoIdentityPoolRolesAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CognitoIdentityPoolRolesAttachmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                identity_pool_id: self.identity_pool_id,
                region: core::default::Default::default(),
                roles: self.roles,
                role_mapping: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CognitoIdentityPoolRolesAttachmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoIdentityPoolRolesAttachmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CognitoIdentityPoolRolesAttachmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_pool_id` after provisioning.\n"]
    pub fn identity_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_pool_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `roles` after provisioning.\n"]
    pub fn roles(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.roles", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl {
    claim: PrimField<String>,
    match_type: PrimField<String>,
    role_arn: PrimField<String>,
    value: PrimField<String>,
}

impl CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl { }

impl ToListMappable for CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl {
    type O = BlockAssignable<CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl {
    #[doc = ""]
    pub claim: PrimField<String>,
    #[doc = ""]
    pub match_type: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildCognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl {
    pub fn build(self) -> CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl {
        CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl {
            claim: self.claim,
            match_type: self.match_type,
            role_arn: self.role_arn,
            value: self.value,
        }
    }
}

pub struct CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleElRef {
    fn new(shared: StackShared, base: String) -> CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleElRef {
        CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `claim` after provisioning.\n"]
    pub fn claim(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.claim", self.base))
    }

    #[doc = "Get a reference to the value of field `match_type` after provisioning.\n"]
    pub fn match_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.match_type", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoIdentityPoolRolesAttachmentRoleMappingElDynamic {
    mapping_rule: Option<DynamicBlock<CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl>>,
}

#[derive(Serialize)]
pub struct CognitoIdentityPoolRolesAttachmentRoleMappingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ambiguous_role_resolution: Option<PrimField<String>>,
    identity_provider: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mapping_rule: Option<Vec<CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl>>,
    dynamic: CognitoIdentityPoolRolesAttachmentRoleMappingElDynamic,
}

impl CognitoIdentityPoolRolesAttachmentRoleMappingEl {
    #[doc = "Set the field `ambiguous_role_resolution`.\n"]
    pub fn set_ambiguous_role_resolution(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ambiguous_role_resolution = Some(v.into());
        self
    }

    #[doc = "Set the field `mapping_rule`.\n"]
    pub fn set_mapping_rule(
        mut self,
        v: impl Into<BlockAssignable<CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mapping_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mapping_rule = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CognitoIdentityPoolRolesAttachmentRoleMappingEl {
    type O = BlockAssignable<CognitoIdentityPoolRolesAttachmentRoleMappingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoIdentityPoolRolesAttachmentRoleMappingEl {
    #[doc = ""]
    pub identity_provider: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildCognitoIdentityPoolRolesAttachmentRoleMappingEl {
    pub fn build(self) -> CognitoIdentityPoolRolesAttachmentRoleMappingEl {
        CognitoIdentityPoolRolesAttachmentRoleMappingEl {
            ambiguous_role_resolution: core::default::Default::default(),
            identity_provider: self.identity_provider,
            type_: self.type_,
            mapping_rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CognitoIdentityPoolRolesAttachmentRoleMappingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CognitoIdentityPoolRolesAttachmentRoleMappingElRef {
    fn new(shared: StackShared, base: String) -> CognitoIdentityPoolRolesAttachmentRoleMappingElRef {
        CognitoIdentityPoolRolesAttachmentRoleMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoIdentityPoolRolesAttachmentRoleMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ambiguous_role_resolution` after provisioning.\n"]
    pub fn ambiguous_role_resolution(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ambiguous_role_resolution", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_provider` after provisioning.\n"]
    pub fn identity_provider(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `mapping_rule` after provisioning.\n"]
    pub fn mapping_rule(&self) -> ListRef<CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mapping_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoIdentityPoolRolesAttachmentDynamic {
    role_mapping: Option<DynamicBlock<CognitoIdentityPoolRolesAttachmentRoleMappingEl>>,
}
