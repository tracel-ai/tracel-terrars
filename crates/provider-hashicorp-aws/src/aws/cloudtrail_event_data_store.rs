use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudtrailEventDataStoreData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_region_enabled: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suspend: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    termination_protection_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_event_selector: Option<Vec<CloudtrailEventDataStoreAdvancedEventSelectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudtrailEventDataStoreTimeoutsEl>,
    dynamic: CloudtrailEventDataStoreDynamic,
}

struct CloudtrailEventDataStore_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudtrailEventDataStoreData>,
}

#[derive(Clone)]
pub struct CloudtrailEventDataStore(Rc<CloudtrailEventDataStore_>);

impl CloudtrailEventDataStore {
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

    #[doc = "Set the field `billing_mode`.\n"]
    pub fn set_billing_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().billing_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `multi_region_enabled`.\n"]
    pub fn set_multi_region_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multi_region_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `organization_enabled`.\n"]
    pub fn set_organization_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().organization_enabled = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `retention_period`.\n"]
    pub fn set_retention_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retention_period = Some(v.into());
        self
    }

    #[doc = "Set the field `suspend`.\n"]
    pub fn set_suspend(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().suspend = Some(v.into());
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

    #[doc = "Set the field `termination_protection_enabled`.\n"]
    pub fn set_termination_protection_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().termination_protection_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_event_selector`.\n"]
    pub fn set_advanced_event_selector(
        self,
        v: impl Into<BlockAssignable<CloudtrailEventDataStoreAdvancedEventSelectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_event_selector = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_event_selector = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudtrailEventDataStoreTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_mode` after provisioning.\n"]
    pub fn billing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.billing_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_region_enabled` after provisioning.\n"]
    pub fn multi_region_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_region_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organization_enabled` after provisioning.\n"]
    pub fn organization_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.organization_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `suspend` after provisioning.\n"]
    pub fn suspend(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suspend", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `termination_protection_enabled` after provisioning.\n"]
    pub fn termination_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.termination_protection_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_event_selector` after provisioning.\n"]
    pub fn advanced_event_selector(&self) -> ListRef<CloudtrailEventDataStoreAdvancedEventSelectorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_event_selector", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudtrailEventDataStoreTimeoutsElRef {
        CloudtrailEventDataStoreTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for CloudtrailEventDataStore {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CloudtrailEventDataStore { }

impl ToListMappable for CloudtrailEventDataStore {
    type O = ListRef<CloudtrailEventDataStoreRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudtrailEventDataStore_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudtrail_event_data_store".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudtrailEventDataStore {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCloudtrailEventDataStore {
    pub fn build(self, stack: &mut Stack) -> CloudtrailEventDataStore {
        let out = CloudtrailEventDataStore(Rc::new(CloudtrailEventDataStore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudtrailEventDataStoreData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                billing_mode: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                multi_region_enabled: core::default::Default::default(),
                name: self.name,
                organization_enabled: core::default::Default::default(),
                region: core::default::Default::default(),
                retention_period: core::default::Default::default(),
                suspend: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                termination_protection_enabled: core::default::Default::default(),
                advanced_event_selector: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudtrailEventDataStoreRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudtrailEventDataStoreRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CloudtrailEventDataStoreRef {
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

    #[doc = "Get a reference to the value of field `billing_mode` after provisioning.\n"]
    pub fn billing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.billing_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_region_enabled` after provisioning.\n"]
    pub fn multi_region_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_region_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organization_enabled` after provisioning.\n"]
    pub fn organization_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.organization_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `suspend` after provisioning.\n"]
    pub fn suspend(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suspend", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `termination_protection_enabled` after provisioning.\n"]
    pub fn termination_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.termination_protection_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_event_selector` after provisioning.\n"]
    pub fn advanced_event_selector(&self) -> ListRef<CloudtrailEventDataStoreAdvancedEventSelectorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_event_selector", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudtrailEventDataStoreTimeoutsElRef {
        CloudtrailEventDataStoreTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ends_with: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    equals: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_ends_with: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_equals: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_starts_with: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starts_with: Option<ListField<PrimField<String>>>,
}

impl CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl {
    #[doc = "Set the field `ends_with`.\n"]
    pub fn set_ends_with(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ends_with = Some(v.into());
        self
    }

    #[doc = "Set the field `equals`.\n"]
    pub fn set_equals(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.equals = Some(v.into());
        self
    }

    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }

    #[doc = "Set the field `not_ends_with`.\n"]
    pub fn set_not_ends_with(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.not_ends_with = Some(v.into());
        self
    }

    #[doc = "Set the field `not_equals`.\n"]
    pub fn set_not_equals(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.not_equals = Some(v.into());
        self
    }

    #[doc = "Set the field `not_starts_with`.\n"]
    pub fn set_not_starts_with(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.not_starts_with = Some(v.into());
        self
    }

    #[doc = "Set the field `starts_with`.\n"]
    pub fn set_starts_with(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.starts_with = Some(v.into());
        self
    }
}

impl ToListMappable for CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl {
    type O = BlockAssignable<CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl {}

impl BuildCloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl {
    pub fn build(self) -> CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl {
        CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl {
            ends_with: core::default::Default::default(),
            equals: core::default::Default::default(),
            field: core::default::Default::default(),
            not_ends_with: core::default::Default::default(),
            not_equals: core::default::Default::default(),
            not_starts_with: core::default::Default::default(),
            starts_with: core::default::Default::default(),
        }
    }
}

pub struct CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorElRef {
    fn new(shared: StackShared, base: String) -> CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorElRef {
        CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ends_with` after provisioning.\n"]
    pub fn ends_with(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ends_with", self.base))
    }

    #[doc = "Get a reference to the value of field `equals` after provisioning.\n"]
    pub fn equals(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.equals", self.base))
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `not_ends_with` after provisioning.\n"]
    pub fn not_ends_with(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.not_ends_with", self.base))
    }

    #[doc = "Get a reference to the value of field `not_equals` after provisioning.\n"]
    pub fn not_equals(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.not_equals", self.base))
    }

    #[doc = "Get a reference to the value of field `not_starts_with` after provisioning.\n"]
    pub fn not_starts_with(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.not_starts_with", self.base))
    }

    #[doc = "Get a reference to the value of field `starts_with` after provisioning.\n"]
    pub fn starts_with(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.starts_with", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudtrailEventDataStoreAdvancedEventSelectorElDynamic {
    field_selector: Option<DynamicBlock<CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl>>,
}

#[derive(Serialize)]
pub struct CloudtrailEventDataStoreAdvancedEventSelectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_selector: Option<Vec<CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl>>,
    dynamic: CloudtrailEventDataStoreAdvancedEventSelectorElDynamic,
}

impl CloudtrailEventDataStoreAdvancedEventSelectorEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `field_selector`.\n"]
    pub fn set_field_selector(
        mut self,
        v: impl Into<BlockAssignable<CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.field_selector = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.field_selector = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CloudtrailEventDataStoreAdvancedEventSelectorEl {
    type O = BlockAssignable<CloudtrailEventDataStoreAdvancedEventSelectorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudtrailEventDataStoreAdvancedEventSelectorEl {}

impl BuildCloudtrailEventDataStoreAdvancedEventSelectorEl {
    pub fn build(self) -> CloudtrailEventDataStoreAdvancedEventSelectorEl {
        CloudtrailEventDataStoreAdvancedEventSelectorEl {
            name: core::default::Default::default(),
            field_selector: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudtrailEventDataStoreAdvancedEventSelectorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudtrailEventDataStoreAdvancedEventSelectorElRef {
    fn new(shared: StackShared, base: String) -> CloudtrailEventDataStoreAdvancedEventSelectorElRef {
        CloudtrailEventDataStoreAdvancedEventSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudtrailEventDataStoreAdvancedEventSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudtrailEventDataStoreTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl CloudtrailEventDataStoreTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for CloudtrailEventDataStoreTimeoutsEl {
    type O = BlockAssignable<CloudtrailEventDataStoreTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudtrailEventDataStoreTimeoutsEl {}

impl BuildCloudtrailEventDataStoreTimeoutsEl {
    pub fn build(self) -> CloudtrailEventDataStoreTimeoutsEl {
        CloudtrailEventDataStoreTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CloudtrailEventDataStoreTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudtrailEventDataStoreTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudtrailEventDataStoreTimeoutsElRef {
        CloudtrailEventDataStoreTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudtrailEventDataStoreTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudtrailEventDataStoreDynamic {
    advanced_event_selector: Option<DynamicBlock<CloudtrailEventDataStoreAdvancedEventSelectorEl>>,
}
