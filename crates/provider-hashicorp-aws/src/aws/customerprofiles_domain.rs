use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CustomerprofilesDomainData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_letter_queue_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_encryption_key: Option<PrimField<String>>,
    default_expiration_days: PrimField<f64>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matching: Option<Vec<CustomerprofilesDomainMatchingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_based_matching: Option<Vec<CustomerprofilesDomainRuleBasedMatchingEl>>,
    dynamic: CustomerprofilesDomainDynamic,
}

struct CustomerprofilesDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CustomerprofilesDomainData>,
}

#[derive(Clone)]
pub struct CustomerprofilesDomain(Rc<CustomerprofilesDomain_>);

impl CustomerprofilesDomain {
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

    #[doc = "Set the field `dead_letter_queue_url`.\n"]
    pub fn set_dead_letter_queue_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dead_letter_queue_url = Some(v.into());
        self
    }

    #[doc = "Set the field `default_encryption_key`.\n"]
    pub fn set_default_encryption_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_encryption_key = Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `matching`.\n"]
    pub fn set_matching(
        self,
        v: impl Into<BlockAssignable<CustomerprofilesDomainMatchingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().matching = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.matching = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `rule_based_matching`.\n"]
    pub fn set_rule_based_matching(
        self,
        v: impl Into<BlockAssignable<CustomerprofilesDomainRuleBasedMatchingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule_based_matching = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule_based_matching = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_queue_url` after provisioning.\n"]
    pub fn dead_letter_queue_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dead_letter_queue_url", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_encryption_key` after provisioning.\n"]
    pub fn default_encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_encryption_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_expiration_days` after provisioning.\n"]
    pub fn default_expiration_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_expiration_days", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `matching` after provisioning.\n"]
    pub fn matching(&self) -> ListRef<CustomerprofilesDomainMatchingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.matching", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule_based_matching` after provisioning.\n"]
    pub fn rule_based_matching(&self) -> ListRef<CustomerprofilesDomainRuleBasedMatchingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule_based_matching", self.extract_ref()),
        )
    }
}

impl Referable for CustomerprofilesDomain {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CustomerprofilesDomain {}

impl ToListMappable for CustomerprofilesDomain {
    type O = ListRef<CustomerprofilesDomainRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CustomerprofilesDomain_ {
    fn extract_resource_type(&self) -> String {
        "aws_customerprofiles_domain".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCustomerprofilesDomain {
    pub tf_id: String,
    #[doc = ""]
    pub default_expiration_days: PrimField<f64>,
    #[doc = ""]
    pub domain_name: PrimField<String>,
}

impl BuildCustomerprofilesDomain {
    pub fn build(self, stack: &mut Stack) -> CustomerprofilesDomain {
        let out = CustomerprofilesDomain(Rc::new(CustomerprofilesDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CustomerprofilesDomainData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                dead_letter_queue_url: core::default::Default::default(),
                default_encryption_key: core::default::Default::default(),
                default_expiration_days: self.default_expiration_days,
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                matching: core::default::Default::default(),
                rule_based_matching: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CustomerprofilesDomainRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CustomerprofilesDomainRef {
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

    #[doc = "Get a reference to the value of field `dead_letter_queue_url` after provisioning.\n"]
    pub fn dead_letter_queue_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dead_letter_queue_url", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_encryption_key` after provisioning.\n"]
    pub fn default_encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_encryption_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_expiration_days` after provisioning.\n"]
    pub fn default_expiration_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_expiration_days", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `matching` after provisioning.\n"]
    pub fn matching(&self) -> ListRef<CustomerprofilesDomainMatchingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.matching", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `rule_based_matching` after provisioning.\n"]
    pub fn rule_based_matching(&self) -> ListRef<CustomerprofilesDomainRuleBasedMatchingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule_based_matching", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesDomainMatchingElAutoMergingElConflictResolutionEl {
    conflict_resolving_model: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_name: Option<PrimField<String>>,
}

impl CustomerprofilesDomainMatchingElAutoMergingElConflictResolutionEl {
    #[doc = "Set the field `source_name`.\n"]
    pub fn set_source_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_name = Some(v.into());
        self
    }
}

impl ToListMappable for CustomerprofilesDomainMatchingElAutoMergingElConflictResolutionEl {
    type O = BlockAssignable<CustomerprofilesDomainMatchingElAutoMergingElConflictResolutionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomainMatchingElAutoMergingElConflictResolutionEl {
    #[doc = ""]
    pub conflict_resolving_model: PrimField<String>,
}

impl BuildCustomerprofilesDomainMatchingElAutoMergingElConflictResolutionEl {
    pub fn build(self) -> CustomerprofilesDomainMatchingElAutoMergingElConflictResolutionEl {
        CustomerprofilesDomainMatchingElAutoMergingElConflictResolutionEl {
            conflict_resolving_model: self.conflict_resolving_model,
            source_name: core::default::Default::default(),
        }
    }
}

pub struct CustomerprofilesDomainMatchingElAutoMergingElConflictResolutionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesDomainMatchingElAutoMergingElConflictResolutionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CustomerprofilesDomainMatchingElAutoMergingElConflictResolutionElRef {
        CustomerprofilesDomainMatchingElAutoMergingElConflictResolutionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomainMatchingElAutoMergingElConflictResolutionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conflict_resolving_model` after provisioning.\n"]
    pub fn conflict_resolving_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.conflict_resolving_model", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_name` after provisioning.\n"]
    pub fn source_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_name", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesDomainMatchingElAutoMergingElConsolidationEl {
    matching_attributes_list: ListField<ListField<PrimField<String>>>,
}

impl CustomerprofilesDomainMatchingElAutoMergingElConsolidationEl {}

impl ToListMappable for CustomerprofilesDomainMatchingElAutoMergingElConsolidationEl {
    type O = BlockAssignable<CustomerprofilesDomainMatchingElAutoMergingElConsolidationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomainMatchingElAutoMergingElConsolidationEl {
    #[doc = ""]
    pub matching_attributes_list: ListField<ListField<PrimField<String>>>,
}

impl BuildCustomerprofilesDomainMatchingElAutoMergingElConsolidationEl {
    pub fn build(self) -> CustomerprofilesDomainMatchingElAutoMergingElConsolidationEl {
        CustomerprofilesDomainMatchingElAutoMergingElConsolidationEl {
            matching_attributes_list: self.matching_attributes_list,
        }
    }
}

pub struct CustomerprofilesDomainMatchingElAutoMergingElConsolidationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesDomainMatchingElAutoMergingElConsolidationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CustomerprofilesDomainMatchingElAutoMergingElConsolidationElRef {
        CustomerprofilesDomainMatchingElAutoMergingElConsolidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomainMatchingElAutoMergingElConsolidationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `matching_attributes_list` after provisioning.\n"]
    pub fn matching_attributes_list(&self) -> ListRef<ListRef<PrimExpr<String>>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.matching_attributes_list", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CustomerprofilesDomainMatchingElAutoMergingElDynamic {
    conflict_resolution:
        Option<DynamicBlock<CustomerprofilesDomainMatchingElAutoMergingElConflictResolutionEl>>,
    consolidation:
        Option<DynamicBlock<CustomerprofilesDomainMatchingElAutoMergingElConsolidationEl>>,
}

#[derive(Serialize)]
pub struct CustomerprofilesDomainMatchingElAutoMergingEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_allowed_confidence_score_for_merging: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conflict_resolution:
        Option<Vec<CustomerprofilesDomainMatchingElAutoMergingElConflictResolutionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consolidation: Option<Vec<CustomerprofilesDomainMatchingElAutoMergingElConsolidationEl>>,
    dynamic: CustomerprofilesDomainMatchingElAutoMergingElDynamic,
}

impl CustomerprofilesDomainMatchingElAutoMergingEl {
    #[doc = "Set the field `min_allowed_confidence_score_for_merging`.\n"]
    pub fn set_min_allowed_confidence_score_for_merging(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.min_allowed_confidence_score_for_merging = Some(v.into());
        self
    }

    #[doc = "Set the field `conflict_resolution`.\n"]
    pub fn set_conflict_resolution(
        mut self,
        v: impl Into<BlockAssignable<CustomerprofilesDomainMatchingElAutoMergingElConflictResolutionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conflict_resolution = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conflict_resolution = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `consolidation`.\n"]
    pub fn set_consolidation(
        mut self,
        v: impl Into<BlockAssignable<CustomerprofilesDomainMatchingElAutoMergingElConsolidationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.consolidation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.consolidation = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CustomerprofilesDomainMatchingElAutoMergingEl {
    type O = BlockAssignable<CustomerprofilesDomainMatchingElAutoMergingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomainMatchingElAutoMergingEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildCustomerprofilesDomainMatchingElAutoMergingEl {
    pub fn build(self) -> CustomerprofilesDomainMatchingElAutoMergingEl {
        CustomerprofilesDomainMatchingElAutoMergingEl {
            enabled: self.enabled,
            min_allowed_confidence_score_for_merging: core::default::Default::default(),
            conflict_resolution: core::default::Default::default(),
            consolidation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CustomerprofilesDomainMatchingElAutoMergingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesDomainMatchingElAutoMergingElRef {
    fn new(shared: StackShared, base: String) -> CustomerprofilesDomainMatchingElAutoMergingElRef {
        CustomerprofilesDomainMatchingElAutoMergingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomainMatchingElAutoMergingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `min_allowed_confidence_score_for_merging` after provisioning.\n"]
    pub fn min_allowed_confidence_score_for_merging(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_allowed_confidence_score_for_merging", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `conflict_resolution` after provisioning.\n"]
    pub fn conflict_resolution(
        &self,
    ) -> ListRef<CustomerprofilesDomainMatchingElAutoMergingElConflictResolutionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conflict_resolution", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `consolidation` after provisioning.\n"]
    pub fn consolidation(
        &self,
    ) -> ListRef<CustomerprofilesDomainMatchingElAutoMergingElConsolidationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consolidation", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesDomainMatchingElExportingConfigElS3ExportingEl {
    s3_bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_key_name: Option<PrimField<String>>,
}

impl CustomerprofilesDomainMatchingElExportingConfigElS3ExportingEl {
    #[doc = "Set the field `s3_key_name`.\n"]
    pub fn set_s3_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_key_name = Some(v.into());
        self
    }
}

impl ToListMappable for CustomerprofilesDomainMatchingElExportingConfigElS3ExportingEl {
    type O = BlockAssignable<CustomerprofilesDomainMatchingElExportingConfigElS3ExportingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomainMatchingElExportingConfigElS3ExportingEl {
    #[doc = ""]
    pub s3_bucket_name: PrimField<String>,
}

impl BuildCustomerprofilesDomainMatchingElExportingConfigElS3ExportingEl {
    pub fn build(self) -> CustomerprofilesDomainMatchingElExportingConfigElS3ExportingEl {
        CustomerprofilesDomainMatchingElExportingConfigElS3ExportingEl {
            s3_bucket_name: self.s3_bucket_name,
            s3_key_name: core::default::Default::default(),
        }
    }
}

pub struct CustomerprofilesDomainMatchingElExportingConfigElS3ExportingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesDomainMatchingElExportingConfigElS3ExportingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CustomerprofilesDomainMatchingElExportingConfigElS3ExportingElRef {
        CustomerprofilesDomainMatchingElExportingConfigElS3ExportingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomainMatchingElExportingConfigElS3ExportingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_key_name` after provisioning.\n"]
    pub fn s3_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_key_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct CustomerprofilesDomainMatchingElExportingConfigElDynamic {
    s3_exporting:
        Option<DynamicBlock<CustomerprofilesDomainMatchingElExportingConfigElS3ExportingEl>>,
}

#[derive(Serialize)]
pub struct CustomerprofilesDomainMatchingElExportingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_exporting: Option<Vec<CustomerprofilesDomainMatchingElExportingConfigElS3ExportingEl>>,
    dynamic: CustomerprofilesDomainMatchingElExportingConfigElDynamic,
}

impl CustomerprofilesDomainMatchingElExportingConfigEl {
    #[doc = "Set the field `s3_exporting`.\n"]
    pub fn set_s3_exporting(
        mut self,
        v: impl Into<BlockAssignable<CustomerprofilesDomainMatchingElExportingConfigElS3ExportingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_exporting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_exporting = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CustomerprofilesDomainMatchingElExportingConfigEl {
    type O = BlockAssignable<CustomerprofilesDomainMatchingElExportingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomainMatchingElExportingConfigEl {}

impl BuildCustomerprofilesDomainMatchingElExportingConfigEl {
    pub fn build(self) -> CustomerprofilesDomainMatchingElExportingConfigEl {
        CustomerprofilesDomainMatchingElExportingConfigEl {
            s3_exporting: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CustomerprofilesDomainMatchingElExportingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesDomainMatchingElExportingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CustomerprofilesDomainMatchingElExportingConfigElRef {
        CustomerprofilesDomainMatchingElExportingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomainMatchingElExportingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_exporting` after provisioning.\n"]
    pub fn s3_exporting(
        &self,
    ) -> ListRef<CustomerprofilesDomainMatchingElExportingConfigElS3ExportingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_exporting", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesDomainMatchingElJobScheduleEl {
    day_of_the_week: PrimField<String>,
    time: PrimField<String>,
}

impl CustomerprofilesDomainMatchingElJobScheduleEl {}

impl ToListMappable for CustomerprofilesDomainMatchingElJobScheduleEl {
    type O = BlockAssignable<CustomerprofilesDomainMatchingElJobScheduleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomainMatchingElJobScheduleEl {
    #[doc = ""]
    pub day_of_the_week: PrimField<String>,
    #[doc = ""]
    pub time: PrimField<String>,
}

impl BuildCustomerprofilesDomainMatchingElJobScheduleEl {
    pub fn build(self) -> CustomerprofilesDomainMatchingElJobScheduleEl {
        CustomerprofilesDomainMatchingElJobScheduleEl {
            day_of_the_week: self.day_of_the_week,
            time: self.time,
        }
    }
}

pub struct CustomerprofilesDomainMatchingElJobScheduleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesDomainMatchingElJobScheduleElRef {
    fn new(shared: StackShared, base: String) -> CustomerprofilesDomainMatchingElJobScheduleElRef {
        CustomerprofilesDomainMatchingElJobScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomainMatchingElJobScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_the_week` after provisioning.\n"]
    pub fn day_of_the_week(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.day_of_the_week", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `time` after provisioning.\n"]
    pub fn time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time", self.base))
    }
}

#[derive(Serialize, Default)]
struct CustomerprofilesDomainMatchingElDynamic {
    auto_merging: Option<DynamicBlock<CustomerprofilesDomainMatchingElAutoMergingEl>>,
    exporting_config: Option<DynamicBlock<CustomerprofilesDomainMatchingElExportingConfigEl>>,
    job_schedule: Option<DynamicBlock<CustomerprofilesDomainMatchingElJobScheduleEl>>,
}

#[derive(Serialize)]
pub struct CustomerprofilesDomainMatchingEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_merging: Option<Vec<CustomerprofilesDomainMatchingElAutoMergingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exporting_config: Option<Vec<CustomerprofilesDomainMatchingElExportingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_schedule: Option<Vec<CustomerprofilesDomainMatchingElJobScheduleEl>>,
    dynamic: CustomerprofilesDomainMatchingElDynamic,
}

impl CustomerprofilesDomainMatchingEl {
    #[doc = "Set the field `auto_merging`.\n"]
    pub fn set_auto_merging(
        mut self,
        v: impl Into<BlockAssignable<CustomerprofilesDomainMatchingElAutoMergingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auto_merging = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_merging = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `exporting_config`.\n"]
    pub fn set_exporting_config(
        mut self,
        v: impl Into<BlockAssignable<CustomerprofilesDomainMatchingElExportingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exporting_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exporting_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `job_schedule`.\n"]
    pub fn set_job_schedule(
        mut self,
        v: impl Into<BlockAssignable<CustomerprofilesDomainMatchingElJobScheduleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.job_schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.job_schedule = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CustomerprofilesDomainMatchingEl {
    type O = BlockAssignable<CustomerprofilesDomainMatchingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomainMatchingEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildCustomerprofilesDomainMatchingEl {
    pub fn build(self) -> CustomerprofilesDomainMatchingEl {
        CustomerprofilesDomainMatchingEl {
            enabled: self.enabled,
            auto_merging: core::default::Default::default(),
            exporting_config: core::default::Default::default(),
            job_schedule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CustomerprofilesDomainMatchingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesDomainMatchingElRef {
    fn new(shared: StackShared, base: String) -> CustomerprofilesDomainMatchingElRef {
        CustomerprofilesDomainMatchingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomainMatchingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_merging` after provisioning.\n"]
    pub fn auto_merging(&self) -> ListRef<CustomerprofilesDomainMatchingElAutoMergingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_merging", self.base))
    }

    #[doc = "Get a reference to the value of field `exporting_config` after provisioning.\n"]
    pub fn exporting_config(
        &self,
    ) -> ListRef<CustomerprofilesDomainMatchingElExportingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exporting_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `job_schedule` after provisioning.\n"]
    pub fn job_schedule(&self) -> ListRef<CustomerprofilesDomainMatchingElJobScheduleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.job_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesDomainRuleBasedMatchingElAttributeTypesSelectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<ListField<PrimField<String>>>,
    attribute_matching_model: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_address: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<ListField<PrimField<String>>>,
}

impl CustomerprofilesDomainRuleBasedMatchingElAttributeTypesSelectorEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.address = Some(v.into());
        self
    }

    #[doc = "Set the field `email_address`.\n"]
    pub fn set_email_address(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email_address = Some(v.into());
        self
    }

    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.phone_number = Some(v.into());
        self
    }
}

impl ToListMappable for CustomerprofilesDomainRuleBasedMatchingElAttributeTypesSelectorEl {
    type O = BlockAssignable<CustomerprofilesDomainRuleBasedMatchingElAttributeTypesSelectorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomainRuleBasedMatchingElAttributeTypesSelectorEl {
    #[doc = ""]
    pub attribute_matching_model: PrimField<String>,
}

impl BuildCustomerprofilesDomainRuleBasedMatchingElAttributeTypesSelectorEl {
    pub fn build(self) -> CustomerprofilesDomainRuleBasedMatchingElAttributeTypesSelectorEl {
        CustomerprofilesDomainRuleBasedMatchingElAttributeTypesSelectorEl {
            address: core::default::Default::default(),
            attribute_matching_model: self.attribute_matching_model,
            email_address: core::default::Default::default(),
            phone_number: core::default::Default::default(),
        }
    }
}

pub struct CustomerprofilesDomainRuleBasedMatchingElAttributeTypesSelectorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesDomainRuleBasedMatchingElAttributeTypesSelectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CustomerprofilesDomainRuleBasedMatchingElAttributeTypesSelectorElRef {
        CustomerprofilesDomainRuleBasedMatchingElAttributeTypesSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomainRuleBasedMatchingElAttributeTypesSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `attribute_matching_model` after provisioning.\n"]
    pub fn attribute_matching_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.attribute_matching_model", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `email_address` after provisioning.\n"]
    pub fn email_address(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.email_address", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesDomainRuleBasedMatchingElConflictResolutionEl {
    conflict_resolving_model: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_name: Option<PrimField<String>>,
}

impl CustomerprofilesDomainRuleBasedMatchingElConflictResolutionEl {
    #[doc = "Set the field `source_name`.\n"]
    pub fn set_source_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_name = Some(v.into());
        self
    }
}

impl ToListMappable for CustomerprofilesDomainRuleBasedMatchingElConflictResolutionEl {
    type O = BlockAssignable<CustomerprofilesDomainRuleBasedMatchingElConflictResolutionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomainRuleBasedMatchingElConflictResolutionEl {
    #[doc = ""]
    pub conflict_resolving_model: PrimField<String>,
}

impl BuildCustomerprofilesDomainRuleBasedMatchingElConflictResolutionEl {
    pub fn build(self) -> CustomerprofilesDomainRuleBasedMatchingElConflictResolutionEl {
        CustomerprofilesDomainRuleBasedMatchingElConflictResolutionEl {
            conflict_resolving_model: self.conflict_resolving_model,
            source_name: core::default::Default::default(),
        }
    }
}

pub struct CustomerprofilesDomainRuleBasedMatchingElConflictResolutionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesDomainRuleBasedMatchingElConflictResolutionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CustomerprofilesDomainRuleBasedMatchingElConflictResolutionElRef {
        CustomerprofilesDomainRuleBasedMatchingElConflictResolutionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomainRuleBasedMatchingElConflictResolutionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conflict_resolving_model` after provisioning.\n"]
    pub fn conflict_resolving_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.conflict_resolving_model", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_name` after provisioning.\n"]
    pub fn source_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_name", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesDomainRuleBasedMatchingElExportingConfigElS3ExportingEl {
    s3_bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_key_name: Option<PrimField<String>>,
}

impl CustomerprofilesDomainRuleBasedMatchingElExportingConfigElS3ExportingEl {
    #[doc = "Set the field `s3_key_name`.\n"]
    pub fn set_s3_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_key_name = Some(v.into());
        self
    }
}

impl ToListMappable for CustomerprofilesDomainRuleBasedMatchingElExportingConfigElS3ExportingEl {
    type O =
        BlockAssignable<CustomerprofilesDomainRuleBasedMatchingElExportingConfigElS3ExportingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomainRuleBasedMatchingElExportingConfigElS3ExportingEl {
    #[doc = ""]
    pub s3_bucket_name: PrimField<String>,
}

impl BuildCustomerprofilesDomainRuleBasedMatchingElExportingConfigElS3ExportingEl {
    pub fn build(self) -> CustomerprofilesDomainRuleBasedMatchingElExportingConfigElS3ExportingEl {
        CustomerprofilesDomainRuleBasedMatchingElExportingConfigElS3ExportingEl {
            s3_bucket_name: self.s3_bucket_name,
            s3_key_name: core::default::Default::default(),
        }
    }
}

pub struct CustomerprofilesDomainRuleBasedMatchingElExportingConfigElS3ExportingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesDomainRuleBasedMatchingElExportingConfigElS3ExportingElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CustomerprofilesDomainRuleBasedMatchingElExportingConfigElS3ExportingElRef {
        CustomerprofilesDomainRuleBasedMatchingElExportingConfigElS3ExportingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomainRuleBasedMatchingElExportingConfigElS3ExportingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.s3_bucket_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_key_name` after provisioning.\n"]
    pub fn s3_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_key_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct CustomerprofilesDomainRuleBasedMatchingElExportingConfigElDynamic {
    s3_exporting: Option<
        DynamicBlock<CustomerprofilesDomainRuleBasedMatchingElExportingConfigElS3ExportingEl>,
    >,
}

#[derive(Serialize)]
pub struct CustomerprofilesDomainRuleBasedMatchingElExportingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_exporting:
        Option<Vec<CustomerprofilesDomainRuleBasedMatchingElExportingConfigElS3ExportingEl>>,
    dynamic: CustomerprofilesDomainRuleBasedMatchingElExportingConfigElDynamic,
}

impl CustomerprofilesDomainRuleBasedMatchingElExportingConfigEl {
    #[doc = "Set the field `s3_exporting`.\n"]
    pub fn set_s3_exporting(
        mut self,
        v: impl Into<
            BlockAssignable<
                CustomerprofilesDomainRuleBasedMatchingElExportingConfigElS3ExportingEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_exporting = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_exporting = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CustomerprofilesDomainRuleBasedMatchingElExportingConfigEl {
    type O = BlockAssignable<CustomerprofilesDomainRuleBasedMatchingElExportingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomainRuleBasedMatchingElExportingConfigEl {}

impl BuildCustomerprofilesDomainRuleBasedMatchingElExportingConfigEl {
    pub fn build(self) -> CustomerprofilesDomainRuleBasedMatchingElExportingConfigEl {
        CustomerprofilesDomainRuleBasedMatchingElExportingConfigEl {
            s3_exporting: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CustomerprofilesDomainRuleBasedMatchingElExportingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesDomainRuleBasedMatchingElExportingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CustomerprofilesDomainRuleBasedMatchingElExportingConfigElRef {
        CustomerprofilesDomainRuleBasedMatchingElExportingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomainRuleBasedMatchingElExportingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_exporting` after provisioning.\n"]
    pub fn s3_exporting(
        &self,
    ) -> ListRef<CustomerprofilesDomainRuleBasedMatchingElExportingConfigElS3ExportingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_exporting", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesDomainRuleBasedMatchingElMatchingRulesEl {
    rule: ListField<PrimField<String>>,
}

impl CustomerprofilesDomainRuleBasedMatchingElMatchingRulesEl {}

impl ToListMappable for CustomerprofilesDomainRuleBasedMatchingElMatchingRulesEl {
    type O = BlockAssignable<CustomerprofilesDomainRuleBasedMatchingElMatchingRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomainRuleBasedMatchingElMatchingRulesEl {
    #[doc = ""]
    pub rule: ListField<PrimField<String>>,
}

impl BuildCustomerprofilesDomainRuleBasedMatchingElMatchingRulesEl {
    pub fn build(self) -> CustomerprofilesDomainRuleBasedMatchingElMatchingRulesEl {
        CustomerprofilesDomainRuleBasedMatchingElMatchingRulesEl { rule: self.rule }
    }
}

pub struct CustomerprofilesDomainRuleBasedMatchingElMatchingRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesDomainRuleBasedMatchingElMatchingRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CustomerprofilesDomainRuleBasedMatchingElMatchingRulesElRef {
        CustomerprofilesDomainRuleBasedMatchingElMatchingRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomainRuleBasedMatchingElMatchingRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct CustomerprofilesDomainRuleBasedMatchingElDynamic {
    attribute_types_selector:
        Option<DynamicBlock<CustomerprofilesDomainRuleBasedMatchingElAttributeTypesSelectorEl>>,
    conflict_resolution:
        Option<DynamicBlock<CustomerprofilesDomainRuleBasedMatchingElConflictResolutionEl>>,
    exporting_config:
        Option<DynamicBlock<CustomerprofilesDomainRuleBasedMatchingElExportingConfigEl>>,
    matching_rules: Option<DynamicBlock<CustomerprofilesDomainRuleBasedMatchingElMatchingRulesEl>>,
}

#[derive(Serialize)]
pub struct CustomerprofilesDomainRuleBasedMatchingEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_allowed_rule_level_for_matching: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_allowed_rule_level_for_merging: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_types_selector:
        Option<Vec<CustomerprofilesDomainRuleBasedMatchingElAttributeTypesSelectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conflict_resolution: Option<Vec<CustomerprofilesDomainRuleBasedMatchingElConflictResolutionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exporting_config: Option<Vec<CustomerprofilesDomainRuleBasedMatchingElExportingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matching_rules: Option<Vec<CustomerprofilesDomainRuleBasedMatchingElMatchingRulesEl>>,
    dynamic: CustomerprofilesDomainRuleBasedMatchingElDynamic,
}

impl CustomerprofilesDomainRuleBasedMatchingEl {
    #[doc = "Set the field `max_allowed_rule_level_for_matching`.\n"]
    pub fn set_max_allowed_rule_level_for_matching(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_allowed_rule_level_for_matching = Some(v.into());
        self
    }

    #[doc = "Set the field `max_allowed_rule_level_for_merging`.\n"]
    pub fn set_max_allowed_rule_level_for_merging(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_allowed_rule_level_for_merging = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc = "Set the field `attribute_types_selector`.\n"]
    pub fn set_attribute_types_selector(
        mut self,
        v: impl Into<BlockAssignable<CustomerprofilesDomainRuleBasedMatchingElAttributeTypesSelectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.attribute_types_selector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.attribute_types_selector = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `conflict_resolution`.\n"]
    pub fn set_conflict_resolution(
        mut self,
        v: impl Into<BlockAssignable<CustomerprofilesDomainRuleBasedMatchingElConflictResolutionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conflict_resolution = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conflict_resolution = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `exporting_config`.\n"]
    pub fn set_exporting_config(
        mut self,
        v: impl Into<BlockAssignable<CustomerprofilesDomainRuleBasedMatchingElExportingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exporting_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exporting_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `matching_rules`.\n"]
    pub fn set_matching_rules(
        mut self,
        v: impl Into<BlockAssignable<CustomerprofilesDomainRuleBasedMatchingElMatchingRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.matching_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.matching_rules = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CustomerprofilesDomainRuleBasedMatchingEl {
    type O = BlockAssignable<CustomerprofilesDomainRuleBasedMatchingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomainRuleBasedMatchingEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildCustomerprofilesDomainRuleBasedMatchingEl {
    pub fn build(self) -> CustomerprofilesDomainRuleBasedMatchingEl {
        CustomerprofilesDomainRuleBasedMatchingEl {
            enabled: self.enabled,
            max_allowed_rule_level_for_matching: core::default::Default::default(),
            max_allowed_rule_level_for_merging: core::default::Default::default(),
            status: core::default::Default::default(),
            attribute_types_selector: core::default::Default::default(),
            conflict_resolution: core::default::Default::default(),
            exporting_config: core::default::Default::default(),
            matching_rules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CustomerprofilesDomainRuleBasedMatchingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CustomerprofilesDomainRuleBasedMatchingElRef {
    fn new(shared: StackShared, base: String) -> CustomerprofilesDomainRuleBasedMatchingElRef {
        CustomerprofilesDomainRuleBasedMatchingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomainRuleBasedMatchingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `max_allowed_rule_level_for_matching` after provisioning.\n"]
    pub fn max_allowed_rule_level_for_matching(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_allowed_rule_level_for_matching", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_allowed_rule_level_for_merging` after provisioning.\n"]
    pub fn max_allowed_rule_level_for_merging(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_allowed_rule_level_for_merging", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `attribute_types_selector` after provisioning.\n"]
    pub fn attribute_types_selector(
        &self,
    ) -> ListRef<CustomerprofilesDomainRuleBasedMatchingElAttributeTypesSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attribute_types_selector", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `conflict_resolution` after provisioning.\n"]
    pub fn conflict_resolution(
        &self,
    ) -> ListRef<CustomerprofilesDomainRuleBasedMatchingElConflictResolutionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.conflict_resolution", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `exporting_config` after provisioning.\n"]
    pub fn exporting_config(
        &self,
    ) -> ListRef<CustomerprofilesDomainRuleBasedMatchingElExportingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exporting_config", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CustomerprofilesDomainDynamic {
    matching: Option<DynamicBlock<CustomerprofilesDomainMatchingEl>>,
    rule_based_matching: Option<DynamicBlock<CustomerprofilesDomainRuleBasedMatchingEl>>,
}
