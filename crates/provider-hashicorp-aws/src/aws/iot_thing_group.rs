use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct IotThingGroupData {
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
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Vec<IotThingGroupPropertiesEl>>,
    dynamic: IotThingGroupDynamic,
}

struct IotThingGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IotThingGroupData>,
}

#[derive(Clone)]
pub struct IotThingGroup(Rc<IotThingGroup_>);

impl IotThingGroup {
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

    #[doc = "Set the field `parent_group_name`.\n"]
    pub fn set_parent_group_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent_group_name = Some(v.into());
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

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(self, v: impl Into<BlockAssignable<IotThingGroupPropertiesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.properties = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> ListRef<IotThingGroupMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parent_group_name` after provisioning.\n"]
    pub fn parent_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_group_name", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> ListRef<IotThingGroupPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
}

impl Referable for IotThingGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for IotThingGroup {}

impl ToListMappable for IotThingGroup {
    type O = ListRef<IotThingGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IotThingGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_iot_thing_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIotThingGroup {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildIotThingGroup {
    pub fn build(self, stack: &mut Stack) -> IotThingGroup {
        let out = IotThingGroup(Rc::new(IotThingGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IotThingGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                parent_group_name: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                properties: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IotThingGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotThingGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl IotThingGroupRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> ListRef<IotThingGroupMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parent_group_name` after provisioning.\n"]
    pub fn parent_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_group_name", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> ListRef<IotThingGroupPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct IotThingGroupMetadataElRootToParentGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_name: Option<PrimField<String>>,
}

impl IotThingGroupMetadataElRootToParentGroupsEl {
    #[doc = "Set the field `group_arn`.\n"]
    pub fn set_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `group_name`.\n"]
    pub fn set_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_name = Some(v.into());
        self
    }
}

impl ToListMappable for IotThingGroupMetadataElRootToParentGroupsEl {
    type O = BlockAssignable<IotThingGroupMetadataElRootToParentGroupsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotThingGroupMetadataElRootToParentGroupsEl {}

impl BuildIotThingGroupMetadataElRootToParentGroupsEl {
    pub fn build(self) -> IotThingGroupMetadataElRootToParentGroupsEl {
        IotThingGroupMetadataElRootToParentGroupsEl {
            group_arn: core::default::Default::default(),
            group_name: core::default::Default::default(),
        }
    }
}

pub struct IotThingGroupMetadataElRootToParentGroupsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotThingGroupMetadataElRootToParentGroupsElRef {
    fn new(shared: StackShared, base: String) -> IotThingGroupMetadataElRootToParentGroupsElRef {
        IotThingGroupMetadataElRootToParentGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotThingGroupMetadataElRootToParentGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_arn` after provisioning.\n"]
    pub fn group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.base))
    }
}

#[derive(Serialize)]
pub struct IotThingGroupMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_to_parent_groups: Option<ListField<IotThingGroupMetadataElRootToParentGroupsEl>>,
}

impl IotThingGroupMetadataEl {
    #[doc = "Set the field `creation_date`.\n"]
    pub fn set_creation_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.creation_date = Some(v.into());
        self
    }

    #[doc = "Set the field `parent_group_name`.\n"]
    pub fn set_parent_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parent_group_name = Some(v.into());
        self
    }

    #[doc = "Set the field `root_to_parent_groups`.\n"]
    pub fn set_root_to_parent_groups(
        mut self,
        v: impl Into<ListField<IotThingGroupMetadataElRootToParentGroupsEl>>,
    ) -> Self {
        self.root_to_parent_groups = Some(v.into());
        self
    }
}

impl ToListMappable for IotThingGroupMetadataEl {
    type O = BlockAssignable<IotThingGroupMetadataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotThingGroupMetadataEl {}

impl BuildIotThingGroupMetadataEl {
    pub fn build(self) -> IotThingGroupMetadataEl {
        IotThingGroupMetadataEl {
            creation_date: core::default::Default::default(),
            parent_group_name: core::default::Default::default(),
            root_to_parent_groups: core::default::Default::default(),
        }
    }
}

pub struct IotThingGroupMetadataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotThingGroupMetadataElRef {
    fn new(shared: StackShared, base: String) -> IotThingGroupMetadataElRef {
        IotThingGroupMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotThingGroupMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_date", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `parent_group_name` after provisioning.\n"]
    pub fn parent_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_group_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `root_to_parent_groups` after provisioning.\n"]
    pub fn root_to_parent_groups(&self) -> ListRef<IotThingGroupMetadataElRootToParentGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.root_to_parent_groups", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct IotThingGroupPropertiesElAttributePayloadEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<RecField<PrimField<String>>>,
}

impl IotThingGroupPropertiesElAttributePayloadEl {
    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.attributes = Some(v.into());
        self
    }
}

impl ToListMappable for IotThingGroupPropertiesElAttributePayloadEl {
    type O = BlockAssignable<IotThingGroupPropertiesElAttributePayloadEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotThingGroupPropertiesElAttributePayloadEl {}

impl BuildIotThingGroupPropertiesElAttributePayloadEl {
    pub fn build(self) -> IotThingGroupPropertiesElAttributePayloadEl {
        IotThingGroupPropertiesElAttributePayloadEl {
            attributes: core::default::Default::default(),
        }
    }
}

pub struct IotThingGroupPropertiesElAttributePayloadElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotThingGroupPropertiesElAttributePayloadElRef {
    fn new(shared: StackShared, base: String) -> IotThingGroupPropertiesElAttributePayloadElRef {
        IotThingGroupPropertiesElAttributePayloadElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotThingGroupPropertiesElAttributePayloadElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotThingGroupPropertiesElDynamic {
    attribute_payload: Option<DynamicBlock<IotThingGroupPropertiesElAttributePayloadEl>>,
}

#[derive(Serialize)]
pub struct IotThingGroupPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_payload: Option<Vec<IotThingGroupPropertiesElAttributePayloadEl>>,
    dynamic: IotThingGroupPropertiesElDynamic,
}

impl IotThingGroupPropertiesEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `attribute_payload`.\n"]
    pub fn set_attribute_payload(
        mut self,
        v: impl Into<BlockAssignable<IotThingGroupPropertiesElAttributePayloadEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.attribute_payload = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.attribute_payload = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for IotThingGroupPropertiesEl {
    type O = BlockAssignable<IotThingGroupPropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotThingGroupPropertiesEl {}

impl BuildIotThingGroupPropertiesEl {
    pub fn build(self) -> IotThingGroupPropertiesEl {
        IotThingGroupPropertiesEl {
            description: core::default::Default::default(),
            attribute_payload: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct IotThingGroupPropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotThingGroupPropertiesElRef {
    fn new(shared: StackShared, base: String) -> IotThingGroupPropertiesElRef {
        IotThingGroupPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotThingGroupPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `attribute_payload` after provisioning.\n"]
    pub fn attribute_payload(&self) -> ListRef<IotThingGroupPropertiesElAttributePayloadElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attribute_payload", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct IotThingGroupDynamic {
    properties: Option<DynamicBlock<IotThingGroupPropertiesEl>>,
}
