use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockProvisionedModelThroughputData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment_duration: Option<PrimField<String>>,
    model_arn: PrimField<String>,
    model_units: PrimField<f64>,
    provisioned_model_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BedrockProvisionedModelThroughputTimeoutsEl>,
}

struct BedrockProvisionedModelThroughput_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockProvisionedModelThroughputData>,
}

#[derive(Clone)]
pub struct BedrockProvisionedModelThroughput(Rc<BedrockProvisionedModelThroughput_>);

impl BedrockProvisionedModelThroughput {
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

    #[doc = "Set the field `commitment_duration`.\n"]
    pub fn set_commitment_duration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().commitment_duration = Some(v.into());
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BedrockProvisionedModelThroughputTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `commitment_duration` after provisioning.\n"]
    pub fn commitment_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commitment_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_units` after provisioning.\n"]
    pub fn model_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_model_arn` after provisioning.\n"]
    pub fn provisioned_model_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_model_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_model_name` after provisioning.\n"]
    pub fn provisioned_model_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_model_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockProvisionedModelThroughputTimeoutsElRef {
        BedrockProvisionedModelThroughputTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for BedrockProvisionedModelThroughput {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BedrockProvisionedModelThroughput { }

impl ToListMappable for BedrockProvisionedModelThroughput {
    type O = ListRef<BedrockProvisionedModelThroughputRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BedrockProvisionedModelThroughput_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrock_provisioned_model_throughput".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockProvisionedModelThroughput {
    pub tf_id: String,
    #[doc = ""]
    pub model_arn: PrimField<String>,
    #[doc = ""]
    pub model_units: PrimField<f64>,
    #[doc = ""]
    pub provisioned_model_name: PrimField<String>,
}

impl BuildBedrockProvisionedModelThroughput {
    pub fn build(self, stack: &mut Stack) -> BedrockProvisionedModelThroughput {
        let out = BedrockProvisionedModelThroughput(Rc::new(BedrockProvisionedModelThroughput_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockProvisionedModelThroughputData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                commitment_duration: core::default::Default::default(),
                model_arn: self.model_arn,
                model_units: self.model_units,
                provisioned_model_name: self.provisioned_model_name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockProvisionedModelThroughputRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockProvisionedModelThroughputRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl BedrockProvisionedModelThroughputRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `commitment_duration` after provisioning.\n"]
    pub fn commitment_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.commitment_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_units` after provisioning.\n"]
    pub fn model_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_model_arn` after provisioning.\n"]
    pub fn provisioned_model_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_model_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_model_name` after provisioning.\n"]
    pub fn provisioned_model_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_model_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockProvisionedModelThroughputTimeoutsElRef {
        BedrockProvisionedModelThroughputTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockProvisionedModelThroughputTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}

impl BedrockProvisionedModelThroughputTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockProvisionedModelThroughputTimeoutsEl {
    type O = BlockAssignable<BedrockProvisionedModelThroughputTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockProvisionedModelThroughputTimeoutsEl {}

impl BuildBedrockProvisionedModelThroughputTimeoutsEl {
    pub fn build(self) -> BedrockProvisionedModelThroughputTimeoutsEl {
        BedrockProvisionedModelThroughputTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct BedrockProvisionedModelThroughputTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockProvisionedModelThroughputTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BedrockProvisionedModelThroughputTimeoutsElRef {
        BedrockProvisionedModelThroughputTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockProvisionedModelThroughputTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
