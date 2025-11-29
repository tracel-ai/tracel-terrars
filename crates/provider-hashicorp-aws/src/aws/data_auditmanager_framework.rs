use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAuditmanagerFrameworkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    framework_type: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataAuditmanagerFramework_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAuditmanagerFrameworkData>,
}

#[derive(Clone)]
pub struct DataAuditmanagerFramework(Rc<DataAuditmanagerFramework_>);

impl DataAuditmanagerFramework {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compliance_type` after provisioning.\n"]
    pub fn compliance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compliance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_sets` after provisioning.\n"]
    pub fn control_sets(&self) -> ListRef<DataAuditmanagerFrameworkControlSetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.control_sets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `framework_type` after provisioning.\n"]
    pub fn framework_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.framework_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Referable for DataAuditmanagerFramework {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataAuditmanagerFramework { }

impl ToListMappable for DataAuditmanagerFramework {
    type O = ListRef<DataAuditmanagerFrameworkRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAuditmanagerFramework_ {
    fn extract_datasource_type(&self) -> String {
        "aws_auditmanager_framework".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAuditmanagerFramework {
    pub tf_id: String,
    #[doc = ""]
    pub framework_type: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataAuditmanagerFramework {
    pub fn build(self, stack: &mut Stack) -> DataAuditmanagerFramework {
        let out = DataAuditmanagerFramework(Rc::new(DataAuditmanagerFramework_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAuditmanagerFrameworkData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                framework_type: self.framework_type,
                name: self.name,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAuditmanagerFrameworkRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAuditmanagerFrameworkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataAuditmanagerFrameworkRef {
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

    #[doc = "Get a reference to the value of field `compliance_type` after provisioning.\n"]
    pub fn compliance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compliance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_sets` after provisioning.\n"]
    pub fn control_sets(&self) -> ListRef<DataAuditmanagerFrameworkControlSetsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.control_sets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `framework_type` after provisioning.\n"]
    pub fn framework_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.framework_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAuditmanagerFrameworkControlSetsElControlsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

impl DataAuditmanagerFrameworkControlSetsElControlsEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
}

impl ToListMappable for DataAuditmanagerFrameworkControlSetsElControlsEl {
    type O = BlockAssignable<DataAuditmanagerFrameworkControlSetsElControlsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAuditmanagerFrameworkControlSetsElControlsEl {}

impl BuildDataAuditmanagerFrameworkControlSetsElControlsEl {
    pub fn build(self) -> DataAuditmanagerFrameworkControlSetsElControlsEl {
        DataAuditmanagerFrameworkControlSetsElControlsEl { id: core::default::Default::default() }
    }
}

pub struct DataAuditmanagerFrameworkControlSetsElControlsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAuditmanagerFrameworkControlSetsElControlsElRef {
    fn new(shared: StackShared, base: String) -> DataAuditmanagerFrameworkControlSetsElControlsElRef {
        DataAuditmanagerFrameworkControlSetsElControlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAuditmanagerFrameworkControlSetsElControlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAuditmanagerFrameworkControlSetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    controls: Option<SetField<DataAuditmanagerFrameworkControlSetsElControlsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataAuditmanagerFrameworkControlSetsEl {
    #[doc = "Set the field `controls`.\n"]
    pub fn set_controls(mut self, v: impl Into<SetField<DataAuditmanagerFrameworkControlSetsElControlsEl>>) -> Self {
        self.controls = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataAuditmanagerFrameworkControlSetsEl {
    type O = BlockAssignable<DataAuditmanagerFrameworkControlSetsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAuditmanagerFrameworkControlSetsEl {}

impl BuildDataAuditmanagerFrameworkControlSetsEl {
    pub fn build(self) -> DataAuditmanagerFrameworkControlSetsEl {
        DataAuditmanagerFrameworkControlSetsEl {
            controls: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataAuditmanagerFrameworkControlSetsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAuditmanagerFrameworkControlSetsElRef {
    fn new(shared: StackShared, base: String) -> DataAuditmanagerFrameworkControlSetsElRef {
        DataAuditmanagerFrameworkControlSetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAuditmanagerFrameworkControlSetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `controls` after provisioning.\n"]
    pub fn controls(&self) -> SetRef<DataAuditmanagerFrameworkControlSetsElControlsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.controls", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
