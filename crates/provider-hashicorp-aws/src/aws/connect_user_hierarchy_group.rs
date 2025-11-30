use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ConnectUserHierarchyGroupData {
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
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
}

struct ConnectUserHierarchyGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConnectUserHierarchyGroupData>,
}

#[derive(Clone)]
pub struct ConnectUserHierarchyGroup(Rc<ConnectUserHierarchyGroup_>);

impl ConnectUserHierarchyGroup {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `parent_group_id`.\n"]
    pub fn set_parent_group_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent_group_id = Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchy_group_id` after provisioning.\n"]
    pub fn hierarchy_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hierarchy_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hierarchy_path` after provisioning.\n"]
    pub fn hierarchy_path(&self) -> ListRef<ConnectUserHierarchyGroupHierarchyPathElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hierarchy_path", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `level_id` after provisioning.\n"]
    pub fn level_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.level_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parent_group_id` after provisioning.\n"]
    pub fn parent_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_group_id", self.extract_ref()),
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

impl Referable for ConnectUserHierarchyGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for ConnectUserHierarchyGroup {}

impl ToListMappable for ConnectUserHierarchyGroup {
    type O = ListRef<ConnectUserHierarchyGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ConnectUserHierarchyGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_connect_user_hierarchy_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConnectUserHierarchyGroup {
    pub tf_id: String,
    #[doc = ""]
    pub instance_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildConnectUserHierarchyGroup {
    pub fn build(self, stack: &mut Stack) -> ConnectUserHierarchyGroup {
        let out = ConnectUserHierarchyGroup(Rc::new(ConnectUserHierarchyGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConnectUserHierarchyGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: self.name,
                parent_group_id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConnectUserHierarchyGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectUserHierarchyGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl ConnectUserHierarchyGroupRef {
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

    #[doc = "Get a reference to the value of field `hierarchy_group_id` after provisioning.\n"]
    pub fn hierarchy_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hierarchy_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `hierarchy_path` after provisioning.\n"]
    pub fn hierarchy_path(&self) -> ListRef<ConnectUserHierarchyGroupHierarchyPathElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hierarchy_path", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `level_id` after provisioning.\n"]
    pub fn level_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.level_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parent_group_id` after provisioning.\n"]
    pub fn parent_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_group_id", self.extract_ref()),
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
pub struct ConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for ConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
    type O = BlockAssignable<ConnectUserHierarchyGroupHierarchyPathElLevelFiveEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {}

impl BuildConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
    pub fn build(self) -> ConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
        ConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct ConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
        ConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
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
pub struct ConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for ConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
    type O = BlockAssignable<ConnectUserHierarchyGroupHierarchyPathElLevelFourEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroupHierarchyPathElLevelFourEl {}

impl BuildConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
    pub fn build(self) -> ConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
        ConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct ConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
        ConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
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
pub struct ConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for ConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
    type O = BlockAssignable<ConnectUserHierarchyGroupHierarchyPathElLevelOneEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroupHierarchyPathElLevelOneEl {}

impl BuildConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
    pub fn build(self) -> ConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
        ConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct ConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
        ConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
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
pub struct ConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for ConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
    type O = BlockAssignable<ConnectUserHierarchyGroupHierarchyPathElLevelThreeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {}

impl BuildConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
    pub fn build(self) -> ConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
        ConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct ConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
        ConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
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
pub struct ConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for ConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
    type O = BlockAssignable<ConnectUserHierarchyGroupHierarchyPathElLevelTwoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {}

impl BuildConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
    pub fn build(self) -> ConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
        ConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct ConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
        ConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
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
pub struct ConnectUserHierarchyGroupHierarchyPathEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    level_five: Option<ListField<ConnectUserHierarchyGroupHierarchyPathElLevelFiveEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level_four: Option<ListField<ConnectUserHierarchyGroupHierarchyPathElLevelFourEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level_one: Option<ListField<ConnectUserHierarchyGroupHierarchyPathElLevelOneEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level_three: Option<ListField<ConnectUserHierarchyGroupHierarchyPathElLevelThreeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level_two: Option<ListField<ConnectUserHierarchyGroupHierarchyPathElLevelTwoEl>>,
}

impl ConnectUserHierarchyGroupHierarchyPathEl {
    #[doc = "Set the field `level_five`.\n"]
    pub fn set_level_five(
        mut self,
        v: impl Into<ListField<ConnectUserHierarchyGroupHierarchyPathElLevelFiveEl>>,
    ) -> Self {
        self.level_five = Some(v.into());
        self
    }

    #[doc = "Set the field `level_four`.\n"]
    pub fn set_level_four(
        mut self,
        v: impl Into<ListField<ConnectUserHierarchyGroupHierarchyPathElLevelFourEl>>,
    ) -> Self {
        self.level_four = Some(v.into());
        self
    }

    #[doc = "Set the field `level_one`.\n"]
    pub fn set_level_one(
        mut self,
        v: impl Into<ListField<ConnectUserHierarchyGroupHierarchyPathElLevelOneEl>>,
    ) -> Self {
        self.level_one = Some(v.into());
        self
    }

    #[doc = "Set the field `level_three`.\n"]
    pub fn set_level_three(
        mut self,
        v: impl Into<ListField<ConnectUserHierarchyGroupHierarchyPathElLevelThreeEl>>,
    ) -> Self {
        self.level_three = Some(v.into());
        self
    }

    #[doc = "Set the field `level_two`.\n"]
    pub fn set_level_two(
        mut self,
        v: impl Into<ListField<ConnectUserHierarchyGroupHierarchyPathElLevelTwoEl>>,
    ) -> Self {
        self.level_two = Some(v.into());
        self
    }
}

impl ToListMappable for ConnectUserHierarchyGroupHierarchyPathEl {
    type O = BlockAssignable<ConnectUserHierarchyGroupHierarchyPathEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectUserHierarchyGroupHierarchyPathEl {}

impl BuildConnectUserHierarchyGroupHierarchyPathEl {
    pub fn build(self) -> ConnectUserHierarchyGroupHierarchyPathEl {
        ConnectUserHierarchyGroupHierarchyPathEl {
            level_five: core::default::Default::default(),
            level_four: core::default::Default::default(),
            level_one: core::default::Default::default(),
            level_three: core::default::Default::default(),
            level_two: core::default::Default::default(),
        }
    }
}

pub struct ConnectUserHierarchyGroupHierarchyPathElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectUserHierarchyGroupHierarchyPathElRef {
    fn new(shared: StackShared, base: String) -> ConnectUserHierarchyGroupHierarchyPathElRef {
        ConnectUserHierarchyGroupHierarchyPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectUserHierarchyGroupHierarchyPathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `level_five` after provisioning.\n"]
    pub fn level_five(&self) -> ListRef<ConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_five", self.base))
    }

    #[doc = "Get a reference to the value of field `level_four` after provisioning.\n"]
    pub fn level_four(&self) -> ListRef<ConnectUserHierarchyGroupHierarchyPathElLevelFourElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_four", self.base))
    }

    #[doc = "Get a reference to the value of field `level_one` after provisioning.\n"]
    pub fn level_one(&self) -> ListRef<ConnectUserHierarchyGroupHierarchyPathElLevelOneElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_one", self.base))
    }

    #[doc = "Get a reference to the value of field `level_three` after provisioning.\n"]
    pub fn level_three(&self) -> ListRef<ConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_three", self.base))
    }

    #[doc = "Get a reference to the value of field `level_two` after provisioning.\n"]
    pub fn level_two(&self) -> ListRef<ConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_two", self.base))
    }
}
