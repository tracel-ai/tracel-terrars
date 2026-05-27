use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SecretManagerSecretData {
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
    replication: Option<Vec<SecretManagerSecretReplicationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation: Option<Vec<SecretManagerSecretRotationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SecretManagerSecretTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topics: Option<Vec<SecretManagerSecretTopicsEl>>,
    dynamic: SecretManagerSecretDynamic,
}
struct SecretManagerSecret_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecretManagerSecretData>,
}
#[derive(Clone)]
pub struct SecretManagerSecret(Rc<SecretManagerSecret_>);
impl SecretManagerSecret {
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
    #[doc = "Set the field `annotations`.\nCustom metadata about the secret.\n\nAnnotations are distinct from various forms of labels. Annotations exist to allow\nclient tools to store their own state information without requiring a database.\n\nAnnotation keys must be between 1 and 63 characters long, have a UTF-8 encoding of\nmaximum 128 bytes, begin and end with an alphanumeric character ([a-z0-9A-Z]), and\nmay have dashes (-), underscores (_), dots (.), and alphanumerics in between these\nsymbols.\n\nThe total size of annotation keys and values must be less than 16KiB.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the secret. Defaults to false.\nWhen the field is set to true in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the secret will fail."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `expire_time`.\nTimestamp in UTC when the Secret is scheduled to expire. This is always provided on output, regardless of what was sent on input.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\".\nOnly one of 'expire_time' or 'ttl' can be provided."]
    pub fn set_expire_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expire_time = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe labels assigned to this Secret.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be assigned to a given resource.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Set the field `ttl`.\nThe TTL for the Secret.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\".\nOnly one of 'ttl' or 'expire_time' can be provided."]
    pub fn set_ttl(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `version_aliases`.\nMapping from version alias to version name.\n\nA version alias is a string with a maximum length of 63 characters and can contain\nuppercase and lowercase letters, numerals, and the hyphen (-) and underscore ('_')\ncharacters. An alias string must start with a letter and cannot be the string\n'latest' or 'NEW'. No more than 50 aliases can be assigned to a given secret.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_version_aliases(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().version_aliases = Some(v.into());
        self
    }
    #[doc = "Set the field `version_destroy_ttl`.\nSecret Version TTL after destruction request.\nThis is a part of the delayed delete feature on Secret Version.\nFor secret with versionDestroyTtl>0, version destruction doesn't happen immediately\non calling destroy instead the version goes to a disabled state and\nthe actual destruction happens after this TTL expires."]
    pub fn set_version_destroy_ttl(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version_destroy_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `replication`.\n"]
    pub fn set_replication(
        self,
        v: impl Into<BlockAssignable<SecretManagerSecretReplicationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().replication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.replication = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rotation`.\n"]
    pub fn set_rotation(
        self,
        v: impl Into<BlockAssignable<SecretManagerSecretRotationEl>>,
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
    pub fn set_timeouts(self, v: impl Into<SecretManagerSecretTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `topics`.\n"]
    pub fn set_topics(self, v: impl Into<BlockAssignable<SecretManagerSecretTopicsEl>>) -> Self {
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
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nCustom metadata about the secret.\n\nAnnotations are distinct from various forms of labels. Annotations exist to allow\nclient tools to store their own state information without requiring a database.\n\nAnnotation keys must be between 1 and 63 characters long, have a UTF-8 encoding of\nmaximum 128 bytes, begin and end with an alphanumeric character ([a-z0-9A-Z]), and\nmay have dashes (-), underscores (_), dots (.), and alphanumerics in between these\nsymbols.\n\nThe total size of annotation keys and values must be less than 16KiB.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the Secret was created."]
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
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the secret. Defaults to false.\nWhen the field is set to true in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the secret will fail."]
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
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nTimestamp in UTC when the Secret is scheduled to expire. This is always provided on output, regardless of what was sent on input.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\".\nOnly one of 'expire_time' or 'ttl' can be provided."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels assigned to this Secret.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be assigned to a given resource.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Secret. Format:\n'projects/{{project}}/secrets/{{secret_id}}'"]
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
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\nThe TTL for the Secret.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\".\nOnly one of 'ttl' or 'expire_time' can be provided."]
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
    #[doc = "Get a reference to the value of field `version_destroy_ttl` after provisioning.\nSecret Version TTL after destruction request.\nThis is a part of the delayed delete feature on Secret Version.\nFor secret with versionDestroyTtl>0, version destruction doesn't happen immediately\non calling destroy instead the version goes to a disabled state and\nthe actual destruction happens after this TTL expires."]
    pub fn version_destroy_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_destroy_ttl", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication` after provisioning.\n"]
    pub fn replication(&self) -> ListRef<SecretManagerSecretReplicationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rotation` after provisioning.\n"]
    pub fn rotation(&self) -> ListRef<SecretManagerSecretRotationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rotation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecretManagerSecretTimeoutsElRef {
        SecretManagerSecretTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> ListRef<SecretManagerSecretTopicsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.topics", self.extract_ref()),
        )
    }
}
impl Referable for SecretManagerSecret {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SecretManagerSecret {}
impl ToListMappable for SecretManagerSecret {
    type O = ListRef<SecretManagerSecretRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SecretManagerSecret_ {
    fn extract_resource_type(&self) -> String {
        "google_secret_manager_secret".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSecretManagerSecret {
    pub tf_id: String,
    #[doc = "This must be unique within the project."]
    pub secret_id: PrimField<String>,
}
impl BuildSecretManagerSecret {
    pub fn build(self, stack: &mut Stack) -> SecretManagerSecret {
        let out = SecretManagerSecret(Rc::new(SecretManagerSecret_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecretManagerSecretData {
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
                project: core::default::Default::default(),
                secret_id: self.secret_id,
                tags: core::default::Default::default(),
                ttl: core::default::Default::default(),
                version_aliases: core::default::Default::default(),
                version_destroy_ttl: core::default::Default::default(),
                replication: core::default::Default::default(),
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
pub struct SecretManagerSecretRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerSecretRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SecretManagerSecretRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nCustom metadata about the secret.\n\nAnnotations are distinct from various forms of labels. Annotations exist to allow\nclient tools to store their own state information without requiring a database.\n\nAnnotation keys must be between 1 and 63 characters long, have a UTF-8 encoding of\nmaximum 128 bytes, begin and end with an alphanumeric character ([a-z0-9A-Z]), and\nmay have dashes (-), underscores (_), dots (.), and alphanumerics in between these\nsymbols.\n\nThe total size of annotation keys and values must be less than 16KiB.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the Secret was created."]
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
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the secret. Defaults to false.\nWhen the field is set to true in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the secret will fail."]
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
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nTimestamp in UTC when the Secret is scheduled to expire. This is always provided on output, regardless of what was sent on input.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\".\nOnly one of 'expire_time' or 'ttl' can be provided."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels assigned to this Secret.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be assigned to a given resource.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Secret. Format:\n'projects/{{project}}/secrets/{{secret_id}}'"]
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
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\nThe TTL for the Secret.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\".\nOnly one of 'ttl' or 'expire_time' can be provided."]
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
    #[doc = "Get a reference to the value of field `version_destroy_ttl` after provisioning.\nSecret Version TTL after destruction request.\nThis is a part of the delayed delete feature on Secret Version.\nFor secret with versionDestroyTtl>0, version destruction doesn't happen immediately\non calling destroy instead the version goes to a disabled state and\nthe actual destruction happens after this TTL expires."]
    pub fn version_destroy_ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_destroy_ttl", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `replication` after provisioning.\n"]
    pub fn replication(&self) -> ListRef<SecretManagerSecretReplicationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rotation` after provisioning.\n"]
    pub fn rotation(&self) -> ListRef<SecretManagerSecretRotationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rotation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecretManagerSecretTimeoutsElRef {
        SecretManagerSecretTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> ListRef<SecretManagerSecretTopicsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.topics", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
    kms_key_name: PrimField<String>,
}
impl SecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {}
impl ToListMappable for SecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
    type O = BlockAssignable<SecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
    #[doc = "The resource name of the Cloud KMS CryptoKey used to encrypt secret payloads."]
    pub kms_key_name: PrimField<String>,
}
impl BuildSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
    pub fn build(self) -> SecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
        SecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
            kms_key_name: self.kms_key_name,
        }
    }
}
pub struct SecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef {
        SecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe resource name of the Cloud KMS CryptoKey used to encrypt secret payloads."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecretManagerSecretReplicationElAutoElDynamic {
    customer_managed_encryption:
        Option<DynamicBlock<SecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl>>,
}
#[derive(Serialize)]
pub struct SecretManagerSecretReplicationElAutoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_encryption:
        Option<Vec<SecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl>>,
    dynamic: SecretManagerSecretReplicationElAutoElDynamic,
}
impl SecretManagerSecretReplicationElAutoEl {
    #[doc = "Set the field `customer_managed_encryption`.\n"]
    pub fn set_customer_managed_encryption(
        mut self,
        v: impl Into<BlockAssignable<SecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.customer_managed_encryption = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.customer_managed_encryption = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecretManagerSecretReplicationElAutoEl {
    type O = BlockAssignable<SecretManagerSecretReplicationElAutoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecretManagerSecretReplicationElAutoEl {}
impl BuildSecretManagerSecretReplicationElAutoEl {
    pub fn build(self) -> SecretManagerSecretReplicationElAutoEl {
        SecretManagerSecretReplicationElAutoEl {
            customer_managed_encryption: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecretManagerSecretReplicationElAutoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerSecretReplicationElAutoElRef {
    fn new(shared: StackShared, base: String) -> SecretManagerSecretReplicationElAutoElRef {
        SecretManagerSecretReplicationElAutoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecretManagerSecretReplicationElAutoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> ListRef<SecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
    kms_key_name: PrimField<String>,
}
impl SecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {}
impl ToListMappable
    for SecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl
{
    type O = BlockAssignable<
        SecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
    #[doc = "Describes the Cloud KMS encryption key that will be used to protect destination secret."]
    pub kms_key_name: PrimField<String>,
}
impl BuildSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
    pub fn build(
        self,
    ) -> SecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
        SecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
            kms_key_name: self.kms_key_name,
        }
    }
}
pub struct SecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef {
        SecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nDescribes the Cloud KMS encryption key that will be used to protect destination secret."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecretManagerSecretReplicationElUserManagedElReplicasElDynamic {
    customer_managed_encryption: Option<
        DynamicBlock<
            SecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct SecretManagerSecretReplicationElUserManagedElReplicasEl {
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_encryption: Option<
        Vec<SecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl>,
    >,
    dynamic: SecretManagerSecretReplicationElUserManagedElReplicasElDynamic,
}
impl SecretManagerSecretReplicationElUserManagedElReplicasEl {
    #[doc = "Set the field `customer_managed_encryption`.\n"]
    pub fn set_customer_managed_encryption(
        mut self,
        v: impl Into<
            BlockAssignable<
                SecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.customer_managed_encryption = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.customer_managed_encryption = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecretManagerSecretReplicationElUserManagedElReplicasEl {
    type O = BlockAssignable<SecretManagerSecretReplicationElUserManagedElReplicasEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecretManagerSecretReplicationElUserManagedElReplicasEl {
    #[doc = "The canonical IDs of the location to replicate data. For example: \"us-east1\"."]
    pub location: PrimField<String>,
}
impl BuildSecretManagerSecretReplicationElUserManagedElReplicasEl {
    pub fn build(self) -> SecretManagerSecretReplicationElUserManagedElReplicasEl {
        SecretManagerSecretReplicationElUserManagedElReplicasEl {
            location: self.location,
            customer_managed_encryption: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecretManagerSecretReplicationElUserManagedElReplicasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerSecretReplicationElUserManagedElReplicasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecretManagerSecretReplicationElUserManagedElReplicasElRef {
        SecretManagerSecretReplicationElUserManagedElReplicasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecretManagerSecretReplicationElUserManagedElReplicasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe canonical IDs of the location to replicate data. For example: \"us-east1\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> ListRef<
        SecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SecretManagerSecretReplicationElUserManagedElDynamic {
    replicas: Option<DynamicBlock<SecretManagerSecretReplicationElUserManagedElReplicasEl>>,
}
#[derive(Serialize)]
pub struct SecretManagerSecretReplicationElUserManagedEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    replicas: Option<Vec<SecretManagerSecretReplicationElUserManagedElReplicasEl>>,
    dynamic: SecretManagerSecretReplicationElUserManagedElDynamic,
}
impl SecretManagerSecretReplicationElUserManagedEl {
    #[doc = "Set the field `replicas`.\n"]
    pub fn set_replicas(
        mut self,
        v: impl Into<BlockAssignable<SecretManagerSecretReplicationElUserManagedElReplicasEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.replicas = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.replicas = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecretManagerSecretReplicationElUserManagedEl {
    type O = BlockAssignable<SecretManagerSecretReplicationElUserManagedEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecretManagerSecretReplicationElUserManagedEl {}
impl BuildSecretManagerSecretReplicationElUserManagedEl {
    pub fn build(self) -> SecretManagerSecretReplicationElUserManagedEl {
        SecretManagerSecretReplicationElUserManagedEl {
            replicas: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecretManagerSecretReplicationElUserManagedElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerSecretReplicationElUserManagedElRef {
    fn new(shared: StackShared, base: String) -> SecretManagerSecretReplicationElUserManagedElRef {
        SecretManagerSecretReplicationElUserManagedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecretManagerSecretReplicationElUserManagedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(&self) -> ListRef<SecretManagerSecretReplicationElUserManagedElReplicasElRef> {
        ListRef::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecretManagerSecretReplicationElDynamic {
    auto: Option<DynamicBlock<SecretManagerSecretReplicationElAutoEl>>,
    user_managed: Option<DynamicBlock<SecretManagerSecretReplicationElUserManagedEl>>,
}
#[derive(Serialize)]
pub struct SecretManagerSecretReplicationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto: Option<Vec<SecretManagerSecretReplicationElAutoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_managed: Option<Vec<SecretManagerSecretReplicationElUserManagedEl>>,
    dynamic: SecretManagerSecretReplicationElDynamic,
}
impl SecretManagerSecretReplicationEl {
    #[doc = "Set the field `auto`.\n"]
    pub fn set_auto(
        mut self,
        v: impl Into<BlockAssignable<SecretManagerSecretReplicationElAutoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auto = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auto = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `user_managed`.\n"]
    pub fn set_user_managed(
        mut self,
        v: impl Into<BlockAssignable<SecretManagerSecretReplicationElUserManagedEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.user_managed = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.user_managed = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecretManagerSecretReplicationEl {
    type O = BlockAssignable<SecretManagerSecretReplicationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecretManagerSecretReplicationEl {}
impl BuildSecretManagerSecretReplicationEl {
    pub fn build(self) -> SecretManagerSecretReplicationEl {
        SecretManagerSecretReplicationEl {
            auto: core::default::Default::default(),
            user_managed: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecretManagerSecretReplicationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerSecretReplicationElRef {
    fn new(shared: StackShared, base: String) -> SecretManagerSecretReplicationElRef {
        SecretManagerSecretReplicationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecretManagerSecretReplicationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto` after provisioning.\n"]
    pub fn auto(&self) -> ListRef<SecretManagerSecretReplicationElAutoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto", self.base))
    }
    #[doc = "Get a reference to the value of field `user_managed` after provisioning.\n"]
    pub fn user_managed(&self) -> ListRef<SecretManagerSecretReplicationElUserManagedElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_managed", self.base))
    }
}
#[derive(Serialize)]
pub struct SecretManagerSecretRotationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    next_rotation_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation_period: Option<PrimField<String>>,
}
impl SecretManagerSecretRotationEl {
    #[doc = "Set the field `next_rotation_time`.\nTimestamp in UTC at which the Secret is scheduled to rotate.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_next_rotation_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_rotation_time = Some(v.into());
        self
    }
    #[doc = "Set the field `rotation_period`.\nThe Duration between rotation notifications. Must be in seconds and at least 3600s (1h) and at most 3153600000s (100 years).\nIf rotationPeriod is set, 'next_rotation_time' must be set. 'next_rotation_time' will be advanced by this period when the service automatically sends rotation notifications."]
    pub fn set_rotation_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rotation_period = Some(v.into());
        self
    }
}
impl ToListMappable for SecretManagerSecretRotationEl {
    type O = BlockAssignable<SecretManagerSecretRotationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecretManagerSecretRotationEl {}
impl BuildSecretManagerSecretRotationEl {
    pub fn build(self) -> SecretManagerSecretRotationEl {
        SecretManagerSecretRotationEl {
            next_rotation_time: core::default::Default::default(),
            rotation_period: core::default::Default::default(),
        }
    }
}
pub struct SecretManagerSecretRotationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerSecretRotationElRef {
    fn new(shared: StackShared, base: String) -> SecretManagerSecretRotationElRef {
        SecretManagerSecretRotationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecretManagerSecretRotationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `next_rotation_time` after provisioning.\nTimestamp in UTC at which the Secret is scheduled to rotate.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn next_rotation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_rotation_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rotation_period` after provisioning.\nThe Duration between rotation notifications. Must be in seconds and at least 3600s (1h) and at most 3153600000s (100 years).\nIf rotationPeriod is set, 'next_rotation_time' must be set. 'next_rotation_time' will be advanced by this period when the service automatically sends rotation notifications."]
    pub fn rotation_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rotation_period", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SecretManagerSecretTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SecretManagerSecretTimeoutsEl {
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
impl ToListMappable for SecretManagerSecretTimeoutsEl {
    type O = BlockAssignable<SecretManagerSecretTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecretManagerSecretTimeoutsEl {}
impl BuildSecretManagerSecretTimeoutsEl {
    pub fn build(self) -> SecretManagerSecretTimeoutsEl {
        SecretManagerSecretTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SecretManagerSecretTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerSecretTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SecretManagerSecretTimeoutsElRef {
        SecretManagerSecretTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecretManagerSecretTimeoutsElRef {
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
pub struct SecretManagerSecretTopicsEl {
    name: PrimField<String>,
}
impl SecretManagerSecretTopicsEl {}
impl ToListMappable for SecretManagerSecretTopicsEl {
    type O = BlockAssignable<SecretManagerSecretTopicsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecretManagerSecretTopicsEl {
    #[doc = "The resource name of the Pub/Sub topic that will be published to, in the following format: projects/*/topics/*.\nFor publication to succeed, the Secret Manager Service Agent service account must have pubsub.publisher permissions on the topic."]
    pub name: PrimField<String>,
}
impl BuildSecretManagerSecretTopicsEl {
    pub fn build(self) -> SecretManagerSecretTopicsEl {
        SecretManagerSecretTopicsEl { name: self.name }
    }
}
pub struct SecretManagerSecretTopicsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecretManagerSecretTopicsElRef {
    fn new(shared: StackShared, base: String) -> SecretManagerSecretTopicsElRef {
        SecretManagerSecretTopicsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecretManagerSecretTopicsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Pub/Sub topic that will be published to, in the following format: projects/*/topics/*.\nFor publication to succeed, the Secret Manager Service Agent service account must have pubsub.publisher permissions on the topic."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct SecretManagerSecretDynamic {
    replication: Option<DynamicBlock<SecretManagerSecretReplicationEl>>,
    rotation: Option<DynamicBlock<SecretManagerSecretRotationEl>>,
    topics: Option<DynamicBlock<SecretManagerSecretTopicsEl>>,
}
