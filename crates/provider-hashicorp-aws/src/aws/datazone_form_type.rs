use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DatazoneFormTypeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    domain_identifier: PrimField<String>,
    name: PrimField<String>,
    owning_project_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<Vec<DatazoneFormTypeModelEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DatazoneFormTypeTimeoutsEl>,
    dynamic: DatazoneFormTypeDynamic,
}
struct DatazoneFormType_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatazoneFormTypeData>,
}
#[derive(Clone)]
pub struct DatazoneFormType(Rc<DatazoneFormType_>);
impl DatazoneFormType {
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
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
        self
    }
    #[doc = "Set the field `model`.\n"]
    pub fn set_model(self, v: impl Into<BlockAssignable<DatazoneFormTypeModelEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().model = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.model = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DatazoneFormTypeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_by", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_identifier` after provisioning.\n"]
    pub fn domain_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `imports` after provisioning.\n"]
    pub fn imports(&self) -> ListRef<DatazoneFormTypeImportsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.imports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `origin_domain_id` after provisioning.\n"]
    pub fn origin_domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_domain_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `origin_project_id` after provisioning.\n"]
    pub fn origin_project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_project_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owning_project_identifier` after provisioning.\n"]
    pub fn owning_project_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owning_project_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> ListRef<DatazoneFormTypeModelElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatazoneFormTypeTimeoutsElRef {
        DatazoneFormTypeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DatazoneFormType {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DatazoneFormType {}
impl ToListMappable for DatazoneFormType {
    type O = ListRef<DatazoneFormTypeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DatazoneFormType_ {
    fn extract_resource_type(&self) -> String {
        "aws_datazone_form_type".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDatazoneFormType {
    pub tf_id: String,
    #[doc = ""]
    pub domain_identifier: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub owning_project_identifier: PrimField<String>,
}
impl BuildDatazoneFormType {
    pub fn build(self, stack: &mut Stack) -> DatazoneFormType {
        let out = DatazoneFormType(Rc::new(DatazoneFormType_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatazoneFormTypeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                domain_identifier: self.domain_identifier,
                name: self.name,
                owning_project_identifier: self.owning_project_identifier,
                region: core::default::Default::default(),
                status: core::default::Default::default(),
                model: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DatazoneFormTypeRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneFormTypeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DatazoneFormTypeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_by", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_identifier` after provisioning.\n"]
    pub fn domain_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `imports` after provisioning.\n"]
    pub fn imports(&self) -> ListRef<DatazoneFormTypeImportsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.imports", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `origin_domain_id` after provisioning.\n"]
    pub fn origin_domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_domain_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `origin_project_id` after provisioning.\n"]
    pub fn origin_project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_project_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owning_project_identifier` after provisioning.\n"]
    pub fn owning_project_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owning_project_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.revision", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> ListRef<DatazoneFormTypeModelElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatazoneFormTypeTimeoutsElRef {
        DatazoneFormTypeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DatazoneFormTypeImportsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<String>>,
}
impl DatazoneFormTypeImportsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `revision`.\n"]
    pub fn set_revision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision = Some(v.into());
        self
    }
}
impl ToListMappable for DatazoneFormTypeImportsEl {
    type O = BlockAssignable<DatazoneFormTypeImportsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatazoneFormTypeImportsEl {}
impl BuildDatazoneFormTypeImportsEl {
    pub fn build(self) -> DatazoneFormTypeImportsEl {
        DatazoneFormTypeImportsEl {
            name: core::default::Default::default(),
            revision: core::default::Default::default(),
        }
    }
}
pub struct DatazoneFormTypeImportsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneFormTypeImportsElRef {
    fn new(shared: StackShared, base: String) -> DatazoneFormTypeImportsElRef {
        DatazoneFormTypeImportsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatazoneFormTypeImportsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
}
#[derive(Serialize)]
pub struct DatazoneFormTypeModelEl {
    smithy: PrimField<String>,
}
impl DatazoneFormTypeModelEl {}
impl ToListMappable for DatazoneFormTypeModelEl {
    type O = BlockAssignable<DatazoneFormTypeModelEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatazoneFormTypeModelEl {
    #[doc = ""]
    pub smithy: PrimField<String>,
}
impl BuildDatazoneFormTypeModelEl {
    pub fn build(self) -> DatazoneFormTypeModelEl {
        DatazoneFormTypeModelEl {
            smithy: self.smithy,
        }
    }
}
pub struct DatazoneFormTypeModelElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneFormTypeModelElRef {
    fn new(shared: StackShared, base: String) -> DatazoneFormTypeModelElRef {
        DatazoneFormTypeModelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatazoneFormTypeModelElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `smithy` after provisioning.\n"]
    pub fn smithy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.smithy", self.base))
    }
}
#[derive(Serialize)]
pub struct DatazoneFormTypeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}
impl DatazoneFormTypeTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}
impl ToListMappable for DatazoneFormTypeTimeoutsEl {
    type O = BlockAssignable<DatazoneFormTypeTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatazoneFormTypeTimeoutsEl {}
impl BuildDatazoneFormTypeTimeoutsEl {
    pub fn build(self) -> DatazoneFormTypeTimeoutsEl {
        DatazoneFormTypeTimeoutsEl {
            create: core::default::Default::default(),
        }
    }
}
pub struct DatazoneFormTypeTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneFormTypeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DatazoneFormTypeTimeoutsElRef {
        DatazoneFormTypeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatazoneFormTypeTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatazoneFormTypeDynamic {
    model: Option<DynamicBlock<DatazoneFormTypeModelEl>>,
}
