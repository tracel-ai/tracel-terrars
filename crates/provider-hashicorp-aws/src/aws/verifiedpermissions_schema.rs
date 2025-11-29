use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VerifiedpermissionsSchemaData {
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
    definition: Option<Vec<VerifiedpermissionsSchemaDefinitionEl>>,
    dynamic: VerifiedpermissionsSchemaDynamic,
}

struct VerifiedpermissionsSchema_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VerifiedpermissionsSchemaData>,
}

#[derive(Clone)]
pub struct VerifiedpermissionsSchema(Rc<VerifiedpermissionsSchema_>);

impl VerifiedpermissionsSchema {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `definition`.\n"]
    pub fn set_definition(self, v: impl Into<BlockAssignable<VerifiedpermissionsSchemaDefinitionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().definition = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.definition = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespaces` after provisioning.\n"]
    pub fn namespaces(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.namespaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_store_id` after provisioning.\n"]
    pub fn policy_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_store_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> ListRef<VerifiedpermissionsSchemaDefinitionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.definition", self.extract_ref()))
    }
}

impl Referable for VerifiedpermissionsSchema {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for VerifiedpermissionsSchema { }

impl ToListMappable for VerifiedpermissionsSchema {
    type O = ListRef<VerifiedpermissionsSchemaRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VerifiedpermissionsSchema_ {
    fn extract_resource_type(&self) -> String {
        "aws_verifiedpermissions_schema".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVerifiedpermissionsSchema {
    pub tf_id: String,
    #[doc = ""]
    pub policy_store_id: PrimField<String>,
}

impl BuildVerifiedpermissionsSchema {
    pub fn build(self, stack: &mut Stack) -> VerifiedpermissionsSchema {
        let out = VerifiedpermissionsSchema(Rc::new(VerifiedpermissionsSchema_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VerifiedpermissionsSchemaData {
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

pub struct VerifiedpermissionsSchemaRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedpermissionsSchemaRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl VerifiedpermissionsSchemaRef {
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

    #[doc = "Get a reference to the value of field `namespaces` after provisioning.\n"]
    pub fn namespaces(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.namespaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_store_id` after provisioning.\n"]
    pub fn policy_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_store_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> ListRef<VerifiedpermissionsSchemaDefinitionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.definition", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct VerifiedpermissionsSchemaDefinitionEl {
    value: PrimField<String>,
}

impl VerifiedpermissionsSchemaDefinitionEl { }

impl ToListMappable for VerifiedpermissionsSchemaDefinitionEl {
    type O = BlockAssignable<VerifiedpermissionsSchemaDefinitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsSchemaDefinitionEl {
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildVerifiedpermissionsSchemaDefinitionEl {
    pub fn build(self) -> VerifiedpermissionsSchemaDefinitionEl {
        VerifiedpermissionsSchemaDefinitionEl { value: self.value }
    }
}

pub struct VerifiedpermissionsSchemaDefinitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VerifiedpermissionsSchemaDefinitionElRef {
    fn new(shared: StackShared, base: String) -> VerifiedpermissionsSchemaDefinitionElRef {
        VerifiedpermissionsSchemaDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsSchemaDefinitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedpermissionsSchemaDynamic {
    definition: Option<DynamicBlock<VerifiedpermissionsSchemaDefinitionEl>>,
}
