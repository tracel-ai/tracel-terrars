use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConnectUserHierarchyStructureData {
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
    instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hierarchy_structure: Option<Vec<ConnectUserHierarchyStructureHierarchyStructureEl>>,
    dynamic: ConnectUserHierarchyStructureDynamic,
}

struct ConnectUserHierarchyStructure_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConnectUserHierarchyStructureData>,
}

#[derive(Clone)]
pub struct ConnectUserHierarchyStructure(Rc<ConnectUserHierarchyStructure_>);

impl ConnectUserHierarchyStructure {
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

    #[doc = "Set the field `hierarchy_structure`.\n"]
    pub fn set_hierarchy_structure(
        self,
        v: impl Into<BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hierarchy_structure = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hierarchy_structure = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchy_structure` after provisioning.\n"]
    pub fn hierarchy_structure(&self) -> ListRef<ConnectUserHierarchyStructureHierarchyStructureElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hierarchy_structure", self.extract_ref()))
    }
}

impl Referable for ConnectUserHierarchyStructure {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ConnectUserHierarchyStructure { }

impl ToListMappable for ConnectUserHierarchyStructure {
    type O = ListRef<ConnectUserHierarchyStructureRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ConnectUserHierarchyStructure_ {
    fn extract_resource_type(&self) -> String {
        "aws_connect_user_hierarchy_structure".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConnectUserHierarchyStructure {
    pub tf_id: String,
    #[doc = ""]
    pub instance_id: PrimField<String>,
}

impl BuildConnectUserHierarchyStructure {
    pub fn build(self, stack: &mut Stack) -> ConnectUserHierarchyStructure {
        let out = ConnectUserHierarchyStructure(Rc::new(ConnectUserHierarchyStructure_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConnectUserHierarchyStructureData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                region: core::default::Default::default(),
                hierarchy_structure: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConnectUserHierarchyStructureRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectUserHierarchyStructureRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl ConnectUserHierarchyStructureRef {
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

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchy_structure` after provisioning.\n"]
    pub fn hierarchy_structure(&self) -> ListRef<ConnectUserHierarchyStructureHierarchyStructureElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hierarchy_structure", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
    name: PrimField<String>,
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl { }

impl ToListMappable for ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
    type O = BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
    pub fn build(self) -> ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
        ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl { name: self.name }
    }
}

pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
    fn new(shared: StackShared, base: String) -> ConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
        ConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
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

#[derive(Serialize)]
pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
    name: PrimField<String>,
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl { }

impl ToListMappable for ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
    type O = BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
    pub fn build(self) -> ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
        ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl { name: self.name }
    }
}

pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
    fn new(shared: StackShared, base: String) -> ConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
        ConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
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

#[derive(Serialize)]
pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
    name: PrimField<String>,
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl { }

impl ToListMappable for ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
    type O = BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
    pub fn build(self) -> ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
        ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl { name: self.name }
    }
}

pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
    fn new(shared: StackShared, base: String) -> ConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
        ConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
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

#[derive(Serialize)]
pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
    name: PrimField<String>,
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl { }

impl ToListMappable for ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
    type O = BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
    pub fn build(self) -> ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
        ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl { name: self.name }
    }
}

pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
    fn new(shared: StackShared, base: String) -> ConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
        ConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
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

#[derive(Serialize)]
pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
    name: PrimField<String>,
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl { }

impl ToListMappable for ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
    type O = BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
    pub fn build(self) -> ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
        ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl { name: self.name }
    }
}

pub struct ConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
    fn new(shared: StackShared, base: String) -> ConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
        ConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
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
struct ConnectUserHierarchyStructureHierarchyStructureElDynamic {
    level_five: Option<DynamicBlock<ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl>>,
    level_four: Option<DynamicBlock<ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl>>,
    level_one: Option<DynamicBlock<ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl>>,
    level_three: Option<DynamicBlock<ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl>>,
    level_two: Option<DynamicBlock<ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl>>,
}

#[derive(Serialize)]
pub struct ConnectUserHierarchyStructureHierarchyStructureEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    level_five: Option<Vec<ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level_four: Option<Vec<ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level_one: Option<Vec<ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level_three: Option<Vec<ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level_two: Option<Vec<ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl>>,
    dynamic: ConnectUserHierarchyStructureHierarchyStructureElDynamic,
}

impl ConnectUserHierarchyStructureHierarchyStructureEl {
    #[doc = "Set the field `level_five`.\n"]
    pub fn set_level_five(
        mut self,
        v: impl Into<BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.level_five = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.level_five = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `level_four`.\n"]
    pub fn set_level_four(
        mut self,
        v: impl Into<BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelFourEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.level_four = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.level_four = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `level_one`.\n"]
    pub fn set_level_one(
        mut self,
        v: impl Into<BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelOneEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.level_one = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.level_one = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `level_three`.\n"]
    pub fn set_level_three(
        mut self,
        v: impl Into<BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.level_three = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.level_three = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `level_two`.\n"]
    pub fn set_level_two(
        mut self,
        v: impl Into<BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.level_two = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.level_two = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ConnectUserHierarchyStructureHierarchyStructureEl {
    type O = BlockAssignable<ConnectUserHierarchyStructureHierarchyStructureEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyStructureHierarchyStructureEl {}

impl BuildConnectUserHierarchyStructureHierarchyStructureEl {
    pub fn build(self) -> ConnectUserHierarchyStructureHierarchyStructureEl {
        ConnectUserHierarchyStructureHierarchyStructureEl {
            level_five: core::default::Default::default(),
            level_four: core::default::Default::default(),
            level_one: core::default::Default::default(),
            level_three: core::default::Default::default(),
            level_two: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ConnectUserHierarchyStructureHierarchyStructureElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectUserHierarchyStructureHierarchyStructureElRef {
    fn new(shared: StackShared, base: String) -> ConnectUserHierarchyStructureHierarchyStructureElRef {
        ConnectUserHierarchyStructureHierarchyStructureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyStructureHierarchyStructureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `level_five` after provisioning.\n"]
    pub fn level_five(&self) -> ListRef<ConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_five", self.base))
    }

    #[doc = "Get a reference to the value of field `level_four` after provisioning.\n"]
    pub fn level_four(&self) -> ListRef<ConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_four", self.base))
    }

    #[doc = "Get a reference to the value of field `level_one` after provisioning.\n"]
    pub fn level_one(&self) -> ListRef<ConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_one", self.base))
    }

    #[doc = "Get a reference to the value of field `level_three` after provisioning.\n"]
    pub fn level_three(&self) -> ListRef<ConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_three", self.base))
    }

    #[doc = "Get a reference to the value of field `level_two` after provisioning.\n"]
    pub fn level_two(&self) -> ListRef<ConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_two", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConnectUserHierarchyStructureDynamic {
    hierarchy_structure: Option<DynamicBlock<ConnectUserHierarchyStructureHierarchyStructureEl>>,
}
