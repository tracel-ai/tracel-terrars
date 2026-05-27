use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataSecretManagerSecretData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    secret_id: PrimField<String>,
}
struct DataSecretManagerSecret_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSecretManagerSecretData>,
}
#[derive(Clone)]
pub struct DataSecretManagerSecret(Rc<DataSecretManagerSecret_>);
impl DataSecretManagerSecret {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
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
    #[doc = "Get a reference to the value of field `replication` after provisioning.\nThe replication policy of the secret data attached to the Secret. It cannot be changed\nafter the Secret has been created."]
    pub fn replication(&self) -> ListRef<DataSecretManagerSecretReplicationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rotation` after provisioning.\nThe rotation time and period for a Secret. At 'next_rotation_time', Secret Manager will send a Pub/Sub notification to the topics configured on the Secret. 'topics' must be set to configure rotation."]
    pub fn rotation(&self) -> ListRef<DataSecretManagerSecretRotationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rotation", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `topics` after provisioning.\nA list of up to 10 Pub/Sub topics to which messages are published when control plane operations are called on the secret or its versions."]
    pub fn topics(&self) -> ListRef<DataSecretManagerSecretTopicsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.topics", self.extract_ref()),
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
}
impl Referable for DataSecretManagerSecret {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataSecretManagerSecret {}
impl ToListMappable for DataSecretManagerSecret {
    type O = ListRef<DataSecretManagerSecretRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataSecretManagerSecret_ {
    fn extract_datasource_type(&self) -> String {
        "google_secret_manager_secret".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataSecretManagerSecret {
    pub tf_id: String,
    #[doc = "This must be unique within the project."]
    pub secret_id: PrimField<String>,
}
impl BuildDataSecretManagerSecret {
    pub fn build(self, stack: &mut Stack) -> DataSecretManagerSecret {
        let out = DataSecretManagerSecret(Rc::new(DataSecretManagerSecret_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSecretManagerSecretData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                secret_id: self.secret_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataSecretManagerSecretRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataSecretManagerSecretRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
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
    #[doc = "Get a reference to the value of field `replication` after provisioning.\nThe replication policy of the secret data attached to the Secret. It cannot be changed\nafter the Secret has been created."]
    pub fn replication(&self) -> ListRef<DataSecretManagerSecretReplicationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.replication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rotation` after provisioning.\nThe rotation time and period for a Secret. At 'next_rotation_time', Secret Manager will send a Pub/Sub notification to the topics configured on the Secret. 'topics' must be set to configure rotation."]
    pub fn rotation(&self) -> ListRef<DataSecretManagerSecretRotationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rotation", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `topics` after provisioning.\nA list of up to 10 Pub/Sub topics to which messages are published when control plane operations are called on the secret or its versions."]
    pub fn topics(&self) -> ListRef<DataSecretManagerSecretTopicsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.topics", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
}
impl DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
    type O = BlockAssignable<DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {}
impl BuildDataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
    pub fn build(self) -> DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
        DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
            kms_key_name: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef {
        DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerSecretReplicationElAutoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_encryption:
        Option<ListField<DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl>>,
}
impl DataSecretManagerSecretReplicationElAutoEl {
    #[doc = "Set the field `customer_managed_encryption`.\n"]
    pub fn set_customer_managed_encryption(
        mut self,
        v: impl Into<ListField<DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl>>,
    ) -> Self {
        self.customer_managed_encryption = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerSecretReplicationElAutoEl {
    type O = BlockAssignable<DataSecretManagerSecretReplicationElAutoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerSecretReplicationElAutoEl {}
impl BuildDataSecretManagerSecretReplicationElAutoEl {
    pub fn build(self) -> DataSecretManagerSecretReplicationElAutoEl {
        DataSecretManagerSecretReplicationElAutoEl {
            customer_managed_encryption: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerSecretReplicationElAutoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretReplicationElAutoElRef {
    fn new(shared: StackShared, base: String) -> DataSecretManagerSecretReplicationElAutoElRef {
        DataSecretManagerSecretReplicationElAutoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerSecretReplicationElAutoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> ListRef<DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
}
impl DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl
{
    type O = BlockAssignable<
        DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl
{}
impl BuildDataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
    pub fn build(
        self,
    ) -> DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl
    {
        DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
            kms_key_name: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef
    {
        DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerSecretReplicationElUserManagedElReplicasEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_encryption: Option<
        ListField<
            DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
}
impl DataSecretManagerSecretReplicationElUserManagedElReplicasEl {
    #[doc = "Set the field `customer_managed_encryption`.\n"]
    pub fn set_customer_managed_encryption(
        mut self,
        v : impl Into < ListField < DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl > >,
    ) -> Self {
        self.customer_managed_encryption = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerSecretReplicationElUserManagedElReplicasEl {
    type O = BlockAssignable<DataSecretManagerSecretReplicationElUserManagedElReplicasEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerSecretReplicationElUserManagedElReplicasEl {}
impl BuildDataSecretManagerSecretReplicationElUserManagedElReplicasEl {
    pub fn build(self) -> DataSecretManagerSecretReplicationElUserManagedElReplicasEl {
        DataSecretManagerSecretReplicationElUserManagedElReplicasEl {
            customer_managed_encryption: core::default::Default::default(),
            location: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerSecretReplicationElUserManagedElReplicasElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretReplicationElUserManagedElReplicasElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSecretManagerSecretReplicationElUserManagedElReplicasElRef {
        DataSecretManagerSecretReplicationElUserManagedElReplicasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerSecretReplicationElUserManagedElReplicasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> ListRef<
        DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerSecretReplicationElUserManagedEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    replicas: Option<ListField<DataSecretManagerSecretReplicationElUserManagedElReplicasEl>>,
}
impl DataSecretManagerSecretReplicationElUserManagedEl {
    #[doc = "Set the field `replicas`.\n"]
    pub fn set_replicas(
        mut self,
        v: impl Into<ListField<DataSecretManagerSecretReplicationElUserManagedElReplicasEl>>,
    ) -> Self {
        self.replicas = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerSecretReplicationElUserManagedEl {
    type O = BlockAssignable<DataSecretManagerSecretReplicationElUserManagedEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerSecretReplicationElUserManagedEl {}
impl BuildDataSecretManagerSecretReplicationElUserManagedEl {
    pub fn build(self) -> DataSecretManagerSecretReplicationElUserManagedEl {
        DataSecretManagerSecretReplicationElUserManagedEl {
            replicas: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerSecretReplicationElUserManagedElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretReplicationElUserManagedElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSecretManagerSecretReplicationElUserManagedElRef {
        DataSecretManagerSecretReplicationElUserManagedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerSecretReplicationElUserManagedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(
        &self,
    ) -> ListRef<DataSecretManagerSecretReplicationElUserManagedElReplicasElRef> {
        ListRef::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerSecretReplicationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto: Option<ListField<DataSecretManagerSecretReplicationElAutoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_managed: Option<ListField<DataSecretManagerSecretReplicationElUserManagedEl>>,
}
impl DataSecretManagerSecretReplicationEl {
    #[doc = "Set the field `auto`.\n"]
    pub fn set_auto(
        mut self,
        v: impl Into<ListField<DataSecretManagerSecretReplicationElAutoEl>>,
    ) -> Self {
        self.auto = Some(v.into());
        self
    }
    #[doc = "Set the field `user_managed`.\n"]
    pub fn set_user_managed(
        mut self,
        v: impl Into<ListField<DataSecretManagerSecretReplicationElUserManagedEl>>,
    ) -> Self {
        self.user_managed = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerSecretReplicationEl {
    type O = BlockAssignable<DataSecretManagerSecretReplicationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerSecretReplicationEl {}
impl BuildDataSecretManagerSecretReplicationEl {
    pub fn build(self) -> DataSecretManagerSecretReplicationEl {
        DataSecretManagerSecretReplicationEl {
            auto: core::default::Default::default(),
            user_managed: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerSecretReplicationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretReplicationElRef {
    fn new(shared: StackShared, base: String) -> DataSecretManagerSecretReplicationElRef {
        DataSecretManagerSecretReplicationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerSecretReplicationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto` after provisioning.\n"]
    pub fn auto(&self) -> ListRef<DataSecretManagerSecretReplicationElAutoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto", self.base))
    }
    #[doc = "Get a reference to the value of field `user_managed` after provisioning.\n"]
    pub fn user_managed(&self) -> ListRef<DataSecretManagerSecretReplicationElUserManagedElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_managed", self.base))
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerSecretRotationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    next_rotation_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation_period: Option<PrimField<String>>,
}
impl DataSecretManagerSecretRotationEl {
    #[doc = "Set the field `next_rotation_time`.\n"]
    pub fn set_next_rotation_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_rotation_time = Some(v.into());
        self
    }
    #[doc = "Set the field `rotation_period`.\n"]
    pub fn set_rotation_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rotation_period = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerSecretRotationEl {
    type O = BlockAssignable<DataSecretManagerSecretRotationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerSecretRotationEl {}
impl BuildDataSecretManagerSecretRotationEl {
    pub fn build(self) -> DataSecretManagerSecretRotationEl {
        DataSecretManagerSecretRotationEl {
            next_rotation_time: core::default::Default::default(),
            rotation_period: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerSecretRotationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretRotationElRef {
    fn new(shared: StackShared, base: String) -> DataSecretManagerSecretRotationElRef {
        DataSecretManagerSecretRotationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerSecretRotationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `next_rotation_time` after provisioning.\n"]
    pub fn next_rotation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_rotation_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rotation_period` after provisioning.\n"]
    pub fn rotation_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rotation_period", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataSecretManagerSecretTopicsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataSecretManagerSecretTopicsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataSecretManagerSecretTopicsEl {
    type O = BlockAssignable<DataSecretManagerSecretTopicsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataSecretManagerSecretTopicsEl {}
impl BuildDataSecretManagerSecretTopicsEl {
    pub fn build(self) -> DataSecretManagerSecretTopicsEl {
        DataSecretManagerSecretTopicsEl {
            name: core::default::Default::default(),
        }
    }
}
pub struct DataSecretManagerSecretTopicsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataSecretManagerSecretTopicsElRef {
    fn new(shared: StackShared, base: String) -> DataSecretManagerSecretTopicsElRef {
        DataSecretManagerSecretTopicsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataSecretManagerSecretTopicsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
