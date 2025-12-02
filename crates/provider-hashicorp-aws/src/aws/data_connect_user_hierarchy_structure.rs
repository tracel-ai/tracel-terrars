use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataConnectUserHierarchyStructureData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataConnectUserHierarchyStructure_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataConnectUserHierarchyStructureData>,
}
#[derive(Clone)]
pub struct DataConnectUserHierarchyStructure(Rc<DataConnectUserHierarchyStructure_>);
impl DataConnectUserHierarchyStructure {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `hierarchy_structure` after provisioning.\n"]
    pub fn hierarchy_structure(
        &self,
    ) -> ListRef<DataConnectUserHierarchyStructureHierarchyStructureElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hierarchy_structure", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
impl Referable for DataConnectUserHierarchyStructure {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataConnectUserHierarchyStructure {}
impl ToListMappable for DataConnectUserHierarchyStructure {
    type O = ListRef<DataConnectUserHierarchyStructureRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataConnectUserHierarchyStructure_ {
    fn extract_datasource_type(&self) -> String {
        "aws_connect_user_hierarchy_structure".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataConnectUserHierarchyStructure {
    pub tf_id: String,
    #[doc = ""]
    pub instance_id: PrimField<String>,
}
impl BuildDataConnectUserHierarchyStructure {
    pub fn build(self, stack: &mut Stack) -> DataConnectUserHierarchyStructure {
        let out = DataConnectUserHierarchyStructure(Rc::new(DataConnectUserHierarchyStructure_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataConnectUserHierarchyStructureData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataConnectUserHierarchyStructureRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataConnectUserHierarchyStructureRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataConnectUserHierarchyStructureRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `hierarchy_structure` after provisioning.\n"]
    pub fn hierarchy_structure(
        &self,
    ) -> ListRef<DataConnectUserHierarchyStructureHierarchyStructureElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hierarchy_structure", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
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
impl ToListMappable for DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
    type O = BlockAssignable<DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {}
impl BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
    pub fn build(self) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef {
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
pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
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
impl ToListMappable for DataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
    type O = BlockAssignable<DataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {}
impl BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
    pub fn build(self) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef {
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
pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
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
impl ToListMappable for DataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
    type O = BlockAssignable<DataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {}
impl BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
    pub fn build(self) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef {
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
pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
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
impl ToListMappable for DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
    type O = BlockAssignable<DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {}
impl BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
    pub fn build(self) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef {
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
pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
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
impl ToListMappable for DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
    type O = BlockAssignable<DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {}
impl BuildDataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
    pub fn build(self) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
        DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef {
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
pub struct DataConnectUserHierarchyStructureHierarchyStructureEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    level_five: Option<ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level_four: Option<ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level_one: Option<ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level_three:
        Option<ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level_two: Option<ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl>>,
}
impl DataConnectUserHierarchyStructureHierarchyStructureEl {
    #[doc = "Set the field `level_five`.\n"]
    pub fn set_level_five(
        mut self,
        v: impl Into<ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveEl>>,
    ) -> Self {
        self.level_five = Some(v.into());
        self
    }
    #[doc = "Set the field `level_four`.\n"]
    pub fn set_level_four(
        mut self,
        v: impl Into<ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelFourEl>>,
    ) -> Self {
        self.level_four = Some(v.into());
        self
    }
    #[doc = "Set the field `level_one`.\n"]
    pub fn set_level_one(
        mut self,
        v: impl Into<ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelOneEl>>,
    ) -> Self {
        self.level_one = Some(v.into());
        self
    }
    #[doc = "Set the field `level_three`.\n"]
    pub fn set_level_three(
        mut self,
        v: impl Into<ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeEl>>,
    ) -> Self {
        self.level_three = Some(v.into());
        self
    }
    #[doc = "Set the field `level_two`.\n"]
    pub fn set_level_two(
        mut self,
        v: impl Into<ListField<DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoEl>>,
    ) -> Self {
        self.level_two = Some(v.into());
        self
    }
}
impl ToListMappable for DataConnectUserHierarchyStructureHierarchyStructureEl {
    type O = BlockAssignable<DataConnectUserHierarchyStructureHierarchyStructureEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataConnectUserHierarchyStructureHierarchyStructureEl {}
impl BuildDataConnectUserHierarchyStructureHierarchyStructureEl {
    pub fn build(self) -> DataConnectUserHierarchyStructureHierarchyStructureEl {
        DataConnectUserHierarchyStructureHierarchyStructureEl {
            level_five: core::default::Default::default(),
            level_four: core::default::Default::default(),
            level_one: core::default::Default::default(),
            level_three: core::default::Default::default(),
            level_two: core::default::Default::default(),
        }
    }
}
pub struct DataConnectUserHierarchyStructureHierarchyStructureElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataConnectUserHierarchyStructureHierarchyStructureElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataConnectUserHierarchyStructureHierarchyStructureElRef {
        DataConnectUserHierarchyStructureHierarchyStructureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataConnectUserHierarchyStructureHierarchyStructureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `level_five` after provisioning.\n"]
    pub fn level_five(
        &self,
    ) -> ListRef<DataConnectUserHierarchyStructureHierarchyStructureElLevelFiveElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_five", self.base))
    }
    #[doc = "Get a reference to the value of field `level_four` after provisioning.\n"]
    pub fn level_four(
        &self,
    ) -> ListRef<DataConnectUserHierarchyStructureHierarchyStructureElLevelFourElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_four", self.base))
    }
    #[doc = "Get a reference to the value of field `level_one` after provisioning.\n"]
    pub fn level_one(
        &self,
    ) -> ListRef<DataConnectUserHierarchyStructureHierarchyStructureElLevelOneElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_one", self.base))
    }
    #[doc = "Get a reference to the value of field `level_three` after provisioning.\n"]
    pub fn level_three(
        &self,
    ) -> ListRef<DataConnectUserHierarchyStructureHierarchyStructureElLevelThreeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_three", self.base))
    }
    #[doc = "Get a reference to the value of field `level_two` after provisioning.\n"]
    pub fn level_two(
        &self,
    ) -> ListRef<DataConnectUserHierarchyStructureHierarchyStructureElLevelTwoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.level_two", self.base))
    }
}
