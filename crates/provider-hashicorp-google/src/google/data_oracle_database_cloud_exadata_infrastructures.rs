use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataOracleDatabaseCloudExadataInfrastructuresData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataOracleDatabaseCloudExadataInfrastructures_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOracleDatabaseCloudExadataInfrastructuresData>,
}
#[derive(Clone)]
pub struct DataOracleDatabaseCloudExadataInfrastructures(
    Rc<DataOracleDatabaseCloudExadataInfrastructures_>,
);
impl DataOracleDatabaseCloudExadataInfrastructures {
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
    #[doc = "Set the field `project`.\nThe ID of the project in which the dataset is located. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `cloud_exadata_infrastructures` after provisioning.\n"]
    pub fn cloud_exadata_infrastructures(
        &self,
    ) -> ListRef<DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructures", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nlocation"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the dataset is located. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
impl Referable for DataOracleDatabaseCloudExadataInfrastructures {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataOracleDatabaseCloudExadataInfrastructures {}
impl ToListMappable for DataOracleDatabaseCloudExadataInfrastructures {
    type O = ListRef<DataOracleDatabaseCloudExadataInfrastructuresRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataOracleDatabaseCloudExadataInfrastructures_ {
    fn extract_datasource_type(&self) -> String {
        "google_oracle_database_cloud_exadata_infrastructures".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataOracleDatabaseCloudExadataInfrastructures {
    pub tf_id: String,
    #[doc = "location"]
    pub location: PrimField<String>,
}
impl BuildDataOracleDatabaseCloudExadataInfrastructures {
    pub fn build(self, stack: &mut Stack) -> DataOracleDatabaseCloudExadataInfrastructures {
        let out = DataOracleDatabaseCloudExadataInfrastructures(Rc::new(
            DataOracleDatabaseCloudExadataInfrastructures_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataOracleDatabaseCloudExadataInfrastructuresData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    location: self.location,
                    project: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataOracleDatabaseCloudExadataInfrastructuresRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseCloudExadataInfrastructuresRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataOracleDatabaseCloudExadataInfrastructuresRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `cloud_exadata_infrastructures` after provisioning.\n"]
    pub fn cloud_exadata_infrastructures(
        &self,
    ) -> ListRef<DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructures", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nlocation"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the dataset is located. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElCustomerContactsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
}
impl DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElCustomerContactsEl { # [doc = "Set the field `email`.\n"] pub fn set_email (mut self , v : impl Into < PrimField < String > >) -> Self { self . email = Some (v . into ()) ; self } }
impl ToListMappable for DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElCustomerContactsEl { type O = BlockAssignable < DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElCustomerContactsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElCustomerContactsEl
{}
impl BuildDataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElCustomerContactsEl { pub fn build (self) -> DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElCustomerContactsEl { DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElCustomerContactsEl { email : core :: default :: Default :: default () , } } }
pub struct DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElCustomerContactsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElCustomerContactsElRef { fn new (shared : StackShared , base : String) -> DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElCustomerContactsElRef { DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElCustomerContactsElRef { shared : shared , base : base . to_string () , } } }
impl DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElCustomerContactsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `email` after provisioning.\n"] pub fn email (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.email" , self . base)) } }
#[derive(Serialize)]
pub struct DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElMaintenanceWindowEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_action_timeout_mins: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days_of_week: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hours_of_day: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_custom_action_timeout_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lead_time_week: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    months: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patching_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preference: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weeks_of_month: Option<ListField<PrimField<f64>>>,
}
impl DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElMaintenanceWindowEl { # [doc = "Set the field `custom_action_timeout_mins`.\n"] pub fn set_custom_action_timeout_mins (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . custom_action_timeout_mins = Some (v . into ()) ; self } # [doc = "Set the field `days_of_week`.\n"] pub fn set_days_of_week (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . days_of_week = Some (v . into ()) ; self } # [doc = "Set the field `hours_of_day`.\n"] pub fn set_hours_of_day (mut self , v : impl Into < ListField < PrimField < f64 > > >) -> Self { self . hours_of_day = Some (v . into ()) ; self } # [doc = "Set the field `is_custom_action_timeout_enabled`.\n"] pub fn set_is_custom_action_timeout_enabled (mut self , v : impl Into < PrimField < bool > >) -> Self { self . is_custom_action_timeout_enabled = Some (v . into ()) ; self } # [doc = "Set the field `lead_time_week`.\n"] pub fn set_lead_time_week (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . lead_time_week = Some (v . into ()) ; self } # [doc = "Set the field `months`.\n"] pub fn set_months (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . months = Some (v . into ()) ; self } # [doc = "Set the field `patching_mode`.\n"] pub fn set_patching_mode (mut self , v : impl Into < PrimField < String > >) -> Self { self . patching_mode = Some (v . into ()) ; self } # [doc = "Set the field `preference`.\n"] pub fn set_preference (mut self , v : impl Into < PrimField < String > >) -> Self { self . preference = Some (v . into ()) ; self } # [doc = "Set the field `weeks_of_month`.\n"] pub fn set_weeks_of_month (mut self , v : impl Into < ListField < PrimField < f64 > > >) -> Self { self . weeks_of_month = Some (v . into ()) ; self } }
impl ToListMappable for DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElMaintenanceWindowEl { type O = BlockAssignable < DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElMaintenanceWindowEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElMaintenanceWindowEl
{}
impl BuildDataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElMaintenanceWindowEl { pub fn build (self) -> DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElMaintenanceWindowEl { DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElMaintenanceWindowEl { custom_action_timeout_mins : core :: default :: Default :: default () , days_of_week : core :: default :: Default :: default () , hours_of_day : core :: default :: Default :: default () , is_custom_action_timeout_enabled : core :: default :: Default :: default () , lead_time_week : core :: default :: Default :: default () , months : core :: default :: Default :: default () , patching_mode : core :: default :: Default :: default () , preference : core :: default :: Default :: default () , weeks_of_month : core :: default :: Default :: default () , } } }
pub struct DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElMaintenanceWindowElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElMaintenanceWindowElRef { fn new (shared : StackShared , base : String) -> DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElMaintenanceWindowElRef { DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElMaintenanceWindowElRef { shared : shared , base : base . to_string () , } } }
impl DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElMaintenanceWindowElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `custom_action_timeout_mins` after provisioning.\n"] pub fn custom_action_timeout_mins (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.custom_action_timeout_mins" , self . base)) } # [doc = "Get a reference to the value of field `days_of_week` after provisioning.\n"] pub fn days_of_week (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.days_of_week" , self . base)) } # [doc = "Get a reference to the value of field `hours_of_day` after provisioning.\n"] pub fn hours_of_day (& self) -> ListRef < PrimExpr < f64 > > { ListRef :: new (self . shared () . clone () , format ! ("{}.hours_of_day" , self . base)) } # [doc = "Get a reference to the value of field `is_custom_action_timeout_enabled` after provisioning.\n"] pub fn is_custom_action_timeout_enabled (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.is_custom_action_timeout_enabled" , self . base)) } # [doc = "Get a reference to the value of field `lead_time_week` after provisioning.\n"] pub fn lead_time_week (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.lead_time_week" , self . base)) } # [doc = "Get a reference to the value of field `months` after provisioning.\n"] pub fn months (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.months" , self . base)) } # [doc = "Get a reference to the value of field `patching_mode` after provisioning.\n"] pub fn patching_mode (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.patching_mode" , self . base)) } # [doc = "Get a reference to the value of field `preference` after provisioning.\n"] pub fn preference (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.preference" , self . base)) } # [doc = "Get a reference to the value of field `weeks_of_month` after provisioning.\n"] pub fn weeks_of_month (& self) -> ListRef < PrimExpr < f64 > > { ListRef :: new (self . shared () . clone () , format ! ("{}.weeks_of_month" , self . base)) } }
#[derive(Serialize)]
pub struct DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesEl { # [serde (skip_serializing_if = "Option::is_none")] activated_storage_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] additional_storage_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] available_storage_size_gb : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] compute_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] cpu_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] customer_contacts : Option < ListField < DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElCustomerContactsEl > > , # [serde (skip_serializing_if = "Option::is_none")] data_storage_size_tb : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] db_node_storage_size_gb : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] db_server_version : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] maintenance_window : Option < ListField < DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElMaintenanceWindowEl > > , # [serde (skip_serializing_if = "Option::is_none")] max_cpu_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] max_data_storage_tb : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] max_db_node_storage_size_gb : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] max_memory_gb : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] memory_size_gb : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] monthly_db_server_version : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] monthly_storage_server_version : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] next_maintenance_run_id : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] next_maintenance_run_time : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] next_security_maintenance_run_time : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] oci_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] ocid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] shape : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] state : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] storage_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] storage_server_version : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] total_storage_size_gb : Option < PrimField < f64 > > , }
impl DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesEl {
    #[doc = "Set the field `activated_storage_count`.\n"]
    pub fn set_activated_storage_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.activated_storage_count = Some(v.into());
        self
    }
    #[doc = "Set the field `additional_storage_count`.\n"]
    pub fn set_additional_storage_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.additional_storage_count = Some(v.into());
        self
    }
    #[doc = "Set the field `available_storage_size_gb`.\n"]
    pub fn set_available_storage_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.available_storage_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `compute_count`.\n"]
    pub fn set_compute_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.compute_count = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_count`.\n"]
    pub fn set_cpu_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_count = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_contacts`.\n"]
    pub fn set_customer_contacts(
        mut self,
        v : impl Into < ListField < DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElCustomerContactsEl > >,
    ) -> Self {
        self.customer_contacts = Some(v.into());
        self
    }
    #[doc = "Set the field `data_storage_size_tb`.\n"]
    pub fn set_data_storage_size_tb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.data_storage_size_tb = Some(v.into());
        self
    }
    #[doc = "Set the field `db_node_storage_size_gb`.\n"]
    pub fn set_db_node_storage_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.db_node_storage_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `db_server_version`.\n"]
    pub fn set_db_server_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_server_version = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        mut self,
        v : impl Into < ListField < DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElMaintenanceWindowEl > >,
    ) -> Self {
        self.maintenance_window = Some(v.into());
        self
    }
    #[doc = "Set the field `max_cpu_count`.\n"]
    pub fn set_max_cpu_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_cpu_count = Some(v.into());
        self
    }
    #[doc = "Set the field `max_data_storage_tb`.\n"]
    pub fn set_max_data_storage_tb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_data_storage_tb = Some(v.into());
        self
    }
    #[doc = "Set the field `max_db_node_storage_size_gb`.\n"]
    pub fn set_max_db_node_storage_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_db_node_storage_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `max_memory_gb`.\n"]
    pub fn set_max_memory_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_memory_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_size_gb`.\n"]
    pub fn set_memory_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `monthly_db_server_version`.\n"]
    pub fn set_monthly_db_server_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.monthly_db_server_version = Some(v.into());
        self
    }
    #[doc = "Set the field `monthly_storage_server_version`.\n"]
    pub fn set_monthly_storage_server_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.monthly_storage_server_version = Some(v.into());
        self
    }
    #[doc = "Set the field `next_maintenance_run_id`.\n"]
    pub fn set_next_maintenance_run_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_maintenance_run_id = Some(v.into());
        self
    }
    #[doc = "Set the field `next_maintenance_run_time`.\n"]
    pub fn set_next_maintenance_run_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_maintenance_run_time = Some(v.into());
        self
    }
    #[doc = "Set the field `next_security_maintenance_run_time`.\n"]
    pub fn set_next_security_maintenance_run_time(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.next_security_maintenance_run_time = Some(v.into());
        self
    }
    #[doc = "Set the field `oci_url`.\n"]
    pub fn set_oci_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oci_url = Some(v.into());
        self
    }
    #[doc = "Set the field `ocid`.\n"]
    pub fn set_ocid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ocid = Some(v.into());
        self
    }
    #[doc = "Set the field `shape`.\n"]
    pub fn set_shape(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.shape = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_count`.\n"]
    pub fn set_storage_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_count = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_server_version`.\n"]
    pub fn set_storage_server_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_server_version = Some(v.into());
        self
    }
    #[doc = "Set the field `total_storage_size_gb`.\n"]
    pub fn set_total_storage_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_storage_size_gb = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesEl
{
    type O = BlockAssignable<
        DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesEl
{}
impl BuildDataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesEl {
    pub fn build(
        self,
    ) -> DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesEl
    {
        DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesEl {
            activated_storage_count: core::default::Default::default(),
            additional_storage_count: core::default::Default::default(),
            available_storage_size_gb: core::default::Default::default(),
            compute_count: core::default::Default::default(),
            cpu_count: core::default::Default::default(),
            customer_contacts: core::default::Default::default(),
            data_storage_size_tb: core::default::Default::default(),
            db_node_storage_size_gb: core::default::Default::default(),
            db_server_version: core::default::Default::default(),
            maintenance_window: core::default::Default::default(),
            max_cpu_count: core::default::Default::default(),
            max_data_storage_tb: core::default::Default::default(),
            max_db_node_storage_size_gb: core::default::Default::default(),
            max_memory_gb: core::default::Default::default(),
            memory_size_gb: core::default::Default::default(),
            monthly_db_server_version: core::default::Default::default(),
            monthly_storage_server_version: core::default::Default::default(),
            next_maintenance_run_id: core::default::Default::default(),
            next_maintenance_run_time: core::default::Default::default(),
            next_security_maintenance_run_time: core::default::Default::default(),
            oci_url: core::default::Default::default(),
            ocid: core::default::Default::default(),
            shape: core::default::Default::default(),
            state: core::default::Default::default(),
            storage_count: core::default::Default::default(),
            storage_server_version: core::default::Default::default(),
            total_storage_size_gb: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElRef
    {
        DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `activated_storage_count` after provisioning.\n"]
    pub fn activated_storage_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activated_storage_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `additional_storage_count` after provisioning.\n"]
    pub fn additional_storage_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_storage_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `available_storage_size_gb` after provisioning.\n"]
    pub fn available_storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.available_storage_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `compute_count` after provisioning.\n"]
    pub fn compute_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_count` after provisioning.\n"]
    pub fn cpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.base))
    }
    #[doc = "Get a reference to the value of field `customer_contacts` after provisioning.\n"]    pub fn customer_contacts (& self) -> ListRef < DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElCustomerContactsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_contacts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_storage_size_tb` after provisioning.\n"]
    pub fn data_storage_size_tb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_storage_size_tb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `db_node_storage_size_gb` after provisioning.\n"]
    pub fn db_node_storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_node_storage_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `db_server_version` after provisioning.\n"]
    pub fn db_server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_server_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]    pub fn maintenance_window (& self) -> ListRef < DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElMaintenanceWindowElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.maintenance_window", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_cpu_count` after provisioning.\n"]
    pub fn max_cpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_cpu_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_data_storage_tb` after provisioning.\n"]
    pub fn max_data_storage_tb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_data_storage_tb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_db_node_storage_size_gb` after provisioning.\n"]
    pub fn max_db_node_storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_db_node_storage_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_memory_gb` after provisioning.\n"]
    pub fn max_memory_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_memory_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_size_gb` after provisioning.\n"]
    pub fn memory_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `monthly_db_server_version` after provisioning.\n"]
    pub fn monthly_db_server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monthly_db_server_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `monthly_storage_server_version` after provisioning.\n"]
    pub fn monthly_storage_server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monthly_storage_server_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_maintenance_run_id` after provisioning.\n"]
    pub fn next_maintenance_run_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_maintenance_run_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_maintenance_run_time` after provisioning.\n"]
    pub fn next_maintenance_run_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_maintenance_run_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_security_maintenance_run_time` after provisioning.\n"]
    pub fn next_security_maintenance_run_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_security_maintenance_run_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\n"]
    pub fn oci_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.base))
    }
    #[doc = "Get a reference to the value of field `ocid` after provisioning.\n"]
    pub fn ocid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }
    #[doc = "Get a reference to the value of field `shape` after provisioning.\n"]
    pub fn shape(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shape", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_count` after provisioning.\n"]
    pub fn storage_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_server_version` after provisioning.\n"]
    pub fn storage_server_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_server_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_storage_size_gb` after provisioning.\n"]
    pub fn total_storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_storage_size_gb", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_exadata_infrastructure_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entitlement_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_oracle_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<
        ListField<
            DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    terraform_labels: Option<RecField<PrimField<String>>>,
}
impl DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresEl {
    #[doc = "Set the field `cloud_exadata_infrastructure_id`.\n"]
    pub fn set_cloud_exadata_infrastructure_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloud_exadata_infrastructure_id = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\n"]
    pub fn set_deletion_protection(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `effective_labels`.\n"]
    pub fn set_effective_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.effective_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `entitlement_id`.\n"]
    pub fn set_entitlement_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entitlement_id = Some(v.into());
        self
    }
    #[doc = "Set the field `gcp_oracle_zone`.\n"]
    pub fn set_gcp_oracle_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_oracle_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v : impl Into < ListField < DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesEl > >,
    ) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `terraform_labels`.\n"]
    pub fn set_terraform_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.terraform_labels = Some(v.into());
        self
    }
}
impl ToListMappable for DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresEl {
    type O =
        BlockAssignable<DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresEl {}
impl BuildDataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresEl {
    pub fn build(
        self,
    ) -> DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresEl {
        DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresEl {
            cloud_exadata_infrastructure_id: core::default::Default::default(),
            create_time: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            deletion_protection: core::default::Default::default(),
            display_name: core::default::Default::default(),
            effective_labels: core::default::Default::default(),
            entitlement_id: core::default::Default::default(),
            gcp_oracle_zone: core::default::Default::default(),
            labels: core::default::Default::default(),
            location: core::default::Default::default(),
            name: core::default::Default::default(),
            project: core::default::Default::default(),
            properties: core::default::Default::default(),
            terraform_labels: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElRef {
        DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\n"]
    pub fn cloud_exadata_infrastructure_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\n"]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `entitlement_id` after provisioning.\n"]
    pub fn entitlement_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entitlement_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\n"]
    pub fn gcp_oracle_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gcp_oracle_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(
        &self,
    ) -> ListRef<
        DataOracleDatabaseCloudExadataInfrastructuresCloudExadataInfrastructuresElPropertiesElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\n"]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.base),
        )
    }
}
