use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DatasyncLocationHdfsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    agent_arns: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kerberos_keytab: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kerberos_keytab_base64: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kerberos_krb5_conf: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kerberos_krb5_conf_base64: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kerberos_principal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_provider_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_factor: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    simple_user: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subdirectory: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_node: Option<Vec<DatasyncLocationHdfsNameNodeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qop_configuration: Option<Vec<DatasyncLocationHdfsQopConfigurationEl>>,
    dynamic: DatasyncLocationHdfsDynamic,
}

struct DatasyncLocationHdfs_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatasyncLocationHdfsData>,
}

#[derive(Clone)]
pub struct DatasyncLocationHdfs(Rc<DatasyncLocationHdfs_>);

impl DatasyncLocationHdfs {
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

    #[doc = "Set the field `authentication_type`.\n"]
    pub fn set_authentication_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().authentication_type = Some(v.into());
        self
    }

    #[doc = "Set the field `block_size`.\n"]
    pub fn set_block_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().block_size = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `kerberos_keytab`.\n"]
    pub fn set_kerberos_keytab(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kerberos_keytab = Some(v.into());
        self
    }

    #[doc = "Set the field `kerberos_keytab_base64`.\n"]
    pub fn set_kerberos_keytab_base64(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kerberos_keytab_base64 = Some(v.into());
        self
    }

    #[doc = "Set the field `kerberos_krb5_conf`.\n"]
    pub fn set_kerberos_krb5_conf(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kerberos_krb5_conf = Some(v.into());
        self
    }

    #[doc = "Set the field `kerberos_krb5_conf_base64`.\n"]
    pub fn set_kerberos_krb5_conf_base64(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kerberos_krb5_conf_base64 = Some(v.into());
        self
    }

    #[doc = "Set the field `kerberos_principal`.\n"]
    pub fn set_kerberos_principal(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kerberos_principal = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_provider_uri`.\n"]
    pub fn set_kms_key_provider_uri(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_provider_uri = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `replication_factor`.\n"]
    pub fn set_replication_factor(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().replication_factor = Some(v.into());
        self
    }

    #[doc = "Set the field `simple_user`.\n"]
    pub fn set_simple_user(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().simple_user = Some(v.into());
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

    #[doc = "Set the field `name_node`.\n"]
    pub fn set_name_node(
        self,
        v: impl Into<BlockAssignable<DatasyncLocationHdfsNameNodeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().name_node = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.name_node = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `qop_configuration`.\n"]
    pub fn set_qop_configuration(
        self,
        v: impl Into<BlockAssignable<DatasyncLocationHdfsQopConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().qop_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.qop_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `agent_arns` after provisioning.\n"]
    pub fn agent_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.agent_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `block_size` after provisioning.\n"]
    pub fn block_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.block_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kerberos_keytab` after provisioning.\n"]
    pub fn kerberos_keytab(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos_keytab", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kerberos_keytab_base64` after provisioning.\n"]
    pub fn kerberos_keytab_base64(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos_keytab_base64", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kerberos_krb5_conf` after provisioning.\n"]
    pub fn kerberos_krb5_conf(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos_krb5_conf", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kerberos_krb5_conf_base64` after provisioning.\n"]
    pub fn kerberos_krb5_conf_base64(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos_krb5_conf_base64", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kerberos_principal` after provisioning.\n"]
    pub fn kerberos_principal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos_principal", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kms_key_provider_uri` after provisioning.\n"]
    pub fn kms_key_provider_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_provider_uri", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_factor` after provisioning.\n"]
    pub fn replication_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_factor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `simple_user` after provisioning.\n"]
    pub fn simple_user(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.simple_user", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subdirectory", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qop_configuration` after provisioning.\n"]
    pub fn qop_configuration(&self) -> ListRef<DatasyncLocationHdfsQopConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.qop_configuration", self.extract_ref()),
        )
    }
}

impl Referable for DatasyncLocationHdfs {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for DatasyncLocationHdfs {}

impl ToListMappable for DatasyncLocationHdfs {
    type O = ListRef<DatasyncLocationHdfsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DatasyncLocationHdfs_ {
    fn extract_resource_type(&self) -> String {
        "aws_datasync_location_hdfs".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatasyncLocationHdfs {
    pub tf_id: String,
    #[doc = ""]
    pub agent_arns: SetField<PrimField<String>>,
}

impl BuildDatasyncLocationHdfs {
    pub fn build(self, stack: &mut Stack) -> DatasyncLocationHdfs {
        let out = DatasyncLocationHdfs(Rc::new(DatasyncLocationHdfs_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatasyncLocationHdfsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                agent_arns: self.agent_arns,
                authentication_type: core::default::Default::default(),
                block_size: core::default::Default::default(),
                id: core::default::Default::default(),
                kerberos_keytab: core::default::Default::default(),
                kerberos_keytab_base64: core::default::Default::default(),
                kerberos_krb5_conf: core::default::Default::default(),
                kerberos_krb5_conf_base64: core::default::Default::default(),
                kerberos_principal: core::default::Default::default(),
                kms_key_provider_uri: core::default::Default::default(),
                region: core::default::Default::default(),
                replication_factor: core::default::Default::default(),
                simple_user: core::default::Default::default(),
                subdirectory: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                name_node: core::default::Default::default(),
                qop_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatasyncLocationHdfsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncLocationHdfsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DatasyncLocationHdfsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_arns` after provisioning.\n"]
    pub fn agent_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.agent_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `block_size` after provisioning.\n"]
    pub fn block_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.block_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kerberos_keytab` after provisioning.\n"]
    pub fn kerberos_keytab(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos_keytab", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kerberos_keytab_base64` after provisioning.\n"]
    pub fn kerberos_keytab_base64(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos_keytab_base64", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kerberos_krb5_conf` after provisioning.\n"]
    pub fn kerberos_krb5_conf(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos_krb5_conf", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kerberos_krb5_conf_base64` after provisioning.\n"]
    pub fn kerberos_krb5_conf_base64(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos_krb5_conf_base64", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kerberos_principal` after provisioning.\n"]
    pub fn kerberos_principal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kerberos_principal", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `kms_key_provider_uri` after provisioning.\n"]
    pub fn kms_key_provider_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_provider_uri", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `replication_factor` after provisioning.\n"]
    pub fn replication_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_factor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `simple_user` after provisioning.\n"]
    pub fn simple_user(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.simple_user", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subdirectory", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qop_configuration` after provisioning.\n"]
    pub fn qop_configuration(&self) -> ListRef<DatasyncLocationHdfsQopConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.qop_configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DatasyncLocationHdfsNameNodeEl {
    hostname: PrimField<String>,
    port: PrimField<f64>,
}

impl DatasyncLocationHdfsNameNodeEl {}

impl ToListMappable for DatasyncLocationHdfsNameNodeEl {
    type O = BlockAssignable<DatasyncLocationHdfsNameNodeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncLocationHdfsNameNodeEl {
    #[doc = ""]
    pub hostname: PrimField<String>,
    #[doc = ""]
    pub port: PrimField<f64>,
}

impl BuildDatasyncLocationHdfsNameNodeEl {
    pub fn build(self) -> DatasyncLocationHdfsNameNodeEl {
        DatasyncLocationHdfsNameNodeEl {
            hostname: self.hostname,
            port: self.port,
        }
    }
}

pub struct DatasyncLocationHdfsNameNodeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncLocationHdfsNameNodeElRef {
    fn new(shared: StackShared, base: String) -> DatasyncLocationHdfsNameNodeElRef {
        DatasyncLocationHdfsNameNodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncLocationHdfsNameNodeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct DatasyncLocationHdfsQopConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_transfer_protection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rpc_protection: Option<PrimField<String>>,
}

impl DatasyncLocationHdfsQopConfigurationEl {
    #[doc = "Set the field `data_transfer_protection`.\n"]
    pub fn set_data_transfer_protection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_transfer_protection = Some(v.into());
        self
    }

    #[doc = "Set the field `rpc_protection`.\n"]
    pub fn set_rpc_protection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rpc_protection = Some(v.into());
        self
    }
}

impl ToListMappable for DatasyncLocationHdfsQopConfigurationEl {
    type O = BlockAssignable<DatasyncLocationHdfsQopConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncLocationHdfsQopConfigurationEl {}

impl BuildDatasyncLocationHdfsQopConfigurationEl {
    pub fn build(self) -> DatasyncLocationHdfsQopConfigurationEl {
        DatasyncLocationHdfsQopConfigurationEl {
            data_transfer_protection: core::default::Default::default(),
            rpc_protection: core::default::Default::default(),
        }
    }
}

pub struct DatasyncLocationHdfsQopConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncLocationHdfsQopConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DatasyncLocationHdfsQopConfigurationElRef {
        DatasyncLocationHdfsQopConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncLocationHdfsQopConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_transfer_protection` after provisioning.\n"]
    pub fn data_transfer_protection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_transfer_protection", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `rpc_protection` after provisioning.\n"]
    pub fn rpc_protection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rpc_protection", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct DatasyncLocationHdfsDynamic {
    name_node: Option<DynamicBlock<DatasyncLocationHdfsNameNodeEl>>,
    qop_configuration: Option<DynamicBlock<DatasyncLocationHdfsQopConfigurationEl>>,
}
