use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GuarddutyMemberDetectorFeatureData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_id: PrimField<String>,
    detector_id: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    status: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_configuration: Option<Vec<GuarddutyMemberDetectorFeatureAdditionalConfigurationEl>>,
    dynamic: GuarddutyMemberDetectorFeatureDynamic,
}

struct GuarddutyMemberDetectorFeature_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GuarddutyMemberDetectorFeatureData>,
}

#[derive(Clone)]
pub struct GuarddutyMemberDetectorFeature(Rc<GuarddutyMemberDetectorFeature_>);

impl GuarddutyMemberDetectorFeature {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `additional_configuration`.\n"]
    pub fn set_additional_configuration(
        self,
        v: impl Into<BlockAssignable<GuarddutyMemberDetectorFeatureAdditionalConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().additional_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.additional_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detector_id` after provisioning.\n"]
    pub fn detector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.detector_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `additional_configuration` after provisioning.\n"]
    pub fn additional_configuration(&self) -> ListRef<GuarddutyMemberDetectorFeatureAdditionalConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.additional_configuration", self.extract_ref()))
    }
}

impl Referable for GuarddutyMemberDetectorFeature {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GuarddutyMemberDetectorFeature { }

impl ToListMappable for GuarddutyMemberDetectorFeature {
    type O = ListRef<GuarddutyMemberDetectorFeatureRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GuarddutyMemberDetectorFeature_ {
    fn extract_resource_type(&self) -> String {
        "aws_guardduty_member_detector_feature".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGuarddutyMemberDetectorFeature {
    pub tf_id: String,
    #[doc = ""]
    pub account_id: PrimField<String>,
    #[doc = ""]
    pub detector_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub status: PrimField<String>,
}

impl BuildGuarddutyMemberDetectorFeature {
    pub fn build(self, stack: &mut Stack) -> GuarddutyMemberDetectorFeature {
        let out = GuarddutyMemberDetectorFeature(Rc::new(GuarddutyMemberDetectorFeature_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GuarddutyMemberDetectorFeatureData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_id: self.account_id,
                detector_id: self.detector_id,
                name: self.name,
                region: core::default::Default::default(),
                status: self.status,
                additional_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GuarddutyMemberDetectorFeatureRef {
    shared: StackShared,
    base: String,
}

impl Ref for GuarddutyMemberDetectorFeatureRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl GuarddutyMemberDetectorFeatureRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detector_id` after provisioning.\n"]
    pub fn detector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.detector_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `additional_configuration` after provisioning.\n"]
    pub fn additional_configuration(&self) -> ListRef<GuarddutyMemberDetectorFeatureAdditionalConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.additional_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GuarddutyMemberDetectorFeatureAdditionalConfigurationEl {
    name: PrimField<String>,
    status: PrimField<String>,
}

impl GuarddutyMemberDetectorFeatureAdditionalConfigurationEl { }

impl ToListMappable for GuarddutyMemberDetectorFeatureAdditionalConfigurationEl {
    type O = BlockAssignable<GuarddutyMemberDetectorFeatureAdditionalConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyMemberDetectorFeatureAdditionalConfigurationEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub status: PrimField<String>,
}

impl BuildGuarddutyMemberDetectorFeatureAdditionalConfigurationEl {
    pub fn build(self) -> GuarddutyMemberDetectorFeatureAdditionalConfigurationEl {
        GuarddutyMemberDetectorFeatureAdditionalConfigurationEl {
            name: self.name,
            status: self.status,
        }
    }
}

pub struct GuarddutyMemberDetectorFeatureAdditionalConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GuarddutyMemberDetectorFeatureAdditionalConfigurationElRef {
    fn new(shared: StackShared, base: String) -> GuarddutyMemberDetectorFeatureAdditionalConfigurationElRef {
        GuarddutyMemberDetectorFeatureAdditionalConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyMemberDetectorFeatureAdditionalConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct GuarddutyMemberDetectorFeatureDynamic {
    additional_configuration: Option<DynamicBlock<GuarddutyMemberDetectorFeatureAdditionalConfigurationEl>>,
}
