use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct IvschatLoggingConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_configuration: Option<Vec<IvschatLoggingConfigurationDestinationConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IvschatLoggingConfigurationTimeoutsEl>,
    dynamic: IvschatLoggingConfigurationDynamic,
}

struct IvschatLoggingConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IvschatLoggingConfigurationData>,
}

#[derive(Clone)]
pub struct IvschatLoggingConfiguration(Rc<IvschatLoggingConfiguration_>);

impl IvschatLoggingConfiguration {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
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

    #[doc = "Set the field `destination_configuration`.\n"]
    pub fn set_destination_configuration(
        self,
        v: impl Into<BlockAssignable<IvschatLoggingConfigurationDestinationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IvschatLoggingConfigurationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `destination_configuration` after provisioning.\n"]
    pub fn destination_configuration(
        &self,
    ) -> ListRef<IvschatLoggingConfigurationDestinationConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IvschatLoggingConfigurationTimeoutsElRef {
        IvschatLoggingConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for IvschatLoggingConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for IvschatLoggingConfiguration {}

impl ToListMappable for IvschatLoggingConfiguration {
    type O = ListRef<IvschatLoggingConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IvschatLoggingConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_ivschat_logging_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIvschatLoggingConfiguration {
    pub tf_id: String,
}

impl BuildIvschatLoggingConfiguration {
    pub fn build(self, stack: &mut Stack) -> IvschatLoggingConfiguration {
        let out = IvschatLoggingConfiguration(Rc::new(IvschatLoggingConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IvschatLoggingConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                destination_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IvschatLoggingConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for IvschatLoggingConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl IvschatLoggingConfigurationRef {
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

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `destination_configuration` after provisioning.\n"]
    pub fn destination_configuration(
        &self,
    ) -> ListRef<IvschatLoggingConfigurationDestinationConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IvschatLoggingConfigurationTimeoutsElRef {
        IvschatLoggingConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct IvschatLoggingConfigurationDestinationConfigurationElCloudwatchLogsEl {
    log_group_name: PrimField<String>,
}

impl IvschatLoggingConfigurationDestinationConfigurationElCloudwatchLogsEl {}

impl ToListMappable for IvschatLoggingConfigurationDestinationConfigurationElCloudwatchLogsEl {
    type O = BlockAssignable<IvschatLoggingConfigurationDestinationConfigurationElCloudwatchLogsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvschatLoggingConfigurationDestinationConfigurationElCloudwatchLogsEl {
    #[doc = ""]
    pub log_group_name: PrimField<String>,
}

impl BuildIvschatLoggingConfigurationDestinationConfigurationElCloudwatchLogsEl {
    pub fn build(self) -> IvschatLoggingConfigurationDestinationConfigurationElCloudwatchLogsEl {
        IvschatLoggingConfigurationDestinationConfigurationElCloudwatchLogsEl {
            log_group_name: self.log_group_name,
        }
    }
}

pub struct IvschatLoggingConfigurationDestinationConfigurationElCloudwatchLogsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IvschatLoggingConfigurationDestinationConfigurationElCloudwatchLogsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IvschatLoggingConfigurationDestinationConfigurationElCloudwatchLogsElRef {
        IvschatLoggingConfigurationDestinationConfigurationElCloudwatchLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvschatLoggingConfigurationDestinationConfigurationElCloudwatchLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_group_name", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct IvschatLoggingConfigurationDestinationConfigurationElFirehoseEl {
    delivery_stream_name: PrimField<String>,
}

impl IvschatLoggingConfigurationDestinationConfigurationElFirehoseEl {}

impl ToListMappable for IvschatLoggingConfigurationDestinationConfigurationElFirehoseEl {
    type O = BlockAssignable<IvschatLoggingConfigurationDestinationConfigurationElFirehoseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvschatLoggingConfigurationDestinationConfigurationElFirehoseEl {
    #[doc = ""]
    pub delivery_stream_name: PrimField<String>,
}

impl BuildIvschatLoggingConfigurationDestinationConfigurationElFirehoseEl {
    pub fn build(self) -> IvschatLoggingConfigurationDestinationConfigurationElFirehoseEl {
        IvschatLoggingConfigurationDestinationConfigurationElFirehoseEl {
            delivery_stream_name: self.delivery_stream_name,
        }
    }
}

pub struct IvschatLoggingConfigurationDestinationConfigurationElFirehoseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IvschatLoggingConfigurationDestinationConfigurationElFirehoseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IvschatLoggingConfigurationDestinationConfigurationElFirehoseElRef {
        IvschatLoggingConfigurationDestinationConfigurationElFirehoseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvschatLoggingConfigurationDestinationConfigurationElFirehoseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delivery_stream_name` after provisioning.\n"]
    pub fn delivery_stream_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_stream_name", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct IvschatLoggingConfigurationDestinationConfigurationElS3El {
    bucket_name: PrimField<String>,
}

impl IvschatLoggingConfigurationDestinationConfigurationElS3El {}

impl ToListMappable for IvschatLoggingConfigurationDestinationConfigurationElS3El {
    type O = BlockAssignable<IvschatLoggingConfigurationDestinationConfigurationElS3El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvschatLoggingConfigurationDestinationConfigurationElS3El {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
}

impl BuildIvschatLoggingConfigurationDestinationConfigurationElS3El {
    pub fn build(self) -> IvschatLoggingConfigurationDestinationConfigurationElS3El {
        IvschatLoggingConfigurationDestinationConfigurationElS3El {
            bucket_name: self.bucket_name,
        }
    }
}

pub struct IvschatLoggingConfigurationDestinationConfigurationElS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IvschatLoggingConfigurationDestinationConfigurationElS3ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IvschatLoggingConfigurationDestinationConfigurationElS3ElRef {
        IvschatLoggingConfigurationDestinationConfigurationElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvschatLoggingConfigurationDestinationConfigurationElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct IvschatLoggingConfigurationDestinationConfigurationElDynamic {
    cloudwatch_logs:
        Option<DynamicBlock<IvschatLoggingConfigurationDestinationConfigurationElCloudwatchLogsEl>>,
    firehose: Option<DynamicBlock<IvschatLoggingConfigurationDestinationConfigurationElFirehoseEl>>,
    s3: Option<DynamicBlock<IvschatLoggingConfigurationDestinationConfigurationElS3El>>,
}

#[derive(Serialize)]
pub struct IvschatLoggingConfigurationDestinationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logs:
        Option<Vec<IvschatLoggingConfigurationDestinationConfigurationElCloudwatchLogsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firehose: Option<Vec<IvschatLoggingConfigurationDestinationConfigurationElFirehoseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<IvschatLoggingConfigurationDestinationConfigurationElS3El>>,
    dynamic: IvschatLoggingConfigurationDestinationConfigurationElDynamic,
}

impl IvschatLoggingConfigurationDestinationConfigurationEl {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v: impl Into<
            BlockAssignable<IvschatLoggingConfigurationDestinationConfigurationElCloudwatchLogsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `firehose`.\n"]
    pub fn set_firehose(
        mut self,
        v: impl Into<BlockAssignable<IvschatLoggingConfigurationDestinationConfigurationElFirehoseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.firehose = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.firehose = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl Into<BlockAssignable<IvschatLoggingConfigurationDestinationConfigurationElS3El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for IvschatLoggingConfigurationDestinationConfigurationEl {
    type O = BlockAssignable<IvschatLoggingConfigurationDestinationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvschatLoggingConfigurationDestinationConfigurationEl {}

impl BuildIvschatLoggingConfigurationDestinationConfigurationEl {
    pub fn build(self) -> IvschatLoggingConfigurationDestinationConfigurationEl {
        IvschatLoggingConfigurationDestinationConfigurationEl {
            cloudwatch_logs: core::default::Default::default(),
            firehose: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct IvschatLoggingConfigurationDestinationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IvschatLoggingConfigurationDestinationConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> IvschatLoggingConfigurationDestinationConfigurationElRef {
        IvschatLoggingConfigurationDestinationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvschatLoggingConfigurationDestinationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(
        &self,
    ) -> ListRef<IvschatLoggingConfigurationDestinationConfigurationElCloudwatchLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_logs", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `firehose` after provisioning.\n"]
    pub fn firehose(
        &self,
    ) -> ListRef<IvschatLoggingConfigurationDestinationConfigurationElFirehoseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<IvschatLoggingConfigurationDestinationConfigurationElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize)]
pub struct IvschatLoggingConfigurationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl IvschatLoggingConfigurationTimeoutsEl {
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

impl ToListMappable for IvschatLoggingConfigurationTimeoutsEl {
    type O = BlockAssignable<IvschatLoggingConfigurationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIvschatLoggingConfigurationTimeoutsEl {}

impl BuildIvschatLoggingConfigurationTimeoutsEl {
    pub fn build(self) -> IvschatLoggingConfigurationTimeoutsEl {
        IvschatLoggingConfigurationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct IvschatLoggingConfigurationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IvschatLoggingConfigurationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IvschatLoggingConfigurationTimeoutsElRef {
        IvschatLoggingConfigurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IvschatLoggingConfigurationTimeoutsElRef {
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
struct IvschatLoggingConfigurationDynamic {
    destination_configuration:
        Option<DynamicBlock<IvschatLoggingConfigurationDestinationConfigurationEl>>,
}
