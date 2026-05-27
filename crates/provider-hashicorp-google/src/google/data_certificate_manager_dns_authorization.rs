use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCertificateManagerDnsAuthorizationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    domain: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataCertificateManagerDnsAuthorization_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCertificateManagerDnsAuthorizationData>,
}
#[derive(Clone)]
pub struct DataCertificateManagerDnsAuthorization(Rc<DataCertificateManagerDnsAuthorization_>);
impl DataCertificateManagerDnsAuthorization {
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
    #[doc = "Set the field `location`.\nThe Certificate Manager location. If not specified, \"global\" is used."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_resource_record` after provisioning.\nThe structure describing the DNS Resource Record that needs to be added\nto DNS configuration for the authorization to be usable by\ncertificate."]
    pub fn dns_resource_record(
        &self,
    ) -> ListRef<DataCertificateManagerDnsAuthorizationDnsResourceRecordElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_resource_record", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nA domain which is being authorized. A DnsAuthorization resource covers a\nsingle domain and its wildcard, e.g. authorization for \"example.com\" can\nbe used to issue certificates for \"example.com\" and \"*.example.com\"."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the DNS Authorization resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe Certificate Manager location. If not specified, \"global\" is used."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\ntype of DNS authorization. If unset during the resource creation, FIXED_RECORD will\nbe used for global resources, and PER_PROJECT_RECORD will be used for other locations.\n\nFIXED_RECORD DNS authorization uses DNS-01 validation method\n\nPER_PROJECT_RECORD DNS authorization allows for independent management\nof Google-managed certificates with DNS authorization across multiple\nprojects. Possible values: [\"FIXED_RECORD\", \"PER_PROJECT_RECORD\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
impl Referable for DataCertificateManagerDnsAuthorization {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCertificateManagerDnsAuthorization {}
impl ToListMappable for DataCertificateManagerDnsAuthorization {
    type O = ListRef<DataCertificateManagerDnsAuthorizationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCertificateManagerDnsAuthorization_ {
    fn extract_datasource_type(&self) -> String {
        "google_certificate_manager_dns_authorization".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCertificateManagerDnsAuthorization {
    pub tf_id: String,
    #[doc = "A domain which is being authorized. A DnsAuthorization resource covers a\nsingle domain and its wildcard, e.g. authorization for \"example.com\" can\nbe used to issue certificates for \"example.com\" and \"*.example.com\"."]
    pub domain: PrimField<String>,
    #[doc = "Name of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub name: PrimField<String>,
}
impl BuildDataCertificateManagerDnsAuthorization {
    pub fn build(self, stack: &mut Stack) -> DataCertificateManagerDnsAuthorization {
        let out = DataCertificateManagerDnsAuthorization(Rc::new(
            DataCertificateManagerDnsAuthorization_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataCertificateManagerDnsAuthorizationData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    domain: self.domain,
                    id: core::default::Default::default(),
                    location: core::default::Default::default(),
                    name: self.name,
                    project: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCertificateManagerDnsAuthorizationRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCertificateManagerDnsAuthorizationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCertificateManagerDnsAuthorizationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_resource_record` after provisioning.\nThe structure describing the DNS Resource Record that needs to be added\nto DNS configuration for the authorization to be usable by\ncertificate."]
    pub fn dns_resource_record(
        &self,
    ) -> ListRef<DataCertificateManagerDnsAuthorizationDnsResourceRecordElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_resource_record", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nA domain which is being authorized. A DnsAuthorization resource covers a\nsingle domain and its wildcard, e.g. authorization for \"example.com\" can\nbe used to issue certificates for \"example.com\" and \"*.example.com\"."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the DNS Authorization resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe Certificate Manager location. If not specified, \"global\" is used."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\ntype of DNS authorization. If unset during the resource creation, FIXED_RECORD will\nbe used for global resources, and PER_PROJECT_RECORD will be used for other locations.\n\nFIXED_RECORD DNS authorization uses DNS-01 validation method\n\nPER_PROJECT_RECORD DNS authorization allows for independent management\nof Google-managed certificates with DNS authorization across multiple\nprojects. Possible values: [\"FIXED_RECORD\", \"PER_PROJECT_RECORD\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCertificateManagerDnsAuthorizationDnsResourceRecordEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataCertificateManagerDnsAuthorizationDnsResourceRecordEl {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataCertificateManagerDnsAuthorizationDnsResourceRecordEl {
    type O = BlockAssignable<DataCertificateManagerDnsAuthorizationDnsResourceRecordEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCertificateManagerDnsAuthorizationDnsResourceRecordEl {}
impl BuildDataCertificateManagerDnsAuthorizationDnsResourceRecordEl {
    pub fn build(self) -> DataCertificateManagerDnsAuthorizationDnsResourceRecordEl {
        DataCertificateManagerDnsAuthorizationDnsResourceRecordEl {
            data: core::default::Default::default(),
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataCertificateManagerDnsAuthorizationDnsResourceRecordElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCertificateManagerDnsAuthorizationDnsResourceRecordElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCertificateManagerDnsAuthorizationDnsResourceRecordElRef {
        DataCertificateManagerDnsAuthorizationDnsResourceRecordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCertificateManagerDnsAuthorizationDnsResourceRecordElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
