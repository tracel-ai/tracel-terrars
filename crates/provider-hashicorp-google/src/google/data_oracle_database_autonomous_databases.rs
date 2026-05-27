use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataOracleDatabaseAutonomousDatabasesData {
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
struct DataOracleDatabaseAutonomousDatabases_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOracleDatabaseAutonomousDatabasesData>,
}
#[derive(Clone)]
pub struct DataOracleDatabaseAutonomousDatabases(Rc<DataOracleDatabaseAutonomousDatabases_>);
impl DataOracleDatabaseAutonomousDatabases {
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
    #[doc = "Get a reference to the value of field `autonomous_databases` after provisioning.\n"]
    pub fn autonomous_databases(
        &self,
    ) -> ListRef<DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autonomous_databases", self.extract_ref()),
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
impl Referable for DataOracleDatabaseAutonomousDatabases {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataOracleDatabaseAutonomousDatabases {}
impl ToListMappable for DataOracleDatabaseAutonomousDatabases {
    type O = ListRef<DataOracleDatabaseAutonomousDatabasesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataOracleDatabaseAutonomousDatabases_ {
    fn extract_datasource_type(&self) -> String {
        "google_oracle_database_autonomous_databases".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataOracleDatabaseAutonomousDatabases {
    pub tf_id: String,
    #[doc = "location"]
    pub location: PrimField<String>,
}
impl BuildDataOracleDatabaseAutonomousDatabases {
    pub fn build(self, stack: &mut Stack) -> DataOracleDatabaseAutonomousDatabases {
        let out = DataOracleDatabaseAutonomousDatabases(Rc::new(
            DataOracleDatabaseAutonomousDatabases_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataOracleDatabaseAutonomousDatabasesData {
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
pub struct DataOracleDatabaseAutonomousDatabasesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseAutonomousDatabasesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataOracleDatabaseAutonomousDatabasesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `autonomous_databases` after provisioning.\n"]
    pub fn autonomous_databases(
        &self,
    ) -> ListRef<DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autonomous_databases", self.extract_ref()),
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
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElApexDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    apex_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ords_version: Option<PrimField<String>>,
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElApexDetailsEl {
    #[doc = "Set the field `apex_version`.\n"]
    pub fn set_apex_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.apex_version = Some(v.into());
        self
    }
    #[doc = "Set the field `ords_version`.\n"]
    pub fn set_ords_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ords_version = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElApexDetailsEl
{
    type O = BlockAssignable<
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElApexDetailsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElApexDetailsEl
{}
impl BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElApexDetailsEl {
    pub fn build(
        self,
    ) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElApexDetailsEl {
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElApexDetailsEl {
            apex_version: core::default::Default::default(),
            ords_version: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElApexDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElApexDetailsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElApexDetailsElRef
    {
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElApexDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElApexDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `apex_version` after provisioning.\n"]
    pub fn apex_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.apex_version", self.base))
    }
    #[doc = "Get a reference to the value of field `ords_version` after provisioning.\n"]
    pub fn ords_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ords_version", self.base))
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElAllConnectionStringsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    high: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    low: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    medium: Option<PrimField<String>>,
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElAllConnectionStringsEl { # [doc = "Set the field `high`.\n"] pub fn set_high (mut self , v : impl Into < PrimField < String > >) -> Self { self . high = Some (v . into ()) ; self } # [doc = "Set the field `low`.\n"] pub fn set_low (mut self , v : impl Into < PrimField < String > >) -> Self { self . low = Some (v . into ()) ; self } # [doc = "Set the field `medium`.\n"] pub fn set_medium (mut self , v : impl Into < PrimField < String > >) -> Self { self . medium = Some (v . into ()) ; self } }
impl ToListMappable for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElAllConnectionStringsEl { type O = BlockAssignable < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElAllConnectionStringsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElAllConnectionStringsEl
{}
impl BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElAllConnectionStringsEl { pub fn build (self) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElAllConnectionStringsEl { DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElAllConnectionStringsEl { high : core :: default :: Default :: default () , low : core :: default :: Default :: default () , medium : core :: default :: Default :: default () , } } }
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElAllConnectionStringsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElAllConnectionStringsElRef { fn new (shared : StackShared , base : String) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElAllConnectionStringsElRef { DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElAllConnectionStringsElRef { shared : shared , base : base . to_string () , } } }
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElAllConnectionStringsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `high` after provisioning.\n"] pub fn high (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.high" , self . base)) } # [doc = "Get a reference to the value of field `low` after provisioning.\n"] pub fn low (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.low" , self . base)) } # [doc = "Get a reference to the value of field `medium` after provisioning.\n"] pub fn medium (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.medium" , self . base)) } }
#[derive(Serialize)]
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElProfilesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_regional: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    syntax_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_authentication: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElProfilesEl { # [doc = "Set the field `consumer_group`.\n"] pub fn set_consumer_group (mut self , v : impl Into < PrimField < String > >) -> Self { self . consumer_group = Some (v . into ()) ; self } # [doc = "Set the field `display_name`.\n"] pub fn set_display_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . display_name = Some (v . into ()) ; self } # [doc = "Set the field `host_format`.\n"] pub fn set_host_format (mut self , v : impl Into < PrimField < String > >) -> Self { self . host_format = Some (v . into ()) ; self } # [doc = "Set the field `is_regional`.\n"] pub fn set_is_regional (mut self , v : impl Into < PrimField < bool > >) -> Self { self . is_regional = Some (v . into ()) ; self } # [doc = "Set the field `protocol`.\n"] pub fn set_protocol (mut self , v : impl Into < PrimField < String > >) -> Self { self . protocol = Some (v . into ()) ; self } # [doc = "Set the field `session_mode`.\n"] pub fn set_session_mode (mut self , v : impl Into < PrimField < String > >) -> Self { self . session_mode = Some (v . into ()) ; self } # [doc = "Set the field `syntax_format`.\n"] pub fn set_syntax_format (mut self , v : impl Into < PrimField < String > >) -> Self { self . syntax_format = Some (v . into ()) ; self } # [doc = "Set the field `tls_authentication`.\n"] pub fn set_tls_authentication (mut self , v : impl Into < PrimField < String > >) -> Self { self . tls_authentication = Some (v . into ()) ; self } # [doc = "Set the field `value`.\n"] pub fn set_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . value = Some (v . into ()) ; self } }
impl ToListMappable for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElProfilesEl { type O = BlockAssignable < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElProfilesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElProfilesEl
{}
impl BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElProfilesEl { pub fn build (self) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElProfilesEl { DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElProfilesEl { consumer_group : core :: default :: Default :: default () , display_name : core :: default :: Default :: default () , host_format : core :: default :: Default :: default () , is_regional : core :: default :: Default :: default () , protocol : core :: default :: Default :: default () , session_mode : core :: default :: Default :: default () , syntax_format : core :: default :: Default :: default () , tls_authentication : core :: default :: Default :: default () , value : core :: default :: Default :: default () , } } }
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElProfilesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElProfilesElRef { fn new (shared : StackShared , base : String) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElProfilesElRef { DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElProfilesElRef { shared : shared , base : base . to_string () , } } }
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElProfilesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `consumer_group` after provisioning.\n"] pub fn consumer_group (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.consumer_group" , self . base)) } # [doc = "Get a reference to the value of field `display_name` after provisioning.\n"] pub fn display_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.display_name" , self . base)) } # [doc = "Get a reference to the value of field `host_format` after provisioning.\n"] pub fn host_format (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.host_format" , self . base)) } # [doc = "Get a reference to the value of field `is_regional` after provisioning.\n"] pub fn is_regional (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.is_regional" , self . base)) } # [doc = "Get a reference to the value of field `protocol` after provisioning.\n"] pub fn protocol (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.protocol" , self . base)) } # [doc = "Get a reference to the value of field `session_mode` after provisioning.\n"] pub fn session_mode (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.session_mode" , self . base)) } # [doc = "Get a reference to the value of field `syntax_format` after provisioning.\n"] pub fn syntax_format (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.syntax_format" , self . base)) } # [doc = "Get a reference to the value of field `tls_authentication` after provisioning.\n"] pub fn tls_authentication (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.tls_authentication" , self . base)) } # [doc = "Get a reference to the value of field `value` after provisioning.\n"] pub fn value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.value" , self . base)) } }
#[derive(Serialize)]
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsEl { # [serde (skip_serializing_if = "Option::is_none")] all_connection_strings : Option < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElAllConnectionStringsEl > > , # [serde (skip_serializing_if = "Option::is_none")] dedicated : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] high : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] low : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] medium : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] profiles : Option < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElProfilesEl > > , }
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsEl {
    #[doc = "Set the field `all_connection_strings`.\n"]
    pub fn set_all_connection_strings(
        mut self,
        v : impl Into < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElAllConnectionStringsEl > >,
    ) -> Self {
        self.all_connection_strings = Some(v.into());
        self
    }
    #[doc = "Set the field `dedicated`.\n"]
    pub fn set_dedicated(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dedicated = Some(v.into());
        self
    }
    #[doc = "Set the field `high`.\n"]
    pub fn set_high(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.high = Some(v.into());
        self
    }
    #[doc = "Set the field `low`.\n"]
    pub fn set_low(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.low = Some(v.into());
        self
    }
    #[doc = "Set the field `medium`.\n"]
    pub fn set_medium(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.medium = Some(v.into());
        self
    }
    #[doc = "Set the field `profiles`.\n"]
    pub fn set_profiles(
        mut self,
        v : impl Into < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElProfilesEl > >,
    ) -> Self {
        self.profiles = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsEl
{
    type O = BlockAssignable<
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsEl
{}
impl
    BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsEl
{
    pub fn build(
        self,
    ) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsEl
    {
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsEl {
            all_connection_strings: core::default::Default::default(),
            dedicated: core::default::Default::default(),
            high: core::default::Default::default(),
            low: core::default::Default::default(),
            medium: core::default::Default::default(),
            profiles: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElRef
    {
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all_connection_strings` after provisioning.\n"]    pub fn all_connection_strings (& self) -> ListRef < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElAllConnectionStringsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.all_connection_strings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dedicated` after provisioning.\n"]
    pub fn dedicated(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dedicated", self.base))
    }
    #[doc = "Get a reference to the value of field `high` after provisioning.\n"]
    pub fn high(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.high", self.base))
    }
    #[doc = "Get a reference to the value of field `low` after provisioning.\n"]
    pub fn low(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.low", self.base))
    }
    #[doc = "Get a reference to the value of field `medium` after provisioning.\n"]
    pub fn medium(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.medium", self.base))
    }
    #[doc = "Get a reference to the value of field `profiles` after provisioning.\n"]    pub fn profiles (& self) -> ListRef < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElProfilesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.profiles", self.base))
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionUrlsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    apex_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_transforms_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    graph_studio_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_learning_notebook_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_learning_user_management_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mongo_db_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ords_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_dev_web_uri: Option<PrimField<String>>,
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionUrlsEl {
    #[doc = "Set the field `apex_uri`.\n"]
    pub fn set_apex_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.apex_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `database_transforms_uri`.\n"]
    pub fn set_database_transforms_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.database_transforms_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `graph_studio_uri`.\n"]
    pub fn set_graph_studio_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.graph_studio_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_learning_notebook_uri`.\n"]
    pub fn set_machine_learning_notebook_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_learning_notebook_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_learning_user_management_uri`.\n"]
    pub fn set_machine_learning_user_management_uri(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.machine_learning_user_management_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `mongo_db_uri`.\n"]
    pub fn set_mongo_db_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mongo_db_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `ords_uri`.\n"]
    pub fn set_ords_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ords_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `sql_dev_web_uri`.\n"]
    pub fn set_sql_dev_web_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sql_dev_web_uri = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionUrlsEl
{
    type O = BlockAssignable<
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionUrlsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionUrlsEl
{}
impl BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionUrlsEl {
    pub fn build(
        self,
    ) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionUrlsEl
    {
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionUrlsEl {
            apex_uri: core::default::Default::default(),
            database_transforms_uri: core::default::Default::default(),
            graph_studio_uri: core::default::Default::default(),
            machine_learning_notebook_uri: core::default::Default::default(),
            machine_learning_user_management_uri: core::default::Default::default(),
            mongo_db_uri: core::default::Default::default(),
            ords_uri: core::default::Default::default(),
            sql_dev_web_uri: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionUrlsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionUrlsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionUrlsElRef
    {
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionUrlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionUrlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `apex_uri` after provisioning.\n"]
    pub fn apex_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.apex_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `database_transforms_uri` after provisioning.\n"]
    pub fn database_transforms_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_transforms_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `graph_studio_uri` after provisioning.\n"]
    pub fn graph_studio_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.graph_studio_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_learning_notebook_uri` after provisioning.\n"]
    pub fn machine_learning_notebook_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.machine_learning_notebook_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_learning_user_management_uri` after provisioning.\n"]
    pub fn machine_learning_user_management_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.machine_learning_user_management_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mongo_db_uri` after provisioning.\n"]
    pub fn mongo_db_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mongo_db_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `ords_uri` after provisioning.\n"]
    pub fn ords_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ords_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `sql_dev_web_uri` after provisioning.\n"]
    pub fn sql_dev_web_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sql_dev_web_uri", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElCustomerContactsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElCustomerContactsEl {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElCustomerContactsEl
{
    type O = BlockAssignable<
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElCustomerContactsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElCustomerContactsEl
{}
impl BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElCustomerContactsEl {
    pub fn build(
        self,
    ) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElCustomerContactsEl
    {
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElCustomerContactsEl {
            email: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElCustomerContactsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElCustomerContactsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElCustomerContactsElRef
    {
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElCustomerContactsElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElCustomerContactsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElLocalStandbyDbEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_guard_role_changed_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disaster_recovery_role_changed_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lag_time_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_details: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElLocalStandbyDbEl {
    #[doc = "Set the field `data_guard_role_changed_time`.\n"]
    pub fn set_data_guard_role_changed_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_guard_role_changed_time = Some(v.into());
        self
    }
    #[doc = "Set the field `disaster_recovery_role_changed_time`.\n"]
    pub fn set_disaster_recovery_role_changed_time(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.disaster_recovery_role_changed_time = Some(v.into());
        self
    }
    #[doc = "Set the field `lag_time_duration`.\n"]
    pub fn set_lag_time_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lag_time_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `lifecycle_details`.\n"]
    pub fn set_lifecycle_details(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_details = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElLocalStandbyDbEl
{
    type O = BlockAssignable<
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElLocalStandbyDbEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElLocalStandbyDbEl
{}
impl BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElLocalStandbyDbEl {
    pub fn build(
        self,
    ) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElLocalStandbyDbEl
    {
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElLocalStandbyDbEl {
            data_guard_role_changed_time: core::default::Default::default(),
            disaster_recovery_role_changed_time: core::default::Default::default(),
            lag_time_duration: core::default::Default::default(),
            lifecycle_details: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElLocalStandbyDbElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElLocalStandbyDbElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElLocalStandbyDbElRef
    {
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElLocalStandbyDbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElLocalStandbyDbElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_guard_role_changed_time` after provisioning.\n"]
    pub fn data_guard_role_changed_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_guard_role_changed_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disaster_recovery_role_changed_time` after provisioning.\n"]
    pub fn disaster_recovery_role_changed_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disaster_recovery_role_changed_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lag_time_duration` after provisioning.\n"]
    pub fn lag_time_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lag_time_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lifecycle_details` after provisioning.\n"]
    pub fn lifecycle_details(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_details", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStartTimeEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStartTimeEl { # [doc = "Set the field `hours`.\n"] pub fn set_hours (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . hours = Some (v . into ()) ; self } # [doc = "Set the field `minutes`.\n"] pub fn set_minutes (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . minutes = Some (v . into ()) ; self } # [doc = "Set the field `nanos`.\n"] pub fn set_nanos (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . nanos = Some (v . into ()) ; self } # [doc = "Set the field `seconds`.\n"] pub fn set_seconds (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . seconds = Some (v . into ()) ; self } }
impl ToListMappable for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStartTimeEl { type O = BlockAssignable < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStartTimeEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStartTimeEl
{}
impl BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStartTimeEl { pub fn build (self) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStartTimeEl { DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStartTimeEl { hours : core :: default :: Default :: default () , minutes : core :: default :: Default :: default () , nanos : core :: default :: Default :: default () , seconds : core :: default :: Default :: default () , } } }
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStartTimeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStartTimeElRef { fn new (shared : StackShared , base : String) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStartTimeElRef { DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStartTimeElRef { shared : shared , base : base . to_string () , } } }
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStartTimeElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `hours` after provisioning.\n"] pub fn hours (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.hours" , self . base)) } # [doc = "Get a reference to the value of field `minutes` after provisioning.\n"] pub fn minutes (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.minutes" , self . base)) } # [doc = "Get a reference to the value of field `nanos` after provisioning.\n"] pub fn nanos (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nanos" , self . base)) } # [doc = "Get a reference to the value of field `seconds` after provisioning.\n"] pub fn seconds (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.seconds" , self . base)) } }
#[derive(Serialize)]
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStopTimeEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStopTimeEl { # [doc = "Set the field `hours`.\n"] pub fn set_hours (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . hours = Some (v . into ()) ; self } # [doc = "Set the field `minutes`.\n"] pub fn set_minutes (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . minutes = Some (v . into ()) ; self } # [doc = "Set the field `nanos`.\n"] pub fn set_nanos (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . nanos = Some (v . into ()) ; self } # [doc = "Set the field `seconds`.\n"] pub fn set_seconds (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . seconds = Some (v . into ()) ; self } }
impl ToListMappable for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStopTimeEl { type O = BlockAssignable < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStopTimeEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStopTimeEl
{}
impl BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStopTimeEl { pub fn build (self) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStopTimeEl { DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStopTimeEl { hours : core :: default :: Default :: default () , minutes : core :: default :: Default :: default () , nanos : core :: default :: Default :: default () , seconds : core :: default :: Default :: default () , } } }
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStopTimeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStopTimeElRef { fn new (shared : StackShared , base : String) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStopTimeElRef { DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStopTimeElRef { shared : shared , base : base . to_string () , } } }
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStopTimeElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `hours` after provisioning.\n"] pub fn hours (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.hours" , self . base)) } # [doc = "Get a reference to the value of field `minutes` after provisioning.\n"] pub fn minutes (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.minutes" , self . base)) } # [doc = "Get a reference to the value of field `nanos` after provisioning.\n"] pub fn nanos (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nanos" , self . base)) } # [doc = "Get a reference to the value of field `seconds` after provisioning.\n"] pub fn seconds (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.seconds" , self . base)) } }
#[derive(Serialize)]
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsEl { # [serde (skip_serializing_if = "Option::is_none")] day_of_week : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] start_time : Option < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStartTimeEl > > , # [serde (skip_serializing_if = "Option::is_none")] stop_time : Option < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStopTimeEl > > , }
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsEl { # [doc = "Set the field `day_of_week`.\n"] pub fn set_day_of_week (mut self , v : impl Into < PrimField < String > >) -> Self { self . day_of_week = Some (v . into ()) ; self } # [doc = "Set the field `start_time`.\n"] pub fn set_start_time (mut self , v : impl Into < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStartTimeEl > >) -> Self { self . start_time = Some (v . into ()) ; self } # [doc = "Set the field `stop_time`.\n"] pub fn set_stop_time (mut self , v : impl Into < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStopTimeEl > >) -> Self { self . stop_time = Some (v . into ()) ; self } }
impl ToListMappable for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsEl { type O = BlockAssignable < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsEl
{}
impl BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsEl { pub fn build (self) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsEl { DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsEl { day_of_week : core :: default :: Default :: default () , start_time : core :: default :: Default :: default () , stop_time : core :: default :: Default :: default () , } } }
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElRef { fn new (shared : StackShared , base : String) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElRef { DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElRef { shared : shared , base : base . to_string () , } } }
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"] pub fn day_of_week (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.day_of_week" , self . base)) } # [doc = "Get a reference to the value of field `start_time` after provisioning.\n"] pub fn start_time (& self) -> ListRef < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStartTimeElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.start_time" , self . base)) } # [doc = "Get a reference to the value of field `stop_time` after provisioning.\n"] pub fn stop_time (& self) -> ListRef < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElStopTimeElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.stop_time" , self . base)) } }
#[derive(Serialize)]
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesEl { # [serde (skip_serializing_if = "Option::is_none")] actual_used_data_storage_size_tb : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] allocated_storage_size_tb : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] apex_details : Option < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElApexDetailsEl > > , # [serde (skip_serializing_if = "Option::is_none")] are_primary_allowlisted_ips_used : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] autonomous_container_database_id : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] available_upgrade_versions : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] backup_retention_period_days : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] character_set : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] compute_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] connection_strings : Option < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsEl > > , # [serde (skip_serializing_if = "Option::is_none")] connection_urls : Option < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionUrlsEl > > , # [serde (skip_serializing_if = "Option::is_none")] cpu_core_count : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] customer_contacts : Option < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElCustomerContactsEl > > , # [serde (skip_serializing_if = "Option::is_none")] data_safe_state : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] data_storage_size_gb : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] data_storage_size_tb : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] database_management_state : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] db_edition : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] db_version : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] db_workload : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] failed_data_recovery_duration : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] is_auto_scaling_enabled : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] is_local_data_guard_enabled : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] is_storage_auto_scaling_enabled : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] license_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] lifecycle_details : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] local_adg_auto_failover_max_data_loss_limit : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] local_disaster_recovery_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] local_standby_db : Option < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElLocalStandbyDbEl > > , # [serde (skip_serializing_if = "Option::is_none")] maintenance_begin_time : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] maintenance_end_time : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] maintenance_schedule_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] memory_per_oracle_compute_unit_gbs : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] memory_table_gbs : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] mtls_connection_required : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] n_character_set : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] next_long_term_backup_time : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] oci_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] ocid : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] open_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] operations_insights_state : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] peer_db_ids : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] permission_level : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] private_endpoint : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] private_endpoint_ip : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] private_endpoint_label : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] refreshable_mode : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] refreshable_state : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] role : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] scheduled_operation_details : Option < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsEl > > , # [serde (skip_serializing_if = "Option::is_none")] secret_id : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] sql_web_developer_url : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] state : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] supported_clone_regions : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] total_auto_backup_storage_size_gbs : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] used_data_storage_size_tbs : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] vault_id : Option < PrimField < String > > , }
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesEl {
    #[doc = "Set the field `actual_used_data_storage_size_tb`.\n"]
    pub fn set_actual_used_data_storage_size_tb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.actual_used_data_storage_size_tb = Some(v.into());
        self
    }
    #[doc = "Set the field `allocated_storage_size_tb`.\n"]
    pub fn set_allocated_storage_size_tb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.allocated_storage_size_tb = Some(v.into());
        self
    }
    #[doc = "Set the field `apex_details`.\n"]
    pub fn set_apex_details(
        mut self,
        v: impl Into<
            ListField<
                DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElApexDetailsEl,
            >,
        >,
    ) -> Self {
        self.apex_details = Some(v.into());
        self
    }
    #[doc = "Set the field `are_primary_allowlisted_ips_used`.\n"]
    pub fn set_are_primary_allowlisted_ips_used(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.are_primary_allowlisted_ips_used = Some(v.into());
        self
    }
    #[doc = "Set the field `autonomous_container_database_id`.\n"]
    pub fn set_autonomous_container_database_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.autonomous_container_database_id = Some(v.into());
        self
    }
    #[doc = "Set the field `available_upgrade_versions`.\n"]
    pub fn set_available_upgrade_versions(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.available_upgrade_versions = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_retention_period_days`.\n"]
    pub fn set_backup_retention_period_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.backup_retention_period_days = Some(v.into());
        self
    }
    #[doc = "Set the field `character_set`.\n"]
    pub fn set_character_set(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.character_set = Some(v.into());
        self
    }
    #[doc = "Set the field `compute_count`.\n"]
    pub fn set_compute_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.compute_count = Some(v.into());
        self
    }
    #[doc = "Set the field `connection_strings`.\n"]
    pub fn set_connection_strings(
        mut self,
        v : impl Into < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsEl > >,
    ) -> Self {
        self.connection_strings = Some(v.into());
        self
    }
    #[doc = "Set the field `connection_urls`.\n"]
    pub fn set_connection_urls(
        mut self,
        v : impl Into < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionUrlsEl > >,
    ) -> Self {
        self.connection_urls = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_core_count`.\n"]
    pub fn set_cpu_core_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_core_count = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_contacts`.\n"]
    pub fn set_customer_contacts(
        mut self,
        v : impl Into < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElCustomerContactsEl > >,
    ) -> Self {
        self.customer_contacts = Some(v.into());
        self
    }
    #[doc = "Set the field `data_safe_state`.\n"]
    pub fn set_data_safe_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_safe_state = Some(v.into());
        self
    }
    #[doc = "Set the field `data_storage_size_gb`.\n"]
    pub fn set_data_storage_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.data_storage_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `data_storage_size_tb`.\n"]
    pub fn set_data_storage_size_tb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.data_storage_size_tb = Some(v.into());
        self
    }
    #[doc = "Set the field `database_management_state`.\n"]
    pub fn set_database_management_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.database_management_state = Some(v.into());
        self
    }
    #[doc = "Set the field `db_edition`.\n"]
    pub fn set_db_edition(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_edition = Some(v.into());
        self
    }
    #[doc = "Set the field `db_version`.\n"]
    pub fn set_db_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_version = Some(v.into());
        self
    }
    #[doc = "Set the field `db_workload`.\n"]
    pub fn set_db_workload(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_workload = Some(v.into());
        self
    }
    #[doc = "Set the field `failed_data_recovery_duration`.\n"]
    pub fn set_failed_data_recovery_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.failed_data_recovery_duration = Some(v.into());
        self
    }
    #[doc = "Set the field `is_auto_scaling_enabled`.\n"]
    pub fn set_is_auto_scaling_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_auto_scaling_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `is_local_data_guard_enabled`.\n"]
    pub fn set_is_local_data_guard_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_local_data_guard_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `is_storage_auto_scaling_enabled`.\n"]
    pub fn set_is_storage_auto_scaling_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_storage_auto_scaling_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `license_type`.\n"]
    pub fn set_license_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.license_type = Some(v.into());
        self
    }
    #[doc = "Set the field `lifecycle_details`.\n"]
    pub fn set_lifecycle_details(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lifecycle_details = Some(v.into());
        self
    }
    #[doc = "Set the field `local_adg_auto_failover_max_data_loss_limit`.\n"]
    pub fn set_local_adg_auto_failover_max_data_loss_limit(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.local_adg_auto_failover_max_data_loss_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `local_disaster_recovery_type`.\n"]
    pub fn set_local_disaster_recovery_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.local_disaster_recovery_type = Some(v.into());
        self
    }
    #[doc = "Set the field `local_standby_db`.\n"]
    pub fn set_local_standby_db(
        mut self,
        v : impl Into < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElLocalStandbyDbEl > >,
    ) -> Self {
        self.local_standby_db = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_begin_time`.\n"]
    pub fn set_maintenance_begin_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.maintenance_begin_time = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_end_time`.\n"]
    pub fn set_maintenance_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.maintenance_end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_schedule_type`.\n"]
    pub fn set_maintenance_schedule_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.maintenance_schedule_type = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_per_oracle_compute_unit_gbs`.\n"]
    pub fn set_memory_per_oracle_compute_unit_gbs(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_per_oracle_compute_unit_gbs = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_table_gbs`.\n"]
    pub fn set_memory_table_gbs(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_table_gbs = Some(v.into());
        self
    }
    #[doc = "Set the field `mtls_connection_required`.\n"]
    pub fn set_mtls_connection_required(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.mtls_connection_required = Some(v.into());
        self
    }
    #[doc = "Set the field `n_character_set`.\n"]
    pub fn set_n_character_set(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.n_character_set = Some(v.into());
        self
    }
    #[doc = "Set the field `next_long_term_backup_time`.\n"]
    pub fn set_next_long_term_backup_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.next_long_term_backup_time = Some(v.into());
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
    #[doc = "Set the field `open_mode`.\n"]
    pub fn set_open_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.open_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `operations_insights_state`.\n"]
    pub fn set_operations_insights_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operations_insights_state = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_db_ids`.\n"]
    pub fn set_peer_db_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.peer_db_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `permission_level`.\n"]
    pub fn set_permission_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.permission_level = Some(v.into());
        self
    }
    #[doc = "Set the field `private_endpoint`.\n"]
    pub fn set_private_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `private_endpoint_ip`.\n"]
    pub fn set_private_endpoint_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_endpoint_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `private_endpoint_label`.\n"]
    pub fn set_private_endpoint_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_endpoint_label = Some(v.into());
        self
    }
    #[doc = "Set the field `refreshable_mode`.\n"]
    pub fn set_refreshable_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.refreshable_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `refreshable_state`.\n"]
    pub fn set_refreshable_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.refreshable_state = Some(v.into());
        self
    }
    #[doc = "Set the field `role`.\n"]
    pub fn set_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role = Some(v.into());
        self
    }
    #[doc = "Set the field `scheduled_operation_details`.\n"]
    pub fn set_scheduled_operation_details(
        mut self,
        v : impl Into < ListField < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsEl > >,
    ) -> Self {
        self.scheduled_operation_details = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_id`.\n"]
    pub fn set_secret_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_id = Some(v.into());
        self
    }
    #[doc = "Set the field `sql_web_developer_url`.\n"]
    pub fn set_sql_web_developer_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sql_web_developer_url = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `supported_clone_regions`.\n"]
    pub fn set_supported_clone_regions(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.supported_clone_regions = Some(v.into());
        self
    }
    #[doc = "Set the field `total_auto_backup_storage_size_gbs`.\n"]
    pub fn set_total_auto_backup_storage_size_gbs(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_auto_backup_storage_size_gbs = Some(v.into());
        self
    }
    #[doc = "Set the field `used_data_storage_size_tbs`.\n"]
    pub fn set_used_data_storage_size_tbs(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.used_data_storage_size_tbs = Some(v.into());
        self
    }
    #[doc = "Set the field `vault_id`.\n"]
    pub fn set_vault_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vault_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesEl {
    type O =
        BlockAssignable<DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesEl {}
impl BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesEl {
    pub fn build(self) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesEl {
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesEl {
            actual_used_data_storage_size_tb: core::default::Default::default(),
            allocated_storage_size_tb: core::default::Default::default(),
            apex_details: core::default::Default::default(),
            are_primary_allowlisted_ips_used: core::default::Default::default(),
            autonomous_container_database_id: core::default::Default::default(),
            available_upgrade_versions: core::default::Default::default(),
            backup_retention_period_days: core::default::Default::default(),
            character_set: core::default::Default::default(),
            compute_count: core::default::Default::default(),
            connection_strings: core::default::Default::default(),
            connection_urls: core::default::Default::default(),
            cpu_core_count: core::default::Default::default(),
            customer_contacts: core::default::Default::default(),
            data_safe_state: core::default::Default::default(),
            data_storage_size_gb: core::default::Default::default(),
            data_storage_size_tb: core::default::Default::default(),
            database_management_state: core::default::Default::default(),
            db_edition: core::default::Default::default(),
            db_version: core::default::Default::default(),
            db_workload: core::default::Default::default(),
            failed_data_recovery_duration: core::default::Default::default(),
            is_auto_scaling_enabled: core::default::Default::default(),
            is_local_data_guard_enabled: core::default::Default::default(),
            is_storage_auto_scaling_enabled: core::default::Default::default(),
            license_type: core::default::Default::default(),
            lifecycle_details: core::default::Default::default(),
            local_adg_auto_failover_max_data_loss_limit: core::default::Default::default(),
            local_disaster_recovery_type: core::default::Default::default(),
            local_standby_db: core::default::Default::default(),
            maintenance_begin_time: core::default::Default::default(),
            maintenance_end_time: core::default::Default::default(),
            maintenance_schedule_type: core::default::Default::default(),
            memory_per_oracle_compute_unit_gbs: core::default::Default::default(),
            memory_table_gbs: core::default::Default::default(),
            mtls_connection_required: core::default::Default::default(),
            n_character_set: core::default::Default::default(),
            next_long_term_backup_time: core::default::Default::default(),
            oci_url: core::default::Default::default(),
            ocid: core::default::Default::default(),
            open_mode: core::default::Default::default(),
            operations_insights_state: core::default::Default::default(),
            peer_db_ids: core::default::Default::default(),
            permission_level: core::default::Default::default(),
            private_endpoint: core::default::Default::default(),
            private_endpoint_ip: core::default::Default::default(),
            private_endpoint_label: core::default::Default::default(),
            refreshable_mode: core::default::Default::default(),
            refreshable_state: core::default::Default::default(),
            role: core::default::Default::default(),
            scheduled_operation_details: core::default::Default::default(),
            secret_id: core::default::Default::default(),
            sql_web_developer_url: core::default::Default::default(),
            state: core::default::Default::default(),
            supported_clone_regions: core::default::Default::default(),
            total_auto_backup_storage_size_gbs: core::default::Default::default(),
            used_data_storage_size_tbs: core::default::Default::default(),
            vault_id: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElRef {
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `actual_used_data_storage_size_tb` after provisioning.\n"]
    pub fn actual_used_data_storage_size_tb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.actual_used_data_storage_size_tb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allocated_storage_size_tb` after provisioning.\n"]
    pub fn allocated_storage_size_tb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocated_storage_size_tb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `apex_details` after provisioning.\n"]
    pub fn apex_details(
        &self,
    ) -> ListRef<
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElApexDetailsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.apex_details", self.base))
    }
    #[doc = "Get a reference to the value of field `are_primary_allowlisted_ips_used` after provisioning.\n"]
    pub fn are_primary_allowlisted_ips_used(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.are_primary_allowlisted_ips_used", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `autonomous_container_database_id` after provisioning.\n"]
    pub fn autonomous_container_database_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.autonomous_container_database_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `available_upgrade_versions` after provisioning.\n"]
    pub fn available_upgrade_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.available_upgrade_versions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `backup_retention_period_days` after provisioning.\n"]
    pub fn backup_retention_period_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backup_retention_period_days", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `character_set` after provisioning.\n"]
    pub fn character_set(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.character_set", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `compute_count` after provisioning.\n"]
    pub fn compute_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `connection_strings` after provisioning.\n"]    pub fn connection_strings (& self) -> ListRef < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionStringsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_strings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `connection_urls` after provisioning.\n"]
    pub fn connection_urls(
        &self,
    ) -> ListRef<
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElConnectionUrlsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_urls", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_core_count` after provisioning.\n"]
    pub fn cpu_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_core_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `customer_contacts` after provisioning.\n"]
    pub fn customer_contacts(
        &self,
    ) -> ListRef<
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElCustomerContactsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.customer_contacts", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_safe_state` after provisioning.\n"]
    pub fn data_safe_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_safe_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_storage_size_gb` after provisioning.\n"]
    pub fn data_storage_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_storage_size_gb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_storage_size_tb` after provisioning.\n"]
    pub fn data_storage_size_tb(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_storage_size_tb", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `database_management_state` after provisioning.\n"]
    pub fn database_management_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_management_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `db_edition` after provisioning.\n"]
    pub fn db_edition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_edition", self.base))
    }
    #[doc = "Get a reference to the value of field `db_version` after provisioning.\n"]
    pub fn db_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_version", self.base))
    }
    #[doc = "Get a reference to the value of field `db_workload` after provisioning.\n"]
    pub fn db_workload(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_workload", self.base))
    }
    #[doc = "Get a reference to the value of field `failed_data_recovery_duration` after provisioning.\n"]
    pub fn failed_data_recovery_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failed_data_recovery_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_auto_scaling_enabled` after provisioning.\n"]
    pub fn is_auto_scaling_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_auto_scaling_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_local_data_guard_enabled` after provisioning.\n"]
    pub fn is_local_data_guard_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_local_data_guard_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `is_storage_auto_scaling_enabled` after provisioning.\n"]
    pub fn is_storage_auto_scaling_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_storage_auto_scaling_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `license_type` after provisioning.\n"]
    pub fn license_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.license_type", self.base))
    }
    #[doc = "Get a reference to the value of field `lifecycle_details` after provisioning.\n"]
    pub fn lifecycle_details(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lifecycle_details", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_adg_auto_failover_max_data_loss_limit` after provisioning.\n"]
    pub fn local_adg_auto_failover_max_data_loss_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_adg_auto_failover_max_data_loss_limit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_disaster_recovery_type` after provisioning.\n"]
    pub fn local_disaster_recovery_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_disaster_recovery_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `local_standby_db` after provisioning.\n"]
    pub fn local_standby_db(
        &self,
    ) -> ListRef<
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElLocalStandbyDbElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.local_standby_db", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_begin_time` after provisioning.\n"]
    pub fn maintenance_begin_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_begin_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_end_time` after provisioning.\n"]
    pub fn maintenance_end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_end_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `maintenance_schedule_type` after provisioning.\n"]
    pub fn maintenance_schedule_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maintenance_schedule_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_per_oracle_compute_unit_gbs` after provisioning.\n"]
    pub fn memory_per_oracle_compute_unit_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_per_oracle_compute_unit_gbs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_table_gbs` after provisioning.\n"]
    pub fn memory_table_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_table_gbs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mtls_connection_required` after provisioning.\n"]
    pub fn mtls_connection_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mtls_connection_required", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `n_character_set` after provisioning.\n"]
    pub fn n_character_set(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.n_character_set", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `next_long_term_backup_time` after provisioning.\n"]
    pub fn next_long_term_backup_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_long_term_backup_time", self.base),
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
    #[doc = "Get a reference to the value of field `open_mode` after provisioning.\n"]
    pub fn open_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.open_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `operations_insights_state` after provisioning.\n"]
    pub fn operations_insights_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operations_insights_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `peer_db_ids` after provisioning.\n"]
    pub fn peer_db_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.peer_db_ids", self.base))
    }
    #[doc = "Get a reference to the value of field `permission_level` after provisioning.\n"]
    pub fn permission_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.permission_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `private_endpoint` after provisioning.\n"]
    pub fn private_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `private_endpoint_ip` after provisioning.\n"]
    pub fn private_endpoint_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_endpoint_ip", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `private_endpoint_label` after provisioning.\n"]
    pub fn private_endpoint_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_endpoint_label", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `refreshable_mode` after provisioning.\n"]
    pub fn refreshable_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.refreshable_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `refreshable_state` after provisioning.\n"]
    pub fn refreshable_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.refreshable_state", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
    #[doc = "Get a reference to the value of field `scheduled_operation_details` after provisioning.\n"]    pub fn scheduled_operation_details (& self) -> ListRef < DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElScheduledOperationDetailsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.scheduled_operation_details", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.base))
    }
    #[doc = "Get a reference to the value of field `sql_web_developer_url` after provisioning.\n"]
    pub fn sql_web_developer_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sql_web_developer_url", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `supported_clone_regions` after provisioning.\n"]
    pub fn supported_clone_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_clone_regions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `total_auto_backup_storage_size_gbs` after provisioning.\n"]
    pub fn total_auto_backup_storage_size_gbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_auto_backup_storage_size_gbs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `used_data_storage_size_tbs` after provisioning.\n"]
    pub fn used_data_storage_size_tbs(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.used_data_storage_size_tbs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vault_id` after provisioning.\n"]
    pub fn vault_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vault_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElSourceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_backups_replication_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autonomous_database: Option<PrimField<String>>,
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElSourceConfigEl {
    #[doc = "Set the field `automatic_backups_replication_enabled`.\n"]
    pub fn set_automatic_backups_replication_enabled(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.automatic_backups_replication_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `autonomous_database`.\n"]
    pub fn set_autonomous_database(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.autonomous_database = Some(v.into());
        self
    }
}
impl ToListMappable for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElSourceConfigEl {
    type O =
        BlockAssignable<DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElSourceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElSourceConfigEl {}
impl BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElSourceConfigEl {
    pub fn build(self) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElSourceConfigEl {
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElSourceConfigEl {
            automatic_backups_replication_enabled: core::default::Default::default(),
            autonomous_database: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElSourceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElSourceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElSourceConfigElRef {
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElSourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElSourceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `automatic_backups_replication_enabled` after provisioning.\n"]
    pub fn automatic_backups_replication_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.automatic_backups_replication_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `autonomous_database` after provisioning.\n"]
    pub fn autonomous_database(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.autonomous_database", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autonomous_database_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disaster_recovery_supported_locations: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entitlement_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    odb_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    odb_subnet: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peer_autonomous_databases: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties:
        Option<ListField<DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_config:
        Option<ListField<DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElSourceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terraform_labels: Option<RecField<PrimField<String>>>,
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesEl {
    #[doc = "Set the field `admin_password`.\n"]
    pub fn set_admin_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.admin_password = Some(v.into());
        self
    }
    #[doc = "Set the field `autonomous_database_id`.\n"]
    pub fn set_autonomous_database_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.autonomous_database_id = Some(v.into());
        self
    }
    #[doc = "Set the field `cidr`.\n"]
    pub fn set_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `database`.\n"]
    pub fn set_database(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.database = Some(v.into());
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
    #[doc = "Set the field `disaster_recovery_supported_locations`.\n"]
    pub fn set_disaster_recovery_supported_locations(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.disaster_recovery_supported_locations = Some(v.into());
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
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `odb_network`.\n"]
    pub fn set_odb_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.odb_network = Some(v.into());
        self
    }
    #[doc = "Set the field `odb_subnet`.\n"]
    pub fn set_odb_subnet(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.odb_subnet = Some(v.into());
        self
    }
    #[doc = "Set the field `peer_autonomous_databases`.\n"]
    pub fn set_peer_autonomous_databases(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.peer_autonomous_databases = Some(v.into());
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
        v: impl Into<ListField<DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesEl>>,
    ) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `source_config`.\n"]
    pub fn set_source_config(
        mut self,
        v: impl Into<
            ListField<DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElSourceConfigEl>,
        >,
    ) -> Self {
        self.source_config = Some(v.into());
        self
    }
    #[doc = "Set the field `terraform_labels`.\n"]
    pub fn set_terraform_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.terraform_labels = Some(v.into());
        self
    }
}
impl ToListMappable for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesEl {
    type O = BlockAssignable<DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesEl {}
impl BuildDataOracleDatabaseAutonomousDatabasesAutonomousDatabasesEl {
    pub fn build(self) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesEl {
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesEl {
            admin_password: core::default::Default::default(),
            autonomous_database_id: core::default::Default::default(),
            cidr: core::default::Default::default(),
            create_time: core::default::Default::default(),
            database: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            deletion_protection: core::default::Default::default(),
            disaster_recovery_supported_locations: core::default::Default::default(),
            display_name: core::default::Default::default(),
            effective_labels: core::default::Default::default(),
            entitlement_id: core::default::Default::default(),
            labels: core::default::Default::default(),
            location: core::default::Default::default(),
            name: core::default::Default::default(),
            network: core::default::Default::default(),
            odb_network: core::default::Default::default(),
            odb_subnet: core::default::Default::default(),
            peer_autonomous_databases: core::default::Default::default(),
            project: core::default::Default::default(),
            properties: core::default::Default::default(),
            source_config: core::default::Default::default(),
            terraform_labels: core::default::Default::default(),
        }
    }
}
pub struct DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElRef {
        DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_password` after provisioning.\n"]
    pub fn admin_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_password", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `autonomous_database_id` after provisioning.\n"]
    pub fn autonomous_database_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.autonomous_database_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
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
    #[doc = "Get a reference to the value of field `disaster_recovery_supported_locations` after provisioning.\n"]
    pub fn disaster_recovery_supported_locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disaster_recovery_supported_locations", self.base),
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
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `odb_network` after provisioning.\n"]
    pub fn odb_network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.odb_network", self.base))
    }
    #[doc = "Get a reference to the value of field `odb_subnet` after provisioning.\n"]
    pub fn odb_subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.odb_subnet", self.base))
    }
    #[doc = "Get a reference to the value of field `peer_autonomous_databases` after provisioning.\n"]
    pub fn peer_autonomous_databases(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.peer_autonomous_databases", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(
        &self,
    ) -> ListRef<DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `source_config` after provisioning.\n"]
    pub fn source_config(
        &self,
    ) -> ListRef<DataOracleDatabaseAutonomousDatabasesAutonomousDatabasesElSourceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\n"]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.base),
        )
    }
}
