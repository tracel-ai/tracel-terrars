use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct IotIndexingConfigurationData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    thing_group_indexing_configuration:
        Option<Vec<IotIndexingConfigurationThingGroupIndexingConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thing_indexing_configuration: Option<Vec<IotIndexingConfigurationThingIndexingConfigurationEl>>,
    dynamic: IotIndexingConfigurationDynamic,
}

struct IotIndexingConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IotIndexingConfigurationData>,
}

#[derive(Clone)]
pub struct IotIndexingConfiguration(Rc<IotIndexingConfiguration_>);

impl IotIndexingConfiguration {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `thing_group_indexing_configuration`.\n"]
    pub fn set_thing_group_indexing_configuration(
        self,
        v: impl Into<BlockAssignable<IotIndexingConfigurationThingGroupIndexingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().thing_group_indexing_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .thing_group_indexing_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `thing_indexing_configuration`.\n"]
    pub fn set_thing_indexing_configuration(
        self,
        v: impl Into<BlockAssignable<IotIndexingConfigurationThingIndexingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().thing_indexing_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .thing_indexing_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `thing_group_indexing_configuration` after provisioning.\n"]
    pub fn thing_group_indexing_configuration(
        &self,
    ) -> ListRef<IotIndexingConfigurationThingGroupIndexingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.thing_group_indexing_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `thing_indexing_configuration` after provisioning.\n"]
    pub fn thing_indexing_configuration(
        &self,
    ) -> ListRef<IotIndexingConfigurationThingIndexingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.thing_indexing_configuration", self.extract_ref()),
        )
    }
}

impl Referable for IotIndexingConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for IotIndexingConfiguration {}

impl ToListMappable for IotIndexingConfiguration {
    type O = ListRef<IotIndexingConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IotIndexingConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_iot_indexing_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIotIndexingConfiguration {
    pub tf_id: String,
}

impl BuildIotIndexingConfiguration {
    pub fn build(self, stack: &mut Stack) -> IotIndexingConfiguration {
        let out = IotIndexingConfiguration(Rc::new(IotIndexingConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IotIndexingConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                thing_group_indexing_configuration: core::default::Default::default(),
                thing_indexing_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IotIndexingConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotIndexingConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl IotIndexingConfigurationRef {
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

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `thing_group_indexing_configuration` after provisioning.\n"]
    pub fn thing_group_indexing_configuration(
        &self,
    ) -> ListRef<IotIndexingConfigurationThingGroupIndexingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.thing_group_indexing_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `thing_indexing_configuration` after provisioning.\n"]
    pub fn thing_indexing_configuration(
        &self,
    ) -> ListRef<IotIndexingConfigurationThingIndexingConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.thing_indexing_configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl {
    type O =
        BlockAssignable<IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl {}

impl BuildIotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl {
    pub fn build(self) -> IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl {
        IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldElRef {
        IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl {
    type O =
        BlockAssignable<IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl {}

impl BuildIotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl {
    pub fn build(self) -> IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl {
        IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldElRef {
        IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotIndexingConfigurationThingGroupIndexingConfigurationElDynamic {
    custom_field: Option<
        DynamicBlock<IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl>,
    >,
    managed_field: Option<
        DynamicBlock<IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl>,
    >,
}

#[derive(Serialize)]
pub struct IotIndexingConfigurationThingGroupIndexingConfigurationEl {
    thing_group_indexing_mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_field:
        Option<Vec<IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_field:
        Option<Vec<IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl>>,
    dynamic: IotIndexingConfigurationThingGroupIndexingConfigurationElDynamic,
}

impl IotIndexingConfigurationThingGroupIndexingConfigurationEl {
    #[doc = "Set the field `custom_field`.\n"]
    pub fn set_custom_field(
        mut self,
        v: impl Into<
            BlockAssignable<IotIndexingConfigurationThingGroupIndexingConfigurationElCustomFieldEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_field = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_field = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `managed_field`.\n"]
    pub fn set_managed_field(
        mut self,
        v: impl Into<
            BlockAssignable<
                IotIndexingConfigurationThingGroupIndexingConfigurationElManagedFieldEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.managed_field = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_field = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for IotIndexingConfigurationThingGroupIndexingConfigurationEl {
    type O = BlockAssignable<IotIndexingConfigurationThingGroupIndexingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfigurationThingGroupIndexingConfigurationEl {
    #[doc = ""]
    pub thing_group_indexing_mode: PrimField<String>,
}

impl BuildIotIndexingConfigurationThingGroupIndexingConfigurationEl {
    pub fn build(self) -> IotIndexingConfigurationThingGroupIndexingConfigurationEl {
        IotIndexingConfigurationThingGroupIndexingConfigurationEl {
            thing_group_indexing_mode: self.thing_group_indexing_mode,
            custom_field: core::default::Default::default(),
            managed_field: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct IotIndexingConfigurationThingGroupIndexingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotIndexingConfigurationThingGroupIndexingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IotIndexingConfigurationThingGroupIndexingConfigurationElRef {
        IotIndexingConfigurationThingGroupIndexingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfigurationThingGroupIndexingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `thing_group_indexing_mode` after provisioning.\n"]
    pub fn thing_group_indexing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.thing_group_indexing_mode", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl {
    type O = BlockAssignable<IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl {}

impl BuildIotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl {
    pub fn build(self) -> IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl {
        IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct IotIndexingConfigurationThingIndexingConfigurationElCustomFieldElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotIndexingConfigurationThingIndexingConfigurationElCustomFieldElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IotIndexingConfigurationThingIndexingConfigurationElCustomFieldElRef {
        IotIndexingConfigurationThingIndexingConfigurationElCustomFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfigurationThingIndexingConfigurationElCustomFieldElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct IotIndexingConfigurationThingIndexingConfigurationElFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    named_shadow_names: Option<SetField<PrimField<String>>>,
}

impl IotIndexingConfigurationThingIndexingConfigurationElFilterEl {
    #[doc = "Set the field `named_shadow_names`.\n"]
    pub fn set_named_shadow_names(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.named_shadow_names = Some(v.into());
        self
    }
}

impl ToListMappable for IotIndexingConfigurationThingIndexingConfigurationElFilterEl {
    type O = BlockAssignable<IotIndexingConfigurationThingIndexingConfigurationElFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfigurationThingIndexingConfigurationElFilterEl {}

impl BuildIotIndexingConfigurationThingIndexingConfigurationElFilterEl {
    pub fn build(self) -> IotIndexingConfigurationThingIndexingConfigurationElFilterEl {
        IotIndexingConfigurationThingIndexingConfigurationElFilterEl {
            named_shadow_names: core::default::Default::default(),
        }
    }
}

pub struct IotIndexingConfigurationThingIndexingConfigurationElFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotIndexingConfigurationThingIndexingConfigurationElFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IotIndexingConfigurationThingIndexingConfigurationElFilterElRef {
        IotIndexingConfigurationThingIndexingConfigurationElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfigurationThingIndexingConfigurationElFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `named_shadow_names` after provisioning.\n"]
    pub fn named_shadow_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.named_shadow_names", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl {
    type O = BlockAssignable<IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl {}

impl BuildIotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl {
    pub fn build(self) -> IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl {
        IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct IotIndexingConfigurationThingIndexingConfigurationElManagedFieldElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotIndexingConfigurationThingIndexingConfigurationElManagedFieldElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IotIndexingConfigurationThingIndexingConfigurationElManagedFieldElRef {
        IotIndexingConfigurationThingIndexingConfigurationElManagedFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfigurationThingIndexingConfigurationElManagedFieldElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotIndexingConfigurationThingIndexingConfigurationElDynamic {
    custom_field:
        Option<DynamicBlock<IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl>>,
    filter: Option<DynamicBlock<IotIndexingConfigurationThingIndexingConfigurationElFilterEl>>,
    managed_field:
        Option<DynamicBlock<IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl>>,
}

#[derive(Serialize)]
pub struct IotIndexingConfigurationThingIndexingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_defender_indexing_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    named_shadow_indexing_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thing_connectivity_indexing_mode: Option<PrimField<String>>,
    thing_indexing_mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_field: Option<Vec<IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<IotIndexingConfigurationThingIndexingConfigurationElFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_field: Option<Vec<IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl>>,
    dynamic: IotIndexingConfigurationThingIndexingConfigurationElDynamic,
}

impl IotIndexingConfigurationThingIndexingConfigurationEl {
    #[doc = "Set the field `device_defender_indexing_mode`.\n"]
    pub fn set_device_defender_indexing_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_defender_indexing_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `named_shadow_indexing_mode`.\n"]
    pub fn set_named_shadow_indexing_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.named_shadow_indexing_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `thing_connectivity_indexing_mode`.\n"]
    pub fn set_thing_connectivity_indexing_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.thing_connectivity_indexing_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_field`.\n"]
    pub fn set_custom_field(
        mut self,
        v: impl Into<BlockAssignable<IotIndexingConfigurationThingIndexingConfigurationElCustomFieldEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_field = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_field = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl Into<BlockAssignable<IotIndexingConfigurationThingIndexingConfigurationElFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `managed_field`.\n"]
    pub fn set_managed_field(
        mut self,
        v: impl Into<
            BlockAssignable<IotIndexingConfigurationThingIndexingConfigurationElManagedFieldEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.managed_field = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_field = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for IotIndexingConfigurationThingIndexingConfigurationEl {
    type O = BlockAssignable<IotIndexingConfigurationThingIndexingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotIndexingConfigurationThingIndexingConfigurationEl {
    #[doc = ""]
    pub thing_indexing_mode: PrimField<String>,
}

impl BuildIotIndexingConfigurationThingIndexingConfigurationEl {
    pub fn build(self) -> IotIndexingConfigurationThingIndexingConfigurationEl {
        IotIndexingConfigurationThingIndexingConfigurationEl {
            device_defender_indexing_mode: core::default::Default::default(),
            named_shadow_indexing_mode: core::default::Default::default(),
            thing_connectivity_indexing_mode: core::default::Default::default(),
            thing_indexing_mode: self.thing_indexing_mode,
            custom_field: core::default::Default::default(),
            filter: core::default::Default::default(),
            managed_field: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct IotIndexingConfigurationThingIndexingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IotIndexingConfigurationThingIndexingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IotIndexingConfigurationThingIndexingConfigurationElRef {
        IotIndexingConfigurationThingIndexingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotIndexingConfigurationThingIndexingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_defender_indexing_mode` after provisioning.\n"]
    pub fn device_defender_indexing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.device_defender_indexing_mode", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `named_shadow_indexing_mode` after provisioning.\n"]
    pub fn named_shadow_indexing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.named_shadow_indexing_mode", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `thing_connectivity_indexing_mode` after provisioning.\n"]
    pub fn thing_connectivity_indexing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.thing_connectivity_indexing_mode", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `thing_indexing_mode` after provisioning.\n"]
    pub fn thing_indexing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.thing_indexing_mode", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(
        &self,
    ) -> ListRef<IotIndexingConfigurationThingIndexingConfigurationElFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct IotIndexingConfigurationDynamic {
    thing_group_indexing_configuration:
        Option<DynamicBlock<IotIndexingConfigurationThingGroupIndexingConfigurationEl>>,
    thing_indexing_configuration:
        Option<DynamicBlock<IotIndexingConfigurationThingIndexingConfigurationEl>>,
}
