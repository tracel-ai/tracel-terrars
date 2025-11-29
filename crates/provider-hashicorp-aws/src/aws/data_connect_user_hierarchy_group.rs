use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataConnectUserHierarchyGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hierarchy_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataConnectUserHierarchyGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataConnectUserHierarchyGroupData>,
}

#[derive(Clone)]
pub struct DataConnectUserHierarchyGroup(Rc<DataConnectUserHierarchyGroup_>);

impl DataConnectUserHierarchyGroup {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `hierarchy_group_id`.\n"]
    pub fn set_hierarchy_group_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().hierarchy_group_id = Some(v.into());
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchy_group_id` after provisioning.\n"]
    pub fn hierarchy_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hierarchy_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchy_path` after provisioning.\n"]
    pub fn hierarchy_path(&self) -> ListRef<DataConnectUserHierarchyGroupHierarchyPathElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hierarchy_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `level_id` after provisioning.\n"]
    pub fn level_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.level_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Referable for DataConnectUserHierarchyGroup {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataConnectUserHierarchyGroup { }

impl ToListMappable for DataConnectUserHierarchyGroup {
    type O = ListRef<DataConnectUserHierarchyGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataConnectUserHierarchyGroup_ {
    fn extract_datasource_type(&self) -> String {
        "aws_connect_user_hierarchy_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataConnectUserHierarchyGroup {
    pub tf_id: String,
    #[doc = ""]
    pub instance_id: PrimField<String>,
}

impl BuildDataConnectUserHierarchyGroup {
    pub fn build(self, stack: &mut Stack) -> DataConnectUserHierarchyGroup {
        let out = DataConnectUserHierarchyGroup(Rc::new(DataConnectUserHierarchyGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataConnectUserHierarchyGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                hierarchy_group_id: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataConnectUserHierarchyGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectUserHierarchyGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataConnectUserHierarchyGroupRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchy_group_id` after provisioning.\n"]
    pub fn hierarchy_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hierarchy_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hierarchy_path` after provisioning.\n"]
    pub fn hierarchy_path(&self) -> ListRef<DataConnectUserHierarchyGroupHierarchyPathElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hierarchy_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `level_id` after provisioning.\n"]
    pub fn level_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.level_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
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

impl ToListMappable for DataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
    type O = BlockAssignable<DataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {}

impl BuildDataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
    pub fn build(self) -> DataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
        DataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
    fn new(shared: StackShared, base: String) -> DataConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
        DataConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef {
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
pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
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

impl ToListMappable for DataConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
    type O = BlockAssignable<DataConnectUserHierarchyGroupHierarchyPathElLevelFourEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyGroupHierarchyPathElLevelFourEl {}

impl BuildDataConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
    pub fn build(self) -> DataConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
        DataConnectUserHierarchyGroupHierarchyPathElLevelFourEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
    fn new(shared: StackShared, base: String) -> DataConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
        DataConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelFourElRef {
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
pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
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

impl ToListMappable for DataConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
    type O = BlockAssignable<DataConnectUserHierarchyGroupHierarchyPathElLevelOneEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyGroupHierarchyPathElLevelOneEl {}

impl BuildDataConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
    pub fn build(self) -> DataConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
        DataConnectUserHierarchyGroupHierarchyPathElLevelOneEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
    fn new(shared: StackShared, base: String) -> DataConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
        DataConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelOneElRef {
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
pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
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

impl ToListMappable for DataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
    type O = BlockAssignable<DataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {}

impl BuildDataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
    pub fn build(self) -> DataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
        DataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
    fn new(shared: StackShared, base: String) -> DataConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
        DataConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef {
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
pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
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

impl ToListMappable for DataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
    type O = BlockAssignable<DataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {}

impl BuildDataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
    pub fn build(self) -> DataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
        DataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
    fn new(shared: StackShared, base: String) -> DataConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
        DataConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef {
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
pub struct DataConnectUserHierarchyGroupHierarchyPathEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    level_five: Option<ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level_four: Option<ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelFourEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level_one: Option<ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelOneEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level_three: Option<ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level_two: Option<ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl>>,
}

impl DataConnectUserHierarchyGroupHierarchyPathEl {
    #[doc = "Set the field `level_five`.\n"]
    pub fn set_level_five(
        mut self,
        v: impl Into<ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelFiveEl>>,
    ) -> Self {
        self.level_five = Some(v.into());
        self
    }

    #[doc = "Set the field `level_four`.\n"]
    pub fn set_level_four(
        mut self,
        v: impl Into<ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelFourEl>>,
    ) -> Self {
        self.level_four = Some(v.into());
        self
    }

    #[doc = "Set the field `level_one`.\n"]
    pub fn set_level_one(
        mut self,
        v: impl Into<ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelOneEl>>,
    ) -> Self {
        self.level_one = Some(v.into());
        self
    }

    #[doc = "Set the field `level_three`.\n"]
    pub fn set_level_three(
        mut self,
        v: impl Into<ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelThreeEl>>,
    ) -> Self {
        self.level_three = Some(v.into());
        self
    }

    #[doc = "Set the field `level_two`.\n"]
    pub fn set_level_two(
        mut self,
        v: impl Into<ListField<DataConnectUserHierarchyGroupHierarchyPathElLevelTwoEl>>,
    ) -> Self {
        self.level_two = Some(v.into());
        self
    }
}

impl ToListMappable for DataConnectUserHierarchyGroupHierarchyPathEl {
    type O = BlockAssignable<DataConnectUserHierarchyGroupHierarchyPathEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectUserHierarchyGroupHierarchyPathEl {}

impl BuildDataConnectUserHierarchyGroupHierarchyPathEl {
    pub fn build(self) -> DataConnectUserHierarchyGroupHierarchyPathEl {
        DataConnectUserHierarchyGroupHierarchyPathEl {
            level_five: core::default::Default::default(),
            level_four: core::default::Default::default(),
            level_one: core::default::Default::default(),
            level_three: core::default::Default::default(),
            level_two: core::default::Default::default(),
        }
    }
}

pub struct DataConnectUserHierarchyGroupHierarchyPathElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectUserHierarchyGroupHierarchyPathElRef {
    fn new(shared: StackShared, base: String) -> DataConnectUserHierarchyGroupHierarchyPathElRef {
        DataConnectUserHierarchyGroupHierarchyPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectUserHierarchyGroupHierarchyPathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `level_five` after provisioning.\n"]
    pub fn level_five(&self) -> ListRef<DataConnectUserHierarchyGroupHierarchyPathElLevelFiveElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_five", self.base))
    }

    #[doc = "Get a reference to the value of field `level_four` after provisioning.\n"]
    pub fn level_four(&self) -> ListRef<DataConnectUserHierarchyGroupHierarchyPathElLevelFourElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_four", self.base))
    }

    #[doc = "Get a reference to the value of field `level_one` after provisioning.\n"]
    pub fn level_one(&self) -> ListRef<DataConnectUserHierarchyGroupHierarchyPathElLevelOneElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_one", self.base))
    }

    #[doc = "Get a reference to the value of field `level_three` after provisioning.\n"]
    pub fn level_three(&self) -> ListRef<DataConnectUserHierarchyGroupHierarchyPathElLevelThreeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_three", self.base))
    }

    #[doc = "Get a reference to the value of field `level_two` after provisioning.\n"]
    pub fn level_two(&self) -> ListRef<DataConnectUserHierarchyGroupHierarchyPathElLevelTwoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_two", self.base))
    }
}
