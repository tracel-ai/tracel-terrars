use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlueDevEndpointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arguments: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_jars_s3_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_python_libs_s3_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    glue_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_nodes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_workers: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_keys: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_configuration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_type: Option<PrimField<String>>,
}

struct GlueDevEndpoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GlueDevEndpointData>,
}

#[derive(Clone)]
pub struct GlueDevEndpoint(Rc<GlueDevEndpoint_>);

impl GlueDevEndpoint {
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

    #[doc = "Set the field `arguments`.\n"]
    pub fn set_arguments(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().arguments = Some(v.into());
        self
    }

    #[doc = "Set the field `extra_jars_s3_path`.\n"]
    pub fn set_extra_jars_s3_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().extra_jars_s3_path = Some(v.into());
        self
    }

    #[doc = "Set the field `extra_python_libs_s3_path`.\n"]
    pub fn set_extra_python_libs_s3_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().extra_python_libs_s3_path = Some(v.into());
        self
    }

    #[doc = "Set the field `glue_version`.\n"]
    pub fn set_glue_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().glue_version = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_nodes`.\n"]
    pub fn set_number_of_nodes(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().number_of_nodes = Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_workers`.\n"]
    pub fn set_number_of_workers(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().number_of_workers = Some(v.into());
        self
    }

    #[doc = "Set the field `public_key`.\n"]
    pub fn set_public_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().public_key = Some(v.into());
        self
    }

    #[doc = "Set the field `public_keys`.\n"]
    pub fn set_public_keys(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().public_keys = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `security_configuration`.\n"]
    pub fn set_security_configuration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_group_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subnet_id = Some(v.into());
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

    #[doc = "Set the field `worker_type`.\n"]
    pub fn set_worker_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().worker_type = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arguments` after provisioning.\n"]
    pub fn arguments(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.arguments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extra_jars_s3_path` after provisioning.\n"]
    pub fn extra_jars_s3_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.extra_jars_s3_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extra_python_libs_s3_path` after provisioning.\n"]
    pub fn extra_python_libs_s3_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.extra_python_libs_s3_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `glue_version` after provisioning.\n"]
    pub fn glue_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.glue_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_nodes` after provisioning.\n"]
    pub fn number_of_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_nodes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_workers` after provisioning.\n"]
    pub fn number_of_workers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_workers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_address` after provisioning.\n"]
    pub fn private_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_address` after provisioning.\n"]
    pub fn public_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_keys` after provisioning.\n"]
    pub fn public_keys(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.public_keys", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `security_configuration` after provisioning.\n"]
    pub fn security_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_type` after provisioning.\n"]
    pub fn worker_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `yarn_endpoint_address` after provisioning.\n"]
    pub fn yarn_endpoint_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.yarn_endpoint_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zeppelin_remote_spark_interpreter_port` after provisioning.\n"]
    pub fn zeppelin_remote_spark_interpreter_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zeppelin_remote_spark_interpreter_port", self.extract_ref()),
        )
    }
}

impl Referable for GlueDevEndpoint {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GlueDevEndpoint { }

impl ToListMappable for GlueDevEndpoint {
    type O = ListRef<GlueDevEndpointRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GlueDevEndpoint_ {
    fn extract_resource_type(&self) -> String {
        "aws_glue_dev_endpoint".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlueDevEndpoint {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildGlueDevEndpoint {
    pub fn build(self, stack: &mut Stack) -> GlueDevEndpoint {
        let out = GlueDevEndpoint(Rc::new(GlueDevEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GlueDevEndpointData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                arguments: core::default::Default::default(),
                extra_jars_s3_path: core::default::Default::default(),
                extra_python_libs_s3_path: core::default::Default::default(),
                glue_version: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                number_of_nodes: core::default::Default::default(),
                number_of_workers: core::default::Default::default(),
                public_key: core::default::Default::default(),
                public_keys: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                security_configuration: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                subnet_id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                worker_type: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlueDevEndpointRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueDevEndpointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl GlueDevEndpointRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arguments` after provisioning.\n"]
    pub fn arguments(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.arguments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extra_jars_s3_path` after provisioning.\n"]
    pub fn extra_jars_s3_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.extra_jars_s3_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extra_python_libs_s3_path` after provisioning.\n"]
    pub fn extra_python_libs_s3_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.extra_python_libs_s3_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `glue_version` after provisioning.\n"]
    pub fn glue_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.glue_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_nodes` after provisioning.\n"]
    pub fn number_of_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_nodes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_workers` after provisioning.\n"]
    pub fn number_of_workers(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_workers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_address` after provisioning.\n"]
    pub fn private_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_address` after provisioning.\n"]
    pub fn public_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_keys` after provisioning.\n"]
    pub fn public_keys(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.public_keys", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `security_configuration` after provisioning.\n"]
    pub fn security_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_type` after provisioning.\n"]
    pub fn worker_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `yarn_endpoint_address` after provisioning.\n"]
    pub fn yarn_endpoint_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.yarn_endpoint_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zeppelin_remote_spark_interpreter_port` after provisioning.\n"]
    pub fn zeppelin_remote_spark_interpreter_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zeppelin_remote_spark_interpreter_port", self.extract_ref()),
        )
    }
}
