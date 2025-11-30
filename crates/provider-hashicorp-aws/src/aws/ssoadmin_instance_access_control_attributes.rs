use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct SsoadminInstanceAccessControlAttributesData {
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
    instance_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute: Option<Vec<SsoadminInstanceAccessControlAttributesAttributeEl>>,
    dynamic: SsoadminInstanceAccessControlAttributesDynamic,
}

struct SsoadminInstanceAccessControlAttributes_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SsoadminInstanceAccessControlAttributesData>,
}

#[derive(Clone)]
pub struct SsoadminInstanceAccessControlAttributes(Rc<SsoadminInstanceAccessControlAttributes_>);

impl SsoadminInstanceAccessControlAttributes {
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

    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(
        self,
        v: impl Into<BlockAssignable<SsoadminInstanceAccessControlAttributesAttributeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attribute = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attribute = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_reason", self.extract_ref()),
        )
    }
}

impl Referable for SsoadminInstanceAccessControlAttributes {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for SsoadminInstanceAccessControlAttributes {}

impl ToListMappable for SsoadminInstanceAccessControlAttributes {
    type O = ListRef<SsoadminInstanceAccessControlAttributesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SsoadminInstanceAccessControlAttributes_ {
    fn extract_resource_type(&self) -> String {
        "aws_ssoadmin_instance_access_control_attributes".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsoadminInstanceAccessControlAttributes {
    pub tf_id: String,
    #[doc = ""]
    pub instance_arn: PrimField<String>,
}

impl BuildSsoadminInstanceAccessControlAttributes {
    pub fn build(self, stack: &mut Stack) -> SsoadminInstanceAccessControlAttributes {
        let out = SsoadminInstanceAccessControlAttributes(Rc::new(
            SsoadminInstanceAccessControlAttributes_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(SsoadminInstanceAccessControlAttributesData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    id: core::default::Default::default(),
                    instance_arn: self.instance_arn,
                    region: core::default::Default::default(),
                    attribute: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsoadminInstanceAccessControlAttributesRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsoadminInstanceAccessControlAttributesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl SsoadminInstanceAccessControlAttributesRef {
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

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_reason", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SsoadminInstanceAccessControlAttributesAttributeElValueEl {
    source: SetField<PrimField<String>>,
}

impl SsoadminInstanceAccessControlAttributesAttributeElValueEl {}

impl ToListMappable for SsoadminInstanceAccessControlAttributesAttributeElValueEl {
    type O = BlockAssignable<SsoadminInstanceAccessControlAttributesAttributeElValueEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsoadminInstanceAccessControlAttributesAttributeElValueEl {
    #[doc = ""]
    pub source: SetField<PrimField<String>>,
}

impl BuildSsoadminInstanceAccessControlAttributesAttributeElValueEl {
    pub fn build(self) -> SsoadminInstanceAccessControlAttributesAttributeElValueEl {
        SsoadminInstanceAccessControlAttributesAttributeElValueEl {
            source: self.source,
        }
    }
}

pub struct SsoadminInstanceAccessControlAttributesAttributeElValueElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsoadminInstanceAccessControlAttributesAttributeElValueElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsoadminInstanceAccessControlAttributesAttributeElValueElRef {
        SsoadminInstanceAccessControlAttributesAttributeElValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsoadminInstanceAccessControlAttributesAttributeElValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsoadminInstanceAccessControlAttributesAttributeElDynamic {
    value: Option<DynamicBlock<SsoadminInstanceAccessControlAttributesAttributeElValueEl>>,
}

#[derive(Serialize)]
pub struct SsoadminInstanceAccessControlAttributesAttributeEl {
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Vec<SsoadminInstanceAccessControlAttributesAttributeElValueEl>>,
    dynamic: SsoadminInstanceAccessControlAttributesAttributeElDynamic,
}

impl SsoadminInstanceAccessControlAttributesAttributeEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v: impl Into<BlockAssignable<SsoadminInstanceAccessControlAttributesAttributeElValueEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.value = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for SsoadminInstanceAccessControlAttributesAttributeEl {
    type O = BlockAssignable<SsoadminInstanceAccessControlAttributesAttributeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsoadminInstanceAccessControlAttributesAttributeEl {
    #[doc = ""]
    pub key: PrimField<String>,
}

impl BuildSsoadminInstanceAccessControlAttributesAttributeEl {
    pub fn build(self) -> SsoadminInstanceAccessControlAttributesAttributeEl {
        SsoadminInstanceAccessControlAttributesAttributeEl {
            key: self.key,
            value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct SsoadminInstanceAccessControlAttributesAttributeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsoadminInstanceAccessControlAttributesAttributeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsoadminInstanceAccessControlAttributesAttributeElRef {
        SsoadminInstanceAccessControlAttributesAttributeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsoadminInstanceAccessControlAttributesAttributeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsoadminInstanceAccessControlAttributesDynamic {
    attribute: Option<DynamicBlock<SsoadminInstanceAccessControlAttributesAttributeEl>>,
}
