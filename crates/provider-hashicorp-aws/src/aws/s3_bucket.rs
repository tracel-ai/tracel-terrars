use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    acceleration_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    acl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_lock_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_payer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cors_rule: Option<Vec<S3BucketCorsRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grant: Option<Vec<S3BucketGrantEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_rule: Option<Vec<S3BucketLifecycleRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<Vec<S3BucketLoggingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_lock_configuration: Option<Vec<S3BucketObjectLockConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_configuration: Option<Vec<S3BucketReplicationConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_encryption_configuration: Option<Vec<S3BucketServerSideEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<S3BucketTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    versioning: Option<Vec<S3BucketVersioningEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    website: Option<Vec<S3BucketWebsiteEl>>,
    dynamic: S3BucketDynamic,
}

struct S3Bucket_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<S3BucketData>,
}

#[derive(Clone)]
pub struct S3Bucket(Rc<S3Bucket_>);

impl S3Bucket {
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

    #[doc = "Set the field `acceleration_status`.\n"]
    pub fn set_acceleration_status(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().acceleration_status = Some(v.into());
        self
    }

    #[doc = "Set the field `acl`.\n"]
    pub fn set_acl(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().acl = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bucket = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bucket_prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `object_lock_enabled`.\n"]
    pub fn set_object_lock_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().object_lock_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `request_payer`.\n"]
    pub fn set_request_payer(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().request_payer = Some(v.into());
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

    #[doc = "Set the field `cors_rule`.\n"]
    pub fn set_cors_rule(self, v: impl Into<BlockAssignable<S3BucketCorsRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cors_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cors_rule = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `grant`.\n"]
    pub fn set_grant(self, v: impl Into<BlockAssignable<S3BucketGrantEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().grant = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.grant = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lifecycle_rule`.\n"]
    pub fn set_lifecycle_rule(self, v: impl Into<BlockAssignable<S3BucketLifecycleRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lifecycle_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lifecycle_rule = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(self, v: impl Into<BlockAssignable<S3BucketLoggingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `object_lock_configuration`.\n"]
    pub fn set_object_lock_configuration(
        self,
        v: impl Into<BlockAssignable<S3BucketObjectLockConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().object_lock_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.object_lock_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `replication_configuration`.\n"]
    pub fn set_replication_configuration(
        self,
        v: impl Into<BlockAssignable<S3BucketReplicationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().replication_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.replication_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `server_side_encryption_configuration`.\n"]
    pub fn set_server_side_encryption_configuration(
        self,
        v: impl Into<BlockAssignable<S3BucketServerSideEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().server_side_encryption_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.server_side_encryption_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<S3BucketTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `versioning`.\n"]
    pub fn set_versioning(self, v: impl Into<BlockAssignable<S3BucketVersioningEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().versioning = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.versioning = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `website`.\n"]
    pub fn set_website(self, v: impl Into<BlockAssignable<S3BucketWebsiteEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().website = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.website = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `acceleration_status` after provisioning.\n"]
    pub fn acceleration_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.acceleration_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `acl` after provisioning.\n"]
    pub fn acl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.acl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_domain_name` after provisioning.\n"]
    pub fn bucket_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_region` after provisioning.\n"]
    pub fn bucket_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_regional_domain_name` after provisioning.\n"]
    pub fn bucket_regional_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_regional_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `object_lock_enabled` after provisioning.\n"]
    pub fn object_lock_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_lock_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_payer` after provisioning.\n"]
    pub fn request_payer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_payer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `website_domain` after provisioning.\n"]
    pub fn website_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.website_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `website_endpoint` after provisioning.\n"]
    pub fn website_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.website_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors_rule` after provisioning.\n"]
    pub fn cors_rule(&self) -> ListRef<S3BucketCorsRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_rule` after provisioning.\n"]
    pub fn lifecycle_rule(&self) -> ListRef<S3BucketLifecycleRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lifecycle_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<S3BucketLoggingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `object_lock_configuration` after provisioning.\n"]
    pub fn object_lock_configuration(&self) -> ListRef<S3BucketObjectLockConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.object_lock_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_configuration` after provisioning.\n"]
    pub fn replication_configuration(&self) -> ListRef<S3BucketReplicationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.replication_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(&self) -> ListRef<S3BucketServerSideEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.server_side_encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3BucketTimeoutsElRef {
        S3BucketTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `versioning` after provisioning.\n"]
    pub fn versioning(&self) -> ListRef<S3BucketVersioningElRef> {
        ListRef::new(self.shared().clone(), format!("{}.versioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `website` after provisioning.\n"]
    pub fn website(&self) -> ListRef<S3BucketWebsiteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.website", self.extract_ref()))
    }
}

impl Referable for S3Bucket {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for S3Bucket { }

impl ToListMappable for S3Bucket {
    type O = ListRef<S3BucketRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for S3Bucket_ {
    fn extract_resource_type(&self) -> String {
        "aws_s3_bucket".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3Bucket {
    pub tf_id: String,
}

impl BuildS3Bucket {
    pub fn build(self, stack: &mut Stack) -> S3Bucket {
        let out = S3Bucket(Rc::new(S3Bucket_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(S3BucketData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                acceleration_status: core::default::Default::default(),
                acl: core::default::Default::default(),
                bucket: core::default::Default::default(),
                bucket_prefix: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                object_lock_enabled: core::default::Default::default(),
                policy: core::default::Default::default(),
                region: core::default::Default::default(),
                request_payer: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                cors_rule: core::default::Default::default(),
                grant: core::default::Default::default(),
                lifecycle_rule: core::default::Default::default(),
                logging: core::default::Default::default(),
                object_lock_configuration: core::default::Default::default(),
                replication_configuration: core::default::Default::default(),
                server_side_encryption_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                versioning: core::default::Default::default(),
                website: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl S3BucketRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acceleration_status` after provisioning.\n"]
    pub fn acceleration_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.acceleration_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `acl` after provisioning.\n"]
    pub fn acl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.acl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_domain_name` after provisioning.\n"]
    pub fn bucket_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_region` after provisioning.\n"]
    pub fn bucket_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_regional_domain_name` after provisioning.\n"]
    pub fn bucket_regional_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_regional_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `object_lock_enabled` after provisioning.\n"]
    pub fn object_lock_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_lock_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_payer` after provisioning.\n"]
    pub fn request_payer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_payer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `website_domain` after provisioning.\n"]
    pub fn website_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.website_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `website_endpoint` after provisioning.\n"]
    pub fn website_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.website_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors_rule` after provisioning.\n"]
    pub fn cors_rule(&self) -> ListRef<S3BucketCorsRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_rule` after provisioning.\n"]
    pub fn lifecycle_rule(&self) -> ListRef<S3BucketLifecycleRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lifecycle_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<S3BucketLoggingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `object_lock_configuration` after provisioning.\n"]
    pub fn object_lock_configuration(&self) -> ListRef<S3BucketObjectLockConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.object_lock_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_configuration` after provisioning.\n"]
    pub fn replication_configuration(&self) -> ListRef<S3BucketReplicationConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.replication_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(&self) -> ListRef<S3BucketServerSideEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.server_side_encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3BucketTimeoutsElRef {
        S3BucketTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `versioning` after provisioning.\n"]
    pub fn versioning(&self) -> ListRef<S3BucketVersioningElRef> {
        ListRef::new(self.shared().clone(), format!("{}.versioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `website` after provisioning.\n"]
    pub fn website(&self) -> ListRef<S3BucketWebsiteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.website", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct S3BucketCorsRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_headers: Option<ListField<PrimField<String>>>,
    allowed_methods: ListField<PrimField<String>>,
    allowed_origins: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age_seconds: Option<PrimField<f64>>,
}

impl S3BucketCorsRuleEl {
    #[doc = "Set the field `allowed_headers`.\n"]
    pub fn set_allowed_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_headers = Some(v.into());
        self
    }

    #[doc = "Set the field `expose_headers`.\n"]
    pub fn set_expose_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }

    #[doc = "Set the field `max_age_seconds`.\n"]
    pub fn set_max_age_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age_seconds = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketCorsRuleEl {
    type O = BlockAssignable<S3BucketCorsRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketCorsRuleEl {
    #[doc = ""]
    pub allowed_methods: ListField<PrimField<String>>,
    #[doc = ""]
    pub allowed_origins: ListField<PrimField<String>>,
}

impl BuildS3BucketCorsRuleEl {
    pub fn build(self) -> S3BucketCorsRuleEl {
        S3BucketCorsRuleEl {
            allowed_headers: core::default::Default::default(),
            allowed_methods: self.allowed_methods,
            allowed_origins: self.allowed_origins,
            expose_headers: core::default::Default::default(),
            max_age_seconds: core::default::Default::default(),
        }
    }
}

pub struct S3BucketCorsRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketCorsRuleElRef {
    fn new(shared: StackShared, base: String) -> S3BucketCorsRuleElRef {
        S3BucketCorsRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketCorsRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_headers` after provisioning.\n"]
    pub fn allowed_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_methods` after provisioning.\n"]
    pub fn allowed_methods(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_origins` after provisioning.\n"]
    pub fn allowed_origins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_origins", self.base))
    }

    #[doc = "Get a reference to the value of field `expose_headers` after provisioning.\n"]
    pub fn expose_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `max_age_seconds` after provisioning.\n"]
    pub fn max_age_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketGrantEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    permissions: SetField<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}

impl S3BucketGrantEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketGrantEl {
    type O = BlockAssignable<S3BucketGrantEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketGrantEl {
    #[doc = ""]
    pub permissions: SetField<PrimField<String>>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildS3BucketGrantEl {
    pub fn build(self) -> S3BucketGrantEl {
        S3BucketGrantEl {
            id: core::default::Default::default(),
            permissions: self.permissions,
            type_: self.type_,
            uri: core::default::Default::default(),
        }
    }
}

pub struct S3BucketGrantElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketGrantElRef {
    fn new(shared: StackShared, base: String) -> S3BucketGrantElRef {
        S3BucketGrantElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketGrantElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketLifecycleRuleElExpirationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expired_object_delete_marker: Option<PrimField<bool>>,
}

impl S3BucketLifecycleRuleElExpirationEl {
    #[doc = "Set the field `date`.\n"]
    pub fn set_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.date = Some(v.into());
        self
    }

    #[doc = "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days = Some(v.into());
        self
    }

    #[doc = "Set the field `expired_object_delete_marker`.\n"]
    pub fn set_expired_object_delete_marker(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.expired_object_delete_marker = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketLifecycleRuleElExpirationEl {
    type O = BlockAssignable<S3BucketLifecycleRuleElExpirationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleRuleElExpirationEl {}

impl BuildS3BucketLifecycleRuleElExpirationEl {
    pub fn build(self) -> S3BucketLifecycleRuleElExpirationEl {
        S3BucketLifecycleRuleElExpirationEl {
            date: core::default::Default::default(),
            days: core::default::Default::default(),
            expired_object_delete_marker: core::default::Default::default(),
        }
    }
}

pub struct S3BucketLifecycleRuleElExpirationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketLifecycleRuleElExpirationElRef {
    fn new(shared: StackShared, base: String) -> S3BucketLifecycleRuleElExpirationElRef {
        S3BucketLifecycleRuleElExpirationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleRuleElExpirationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `date` after provisioning.\n"]
    pub fn date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date", self.base))
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }

    #[doc = "Get a reference to the value of field `expired_object_delete_marker` after provisioning.\n"]
    pub fn expired_object_delete_marker(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.expired_object_delete_marker", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketLifecycleRuleElNoncurrentVersionExpirationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<PrimField<f64>>,
}

impl S3BucketLifecycleRuleElNoncurrentVersionExpirationEl {
    #[doc = "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketLifecycleRuleElNoncurrentVersionExpirationEl {
    type O = BlockAssignable<S3BucketLifecycleRuleElNoncurrentVersionExpirationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleRuleElNoncurrentVersionExpirationEl {}

impl BuildS3BucketLifecycleRuleElNoncurrentVersionExpirationEl {
    pub fn build(self) -> S3BucketLifecycleRuleElNoncurrentVersionExpirationEl {
        S3BucketLifecycleRuleElNoncurrentVersionExpirationEl { days: core::default::Default::default() }
    }
}

pub struct S3BucketLifecycleRuleElNoncurrentVersionExpirationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketLifecycleRuleElNoncurrentVersionExpirationElRef {
    fn new(shared: StackShared, base: String) -> S3BucketLifecycleRuleElNoncurrentVersionExpirationElRef {
        S3BucketLifecycleRuleElNoncurrentVersionExpirationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleRuleElNoncurrentVersionExpirationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketLifecycleRuleElNoncurrentVersionTransitionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<PrimField<f64>>,
    storage_class: PrimField<String>,
}

impl S3BucketLifecycleRuleElNoncurrentVersionTransitionEl {
    #[doc = "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketLifecycleRuleElNoncurrentVersionTransitionEl {
    type O = BlockAssignable<S3BucketLifecycleRuleElNoncurrentVersionTransitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleRuleElNoncurrentVersionTransitionEl {
    #[doc = ""]
    pub storage_class: PrimField<String>,
}

impl BuildS3BucketLifecycleRuleElNoncurrentVersionTransitionEl {
    pub fn build(self) -> S3BucketLifecycleRuleElNoncurrentVersionTransitionEl {
        S3BucketLifecycleRuleElNoncurrentVersionTransitionEl {
            days: core::default::Default::default(),
            storage_class: self.storage_class,
        }
    }
}

pub struct S3BucketLifecycleRuleElNoncurrentVersionTransitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketLifecycleRuleElNoncurrentVersionTransitionElRef {
    fn new(shared: StackShared, base: String) -> S3BucketLifecycleRuleElNoncurrentVersionTransitionElRef {
        S3BucketLifecycleRuleElNoncurrentVersionTransitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleRuleElNoncurrentVersionTransitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketLifecycleRuleElTransitionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<PrimField<f64>>,
    storage_class: PrimField<String>,
}

impl S3BucketLifecycleRuleElTransitionEl {
    #[doc = "Set the field `date`.\n"]
    pub fn set_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.date = Some(v.into());
        self
    }

    #[doc = "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketLifecycleRuleElTransitionEl {
    type O = BlockAssignable<S3BucketLifecycleRuleElTransitionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleRuleElTransitionEl {
    #[doc = ""]
    pub storage_class: PrimField<String>,
}

impl BuildS3BucketLifecycleRuleElTransitionEl {
    pub fn build(self) -> S3BucketLifecycleRuleElTransitionEl {
        S3BucketLifecycleRuleElTransitionEl {
            date: core::default::Default::default(),
            days: core::default::Default::default(),
            storage_class: self.storage_class,
        }
    }
}

pub struct S3BucketLifecycleRuleElTransitionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketLifecycleRuleElTransitionElRef {
    fn new(shared: StackShared, base: String) -> S3BucketLifecycleRuleElTransitionElRef {
        S3BucketLifecycleRuleElTransitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleRuleElTransitionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `date` after provisioning.\n"]
    pub fn date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date", self.base))
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketLifecycleRuleElDynamic {
    expiration: Option<DynamicBlock<S3BucketLifecycleRuleElExpirationEl>>,
    noncurrent_version_expiration: Option<DynamicBlock<S3BucketLifecycleRuleElNoncurrentVersionExpirationEl>>,
    noncurrent_version_transition: Option<DynamicBlock<S3BucketLifecycleRuleElNoncurrentVersionTransitionEl>>,
    transition: Option<DynamicBlock<S3BucketLifecycleRuleElTransitionEl>>,
}

#[derive(Serialize)]
pub struct S3BucketLifecycleRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    abort_incomplete_multipart_upload_days: Option<PrimField<f64>>,
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration: Option<Vec<S3BucketLifecycleRuleElExpirationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    noncurrent_version_expiration: Option<Vec<S3BucketLifecycleRuleElNoncurrentVersionExpirationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    noncurrent_version_transition: Option<Vec<S3BucketLifecycleRuleElNoncurrentVersionTransitionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transition: Option<Vec<S3BucketLifecycleRuleElTransitionEl>>,
    dynamic: S3BucketLifecycleRuleElDynamic,
}

impl S3BucketLifecycleRuleEl {
    #[doc = "Set the field `abort_incomplete_multipart_upload_days`.\n"]
    pub fn set_abort_incomplete_multipart_upload_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.abort_incomplete_multipart_upload_days = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }

    #[doc = "Set the field `expiration`.\n"]
    pub fn set_expiration(mut self, v: impl Into<BlockAssignable<S3BucketLifecycleRuleElExpirationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.expiration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.expiration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `noncurrent_version_expiration`.\n"]
    pub fn set_noncurrent_version_expiration(
        mut self,
        v: impl Into<BlockAssignable<S3BucketLifecycleRuleElNoncurrentVersionExpirationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.noncurrent_version_expiration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.noncurrent_version_expiration = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `noncurrent_version_transition`.\n"]
    pub fn set_noncurrent_version_transition(
        mut self,
        v: impl Into<BlockAssignable<S3BucketLifecycleRuleElNoncurrentVersionTransitionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.noncurrent_version_transition = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.noncurrent_version_transition = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transition`.\n"]
    pub fn set_transition(mut self, v: impl Into<BlockAssignable<S3BucketLifecycleRuleElTransitionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.transition = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.transition = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3BucketLifecycleRuleEl {
    type O = BlockAssignable<S3BucketLifecycleRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLifecycleRuleEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildS3BucketLifecycleRuleEl {
    pub fn build(self) -> S3BucketLifecycleRuleEl {
        S3BucketLifecycleRuleEl {
            abort_incomplete_multipart_upload_days: core::default::Default::default(),
            enabled: self.enabled,
            id: core::default::Default::default(),
            prefix: core::default::Default::default(),
            tags: core::default::Default::default(),
            expiration: core::default::Default::default(),
            noncurrent_version_expiration: core::default::Default::default(),
            noncurrent_version_transition: core::default::Default::default(),
            transition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketLifecycleRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketLifecycleRuleElRef {
    fn new(shared: StackShared, base: String) -> S3BucketLifecycleRuleElRef {
        S3BucketLifecycleRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLifecycleRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `abort_incomplete_multipart_upload_days` after provisioning.\n"]
    pub fn abort_incomplete_multipart_upload_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.abort_incomplete_multipart_upload_days", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `expiration` after provisioning.\n"]
    pub fn expiration(&self) -> ListRef<S3BucketLifecycleRuleElExpirationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.expiration", self.base))
    }

    #[doc = "Get a reference to the value of field `noncurrent_version_expiration` after provisioning.\n"]
    pub fn noncurrent_version_expiration(&self) -> ListRef<S3BucketLifecycleRuleElNoncurrentVersionExpirationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.noncurrent_version_expiration", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketLoggingEl {
    target_bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_prefix: Option<PrimField<String>>,
}

impl S3BucketLoggingEl {
    #[doc = "Set the field `target_prefix`.\n"]
    pub fn set_target_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketLoggingEl {
    type O = BlockAssignable<S3BucketLoggingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketLoggingEl {
    #[doc = ""]
    pub target_bucket: PrimField<String>,
}

impl BuildS3BucketLoggingEl {
    pub fn build(self) -> S3BucketLoggingEl {
        S3BucketLoggingEl {
            target_bucket: self.target_bucket,
            target_prefix: core::default::Default::default(),
        }
    }
}

pub struct S3BucketLoggingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketLoggingElRef {
    fn new(shared: StackShared, base: String) -> S3BucketLoggingElRef {
        S3BucketLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketLoggingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_bucket` after provisioning.\n"]
    pub fn target_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `target_prefix` after provisioning.\n"]
    pub fn target_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketObjectLockConfigurationElRuleElDefaultRetentionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<PrimField<f64>>,
    mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    years: Option<PrimField<f64>>,
}

impl S3BucketObjectLockConfigurationElRuleElDefaultRetentionEl {
    #[doc = "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days = Some(v.into());
        self
    }

    #[doc = "Set the field `years`.\n"]
    pub fn set_years(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.years = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketObjectLockConfigurationElRuleElDefaultRetentionEl {
    type O = BlockAssignable<S3BucketObjectLockConfigurationElRuleElDefaultRetentionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketObjectLockConfigurationElRuleElDefaultRetentionEl {
    #[doc = ""]
    pub mode: PrimField<String>,
}

impl BuildS3BucketObjectLockConfigurationElRuleElDefaultRetentionEl {
    pub fn build(self) -> S3BucketObjectLockConfigurationElRuleElDefaultRetentionEl {
        S3BucketObjectLockConfigurationElRuleElDefaultRetentionEl {
            days: core::default::Default::default(),
            mode: self.mode,
            years: core::default::Default::default(),
        }
    }
}

pub struct S3BucketObjectLockConfigurationElRuleElDefaultRetentionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketObjectLockConfigurationElRuleElDefaultRetentionElRef {
    fn new(shared: StackShared, base: String) -> S3BucketObjectLockConfigurationElRuleElDefaultRetentionElRef {
        S3BucketObjectLockConfigurationElRuleElDefaultRetentionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketObjectLockConfigurationElRuleElDefaultRetentionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `years` after provisioning.\n"]
    pub fn years(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.years", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketObjectLockConfigurationElRuleElDynamic {
    default_retention: Option<DynamicBlock<S3BucketObjectLockConfigurationElRuleElDefaultRetentionEl>>,
}

#[derive(Serialize)]
pub struct S3BucketObjectLockConfigurationElRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_retention: Option<Vec<S3BucketObjectLockConfigurationElRuleElDefaultRetentionEl>>,
    dynamic: S3BucketObjectLockConfigurationElRuleElDynamic,
}

impl S3BucketObjectLockConfigurationElRuleEl {
    #[doc = "Set the field `default_retention`.\n"]
    pub fn set_default_retention(
        mut self,
        v: impl Into<BlockAssignable<S3BucketObjectLockConfigurationElRuleElDefaultRetentionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_retention = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_retention = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3BucketObjectLockConfigurationElRuleEl {
    type O = BlockAssignable<S3BucketObjectLockConfigurationElRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketObjectLockConfigurationElRuleEl {}

impl BuildS3BucketObjectLockConfigurationElRuleEl {
    pub fn build(self) -> S3BucketObjectLockConfigurationElRuleEl {
        S3BucketObjectLockConfigurationElRuleEl {
            default_retention: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketObjectLockConfigurationElRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketObjectLockConfigurationElRuleElRef {
    fn new(shared: StackShared, base: String) -> S3BucketObjectLockConfigurationElRuleElRef {
        S3BucketObjectLockConfigurationElRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketObjectLockConfigurationElRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_retention` after provisioning.\n"]
    pub fn default_retention(&self) -> ListRef<S3BucketObjectLockConfigurationElRuleElDefaultRetentionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_retention", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketObjectLockConfigurationElDynamic {
    rule: Option<DynamicBlock<S3BucketObjectLockConfigurationElRuleEl>>,
}

#[derive(Serialize)]
pub struct S3BucketObjectLockConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    object_lock_enabled: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<S3BucketObjectLockConfigurationElRuleEl>>,
    dynamic: S3BucketObjectLockConfigurationElDynamic,
}

impl S3BucketObjectLockConfigurationEl {
    #[doc = "Set the field `object_lock_enabled`.\n"]
    pub fn set_object_lock_enabled(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.object_lock_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl Into<BlockAssignable<S3BucketObjectLockConfigurationElRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rule = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3BucketObjectLockConfigurationEl {
    type O = BlockAssignable<S3BucketObjectLockConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketObjectLockConfigurationEl {}

impl BuildS3BucketObjectLockConfigurationEl {
    pub fn build(self) -> S3BucketObjectLockConfigurationEl {
        S3BucketObjectLockConfigurationEl {
            object_lock_enabled: core::default::Default::default(),
            rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketObjectLockConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketObjectLockConfigurationElRef {
    fn new(shared: StackShared, base: String) -> S3BucketObjectLockConfigurationElRef {
        S3BucketObjectLockConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketObjectLockConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object_lock_enabled` after provisioning.\n"]
    pub fn object_lock_enabled(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_lock_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<S3BucketObjectLockConfigurationElRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationElRulesElDestinationElAccessControlTranslationEl {
    owner: PrimField<String>,
}

impl S3BucketReplicationConfigurationElRulesElDestinationElAccessControlTranslationEl { }

impl ToListMappable for S3BucketReplicationConfigurationElRulesElDestinationElAccessControlTranslationEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationElRulesElDestinationElAccessControlTranslationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationElRulesElDestinationElAccessControlTranslationEl {
    #[doc = ""]
    pub owner: PrimField<String>,
}

impl BuildS3BucketReplicationConfigurationElRulesElDestinationElAccessControlTranslationEl {
    pub fn build(self) -> S3BucketReplicationConfigurationElRulesElDestinationElAccessControlTranslationEl {
        S3BucketReplicationConfigurationElRulesElDestinationElAccessControlTranslationEl { owner: self.owner }
    }
}

pub struct S3BucketReplicationConfigurationElRulesElDestinationElAccessControlTranslationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketReplicationConfigurationElRulesElDestinationElAccessControlTranslationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationElRulesElDestinationElAccessControlTranslationElRef {
        S3BucketReplicationConfigurationElRulesElDestinationElAccessControlTranslationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationElRulesElDestinationElAccessControlTranslationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationElRulesElDestinationElMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl S3BucketReplicationConfigurationElRulesElDestinationElMetricsEl {
    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketReplicationConfigurationElRulesElDestinationElMetricsEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationElRulesElDestinationElMetricsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationElRulesElDestinationElMetricsEl {}

impl BuildS3BucketReplicationConfigurationElRulesElDestinationElMetricsEl {
    pub fn build(self) -> S3BucketReplicationConfigurationElRulesElDestinationElMetricsEl {
        S3BucketReplicationConfigurationElRulesElDestinationElMetricsEl {
            minutes: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct S3BucketReplicationConfigurationElRulesElDestinationElMetricsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketReplicationConfigurationElRulesElDestinationElMetricsElRef {
    fn new(shared: StackShared, base: String) -> S3BucketReplicationConfigurationElRulesElDestinationElMetricsElRef {
        S3BucketReplicationConfigurationElRulesElDestinationElMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationElRulesElDestinationElMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationElRulesElDestinationElReplicationTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl S3BucketReplicationConfigurationElRulesElDestinationElReplicationTimeEl {
    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketReplicationConfigurationElRulesElDestinationElReplicationTimeEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationElRulesElDestinationElReplicationTimeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationElRulesElDestinationElReplicationTimeEl {}

impl BuildS3BucketReplicationConfigurationElRulesElDestinationElReplicationTimeEl {
    pub fn build(self) -> S3BucketReplicationConfigurationElRulesElDestinationElReplicationTimeEl {
        S3BucketReplicationConfigurationElRulesElDestinationElReplicationTimeEl {
            minutes: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct S3BucketReplicationConfigurationElRulesElDestinationElReplicationTimeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketReplicationConfigurationElRulesElDestinationElReplicationTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationElRulesElDestinationElReplicationTimeElRef {
        S3BucketReplicationConfigurationElRulesElDestinationElReplicationTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationElRulesElDestinationElReplicationTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationElRulesElDestinationElDynamic {
    access_control_translation: Option<
        DynamicBlock<S3BucketReplicationConfigurationElRulesElDestinationElAccessControlTranslationEl>,
    >,
    metrics: Option<DynamicBlock<S3BucketReplicationConfigurationElRulesElDestinationElMetricsEl>>,
    replication_time: Option<DynamicBlock<S3BucketReplicationConfigurationElRulesElDestinationElReplicationTimeEl>>,
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationElRulesElDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replica_kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_control_translation: Option<
        Vec<S3BucketReplicationConfigurationElRulesElDestinationElAccessControlTranslationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics: Option<Vec<S3BucketReplicationConfigurationElRulesElDestinationElMetricsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_time: Option<Vec<S3BucketReplicationConfigurationElRulesElDestinationElReplicationTimeEl>>,
    dynamic: S3BucketReplicationConfigurationElRulesElDestinationElDynamic,
}

impl S3BucketReplicationConfigurationElRulesElDestinationEl {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account_id = Some(v.into());
        self
    }

    #[doc = "Set the field `replica_kms_key_id`.\n"]
    pub fn set_replica_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.replica_kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `storage_class`.\n"]
    pub fn set_storage_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_class = Some(v.into());
        self
    }

    #[doc = "Set the field `access_control_translation`.\n"]
    pub fn set_access_control_translation(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3BucketReplicationConfigurationElRulesElDestinationElAccessControlTranslationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_control_translation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_control_translation = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metrics`.\n"]
    pub fn set_metrics(
        mut self,
        v: impl Into<BlockAssignable<S3BucketReplicationConfigurationElRulesElDestinationElMetricsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metrics = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metrics = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `replication_time`.\n"]
    pub fn set_replication_time(
        mut self,
        v: impl Into<BlockAssignable<S3BucketReplicationConfigurationElRulesElDestinationElReplicationTimeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.replication_time = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.replication_time = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3BucketReplicationConfigurationElRulesElDestinationEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationElRulesElDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationElRulesElDestinationEl {
    #[doc = ""]
    pub bucket: PrimField<String>,
}

impl BuildS3BucketReplicationConfigurationElRulesElDestinationEl {
    pub fn build(self) -> S3BucketReplicationConfigurationElRulesElDestinationEl {
        S3BucketReplicationConfigurationElRulesElDestinationEl {
            account_id: core::default::Default::default(),
            bucket: self.bucket,
            replica_kms_key_id: core::default::Default::default(),
            storage_class: core::default::Default::default(),
            access_control_translation: core::default::Default::default(),
            metrics: core::default::Default::default(),
            replication_time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketReplicationConfigurationElRulesElDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketReplicationConfigurationElRulesElDestinationElRef {
    fn new(shared: StackShared, base: String) -> S3BucketReplicationConfigurationElRulesElDestinationElRef {
        S3BucketReplicationConfigurationElRulesElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationElRulesElDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `replica_kms_key_id` after provisioning.\n"]
    pub fn replica_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.replica_kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_translation` after provisioning.\n"]
    pub fn access_control_translation(
        &self,
    ) -> ListRef<S3BucketReplicationConfigurationElRulesElDestinationElAccessControlTranslationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_control_translation", self.base))
    }

    #[doc = "Get a reference to the value of field `metrics` after provisioning.\n"]
    pub fn metrics(&self) -> ListRef<S3BucketReplicationConfigurationElRulesElDestinationElMetricsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `replication_time` after provisioning.\n"]
    pub fn replication_time(
        &self,
    ) -> ListRef<S3BucketReplicationConfigurationElRulesElDestinationElReplicationTimeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.replication_time", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationElRulesElFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

impl S3BucketReplicationConfigurationElRulesElFilterEl {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketReplicationConfigurationElRulesElFilterEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationElRulesElFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationElRulesElFilterEl {}

impl BuildS3BucketReplicationConfigurationElRulesElFilterEl {
    pub fn build(self) -> S3BucketReplicationConfigurationElRulesElFilterEl {
        S3BucketReplicationConfigurationElRulesElFilterEl {
            prefix: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct S3BucketReplicationConfigurationElRulesElFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketReplicationConfigurationElRulesElFilterElRef {
    fn new(shared: StackShared, base: String) -> S3BucketReplicationConfigurationElRulesElFilterElRef {
        S3BucketReplicationConfigurationElRulesElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationElRulesElFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl {
    enabled: PrimField<bool>,
}

impl S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl { }

impl ToListMappable for S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl {
    type O =
        BlockAssignable<S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildS3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl {
    pub fn build(self) -> S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl {
        S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl {
            enabled: self.enabled,
        }
    }
}

pub struct S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef {
        S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElDynamic {
    sse_kms_encrypted_objects: Option<
        DynamicBlock<S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl>,
    >,
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sse_kms_encrypted_objects: Option<
        Vec<S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl>,
    >,
    dynamic: S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElDynamic,
}

impl S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaEl {
    #[doc = "Set the field `sse_kms_encrypted_objects`.\n"]
    pub fn set_sse_kms_encrypted_objects(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElSseKmsEncryptedObjectsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sse_kms_encrypted_objects = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sse_kms_encrypted_objects = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaEl {}

impl BuildS3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaEl {
    pub fn build(self) -> S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaEl {
        S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaEl {
            sse_kms_encrypted_objects: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElRef {
        S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sse_kms_encrypted_objects` after provisioning.\n"]
    pub fn sse_kms_encrypted_objects(
        &self,
    ) -> ListRef<S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElSseKmsEncryptedObjectsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sse_kms_encrypted_objects", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationElRulesElDynamic {
    destination: Option<DynamicBlock<S3BucketReplicationConfigurationElRulesElDestinationEl>>,
    filter: Option<DynamicBlock<S3BucketReplicationConfigurationElRulesElFilterEl>>,
    source_selection_criteria: Option<
        DynamicBlock<S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaEl>,
    >,
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationElRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_marker_replication_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    status: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<Vec<S3BucketReplicationConfigurationElRulesElDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<S3BucketReplicationConfigurationElRulesElFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_selection_criteria: Option<Vec<S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaEl>>,
    dynamic: S3BucketReplicationConfigurationElRulesElDynamic,
}

impl S3BucketReplicationConfigurationElRulesEl {
    #[doc = "Set the field `delete_marker_replication_status`.\n"]
    pub fn set_delete_marker_replication_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete_marker_replication_status = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.priority = Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v: impl Into<BlockAssignable<S3BucketReplicationConfigurationElRulesElDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl Into<BlockAssignable<S3BucketReplicationConfigurationElRulesElFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_selection_criteria`.\n"]
    pub fn set_source_selection_criteria(
        mut self,
        v: impl Into<BlockAssignable<S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_selection_criteria = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_selection_criteria = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3BucketReplicationConfigurationElRulesEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationElRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationElRulesEl {
    #[doc = ""]
    pub status: PrimField<String>,
}

impl BuildS3BucketReplicationConfigurationElRulesEl {
    pub fn build(self) -> S3BucketReplicationConfigurationElRulesEl {
        S3BucketReplicationConfigurationElRulesEl {
            delete_marker_replication_status: core::default::Default::default(),
            id: core::default::Default::default(),
            prefix: core::default::Default::default(),
            priority: core::default::Default::default(),
            status: self.status,
            destination: core::default::Default::default(),
            filter: core::default::Default::default(),
            source_selection_criteria: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketReplicationConfigurationElRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketReplicationConfigurationElRulesElRef {
    fn new(shared: StackShared, base: String) -> S3BucketReplicationConfigurationElRulesElRef {
        S3BucketReplicationConfigurationElRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationElRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_marker_replication_status` after provisioning.\n"]
    pub fn delete_marker_replication_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_marker_replication_status", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> ListRef<S3BucketReplicationConfigurationElRulesElDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<S3BucketReplicationConfigurationElRulesElFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `source_selection_criteria` after provisioning.\n"]
    pub fn source_selection_criteria(
        &self,
    ) -> ListRef<S3BucketReplicationConfigurationElRulesElSourceSelectionCriteriaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_selection_criteria", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketReplicationConfigurationElDynamic {
    rules: Option<DynamicBlock<S3BucketReplicationConfigurationElRulesEl>>,
}

#[derive(Serialize)]
pub struct S3BucketReplicationConfigurationEl {
    role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<S3BucketReplicationConfigurationElRulesEl>>,
    dynamic: S3BucketReplicationConfigurationElDynamic,
}

impl S3BucketReplicationConfigurationEl {
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(mut self, v: impl Into<BlockAssignable<S3BucketReplicationConfigurationElRulesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rules = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3BucketReplicationConfigurationEl {
    type O = BlockAssignable<S3BucketReplicationConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketReplicationConfigurationEl {
    #[doc = ""]
    pub role: PrimField<String>,
}

impl BuildS3BucketReplicationConfigurationEl {
    pub fn build(self) -> S3BucketReplicationConfigurationEl {
        S3BucketReplicationConfigurationEl {
            role: self.role,
            rules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketReplicationConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketReplicationConfigurationElRef {
    fn new(shared: StackShared, base: String) -> S3BucketReplicationConfigurationElRef {
        S3BucketReplicationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketReplicationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketServerSideEncryptionConfigurationElRuleElApplyServerSideEncryptionByDefaultEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_master_key_id: Option<PrimField<String>>,
    sse_algorithm: PrimField<String>,
}

impl S3BucketServerSideEncryptionConfigurationElRuleElApplyServerSideEncryptionByDefaultEl {
    #[doc = "Set the field `kms_master_key_id`.\n"]
    pub fn set_kms_master_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_master_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketServerSideEncryptionConfigurationElRuleElApplyServerSideEncryptionByDefaultEl {
    type O = BlockAssignable<S3BucketServerSideEncryptionConfigurationElRuleElApplyServerSideEncryptionByDefaultEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketServerSideEncryptionConfigurationElRuleElApplyServerSideEncryptionByDefaultEl {
    #[doc = ""]
    pub sse_algorithm: PrimField<String>,
}

impl BuildS3BucketServerSideEncryptionConfigurationElRuleElApplyServerSideEncryptionByDefaultEl {
    pub fn build(self) -> S3BucketServerSideEncryptionConfigurationElRuleElApplyServerSideEncryptionByDefaultEl {
        S3BucketServerSideEncryptionConfigurationElRuleElApplyServerSideEncryptionByDefaultEl {
            kms_master_key_id: core::default::Default::default(),
            sse_algorithm: self.sse_algorithm,
        }
    }
}

pub struct S3BucketServerSideEncryptionConfigurationElRuleElApplyServerSideEncryptionByDefaultElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketServerSideEncryptionConfigurationElRuleElApplyServerSideEncryptionByDefaultElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> S3BucketServerSideEncryptionConfigurationElRuleElApplyServerSideEncryptionByDefaultElRef {
        S3BucketServerSideEncryptionConfigurationElRuleElApplyServerSideEncryptionByDefaultElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketServerSideEncryptionConfigurationElRuleElApplyServerSideEncryptionByDefaultElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_master_key_id` after provisioning.\n"]
    pub fn kms_master_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_master_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `sse_algorithm` after provisioning.\n"]
    pub fn sse_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sse_algorithm", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketServerSideEncryptionConfigurationElRuleElDynamic {
    apply_server_side_encryption_by_default: Option<
        DynamicBlock<S3BucketServerSideEncryptionConfigurationElRuleElApplyServerSideEncryptionByDefaultEl>,
    >,
}

#[derive(Serialize)]
pub struct S3BucketServerSideEncryptionConfigurationElRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_key_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apply_server_side_encryption_by_default: Option<
        Vec<S3BucketServerSideEncryptionConfigurationElRuleElApplyServerSideEncryptionByDefaultEl>,
    >,
    dynamic: S3BucketServerSideEncryptionConfigurationElRuleElDynamic,
}

impl S3BucketServerSideEncryptionConfigurationElRuleEl {
    #[doc = "Set the field `bucket_key_enabled`.\n"]
    pub fn set_bucket_key_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.bucket_key_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `apply_server_side_encryption_by_default`.\n"]
    pub fn set_apply_server_side_encryption_by_default(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            S3BucketServerSideEncryptionConfigurationElRuleElApplyServerSideEncryptionByDefaultEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.apply_server_side_encryption_by_default = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.apply_server_side_encryption_by_default = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3BucketServerSideEncryptionConfigurationElRuleEl {
    type O = BlockAssignable<S3BucketServerSideEncryptionConfigurationElRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketServerSideEncryptionConfigurationElRuleEl {}

impl BuildS3BucketServerSideEncryptionConfigurationElRuleEl {
    pub fn build(self) -> S3BucketServerSideEncryptionConfigurationElRuleEl {
        S3BucketServerSideEncryptionConfigurationElRuleEl {
            bucket_key_enabled: core::default::Default::default(),
            apply_server_side_encryption_by_default: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketServerSideEncryptionConfigurationElRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketServerSideEncryptionConfigurationElRuleElRef {
    fn new(shared: StackShared, base: String) -> S3BucketServerSideEncryptionConfigurationElRuleElRef {
        S3BucketServerSideEncryptionConfigurationElRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketServerSideEncryptionConfigurationElRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_key_enabled` after provisioning.\n"]
    pub fn bucket_key_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_key_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `apply_server_side_encryption_by_default` after provisioning.\n"]
    pub fn apply_server_side_encryption_by_default(
        &self,
    ) -> ListRef<S3BucketServerSideEncryptionConfigurationElRuleElApplyServerSideEncryptionByDefaultElRef> {
        ListRef::new(self.shared().clone(), format!("{}.apply_server_side_encryption_by_default", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketServerSideEncryptionConfigurationElDynamic {
    rule: Option<DynamicBlock<S3BucketServerSideEncryptionConfigurationElRuleEl>>,
}

#[derive(Serialize)]
pub struct S3BucketServerSideEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<S3BucketServerSideEncryptionConfigurationElRuleEl>>,
    dynamic: S3BucketServerSideEncryptionConfigurationElDynamic,
}

impl S3BucketServerSideEncryptionConfigurationEl {
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl Into<BlockAssignable<S3BucketServerSideEncryptionConfigurationElRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rule = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for S3BucketServerSideEncryptionConfigurationEl {
    type O = BlockAssignable<S3BucketServerSideEncryptionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketServerSideEncryptionConfigurationEl {}

impl BuildS3BucketServerSideEncryptionConfigurationEl {
    pub fn build(self) -> S3BucketServerSideEncryptionConfigurationEl {
        S3BucketServerSideEncryptionConfigurationEl {
            rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct S3BucketServerSideEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketServerSideEncryptionConfigurationElRef {
    fn new(shared: StackShared, base: String) -> S3BucketServerSideEncryptionConfigurationElRef {
        S3BucketServerSideEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketServerSideEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<S3BucketServerSideEncryptionConfigurationElRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl S3BucketTimeoutsEl {
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

    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketTimeoutsEl {
    type O = BlockAssignable<S3BucketTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketTimeoutsEl {}

impl BuildS3BucketTimeoutsEl {
    pub fn build(self) -> S3BucketTimeoutsEl {
        S3BucketTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            read: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct S3BucketTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> S3BucketTimeoutsElRef {
        S3BucketTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketTimeoutsElRef {
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

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketVersioningEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mfa_delete: Option<PrimField<bool>>,
}

impl S3BucketVersioningEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `mfa_delete`.\n"]
    pub fn set_mfa_delete(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.mfa_delete = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketVersioningEl {
    type O = BlockAssignable<S3BucketVersioningEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketVersioningEl {}

impl BuildS3BucketVersioningEl {
    pub fn build(self) -> S3BucketVersioningEl {
        S3BucketVersioningEl {
            enabled: core::default::Default::default(),
            mfa_delete: core::default::Default::default(),
        }
    }
}

pub struct S3BucketVersioningElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketVersioningElRef {
    fn new(shared: StackShared, base: String) -> S3BucketVersioningElRef {
        S3BucketVersioningElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketVersioningElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `mfa_delete` after provisioning.\n"]
    pub fn mfa_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mfa_delete", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketWebsiteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    error_document: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index_document: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_all_requests_to: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_rules: Option<PrimField<String>>,
}

impl S3BucketWebsiteEl {
    #[doc = "Set the field `error_document`.\n"]
    pub fn set_error_document(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error_document = Some(v.into());
        self
    }

    #[doc = "Set the field `index_document`.\n"]
    pub fn set_index_document(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.index_document = Some(v.into());
        self
    }

    #[doc = "Set the field `redirect_all_requests_to`.\n"]
    pub fn set_redirect_all_requests_to(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redirect_all_requests_to = Some(v.into());
        self
    }

    #[doc = "Set the field `routing_rules`.\n"]
    pub fn set_routing_rules(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.routing_rules = Some(v.into());
        self
    }
}

impl ToListMappable for S3BucketWebsiteEl {
    type O = BlockAssignable<S3BucketWebsiteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketWebsiteEl {}

impl BuildS3BucketWebsiteEl {
    pub fn build(self) -> S3BucketWebsiteEl {
        S3BucketWebsiteEl {
            error_document: core::default::Default::default(),
            index_document: core::default::Default::default(),
            redirect_all_requests_to: core::default::Default::default(),
            routing_rules: core::default::Default::default(),
        }
    }
}

pub struct S3BucketWebsiteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for S3BucketWebsiteElRef {
    fn new(shared: StackShared, base: String) -> S3BucketWebsiteElRef {
        S3BucketWebsiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketWebsiteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error_document` after provisioning.\n"]
    pub fn error_document(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_document", self.base))
    }

    #[doc = "Get a reference to the value of field `index_document` after provisioning.\n"]
    pub fn index_document(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index_document", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect_all_requests_to` after provisioning.\n"]
    pub fn redirect_all_requests_to(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_all_requests_to", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_rules` after provisioning.\n"]
    pub fn routing_rules(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.routing_rules", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketDynamic {
    cors_rule: Option<DynamicBlock<S3BucketCorsRuleEl>>,
    grant: Option<DynamicBlock<S3BucketGrantEl>>,
    lifecycle_rule: Option<DynamicBlock<S3BucketLifecycleRuleEl>>,
    logging: Option<DynamicBlock<S3BucketLoggingEl>>,
    object_lock_configuration: Option<DynamicBlock<S3BucketObjectLockConfigurationEl>>,
    replication_configuration: Option<DynamicBlock<S3BucketReplicationConfigurationEl>>,
    server_side_encryption_configuration: Option<DynamicBlock<S3BucketServerSideEncryptionConfigurationEl>>,
    versioning: Option<DynamicBlock<S3BucketVersioningEl>>,
    website: Option<DynamicBlock<S3BucketWebsiteEl>>,
}
