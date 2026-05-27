use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SecretManagerRegionalSecretData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    secret_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_aliases: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_destroy_ttl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_encryption:
        Option<Vec<SecretManagerRegionalSecretCustomerManagedEncryptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation: Option<Vec<SecretManagerRegionalSecretRotationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SecretManagerRegionalSecretTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topics: Option<Vec<SecretManagerRegionalSecretTopicsEl>>,
    dynamic: SecretManagerRegionalSecretDynamic,
}
struct SecretManagerRegionalSecret_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecretManagerRegionalSecretData>,
}
#[derive(Clone)]
pub struct SecretManagerRegionalSecret(Rc<SecretManagerRegionalSecret_>);
impl SecretManagerRegionalSecret {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `annotations`.\nCustom metadata about the regional secret.\n\nAnnotations are distinct from various forms of labels. Annotations exist to allow\nclient tools to store their own state information without requiring a database.\n\nAnnotation keys must be between 1 and 63 characters long, have a UTF-8 encoding of\nmaximum 128 bytes, begin and end with an alphanumeric character ([a-z0-9A-Z]), and\nmay have dashes (-), underscores (_), dots (.), and alphanumerics in between these\nsymbols.\n\nThe total size of annotation keys and values must be less than 16KiB.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the regional secret. Defaults to false.\nWhen the field is set to true in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the federation will fail."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `expire_time`.\nTimestamp in UTC when the regional secret is scheduled to expire. This is always provided on\noutput, regardless of what was sent on input. A timestamp in RFC3339 UTC \"Zulu\" format, with\nnanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\". Only one of 'expire_time' or 'ttl' can be provided."]
    pub fn set_expire_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expire_time = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe labels assigned to this regional secret.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be assigned to a given resource.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}."]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\nThe TTL for the regional secret. A duration in seconds with up to nine fractional digits,\nterminated by 's'. Example: \"3.5s\". Only one of 'ttl' or 'expire_time' can be provided."]
    pub fn set_ttl(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `version_aliases`.\nMapping from version alias to version name.\n\nA version alias is a string with a maximum length of 63 characters and can contain\nuppercase and lowercase letters, numerals, and the hyphen (-) and underscore ('_')\ncharacters. An alias string must start with a letter and cannot be the string\n'latest' or 'NEW'. No more than 50 aliases can be assigned to a given secret.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_version_aliases(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().version_aliases = Some(v.into());
        self
    }
    #[doc = "Set the field `version_destroy_ttl`.\nSecret Version TTL after destruction request.\nThis is a part of the delayed delete feature on Secret Version.\nFor secret with versionDestroyTtl>0, version destruction doesn't happen immediately\non calling destroy instead the version goes to a disabled state and\nthe actual destruction happens after this TTL expires. It must be atleast 24h."]
    pub fn set_version_destroy_ttl(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version_destroy_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_managed_encryption`.\n"]
    pub fn set_customer_managed_encryption(
        self,
        v: impl Into<BlockAssignable<SecretManagerRegionalSecretCustomerManagedEncryptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().customer_managed_encryption = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.customer_managed_encryption = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rotation`.\n"]
    pub fn set_rotation(
        self,
        v: impl Into<BlockAssignable<SecretManagerRegionalSecretRotationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rotation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rotation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SecretManagerRegionalSecretTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `topics`.\n"]
    pub fn set_topics(
        self,
        v: impl Into<BlockAssignable<SecretManagerRegionalSecretTopicsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().topics = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.topics = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nCustom metadata about the regional secret.\n\nAnnotations are distinct from various forms of labels. Annotations exist to allow\nclient tools to store their own state information without requiring a database.\n\nAnnotation keys must be between 1 and 63 characters long, have a UTF-8 encoding of\nmaximum 128 bytes, begin and end with an alphanumeric character ([a-z0-9A-Z]), and\nmay have dashes (-), underscores (_), dots (.), and alphanumerics in between these\nsymbols.\n\nThe total size of annotation keys and values must be less than 16KiB.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the regional secret was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the regional secret. Defaults to false.\nWhen the field is set to true in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the federation will fail."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nTimestamp in UTC when the regional secret is scheduled to expire. This is always provided on\noutput, regardless of what was sent on input. A timestamp in RFC3339 UTC \"Zulu\" format, with\nnanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\". Only one of 'expire_time' or 'ttl' can be provided."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels assigned to this regional secret.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be assigned to a given resource.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the regional secret. eg us-central1"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the regional secret. Format:\n'projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\nThis must be unique within the project."]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}."]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\nThe TTL for the regional secret. A duration in seconds with up to nine fractional digits,\nterminated by 's'. Example: \"3.5s\". Only one of 'ttl' or 'expire_time' can be provided."]
    pub fn ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `version_aliases` after provisioning.\nMapping from version alias to version name.\n\nA version alias is a string with a maximum length of 63 characters and can contain\nuppercase and lowercase letters, numerals, and the hyphen (-) and underscore ('_')\ncharacters. An alias string must start with a letter and cannot be the string\n'latest' or 'NEW'. No more than 50 aliases can be assigned to a given secret.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn version_aliases(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.version_aliases", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_destroy_ttl` after provisioning.\nSecret Version TTL after destruction request.\nThis is a part of the delayed delete feature on Secret Version.\nFor secret with versionDestroyTtl>0, version destruction doesn't happen immediately\non calling destroy instead the version goes to a disabled state and\nthe actual destruction happens after this TTL expires. It must be atleast 24h."]
    pub fn version_destroy_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_destroy_ttl", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> ListRef<SecretManagerRegionalSecretCustomerManagedEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rotation` after provisioning.\n"]
    pub fn rotation(&self) -> ListRef<SecretManagerRegionalSecretRotationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rotation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecretManagerRegionalSecretTimeoutsElRef {
        SecretManagerRegionalSecretTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> ListRef<SecretManagerRegionalSecretTopicsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.topics", self.extract_ref()),
        )
    }
}
impl Referable for SecretManagerRegionalSecret {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SecretManagerRegionalSecret {}
impl ToListMappable for SecretManagerRegionalSecret {
    type O = ListRef<SecretManagerRegionalSecretRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SecretManagerRegionalSecret_ {
    fn extract_resource_type(&self) -> String {
        "google_secret_manager_regional_secret".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSecretManagerRegionalSecret {
    pub tf_id: String,
    #[doc = "The location of the regional secret. eg us-central1"]
    pub location: PrimField<String>,
    #[doc = "This must be unique within the project."]
    pub secret_id: PrimField<String>,
}
impl BuildSecretManagerRegionalSecret {
    pub fn build(self, stack: &mut Stack) -> SecretManagerRegionalSecret {
        let out = SecretManagerRegionalSecret(Rc::new(SecretManagerRegionalSecret_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecretManagerRegionalSecretData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                expire_time: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                secret_id: self.secret_id,
                tags: core::default::Default::default(),
                ttl: core::default::Default::default(),
                version_aliases: core::default::Default::default(),
                version_destroy_ttl: core::default::Default::default(),
                customer_managed_encryption: core::default::Default::default(),
                rotation: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                topics: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SecretManagerRegionalSecretRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerRegionalSecretRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SecretManagerRegionalSecretRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nCustom metadata about the regional secret.\n\nAnnotations are distinct from various forms of labels. Annotations exist to allow\nclient tools to store their own state information without requiring a database.\n\nAnnotation keys must be between 1 and 63 characters long, have a UTF-8 encoding of\nmaximum 128 bytes, begin and end with an alphanumeric character ([a-z0-9A-Z]), and\nmay have dashes (-), underscores (_), dots (.), and alphanumerics in between these\nsymbols.\n\nThe total size of annotation keys and values must be less than 16KiB.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the regional secret was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the regional secret. Defaults to false.\nWhen the field is set to true in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the federation will fail."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nTimestamp in UTC when the regional secret is scheduled to expire. This is always provided on\noutput, regardless of what was sent on input. A timestamp in RFC3339 UTC \"Zulu\" format, with\nnanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\". Only one of 'expire_time' or 'ttl' can be provided."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels assigned to this regional secret.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be assigned to a given resource.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the regional secret. eg us-central1"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the regional secret. Format:\n'projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\nThis must be unique within the project."]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}."]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\nThe TTL for the regional secret. A duration in seconds with up to nine fractional digits,\nterminated by 's'. Example: \"3.5s\". Only one of 'ttl' or 'expire_time' can be provided."]
    pub fn ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `version_aliases` after provisioning.\nMapping from version alias to version name.\n\nA version alias is a string with a maximum length of 63 characters and can contain\nuppercase and lowercase letters, numerals, and the hyphen (-) and underscore ('_')\ncharacters. An alias string must start with a letter and cannot be the string\n'latest' or 'NEW'. No more than 50 aliases can be assigned to a given secret.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn version_aliases(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.version_aliases", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_destroy_ttl` after provisioning.\nSecret Version TTL after destruction request.\nThis is a part of the delayed delete feature on Secret Version.\nFor secret with versionDestroyTtl>0, version destruction doesn't happen immediately\non calling destroy instead the version goes to a disabled state and\nthe actual destruction happens after this TTL expires. It must be atleast 24h."]
    pub fn version_destroy_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_destroy_ttl", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> ListRef<SecretManagerRegionalSecretCustomerManagedEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rotation` after provisioning.\n"]
    pub fn rotation(&self) -> ListRef<SecretManagerRegionalSecretRotationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rotation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecretManagerRegionalSecretTimeoutsElRef {
        SecretManagerRegionalSecretTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> ListRef<SecretManagerRegionalSecretTopicsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.topics", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SecretManagerRegionalSecretCustomerManagedEncryptionEl {
    kms_key_name: PrimField<String>,
}
impl SecretManagerRegionalSecretCustomerManagedEncryptionEl {}
impl ToListMappable for SecretManagerRegionalSecretCustomerManagedEncryptionEl {
    type O = BlockAssignable<SecretManagerRegionalSecretCustomerManagedEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecretManagerRegionalSecretCustomerManagedEncryptionEl {
    #[doc = "The resource name of the Cloud KMS CryptoKey used to encrypt secret payloads."]
    pub kms_key_name: PrimField<String>,
}
impl BuildSecretManagerRegionalSecretCustomerManagedEncryptionEl {
    pub fn build(self) -> SecretManagerRegionalSecretCustomerManagedEncryptionEl {
        SecretManagerRegionalSecretCustomerManagedEncryptionEl {
            kms_key_name: self.kms_key_name,
        }
    }
}
pub struct SecretManagerRegionalSecretCustomerManagedEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerRegionalSecretCustomerManagedEncryptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecretManagerRegionalSecretCustomerManagedEncryptionElRef {
        SecretManagerRegionalSecretCustomerManagedEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecretManagerRegionalSecretCustomerManagedEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe resource name of the Cloud KMS CryptoKey used to encrypt secret payloads."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct SecretManagerRegionalSecretRotationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    next_rotation_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation_period: Option<PrimField<String>>,
}
impl SecretManagerRegionalSecretRotationEl {
    #[doc = "Set the field `next_rotation_time`.\nTimestamp in UTC at which the Secret is scheduled to rotate.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_next_rotation_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_rotation_time = Some(v.into());
        self
    }
    #[doc = "Set the field `rotation_period`.\nThe Duration between rotation notifications. Must be in seconds and at least 3600s (1h)\nand at most 3153600000s (100 years). If rotationPeriod is set, 'next_rotation_time' must\nbe set. 'next_rotation_time' will be advanced by this period when the service\nautomatically sends rotation notifications."]
    pub fn set_rotation_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rotation_period = Some(v.into());
        self
    }
}
impl ToListMappable for SecretManagerRegionalSecretRotationEl {
    type O = BlockAssignable<SecretManagerRegionalSecretRotationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecretManagerRegionalSecretRotationEl {}
impl BuildSecretManagerRegionalSecretRotationEl {
    pub fn build(self) -> SecretManagerRegionalSecretRotationEl {
        SecretManagerRegionalSecretRotationEl {
            next_rotation_time: core::default::Default::default(),
            rotation_period: core::default::Default::default(),
        }
    }
}
pub struct SecretManagerRegionalSecretRotationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerRegionalSecretRotationElRef {
    fn new(shared: StackShared, base: String) -> SecretManagerRegionalSecretRotationElRef {
        SecretManagerRegionalSecretRotationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecretManagerRegionalSecretRotationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `next_rotation_time` after provisioning.\nTimestamp in UTC at which the Secret is scheduled to rotate.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn next_rotation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_rotation_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rotation_period` after provisioning.\nThe Duration between rotation notifications. Must be in seconds and at least 3600s (1h)\nand at most 3153600000s (100 years). If rotationPeriod is set, 'next_rotation_time' must\nbe set. 'next_rotation_time' will be advanced by this period when the service\nautomatically sends rotation notifications."]
    pub fn rotation_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rotation_period", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SecretManagerRegionalSecretTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SecretManagerRegionalSecretTimeoutsEl {
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
impl ToListMappable for SecretManagerRegionalSecretTimeoutsEl {
    type O = BlockAssignable<SecretManagerRegionalSecretTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecretManagerRegionalSecretTimeoutsEl {}
impl BuildSecretManagerRegionalSecretTimeoutsEl {
    pub fn build(self) -> SecretManagerRegionalSecretTimeoutsEl {
        SecretManagerRegionalSecretTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SecretManagerRegionalSecretTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerRegionalSecretTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SecretManagerRegionalSecretTimeoutsElRef {
        SecretManagerRegionalSecretTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecretManagerRegionalSecretTimeoutsElRef {
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
#[derive(Serialize)]
pub struct SecretManagerRegionalSecretTopicsEl {
    name: PrimField<String>,
}
impl SecretManagerRegionalSecretTopicsEl {}
impl ToListMappable for SecretManagerRegionalSecretTopicsEl {
    type O = BlockAssignable<SecretManagerRegionalSecretTopicsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecretManagerRegionalSecretTopicsEl {
    #[doc = "The resource name of the Pub/Sub topic that will be published to, in the following format:\nprojects/*/topics/*. For publication to succeed, the Secret Manager Service\nAgent service account must have pubsub.publisher permissions on the topic."]
    pub name: PrimField<String>,
}
impl BuildSecretManagerRegionalSecretTopicsEl {
    pub fn build(self) -> SecretManagerRegionalSecretTopicsEl {
        SecretManagerRegionalSecretTopicsEl { name: self.name }
    }
}
pub struct SecretManagerRegionalSecretTopicsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerRegionalSecretTopicsElRef {
    fn new(shared: StackShared, base: String) -> SecretManagerRegionalSecretTopicsElRef {
        SecretManagerRegionalSecretTopicsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecretManagerRegionalSecretTopicsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Pub/Sub topic that will be published to, in the following format:\nprojects/*/topics/*. For publication to succeed, the Secret Manager Service\nAgent service account must have pubsub.publisher permissions on the topic."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecretManagerRegionalSecretDynamic {
    customer_managed_encryption:
        Option<DynamicBlock<SecretManagerRegionalSecretCustomerManagedEncryptionEl>>,
    rotation: Option<DynamicBlock<SecretManagerRegionalSecretRotationEl>>,
    topics: Option<DynamicBlock<SecretManagerRegionalSecretTopicsEl>>,
}
