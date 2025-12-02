use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DatazoneGlossaryTermData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_identifier: Option<PrimField<String>>,
    glossary_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    long_description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    term_relations: Option<Vec<DatazoneGlossaryTermTermRelationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DatazoneGlossaryTermTimeoutsEl>,
    dynamic: DatazoneGlossaryTermDynamic,
}
struct DatazoneGlossaryTerm_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatazoneGlossaryTermData>,
}
#[derive(Clone)]
pub struct DatazoneGlossaryTerm(Rc<DatazoneGlossaryTerm_>);
impl DatazoneGlossaryTerm {
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
    #[doc = "Set the field `domain_identifier`.\n"]
    pub fn set_domain_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `long_description`.\n"]
    pub fn set_long_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().long_description = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `short_description`.\n"]
    pub fn set_short_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().short_description = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().status = Some(v.into());
        self
    }
    #[doc = "Set the field `term_relations`.\n"]
    pub fn set_term_relations(
        self,
        v: impl Into<BlockAssignable<DatazoneGlossaryTermTermRelationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().term_relations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.term_relations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DatazoneGlossaryTermTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `domain_identifier` after provisioning.\n"]
    pub fn domain_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `glossary_identifier` after provisioning.\n"]
    pub fn glossary_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.glossary_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `long_description` after provisioning.\n"]
    pub fn long_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.long_description", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `short_description` after provisioning.\n"]
    pub fn short_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.short_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `term_relations` after provisioning.\n"]
    pub fn term_relations(&self) -> ListRef<DatazoneGlossaryTermTermRelationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.term_relations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatazoneGlossaryTermTimeoutsElRef {
        DatazoneGlossaryTermTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DatazoneGlossaryTerm {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DatazoneGlossaryTerm {}
impl ToListMappable for DatazoneGlossaryTerm {
    type O = ListRef<DatazoneGlossaryTermRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DatazoneGlossaryTerm_ {
    fn extract_resource_type(&self) -> String {
        "aws_datazone_glossary_term".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDatazoneGlossaryTerm {
    pub tf_id: String,
    #[doc = ""]
    pub glossary_identifier: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildDatazoneGlossaryTerm {
    pub fn build(self, stack: &mut Stack) -> DatazoneGlossaryTerm {
        let out = DatazoneGlossaryTerm(Rc::new(DatazoneGlossaryTerm_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatazoneGlossaryTermData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                domain_identifier: core::default::Default::default(),
                glossary_identifier: self.glossary_identifier,
                long_description: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                short_description: core::default::Default::default(),
                status: core::default::Default::default(),
                term_relations: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DatazoneGlossaryTermRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneGlossaryTermRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DatazoneGlossaryTermRef {
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
    #[doc = "Get a reference to the value of field `domain_identifier` after provisioning.\n"]
    pub fn domain_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `glossary_identifier` after provisioning.\n"]
    pub fn glossary_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.glossary_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `long_description` after provisioning.\n"]
    pub fn long_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.long_description", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `short_description` after provisioning.\n"]
    pub fn short_description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.short_description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `term_relations` after provisioning.\n"]
    pub fn term_relations(&self) -> ListRef<DatazoneGlossaryTermTermRelationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.term_relations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatazoneGlossaryTermTimeoutsElRef {
        DatazoneGlossaryTermTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DatazoneGlossaryTermTermRelationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    classifies: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_a: Option<SetField<PrimField<String>>>,
}
impl DatazoneGlossaryTermTermRelationsEl {
    #[doc = "Set the field `classifies`.\n"]
    pub fn set_classifies(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.classifies = Some(v.into());
        self
    }
    #[doc = "Set the field `is_a`.\n"]
    pub fn set_is_a(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.is_a = Some(v.into());
        self
    }
}
impl ToListMappable for DatazoneGlossaryTermTermRelationsEl {
    type O = BlockAssignable<DatazoneGlossaryTermTermRelationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatazoneGlossaryTermTermRelationsEl {}
impl BuildDatazoneGlossaryTermTermRelationsEl {
    pub fn build(self) -> DatazoneGlossaryTermTermRelationsEl {
        DatazoneGlossaryTermTermRelationsEl {
            classifies: core::default::Default::default(),
            is_a: core::default::Default::default(),
        }
    }
}
pub struct DatazoneGlossaryTermTermRelationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneGlossaryTermTermRelationsElRef {
    fn new(shared: StackShared, base: String) -> DatazoneGlossaryTermTermRelationsElRef {
        DatazoneGlossaryTermTermRelationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatazoneGlossaryTermTermRelationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `classifies` after provisioning.\n"]
    pub fn classifies(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.classifies", self.base))
    }
    #[doc = "Get a reference to the value of field `is_a` after provisioning.\n"]
    pub fn is_a(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.is_a", self.base))
    }
}
#[derive(Serialize)]
pub struct DatazoneGlossaryTermTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}
impl DatazoneGlossaryTermTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}
impl ToListMappable for DatazoneGlossaryTermTimeoutsEl {
    type O = BlockAssignable<DatazoneGlossaryTermTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDatazoneGlossaryTermTimeoutsEl {}
impl BuildDatazoneGlossaryTermTimeoutsEl {
    pub fn build(self) -> DatazoneGlossaryTermTimeoutsEl {
        DatazoneGlossaryTermTimeoutsEl {
            create: core::default::Default::default(),
        }
    }
}
pub struct DatazoneGlossaryTermTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DatazoneGlossaryTermTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DatazoneGlossaryTermTimeoutsElRef {
        DatazoneGlossaryTermTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DatazoneGlossaryTermTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
#[derive(Serialize, Default)]
struct DatazoneGlossaryTermDynamic {
    term_relations: Option<DynamicBlock<DatazoneGlossaryTermTermRelationsEl>>,
}
