use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ConfigConformancePackData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_s3_bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_s3_key_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_body: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_s3_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_parameter: Option<Vec<ConfigConformancePackInputParameterEl>>,
    dynamic: ConfigConformancePackDynamic,
}
struct ConfigConformancePack_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConfigConformancePackData>,
}
#[derive(Clone)]
pub struct ConfigConformancePack(Rc<ConfigConformancePack_>);
impl ConfigConformancePack {
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
    #[doc = "Set the field `delivery_s3_bucket`.\n"]
    pub fn set_delivery_s3_bucket(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().delivery_s3_bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `delivery_s3_key_prefix`.\n"]
    pub fn set_delivery_s3_key_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().delivery_s3_key_prefix = Some(v.into());
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
    #[doc = "Set the field `template_body`.\n"]
    pub fn set_template_body(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().template_body = Some(v.into());
        self
    }
    #[doc = "Set the field `template_s3_uri`.\n"]
    pub fn set_template_s3_uri(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().template_s3_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `input_parameter`.\n"]
    pub fn set_input_parameter(
        self,
        v: impl Into<BlockAssignable<ConfigConformancePackInputParameterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_parameter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_parameter = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `delivery_s3_bucket` after provisioning.\n"]
    pub fn delivery_s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_s3_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delivery_s3_key_prefix` after provisioning.\n"]
    pub fn delivery_s3_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_s3_key_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_body` after provisioning.\n"]
    pub fn template_body(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_body", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_s3_uri` after provisioning.\n"]
    pub fn template_s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_s3_uri", self.extract_ref()),
        )
    }
}
impl Referable for ConfigConformancePack {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ConfigConformancePack {}
impl ToListMappable for ConfigConformancePack {
    type O = ListRef<ConfigConformancePackRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ConfigConformancePack_ {
    fn extract_resource_type(&self) -> String {
        "aws_config_conformance_pack".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildConfigConformancePack {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildConfigConformancePack {
    pub fn build(self, stack: &mut Stack) -> ConfigConformancePack {
        let out = ConfigConformancePack(Rc::new(ConfigConformancePack_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConfigConformancePackData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                delivery_s3_bucket: core::default::Default::default(),
                delivery_s3_key_prefix: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                template_body: core::default::Default::default(),
                template_s3_uri: core::default::Default::default(),
                input_parameter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ConfigConformancePackRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigConformancePackRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ConfigConformancePackRef {
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
    #[doc = "Get a reference to the value of field `delivery_s3_bucket` after provisioning.\n"]
    pub fn delivery_s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_s3_bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delivery_s3_key_prefix` after provisioning.\n"]
    pub fn delivery_s3_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_s3_key_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_body` after provisioning.\n"]
    pub fn template_body(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_body", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_s3_uri` after provisioning.\n"]
    pub fn template_s3_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_s3_uri", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ConfigConformancePackInputParameterEl {
    parameter_name: PrimField<String>,
    parameter_value: PrimField<String>,
}
impl ConfigConformancePackInputParameterEl {}
impl ToListMappable for ConfigConformancePackInputParameterEl {
    type O = BlockAssignable<ConfigConformancePackInputParameterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildConfigConformancePackInputParameterEl {
    #[doc = ""]
    pub parameter_name: PrimField<String>,
    #[doc = ""]
    pub parameter_value: PrimField<String>,
}
impl BuildConfigConformancePackInputParameterEl {
    pub fn build(self) -> ConfigConformancePackInputParameterEl {
        ConfigConformancePackInputParameterEl {
            parameter_name: self.parameter_name,
            parameter_value: self.parameter_value,
        }
    }
}
pub struct ConfigConformancePackInputParameterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ConfigConformancePackInputParameterElRef {
    fn new(shared: StackShared, base: String) -> ConfigConformancePackInputParameterElRef {
        ConfigConformancePackInputParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ConfigConformancePackInputParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `parameter_name` after provisioning.\n"]
    pub fn parameter_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `parameter_value` after provisioning.\n"]
    pub fn parameter_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parameter_value", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ConfigConformancePackDynamic {
    input_parameter: Option<DynamicBlock<ConfigConformancePackInputParameterEl>>,
}
