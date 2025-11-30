use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CloudwatchLogDeliveryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    delivery_destination_arn: PrimField<String>,
    delivery_source_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_fields: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_delivery_configuration: Option<ListField<CloudwatchLogDeliveryS3DeliveryConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct CloudwatchLogDelivery_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudwatchLogDeliveryData>,
}

#[derive(Clone)]
pub struct CloudwatchLogDelivery(Rc<CloudwatchLogDelivery_>);

impl CloudwatchLogDelivery {
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

    #[doc = "Set the field `field_delimiter`.\n"]
    pub fn set_field_delimiter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().field_delimiter = Some(v.into());
        self
    }

    #[doc = "Set the field `record_fields`.\n"]
    pub fn set_record_fields(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().record_fields = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_delivery_configuration`.\n"]
    pub fn set_s3_delivery_configuration(
        self,
        v: impl Into<ListField<CloudwatchLogDeliveryS3DeliveryConfigurationEl>>,
    ) -> Self {
        self.0.data.borrow_mut().s3_delivery_configuration = Some(v.into());
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

    #[doc = "Get a reference to the value of field `delivery_destination_arn` after provisioning.\n"]
    pub fn delivery_destination_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_destination_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delivery_source_name` after provisioning.\n"]
    pub fn delivery_source_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_source_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `field_delimiter` after provisioning.\n"]
    pub fn field_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.field_delimiter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `record_fields` after provisioning.\n"]
    pub fn record_fields(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.record_fields", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_delivery_configuration` after provisioning.\n"]
    pub fn s3_delivery_configuration(
        &self,
    ) -> ListRef<CloudwatchLogDeliveryS3DeliveryConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_delivery_configuration", self.extract_ref()),
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
}

impl Referable for CloudwatchLogDelivery {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CloudwatchLogDelivery {}

impl ToListMappable for CloudwatchLogDelivery {
    type O = ListRef<CloudwatchLogDeliveryRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudwatchLogDelivery_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudwatch_log_delivery".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudwatchLogDelivery {
    pub tf_id: String,
    #[doc = ""]
    pub delivery_destination_arn: PrimField<String>,
    #[doc = ""]
    pub delivery_source_name: PrimField<String>,
}

impl BuildCloudwatchLogDelivery {
    pub fn build(self, stack: &mut Stack) -> CloudwatchLogDelivery {
        let out = CloudwatchLogDelivery(Rc::new(CloudwatchLogDelivery_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudwatchLogDeliveryData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                delivery_destination_arn: self.delivery_destination_arn,
                delivery_source_name: self.delivery_source_name,
                field_delimiter: core::default::Default::default(),
                record_fields: core::default::Default::default(),
                region: core::default::Default::default(),
                s3_delivery_configuration: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudwatchLogDeliveryRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchLogDeliveryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CloudwatchLogDeliveryRef {
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

    #[doc = "Get a reference to the value of field `delivery_destination_arn` after provisioning.\n"]
    pub fn delivery_destination_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_destination_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delivery_source_name` after provisioning.\n"]
    pub fn delivery_source_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_source_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `field_delimiter` after provisioning.\n"]
    pub fn field_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.field_delimiter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `record_fields` after provisioning.\n"]
    pub fn record_fields(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.record_fields", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `s3_delivery_configuration` after provisioning.\n"]
    pub fn s3_delivery_configuration(
        &self,
    ) -> ListRef<CloudwatchLogDeliveryS3DeliveryConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_delivery_configuration", self.extract_ref()),
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
}

#[derive(Serialize)]
pub struct CloudwatchLogDeliveryS3DeliveryConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_hive_compatible_path: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix_path: Option<PrimField<String>>,
}

impl CloudwatchLogDeliveryS3DeliveryConfigurationEl {
    #[doc = "Set the field `enable_hive_compatible_path`.\n"]
    pub fn set_enable_hive_compatible_path(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_hive_compatible_path = Some(v.into());
        self
    }

    #[doc = "Set the field `suffix_path`.\n"]
    pub fn set_suffix_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix_path = Some(v.into());
        self
    }
}

impl ToListMappable for CloudwatchLogDeliveryS3DeliveryConfigurationEl {
    type O = BlockAssignable<CloudwatchLogDeliveryS3DeliveryConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogDeliveryS3DeliveryConfigurationEl {}

impl BuildCloudwatchLogDeliveryS3DeliveryConfigurationEl {
    pub fn build(self) -> CloudwatchLogDeliveryS3DeliveryConfigurationEl {
        CloudwatchLogDeliveryS3DeliveryConfigurationEl {
            enable_hive_compatible_path: core::default::Default::default(),
            suffix_path: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchLogDeliveryS3DeliveryConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudwatchLogDeliveryS3DeliveryConfigurationElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchLogDeliveryS3DeliveryConfigurationElRef {
        CloudwatchLogDeliveryS3DeliveryConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogDeliveryS3DeliveryConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_hive_compatible_path` after provisioning.\n"]
    pub fn enable_hive_compatible_path(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_hive_compatible_path", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `suffix_path` after provisioning.\n"]
    pub fn suffix_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix_path", self.base))
    }
}
