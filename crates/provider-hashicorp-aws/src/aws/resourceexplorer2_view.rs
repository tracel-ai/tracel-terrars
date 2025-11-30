use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct Resourceexplorer2ViewData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_view: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<Vec<Resourceexplorer2ViewFiltersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    included_property: Option<Vec<Resourceexplorer2ViewIncludedPropertyEl>>,
    dynamic: Resourceexplorer2ViewDynamic,
}

struct Resourceexplorer2View_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Resourceexplorer2ViewData>,
}

#[derive(Clone)]
pub struct Resourceexplorer2View(Rc<Resourceexplorer2View_>);

impl Resourceexplorer2View {
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

    #[doc = "Set the field `default_view`.\n"]
    pub fn set_default_view(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default_view = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().scope = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(
        self,
        v: impl Into<BlockAssignable<Resourceexplorer2ViewFiltersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filters = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `included_property`.\n"]
    pub fn set_included_property(
        self,
        v: impl Into<BlockAssignable<Resourceexplorer2ViewIncludedPropertyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().included_property = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.included_property = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_view` after provisioning.\n"]
    pub fn default_view(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_view", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> ListRef<Resourceexplorer2ViewFiltersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `included_property` after provisioning.\n"]
    pub fn included_property(&self) -> ListRef<Resourceexplorer2ViewIncludedPropertyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_property", self.extract_ref()),
        )
    }
}

impl Referable for Resourceexplorer2View {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for Resourceexplorer2View {}

impl ToListMappable for Resourceexplorer2View {
    type O = ListRef<Resourceexplorer2ViewRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Resourceexplorer2View_ {
    fn extract_resource_type(&self) -> String {
        "aws_resourceexplorer2_view".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildResourceexplorer2View {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildResourceexplorer2View {
    pub fn build(self, stack: &mut Stack) -> Resourceexplorer2View {
        let out = Resourceexplorer2View(Rc::new(Resourceexplorer2View_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Resourceexplorer2ViewData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                default_view: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                scope: core::default::Default::default(),
                tags: core::default::Default::default(),
                filters: core::default::Default::default(),
                included_property: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Resourceexplorer2ViewRef {
    shared: StackShared,
    base: String,
}

impl Ref for Resourceexplorer2ViewRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl Resourceexplorer2ViewRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_view` after provisioning.\n"]
    pub fn default_view(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_view", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> ListRef<Resourceexplorer2ViewFiltersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `included_property` after provisioning.\n"]
    pub fn included_property(&self) -> ListRef<Resourceexplorer2ViewIncludedPropertyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.included_property", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Resourceexplorer2ViewFiltersEl {
    filter_string: PrimField<String>,
}

impl Resourceexplorer2ViewFiltersEl {}

impl ToListMappable for Resourceexplorer2ViewFiltersEl {
    type O = BlockAssignable<Resourceexplorer2ViewFiltersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResourceexplorer2ViewFiltersEl {
    #[doc = ""]
    pub filter_string: PrimField<String>,
}

impl BuildResourceexplorer2ViewFiltersEl {
    pub fn build(self) -> Resourceexplorer2ViewFiltersEl {
        Resourceexplorer2ViewFiltersEl {
            filter_string: self.filter_string,
        }
    }
}

pub struct Resourceexplorer2ViewFiltersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Resourceexplorer2ViewFiltersElRef {
    fn new(shared: StackShared, base: String) -> Resourceexplorer2ViewFiltersElRef {
        Resourceexplorer2ViewFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Resourceexplorer2ViewFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter_string` after provisioning.\n"]
    pub fn filter_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_string", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Resourceexplorer2ViewIncludedPropertyEl {
    name: PrimField<String>,
}

impl Resourceexplorer2ViewIncludedPropertyEl {}

impl ToListMappable for Resourceexplorer2ViewIncludedPropertyEl {
    type O = BlockAssignable<Resourceexplorer2ViewIncludedPropertyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResourceexplorer2ViewIncludedPropertyEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildResourceexplorer2ViewIncludedPropertyEl {
    pub fn build(self) -> Resourceexplorer2ViewIncludedPropertyEl {
        Resourceexplorer2ViewIncludedPropertyEl { name: self.name }
    }
}

pub struct Resourceexplorer2ViewIncludedPropertyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Resourceexplorer2ViewIncludedPropertyElRef {
    fn new(shared: StackShared, base: String) -> Resourceexplorer2ViewIncludedPropertyElRef {
        Resourceexplorer2ViewIncludedPropertyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Resourceexplorer2ViewIncludedPropertyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct Resourceexplorer2ViewDynamic {
    filters: Option<DynamicBlock<Resourceexplorer2ViewFiltersEl>>,
    included_property: Option<DynamicBlock<Resourceexplorer2ViewIncludedPropertyEl>>,
}
