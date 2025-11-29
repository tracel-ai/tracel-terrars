use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSecurityhubStandardsControlAssociationsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    security_control_id: PrimField<String>,
}

struct DataSecurityhubStandardsControlAssociations_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSecurityhubStandardsControlAssociationsData>,
}

#[derive(Clone)]
pub struct DataSecurityhubStandardsControlAssociations(Rc<DataSecurityhubStandardsControlAssociations_>);

impl DataSecurityhubStandardsControlAssociations {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
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

    #[doc = "Get a reference to the value of field `security_control_id` after provisioning.\n"]
    pub fn security_control_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_control_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `standards_control_associations` after provisioning.\n"]
    pub fn standards_control_associations(
        &self,
    ) -> ListRef<DataSecurityhubStandardsControlAssociationsStandardsControlAssociationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.standards_control_associations", self.extract_ref()))
    }
}

impl Referable for DataSecurityhubStandardsControlAssociations {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataSecurityhubStandardsControlAssociations { }

impl ToListMappable for DataSecurityhubStandardsControlAssociations {
    type O = ListRef<DataSecurityhubStandardsControlAssociationsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSecurityhubStandardsControlAssociations_ {
    fn extract_datasource_type(&self) -> String {
        "aws_securityhub_standards_control_associations".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSecurityhubStandardsControlAssociations {
    pub tf_id: String,
    #[doc = ""]
    pub security_control_id: PrimField<String>,
}

impl BuildDataSecurityhubStandardsControlAssociations {
    pub fn build(self, stack: &mut Stack) -> DataSecurityhubStandardsControlAssociations {
        let out = DataSecurityhubStandardsControlAssociations(Rc::new(DataSecurityhubStandardsControlAssociations_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSecurityhubStandardsControlAssociationsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
                security_control_id: self.security_control_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSecurityhubStandardsControlAssociationsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSecurityhubStandardsControlAssociationsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataSecurityhubStandardsControlAssociationsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `security_control_id` after provisioning.\n"]
    pub fn security_control_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_control_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `standards_control_associations` after provisioning.\n"]
    pub fn standards_control_associations(
        &self,
    ) -> ListRef<DataSecurityhubStandardsControlAssociationsStandardsControlAssociationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.standards_control_associations", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSecurityhubStandardsControlAssociationsStandardsControlAssociationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    association_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_requirements: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_control_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_control_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standards_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standards_control_description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    standards_control_title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_reason: Option<PrimField<String>>,
}

impl DataSecurityhubStandardsControlAssociationsStandardsControlAssociationsEl {
    #[doc = "Set the field `association_status`.\n"]
    pub fn set_association_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.association_status = Some(v.into());
        self
    }

    #[doc = "Set the field `related_requirements`.\n"]
    pub fn set_related_requirements(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.related_requirements = Some(v.into());
        self
    }

    #[doc = "Set the field `security_control_arn`.\n"]
    pub fn set_security_control_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_control_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `security_control_id`.\n"]
    pub fn set_security_control_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_control_id = Some(v.into());
        self
    }

    #[doc = "Set the field `standards_arn`.\n"]
    pub fn set_standards_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.standards_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `standards_control_description`.\n"]
    pub fn set_standards_control_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.standards_control_description = Some(v.into());
        self
    }

    #[doc = "Set the field `standards_control_title`.\n"]
    pub fn set_standards_control_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.standards_control_title = Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.updated_at = Some(v.into());
        self
    }

    #[doc = "Set the field `updated_reason`.\n"]
    pub fn set_updated_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.updated_reason = Some(v.into());
        self
    }
}

impl ToListMappable for DataSecurityhubStandardsControlAssociationsStandardsControlAssociationsEl {
    type O = BlockAssignable<DataSecurityhubStandardsControlAssociationsStandardsControlAssociationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecurityhubStandardsControlAssociationsStandardsControlAssociationsEl {}

impl BuildDataSecurityhubStandardsControlAssociationsStandardsControlAssociationsEl {
    pub fn build(self) -> DataSecurityhubStandardsControlAssociationsStandardsControlAssociationsEl {
        DataSecurityhubStandardsControlAssociationsStandardsControlAssociationsEl {
            association_status: core::default::Default::default(),
            related_requirements: core::default::Default::default(),
            security_control_arn: core::default::Default::default(),
            security_control_id: core::default::Default::default(),
            standards_arn: core::default::Default::default(),
            standards_control_description: core::default::Default::default(),
            standards_control_title: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            updated_reason: core::default::Default::default(),
        }
    }
}

pub struct DataSecurityhubStandardsControlAssociationsStandardsControlAssociationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSecurityhubStandardsControlAssociationsStandardsControlAssociationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSecurityhubStandardsControlAssociationsStandardsControlAssociationsElRef {
        DataSecurityhubStandardsControlAssociationsStandardsControlAssociationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecurityhubStandardsControlAssociationsStandardsControlAssociationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `association_status` after provisioning.\n"]
    pub fn association_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.association_status", self.base))
    }

    #[doc = "Get a reference to the value of field `related_requirements` after provisioning.\n"]
    pub fn related_requirements(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.related_requirements", self.base))
    }

    #[doc = "Get a reference to the value of field `security_control_arn` after provisioning.\n"]
    pub fn security_control_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_control_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `security_control_id` after provisioning.\n"]
    pub fn security_control_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_control_id", self.base))
    }

    #[doc = "Get a reference to the value of field `standards_arn` after provisioning.\n"]
    pub fn standards_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.standards_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `standards_control_description` after provisioning.\n"]
    pub fn standards_control_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.standards_control_description", self.base))
    }

    #[doc = "Get a reference to the value of field `standards_control_title` after provisioning.\n"]
    pub fn standards_control_title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.standards_control_title", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_reason` after provisioning.\n"]
    pub fn updated_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_reason", self.base))
    }
}
