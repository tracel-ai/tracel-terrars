use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DatasyncLocationAzureBlobData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_tier: Option<PrimField<String>>,
    agent_arns: SetField<PrimField<String>>,
    authentication_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blob_type: Option<PrimField<String>>,
    container_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subdirectory: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sas_configuration: Option<Vec<DatasyncLocationAzureBlobSasConfigurationEl>>,
    dynamic: DatasyncLocationAzureBlobDynamic,
}

struct DatasyncLocationAzureBlob_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatasyncLocationAzureBlobData>,
}

#[derive(Clone)]
pub struct DatasyncLocationAzureBlob(Rc<DatasyncLocationAzureBlob_>);

impl DatasyncLocationAzureBlob {
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

    #[doc = "Set the field `access_tier`.\n"]
    pub fn set_access_tier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_tier = Some(v.into());
        self
    }

    #[doc = "Set the field `blob_type`.\n"]
    pub fn set_blob_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().blob_type = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `subdirectory`.\n"]
    pub fn set_subdirectory(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subdirectory = Some(v.into());
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

    #[doc = "Set the field `sas_configuration`.\n"]
    pub fn set_sas_configuration(
        self,
        v: impl Into<BlockAssignable<DatasyncLocationAzureBlobSasConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sas_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sas_configuration = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `access_tier` after provisioning.\n"]
    pub fn access_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_tier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_arns` after provisioning.\n"]
    pub fn agent_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.agent_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blob_type` after provisioning.\n"]
    pub fn blob_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blob_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_url` after provisioning.\n"]
    pub fn container_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subdirectory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sas_configuration` after provisioning.\n"]
    pub fn sas_configuration(&self) -> ListRef<DatasyncLocationAzureBlobSasConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sas_configuration", self.extract_ref()))
    }
}

impl Referable for DatasyncLocationAzureBlob {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DatasyncLocationAzureBlob { }

impl ToListMappable for DatasyncLocationAzureBlob {
    type O = ListRef<DatasyncLocationAzureBlobRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DatasyncLocationAzureBlob_ {
    fn extract_resource_type(&self) -> String {
        "aws_datasync_location_azure_blob".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatasyncLocationAzureBlob {
    pub tf_id: String,
    #[doc = ""]
    pub agent_arns: SetField<PrimField<String>>,
    #[doc = ""]
    pub authentication_type: PrimField<String>,
    #[doc = ""]
    pub container_url: PrimField<String>,
}

impl BuildDatasyncLocationAzureBlob {
    pub fn build(self, stack: &mut Stack) -> DatasyncLocationAzureBlob {
        let out = DatasyncLocationAzureBlob(Rc::new(DatasyncLocationAzureBlob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatasyncLocationAzureBlobData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_tier: core::default::Default::default(),
                agent_arns: self.agent_arns,
                authentication_type: self.authentication_type,
                blob_type: core::default::Default::default(),
                container_url: self.container_url,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                subdirectory: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                sas_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatasyncLocationAzureBlobRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncLocationAzureBlobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DatasyncLocationAzureBlobRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_tier` after provisioning.\n"]
    pub fn access_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_tier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_arns` after provisioning.\n"]
    pub fn agent_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.agent_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blob_type` after provisioning.\n"]
    pub fn blob_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blob_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_url` after provisioning.\n"]
    pub fn container_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subdirectory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sas_configuration` after provisioning.\n"]
    pub fn sas_configuration(&self) -> ListRef<DatasyncLocationAzureBlobSasConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sas_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DatasyncLocationAzureBlobSasConfigurationEl {
    token: PrimField<String>,
}

impl DatasyncLocationAzureBlobSasConfigurationEl { }

impl ToListMappable for DatasyncLocationAzureBlobSasConfigurationEl {
    type O = BlockAssignable<DatasyncLocationAzureBlobSasConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncLocationAzureBlobSasConfigurationEl {
    #[doc = ""]
    pub token: PrimField<String>,
}

impl BuildDatasyncLocationAzureBlobSasConfigurationEl {
    pub fn build(self) -> DatasyncLocationAzureBlobSasConfigurationEl {
        DatasyncLocationAzureBlobSasConfigurationEl { token: self.token }
    }
}

pub struct DatasyncLocationAzureBlobSasConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncLocationAzureBlobSasConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DatasyncLocationAzureBlobSasConfigurationElRef {
        DatasyncLocationAzureBlobSasConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncLocationAzureBlobSasConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatasyncLocationAzureBlobDynamic {
    sas_configuration: Option<DynamicBlock<DatasyncLocationAzureBlobSasConfigurationEl>>,
}
