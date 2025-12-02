use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBackupFrameworkData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
struct DataBackupFramework_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBackupFrameworkData>,
}
#[derive(Clone)]
pub struct DataBackupFramework(Rc<DataBackupFramework_>);
impl DataBackupFramework {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
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
    #[doc = "Get a reference to the value of field `control` after provisioning.\n"]
    pub fn control(&self) -> ListRef<DataBackupFrameworkControlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_status` after provisioning.\n"]
    pub fn deployment_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}
impl Referable for DataBackupFramework {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBackupFramework {}
impl ToListMappable for DataBackupFramework {
    type O = ListRef<DataBackupFrameworkRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBackupFramework_ {
    fn extract_datasource_type(&self) -> String {
        "aws_backup_framework".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBackupFramework {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildDataBackupFramework {
    pub fn build(self, stack: &mut Stack) -> DataBackupFramework {
        let out = DataBackupFramework(Rc::new(DataBackupFramework_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBackupFrameworkData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBackupFrameworkRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupFrameworkRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBackupFrameworkRef {
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
    #[doc = "Get a reference to the value of field `control` after provisioning.\n"]
    pub fn control(&self) -> ListRef<DataBackupFrameworkControlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.control", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_status` after provisioning.\n"]
    pub fn deployment_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataBackupFrameworkControlElInputParameterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataBackupFrameworkControlElInputParameterEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupFrameworkControlElInputParameterEl {
    type O = BlockAssignable<DataBackupFrameworkControlElInputParameterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupFrameworkControlElInputParameterEl {}
impl BuildDataBackupFrameworkControlElInputParameterEl {
    pub fn build(self) -> DataBackupFrameworkControlElInputParameterEl {
        DataBackupFrameworkControlElInputParameterEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataBackupFrameworkControlElInputParameterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupFrameworkControlElInputParameterElRef {
    fn new(shared: StackShared, base: String) -> DataBackupFrameworkControlElInputParameterElRef {
        DataBackupFrameworkControlElInputParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupFrameworkControlElInputParameterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBackupFrameworkControlElScopeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    compliance_resource_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compliance_resource_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
impl DataBackupFrameworkControlElScopeEl {
    #[doc = "Set the field `compliance_resource_ids`.\n"]
    pub fn set_compliance_resource_ids(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.compliance_resource_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `compliance_resource_types`.\n"]
    pub fn set_compliance_resource_types(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.compliance_resource_types = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupFrameworkControlElScopeEl {
    type O = BlockAssignable<DataBackupFrameworkControlElScopeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupFrameworkControlElScopeEl {}
impl BuildDataBackupFrameworkControlElScopeEl {
    pub fn build(self) -> DataBackupFrameworkControlElScopeEl {
        DataBackupFrameworkControlElScopeEl {
            compliance_resource_ids: core::default::Default::default(),
            compliance_resource_types: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}
pub struct DataBackupFrameworkControlElScopeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupFrameworkControlElScopeElRef {
    fn new(shared: StackShared, base: String) -> DataBackupFrameworkControlElScopeElRef {
        DataBackupFrameworkControlElScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupFrameworkControlElScopeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `compliance_resource_ids` after provisioning.\n"]
    pub fn compliance_resource_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compliance_resource_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `compliance_resource_types` after provisioning.\n"]
    pub fn compliance_resource_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compliance_resource_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBackupFrameworkControlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_parameter: Option<ListField<DataBackupFrameworkControlElInputParameterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<ListField<DataBackupFrameworkControlElScopeEl>>,
}
impl DataBackupFrameworkControlEl {
    #[doc = "Set the field `input_parameter`.\n"]
    pub fn set_input_parameter(
        mut self,
        v: impl Into<ListField<DataBackupFrameworkControlElInputParameterEl>>,
    ) -> Self {
        self.input_parameter = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(
        mut self,
        v: impl Into<ListField<DataBackupFrameworkControlElScopeEl>>,
    ) -> Self {
        self.scope = Some(v.into());
        self
    }
}
impl ToListMappable for DataBackupFrameworkControlEl {
    type O = BlockAssignable<DataBackupFrameworkControlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBackupFrameworkControlEl {}
impl BuildDataBackupFrameworkControlEl {
    pub fn build(self) -> DataBackupFrameworkControlEl {
        DataBackupFrameworkControlEl {
            input_parameter: core::default::Default::default(),
            name: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}
pub struct DataBackupFrameworkControlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBackupFrameworkControlElRef {
    fn new(shared: StackShared, base: String) -> DataBackupFrameworkControlElRef {
        DataBackupFrameworkControlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBackupFrameworkControlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `input_parameter` after provisioning.\n"]
    pub fn input_parameter(&self) -> ListRef<DataBackupFrameworkControlElInputParameterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_parameter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> ListRef<DataBackupFrameworkControlElScopeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}
