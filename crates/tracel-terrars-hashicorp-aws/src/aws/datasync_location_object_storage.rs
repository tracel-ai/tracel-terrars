use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DatasyncLocationObjectStorageData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_arns: Option<SetField<PrimField<String>>>,
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_certificate: Option<PrimField<String>>,
    server_hostname: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subdirectory: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
}

struct DatasyncLocationObjectStorage_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatasyncLocationObjectStorageData>,
}

#[derive(Clone)]
pub struct DatasyncLocationObjectStorage(Rc<DatasyncLocationObjectStorage_>);

impl DatasyncLocationObjectStorage {
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

    #[doc = "Set the field `access_key`.\n"]
    pub fn set_access_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_key = Some(v.into());
        self
    }

    #[doc = "Set the field `agent_arns`.\n"]
    pub fn set_agent_arns(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().agent_arns = Some(v.into());
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

    #[doc = "Set the field `secret_key`.\n"]
    pub fn set_secret_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().secret_key = Some(v.into());
        self
    }

    #[doc = "Set the field `server_certificate`.\n"]
    pub fn set_server_certificate(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().server_certificate = Some(v.into());
        self
    }

    #[doc = "Set the field `server_port`.\n"]
    pub fn set_server_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().server_port = Some(v.into());
        self
    }

    #[doc = "Set the field `server_protocol`.\n"]
    pub fn set_server_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().server_protocol = Some(v.into());
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

    #[doc = "Get a reference to the value of field `access_key` after provisioning.\n"]
    pub fn access_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_arns` after provisioning.\n"]
    pub fn agent_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.agent_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `secret_key` after provisioning.\n"]
    pub fn secret_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_certificate` after provisioning.\n"]
    pub fn server_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_hostname` after provisioning.\n"]
    pub fn server_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_port` after provisioning.\n"]
    pub fn server_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_protocol` after provisioning.\n"]
    pub fn server_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_protocol", self.extract_ref()))
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
}

impl Referable for DatasyncLocationObjectStorage {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DatasyncLocationObjectStorage { }

impl ToListMappable for DatasyncLocationObjectStorage {
    type O = ListRef<DatasyncLocationObjectStorageRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DatasyncLocationObjectStorage_ {
    fn extract_resource_type(&self) -> String {
        "aws_datasync_location_object_storage".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatasyncLocationObjectStorage {
    pub tf_id: String,
    #[doc = ""]
    pub bucket_name: PrimField<String>,
    #[doc = ""]
    pub server_hostname: PrimField<String>,
}

impl BuildDatasyncLocationObjectStorage {
    pub fn build(self, stack: &mut Stack) -> DatasyncLocationObjectStorage {
        let out = DatasyncLocationObjectStorage(Rc::new(DatasyncLocationObjectStorage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatasyncLocationObjectStorageData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_key: core::default::Default::default(),
                agent_arns: core::default::Default::default(),
                bucket_name: self.bucket_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                secret_key: core::default::Default::default(),
                server_certificate: core::default::Default::default(),
                server_hostname: self.server_hostname,
                server_port: core::default::Default::default(),
                server_protocol: core::default::Default::default(),
                subdirectory: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatasyncLocationObjectStorageRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncLocationObjectStorageRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DatasyncLocationObjectStorageRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_key` after provisioning.\n"]
    pub fn access_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_arns` after provisioning.\n"]
    pub fn agent_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.agent_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `secret_key` after provisioning.\n"]
    pub fn secret_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_certificate` after provisioning.\n"]
    pub fn server_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_hostname` after provisioning.\n"]
    pub fn server_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_port` after provisioning.\n"]
    pub fn server_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_protocol` after provisioning.\n"]
    pub fn server_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_protocol", self.extract_ref()))
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
}
