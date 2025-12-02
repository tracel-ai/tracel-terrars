use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GuarddutyOrganizationConfigurationFeatureData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    auto_enable: PrimField<String>,
    detector_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_configuration:
        Option<Vec<GuarddutyOrganizationConfigurationFeatureAdditionalConfigurationEl>>,
    dynamic: GuarddutyOrganizationConfigurationFeatureDynamic,
}
struct GuarddutyOrganizationConfigurationFeature_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GuarddutyOrganizationConfigurationFeatureData>,
}
#[derive(Clone)]
pub struct GuarddutyOrganizationConfigurationFeature(
    Rc<GuarddutyOrganizationConfigurationFeature_>,
);
impl GuarddutyOrganizationConfigurationFeature {
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
    #[doc = "Set the field `additional_configuration`.\n"]
    pub fn set_additional_configuration(
        self,
        v: impl Into<
            BlockAssignable<GuarddutyOrganizationConfigurationFeatureAdditionalConfigurationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().additional_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.additional_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `auto_enable` after provisioning.\n"]
    pub fn auto_enable(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_enable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `detector_id` after provisioning.\n"]
    pub fn detector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.detector_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `additional_configuration` after provisioning.\n"]
    pub fn additional_configuration(
        &self,
    ) -> ListRef<GuarddutyOrganizationConfigurationFeatureAdditionalConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_configuration", self.extract_ref()),
        )
    }
}
impl Referable for GuarddutyOrganizationConfigurationFeature {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GuarddutyOrganizationConfigurationFeature {}
impl ToListMappable for GuarddutyOrganizationConfigurationFeature {
    type O = ListRef<GuarddutyOrganizationConfigurationFeatureRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GuarddutyOrganizationConfigurationFeature_ {
    fn extract_resource_type(&self) -> String {
        "aws_guardduty_organization_configuration_feature".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGuarddutyOrganizationConfigurationFeature {
    pub tf_id: String,
    #[doc = ""]
    pub auto_enable: PrimField<String>,
    #[doc = ""]
    pub detector_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildGuarddutyOrganizationConfigurationFeature {
    pub fn build(self, stack: &mut Stack) -> GuarddutyOrganizationConfigurationFeature {
        let out = GuarddutyOrganizationConfigurationFeature(Rc::new(
            GuarddutyOrganizationConfigurationFeature_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(GuarddutyOrganizationConfigurationFeatureData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    auto_enable: self.auto_enable,
                    detector_id: self.detector_id,
                    id: core::default::Default::default(),
                    name: self.name,
                    region: core::default::Default::default(),
                    additional_configuration: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GuarddutyOrganizationConfigurationFeatureRef {
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyOrganizationConfigurationFeatureRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GuarddutyOrganizationConfigurationFeatureRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_enable` after provisioning.\n"]
    pub fn auto_enable(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_enable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `detector_id` after provisioning.\n"]
    pub fn detector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.detector_id", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `additional_configuration` after provisioning.\n"]
    pub fn additional_configuration(
        &self,
    ) -> ListRef<GuarddutyOrganizationConfigurationFeatureAdditionalConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_configuration", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GuarddutyOrganizationConfigurationFeatureAdditionalConfigurationEl {
    auto_enable: PrimField<String>,
    name: PrimField<String>,
}
impl GuarddutyOrganizationConfigurationFeatureAdditionalConfigurationEl {}
impl ToListMappable for GuarddutyOrganizationConfigurationFeatureAdditionalConfigurationEl {
    type O = BlockAssignable<GuarddutyOrganizationConfigurationFeatureAdditionalConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGuarddutyOrganizationConfigurationFeatureAdditionalConfigurationEl {
    #[doc = ""]
    pub auto_enable: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildGuarddutyOrganizationConfigurationFeatureAdditionalConfigurationEl {
    pub fn build(self) -> GuarddutyOrganizationConfigurationFeatureAdditionalConfigurationEl {
        GuarddutyOrganizationConfigurationFeatureAdditionalConfigurationEl {
            auto_enable: self.auto_enable,
            name: self.name,
        }
    }
}
pub struct GuarddutyOrganizationConfigurationFeatureAdditionalConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyOrganizationConfigurationFeatureAdditionalConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GuarddutyOrganizationConfigurationFeatureAdditionalConfigurationElRef {
        GuarddutyOrganizationConfigurationFeatureAdditionalConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GuarddutyOrganizationConfigurationFeatureAdditionalConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_enable` after provisioning.\n"]
    pub fn auto_enable(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_enable", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct GuarddutyOrganizationConfigurationFeatureDynamic {
    additional_configuration:
        Option<DynamicBlock<GuarddutyOrganizationConfigurationFeatureAdditionalConfigurationEl>>,
}
