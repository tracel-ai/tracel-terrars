use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct Inspector2FilterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_criteria: Option<Vec<Inspector2FilterFilterCriteriaEl>>,
    dynamic: Inspector2FilterDynamic,
}

struct Inspector2Filter_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Inspector2FilterData>,
}

#[derive(Clone)]
pub struct Inspector2Filter(Rc<Inspector2Filter_>);

impl Inspector2Filter {
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

    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().reason = Some(v.into());
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

    #[doc = "Set the field `filter_criteria`.\n"]
    pub fn set_filter_criteria(
        self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_criteria = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_criteria = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reason", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `filter_criteria` after provisioning.\n"]
    pub fn filter_criteria(&self) -> ListRef<Inspector2FilterFilterCriteriaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_criteria", self.extract_ref()),
        )
    }
}

impl Referable for Inspector2Filter {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for Inspector2Filter {}

impl ToListMappable for Inspector2Filter {
    type O = ListRef<Inspector2FilterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Inspector2Filter_ {
    fn extract_resource_type(&self) -> String {
        "aws_inspector2_filter".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildInspector2Filter {
    pub tf_id: String,
    #[doc = ""]
    pub action: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildInspector2Filter {
    pub fn build(self, stack: &mut Stack) -> Inspector2Filter {
        let out = Inspector2Filter(Rc::new(Inspector2Filter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Inspector2FilterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                action: self.action,
                description: core::default::Default::default(),
                name: self.name,
                reason: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                filter_criteria: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Inspector2FilterRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl Inspector2FilterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reason", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `filter_criteria` after provisioning.\n"]
    pub fn filter_criteria(&self) -> ListRef<Inspector2FilterFilterCriteriaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_criteria", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElAwsAccountIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElAwsAccountIdEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElAwsAccountIdEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElAwsAccountIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElAwsAccountIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElAwsAccountIdEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElAwsAccountIdEl {
        Inspector2FilterFilterCriteriaElAwsAccountIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElAwsAccountIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElAwsAccountIdElRef {
    fn new(shared: StackShared, base: String) -> Inspector2FilterFilterCriteriaElAwsAccountIdElRef {
        Inspector2FilterFilterCriteriaElAwsAccountIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElAwsAccountIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElCodeRepositoryProjectNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElCodeRepositoryProjectNameEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElCodeRepositoryProjectNameEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElCodeRepositoryProjectNameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElCodeRepositoryProjectNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElCodeRepositoryProjectNameEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElCodeRepositoryProjectNameEl {
        Inspector2FilterFilterCriteriaElCodeRepositoryProjectNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElCodeRepositoryProjectNameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElCodeRepositoryProjectNameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElCodeRepositoryProjectNameElRef {
        Inspector2FilterFilterCriteriaElCodeRepositoryProjectNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElCodeRepositoryProjectNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElCodeRepositoryProviderTypeEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElCodeRepositoryProviderTypeEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElCodeRepositoryProviderTypeEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElCodeRepositoryProviderTypeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElCodeRepositoryProviderTypeEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElCodeRepositoryProviderTypeEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElCodeRepositoryProviderTypeEl {
        Inspector2FilterFilterCriteriaElCodeRepositoryProviderTypeEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElCodeRepositoryProviderTypeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElCodeRepositoryProviderTypeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElCodeRepositoryProviderTypeElRef {
        Inspector2FilterFilterCriteriaElCodeRepositoryProviderTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElCodeRepositoryProviderTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorNameEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorNameEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorNameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElCodeVulnerabilityDetectorNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElCodeVulnerabilityDetectorNameEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorNameEl {
        Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorNameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorNameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorNameElRef {
        Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorTagsEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorTagsEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorTagsEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElCodeVulnerabilityDetectorTagsEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElCodeVulnerabilityDetectorTagsEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorTagsEl {
        Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorTagsEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorTagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorTagsElRef {
        Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElCodeVulnerabilityFilePathEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElCodeVulnerabilityFilePathEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElCodeVulnerabilityFilePathEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElCodeVulnerabilityFilePathEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElCodeVulnerabilityFilePathEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElCodeVulnerabilityFilePathEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElCodeVulnerabilityFilePathEl {
        Inspector2FilterFilterCriteriaElCodeVulnerabilityFilePathEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElCodeVulnerabilityFilePathElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElCodeVulnerabilityFilePathElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElCodeVulnerabilityFilePathElRef {
        Inspector2FilterFilterCriteriaElCodeVulnerabilityFilePathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElCodeVulnerabilityFilePathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElComponentIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElComponentIdEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElComponentIdEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElComponentIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElComponentIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElComponentIdEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElComponentIdEl {
        Inspector2FilterFilterCriteriaElComponentIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElComponentIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElComponentIdElRef {
    fn new(shared: StackShared, base: String) -> Inspector2FilterFilterCriteriaElComponentIdElRef {
        Inspector2FilterFilterCriteriaElComponentIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElComponentIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElComponentTypeEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElComponentTypeEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElComponentTypeEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElComponentTypeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElComponentTypeEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElComponentTypeEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElComponentTypeEl {
        Inspector2FilterFilterCriteriaElComponentTypeEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElComponentTypeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElComponentTypeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElComponentTypeElRef {
        Inspector2FilterFilterCriteriaElComponentTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElComponentTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElEc2InstanceImageIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElEc2InstanceImageIdEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElEc2InstanceImageIdEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElEc2InstanceImageIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElEc2InstanceImageIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElEc2InstanceImageIdEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElEc2InstanceImageIdEl {
        Inspector2FilterFilterCriteriaElEc2InstanceImageIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElEc2InstanceImageIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElEc2InstanceImageIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElEc2InstanceImageIdElRef {
        Inspector2FilterFilterCriteriaElEc2InstanceImageIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElEc2InstanceImageIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElEc2InstanceSubnetIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElEc2InstanceSubnetIdEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElEc2InstanceSubnetIdEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElEc2InstanceSubnetIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElEc2InstanceSubnetIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElEc2InstanceSubnetIdEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElEc2InstanceSubnetIdEl {
        Inspector2FilterFilterCriteriaElEc2InstanceSubnetIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElEc2InstanceSubnetIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElEc2InstanceSubnetIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElEc2InstanceSubnetIdElRef {
        Inspector2FilterFilterCriteriaElEc2InstanceSubnetIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElEc2InstanceSubnetIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElEc2InstanceVpcIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElEc2InstanceVpcIdEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElEc2InstanceVpcIdEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElEc2InstanceVpcIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElEc2InstanceVpcIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElEc2InstanceVpcIdEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElEc2InstanceVpcIdEl {
        Inspector2FilterFilterCriteriaElEc2InstanceVpcIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElEc2InstanceVpcIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElEc2InstanceVpcIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElEc2InstanceVpcIdElRef {
        Inspector2FilterFilterCriteriaElEc2InstanceVpcIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElEc2InstanceVpcIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElEcrImageArchitectureEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElEcrImageArchitectureEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElEcrImageArchitectureEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElEcrImageArchitectureEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElEcrImageArchitectureEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElEcrImageArchitectureEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElEcrImageArchitectureEl {
        Inspector2FilterFilterCriteriaElEcrImageArchitectureEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElEcrImageArchitectureElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElEcrImageArchitectureElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElEcrImageArchitectureElRef {
        Inspector2FilterFilterCriteriaElEcrImageArchitectureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElEcrImageArchitectureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElEcrImageHashEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElEcrImageHashEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElEcrImageHashEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElEcrImageHashEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElEcrImageHashEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElEcrImageHashEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElEcrImageHashEl {
        Inspector2FilterFilterCriteriaElEcrImageHashEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElEcrImageHashElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElEcrImageHashElRef {
    fn new(shared: StackShared, base: String) -> Inspector2FilterFilterCriteriaElEcrImageHashElRef {
        Inspector2FilterFilterCriteriaElEcrImageHashElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElEcrImageHashElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElEcrImageInUseCountEl {
    lower_inclusive: PrimField<f64>,
    upper_inclusive: PrimField<f64>,
}

impl Inspector2FilterFilterCriteriaElEcrImageInUseCountEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElEcrImageInUseCountEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElEcrImageInUseCountEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElEcrImageInUseCountEl {
    #[doc = ""]
    pub lower_inclusive: PrimField<f64>,
    #[doc = ""]
    pub upper_inclusive: PrimField<f64>,
}

impl BuildInspector2FilterFilterCriteriaElEcrImageInUseCountEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElEcrImageInUseCountEl {
        Inspector2FilterFilterCriteriaElEcrImageInUseCountEl {
            lower_inclusive: self.lower_inclusive,
            upper_inclusive: self.upper_inclusive,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElEcrImageInUseCountElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElEcrImageInUseCountElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElEcrImageInUseCountElRef {
        Inspector2FilterFilterCriteriaElEcrImageInUseCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElEcrImageInUseCountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lower_inclusive` after provisioning.\n"]
    pub fn lower_inclusive(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lower_inclusive", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `upper_inclusive` after provisioning.\n"]
    pub fn upper_inclusive(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.upper_inclusive", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElEcrImageLastInUseAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_inclusive: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_inclusive: Option<PrimField<String>>,
}

impl Inspector2FilterFilterCriteriaElEcrImageLastInUseAtEl {
    #[doc = "Set the field `end_inclusive`.\n"]
    pub fn set_end_inclusive(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_inclusive = Some(v.into());
        self
    }

    #[doc = "Set the field `start_inclusive`.\n"]
    pub fn set_start_inclusive(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_inclusive = Some(v.into());
        self
    }
}

impl ToListMappable for Inspector2FilterFilterCriteriaElEcrImageLastInUseAtEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElEcrImageLastInUseAtEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElEcrImageLastInUseAtEl {}

impl BuildInspector2FilterFilterCriteriaElEcrImageLastInUseAtEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElEcrImageLastInUseAtEl {
        Inspector2FilterFilterCriteriaElEcrImageLastInUseAtEl {
            end_inclusive: core::default::Default::default(),
            start_inclusive: core::default::Default::default(),
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElEcrImageLastInUseAtElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElEcrImageLastInUseAtElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElEcrImageLastInUseAtElRef {
        Inspector2FilterFilterCriteriaElEcrImageLastInUseAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElEcrImageLastInUseAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_inclusive` after provisioning.\n"]
    pub fn end_inclusive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_inclusive", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `start_inclusive` after provisioning.\n"]
    pub fn start_inclusive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_inclusive", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElEcrImagePushedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_inclusive: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_inclusive: Option<PrimField<String>>,
}

impl Inspector2FilterFilterCriteriaElEcrImagePushedAtEl {
    #[doc = "Set the field `end_inclusive`.\n"]
    pub fn set_end_inclusive(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_inclusive = Some(v.into());
        self
    }

    #[doc = "Set the field `start_inclusive`.\n"]
    pub fn set_start_inclusive(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_inclusive = Some(v.into());
        self
    }
}

impl ToListMappable for Inspector2FilterFilterCriteriaElEcrImagePushedAtEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElEcrImagePushedAtEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElEcrImagePushedAtEl {}

impl BuildInspector2FilterFilterCriteriaElEcrImagePushedAtEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElEcrImagePushedAtEl {
        Inspector2FilterFilterCriteriaElEcrImagePushedAtEl {
            end_inclusive: core::default::Default::default(),
            start_inclusive: core::default::Default::default(),
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElEcrImagePushedAtElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElEcrImagePushedAtElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElEcrImagePushedAtElRef {
        Inspector2FilterFilterCriteriaElEcrImagePushedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElEcrImagePushedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_inclusive` after provisioning.\n"]
    pub fn end_inclusive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_inclusive", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `start_inclusive` after provisioning.\n"]
    pub fn start_inclusive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_inclusive", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElEcrImageRegistryEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElEcrImageRegistryEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElEcrImageRegistryEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElEcrImageRegistryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElEcrImageRegistryEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElEcrImageRegistryEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElEcrImageRegistryEl {
        Inspector2FilterFilterCriteriaElEcrImageRegistryEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElEcrImageRegistryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElEcrImageRegistryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElEcrImageRegistryElRef {
        Inspector2FilterFilterCriteriaElEcrImageRegistryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElEcrImageRegistryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElEcrImageRepositoryNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElEcrImageRepositoryNameEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElEcrImageRepositoryNameEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElEcrImageRepositoryNameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElEcrImageRepositoryNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElEcrImageRepositoryNameEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElEcrImageRepositoryNameEl {
        Inspector2FilterFilterCriteriaElEcrImageRepositoryNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElEcrImageRepositoryNameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElEcrImageRepositoryNameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElEcrImageRepositoryNameElRef {
        Inspector2FilterFilterCriteriaElEcrImageRepositoryNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElEcrImageRepositoryNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElEcrImageTagsEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElEcrImageTagsEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElEcrImageTagsEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElEcrImageTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElEcrImageTagsEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElEcrImageTagsEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElEcrImageTagsEl {
        Inspector2FilterFilterCriteriaElEcrImageTagsEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElEcrImageTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElEcrImageTagsElRef {
    fn new(shared: StackShared, base: String) -> Inspector2FilterFilterCriteriaElEcrImageTagsElRef {
        Inspector2FilterFilterCriteriaElEcrImageTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElEcrImageTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElEpssScoreEl {
    lower_inclusive: PrimField<f64>,
    upper_inclusive: PrimField<f64>,
}

impl Inspector2FilterFilterCriteriaElEpssScoreEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElEpssScoreEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElEpssScoreEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElEpssScoreEl {
    #[doc = ""]
    pub lower_inclusive: PrimField<f64>,
    #[doc = ""]
    pub upper_inclusive: PrimField<f64>,
}

impl BuildInspector2FilterFilterCriteriaElEpssScoreEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElEpssScoreEl {
        Inspector2FilterFilterCriteriaElEpssScoreEl {
            lower_inclusive: self.lower_inclusive,
            upper_inclusive: self.upper_inclusive,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElEpssScoreElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElEpssScoreElRef {
    fn new(shared: StackShared, base: String) -> Inspector2FilterFilterCriteriaElEpssScoreElRef {
        Inspector2FilterFilterCriteriaElEpssScoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElEpssScoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lower_inclusive` after provisioning.\n"]
    pub fn lower_inclusive(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lower_inclusive", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `upper_inclusive` after provisioning.\n"]
    pub fn upper_inclusive(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.upper_inclusive", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElExploitAvailableEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElExploitAvailableEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElExploitAvailableEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElExploitAvailableEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElExploitAvailableEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElExploitAvailableEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElExploitAvailableEl {
        Inspector2FilterFilterCriteriaElExploitAvailableEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElExploitAvailableElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElExploitAvailableElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElExploitAvailableElRef {
        Inspector2FilterFilterCriteriaElExploitAvailableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElExploitAvailableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElFindingArnEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElFindingArnEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElFindingArnEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElFindingArnEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElFindingArnEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElFindingArnEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElFindingArnEl {
        Inspector2FilterFilterCriteriaElFindingArnEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElFindingArnElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElFindingArnElRef {
    fn new(shared: StackShared, base: String) -> Inspector2FilterFilterCriteriaElFindingArnElRef {
        Inspector2FilterFilterCriteriaElFindingArnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElFindingArnElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElFindingStatusEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElFindingStatusEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElFindingStatusEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElFindingStatusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElFindingStatusEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElFindingStatusEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElFindingStatusEl {
        Inspector2FilterFilterCriteriaElFindingStatusEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElFindingStatusElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElFindingStatusElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElFindingStatusElRef {
        Inspector2FilterFilterCriteriaElFindingStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElFindingStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElFindingTypeEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElFindingTypeEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElFindingTypeEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElFindingTypeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElFindingTypeEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElFindingTypeEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElFindingTypeEl {
        Inspector2FilterFilterCriteriaElFindingTypeEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElFindingTypeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElFindingTypeElRef {
    fn new(shared: StackShared, base: String) -> Inspector2FilterFilterCriteriaElFindingTypeElRef {
        Inspector2FilterFilterCriteriaElFindingTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElFindingTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElFirstObservedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_inclusive: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_inclusive: Option<PrimField<String>>,
}

impl Inspector2FilterFilterCriteriaElFirstObservedAtEl {
    #[doc = "Set the field `end_inclusive`.\n"]
    pub fn set_end_inclusive(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_inclusive = Some(v.into());
        self
    }

    #[doc = "Set the field `start_inclusive`.\n"]
    pub fn set_start_inclusive(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_inclusive = Some(v.into());
        self
    }
}

impl ToListMappable for Inspector2FilterFilterCriteriaElFirstObservedAtEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElFirstObservedAtEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElFirstObservedAtEl {}

impl BuildInspector2FilterFilterCriteriaElFirstObservedAtEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElFirstObservedAtEl {
        Inspector2FilterFilterCriteriaElFirstObservedAtEl {
            end_inclusive: core::default::Default::default(),
            start_inclusive: core::default::Default::default(),
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElFirstObservedAtElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElFirstObservedAtElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElFirstObservedAtElRef {
        Inspector2FilterFilterCriteriaElFirstObservedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElFirstObservedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_inclusive` after provisioning.\n"]
    pub fn end_inclusive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_inclusive", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `start_inclusive` after provisioning.\n"]
    pub fn start_inclusive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_inclusive", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElFixAvailableEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElFixAvailableEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElFixAvailableEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElFixAvailableEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElFixAvailableEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElFixAvailableEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElFixAvailableEl {
        Inspector2FilterFilterCriteriaElFixAvailableEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElFixAvailableElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElFixAvailableElRef {
    fn new(shared: StackShared, base: String) -> Inspector2FilterFilterCriteriaElFixAvailableElRef {
        Inspector2FilterFilterCriteriaElFixAvailableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElFixAvailableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElInspectorScoreEl {
    lower_inclusive: PrimField<f64>,
    upper_inclusive: PrimField<f64>,
}

impl Inspector2FilterFilterCriteriaElInspectorScoreEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElInspectorScoreEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElInspectorScoreEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElInspectorScoreEl {
    #[doc = ""]
    pub lower_inclusive: PrimField<f64>,
    #[doc = ""]
    pub upper_inclusive: PrimField<f64>,
}

impl BuildInspector2FilterFilterCriteriaElInspectorScoreEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElInspectorScoreEl {
        Inspector2FilterFilterCriteriaElInspectorScoreEl {
            lower_inclusive: self.lower_inclusive,
            upper_inclusive: self.upper_inclusive,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElInspectorScoreElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElInspectorScoreElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElInspectorScoreElRef {
        Inspector2FilterFilterCriteriaElInspectorScoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElInspectorScoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lower_inclusive` after provisioning.\n"]
    pub fn lower_inclusive(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lower_inclusive", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `upper_inclusive` after provisioning.\n"]
    pub fn upper_inclusive(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.upper_inclusive", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElLambdaFunctionExecutionRoleArnEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElLambdaFunctionExecutionRoleArnEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElLambdaFunctionExecutionRoleArnEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElLambdaFunctionExecutionRoleArnEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElLambdaFunctionExecutionRoleArnEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElLambdaFunctionExecutionRoleArnEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElLambdaFunctionExecutionRoleArnEl {
        Inspector2FilterFilterCriteriaElLambdaFunctionExecutionRoleArnEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElLambdaFunctionExecutionRoleArnElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElLambdaFunctionExecutionRoleArnElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElLambdaFunctionExecutionRoleArnElRef {
        Inspector2FilterFilterCriteriaElLambdaFunctionExecutionRoleArnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElLambdaFunctionExecutionRoleArnElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElLambdaFunctionLastModifiedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_inclusive: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_inclusive: Option<PrimField<String>>,
}

impl Inspector2FilterFilterCriteriaElLambdaFunctionLastModifiedAtEl {
    #[doc = "Set the field `end_inclusive`.\n"]
    pub fn set_end_inclusive(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_inclusive = Some(v.into());
        self
    }

    #[doc = "Set the field `start_inclusive`.\n"]
    pub fn set_start_inclusive(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_inclusive = Some(v.into());
        self
    }
}

impl ToListMappable for Inspector2FilterFilterCriteriaElLambdaFunctionLastModifiedAtEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElLambdaFunctionLastModifiedAtEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElLambdaFunctionLastModifiedAtEl {}

impl BuildInspector2FilterFilterCriteriaElLambdaFunctionLastModifiedAtEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElLambdaFunctionLastModifiedAtEl {
        Inspector2FilterFilterCriteriaElLambdaFunctionLastModifiedAtEl {
            end_inclusive: core::default::Default::default(),
            start_inclusive: core::default::Default::default(),
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElLambdaFunctionLastModifiedAtElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElLambdaFunctionLastModifiedAtElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElLambdaFunctionLastModifiedAtElRef {
        Inspector2FilterFilterCriteriaElLambdaFunctionLastModifiedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElLambdaFunctionLastModifiedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_inclusive` after provisioning.\n"]
    pub fn end_inclusive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_inclusive", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `start_inclusive` after provisioning.\n"]
    pub fn start_inclusive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_inclusive", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElLambdaFunctionLayersEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElLambdaFunctionLayersEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElLambdaFunctionLayersEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElLambdaFunctionLayersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElLambdaFunctionLayersEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElLambdaFunctionLayersEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElLambdaFunctionLayersEl {
        Inspector2FilterFilterCriteriaElLambdaFunctionLayersEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElLambdaFunctionLayersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElLambdaFunctionLayersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElLambdaFunctionLayersElRef {
        Inspector2FilterFilterCriteriaElLambdaFunctionLayersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElLambdaFunctionLayersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElLambdaFunctionNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElLambdaFunctionNameEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElLambdaFunctionNameEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElLambdaFunctionNameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElLambdaFunctionNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElLambdaFunctionNameEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElLambdaFunctionNameEl {
        Inspector2FilterFilterCriteriaElLambdaFunctionNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElLambdaFunctionNameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElLambdaFunctionNameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElLambdaFunctionNameElRef {
        Inspector2FilterFilterCriteriaElLambdaFunctionNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElLambdaFunctionNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElLambdaFunctionRuntimeEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElLambdaFunctionRuntimeEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElLambdaFunctionRuntimeEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElLambdaFunctionRuntimeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElLambdaFunctionRuntimeEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElLambdaFunctionRuntimeEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElLambdaFunctionRuntimeEl {
        Inspector2FilterFilterCriteriaElLambdaFunctionRuntimeEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElLambdaFunctionRuntimeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElLambdaFunctionRuntimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElLambdaFunctionRuntimeElRef {
        Inspector2FilterFilterCriteriaElLambdaFunctionRuntimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElLambdaFunctionRuntimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElLastObservedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_inclusive: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_inclusive: Option<PrimField<String>>,
}

impl Inspector2FilterFilterCriteriaElLastObservedAtEl {
    #[doc = "Set the field `end_inclusive`.\n"]
    pub fn set_end_inclusive(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_inclusive = Some(v.into());
        self
    }

    #[doc = "Set the field `start_inclusive`.\n"]
    pub fn set_start_inclusive(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_inclusive = Some(v.into());
        self
    }
}

impl ToListMappable for Inspector2FilterFilterCriteriaElLastObservedAtEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElLastObservedAtEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElLastObservedAtEl {}

impl BuildInspector2FilterFilterCriteriaElLastObservedAtEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElLastObservedAtEl {
        Inspector2FilterFilterCriteriaElLastObservedAtEl {
            end_inclusive: core::default::Default::default(),
            start_inclusive: core::default::Default::default(),
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElLastObservedAtElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElLastObservedAtElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElLastObservedAtElRef {
        Inspector2FilterFilterCriteriaElLastObservedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElLastObservedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_inclusive` after provisioning.\n"]
    pub fn end_inclusive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_inclusive", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `start_inclusive` after provisioning.\n"]
    pub fn start_inclusive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_inclusive", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElNetworkProtocolEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElNetworkProtocolEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElNetworkProtocolEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElNetworkProtocolEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElNetworkProtocolEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElNetworkProtocolEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElNetworkProtocolEl {
        Inspector2FilterFilterCriteriaElNetworkProtocolEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElNetworkProtocolElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElNetworkProtocolElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElNetworkProtocolElRef {
        Inspector2FilterFilterCriteriaElNetworkProtocolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElNetworkProtocolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElPortRangeEl {
    begin_inclusive: PrimField<f64>,
    end_inclusive: PrimField<f64>,
}

impl Inspector2FilterFilterCriteriaElPortRangeEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElPortRangeEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElPortRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElPortRangeEl {
    #[doc = ""]
    pub begin_inclusive: PrimField<f64>,
    #[doc = ""]
    pub end_inclusive: PrimField<f64>,
}

impl BuildInspector2FilterFilterCriteriaElPortRangeEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElPortRangeEl {
        Inspector2FilterFilterCriteriaElPortRangeEl {
            begin_inclusive: self.begin_inclusive,
            end_inclusive: self.end_inclusive,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElPortRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElPortRangeElRef {
    fn new(shared: StackShared, base: String) -> Inspector2FilterFilterCriteriaElPortRangeElRef {
        Inspector2FilterFilterCriteriaElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElPortRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `begin_inclusive` after provisioning.\n"]
    pub fn begin_inclusive(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.begin_inclusive", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `end_inclusive` after provisioning.\n"]
    pub fn end_inclusive(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_inclusive", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElRelatedVulnerabilitiesEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElRelatedVulnerabilitiesEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElRelatedVulnerabilitiesEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElRelatedVulnerabilitiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElRelatedVulnerabilitiesEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElRelatedVulnerabilitiesEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElRelatedVulnerabilitiesEl {
        Inspector2FilterFilterCriteriaElRelatedVulnerabilitiesEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElRelatedVulnerabilitiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElRelatedVulnerabilitiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElRelatedVulnerabilitiesElRef {
        Inspector2FilterFilterCriteriaElRelatedVulnerabilitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElRelatedVulnerabilitiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElResourceIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElResourceIdEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElResourceIdEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElResourceIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElResourceIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElResourceIdEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElResourceIdEl {
        Inspector2FilterFilterCriteriaElResourceIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElResourceIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElResourceIdElRef {
    fn new(shared: StackShared, base: String) -> Inspector2FilterFilterCriteriaElResourceIdElRef {
        Inspector2FilterFilterCriteriaElResourceIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElResourceIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElResourceTagsEl {
    comparison: PrimField<String>,
    key: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElResourceTagsEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElResourceTagsEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElResourceTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElResourceTagsEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElResourceTagsEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElResourceTagsEl {
        Inspector2FilterFilterCriteriaElResourceTagsEl {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElResourceTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElResourceTagsElRef {
    fn new(shared: StackShared, base: String) -> Inspector2FilterFilterCriteriaElResourceTagsElRef {
        Inspector2FilterFilterCriteriaElResourceTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElResourceTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElResourceTypeEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElResourceTypeEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElResourceTypeEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElResourceTypeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElResourceTypeEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElResourceTypeEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElResourceTypeEl {
        Inspector2FilterFilterCriteriaElResourceTypeEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElResourceTypeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElResourceTypeElRef {
    fn new(shared: StackShared, base: String) -> Inspector2FilterFilterCriteriaElResourceTypeElRef {
        Inspector2FilterFilterCriteriaElResourceTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElResourceTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElSeverityEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElSeverityEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElSeverityEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElSeverityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElSeverityEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElSeverityEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElSeverityEl {
        Inspector2FilterFilterCriteriaElSeverityEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElSeverityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElSeverityElRef {
    fn new(shared: StackShared, base: String) -> Inspector2FilterFilterCriteriaElSeverityElRef {
        Inspector2FilterFilterCriteriaElSeverityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElSeverityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElTitleEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElTitleEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElTitleEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElTitleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElTitleEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElTitleEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElTitleEl {
        Inspector2FilterFilterCriteriaElTitleEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElTitleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElTitleElRef {
    fn new(shared: StackShared, base: String) -> Inspector2FilterFilterCriteriaElTitleElRef {
        Inspector2FilterFilterCriteriaElTitleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElTitleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElUpdatedAtEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end_inclusive: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_inclusive: Option<PrimField<String>>,
}

impl Inspector2FilterFilterCriteriaElUpdatedAtEl {
    #[doc = "Set the field `end_inclusive`.\n"]
    pub fn set_end_inclusive(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_inclusive = Some(v.into());
        self
    }

    #[doc = "Set the field `start_inclusive`.\n"]
    pub fn set_start_inclusive(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_inclusive = Some(v.into());
        self
    }
}

impl ToListMappable for Inspector2FilterFilterCriteriaElUpdatedAtEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElUpdatedAtEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElUpdatedAtEl {}

impl BuildInspector2FilterFilterCriteriaElUpdatedAtEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElUpdatedAtEl {
        Inspector2FilterFilterCriteriaElUpdatedAtEl {
            end_inclusive: core::default::Default::default(),
            start_inclusive: core::default::Default::default(),
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElUpdatedAtElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElUpdatedAtElRef {
    fn new(shared: StackShared, base: String) -> Inspector2FilterFilterCriteriaElUpdatedAtElRef {
        Inspector2FilterFilterCriteriaElUpdatedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElUpdatedAtElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_inclusive` after provisioning.\n"]
    pub fn end_inclusive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.end_inclusive", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `start_inclusive` after provisioning.\n"]
    pub fn start_inclusive(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_inclusive", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElVendorSeverityEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElVendorSeverityEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElVendorSeverityEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElVendorSeverityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElVendorSeverityEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElVendorSeverityEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElVendorSeverityEl {
        Inspector2FilterFilterCriteriaElVendorSeverityEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElVendorSeverityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElVendorSeverityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElVendorSeverityElRef {
        Inspector2FilterFilterCriteriaElVendorSeverityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElVendorSeverityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElVulnerabilityIdEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElVulnerabilityIdEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElVulnerabilityIdEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElVulnerabilityIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElVulnerabilityIdEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElVulnerabilityIdEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElVulnerabilityIdEl {
        Inspector2FilterFilterCriteriaElVulnerabilityIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElVulnerabilityIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElVulnerabilityIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElVulnerabilityIdElRef {
        Inspector2FilterFilterCriteriaElVulnerabilityIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElVulnerabilityIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElVulnerabilitySourceEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElVulnerabilitySourceEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElVulnerabilitySourceEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElVulnerabilitySourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElVulnerabilitySourceEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElVulnerabilitySourceEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElVulnerabilitySourceEl {
        Inspector2FilterFilterCriteriaElVulnerabilitySourceEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElVulnerabilitySourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElVulnerabilitySourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElVulnerabilitySourceElRef {
        Inspector2FilterFilterCriteriaElVulnerabilitySourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElVulnerabilitySourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesElArchitectureEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesElArchitectureEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElVulnerablePackagesElArchitectureEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElVulnerablePackagesElArchitectureEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElVulnerablePackagesElArchitectureEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElVulnerablePackagesElArchitectureEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElVulnerablePackagesElArchitectureEl {
        Inspector2FilterFilterCriteriaElVulnerablePackagesElArchitectureEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesElArchitectureElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElVulnerablePackagesElArchitectureElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElVulnerablePackagesElArchitectureElRef {
        Inspector2FilterFilterCriteriaElVulnerablePackagesElArchitectureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesElArchitectureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesElEpochEl {
    lower_inclusive: PrimField<f64>,
    upper_inclusive: PrimField<f64>,
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesElEpochEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElVulnerablePackagesElEpochEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElVulnerablePackagesElEpochEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElVulnerablePackagesElEpochEl {
    #[doc = ""]
    pub lower_inclusive: PrimField<f64>,
    #[doc = ""]
    pub upper_inclusive: PrimField<f64>,
}

impl BuildInspector2FilterFilterCriteriaElVulnerablePackagesElEpochEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElVulnerablePackagesElEpochEl {
        Inspector2FilterFilterCriteriaElVulnerablePackagesElEpochEl {
            lower_inclusive: self.lower_inclusive,
            upper_inclusive: self.upper_inclusive,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesElEpochElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElVulnerablePackagesElEpochElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElVulnerablePackagesElEpochElRef {
        Inspector2FilterFilterCriteriaElVulnerablePackagesElEpochElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesElEpochElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lower_inclusive` after provisioning.\n"]
    pub fn lower_inclusive(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lower_inclusive", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `upper_inclusive` after provisioning.\n"]
    pub fn upper_inclusive(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.upper_inclusive", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesElFilePathEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesElFilePathEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElVulnerablePackagesElFilePathEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElVulnerablePackagesElFilePathEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElVulnerablePackagesElFilePathEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElVulnerablePackagesElFilePathEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElVulnerablePackagesElFilePathEl {
        Inspector2FilterFilterCriteriaElVulnerablePackagesElFilePathEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesElFilePathElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElVulnerablePackagesElFilePathElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElVulnerablePackagesElFilePathElRef {
        Inspector2FilterFilterCriteriaElVulnerablePackagesElFilePathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesElFilePathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesElNameEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesElNameEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElVulnerablePackagesElNameEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElVulnerablePackagesElNameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElVulnerablePackagesElNameEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElVulnerablePackagesElNameEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElVulnerablePackagesElNameEl {
        Inspector2FilterFilterCriteriaElVulnerablePackagesElNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesElNameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElVulnerablePackagesElNameElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElVulnerablePackagesElNameElRef {
        Inspector2FilterFilterCriteriaElVulnerablePackagesElNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesElNameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesElReleaseEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesElReleaseEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElVulnerablePackagesElReleaseEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElVulnerablePackagesElReleaseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElVulnerablePackagesElReleaseEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElVulnerablePackagesElReleaseEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElVulnerablePackagesElReleaseEl {
        Inspector2FilterFilterCriteriaElVulnerablePackagesElReleaseEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesElReleaseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElVulnerablePackagesElReleaseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElVulnerablePackagesElReleaseElRef {
        Inspector2FilterFilterCriteriaElVulnerablePackagesElReleaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesElReleaseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLambdaLayerArnEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLambdaLayerArnEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLambdaLayerArnEl {
    type O =
        BlockAssignable<Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLambdaLayerArnEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElVulnerablePackagesElSourceLambdaLayerArnEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElVulnerablePackagesElSourceLambdaLayerArnEl {
    pub fn build(
        self,
    ) -> Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLambdaLayerArnEl {
        Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLambdaLayerArnEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLambdaLayerArnElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLambdaLayerArnElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLambdaLayerArnElRef {
        Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLambdaLayerArnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLambdaLayerArnElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLayerHashEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLayerHashEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLayerHashEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLayerHashEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElVulnerablePackagesElSourceLayerHashEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElVulnerablePackagesElSourceLayerHashEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLayerHashEl {
        Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLayerHashEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLayerHashElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLayerHashElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLayerHashElRef {
        Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLayerHashElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLayerHashElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesElVersionEl {
    comparison: PrimField<String>,
    value: PrimField<String>,
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesElVersionEl {}

impl ToListMappable for Inspector2FilterFilterCriteriaElVulnerablePackagesElVersionEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElVulnerablePackagesElVersionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElVulnerablePackagesElVersionEl {
    #[doc = ""]
    pub comparison: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildInspector2FilterFilterCriteriaElVulnerablePackagesElVersionEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElVulnerablePackagesElVersionEl {
        Inspector2FilterFilterCriteriaElVulnerablePackagesElVersionEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesElVersionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElVulnerablePackagesElVersionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElVulnerablePackagesElVersionElRef {
        Inspector2FilterFilterCriteriaElVulnerablePackagesElVersionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesElVersionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Inspector2FilterFilterCriteriaElVulnerablePackagesElDynamic {
    architecture:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElVulnerablePackagesElArchitectureEl>>,
    epoch: Option<DynamicBlock<Inspector2FilterFilterCriteriaElVulnerablePackagesElEpochEl>>,
    file_path: Option<DynamicBlock<Inspector2FilterFilterCriteriaElVulnerablePackagesElFilePathEl>>,
    name: Option<DynamicBlock<Inspector2FilterFilterCriteriaElVulnerablePackagesElNameEl>>,
    release: Option<DynamicBlock<Inspector2FilterFilterCriteriaElVulnerablePackagesElReleaseEl>>,
    source_lambda_layer_arn: Option<
        DynamicBlock<Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLambdaLayerArnEl>,
    >,
    source_layer_hash:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLayerHashEl>>,
    version: Option<DynamicBlock<Inspector2FilterFilterCriteriaElVulnerablePackagesElVersionEl>>,
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    architecture: Option<Vec<Inspector2FilterFilterCriteriaElVulnerablePackagesElArchitectureEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    epoch: Option<Vec<Inspector2FilterFilterCriteriaElVulnerablePackagesElEpochEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_path: Option<Vec<Inspector2FilterFilterCriteriaElVulnerablePackagesElFilePathEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Vec<Inspector2FilterFilterCriteriaElVulnerablePackagesElNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    release: Option<Vec<Inspector2FilterFilterCriteriaElVulnerablePackagesElReleaseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_lambda_layer_arn:
        Option<Vec<Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLambdaLayerArnEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_layer_hash:
        Option<Vec<Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLayerHashEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<Vec<Inspector2FilterFilterCriteriaElVulnerablePackagesElVersionEl>>,
    dynamic: Inspector2FilterFilterCriteriaElVulnerablePackagesElDynamic,
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesEl {
    #[doc = "Set the field `architecture`.\n"]
    pub fn set_architecture(
        mut self,
        v: impl Into<
            BlockAssignable<Inspector2FilterFilterCriteriaElVulnerablePackagesElArchitectureEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.architecture = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.architecture = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `epoch`.\n"]
    pub fn set_epoch(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElVulnerablePackagesElEpochEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.epoch = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.epoch = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `file_path`.\n"]
    pub fn set_file_path(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElVulnerablePackagesElFilePathEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.file_path = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.file_path = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElVulnerablePackagesElNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.name = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `release`.\n"]
    pub fn set_release(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElVulnerablePackagesElReleaseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.release = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.release = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `source_lambda_layer_arn`.\n"]
    pub fn set_source_lambda_layer_arn(
        mut self,
        v: impl Into<
            BlockAssignable<
                Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLambdaLayerArnEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_lambda_layer_arn = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_lambda_layer_arn = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `source_layer_hash`.\n"]
    pub fn set_source_layer_hash(
        mut self,
        v: impl Into<
            BlockAssignable<Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLayerHashEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_layer_hash = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_layer_hash = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElVulnerablePackagesElVersionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.version = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.version = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for Inspector2FilterFilterCriteriaElVulnerablePackagesEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaElVulnerablePackagesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaElVulnerablePackagesEl {}

impl BuildInspector2FilterFilterCriteriaElVulnerablePackagesEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaElVulnerablePackagesEl {
        Inspector2FilterFilterCriteriaElVulnerablePackagesEl {
            architecture: core::default::Default::default(),
            epoch: core::default::Default::default(),
            file_path: core::default::Default::default(),
            name: core::default::Default::default(),
            release: core::default::Default::default(),
            source_lambda_layer_arn: core::default::Default::default(),
            source_layer_hash: core::default::Default::default(),
            version: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElVulnerablePackagesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElVulnerablePackagesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Inspector2FilterFilterCriteriaElVulnerablePackagesElRef {
        Inspector2FilterFilterCriteriaElVulnerablePackagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElVulnerablePackagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(
        &self,
    ) -> ListRef<Inspector2FilterFilterCriteriaElVulnerablePackagesElArchitectureElRef> {
        ListRef::new(self.shared().clone(), format!("{}.architecture", self.base))
    }

    #[doc = "Get a reference to the value of field `epoch` after provisioning.\n"]
    pub fn epoch(&self) -> ListRef<Inspector2FilterFilterCriteriaElVulnerablePackagesElEpochElRef> {
        ListRef::new(self.shared().clone(), format!("{}.epoch", self.base))
    }

    #[doc = "Get a reference to the value of field `file_path` after provisioning.\n"]
    pub fn file_path(
        &self,
    ) -> ListRef<Inspector2FilterFilterCriteriaElVulnerablePackagesElFilePathElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file_path", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> ListRef<Inspector2FilterFilterCriteriaElVulnerablePackagesElNameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `release` after provisioning.\n"]
    pub fn release(
        &self,
    ) -> ListRef<Inspector2FilterFilterCriteriaElVulnerablePackagesElReleaseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.release", self.base))
    }

    #[doc = "Get a reference to the value of field `source_lambda_layer_arn` after provisioning.\n"]
    pub fn source_lambda_layer_arn(
        &self,
    ) -> ListRef<Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLambdaLayerArnElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_lambda_layer_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_layer_hash` after provisioning.\n"]
    pub fn source_layer_hash(
        &self,
    ) -> ListRef<Inspector2FilterFilterCriteriaElVulnerablePackagesElSourceLayerHashElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_layer_hash", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(
        &self,
    ) -> ListRef<Inspector2FilterFilterCriteriaElVulnerablePackagesElVersionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct Inspector2FilterFilterCriteriaElDynamic {
    aws_account_id: Option<DynamicBlock<Inspector2FilterFilterCriteriaElAwsAccountIdEl>>,
    code_repository_project_name:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElCodeRepositoryProjectNameEl>>,
    code_repository_provider_type:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElCodeRepositoryProviderTypeEl>>,
    code_vulnerability_detector_name:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorNameEl>>,
    code_vulnerability_detector_tags:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorTagsEl>>,
    code_vulnerability_file_path:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElCodeVulnerabilityFilePathEl>>,
    component_id: Option<DynamicBlock<Inspector2FilterFilterCriteriaElComponentIdEl>>,
    component_type: Option<DynamicBlock<Inspector2FilterFilterCriteriaElComponentTypeEl>>,
    ec2_instance_image_id:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElEc2InstanceImageIdEl>>,
    ec2_instance_subnet_id:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElEc2InstanceSubnetIdEl>>,
    ec2_instance_vpc_id: Option<DynamicBlock<Inspector2FilterFilterCriteriaElEc2InstanceVpcIdEl>>,
    ecr_image_architecture:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElEcrImageArchitectureEl>>,
    ecr_image_hash: Option<DynamicBlock<Inspector2FilterFilterCriteriaElEcrImageHashEl>>,
    ecr_image_in_use_count:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElEcrImageInUseCountEl>>,
    ecr_image_last_in_use_at:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElEcrImageLastInUseAtEl>>,
    ecr_image_pushed_at: Option<DynamicBlock<Inspector2FilterFilterCriteriaElEcrImagePushedAtEl>>,
    ecr_image_registry: Option<DynamicBlock<Inspector2FilterFilterCriteriaElEcrImageRegistryEl>>,
    ecr_image_repository_name:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElEcrImageRepositoryNameEl>>,
    ecr_image_tags: Option<DynamicBlock<Inspector2FilterFilterCriteriaElEcrImageTagsEl>>,
    epss_score: Option<DynamicBlock<Inspector2FilterFilterCriteriaElEpssScoreEl>>,
    exploit_available: Option<DynamicBlock<Inspector2FilterFilterCriteriaElExploitAvailableEl>>,
    finding_arn: Option<DynamicBlock<Inspector2FilterFilterCriteriaElFindingArnEl>>,
    finding_status: Option<DynamicBlock<Inspector2FilterFilterCriteriaElFindingStatusEl>>,
    finding_type: Option<DynamicBlock<Inspector2FilterFilterCriteriaElFindingTypeEl>>,
    first_observed_at: Option<DynamicBlock<Inspector2FilterFilterCriteriaElFirstObservedAtEl>>,
    fix_available: Option<DynamicBlock<Inspector2FilterFilterCriteriaElFixAvailableEl>>,
    inspector_score: Option<DynamicBlock<Inspector2FilterFilterCriteriaElInspectorScoreEl>>,
    lambda_function_execution_role_arn:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElLambdaFunctionExecutionRoleArnEl>>,
    lambda_function_last_modified_at:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElLambdaFunctionLastModifiedAtEl>>,
    lambda_function_layers:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElLambdaFunctionLayersEl>>,
    lambda_function_name:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElLambdaFunctionNameEl>>,
    lambda_function_runtime:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElLambdaFunctionRuntimeEl>>,
    last_observed_at: Option<DynamicBlock<Inspector2FilterFilterCriteriaElLastObservedAtEl>>,
    network_protocol: Option<DynamicBlock<Inspector2FilterFilterCriteriaElNetworkProtocolEl>>,
    port_range: Option<DynamicBlock<Inspector2FilterFilterCriteriaElPortRangeEl>>,
    related_vulnerabilities:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElRelatedVulnerabilitiesEl>>,
    resource_id: Option<DynamicBlock<Inspector2FilterFilterCriteriaElResourceIdEl>>,
    resource_tags: Option<DynamicBlock<Inspector2FilterFilterCriteriaElResourceTagsEl>>,
    resource_type: Option<DynamicBlock<Inspector2FilterFilterCriteriaElResourceTypeEl>>,
    severity: Option<DynamicBlock<Inspector2FilterFilterCriteriaElSeverityEl>>,
    title: Option<DynamicBlock<Inspector2FilterFilterCriteriaElTitleEl>>,
    updated_at: Option<DynamicBlock<Inspector2FilterFilterCriteriaElUpdatedAtEl>>,
    vendor_severity: Option<DynamicBlock<Inspector2FilterFilterCriteriaElVendorSeverityEl>>,
    vulnerability_id: Option<DynamicBlock<Inspector2FilterFilterCriteriaElVulnerabilityIdEl>>,
    vulnerability_source:
        Option<DynamicBlock<Inspector2FilterFilterCriteriaElVulnerabilitySourceEl>>,
    vulnerable_packages: Option<DynamicBlock<Inspector2FilterFilterCriteriaElVulnerablePackagesEl>>,
}

#[derive(Serialize)]
pub struct Inspector2FilterFilterCriteriaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_account_id: Option<Vec<Inspector2FilterFilterCriteriaElAwsAccountIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_repository_project_name:
        Option<Vec<Inspector2FilterFilterCriteriaElCodeRepositoryProjectNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_repository_provider_type:
        Option<Vec<Inspector2FilterFilterCriteriaElCodeRepositoryProviderTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_vulnerability_detector_name:
        Option<Vec<Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_vulnerability_detector_tags:
        Option<Vec<Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorTagsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_vulnerability_file_path:
        Option<Vec<Inspector2FilterFilterCriteriaElCodeVulnerabilityFilePathEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    component_id: Option<Vec<Inspector2FilterFilterCriteriaElComponentIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    component_type: Option<Vec<Inspector2FilterFilterCriteriaElComponentTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ec2_instance_image_id: Option<Vec<Inspector2FilterFilterCriteriaElEc2InstanceImageIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ec2_instance_subnet_id: Option<Vec<Inspector2FilterFilterCriteriaElEc2InstanceSubnetIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ec2_instance_vpc_id: Option<Vec<Inspector2FilterFilterCriteriaElEc2InstanceVpcIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecr_image_architecture: Option<Vec<Inspector2FilterFilterCriteriaElEcrImageArchitectureEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecr_image_hash: Option<Vec<Inspector2FilterFilterCriteriaElEcrImageHashEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecr_image_in_use_count: Option<Vec<Inspector2FilterFilterCriteriaElEcrImageInUseCountEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecr_image_last_in_use_at: Option<Vec<Inspector2FilterFilterCriteriaElEcrImageLastInUseAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecr_image_pushed_at: Option<Vec<Inspector2FilterFilterCriteriaElEcrImagePushedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecr_image_registry: Option<Vec<Inspector2FilterFilterCriteriaElEcrImageRegistryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecr_image_repository_name:
        Option<Vec<Inspector2FilterFilterCriteriaElEcrImageRepositoryNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecr_image_tags: Option<Vec<Inspector2FilterFilterCriteriaElEcrImageTagsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    epss_score: Option<Vec<Inspector2FilterFilterCriteriaElEpssScoreEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exploit_available: Option<Vec<Inspector2FilterFilterCriteriaElExploitAvailableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finding_arn: Option<Vec<Inspector2FilterFilterCriteriaElFindingArnEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finding_status: Option<Vec<Inspector2FilterFilterCriteriaElFindingStatusEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finding_type: Option<Vec<Inspector2FilterFilterCriteriaElFindingTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_observed_at: Option<Vec<Inspector2FilterFilterCriteriaElFirstObservedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fix_available: Option<Vec<Inspector2FilterFilterCriteriaElFixAvailableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inspector_score: Option<Vec<Inspector2FilterFilterCriteriaElInspectorScoreEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_function_execution_role_arn:
        Option<Vec<Inspector2FilterFilterCriteriaElLambdaFunctionExecutionRoleArnEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_function_last_modified_at:
        Option<Vec<Inspector2FilterFilterCriteriaElLambdaFunctionLastModifiedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_function_layers: Option<Vec<Inspector2FilterFilterCriteriaElLambdaFunctionLayersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_function_name: Option<Vec<Inspector2FilterFilterCriteriaElLambdaFunctionNameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_function_runtime: Option<Vec<Inspector2FilterFilterCriteriaElLambdaFunctionRuntimeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_observed_at: Option<Vec<Inspector2FilterFilterCriteriaElLastObservedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_protocol: Option<Vec<Inspector2FilterFilterCriteriaElNetworkProtocolEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<Vec<Inspector2FilterFilterCriteriaElPortRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_vulnerabilities: Option<Vec<Inspector2FilterFilterCriteriaElRelatedVulnerabilitiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<Vec<Inspector2FilterFilterCriteriaElResourceIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_tags: Option<Vec<Inspector2FilterFilterCriteriaElResourceTagsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<Vec<Inspector2FilterFilterCriteriaElResourceTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<Vec<Inspector2FilterFilterCriteriaElSeverityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Vec<Inspector2FilterFilterCriteriaElTitleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<Vec<Inspector2FilterFilterCriteriaElUpdatedAtEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vendor_severity: Option<Vec<Inspector2FilterFilterCriteriaElVendorSeverityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vulnerability_id: Option<Vec<Inspector2FilterFilterCriteriaElVulnerabilityIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vulnerability_source: Option<Vec<Inspector2FilterFilterCriteriaElVulnerabilitySourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vulnerable_packages: Option<Vec<Inspector2FilterFilterCriteriaElVulnerablePackagesEl>>,
    dynamic: Inspector2FilterFilterCriteriaElDynamic,
}

impl Inspector2FilterFilterCriteriaEl {
    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElAwsAccountIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.aws_account_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_account_id = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `code_repository_project_name`.\n"]
    pub fn set_code_repository_project_name(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElCodeRepositoryProjectNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_repository_project_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository_project_name = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `code_repository_provider_type`.\n"]
    pub fn set_code_repository_provider_type(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElCodeRepositoryProviderTypeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_repository_provider_type = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository_provider_type = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `code_vulnerability_detector_name`.\n"]
    pub fn set_code_vulnerability_detector_name(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_vulnerability_detector_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_vulnerability_detector_name = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `code_vulnerability_detector_tags`.\n"]
    pub fn set_code_vulnerability_detector_tags(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElCodeVulnerabilityDetectorTagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_vulnerability_detector_tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_vulnerability_detector_tags = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `code_vulnerability_file_path`.\n"]
    pub fn set_code_vulnerability_file_path(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElCodeVulnerabilityFilePathEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_vulnerability_file_path = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_vulnerability_file_path = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `component_id`.\n"]
    pub fn set_component_id(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElComponentIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.component_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.component_id = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `component_type`.\n"]
    pub fn set_component_type(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElComponentTypeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.component_type = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.component_type = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ec2_instance_image_id`.\n"]
    pub fn set_ec2_instance_image_id(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElEc2InstanceImageIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ec2_instance_image_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ec2_instance_image_id = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ec2_instance_subnet_id`.\n"]
    pub fn set_ec2_instance_subnet_id(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElEc2InstanceSubnetIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ec2_instance_subnet_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ec2_instance_subnet_id = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ec2_instance_vpc_id`.\n"]
    pub fn set_ec2_instance_vpc_id(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElEc2InstanceVpcIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ec2_instance_vpc_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ec2_instance_vpc_id = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ecr_image_architecture`.\n"]
    pub fn set_ecr_image_architecture(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElEcrImageArchitectureEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ecr_image_architecture = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_architecture = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ecr_image_hash`.\n"]
    pub fn set_ecr_image_hash(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElEcrImageHashEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ecr_image_hash = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_hash = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ecr_image_in_use_count`.\n"]
    pub fn set_ecr_image_in_use_count(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElEcrImageInUseCountEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ecr_image_in_use_count = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_in_use_count = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ecr_image_last_in_use_at`.\n"]
    pub fn set_ecr_image_last_in_use_at(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElEcrImageLastInUseAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ecr_image_last_in_use_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_last_in_use_at = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ecr_image_pushed_at`.\n"]
    pub fn set_ecr_image_pushed_at(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElEcrImagePushedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ecr_image_pushed_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_pushed_at = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ecr_image_registry`.\n"]
    pub fn set_ecr_image_registry(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElEcrImageRegistryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ecr_image_registry = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_registry = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ecr_image_repository_name`.\n"]
    pub fn set_ecr_image_repository_name(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElEcrImageRepositoryNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ecr_image_repository_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_repository_name = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ecr_image_tags`.\n"]
    pub fn set_ecr_image_tags(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElEcrImageTagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ecr_image_tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_tags = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `epss_score`.\n"]
    pub fn set_epss_score(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElEpssScoreEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.epss_score = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.epss_score = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `exploit_available`.\n"]
    pub fn set_exploit_available(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElExploitAvailableEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exploit_available = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exploit_available = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `finding_arn`.\n"]
    pub fn set_finding_arn(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElFindingArnEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.finding_arn = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_arn = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `finding_status`.\n"]
    pub fn set_finding_status(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElFindingStatusEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.finding_status = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_status = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `finding_type`.\n"]
    pub fn set_finding_type(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElFindingTypeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.finding_type = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_type = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `first_observed_at`.\n"]
    pub fn set_first_observed_at(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElFirstObservedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.first_observed_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.first_observed_at = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `fix_available`.\n"]
    pub fn set_fix_available(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElFixAvailableEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fix_available = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fix_available = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `inspector_score`.\n"]
    pub fn set_inspector_score(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElInspectorScoreEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inspector_score = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inspector_score = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `lambda_function_execution_role_arn`.\n"]
    pub fn set_lambda_function_execution_role_arn(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElLambdaFunctionExecutionRoleArnEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda_function_execution_role_arn = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_execution_role_arn = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `lambda_function_last_modified_at`.\n"]
    pub fn set_lambda_function_last_modified_at(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElLambdaFunctionLastModifiedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda_function_last_modified_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_last_modified_at = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `lambda_function_layers`.\n"]
    pub fn set_lambda_function_layers(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElLambdaFunctionLayersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda_function_layers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_layers = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `lambda_function_name`.\n"]
    pub fn set_lambda_function_name(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElLambdaFunctionNameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda_function_name = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_name = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `lambda_function_runtime`.\n"]
    pub fn set_lambda_function_runtime(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElLambdaFunctionRuntimeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lambda_function_runtime = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_runtime = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `last_observed_at`.\n"]
    pub fn set_last_observed_at(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElLastObservedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.last_observed_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.last_observed_at = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `network_protocol`.\n"]
    pub fn set_network_protocol(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElNetworkProtocolEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network_protocol = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network_protocol = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElPortRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.port_range = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.port_range = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `related_vulnerabilities`.\n"]
    pub fn set_related_vulnerabilities(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElRelatedVulnerabilitiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.related_vulnerabilities = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.related_vulnerabilities = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `resource_id`.\n"]
    pub fn set_resource_id(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElResourceIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_id = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElResourceTagsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_tags = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElResourceTypeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_type = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_type = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElSeverityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.severity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.severity = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `title`.\n"]
    pub fn set_title(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElTitleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.title = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.title = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElUpdatedAtEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.updated_at = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.updated_at = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `vendor_severity`.\n"]
    pub fn set_vendor_severity(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElVendorSeverityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vendor_severity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vendor_severity = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `vulnerability_id`.\n"]
    pub fn set_vulnerability_id(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElVulnerabilityIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vulnerability_id = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vulnerability_id = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `vulnerability_source`.\n"]
    pub fn set_vulnerability_source(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElVulnerabilitySourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vulnerability_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vulnerability_source = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `vulnerable_packages`.\n"]
    pub fn set_vulnerable_packages(
        mut self,
        v: impl Into<BlockAssignable<Inspector2FilterFilterCriteriaElVulnerablePackagesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vulnerable_packages = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vulnerable_packages = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for Inspector2FilterFilterCriteriaEl {
    type O = BlockAssignable<Inspector2FilterFilterCriteriaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2FilterFilterCriteriaEl {}

impl BuildInspector2FilterFilterCriteriaEl {
    pub fn build(self) -> Inspector2FilterFilterCriteriaEl {
        Inspector2FilterFilterCriteriaEl {
            aws_account_id: core::default::Default::default(),
            code_repository_project_name: core::default::Default::default(),
            code_repository_provider_type: core::default::Default::default(),
            code_vulnerability_detector_name: core::default::Default::default(),
            code_vulnerability_detector_tags: core::default::Default::default(),
            code_vulnerability_file_path: core::default::Default::default(),
            component_id: core::default::Default::default(),
            component_type: core::default::Default::default(),
            ec2_instance_image_id: core::default::Default::default(),
            ec2_instance_subnet_id: core::default::Default::default(),
            ec2_instance_vpc_id: core::default::Default::default(),
            ecr_image_architecture: core::default::Default::default(),
            ecr_image_hash: core::default::Default::default(),
            ecr_image_in_use_count: core::default::Default::default(),
            ecr_image_last_in_use_at: core::default::Default::default(),
            ecr_image_pushed_at: core::default::Default::default(),
            ecr_image_registry: core::default::Default::default(),
            ecr_image_repository_name: core::default::Default::default(),
            ecr_image_tags: core::default::Default::default(),
            epss_score: core::default::Default::default(),
            exploit_available: core::default::Default::default(),
            finding_arn: core::default::Default::default(),
            finding_status: core::default::Default::default(),
            finding_type: core::default::Default::default(),
            first_observed_at: core::default::Default::default(),
            fix_available: core::default::Default::default(),
            inspector_score: core::default::Default::default(),
            lambda_function_execution_role_arn: core::default::Default::default(),
            lambda_function_last_modified_at: core::default::Default::default(),
            lambda_function_layers: core::default::Default::default(),
            lambda_function_name: core::default::Default::default(),
            lambda_function_runtime: core::default::Default::default(),
            last_observed_at: core::default::Default::default(),
            network_protocol: core::default::Default::default(),
            port_range: core::default::Default::default(),
            related_vulnerabilities: core::default::Default::default(),
            resource_id: core::default::Default::default(),
            resource_tags: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            severity: core::default::Default::default(),
            title: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            vendor_severity: core::default::Default::default(),
            vulnerability_id: core::default::Default::default(),
            vulnerability_source: core::default::Default::default(),
            vulnerable_packages: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct Inspector2FilterFilterCriteriaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2FilterFilterCriteriaElRef {
    fn new(shared: StackShared, base: String) -> Inspector2FilterFilterCriteriaElRef {
        Inspector2FilterFilterCriteriaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2FilterFilterCriteriaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct Inspector2FilterDynamic {
    filter_criteria: Option<DynamicBlock<Inspector2FilterFilterCriteriaEl>>,
}
