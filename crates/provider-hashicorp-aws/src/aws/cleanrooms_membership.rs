use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CleanroomsMembershipData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    collaboration_id: PrimField<String>,
    query_log_status: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_result_configuration: Option<Vec<CleanroomsMembershipDefaultResultConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_configuration: Option<Vec<CleanroomsMembershipPaymentConfigurationEl>>,
    dynamic: CleanroomsMembershipDynamic,
}

struct CleanroomsMembership_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CleanroomsMembershipData>,
}

#[derive(Clone)]
pub struct CleanroomsMembership(Rc<CleanroomsMembership_>);

impl CleanroomsMembership {
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

    #[doc = "Set the field `default_result_configuration`.\n"]
    pub fn set_default_result_configuration(
        self,
        v: impl Into<BlockAssignable<CleanroomsMembershipDefaultResultConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_result_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .default_result_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `payment_configuration`.\n"]
    pub fn set_payment_configuration(
        self,
        v: impl Into<BlockAssignable<CleanroomsMembershipPaymentConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().payment_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.payment_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collaboration_arn` after provisioning.\n"]
    pub fn collaboration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaboration_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `collaboration_creator_account_id` after provisioning.\n"]
    pub fn collaboration_creator_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaboration_creator_account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `collaboration_creator_display_name` after provisioning.\n"]
    pub fn collaboration_creator_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaboration_creator_display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `collaboration_id` after provisioning.\n"]
    pub fn collaboration_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaboration_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `collaboration_name` after provisioning.\n"]
    pub fn collaboration_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaboration_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `member_abilities` after provisioning.\n"]
    pub fn member_abilities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.member_abilities", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `query_log_status` after provisioning.\n"]
    pub fn query_log_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_log_status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_result_configuration` after provisioning.\n"]
    pub fn default_result_configuration(
        &self,
    ) -> ListRef<CleanroomsMembershipDefaultResultConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_result_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `payment_configuration` after provisioning.\n"]
    pub fn payment_configuration(&self) -> ListRef<CleanroomsMembershipPaymentConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.payment_configuration", self.extract_ref()),
        )
    }
}

impl Referable for CleanroomsMembership {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CleanroomsMembership {}

impl ToListMappable for CleanroomsMembership {
    type O = ListRef<CleanroomsMembershipRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CleanroomsMembership_ {
    fn extract_resource_type(&self) -> String {
        "aws_cleanrooms_membership".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCleanroomsMembership {
    pub tf_id: String,
    #[doc = ""]
    pub collaboration_id: PrimField<String>,
    #[doc = ""]
    pub query_log_status: PrimField<String>,
}

impl BuildCleanroomsMembership {
    pub fn build(self, stack: &mut Stack) -> CleanroomsMembership {
        let out = CleanroomsMembership(Rc::new(CleanroomsMembership_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CleanroomsMembershipData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                collaboration_id: self.collaboration_id,
                query_log_status: self.query_log_status,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                default_result_configuration: core::default::Default::default(),
                payment_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CleanroomsMembershipRef {
    shared: StackShared,
    base: String,
}

impl Ref for CleanroomsMembershipRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CleanroomsMembershipRef {
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

    #[doc = "Get a reference to the value of field `collaboration_arn` after provisioning.\n"]
    pub fn collaboration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaboration_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `collaboration_creator_account_id` after provisioning.\n"]
    pub fn collaboration_creator_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaboration_creator_account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `collaboration_creator_display_name` after provisioning.\n"]
    pub fn collaboration_creator_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaboration_creator_display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `collaboration_id` after provisioning.\n"]
    pub fn collaboration_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaboration_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `collaboration_name` after provisioning.\n"]
    pub fn collaboration_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaboration_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `member_abilities` after provisioning.\n"]
    pub fn member_abilities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.member_abilities", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `query_log_status` after provisioning.\n"]
    pub fn query_log_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_log_status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_result_configuration` after provisioning.\n"]
    pub fn default_result_configuration(
        &self,
    ) -> ListRef<CleanroomsMembershipDefaultResultConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_result_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `payment_configuration` after provisioning.\n"]
    pub fn payment_configuration(&self) -> ListRef<CleanroomsMembershipPaymentConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.payment_configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElS3El {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_prefix: Option<PrimField<String>>,
    result_format: PrimField<String>,
}

impl CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElS3El {
    #[doc = "Set the field `key_prefix`.\n"]
    pub fn set_key_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElS3El {
    type O =
        BlockAssignable<CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElS3El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElS3El {
    #[doc = ""]
    pub bucket: PrimField<String>,
    #[doc = ""]
    pub result_format: PrimField<String>,
}

impl BuildCleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElS3El {
    pub fn build(
        self,
    ) -> CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElS3El {
        CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElS3El {
            bucket: self.bucket,
            key_prefix: core::default::Default::default(),
            result_format: self.result_format,
        }
    }
}

pub struct CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElS3ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElS3ElRef {
        CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `key_prefix` after provisioning.\n"]
    pub fn key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `result_format` after provisioning.\n"]
    pub fn result_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.result_format", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElDynamic {
    s3: Option<
        DynamicBlock<CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElS3El>,
    >,
}

#[derive(Serialize)]
pub struct CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElS3El>>,
    dynamic: CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElDynamic,
}

impl CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl Into<
            BlockAssignable<
                CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElS3El,
            >,
        >,
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

impl ToListMappable for CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationEl {
    type O = BlockAssignable<CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCleanroomsMembershipDefaultResultConfigurationElOutputConfigurationEl {}

impl BuildCleanroomsMembershipDefaultResultConfigurationElOutputConfigurationEl {
    pub fn build(self) -> CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationEl {
        CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationEl {
            s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElRef {
        CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(
        &self,
    ) -> ListRef<CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct CleanroomsMembershipDefaultResultConfigurationElDynamic {
    output_configuration:
        Option<DynamicBlock<CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationEl>>,
}

#[derive(Serialize)]
pub struct CleanroomsMembershipDefaultResultConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_configuration:
        Option<Vec<CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationEl>>,
    dynamic: CleanroomsMembershipDefaultResultConfigurationElDynamic,
}

impl CleanroomsMembershipDefaultResultConfigurationEl {
    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `output_configuration`.\n"]
    pub fn set_output_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CleanroomsMembershipDefaultResultConfigurationEl {
    type O = BlockAssignable<CleanroomsMembershipDefaultResultConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCleanroomsMembershipDefaultResultConfigurationEl {}

impl BuildCleanroomsMembershipDefaultResultConfigurationEl {
    pub fn build(self) -> CleanroomsMembershipDefaultResultConfigurationEl {
        CleanroomsMembershipDefaultResultConfigurationEl {
            role_arn: core::default::Default::default(),
            output_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CleanroomsMembershipDefaultResultConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CleanroomsMembershipDefaultResultConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CleanroomsMembershipDefaultResultConfigurationElRef {
        CleanroomsMembershipDefaultResultConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CleanroomsMembershipDefaultResultConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `output_configuration` after provisioning.\n"]
    pub fn output_configuration(
        &self,
    ) -> ListRef<CleanroomsMembershipDefaultResultConfigurationElOutputConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CleanroomsMembershipPaymentConfigurationElQueryComputeEl {
    is_responsible: PrimField<bool>,
}

impl CleanroomsMembershipPaymentConfigurationElQueryComputeEl {}

impl ToListMappable for CleanroomsMembershipPaymentConfigurationElQueryComputeEl {
    type O = BlockAssignable<CleanroomsMembershipPaymentConfigurationElQueryComputeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCleanroomsMembershipPaymentConfigurationElQueryComputeEl {
    #[doc = ""]
    pub is_responsible: PrimField<bool>,
}

impl BuildCleanroomsMembershipPaymentConfigurationElQueryComputeEl {
    pub fn build(self) -> CleanroomsMembershipPaymentConfigurationElQueryComputeEl {
        CleanroomsMembershipPaymentConfigurationElQueryComputeEl {
            is_responsible: self.is_responsible,
        }
    }
}

pub struct CleanroomsMembershipPaymentConfigurationElQueryComputeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CleanroomsMembershipPaymentConfigurationElQueryComputeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CleanroomsMembershipPaymentConfigurationElQueryComputeElRef {
        CleanroomsMembershipPaymentConfigurationElQueryComputeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CleanroomsMembershipPaymentConfigurationElQueryComputeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_responsible` after provisioning.\n"]
    pub fn is_responsible(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_responsible", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CleanroomsMembershipPaymentConfigurationElDynamic {
    query_compute: Option<DynamicBlock<CleanroomsMembershipPaymentConfigurationElQueryComputeEl>>,
}

#[derive(Serialize)]
pub struct CleanroomsMembershipPaymentConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    query_compute: Option<Vec<CleanroomsMembershipPaymentConfigurationElQueryComputeEl>>,
    dynamic: CleanroomsMembershipPaymentConfigurationElDynamic,
}

impl CleanroomsMembershipPaymentConfigurationEl {
    #[doc = "Set the field `query_compute`.\n"]
    pub fn set_query_compute(
        mut self,
        v: impl Into<BlockAssignable<CleanroomsMembershipPaymentConfigurationElQueryComputeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_compute = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_compute = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CleanroomsMembershipPaymentConfigurationEl {
    type O = BlockAssignable<CleanroomsMembershipPaymentConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCleanroomsMembershipPaymentConfigurationEl {}

impl BuildCleanroomsMembershipPaymentConfigurationEl {
    pub fn build(self) -> CleanroomsMembershipPaymentConfigurationEl {
        CleanroomsMembershipPaymentConfigurationEl {
            query_compute: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CleanroomsMembershipPaymentConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CleanroomsMembershipPaymentConfigurationElRef {
    fn new(shared: StackShared, base: String) -> CleanroomsMembershipPaymentConfigurationElRef {
        CleanroomsMembershipPaymentConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CleanroomsMembershipPaymentConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `query_compute` after provisioning.\n"]
    pub fn query_compute(
        &self,
    ) -> ListRef<CleanroomsMembershipPaymentConfigurationElQueryComputeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_compute", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CleanroomsMembershipDynamic {
    default_result_configuration:
        Option<DynamicBlock<CleanroomsMembershipDefaultResultConfigurationEl>>,
    payment_configuration: Option<DynamicBlock<CleanroomsMembershipPaymentConfigurationEl>>,
}
