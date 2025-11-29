use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2AllowedImagesSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    state: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_criterion: Option<Vec<Ec2AllowedImagesSettingsImageCriterionEl>>,
    dynamic: Ec2AllowedImagesSettingsDynamic,
}

struct Ec2AllowedImagesSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Ec2AllowedImagesSettingsData>,
}

#[derive(Clone)]
pub struct Ec2AllowedImagesSettings(Rc<Ec2AllowedImagesSettings_>);

impl Ec2AllowedImagesSettings {
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

    #[doc = "Set the field `image_criterion`.\n"]
    pub fn set_image_criterion(self, v: impl Into<BlockAssignable<Ec2AllowedImagesSettingsImageCriterionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_criterion = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_criterion = Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_criterion` after provisioning.\n"]
    pub fn image_criterion(&self) -> ListRef<Ec2AllowedImagesSettingsImageCriterionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_criterion", self.extract_ref()))
    }
}

impl Referable for Ec2AllowedImagesSettings {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Ec2AllowedImagesSettings { }

impl ToListMappable for Ec2AllowedImagesSettings {
    type O = ListRef<Ec2AllowedImagesSettingsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Ec2AllowedImagesSettings_ {
    fn extract_resource_type(&self) -> String {
        "aws_ec2_allowed_images_settings".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2AllowedImagesSettings {
    pub tf_id: String,
    #[doc = ""]
    pub state: PrimField<String>,
}

impl BuildEc2AllowedImagesSettings {
    pub fn build(self, stack: &mut Stack) -> Ec2AllowedImagesSettings {
        let out = Ec2AllowedImagesSettings(Rc::new(Ec2AllowedImagesSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Ec2AllowedImagesSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                region: core::default::Default::default(),
                state: self.state,
                image_criterion: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2AllowedImagesSettingsRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2AllowedImagesSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Ec2AllowedImagesSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_criterion` after provisioning.\n"]
    pub fn image_criterion(&self) -> ListRef<Ec2AllowedImagesSettingsImageCriterionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_criterion", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Ec2AllowedImagesSettingsImageCriterionElCreationDateConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_days_since_created: Option<PrimField<f64>>,
}

impl Ec2AllowedImagesSettingsImageCriterionElCreationDateConditionEl {
    #[doc = "Set the field `maximum_days_since_created`.\n"]
    pub fn set_maximum_days_since_created(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_days_since_created = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2AllowedImagesSettingsImageCriterionElCreationDateConditionEl {
    type O = BlockAssignable<Ec2AllowedImagesSettingsImageCriterionElCreationDateConditionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2AllowedImagesSettingsImageCriterionElCreationDateConditionEl {}

impl BuildEc2AllowedImagesSettingsImageCriterionElCreationDateConditionEl {
    pub fn build(self) -> Ec2AllowedImagesSettingsImageCriterionElCreationDateConditionEl {
        Ec2AllowedImagesSettingsImageCriterionElCreationDateConditionEl {
            maximum_days_since_created: core::default::Default::default(),
        }
    }
}

pub struct Ec2AllowedImagesSettingsImageCriterionElCreationDateConditionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2AllowedImagesSettingsImageCriterionElCreationDateConditionElRef {
    fn new(shared: StackShared, base: String) -> Ec2AllowedImagesSettingsImageCriterionElCreationDateConditionElRef {
        Ec2AllowedImagesSettingsImageCriterionElCreationDateConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2AllowedImagesSettingsImageCriterionElCreationDateConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_days_since_created` after provisioning.\n"]
    pub fn maximum_days_since_created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_days_since_created", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2AllowedImagesSettingsImageCriterionElDeprecationTimeConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_days_since_deprecated: Option<PrimField<f64>>,
}

impl Ec2AllowedImagesSettingsImageCriterionElDeprecationTimeConditionEl {
    #[doc = "Set the field `maximum_days_since_deprecated`.\n"]
    pub fn set_maximum_days_since_deprecated(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_days_since_deprecated = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2AllowedImagesSettingsImageCriterionElDeprecationTimeConditionEl {
    type O = BlockAssignable<Ec2AllowedImagesSettingsImageCriterionElDeprecationTimeConditionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2AllowedImagesSettingsImageCriterionElDeprecationTimeConditionEl {}

impl BuildEc2AllowedImagesSettingsImageCriterionElDeprecationTimeConditionEl {
    pub fn build(self) -> Ec2AllowedImagesSettingsImageCriterionElDeprecationTimeConditionEl {
        Ec2AllowedImagesSettingsImageCriterionElDeprecationTimeConditionEl {
            maximum_days_since_deprecated: core::default::Default::default(),
        }
    }
}

pub struct Ec2AllowedImagesSettingsImageCriterionElDeprecationTimeConditionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2AllowedImagesSettingsImageCriterionElDeprecationTimeConditionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Ec2AllowedImagesSettingsImageCriterionElDeprecationTimeConditionElRef {
        Ec2AllowedImagesSettingsImageCriterionElDeprecationTimeConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2AllowedImagesSettingsImageCriterionElDeprecationTimeConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_days_since_deprecated` after provisioning.\n"]
    pub fn maximum_days_since_deprecated(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_days_since_deprecated", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2AllowedImagesSettingsImageCriterionElDynamic {
    creation_date_condition: Option<DynamicBlock<Ec2AllowedImagesSettingsImageCriterionElCreationDateConditionEl>>,
    deprecation_time_condition: Option<
        DynamicBlock<Ec2AllowedImagesSettingsImageCriterionElDeprecationTimeConditionEl>,
    >,
}

#[derive(Serialize)]
pub struct Ec2AllowedImagesSettingsImageCriterionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_names: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_providers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marketplace_product_codes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_date_condition: Option<Vec<Ec2AllowedImagesSettingsImageCriterionElCreationDateConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deprecation_time_condition: Option<Vec<Ec2AllowedImagesSettingsImageCriterionElDeprecationTimeConditionEl>>,
    dynamic: Ec2AllowedImagesSettingsImageCriterionElDynamic,
}

impl Ec2AllowedImagesSettingsImageCriterionEl {
    #[doc = "Set the field `image_names`.\n"]
    pub fn set_image_names(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.image_names = Some(v.into());
        self
    }

    #[doc = "Set the field `image_providers`.\n"]
    pub fn set_image_providers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.image_providers = Some(v.into());
        self
    }

    #[doc = "Set the field `marketplace_product_codes`.\n"]
    pub fn set_marketplace_product_codes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.marketplace_product_codes = Some(v.into());
        self
    }

    #[doc = "Set the field `creation_date_condition`.\n"]
    pub fn set_creation_date_condition(
        mut self,
        v: impl Into<BlockAssignable<Ec2AllowedImagesSettingsImageCriterionElCreationDateConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.creation_date_condition = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.creation_date_condition = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deprecation_time_condition`.\n"]
    pub fn set_deprecation_time_condition(
        mut self,
        v: impl Into<BlockAssignable<Ec2AllowedImagesSettingsImageCriterionElDeprecationTimeConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.deprecation_time_condition = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.deprecation_time_condition = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for Ec2AllowedImagesSettingsImageCriterionEl {
    type O = BlockAssignable<Ec2AllowedImagesSettingsImageCriterionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2AllowedImagesSettingsImageCriterionEl {}

impl BuildEc2AllowedImagesSettingsImageCriterionEl {
    pub fn build(self) -> Ec2AllowedImagesSettingsImageCriterionEl {
        Ec2AllowedImagesSettingsImageCriterionEl {
            image_names: core::default::Default::default(),
            image_providers: core::default::Default::default(),
            marketplace_product_codes: core::default::Default::default(),
            creation_date_condition: core::default::Default::default(),
            deprecation_time_condition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Ec2AllowedImagesSettingsImageCriterionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2AllowedImagesSettingsImageCriterionElRef {
    fn new(shared: StackShared, base: String) -> Ec2AllowedImagesSettingsImageCriterionElRef {
        Ec2AllowedImagesSettingsImageCriterionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2AllowedImagesSettingsImageCriterionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_names` after provisioning.\n"]
    pub fn image_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.image_names", self.base))
    }

    #[doc = "Get a reference to the value of field `image_providers` after provisioning.\n"]
    pub fn image_providers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.image_providers", self.base))
    }

    #[doc = "Get a reference to the value of field `marketplace_product_codes` after provisioning.\n"]
    pub fn marketplace_product_codes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.marketplace_product_codes", self.base))
    }

    #[doc = "Get a reference to the value of field `creation_date_condition` after provisioning.\n"]
    pub fn creation_date_condition(&self) -> ListRef<Ec2AllowedImagesSettingsImageCriterionElCreationDateConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.creation_date_condition", self.base))
    }

    #[doc = "Get a reference to the value of field `deprecation_time_condition` after provisioning.\n"]
    pub fn deprecation_time_condition(
        &self,
    ) -> ListRef<Ec2AllowedImagesSettingsImageCriterionElDeprecationTimeConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deprecation_time_condition", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2AllowedImagesSettingsDynamic {
    image_criterion: Option<DynamicBlock<Ec2AllowedImagesSettingsImageCriterionEl>>,
}
