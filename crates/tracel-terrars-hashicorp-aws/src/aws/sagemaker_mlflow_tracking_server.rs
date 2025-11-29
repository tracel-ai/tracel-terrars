use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerMlflowTrackingServerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    artifact_store_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_model_registration: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mlflow_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    tracking_server_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracking_server_size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weekly_maintenance_window_start: Option<PrimField<String>>,
}

struct SagemakerMlflowTrackingServer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerMlflowTrackingServerData>,
}

#[derive(Clone)]
pub struct SagemakerMlflowTrackingServer(Rc<SagemakerMlflowTrackingServer_>);

impl SagemakerMlflowTrackingServer {
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

    #[doc = "Set the field `automatic_model_registration`.\n"]
    pub fn set_automatic_model_registration(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().automatic_model_registration = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `mlflow_version`.\n"]
    pub fn set_mlflow_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mlflow_version = Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `tracking_server_size`.\n"]
    pub fn set_tracking_server_size(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tracking_server_size = Some(v.into());
        self
    }

    #[doc = "Set the field `weekly_maintenance_window_start`.\n"]
    pub fn set_weekly_maintenance_window_start(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().weekly_maintenance_window_start = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `artifact_store_uri` after provisioning.\n"]
    pub fn artifact_store_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.artifact_store_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_model_registration` after provisioning.\n"]
    pub fn automatic_model_registration(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.automatic_model_registration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mlflow_version` after provisioning.\n"]
    pub fn mlflow_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mlflow_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracking_server_name` after provisioning.\n"]
    pub fn tracking_server_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tracking_server_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracking_server_size` after provisioning.\n"]
    pub fn tracking_server_size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tracking_server_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracking_server_url` after provisioning.\n"]
    pub fn tracking_server_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tracking_server_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_window_start` after provisioning.\n"]
    pub fn weekly_maintenance_window_start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.weekly_maintenance_window_start", self.extract_ref()))
    }
}

impl Referable for SagemakerMlflowTrackingServer {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SagemakerMlflowTrackingServer { }

impl ToListMappable for SagemakerMlflowTrackingServer {
    type O = ListRef<SagemakerMlflowTrackingServerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerMlflowTrackingServer_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_mlflow_tracking_server".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerMlflowTrackingServer {
    pub tf_id: String,
    #[doc = ""]
    pub artifact_store_uri: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub tracking_server_name: PrimField<String>,
}

impl BuildSagemakerMlflowTrackingServer {
    pub fn build(self, stack: &mut Stack) -> SagemakerMlflowTrackingServer {
        let out = SagemakerMlflowTrackingServer(Rc::new(SagemakerMlflowTrackingServer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SagemakerMlflowTrackingServerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                artifact_store_uri: self.artifact_store_uri,
                automatic_model_registration: core::default::Default::default(),
                id: core::default::Default::default(),
                mlflow_version: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                tracking_server_name: self.tracking_server_name,
                tracking_server_size: core::default::Default::default(),
                weekly_maintenance_window_start: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerMlflowTrackingServerRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerMlflowTrackingServerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl SagemakerMlflowTrackingServerRef {
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

    #[doc = "Get a reference to the value of field `artifact_store_uri` after provisioning.\n"]
    pub fn artifact_store_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.artifact_store_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_model_registration` after provisioning.\n"]
    pub fn automatic_model_registration(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.automatic_model_registration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mlflow_version` after provisioning.\n"]
    pub fn mlflow_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mlflow_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracking_server_name` after provisioning.\n"]
    pub fn tracking_server_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tracking_server_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracking_server_size` after provisioning.\n"]
    pub fn tracking_server_size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tracking_server_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tracking_server_url` after provisioning.\n"]
    pub fn tracking_server_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tracking_server_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_window_start` after provisioning.\n"]
    pub fn weekly_maintenance_window_start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.weekly_maintenance_window_start", self.extract_ref()))
    }
}
