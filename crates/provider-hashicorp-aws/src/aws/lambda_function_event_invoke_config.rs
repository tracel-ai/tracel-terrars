use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct LambdaFunctionEventInvokeConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    function_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_event_age_in_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_retry_attempts: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qualifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_config: Option<Vec<LambdaFunctionEventInvokeConfigDestinationConfigEl>>,
    dynamic: LambdaFunctionEventInvokeConfigDynamic,
}

struct LambdaFunctionEventInvokeConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LambdaFunctionEventInvokeConfigData>,
}

#[derive(Clone)]
pub struct LambdaFunctionEventInvokeConfig(Rc<LambdaFunctionEventInvokeConfig_>);

impl LambdaFunctionEventInvokeConfig {
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

    #[doc = "Set the field `maximum_event_age_in_seconds`.\n"]
    pub fn set_maximum_event_age_in_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().maximum_event_age_in_seconds = Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_retry_attempts`.\n"]
    pub fn set_maximum_retry_attempts(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().maximum_retry_attempts = Some(v.into());
        self
    }

    #[doc = "Set the field `qualifier`.\n"]
    pub fn set_qualifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().qualifier = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `destination_config`.\n"]
    pub fn set_destination_config(
        self,
        v: impl Into<BlockAssignable<LambdaFunctionEventInvokeConfigDestinationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_config = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_event_age_in_seconds` after provisioning.\n"]
    pub fn maximum_event_age_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_event_age_in_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maximum_retry_attempts` after provisioning.\n"]
    pub fn maximum_retry_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_retry_attempts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qualifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(
        &self,
    ) -> ListRef<LambdaFunctionEventInvokeConfigDestinationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_config", self.extract_ref()),
        )
    }
}

impl Referable for LambdaFunctionEventInvokeConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for LambdaFunctionEventInvokeConfig {}

impl ToListMappable for LambdaFunctionEventInvokeConfig {
    type O = ListRef<LambdaFunctionEventInvokeConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LambdaFunctionEventInvokeConfig_ {
    fn extract_resource_type(&self) -> String {
        "aws_lambda_function_event_invoke_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLambdaFunctionEventInvokeConfig {
    pub tf_id: String,
    #[doc = ""]
    pub function_name: PrimField<String>,
}

impl BuildLambdaFunctionEventInvokeConfig {
    pub fn build(self, stack: &mut Stack) -> LambdaFunctionEventInvokeConfig {
        let out = LambdaFunctionEventInvokeConfig(Rc::new(LambdaFunctionEventInvokeConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LambdaFunctionEventInvokeConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                function_name: self.function_name,
                id: core::default::Default::default(),
                maximum_event_age_in_seconds: core::default::Default::default(),
                maximum_retry_attempts: core::default::Default::default(),
                qualifier: core::default::Default::default(),
                region: core::default::Default::default(),
                destination_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LambdaFunctionEventInvokeConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for LambdaFunctionEventInvokeConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl LambdaFunctionEventInvokeConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_event_age_in_seconds` after provisioning.\n"]
    pub fn maximum_event_age_in_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_event_age_in_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maximum_retry_attempts` after provisioning.\n"]
    pub fn maximum_retry_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_retry_attempts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qualifier", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(
        &self,
    ) -> ListRef<LambdaFunctionEventInvokeConfigDestinationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl {
    destination: PrimField<String>,
}

impl LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl {}

impl ToListMappable for LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl {
    type O = BlockAssignable<LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl {
    #[doc = ""]
    pub destination: PrimField<String>,
}

impl BuildLambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl {
    pub fn build(self) -> LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl {
        LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl {
            destination: self.destination,
        }
    }
}

pub struct LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureElRef {
        LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl {
    destination: PrimField<String>,
}

impl LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl {}

impl ToListMappable for LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl {
    type O = BlockAssignable<LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl {
    #[doc = ""]
    pub destination: PrimField<String>,
}

impl BuildLambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl {
    pub fn build(self) -> LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl {
        LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl {
            destination: self.destination,
        }
    }
}

pub struct LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessElRef {
        LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct LambdaFunctionEventInvokeConfigDestinationConfigElDynamic {
    on_failure: Option<DynamicBlock<LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl>>,
    on_success: Option<DynamicBlock<LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl>>,
}

#[derive(Serialize)]
pub struct LambdaFunctionEventInvokeConfigDestinationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    on_failure: Option<Vec<LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_success: Option<Vec<LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl>>,
    dynamic: LambdaFunctionEventInvokeConfigDestinationConfigElDynamic,
}

impl LambdaFunctionEventInvokeConfigDestinationConfigEl {
    #[doc = "Set the field `on_failure`.\n"]
    pub fn set_on_failure(
        mut self,
        v: impl Into<BlockAssignable<LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.on_failure = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.on_failure = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `on_success`.\n"]
    pub fn set_on_success(
        mut self,
        v: impl Into<BlockAssignable<LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.on_success = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.on_success = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for LambdaFunctionEventInvokeConfigDestinationConfigEl {
    type O = BlockAssignable<LambdaFunctionEventInvokeConfigDestinationConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionEventInvokeConfigDestinationConfigEl {}

impl BuildLambdaFunctionEventInvokeConfigDestinationConfigEl {
    pub fn build(self) -> LambdaFunctionEventInvokeConfigDestinationConfigEl {
        LambdaFunctionEventInvokeConfigDestinationConfigEl {
            on_failure: core::default::Default::default(),
            on_success: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LambdaFunctionEventInvokeConfigDestinationConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LambdaFunctionEventInvokeConfigDestinationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LambdaFunctionEventInvokeConfigDestinationConfigElRef {
        LambdaFunctionEventInvokeConfigDestinationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionEventInvokeConfigDestinationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_failure` after provisioning.\n"]
    pub fn on_failure(
        &self,
    ) -> ListRef<LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureElRef> {
        ListRef::new(self.shared().clone(), format!("{}.on_failure", self.base))
    }

    #[doc = "Get a reference to the value of field `on_success` after provisioning.\n"]
    pub fn on_success(
        &self,
    ) -> ListRef<LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.on_success", self.base))
    }
}

#[derive(Serialize, Default)]
struct LambdaFunctionEventInvokeConfigDynamic {
    destination_config: Option<DynamicBlock<LambdaFunctionEventInvokeConfigDestinationConfigEl>>,
}
