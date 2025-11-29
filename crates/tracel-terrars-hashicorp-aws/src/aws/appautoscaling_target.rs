use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppautoscalingTargetData {
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
    max_capacity: PrimField<f64>,
    min_capacity: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    resource_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    scalable_dimension: PrimField<String>,
    service_namespace: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suspended_state: Option<Vec<AppautoscalingTargetSuspendedStateEl>>,
    dynamic: AppautoscalingTargetDynamic,
}

struct AppautoscalingTarget_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppautoscalingTargetData>,
}

#[derive(Clone)]
pub struct AppautoscalingTarget(Rc<AppautoscalingTarget_>);

impl AppautoscalingTarget {
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

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().role_arn = Some(v.into());
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

    #[doc = "Set the field `suspended_state`.\n"]
    pub fn set_suspended_state(self, v: impl Into<BlockAssignable<AppautoscalingTargetSuspendedStateEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().suspended_state = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.suspended_state = Some(d);
            },
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

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_capacity` after provisioning.\n"]
    pub fn min_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_capacity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scalable_dimension` after provisioning.\n"]
    pub fn scalable_dimension(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scalable_dimension", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_namespace` after provisioning.\n"]
    pub fn service_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `suspended_state` after provisioning.\n"]
    pub fn suspended_state(&self) -> ListRef<AppautoscalingTargetSuspendedStateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.suspended_state", self.extract_ref()))
    }
}

impl Referable for AppautoscalingTarget {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AppautoscalingTarget { }

impl ToListMappable for AppautoscalingTarget {
    type O = ListRef<AppautoscalingTargetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AppautoscalingTarget_ {
    fn extract_resource_type(&self) -> String {
        "aws_appautoscaling_target".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppautoscalingTarget {
    pub tf_id: String,
    #[doc = ""]
    pub max_capacity: PrimField<f64>,
    #[doc = ""]
    pub min_capacity: PrimField<f64>,
    #[doc = ""]
    pub resource_id: PrimField<String>,
    #[doc = ""]
    pub scalable_dimension: PrimField<String>,
    #[doc = ""]
    pub service_namespace: PrimField<String>,
}

impl BuildAppautoscalingTarget {
    pub fn build(self, stack: &mut Stack) -> AppautoscalingTarget {
        let out = AppautoscalingTarget(Rc::new(AppautoscalingTarget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppautoscalingTargetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                max_capacity: self.max_capacity,
                min_capacity: self.min_capacity,
                region: core::default::Default::default(),
                resource_id: self.resource_id,
                role_arn: core::default::Default::default(),
                scalable_dimension: self.scalable_dimension,
                service_namespace: self.service_namespace,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                suspended_state: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppautoscalingTargetRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingTargetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl AppautoscalingTargetRef {
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

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_capacity` after provisioning.\n"]
    pub fn min_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_capacity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scalable_dimension` after provisioning.\n"]
    pub fn scalable_dimension(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scalable_dimension", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_namespace` after provisioning.\n"]
    pub fn service_namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `suspended_state` after provisioning.\n"]
    pub fn suspended_state(&self) -> ListRef<AppautoscalingTargetSuspendedStateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.suspended_state", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppautoscalingTargetSuspendedStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dynamic_scaling_in_suspended: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynamic_scaling_out_suspended: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled_scaling_suspended: Option<PrimField<bool>>,
}

impl AppautoscalingTargetSuspendedStateEl {
    #[doc = "Set the field `dynamic_scaling_in_suspended`.\n"]
    pub fn set_dynamic_scaling_in_suspended(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.dynamic_scaling_in_suspended = Some(v.into());
        self
    }

    #[doc = "Set the field `dynamic_scaling_out_suspended`.\n"]
    pub fn set_dynamic_scaling_out_suspended(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.dynamic_scaling_out_suspended = Some(v.into());
        self
    }

    #[doc = "Set the field `scheduled_scaling_suspended`.\n"]
    pub fn set_scheduled_scaling_suspended(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.scheduled_scaling_suspended = Some(v.into());
        self
    }
}

impl ToListMappable for AppautoscalingTargetSuspendedStateEl {
    type O = BlockAssignable<AppautoscalingTargetSuspendedStateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppautoscalingTargetSuspendedStateEl {}

impl BuildAppautoscalingTargetSuspendedStateEl {
    pub fn build(self) -> AppautoscalingTargetSuspendedStateEl {
        AppautoscalingTargetSuspendedStateEl {
            dynamic_scaling_in_suspended: core::default::Default::default(),
            dynamic_scaling_out_suspended: core::default::Default::default(),
            scheduled_scaling_suspended: core::default::Default::default(),
        }
    }
}

pub struct AppautoscalingTargetSuspendedStateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppautoscalingTargetSuspendedStateElRef {
    fn new(shared: StackShared, base: String) -> AppautoscalingTargetSuspendedStateElRef {
        AppautoscalingTargetSuspendedStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppautoscalingTargetSuspendedStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dynamic_scaling_in_suspended` after provisioning.\n"]
    pub fn dynamic_scaling_in_suspended(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dynamic_scaling_in_suspended", self.base))
    }

    #[doc = "Get a reference to the value of field `dynamic_scaling_out_suspended` after provisioning.\n"]
    pub fn dynamic_scaling_out_suspended(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dynamic_scaling_out_suspended", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduled_scaling_suspended` after provisioning.\n"]
    pub fn scheduled_scaling_suspended(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.scheduled_scaling_suspended", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppautoscalingTargetDynamic {
    suspended_state: Option<DynamicBlock<AppautoscalingTargetSuspendedStateEl>>,
}
