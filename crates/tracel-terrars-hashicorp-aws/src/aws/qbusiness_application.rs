use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct QbusinessApplicationData {
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
    display_name: PrimField<String>,
    iam_service_role_arn: PrimField<String>,
    identity_center_instance_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments_configuration: Option<Vec<QbusinessApplicationAttachmentsConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration: Option<Vec<QbusinessApplicationEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<QbusinessApplicationTimeoutsEl>,
    dynamic: QbusinessApplicationDynamic,
}

struct QbusinessApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<QbusinessApplicationData>,
}

#[derive(Clone)]
pub struct QbusinessApplication(Rc<QbusinessApplication_>);

impl QbusinessApplication {
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

    #[doc = "Set the field `description`.\nA description of the Amazon Q application."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `attachments_configuration`.\n"]
    pub fn set_attachments_configuration(
        self,
        v: impl Into<BlockAssignable<QbusinessApplicationAttachmentsConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attachments_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attachments_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl Into<BlockAssignable<QbusinessApplicationEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<QbusinessApplicationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the Amazon Q application."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Amazon Q application."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `iam_service_role_arn` after provisioning.\nThe Amazon Resource Name (ARN) of the IAM service role that provides permissions for the Amazon Q application."]
    pub fn iam_service_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_center_application_arn` after provisioning.\n"]
    pub fn identity_center_application_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_center_application_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identity_center_instance_arn` after provisioning.\nARN of the IAM Identity Center instance you are either creating for—or connecting to—your Amazon Q Business application"]
    pub fn identity_center_instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_center_instance_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attachments_configuration` after provisioning.\n"]
    pub fn attachments_configuration(&self) -> ListRef<QbusinessApplicationAttachmentsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.attachments_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> ListRef<QbusinessApplicationEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> QbusinessApplicationTimeoutsElRef {
        QbusinessApplicationTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for QbusinessApplication {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for QbusinessApplication { }

impl ToListMappable for QbusinessApplication {
    type O = ListRef<QbusinessApplicationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for QbusinessApplication_ {
    fn extract_resource_type(&self) -> String {
        "aws_qbusiness_application".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildQbusinessApplication {
    pub tf_id: String,
    #[doc = "The display name of the Amazon Q application."]
    pub display_name: PrimField<String>,
    #[doc =
        "The Amazon Resource Name (ARN) of the IAM service role that provides permissions for the Amazon Q application."]
    pub iam_service_role_arn: PrimField<String>,
    #[doc =
        "ARN of the IAM Identity Center instance you are either creating for—or connecting to—your Amazon Q Business application"]
    pub identity_center_instance_arn: PrimField<String>,
}

impl BuildQbusinessApplication {
    pub fn build(self, stack: &mut Stack) -> QbusinessApplication {
        let out = QbusinessApplication(Rc::new(QbusinessApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(QbusinessApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                display_name: self.display_name,
                iam_service_role_arn: self.iam_service_role_arn,
                identity_center_instance_arn: self.identity_center_instance_arn,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                attachments_configuration: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct QbusinessApplicationRef {
    shared: StackShared,
    base: String,
}

impl Ref for QbusinessApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl QbusinessApplicationRef {
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

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the Amazon Q application."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Amazon Q application."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `iam_service_role_arn` after provisioning.\nThe Amazon Resource Name (ARN) of the IAM service role that provides permissions for the Amazon Q application."]
    pub fn iam_service_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_center_application_arn` after provisioning.\n"]
    pub fn identity_center_application_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_center_application_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identity_center_instance_arn` after provisioning.\nARN of the IAM Identity Center instance you are either creating for—or connecting to—your Amazon Q Business application"]
    pub fn identity_center_instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_center_instance_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attachments_configuration` after provisioning.\n"]
    pub fn attachments_configuration(&self) -> ListRef<QbusinessApplicationAttachmentsConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.attachments_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> ListRef<QbusinessApplicationEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> QbusinessApplicationTimeoutsElRef {
        QbusinessApplicationTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct QbusinessApplicationAttachmentsConfigurationEl {
    attachments_control_mode: PrimField<String>,
}

impl QbusinessApplicationAttachmentsConfigurationEl { }

impl ToListMappable for QbusinessApplicationAttachmentsConfigurationEl {
    type O = BlockAssignable<QbusinessApplicationAttachmentsConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQbusinessApplicationAttachmentsConfigurationEl {
    #[doc = "Status information about whether file upload functionality is activated or deactivated for your end user."]
    pub attachments_control_mode: PrimField<String>,
}

impl BuildQbusinessApplicationAttachmentsConfigurationEl {
    pub fn build(self) -> QbusinessApplicationAttachmentsConfigurationEl {
        QbusinessApplicationAttachmentsConfigurationEl { attachments_control_mode: self.attachments_control_mode }
    }
}

pub struct QbusinessApplicationAttachmentsConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for QbusinessApplicationAttachmentsConfigurationElRef {
    fn new(shared: StackShared, base: String) -> QbusinessApplicationAttachmentsConfigurationElRef {
        QbusinessApplicationAttachmentsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QbusinessApplicationAttachmentsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attachments_control_mode` after provisioning.\nStatus information about whether file upload functionality is activated or deactivated for your end user."]
    pub fn attachments_control_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attachments_control_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct QbusinessApplicationEncryptionConfigurationEl {
    kms_key_id: PrimField<String>,
}

impl QbusinessApplicationEncryptionConfigurationEl { }

impl ToListMappable for QbusinessApplicationEncryptionConfigurationEl {
    type O = BlockAssignable<QbusinessApplicationEncryptionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQbusinessApplicationEncryptionConfigurationEl {
    #[doc =
        "The identifier of the AWS KMS key that is used to encrypt your data. Amazon Q doesn't support asymmetric keys."]
    pub kms_key_id: PrimField<String>,
}

impl BuildQbusinessApplicationEncryptionConfigurationEl {
    pub fn build(self) -> QbusinessApplicationEncryptionConfigurationEl {
        QbusinessApplicationEncryptionConfigurationEl { kms_key_id: self.kms_key_id }
    }
}

pub struct QbusinessApplicationEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for QbusinessApplicationEncryptionConfigurationElRef {
    fn new(shared: StackShared, base: String) -> QbusinessApplicationEncryptionConfigurationElRef {
        QbusinessApplicationEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QbusinessApplicationEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_id` after provisioning.\nThe identifier of the AWS KMS key that is used to encrypt your data. Amazon Q doesn't support asymmetric keys."]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct QbusinessApplicationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl QbusinessApplicationTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for QbusinessApplicationTimeoutsEl {
    type O = BlockAssignable<QbusinessApplicationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQbusinessApplicationTimeoutsEl {}

impl BuildQbusinessApplicationTimeoutsEl {
    pub fn build(self) -> QbusinessApplicationTimeoutsEl {
        QbusinessApplicationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct QbusinessApplicationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for QbusinessApplicationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> QbusinessApplicationTimeoutsElRef {
        QbusinessApplicationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QbusinessApplicationTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct QbusinessApplicationDynamic {
    attachments_configuration: Option<DynamicBlock<QbusinessApplicationAttachmentsConfigurationEl>>,
    encryption_configuration: Option<DynamicBlock<QbusinessApplicationEncryptionConfigurationEl>>,
}
