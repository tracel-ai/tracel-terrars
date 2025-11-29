use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudtrailData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_watch_logs_group_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_watch_logs_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_log_file_validation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_global_service_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_multi_region_trail: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_organization_trail: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    s3_bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_key_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sns_topic_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_event_selector: Option<Vec<CloudtrailAdvancedEventSelectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_selector: Option<Vec<CloudtrailEventSelectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insight_selector: Option<Vec<CloudtrailInsightSelectorEl>>,
    dynamic: CloudtrailDynamic,
}

struct Cloudtrail_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudtrailData>,
}

#[derive(Clone)]
pub struct Cloudtrail(Rc<Cloudtrail_>);

impl Cloudtrail {
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

    #[doc = "Set the field `cloud_watch_logs_group_arn`.\n"]
    pub fn set_cloud_watch_logs_group_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_watch_logs_group_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_watch_logs_role_arn`.\n"]
    pub fn set_cloud_watch_logs_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloud_watch_logs_role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_log_file_validation`.\n"]
    pub fn set_enable_log_file_validation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_log_file_validation = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_logging`.\n"]
    pub fn set_enable_logging(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_logging = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `include_global_service_events`.\n"]
    pub fn set_include_global_service_events(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_global_service_events = Some(v.into());
        self
    }

    #[doc = "Set the field `is_multi_region_trail`.\n"]
    pub fn set_is_multi_region_trail(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_multi_region_trail = Some(v.into());
        self
    }

    #[doc = "Set the field `is_organization_trail`.\n"]
    pub fn set_is_organization_trail(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_organization_trail = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_key_prefix`.\n"]
    pub fn set_s3_key_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_key_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `sns_topic_name`.\n"]
    pub fn set_sns_topic_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sns_topic_name = Some(v.into());
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

    #[doc = "Set the field `advanced_event_selector`.\n"]
    pub fn set_advanced_event_selector(self, v: impl Into<BlockAssignable<CloudtrailAdvancedEventSelectorEl>>) -> Self {
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

    #[doc = "Set the field `event_selector`.\n"]
    pub fn set_event_selector(self, v: impl Into<BlockAssignable<CloudtrailEventSelectorEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_selector = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_selector = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `insight_selector`.\n"]
    pub fn set_insight_selector(self, v: impl Into<BlockAssignable<CloudtrailInsightSelectorEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().insight_selector = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.insight_selector = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_watch_logs_group_arn` after provisioning.\n"]
    pub fn cloud_watch_logs_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_logs_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_watch_logs_role_arn` after provisioning.\n"]
    pub fn cloud_watch_logs_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_logs_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_log_file_validation` after provisioning.\n"]
    pub fn enable_log_file_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_log_file_validation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_logging` after provisioning.\n"]
    pub fn enable_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_region` after provisioning.\n"]
    pub fn home_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.home_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_global_service_events` after provisioning.\n"]
    pub fn include_global_service_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_global_service_events", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_multi_region_trail` after provisioning.\n"]
    pub fn is_multi_region_trail(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_multi_region_trail", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_organization_trail` after provisioning.\n"]
    pub fn is_organization_trail(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_organization_trail", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_key_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_topic_name` after provisioning.\n"]
    pub fn sns_topic_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_event_selector` after provisioning.\n"]
    pub fn advanced_event_selector(&self) -> ListRef<CloudtrailAdvancedEventSelectorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_event_selector", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_selector` after provisioning.\n"]
    pub fn event_selector(&self) -> ListRef<CloudtrailEventSelectorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.event_selector", self.extract_ref()))
    }
}

impl Referable for Cloudtrail {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Cloudtrail { }

impl ToListMappable for Cloudtrail {
    type O = ListRef<CloudtrailRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Cloudtrail_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudtrail".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudtrail {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub s3_bucket_name: PrimField<String>,
}

impl BuildCloudtrail {
    pub fn build(self, stack: &mut Stack) -> Cloudtrail {
        let out = Cloudtrail(Rc::new(Cloudtrail_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudtrailData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cloud_watch_logs_group_arn: core::default::Default::default(),
                cloud_watch_logs_role_arn: core::default::Default::default(),
                enable_log_file_validation: core::default::Default::default(),
                enable_logging: core::default::Default::default(),
                id: core::default::Default::default(),
                include_global_service_events: core::default::Default::default(),
                is_multi_region_trail: core::default::Default::default(),
                is_organization_trail: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                s3_bucket_name: self.s3_bucket_name,
                s3_key_prefix: core::default::Default::default(),
                sns_topic_name: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                advanced_event_selector: core::default::Default::default(),
                event_selector: core::default::Default::default(),
                insight_selector: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudtrailRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudtrailRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CloudtrailRef {
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

    #[doc = "Get a reference to the value of field `cloud_watch_logs_group_arn` after provisioning.\n"]
    pub fn cloud_watch_logs_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_logs_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_watch_logs_role_arn` after provisioning.\n"]
    pub fn cloud_watch_logs_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_logs_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_log_file_validation` after provisioning.\n"]
    pub fn enable_log_file_validation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_log_file_validation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_logging` after provisioning.\n"]
    pub fn enable_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_region` after provisioning.\n"]
    pub fn home_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.home_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_global_service_events` after provisioning.\n"]
    pub fn include_global_service_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_global_service_events", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_multi_region_trail` after provisioning.\n"]
    pub fn is_multi_region_trail(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_multi_region_trail", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_organization_trail` after provisioning.\n"]
    pub fn is_organization_trail(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_organization_trail", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_key_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_topic_name` after provisioning.\n"]
    pub fn sns_topic_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_event_selector` after provisioning.\n"]
    pub fn advanced_event_selector(&self) -> ListRef<CloudtrailAdvancedEventSelectorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_event_selector", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_selector` after provisioning.\n"]
    pub fn event_selector(&self) -> ListRef<CloudtrailEventSelectorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.event_selector", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudtrailAdvancedEventSelectorElFieldSelectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ends_with: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    equals: Option<ListField<PrimField<String>>>,
    field: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_ends_with: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_equals: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_starts_with: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starts_with: Option<ListField<PrimField<String>>>,
}

impl CloudtrailAdvancedEventSelectorElFieldSelectorEl {
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

impl ToListMappable for CloudtrailAdvancedEventSelectorElFieldSelectorEl {
    type O = BlockAssignable<CloudtrailAdvancedEventSelectorElFieldSelectorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudtrailAdvancedEventSelectorElFieldSelectorEl {
    #[doc = ""]
    pub field: PrimField<String>,
}

impl BuildCloudtrailAdvancedEventSelectorElFieldSelectorEl {
    pub fn build(self) -> CloudtrailAdvancedEventSelectorElFieldSelectorEl {
        CloudtrailAdvancedEventSelectorElFieldSelectorEl {
            ends_with: core::default::Default::default(),
            equals: core::default::Default::default(),
            field: self.field,
            not_ends_with: core::default::Default::default(),
            not_equals: core::default::Default::default(),
            not_starts_with: core::default::Default::default(),
            starts_with: core::default::Default::default(),
        }
    }
}

pub struct CloudtrailAdvancedEventSelectorElFieldSelectorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudtrailAdvancedEventSelectorElFieldSelectorElRef {
    fn new(shared: StackShared, base: String) -> CloudtrailAdvancedEventSelectorElFieldSelectorElRef {
        CloudtrailAdvancedEventSelectorElFieldSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudtrailAdvancedEventSelectorElFieldSelectorElRef {
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
struct CloudtrailAdvancedEventSelectorElDynamic {
    field_selector: Option<DynamicBlock<CloudtrailAdvancedEventSelectorElFieldSelectorEl>>,
}

#[derive(Serialize)]
pub struct CloudtrailAdvancedEventSelectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_selector: Option<Vec<CloudtrailAdvancedEventSelectorElFieldSelectorEl>>,
    dynamic: CloudtrailAdvancedEventSelectorElDynamic,
}

impl CloudtrailAdvancedEventSelectorEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `field_selector`.\n"]
    pub fn set_field_selector(
        mut self,
        v: impl Into<BlockAssignable<CloudtrailAdvancedEventSelectorElFieldSelectorEl>>,
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

impl ToListMappable for CloudtrailAdvancedEventSelectorEl {
    type O = BlockAssignable<CloudtrailAdvancedEventSelectorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudtrailAdvancedEventSelectorEl {}

impl BuildCloudtrailAdvancedEventSelectorEl {
    pub fn build(self) -> CloudtrailAdvancedEventSelectorEl {
        CloudtrailAdvancedEventSelectorEl {
            name: core::default::Default::default(),
            field_selector: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudtrailAdvancedEventSelectorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudtrailAdvancedEventSelectorElRef {
    fn new(shared: StackShared, base: String) -> CloudtrailAdvancedEventSelectorElRef {
        CloudtrailAdvancedEventSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudtrailAdvancedEventSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudtrailEventSelectorElDataResourceEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    values: ListField<PrimField<String>>,
}

impl CloudtrailEventSelectorElDataResourceEl { }

impl ToListMappable for CloudtrailEventSelectorElDataResourceEl {
    type O = BlockAssignable<CloudtrailEventSelectorElDataResourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudtrailEventSelectorElDataResourceEl {
    #[doc = ""]
    pub type_: PrimField<String>,
    #[doc = ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildCloudtrailEventSelectorElDataResourceEl {
    pub fn build(self) -> CloudtrailEventSelectorElDataResourceEl {
        CloudtrailEventSelectorElDataResourceEl {
            type_: self.type_,
            values: self.values,
        }
    }
}

pub struct CloudtrailEventSelectorElDataResourceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudtrailEventSelectorElDataResourceElRef {
    fn new(shared: StackShared, base: String) -> CloudtrailEventSelectorElDataResourceElRef {
        CloudtrailEventSelectorElDataResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudtrailEventSelectorElDataResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudtrailEventSelectorElDynamic {
    data_resource: Option<DynamicBlock<CloudtrailEventSelectorElDataResourceEl>>,
}

#[derive(Serialize)]
pub struct CloudtrailEventSelectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_management_event_sources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_management_events: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_write_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_resource: Option<Vec<CloudtrailEventSelectorElDataResourceEl>>,
    dynamic: CloudtrailEventSelectorElDynamic,
}

impl CloudtrailEventSelectorEl {
    #[doc = "Set the field `exclude_management_event_sources`.\n"]
    pub fn set_exclude_management_event_sources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exclude_management_event_sources = Some(v.into());
        self
    }

    #[doc = "Set the field `include_management_events`.\n"]
    pub fn set_include_management_events(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_management_events = Some(v.into());
        self
    }

    #[doc = "Set the field `read_write_type`.\n"]
    pub fn set_read_write_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read_write_type = Some(v.into());
        self
    }

    #[doc = "Set the field `data_resource`.\n"]
    pub fn set_data_resource(mut self, v: impl Into<BlockAssignable<CloudtrailEventSelectorElDataResourceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_resource = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_resource = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CloudtrailEventSelectorEl {
    type O = BlockAssignable<CloudtrailEventSelectorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudtrailEventSelectorEl {}

impl BuildCloudtrailEventSelectorEl {
    pub fn build(self) -> CloudtrailEventSelectorEl {
        CloudtrailEventSelectorEl {
            exclude_management_event_sources: core::default::Default::default(),
            include_management_events: core::default::Default::default(),
            read_write_type: core::default::Default::default(),
            data_resource: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudtrailEventSelectorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudtrailEventSelectorElRef {
    fn new(shared: StackShared, base: String) -> CloudtrailEventSelectorElRef {
        CloudtrailEventSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudtrailEventSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclude_management_event_sources` after provisioning.\n"]
    pub fn exclude_management_event_sources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exclude_management_event_sources", self.base))
    }

    #[doc = "Get a reference to the value of field `include_management_events` after provisioning.\n"]
    pub fn include_management_events(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_management_events", self.base))
    }

    #[doc = "Get a reference to the value of field `read_write_type` after provisioning.\n"]
    pub fn read_write_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_write_type", self.base))
    }

    #[doc = "Get a reference to the value of field `data_resource` after provisioning.\n"]
    pub fn data_resource(&self) -> ListRef<CloudtrailEventSelectorElDataResourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudtrailInsightSelectorEl {
    insight_type: PrimField<String>,
}

impl CloudtrailInsightSelectorEl { }

impl ToListMappable for CloudtrailInsightSelectorEl {
    type O = BlockAssignable<CloudtrailInsightSelectorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudtrailInsightSelectorEl {
    #[doc = ""]
    pub insight_type: PrimField<String>,
}

impl BuildCloudtrailInsightSelectorEl {
    pub fn build(self) -> CloudtrailInsightSelectorEl {
        CloudtrailInsightSelectorEl { insight_type: self.insight_type }
    }
}

pub struct CloudtrailInsightSelectorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudtrailInsightSelectorElRef {
    fn new(shared: StackShared, base: String) -> CloudtrailInsightSelectorElRef {
        CloudtrailInsightSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudtrailInsightSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `insight_type` after provisioning.\n"]
    pub fn insight_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.insight_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudtrailDynamic {
    advanced_event_selector: Option<DynamicBlock<CloudtrailAdvancedEventSelectorEl>>,
    event_selector: Option<DynamicBlock<CloudtrailEventSelectorEl>>,
    insight_selector: Option<DynamicBlock<CloudtrailInsightSelectorEl>>,
}
