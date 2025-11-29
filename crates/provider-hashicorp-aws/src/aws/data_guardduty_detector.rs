use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataGuarddutyDetectorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataGuarddutyDetector_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataGuarddutyDetectorData>,
}

#[derive(Clone)]
pub struct DataGuarddutyDetector(Rc<DataGuarddutyDetector_>);

impl DataGuarddutyDetector {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `features` after provisioning.\n"]
    pub fn features(&self) -> ListRef<DataGuarddutyDetectorFeaturesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.features", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `finding_publishing_frequency` after provisioning.\n"]
    pub fn finding_publishing_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.finding_publishing_frequency", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Referable for DataGuarddutyDetector {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataGuarddutyDetector { }

impl ToListMappable for DataGuarddutyDetector {
    type O = ListRef<DataGuarddutyDetectorRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataGuarddutyDetector_ {
    fn extract_datasource_type(&self) -> String {
        "aws_guardduty_detector".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataGuarddutyDetector {
    pub tf_id: String,
}

impl BuildDataGuarddutyDetector {
    pub fn build(self, stack: &mut Stack) -> DataGuarddutyDetector {
        let out = DataGuarddutyDetector(Rc::new(DataGuarddutyDetector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataGuarddutyDetectorData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataGuarddutyDetectorRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGuarddutyDetectorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataGuarddutyDetectorRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `features` after provisioning.\n"]
    pub fn features(&self) -> ListRef<DataGuarddutyDetectorFeaturesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.features", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `finding_publishing_frequency` after provisioning.\n"]
    pub fn finding_publishing_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.finding_publishing_frequency", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataGuarddutyDetectorFeaturesElAdditionalConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl DataGuarddutyDetectorFeaturesElAdditionalConfigurationEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for DataGuarddutyDetectorFeaturesElAdditionalConfigurationEl {
    type O = BlockAssignable<DataGuarddutyDetectorFeaturesElAdditionalConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGuarddutyDetectorFeaturesElAdditionalConfigurationEl {}

impl BuildDataGuarddutyDetectorFeaturesElAdditionalConfigurationEl {
    pub fn build(self) -> DataGuarddutyDetectorFeaturesElAdditionalConfigurationEl {
        DataGuarddutyDetectorFeaturesElAdditionalConfigurationEl {
            name: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct DataGuarddutyDetectorFeaturesElAdditionalConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGuarddutyDetectorFeaturesElAdditionalConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataGuarddutyDetectorFeaturesElAdditionalConfigurationElRef {
        DataGuarddutyDetectorFeaturesElAdditionalConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGuarddutyDetectorFeaturesElAdditionalConfigurationElRef {
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

#[derive(Serialize)]
pub struct DataGuarddutyDetectorFeaturesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_configuration: Option<ListField<DataGuarddutyDetectorFeaturesElAdditionalConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl DataGuarddutyDetectorFeaturesEl {
    #[doc = "Set the field `additional_configuration`.\n"]
    pub fn set_additional_configuration(
        mut self,
        v: impl Into<ListField<DataGuarddutyDetectorFeaturesElAdditionalConfigurationEl>>,
    ) -> Self {
        self.additional_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for DataGuarddutyDetectorFeaturesEl {
    type O = BlockAssignable<DataGuarddutyDetectorFeaturesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGuarddutyDetectorFeaturesEl {}

impl BuildDataGuarddutyDetectorFeaturesEl {
    pub fn build(self) -> DataGuarddutyDetectorFeaturesEl {
        DataGuarddutyDetectorFeaturesEl {
            additional_configuration: core::default::Default::default(),
            name: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct DataGuarddutyDetectorFeaturesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataGuarddutyDetectorFeaturesElRef {
    fn new(shared: StackShared, base: String) -> DataGuarddutyDetectorFeaturesElRef {
        DataGuarddutyDetectorFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGuarddutyDetectorFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_configuration` after provisioning.\n"]
    pub fn additional_configuration(&self) -> ListRef<DataGuarddutyDetectorFeaturesElAdditionalConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.additional_configuration", self.base))
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
