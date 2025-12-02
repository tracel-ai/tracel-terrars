use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct WorkspaceswebSessionLoggerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_encryption_context: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_filter: Option<Vec<WorkspaceswebSessionLoggerEventFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_configuration: Option<Vec<WorkspaceswebSessionLoggerLogConfigurationEl>>,
    dynamic: WorkspaceswebSessionLoggerDynamic,
}
struct WorkspaceswebSessionLogger_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WorkspaceswebSessionLoggerData>,
}
#[derive(Clone)]
pub struct WorkspaceswebSessionLogger(Rc<WorkspaceswebSessionLogger_>);
impl WorkspaceswebSessionLogger {
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
    #[doc = "Set the field `additional_encryption_context`.\n"]
    pub fn set_additional_encryption_context(
        self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().additional_encryption_context = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_managed_key`.\n"]
    pub fn set_customer_managed_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_managed_key = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
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
    #[doc = "Set the field `event_filter`.\n"]
    pub fn set_event_filter(
        self,
        v: impl Into<BlockAssignable<WorkspaceswebSessionLoggerEventFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `log_configuration`.\n"]
    pub fn set_log_configuration(
        self,
        v: impl Into<BlockAssignable<WorkspaceswebSessionLoggerLogConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `additional_encryption_context` after provisioning.\n"]
    pub fn additional_encryption_context(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.additional_encryption_context", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `associated_portal_arns` after provisioning.\n"]
    pub fn associated_portal_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.associated_portal_arns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_managed_key` after provisioning.\n"]
    pub fn customer_managed_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_managed_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `session_logger_arn` after provisioning.\n"]
    pub fn session_logger_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_logger_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `event_filter` after provisioning.\n"]
    pub fn event_filter(&self) -> ListRef<WorkspaceswebSessionLoggerEventFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> ListRef<WorkspaceswebSessionLoggerLogConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_configuration", self.extract_ref()),
        )
    }
}
impl Referable for WorkspaceswebSessionLogger {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for WorkspaceswebSessionLogger {}
impl ToListMappable for WorkspaceswebSessionLogger {
    type O = ListRef<WorkspaceswebSessionLoggerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for WorkspaceswebSessionLogger_ {
    fn extract_resource_type(&self) -> String {
        "aws_workspacesweb_session_logger".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildWorkspaceswebSessionLogger {
    pub tf_id: String,
}
impl BuildWorkspaceswebSessionLogger {
    pub fn build(self, stack: &mut Stack) -> WorkspaceswebSessionLogger {
        let out = WorkspaceswebSessionLogger(Rc::new(WorkspaceswebSessionLogger_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WorkspaceswebSessionLoggerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                additional_encryption_context: core::default::Default::default(),
                customer_managed_key: core::default::Default::default(),
                display_name: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                event_filter: core::default::Default::default(),
                log_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct WorkspaceswebSessionLoggerRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkspaceswebSessionLoggerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl WorkspaceswebSessionLoggerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_encryption_context` after provisioning.\n"]
    pub fn additional_encryption_context(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.additional_encryption_context", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `associated_portal_arns` after provisioning.\n"]
    pub fn associated_portal_arns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.associated_portal_arns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_managed_key` after provisioning.\n"]
    pub fn customer_managed_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_managed_key", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `session_logger_arn` after provisioning.\n"]
    pub fn session_logger_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_logger_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `event_filter` after provisioning.\n"]
    pub fn event_filter(&self) -> ListRef<WorkspaceswebSessionLoggerEventFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.event_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> ListRef<WorkspaceswebSessionLoggerLogConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_configuration", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct WorkspaceswebSessionLoggerEventFilterElAllEl {}
impl WorkspaceswebSessionLoggerEventFilterElAllEl {}
impl ToListMappable for WorkspaceswebSessionLoggerEventFilterElAllEl {
    type O = BlockAssignable<WorkspaceswebSessionLoggerEventFilterElAllEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkspaceswebSessionLoggerEventFilterElAllEl {}
impl BuildWorkspaceswebSessionLoggerEventFilterElAllEl {
    pub fn build(self) -> WorkspaceswebSessionLoggerEventFilterElAllEl {
        WorkspaceswebSessionLoggerEventFilterElAllEl {}
    }
}
pub struct WorkspaceswebSessionLoggerEventFilterElAllElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkspaceswebSessionLoggerEventFilterElAllElRef {
    fn new(shared: StackShared, base: String) -> WorkspaceswebSessionLoggerEventFilterElAllElRef {
        WorkspaceswebSessionLoggerEventFilterElAllElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkspaceswebSessionLoggerEventFilterElAllElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct WorkspaceswebSessionLoggerEventFilterElDynamic {
    all: Option<DynamicBlock<WorkspaceswebSessionLoggerEventFilterElAllEl>>,
}
#[derive(Serialize)]
pub struct WorkspaceswebSessionLoggerEventFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<Vec<WorkspaceswebSessionLoggerEventFilterElAllEl>>,
    dynamic: WorkspaceswebSessionLoggerEventFilterElDynamic,
}
impl WorkspaceswebSessionLoggerEventFilterEl {
    #[doc = "Set the field `include`.\n"]
    pub fn set_include(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.include = Some(v.into());
        self
    }
    #[doc = "Set the field `all`.\n"]
    pub fn set_all(
        mut self,
        v: impl Into<BlockAssignable<WorkspaceswebSessionLoggerEventFilterElAllEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.all = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.all = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for WorkspaceswebSessionLoggerEventFilterEl {
    type O = BlockAssignable<WorkspaceswebSessionLoggerEventFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkspaceswebSessionLoggerEventFilterEl {}
impl BuildWorkspaceswebSessionLoggerEventFilterEl {
    pub fn build(self) -> WorkspaceswebSessionLoggerEventFilterEl {
        WorkspaceswebSessionLoggerEventFilterEl {
            include: core::default::Default::default(),
            all: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct WorkspaceswebSessionLoggerEventFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkspaceswebSessionLoggerEventFilterElRef {
    fn new(shared: StackShared, base: String) -> WorkspaceswebSessionLoggerEventFilterElRef {
        WorkspaceswebSessionLoggerEventFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkspaceswebSessionLoggerEventFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
    #[doc = "Get a reference to the value of field `all` after provisioning.\n"]
    pub fn all(&self) -> ListRef<WorkspaceswebSessionLoggerEventFilterElAllElRef> {
        ListRef::new(self.shared().clone(), format!("{}.all", self.base))
    }
}
#[derive(Serialize)]
pub struct WorkspaceswebSessionLoggerLogConfigurationElS3El {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_owner: Option<PrimField<String>>,
    folder_structure: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_prefix: Option<PrimField<String>>,
    log_file_format: PrimField<String>,
}
impl WorkspaceswebSessionLoggerLogConfigurationElS3El {
    #[doc = "Set the field `bucket_owner`.\n"]
    pub fn set_bucket_owner(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_owner = Some(v.into());
        self
    }
    #[doc = "Set the field `key_prefix`.\n"]
    pub fn set_key_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_prefix = Some(v.into());
        self
    }
}
impl ToListMappable for WorkspaceswebSessionLoggerLogConfigurationElS3El {
    type O = BlockAssignable<WorkspaceswebSessionLoggerLogConfigurationElS3El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkspaceswebSessionLoggerLogConfigurationElS3El {
    #[doc = ""]
    pub bucket: PrimField<String>,
    #[doc = ""]
    pub folder_structure: PrimField<String>,
    #[doc = ""]
    pub log_file_format: PrimField<String>,
}
impl BuildWorkspaceswebSessionLoggerLogConfigurationElS3El {
    pub fn build(self) -> WorkspaceswebSessionLoggerLogConfigurationElS3El {
        WorkspaceswebSessionLoggerLogConfigurationElS3El {
            bucket: self.bucket,
            bucket_owner: core::default::Default::default(),
            folder_structure: self.folder_structure,
            key_prefix: core::default::Default::default(),
            log_file_format: self.log_file_format,
        }
    }
}
pub struct WorkspaceswebSessionLoggerLogConfigurationElS3ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkspaceswebSessionLoggerLogConfigurationElS3ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> WorkspaceswebSessionLoggerLogConfigurationElS3ElRef {
        WorkspaceswebSessionLoggerLogConfigurationElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkspaceswebSessionLoggerLogConfigurationElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `bucket_owner` after provisioning.\n"]
    pub fn bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_owner", self.base))
    }
    #[doc = "Get a reference to the value of field `folder_structure` after provisioning.\n"]
    pub fn folder_structure(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.folder_structure", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_prefix` after provisioning.\n"]
    pub fn key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.base))
    }
    #[doc = "Get a reference to the value of field `log_file_format` after provisioning.\n"]
    pub fn log_file_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_file_format", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct WorkspaceswebSessionLoggerLogConfigurationElDynamic {
    s3: Option<DynamicBlock<WorkspaceswebSessionLoggerLogConfigurationElS3El>>,
}
#[derive(Serialize)]
pub struct WorkspaceswebSessionLoggerLogConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<WorkspaceswebSessionLoggerLogConfigurationElS3El>>,
    dynamic: WorkspaceswebSessionLoggerLogConfigurationElDynamic,
}
impl WorkspaceswebSessionLoggerLogConfigurationEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl Into<BlockAssignable<WorkspaceswebSessionLoggerLogConfigurationElS3El>>,
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
impl ToListMappable for WorkspaceswebSessionLoggerLogConfigurationEl {
    type O = BlockAssignable<WorkspaceswebSessionLoggerLogConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildWorkspaceswebSessionLoggerLogConfigurationEl {}
impl BuildWorkspaceswebSessionLoggerLogConfigurationEl {
    pub fn build(self) -> WorkspaceswebSessionLoggerLogConfigurationEl {
        WorkspaceswebSessionLoggerLogConfigurationEl {
            s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct WorkspaceswebSessionLoggerLogConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for WorkspaceswebSessionLoggerLogConfigurationElRef {
    fn new(shared: StackShared, base: String) -> WorkspaceswebSessionLoggerLogConfigurationElRef {
        WorkspaceswebSessionLoggerLogConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl WorkspaceswebSessionLoggerLogConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<WorkspaceswebSessionLoggerLogConfigurationElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}
#[derive(Serialize, Default)]
struct WorkspaceswebSessionLoggerDynamic {
    event_filter: Option<DynamicBlock<WorkspaceswebSessionLoggerEventFilterEl>>,
    log_configuration: Option<DynamicBlock<WorkspaceswebSessionLoggerLogConfigurationEl>>,
}
