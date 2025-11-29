use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AutoscalingTrafficSourceAttachmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    autoscaling_group_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AutoscalingTrafficSourceAttachmentTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traffic_source: Option<Vec<AutoscalingTrafficSourceAttachmentTrafficSourceEl>>,
    dynamic: AutoscalingTrafficSourceAttachmentDynamic,
}

struct AutoscalingTrafficSourceAttachment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AutoscalingTrafficSourceAttachmentData>,
}

#[derive(Clone)]
pub struct AutoscalingTrafficSourceAttachment(Rc<AutoscalingTrafficSourceAttachment_>);

impl AutoscalingTrafficSourceAttachment {
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AutoscalingTrafficSourceAttachmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `traffic_source`.\n"]
    pub fn set_traffic_source(
        self,
        v: impl Into<BlockAssignable<AutoscalingTrafficSourceAttachmentTrafficSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().traffic_source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.traffic_source = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `autoscaling_group_name` after provisioning.\n"]
    pub fn autoscaling_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_group_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AutoscalingTrafficSourceAttachmentTimeoutsElRef {
        AutoscalingTrafficSourceAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `traffic_source` after provisioning.\n"]
    pub fn traffic_source(&self) -> ListRef<AutoscalingTrafficSourceAttachmentTrafficSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.traffic_source", self.extract_ref()))
    }
}

impl Referable for AutoscalingTrafficSourceAttachment {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AutoscalingTrafficSourceAttachment { }

impl ToListMappable for AutoscalingTrafficSourceAttachment {
    type O = ListRef<AutoscalingTrafficSourceAttachmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AutoscalingTrafficSourceAttachment_ {
    fn extract_resource_type(&self) -> String {
        "aws_autoscaling_traffic_source_attachment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAutoscalingTrafficSourceAttachment {
    pub tf_id: String,
    #[doc = ""]
    pub autoscaling_group_name: PrimField<String>,
}

impl BuildAutoscalingTrafficSourceAttachment {
    pub fn build(self, stack: &mut Stack) -> AutoscalingTrafficSourceAttachment {
        let out = AutoscalingTrafficSourceAttachment(Rc::new(AutoscalingTrafficSourceAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AutoscalingTrafficSourceAttachmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                autoscaling_group_name: self.autoscaling_group_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                traffic_source: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AutoscalingTrafficSourceAttachmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingTrafficSourceAttachmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl AutoscalingTrafficSourceAttachmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_group_name` after provisioning.\n"]
    pub fn autoscaling_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_group_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AutoscalingTrafficSourceAttachmentTimeoutsElRef {
        AutoscalingTrafficSourceAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `traffic_source` after provisioning.\n"]
    pub fn traffic_source(&self) -> ListRef<AutoscalingTrafficSourceAttachmentTrafficSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.traffic_source", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AutoscalingTrafficSourceAttachmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl AutoscalingTrafficSourceAttachmentTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for AutoscalingTrafficSourceAttachmentTimeoutsEl {
    type O = BlockAssignable<AutoscalingTrafficSourceAttachmentTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingTrafficSourceAttachmentTimeoutsEl {}

impl BuildAutoscalingTrafficSourceAttachmentTimeoutsEl {
    pub fn build(self) -> AutoscalingTrafficSourceAttachmentTimeoutsEl {
        AutoscalingTrafficSourceAttachmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct AutoscalingTrafficSourceAttachmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingTrafficSourceAttachmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingTrafficSourceAttachmentTimeoutsElRef {
        AutoscalingTrafficSourceAttachmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingTrafficSourceAttachmentTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize)]
pub struct AutoscalingTrafficSourceAttachmentTrafficSourceEl {
    identifier: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl AutoscalingTrafficSourceAttachmentTrafficSourceEl { }

impl ToListMappable for AutoscalingTrafficSourceAttachmentTrafficSourceEl {
    type O = BlockAssignable<AutoscalingTrafficSourceAttachmentTrafficSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAutoscalingTrafficSourceAttachmentTrafficSourceEl {
    #[doc = ""]
    pub identifier: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildAutoscalingTrafficSourceAttachmentTrafficSourceEl {
    pub fn build(self) -> AutoscalingTrafficSourceAttachmentTrafficSourceEl {
        AutoscalingTrafficSourceAttachmentTrafficSourceEl {
            identifier: self.identifier,
            type_: self.type_,
        }
    }
}

pub struct AutoscalingTrafficSourceAttachmentTrafficSourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AutoscalingTrafficSourceAttachmentTrafficSourceElRef {
    fn new(shared: StackShared, base: String) -> AutoscalingTrafficSourceAttachmentTrafficSourceElRef {
        AutoscalingTrafficSourceAttachmentTrafficSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AutoscalingTrafficSourceAttachmentTrafficSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AutoscalingTrafficSourceAttachmentDynamic {
    traffic_source: Option<DynamicBlock<AutoscalingTrafficSourceAttachmentTrafficSourceEl>>,
}
