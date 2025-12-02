use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct TimestreaminfluxdbDbInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    allocated_storage: PrimField<f64>,
    bucket: PrimField<String>,
    db_instance_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_parameter_group_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_storage_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_type: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_type: Option<PrimField<String>>,
    organization: PrimField<String>,
    password: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publicly_accessible: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    username: PrimField<String>,
    vpc_security_group_ids: SetField<PrimField<String>>,
    vpc_subnet_ids: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_delivery_configuration: Option<Vec<TimestreaminfluxdbDbInstanceLogDeliveryConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<TimestreaminfluxdbDbInstanceTimeoutsEl>,
    dynamic: TimestreaminfluxdbDbInstanceDynamic,
}
struct TimestreaminfluxdbDbInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TimestreaminfluxdbDbInstanceData>,
}
#[derive(Clone)]
pub struct TimestreaminfluxdbDbInstance(Rc<TimestreaminfluxdbDbInstance_>);
impl TimestreaminfluxdbDbInstance {
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
    #[doc = "Set the field `db_parameter_group_identifier`.\nThe id of the DB parameter group assigned to your DB instance."]
    pub fn set_db_parameter_group_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().db_parameter_group_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `db_storage_type`.\nThe Timestream for InfluxDB DB storage type to read and write InfluxDB data. \n\t\t\t\t\tYou can choose between 3 different types of provisioned Influx IOPS included storage according \n\t\t\t\t\tto your workloads requirements: Influx IO Included 3000 IOPS, Influx IO Included 12000 IOPS, \n\t\t\t\t\tInflux IO Included 16000 IOPS."]
    pub fn set_db_storage_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().db_storage_type = Some(v.into());
        self
    }
    #[doc = "Set the field `deployment_type`.\nSpecifies whether the DB instance will be deployed as a standalone instance or \n\t\t\t\t\twith a Multi-AZ standby for high availability."]
    pub fn set_deployment_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deployment_type = Some(v.into());
        self
    }
    #[doc = "Set the field `network_type`.\nSpecifies whether the networkType of the Timestream for InfluxDB instance is \n\t\t\t\t\tIPV4, which can communicate over IPv4 protocol only, or DUAL, which can communicate \n\t\t\t\t\tover both IPv4 and IPv6 protocols."]
    pub fn set_network_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network_type = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nThe port number on which InfluxDB accepts connections."]
    pub fn set_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = Some(v.into());
        self
    }
    #[doc = "Set the field `publicly_accessible`.\nConfigures the DB instance with a public IP to facilitate access."]
    pub fn set_publicly_accessible(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().publicly_accessible = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `log_delivery_configuration`.\n"]
    pub fn set_log_delivery_configuration(
        self,
        v: impl Into<BlockAssignable<TimestreaminfluxdbDbInstanceLogDeliveryConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_delivery_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_delivery_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<TimestreaminfluxdbDbInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `allocated_storage` after provisioning.\nThe amount of storage to allocate for your DB storage type in GiB (gibibytes)."]
    pub fn allocated_storage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocated_storage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\nThe Availability Zone in which the DB instance resides."]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe name of the initial InfluxDB bucket. All InfluxDB data is stored in a bucket. \n\t\t\t\t\tA bucket combines the concept of a database and a retention period (the duration of time \n\t\t\t\t\tthat each data point persists). A bucket belongs to an organization."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_instance_type` after provisioning.\nThe Timestream for InfluxDB DB instance type to run InfluxDB on."]
    pub fn db_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_instance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_parameter_group_identifier` after provisioning.\nThe id of the DB parameter group assigned to your DB instance."]
    pub fn db_parameter_group_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_parameter_group_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_storage_type` after provisioning.\nThe Timestream for InfluxDB DB storage type to read and write InfluxDB data. \n\t\t\t\t\tYou can choose between 3 different types of provisioned Influx IOPS included storage according \n\t\t\t\t\tto your workloads requirements: Influx IO Included 3000 IOPS, Influx IO Included 12000 IOPS, \n\t\t\t\t\tInflux IO Included 16000 IOPS."]
    pub fn db_storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_storage_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\nSpecifies whether the DB instance will be deployed as a standalone instance or \n\t\t\t\t\twith a Multi-AZ standby for high availability."]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe endpoint used to connect to InfluxDB. The default InfluxDB port is 8086."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `influx_auth_parameters_secret_arn` after provisioning.\nThe Amazon Resource Name (ARN) of the AWS Secrets Manager secret containing the \n\t\t\t\t\tinitial InfluxDB authorization parameters. The secret value is a JSON formatted \n\t\t\t\t\tkey-value pair holding InfluxDB authorization values: organization, bucket, \n\t\t\t\t\tusername, and password."]
    pub fn influx_auth_parameters_secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.influx_auth_parameters_secret_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name that uniquely identifies the DB instance when interacting with the \n\t\t\t\t\tAmazon Timestream for InfluxDB API and CLI commands. This name will also be a \n\t\t\t\t\tprefix included in the endpoint. DB instance names must be unique per customer \n\t\t\t\t\tand per region."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_type` after provisioning.\nSpecifies whether the networkType of the Timestream for InfluxDB instance is \n\t\t\t\t\tIPV4, which can communicate over IPv4 protocol only, or DUAL, which can communicate \n\t\t\t\t\tover both IPv4 and IPv6 protocols."]
    pub fn network_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nThe name of the initial organization for the initial admin user in InfluxDB. An \n\t\t\t\t\tInfluxDB organization is a workspace for a group of users."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nThe password of the initial admin user created in InfluxDB. This password will \n\t\t\t\t\tallow you to access the InfluxDB UI to perform various administrative tasks and \n\t\t\t\t\talso use the InfluxDB CLI to create an operator token. These attributes will be \n\t\t\t\t\tstored in a Secret created in AWS SecretManager in your account."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port number on which InfluxDB accepts connections."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\nConfigures the DB instance with a public IP to facilitate access."]
    pub fn publicly_accessible(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publicly_accessible", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_availability_zone` after provisioning.\nThe Availability Zone in which the standby instance is located when deploying \n\t\t\t\t\twith a MultiAZ standby instance."]
    pub fn secondary_availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secondary_availability_zone", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe username of the initial admin user created in InfluxDB. \n\t\t\t\t\tMust start with a letter and can't end with a hyphen or contain two \n\t\t\t\t\tconsecutive hyphens. For example, my-user1. This username will allow \n\t\t\t\t\tyou to access the InfluxDB UI to perform various administrative tasks \n\t\t\t\t\tand also use the InfluxDB CLI to create an operator token. These \n\t\t\t\t\tattributes will be stored in a Secret created in Amazon Secrets \n\t\t\t\t\tManager in your account"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\nA list of VPC security group IDs to associate with the DB instance."]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.vpc_security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_subnet_ids` after provisioning.\nA list of VPC subnet IDs to associate with the DB instance. Provide at least \n\t\t\t\t\ttwo VPC subnet IDs in different availability zones when deploying with a Multi-AZ standby."]
    pub fn vpc_subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.vpc_subnet_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_delivery_configuration` after provisioning.\n"]
    pub fn log_delivery_configuration(
        &self,
    ) -> ListRef<TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_delivery_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TimestreaminfluxdbDbInstanceTimeoutsElRef {
        TimestreaminfluxdbDbInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for TimestreaminfluxdbDbInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for TimestreaminfluxdbDbInstance {}
impl ToListMappable for TimestreaminfluxdbDbInstance {
    type O = ListRef<TimestreaminfluxdbDbInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for TimestreaminfluxdbDbInstance_ {
    fn extract_resource_type(&self) -> String {
        "aws_timestreaminfluxdb_db_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildTimestreaminfluxdbDbInstance {
    pub tf_id: String,
    #[doc = "The amount of storage to allocate for your DB storage type in GiB (gibibytes)."]
    pub allocated_storage: PrimField<f64>,
    #[doc = "The name of the initial InfluxDB bucket. All InfluxDB data is stored in a bucket. \n\t\t\t\t\tA bucket combines the concept of a database and a retention period (the duration of time \n\t\t\t\t\tthat each data point persists). A bucket belongs to an organization."]
    pub bucket: PrimField<String>,
    #[doc = "The Timestream for InfluxDB DB instance type to run InfluxDB on."]
    pub db_instance_type: PrimField<String>,
    #[doc = "The name that uniquely identifies the DB instance when interacting with the \n\t\t\t\t\tAmazon Timestream for InfluxDB API and CLI commands. This name will also be a \n\t\t\t\t\tprefix included in the endpoint. DB instance names must be unique per customer \n\t\t\t\t\tand per region."]
    pub name: PrimField<String>,
    #[doc = "The name of the initial organization for the initial admin user in InfluxDB. An \n\t\t\t\t\tInfluxDB organization is a workspace for a group of users."]
    pub organization: PrimField<String>,
    #[doc = "The password of the initial admin user created in InfluxDB. This password will \n\t\t\t\t\tallow you to access the InfluxDB UI to perform various administrative tasks and \n\t\t\t\t\talso use the InfluxDB CLI to create an operator token. These attributes will be \n\t\t\t\t\tstored in a Secret created in AWS SecretManager in your account."]
    pub password: PrimField<String>,
    #[doc = "The username of the initial admin user created in InfluxDB. \n\t\t\t\t\tMust start with a letter and can't end with a hyphen or contain two \n\t\t\t\t\tconsecutive hyphens. For example, my-user1. This username will allow \n\t\t\t\t\tyou to access the InfluxDB UI to perform various administrative tasks \n\t\t\t\t\tand also use the InfluxDB CLI to create an operator token. These \n\t\t\t\t\tattributes will be stored in a Secret created in Amazon Secrets \n\t\t\t\t\tManager in your account"]
    pub username: PrimField<String>,
    #[doc = "A list of VPC security group IDs to associate with the DB instance."]
    pub vpc_security_group_ids: SetField<PrimField<String>>,
    #[doc = "A list of VPC subnet IDs to associate with the DB instance. Provide at least \n\t\t\t\t\ttwo VPC subnet IDs in different availability zones when deploying with a Multi-AZ standby."]
    pub vpc_subnet_ids: SetField<PrimField<String>>,
}
impl BuildTimestreaminfluxdbDbInstance {
    pub fn build(self, stack: &mut Stack) -> TimestreaminfluxdbDbInstance {
        let out = TimestreaminfluxdbDbInstance(Rc::new(TimestreaminfluxdbDbInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TimestreaminfluxdbDbInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                allocated_storage: self.allocated_storage,
                bucket: self.bucket,
                db_instance_type: self.db_instance_type,
                db_parameter_group_identifier: core::default::Default::default(),
                db_storage_type: core::default::Default::default(),
                deployment_type: core::default::Default::default(),
                name: self.name,
                network_type: core::default::Default::default(),
                organization: self.organization,
                password: self.password,
                port: core::default::Default::default(),
                publicly_accessible: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                username: self.username,
                vpc_security_group_ids: self.vpc_security_group_ids,
                vpc_subnet_ids: self.vpc_subnet_ids,
                log_delivery_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct TimestreaminfluxdbDbInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for TimestreaminfluxdbDbInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl TimestreaminfluxdbDbInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allocated_storage` after provisioning.\nThe amount of storage to allocate for your DB storage type in GiB (gibibytes)."]
    pub fn allocated_storage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocated_storage", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\nThe Availability Zone in which the DB instance resides."]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe name of the initial InfluxDB bucket. All InfluxDB data is stored in a bucket. \n\t\t\t\t\tA bucket combines the concept of a database and a retention period (the duration of time \n\t\t\t\t\tthat each data point persists). A bucket belongs to an organization."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_instance_type` after provisioning.\nThe Timestream for InfluxDB DB instance type to run InfluxDB on."]
    pub fn db_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_instance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_parameter_group_identifier` after provisioning.\nThe id of the DB parameter group assigned to your DB instance."]
    pub fn db_parameter_group_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_parameter_group_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `db_storage_type` after provisioning.\nThe Timestream for InfluxDB DB storage type to read and write InfluxDB data. \n\t\t\t\t\tYou can choose between 3 different types of provisioned Influx IOPS included storage according \n\t\t\t\t\tto your workloads requirements: Influx IO Included 3000 IOPS, Influx IO Included 12000 IOPS, \n\t\t\t\t\tInflux IO Included 16000 IOPS."]
    pub fn db_storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_storage_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\nSpecifies whether the DB instance will be deployed as a standalone instance or \n\t\t\t\t\twith a Multi-AZ standby for high availability."]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe endpoint used to connect to InfluxDB. The default InfluxDB port is 8086."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `influx_auth_parameters_secret_arn` after provisioning.\nThe Amazon Resource Name (ARN) of the AWS Secrets Manager secret containing the \n\t\t\t\t\tinitial InfluxDB authorization parameters. The secret value is a JSON formatted \n\t\t\t\t\tkey-value pair holding InfluxDB authorization values: organization, bucket, \n\t\t\t\t\tusername, and password."]
    pub fn influx_auth_parameters_secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.influx_auth_parameters_secret_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name that uniquely identifies the DB instance when interacting with the \n\t\t\t\t\tAmazon Timestream for InfluxDB API and CLI commands. This name will also be a \n\t\t\t\t\tprefix included in the endpoint. DB instance names must be unique per customer \n\t\t\t\t\tand per region."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_type` after provisioning.\nSpecifies whether the networkType of the Timestream for InfluxDB instance is \n\t\t\t\t\tIPV4, which can communicate over IPv4 protocol only, or DUAL, which can communicate \n\t\t\t\t\tover both IPv4 and IPv6 protocols."]
    pub fn network_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nThe name of the initial organization for the initial admin user in InfluxDB. An \n\t\t\t\t\tInfluxDB organization is a workspace for a group of users."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\nThe password of the initial admin user created in InfluxDB. This password will \n\t\t\t\t\tallow you to access the InfluxDB UI to perform various administrative tasks and \n\t\t\t\t\talso use the InfluxDB CLI to create an operator token. These attributes will be \n\t\t\t\t\tstored in a Secret created in AWS SecretManager in your account."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port number on which InfluxDB accepts connections."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\nConfigures the DB instance with a public IP to facilitate access."]
    pub fn publicly_accessible(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.publicly_accessible", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_availability_zone` after provisioning.\nThe Availability Zone in which the standby instance is located when deploying \n\t\t\t\t\twith a MultiAZ standby instance."]
    pub fn secondary_availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secondary_availability_zone", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe username of the initial admin user created in InfluxDB. \n\t\t\t\t\tMust start with a letter and can't end with a hyphen or contain two \n\t\t\t\t\tconsecutive hyphens. For example, my-user1. This username will allow \n\t\t\t\t\tyou to access the InfluxDB UI to perform various administrative tasks \n\t\t\t\t\tand also use the InfluxDB CLI to create an operator token. These \n\t\t\t\t\tattributes will be stored in a Secret created in Amazon Secrets \n\t\t\t\t\tManager in your account"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.username", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\nA list of VPC security group IDs to associate with the DB instance."]
    pub fn vpc_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.vpc_security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_subnet_ids` after provisioning.\nA list of VPC subnet IDs to associate with the DB instance. Provide at least \n\t\t\t\t\ttwo VPC subnet IDs in different availability zones when deploying with a Multi-AZ standby."]
    pub fn vpc_subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.vpc_subnet_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_delivery_configuration` after provisioning.\n"]
    pub fn log_delivery_configuration(
        &self,
    ) -> ListRef<TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_delivery_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TimestreaminfluxdbDbInstanceTimeoutsElRef {
        TimestreaminfluxdbDbInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElS3ConfigurationEl {
    bucket_name: PrimField<String>,
    enabled: PrimField<bool>,
}
impl TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElS3ConfigurationEl {}
impl ToListMappable for TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElS3ConfigurationEl {
    type O =
        BlockAssignable<TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElS3ConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTimestreaminfluxdbDbInstanceLogDeliveryConfigurationElS3ConfigurationEl {
    #[doc = "The name of the S3 bucket to deliver logs to."]
    pub bucket_name: PrimField<String>,
    #[doc = "Indicates whether log delivery to the S3 bucket is enabled."]
    pub enabled: PrimField<bool>,
}
impl BuildTimestreaminfluxdbDbInstanceLogDeliveryConfigurationElS3ConfigurationEl {
    pub fn build(self) -> TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElS3ConfigurationEl {
        TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElS3ConfigurationEl {
            bucket_name: self.bucket_name,
            enabled: self.enabled,
        }
    }
}
pub struct TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElS3ConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElS3ConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElS3ConfigurationElRef {
        TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElS3ConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElS3ConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nThe name of the S3 bucket to deliver logs to."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nIndicates whether log delivery to the S3 bucket is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElDynamic {
    s3_configuration: Option<
        DynamicBlock<TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElS3ConfigurationEl>,
    >,
}
#[derive(Serialize)]
pub struct TimestreaminfluxdbDbInstanceLogDeliveryConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_configuration:
        Option<Vec<TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElS3ConfigurationEl>>,
    dynamic: TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElDynamic,
}
impl TimestreaminfluxdbDbInstanceLogDeliveryConfigurationEl {
    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElS3ConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for TimestreaminfluxdbDbInstanceLogDeliveryConfigurationEl {
    type O = BlockAssignable<TimestreaminfluxdbDbInstanceLogDeliveryConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTimestreaminfluxdbDbInstanceLogDeliveryConfigurationEl {}
impl BuildTimestreaminfluxdbDbInstanceLogDeliveryConfigurationEl {
    pub fn build(self) -> TimestreaminfluxdbDbInstanceLogDeliveryConfigurationEl {
        TimestreaminfluxdbDbInstanceLogDeliveryConfigurationEl {
            s3_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElRef {
        TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> ListRef<TimestreaminfluxdbDbInstanceLogDeliveryConfigurationElS3ConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_configuration", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct TimestreaminfluxdbDbInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl TimestreaminfluxdbDbInstanceTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for TimestreaminfluxdbDbInstanceTimeoutsEl {
    type O = BlockAssignable<TimestreaminfluxdbDbInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildTimestreaminfluxdbDbInstanceTimeoutsEl {}
impl BuildTimestreaminfluxdbDbInstanceTimeoutsEl {
    pub fn build(self) -> TimestreaminfluxdbDbInstanceTimeoutsEl {
        TimestreaminfluxdbDbInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct TimestreaminfluxdbDbInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for TimestreaminfluxdbDbInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> TimestreaminfluxdbDbInstanceTimeoutsElRef {
        TimestreaminfluxdbDbInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl TimestreaminfluxdbDbInstanceTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct TimestreaminfluxdbDbInstanceDynamic {
    log_delivery_configuration:
        Option<DynamicBlock<TimestreaminfluxdbDbInstanceLogDeliveryConfigurationEl>>,
}
