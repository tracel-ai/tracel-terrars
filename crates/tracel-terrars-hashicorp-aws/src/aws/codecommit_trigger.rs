use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodecommitTriggerData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    repository_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger: Option<Vec<CodecommitTriggerTriggerEl>>,
    dynamic: CodecommitTriggerDynamic,
}

struct CodecommitTrigger_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CodecommitTriggerData>,
}

#[derive(Clone)]
pub struct CodecommitTrigger(Rc<CodecommitTrigger_>);

impl CodecommitTrigger {
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

    #[doc = "Set the field `trigger`.\n"]
    pub fn set_trigger(self, v: impl Into<BlockAssignable<CodecommitTriggerTriggerEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().trigger = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.trigger = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `configuration_id` after provisioning.\n"]
    pub fn configuration_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.extract_ref()))
    }
}

impl Referable for CodecommitTrigger {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CodecommitTrigger { }

impl ToListMappable for CodecommitTrigger {
    type O = ListRef<CodecommitTriggerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CodecommitTrigger_ {
    fn extract_resource_type(&self) -> String {
        "aws_codecommit_trigger".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodecommitTrigger {
    pub tf_id: String,
    #[doc = ""]
    pub repository_name: PrimField<String>,
}

impl BuildCodecommitTrigger {
    pub fn build(self, stack: &mut Stack) -> CodecommitTrigger {
        let out = CodecommitTrigger(Rc::new(CodecommitTrigger_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CodecommitTriggerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                repository_name: self.repository_name,
                trigger: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodecommitTriggerRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodecommitTriggerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CodecommitTriggerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration_id` after provisioning.\n"]
    pub fn configuration_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CodecommitTriggerTriggerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branches: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_data: Option<PrimField<String>>,
    destination_arn: PrimField<String>,
    events: ListField<PrimField<String>>,
    name: PrimField<String>,
}

impl CodecommitTriggerTriggerEl {
    #[doc = "Set the field `branches`.\n"]
    pub fn set_branches(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.branches = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_data`.\n"]
    pub fn set_custom_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_data = Some(v.into());
        self
    }
}

impl ToListMappable for CodecommitTriggerTriggerEl {
    type O = BlockAssignable<CodecommitTriggerTriggerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodecommitTriggerTriggerEl {
    #[doc = ""]
    pub destination_arn: PrimField<String>,
    #[doc = ""]
    pub events: ListField<PrimField<String>>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCodecommitTriggerTriggerEl {
    pub fn build(self) -> CodecommitTriggerTriggerEl {
        CodecommitTriggerTriggerEl {
            branches: core::default::Default::default(),
            custom_data: core::default::Default::default(),
            destination_arn: self.destination_arn,
            events: self.events,
            name: self.name,
        }
    }
}

pub struct CodecommitTriggerTriggerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodecommitTriggerTriggerElRef {
    fn new(shared: StackShared, base: String) -> CodecommitTriggerTriggerElRef {
        CodecommitTriggerTriggerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodecommitTriggerTriggerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branches` after provisioning.\n"]
    pub fn branches(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.branches", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_data` after provisioning.\n"]
    pub fn custom_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_data", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.events", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodecommitTriggerDynamic {
    trigger: Option<DynamicBlock<CodecommitTriggerTriggerEl>>,
}
